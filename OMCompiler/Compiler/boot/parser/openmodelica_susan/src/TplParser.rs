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

use crate::Tpl;
use crate::TplAbsyn;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

//protected import Print;
pub const TabSpaces: i32 = 4;

pub mod CacheTree {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = Arc<metamodelica::List<TplAbsyn::ASTDef>>;

    pub fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (inKey.clone()).clone();
        outString
    }

    pub fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (literal!("#OPAQUE#")).clone();
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
    pub use self::Tree::{NODE,LEAF,EMPTY};

    pub type ValueNode = ArcStr;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TplAbsyn::ASTDef>>, Arc<metamodelica::List<TplAbsyn::ASTDef>>, ArcStr) -> Result<Arc<metamodelica::List<TplAbsyn::ASTDef>>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value = metamodelica::nil();
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
            let mut value: Value = metamodelica::nil();
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare((inKey.clone()).clone(), (var_field!((*tree).key, Tree::LEAF).clone()).clone());
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }), right: Arc::new(crate::TplParser::CacheTree::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::TplParser::CacheTree::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }) });
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
        let mut value: Value = metamodelica::nil();
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

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TplAbsyn::ASTDef>>, Arc<metamodelica::List<TplAbsyn::ASTDef>>, ArcStr) -> Result<Arc<metamodelica::List<TplAbsyn::ASTDef>>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key = arcstr::literal!("");
        let mut value: Value = metamodelica::nil();
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Arc<metamodelica::List<TplAbsyn::ASTDef>>>) -> Result<Arc<metamodelica::List<TplAbsyn::ASTDef>>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<metamodelica::List<TplAbsyn::ASTDef>>>) -> Result<Value> + 'static>;

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
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }), right: Arc::new(crate::TplParser::CacheTree::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::TplParser::CacheTree::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }) });
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
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

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
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

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
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

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>) -> Result<()> + 'static>) -> Result<()> {
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

    pub fn fromList(mut inValues: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TplAbsyn::ASTDef>>, Arc<metamodelica::List<TplAbsyn::ASTDef>>, ArcStr) -> Result<Arc<metamodelica::List<TplAbsyn::ASTDef>>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::TplParser::CacheTree::Tree::EMPTY);
        let mut key: Key = arcstr::literal!("");
        let mut value: Value = metamodelica::nil();
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value = metamodelica::nil();
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
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Option<Arc<metamodelica::List<TplAbsyn::ASTDef>>> {
        let mut value: Option<Arc<metamodelica::List<TplAbsyn::ASTDef>>> = None;
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TplAbsyn::ASTDef>>, Arc<metamodelica::List<TplAbsyn::ASTDef>>, ArcStr) -> Result<Arc<metamodelica::List<TplAbsyn::ASTDef>>> + 'static>) -> Result<Arc<Tree>> {
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

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<metamodelica::List<TplAbsyn::ASTDef>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<TplAbsyn::ASTDef>>>> {
        let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<TplAbsyn::ASTDef>>>> = lst;
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

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>) -> Result<Arc<metamodelica::List<TplAbsyn::ASTDef>>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = metamodelica::nil();
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
            let mut new_value: Value = metamodelica::nil();
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

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>, FT) -> Result<(Arc<metamodelica::List<TplAbsyn::ASTDef>>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = metamodelica::nil();
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
            let mut new_value: Value = metamodelica::nil();
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
        let mut outTree: Arc<Tree> = Arc::new(crate::TplParser::CacheTree::Tree::EMPTY);
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::TplParser::CacheTree::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::TplParser::CacheTree::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::TplParser::CacheTree::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::TplParser::CacheTree::Tree::EMPTY), node.clone())?
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

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>)>>) -> Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>)>> {
        let mut lst: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<TplAbsyn::ASTDef>>)>> = lst;
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
        let mut outTree: Arc<Tree> = add(tree.clone(), (key.clone()).clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Arc<metamodelica::List<TplAbsyn::ASTDef>>, Arc<metamodelica::List<TplAbsyn::ASTDef>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TplAbsyn::ASTDef>>, Arc<metamodelica::List<TplAbsyn::ASTDef>>, ArcStr) -> Result<Arc<metamodelica::List<TplAbsyn::ASTDef>>> + 'static>)).unwrap();
        outTree
    }

}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseInfo {
    pub fileName: ArcStr,
    pub errors: Arc<metamodelica::List<ArcStr>>,
    pub wasFatalError: bool,
}

pub type PARSE_INFO = ParseInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineInfo {
    pub parseInfo: ParseInfo,
    pub lineNumber: i32,
    pub lineLength: i32,
    pub startOfLineChars: Arc<metamodelica::List<ArcStr>>,
}

pub type LINE_INFO = LineInfo;


pub fn getPosition(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(i32, i32)> {
    let mut outLineNumber: i32 = 0;
    let mut outColumnNumber: i32 = 0;
    (outLineNumber, outColumnNumber) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (chars, LineInfo { lineLength: llen, lineNumber: lnum, .. }) => {
            let mut tillEnd: i32 = 0;
            tillEnd = charsTillEndOfLine(chars.clone(), 0)?;
            (lnum.clone(), llen.clone() - tillEnd.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outLineNumber, outColumnNumber))
}

pub type LineColumnNumber = (i32, i32);

pub static dummySourceInfo: std::sync::LazyLock<SourceInfo> = std::sync::LazyLock::new(|| { TplAbsyn::dummySourceInfo.clone() });

pub fn captureStartPosition(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inColumnOffset: i32) -> Result<LineColumnNumber> {
    let mut outLineColumnNumber: LineColumnNumber;
    let mut line: i32 = 0;
    let mut col: i32 = 0;
    (line, col) = getPosition(inChars.clone(), inLineInfo.clone())?;
    col = col.clone() - inColumnOffset.clone();
    outLineColumnNumber = (line.clone(), col.clone());
    Ok(outLineColumnNumber)
}

//TODO: add correct TIME_STAMP
pub fn tplSourceInfo(mut inStartLineColumnNumber: LineColumnNumber, mut inEndChars: Arc<metamodelica::List<ArcStr>>, mut inEndLineInfo: LineInfo) -> Result<SourceInfo> {
    let mut outSourceInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outSourceInfo = (::match_deref::match_deref! { match &((inStartLineColumnNumber.clone(), inEndChars.clone(), inEndLineInfo.clone())) {
        ((startL, startC), _, endlinfo @ LineInfo { parseInfo: ParseInfo { fileName, .. }, .. }) => {
            let mut endL: i32 = 0;
            let mut endC: i32 = 0;
            (endL, endC) = getPosition(inEndChars.clone(), endlinfo.clone())?;
            outSourceInfo = SourceInfo { fileName: (fileName.clone()).clone(), isReadOnly: false, lineNumberStart: startL.clone(), columnNumberStart: startC.clone(), lineNumberEnd: endL.clone(), columnNumberEnd: endC.clone(), lastModification: metamodelica::OrderedFloat(0.0_f64) };
            outSourceInfo.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSourceInfo)
}

pub fn startPositionFromExp(mut inExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo)) -> Result<LineColumnNumber> {
    let mut outLineColumnNumber: LineColumnNumber;
    let mut line: i32 = 0;
    let mut col: i32 = 0;
    outLineColumnNumber = (::match_deref::match_deref! { match &(inExpression.clone()) {
        (_, SourceInfo { columnNumberStart: startC, lineNumberStart: startL, .. }) => {
            (startL.clone(), startC.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outLineColumnNumber)
}

pub fn charsTillEndOfLine(mut inChars: Arc<metamodelica::List<ArcStr>>, mut outCharsTillEnd: i32) -> Result<i32> {
    let mut outCharsTillEnd: i32 = outCharsTillEnd;
    let mut i: i32 = 0;
    for mut c in &*inChars.clone() {
        let mut c = c.clone();
        i = stringCharInt((c.clone()).clone())?;
        if i.clone() == 10 || i.clone() == 13 {
            return Ok(outCharsTillEnd);
        }
        outCharsTillEnd = outCharsTillEnd.clone() + if (i.clone() == 9) {TabSpaces.clone()} else {1};
    }
    Ok(outCharsTillEnd)
}

pub fn makeStartLineInfo(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inFileName: ArcStr) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    let mut llen: i32 = 0;
    llen = charsTillEndOfLine(inChars.clone(), 1)?;
    outLineInfo = LineInfo { parseInfo: ParseInfo { fileName: (inFileName.clone()).clone(), errors: metamodelica::nil(), wasFatalError: false }, lineNumber: 1, lineLength: llen.clone(), startOfLineChars: inChars.clone() };
    Ok(outLineInfo)
}

pub fn printAndFailIfError(mut inLineInfo: LineInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inLineInfo.clone()) {
        LineInfo { parseInfo: ParseInfo { errors: Deref @ metamodelica::List::Nil, .. }, .. } => {
            println!("{}", (literal!("\nSusan parsing successful.\n")).clone());
            ()
        },
        LineInfo { parseInfo: ParseInfo { errors: errLst @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. } => {
            println!("{}", (literal!("\nSusan parse error(s):\n")).clone());
            println!("{}", stringDelimitList(errLst.clone().reverse(), (literal!("\n")).clone()));
            println!("{}", (literal!("\n")).clone());
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn parseError(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inErrMessage: ArcStr, mut isFatal: bool) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    outLineInfo = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inErrMessage.clone(), isFatal.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo @ LineInfo { startOfLineChars: solchars, lineLength: llen, lineNumber: lnum, parseInfo: ParseInfo { wasFatalError: false, errors: errLst, fileName: fname } }, errMsg, isfatal) => {
                    let mut locStr: ArcStr = arcstr::literal!("");
                    let mut colnum: i32 = 0;
                    let mut errMsg = (*errMsg).clone();
                    (_, colnum) = getPosition(chars.clone(), linfo.clone())?;
                    locStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(lnum.clone())); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*intString(colnum.clone())); ArcStr::from(__mm_s) }).clone();
                    errMsg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fname.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*locStr.clone()); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*locStr.clone()); __mm_s.push_str(&*literal!(" Error:(parser)")); __mm_s.push_str(&*errMsg.clone()); __mm_s.push_str(&*literal!("(col ")); __mm_s.push_str(&*intString(colnum.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.parseError msg: ")); __mm_s.push_str(&*errMsg.clone()); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(LineInfo { parseInfo: ParseInfo { fileName: (fname.clone()).clone(), errors: cons((errMsg.clone()).clone(), errLst.clone()), wasFatalError: isfatal.clone() }, lineNumber: lnum.clone(), lineLength: llen.clone(), startOfLineChars: solchars.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, linfo @ LineInfo { parseInfo: ParseInfo { wasFatalError: true, .. }, .. }, _, _) => {
                    Ok(linfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- !!! TplParser.parseError failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLineInfo)
}

pub fn parseErrorPrevPosition(mut inCharsPrevPos: Arc<metamodelica::List<ArcStr>>, mut inLineInfoPrevPos: LineInfo, mut inLineInfo: LineInfo, mut inErrMessage: ArcStr, mut isFatal: bool) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    outLineInfo = 'mc: {
        let __mc_input = (inCharsPrevPos.clone(), inLineInfoPrevPos.clone(), inLineInfo.clone(), inErrMessage.clone(), isFatal.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (charspp, LineInfo { startOfLineChars: solcharspp, lineLength: llenpp, lineNumber: lnumpp, .. }, LineInfo { startOfLineChars: solchars, lineLength: llen, lineNumber: lnum, parseInfo: pinfo }, errMsg, isfatal) => {
                    let mut linfopp: LineInfo;
                    let mut pinfo = (*pinfo).clone();
                    linfopp = LineInfo { parseInfo: pinfo.clone(), lineNumber: lnumpp.clone(), lineLength: llenpp.clone(), startOfLineChars: solcharspp.clone() };
                    let LineInfo { parseInfo: __pa0, .. } = (parseError(charspp.clone(), linfopp.clone(), (errMsg.clone()).clone(), isfatal.clone())?) else { bail!("pattern mismatch") };
                    pinfo = __pa0.clone();
                    Ok(LineInfo { parseInfo: pinfo.clone(), lineNumber: lnum.clone(), lineLength: llen.clone(), startOfLineChars: solchars.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- !!! TplParser.parseErrorPrevPosition failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLineInfo)
}

pub fn wasFatalError(mut inLineInfo: LineInfo) -> bool {
    let mut outWasError: bool = false;
    outWasError = (match inLineInfo.clone() {
        LineInfo { parseInfo: ParseInfo { wasFatalError: true, .. }, .. } => true,
        _ => false,
    });
    outWasError
}

pub fn mergeErrors(mut inLineInfo: LineInfo, mut inLineInfoToAddErrorsFrom: LineInfo) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    outLineInfo = 'mc: {
        let __mc_input = (inLineInfo.clone(), inLineInfoToAddErrorsFrom.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (LineInfo { startOfLineChars: ref solchars, lineLength: mut llen, lineNumber: mut lnum, parseInfo: ParseInfo { wasFatalError: mut wasFatalError, errors: ref errLst, fileName: mut fname } }, LineInfo { parseInfo: ParseInfo { errors: ref errLstToAdd, .. }, .. }) = __mc_input.clone() else { bail!("nomatch") };
            let mut errLst = errLst.clone();
            errLst = listAppend(errLstToAdd.clone(), errLst.clone());
            Ok(LineInfo { parseInfo: ParseInfo { fileName: (fname.clone()).clone(), errors: errLst.clone(), wasFatalError: wasFatalError.clone() }, lineNumber: lnum.clone(), lineLength: llen.clone(), startOfLineChars: solchars.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- !!! TplParser.mergeErrors failed.\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLineInfo)
}

pub fn parseErrorPrevPositionOpt(mut inCharsPrevPos: Arc<metamodelica::List<ArcStr>>, mut inLineInfoPrevPos: LineInfo, mut inLineInfo: LineInfo, mut inErrMessage: Option<ArcStr>, mut isFatal: bool) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    outLineInfo = 'mc: {
        let __mc_input = (inCharsPrevPos.clone(), inLineInfoPrevPos.clone(), inLineInfo.clone(), inErrMessage.clone(), isFatal.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, linfo, None, _) => {
                    Ok(linfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (charspp, linfopp, linfo, Some(errMsg), isfatal) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseErrorPrevPosition(charspp.clone(), linfopp.clone(), linfo.clone(), (errMsg.clone()).clone(), isfatal.clone())?;
                    Ok(linfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- !!! TplParser.parseErrorPrevPositionOpt failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLineInfo)
}

pub fn parseErrorPrevPositionOptInfoChars(mut inLineInfoPrevPos: LineInfo, mut inLineInfo: LineInfo, mut inErrMessage: Option<ArcStr>, mut isFatal: bool) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    let mut sol_chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let LineInfo { startOfLineChars: __pa0, .. } = (inLineInfoPrevPos.clone()) else { bail!("pattern mismatch") };
    sol_chars = __pa0.clone();
    outLineInfo = parseErrorPrevPositionOpt(sol_chars.clone(), inLineInfoPrevPos.clone(), inLineInfo.clone(), inErrMessage.clone(), isFatal.clone())?;
    Ok(outLineInfo)
}

pub fn expectChar(mut chars: Arc<metamodelica::List<ArcStr>>, mut lineInfo: LineInfo, mut inExpectedChar: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut chars: Arc<metamodelica::List<ArcStr>> = chars;
    let mut lineInfo: LineInfo = lineInfo;
    chars = (::match_deref::match_deref! { match &(chars.clone()) {
        Deref @ metamodelica::List::Cons { head: c, tail: rest } if (stringEq((c.clone()).clone(), (inExpectedChar.clone()).clone())) => {
            rest.clone()
        },
        _ => {
            lineInfo = parseError(chars.clone(), lineInfo.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected character '")); __mm_s.push_str(&*inExpectedChar.clone()); __mm_s.push_str(&*literal!("' at the position.")); ArcStr::from(__mm_s) }).clone(), false)?;
            chars.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((chars, lineInfo))
}

//intended to say error before the last interleave, but need
//TODO: remember the last position before interleave in the LINE_INFO
pub fn interleaveExpectChar(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inExpectedChar: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inExpectedChar.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, ec) => {
                    let mut c: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    c = __pa0.clone();
                    chars = __pa1.clone();
                    let true = (stringEq((c.clone()).clone(), (ec.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, ec) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected character '")); __mm_s.push_str(&*ec.clone()); __mm_s.push_str(&*literal!("' after the position.")); ArcStr::from(__mm_s) }).clone(), false)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- !!! TplParser.interleaveExpectChar failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn takeKeywordChars(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inKeywordChars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outChars = (::match_deref::match_deref! { match &((inChars.clone(), inKeywordChars.clone())) {
        (Deref @ metamodelica::List::Cons { head: c, tail: chars }, Deref @ metamodelica::List::Cons { head: kwc, tail: kwchars }) => {
            let true = (stringEq((c.clone()).clone(), (kwc.clone()).clone())) else { bail!("pattern mismatch") };
            takeKeywordChars(chars.clone(), kwchars.clone())?
        },
        (chars, Deref @ metamodelica::List::Nil) => {
            chars.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outChars)
}

pub fn isKeyword(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inKeywordChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, bool)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut isKeyword: bool = false;
    (outChars, isKeyword) = 'mc: {
        let __mc_input = (inChars.clone(), inKeywordChars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, kwchars) => {
                    let mut chars = (*chars).clone();
                    chars = takeKeywordChars(chars.clone(), kwchars.clone())?;
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, isKeyword))
}

pub fn interleaveExpectKeyWord(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inKeywordChars: Arc<metamodelica::List<ArcStr>>, mut isFatal: bool) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inKeywordChars.clone(), isFatal.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, kwchars, _) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(isKeyword(chars.clone(), kwchars.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, kwchars, isfatal) => {
                    let mut kw: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), kwchars.clone())?) {
                        (_, false) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    kw = (stringCharListString(kwchars.clone())).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected keyword '")); __mm_s.push_str(&*kw.clone()); __mm_s.push_str(&*literal!("' at the position.")); ArcStr::from(__mm_s) }).clone(), isfatal.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- !!! TplParser.interleaveExpectKeyWord failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

pub fn interleaveExpectEndOfFile(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(interleave(chars.clone(), linfo.clone())?) {
                        (Deref @ metamodelica::List::Nil, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    linfo = __pa0.clone();
                    Ok((metamodelica::nil(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected end of file at the position.")).clone(), false)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- !!! TplParser.interleaveExpectEndOfFile failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

pub fn openFile(mut inFile: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Option<ArcStr>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outErrorOpt: Option<ArcStr> = None;
    (outChars, outLineInfo, outErrorOpt) = 'mc: {
        let __mc_input = inFile.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else { bail!("nomatch") };
            let mut src: ArcStr = arcstr::literal!("");
            let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut linfo: LineInfo;
            let true = (System::regularFileExists((file.clone()).clone())) else { bail!("pattern mismatch") };
            src = (System::readFile((file.clone()).clone())?).clone();
            chars = stringListStringChar((src.clone()).clone());
            linfo = makeStartLineInfo(chars.clone(), (file.clone()).clone())?;
            Ok((chars.clone(), linfo.clone(), None))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else { bail!("nomatch") };
            let mut errStr: ArcStr = arcstr::literal!("");
            let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut linfo: LineInfo;
            let false = (System::regularFileExists((file.clone()).clone())) else { bail!("pattern mismatch") };
            chars = metamodelica::nil();
            linfo = makeStartLineInfo(chars.clone(), (file.clone()).clone())?;
            errStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("No such file '")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) }).clone();
            Ok((chars.clone(), linfo.clone(), Some((errStr.clone()).clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.openFile failed for file '")); __mm_s.push_str(&*inFile.clone()); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outErrorOpt))
}

pub fn templPackageFromFile(mut inFile: ArcStr) -> Result<TplAbsyn::TemplPackage> {
    let mut outTemplPackage: TplAbsyn::TemplPackage;
    outTemplPackage = 'mc: {
        let __mc_input = inFile.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else { bail!("nomatch") };
            let mut errOpt: Option<ArcStr> = None;
            let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut linfo: LineInfo;
            let mut tplPackage: TplAbsyn::TemplPackage;
            (chars, linfo, errOpt) = openFile((file.clone()).clone())?;
            linfo = parseErrorPrevPositionOpt(chars.clone(), linfo.clone(), linfo.clone(), errOpt.clone(), true)?;
            (chars, linfo, tplPackage, _) = templPackage(chars.clone(), linfo.clone(), Arc::new(crate::TplParser::CacheTree::Tree::EMPTY))?;
            (_, linfo) = interleaveExpectEndOfFile(chars.clone(), linfo.clone())?;
            printAndFailIfError(linfo.clone())?;
            Ok(tplPackage.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.templPackageFromFile failed for file '")); __mm_s.push_str(&*inFile.clone()); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTemplPackage)
}

fn typeviewDefsFromInterfaceFile(mut interfaceName: Arc<TplAbsyn::PathIdent>, mut astDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>>, mut cachedDefs: Arc<CacheTree::Tree>) -> Result<(Arc<metamodelica::List<TplAbsyn::ASTDef>>, LineInfo, Option<ArcStr>, Arc<CacheTree::Tree>)> {
    let mut astDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = astDefs;
    let mut linfo: LineInfo;
    let mut errOpt: Option<ArcStr> = None;
    let mut cachedDefs: Arc<CacheTree::Tree> = cachedDefs;
    let mut file: ArcStr = arcstr::literal!("");
    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut newAstDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = metamodelica::nil();
    file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*TplAbsyn::pathIdentString(interfaceName.clone())?); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone();
    match '__try0: {
        if unwrap_break_err!(CacheTree::hasKey(cachedDefs.clone(), (file.clone()).clone()), '__try0) {
            astDefs = listAppend(CacheTree::get(cachedDefs.clone(), (file.clone()).clone())?, astDefs.clone());
            linfo = LineInfo { parseInfo: ParseInfo { fileName: (literal!("cachedResult")).clone(), errors: metamodelica::nil(), wasFatalError: false }, lineNumber: 0, lineLength: 0, startOfLineChars: metamodelica::nil() };
            errOpt = None;
            return Ok((astDefs, linfo, errOpt, cachedDefs));
        }
        (chars, linfo, errOpt) = unwrap_break_err!(openFile((file.clone()).clone()), '__try0);
        (chars, linfo) = unwrap_break_err!(interleave(chars.clone(), linfo.clone()), '__try0);
        (chars, linfo, _, newAstDefs) = unwrap_break_err!(interfacePackage(chars.clone(), linfo.clone(), metamodelica::nil()), '__try0);
        (_, linfo) = unwrap_break_err!(interleaveExpectEndOfFile(chars.clone(), linfo.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::FAILTRACE.clone()), '__try0) {
            unwrap_break_err!(Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Loaded interface file: ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()), '__try0);
        }
        cachedDefs = unwrap_break_err!(CacheTree::add(cachedDefs.clone(), (file.clone()).clone(), newAstDefs.clone(), (std::sync::Arc::new(CacheTree::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)), '__try0);
        astDefs = listAppend(newAstDefs.clone(), astDefs.clone());
        Ok::<_, anyhow::Error>((astDefs.clone(), cachedDefs.clone(), chars.clone(), errOpt.clone(), linfo.clone(), newAstDefs.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            astDefs = __try0_o0;
            cachedDefs = __try0_o1;
            chars = __try0_o2;
            errOpt = __try0_o3;
            linfo = __try0_o4;
            newAstDefs = __try0_o5;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.typeviewDefsFromInterfaceFile ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            bail!("fail");
        }
    }
    Ok((astDefs, linfo, errOpt, cachedDefs))
}

fn typeviewDefsFromTemplateFile(mut packageName: Arc<TplAbsyn::PathIdent>, mut isUnqualifiedImport: bool, mut astDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>>, mut cachedDefs: Arc<CacheTree::Tree>) -> Result<(Arc<metamodelica::List<TplAbsyn::ASTDef>>, LineInfo, Option<ArcStr>, Arc<CacheTree::Tree>)> {
    let mut astDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = astDefs;
    let mut linfo: LineInfo;
    let mut errOpt: Option<ArcStr> = None;
    let mut cachedDefs: Arc<CacheTree::Tree> = cachedDefs;
    let mut file: ArcStr = arcstr::literal!("");
    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut newAstDef: TplAbsyn::ASTDef;
    let mut tplPackage: TplAbsyn::TemplPackage;
    let mut templateDefs: Arc<metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>> = metamodelica::nil();
    let mut astTypes: Arc<metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>> = metamodelica::nil();
    file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*TplAbsyn::pathIdentString(packageName.clone())?); __mm_s.push_str(&*literal!(".tpl")); ArcStr::from(__mm_s) }).clone();
    match '__try0: {
        if unwrap_break_err!(CacheTree::hasKey(cachedDefs.clone(), (file.clone()).clone()), '__try0) {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(CacheTree::get(cachedDefs.clone(), (file.clone()).clone()), '__try0)) {
                Deref @ metamodelica::List::Cons { head: TplAbsyn::ASTDef { importPackage: _, isDefault: _, types: __pa1 }, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            astTypes = __pa1.clone();
            astDefs = cons(TplAbsyn::ASTDef { importPackage: packageName.clone(), isDefault: isUnqualifiedImport.clone(), types: astTypes.clone() }, astDefs.clone());
            linfo = LineInfo { parseInfo: ParseInfo { fileName: (literal!("cachedResult")).clone(), errors: metamodelica::nil(), wasFatalError: false }, lineNumber: 0, lineLength: 0, startOfLineChars: metamodelica::nil() };
            errOpt = None;
            return Ok((astDefs, linfo, errOpt, cachedDefs));
        }
        (chars, linfo, errOpt) = unwrap_break_err!(openFile((file.clone()).clone()), '__try0);
        (chars, linfo, tplPackage, cachedDefs) = unwrap_break_err!(templPackage(chars.clone(), linfo.clone(), cachedDefs.clone()), '__try0);
        (_, linfo) = unwrap_break_err!(interleaveExpectEndOfFile(chars.clone(), linfo.clone()), '__try0);
        let TplAbsyn::TEMPL_PACKAGE { templateDefs: __pa3, .. } = (unwrap_break_err!(TplAbsyn::fullyQualifyTemplatePackage(tplPackage.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        templateDefs = __pa3.clone();
        astTypes = List::map(templateDefs.clone(), (std::sync::Arc::new(templateDefToAstDefType) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, TplAbsyn::TemplateDef)) -> Result<(ArcStr, TplAbsyn::TypeInfo)> + 'static>));
        newAstDef = TplAbsyn::ASTDef { importPackage: packageName.clone(), isDefault: isUnqualifiedImport.clone(), types: astTypes.clone() };
        cachedDefs = unwrap_break_err!(CacheTree::add(cachedDefs.clone(), (file.clone()).clone(), cons(newAstDef.clone(), metamodelica::nil()), (std::sync::Arc::new(CacheTree::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)), '__try0);
        astDefs = cons(newAstDef.clone(), astDefs.clone());
        if unwrap_break_err!(Flags::isSet(Flags::FAILTRACE.clone()), '__try0) {
            unwrap_break_err!(Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Loaded typeview from template file: ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((astDefs.clone(), astTypes.clone(), cachedDefs.clone(), chars.clone(), errOpt.clone(), linfo.clone(), newAstDef.clone(), templateDefs.clone(), tplPackage.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
            astDefs = __try0_o0;
            astTypes = __try0_o1;
            cachedDefs = __try0_o2;
            chars = __try0_o3;
            errOpt = __try0_o4;
            linfo = __try0_o5;
            newAstDef = __try0_o6;
            templateDefs = __try0_o7;
            tplPackage = __try0_o8;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.typeviewDefsFromInterfaceFile ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            bail!("fail");
        }
    }
    Ok((astDefs, linfo, errOpt, cachedDefs))
}

pub fn templateDefToAstDefType(mut inTemplateDef: (ArcStr, TplAbsyn::TemplateDef)) -> Result<(ArcStr, TplAbsyn::TypeInfo)> {
    let mut outType: (ArcStr, TplAbsyn::TypeInfo);
    outType = 'mc: {
        let __mc_input = inTemplateDef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let (mut id, TplAbsyn::TemplateDef::STR_TOKEN_DEF { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok((id.clone(), TplAbsyn::TypeInfo::TI_CONST_TYPE { constType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE) }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut id, TplAbsyn::TemplateDef::LITERAL_DEF { litType: mut litType, .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok((id.clone(), TplAbsyn::TypeInfo::TI_CONST_TYPE { constType: litType.clone() }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut id, TplAbsyn::TemplateDef::TEMPLATE_DEF { args: ref iargs, .. }) = __mc_input.clone() else { bail!("nomatch") };
            let mut oargs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
            let mut iargs = iargs.clone();
            iargs = cons(TplAbsyn::imlicitTxtArg.clone(), iargs.clone());
            oargs = List::filterOnTrue(iargs.clone(), (std::sync::Arc::new(fnptr!(TplAbsyn::isText, (ArcStr, Arc<TplAbsyn::TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TplAbsyn::TypeSignature>)) -> Result<bool> + 'static>));
            Ok((id.clone(), TplAbsyn::TypeInfo::TI_FUN_TYPE { inArgs: iargs.clone(), outArgs: oargs.clone(), tyVars: metamodelica::nil() }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("Parse error - TplParser.templateDefToAstDefType failed.\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

/*
newLine:
  \r \n  //CR + LF ... Windows
  |
  \n     //CR only ... Linux
  |
  \r     //LF only ... Mac OS up to 9
*/
pub fn newLine(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (Deref @ metamodelica::List::Cons { head: c, tail: chars }, LineInfo { lineNumber: lnum, parseInfo: pinfo, .. }) => {
            let mut llen: i32 = 0;
            let mut i: i32 = 0;
            let mut chars = (*chars).clone();
            let mut lnum = (*lnum).clone();
            i = stringCharInt((c.clone()).clone())?;
            if i.clone() == 13 {
                chars = (::match_deref::match_deref! { match &(chars.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ "\n", tail: chars } => chars.clone(),
        _ => chars.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            let true = (i.clone() == 10 || i.clone() == 13) else { bail!("pattern mismatch") };
            llen = charsTillEndOfLine(chars.clone(), 1)?;
            lnum = lnum.clone() + 1;
            (chars.clone(), LineInfo { parseInfo: pinfo.clone(), lineNumber: lnum.clone(), lineLength: llen.clone(), startOfLineChars: chars.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo))
}

/*
// interleave will be applied before every token
interleave:  //i.e. space / comment
  [' '\n\r\t] interleave
  |
  '//' toEndOfLine  interleave
  |
  '/''*' comment  interleave
  |
  _ //just nothing
*/
pub fn interleave(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ " ", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\t", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: chars } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = toEndOfLine(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: Deref @ metamodelica::List::Cons { head: Deref @ "*", tail: chars } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = comment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars @ Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: Deref @ metamodelica::List::Cons { head: Deref @ "*", tail: charsRest } }, linfo) => {
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        (_, _) = unwrap_break_err!(comment(charsRest.clone(), linfo.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Unmatched /* */ comment - reached end of file.")).clone(), true)?;
                    Ok((metamodelica::nil(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
toEndOfLine:
    \n
    |
    eof  //end of stream ~ {}
    |
    any  toEndOfLine //any is any character
*/
pub fn toEndOfLine(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = toEndOfLine(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, linfo) => {
                    Ok((metamodelica::nil(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

//comment:
//  '*''/'
//  |
//  '/''*' comment comment  //nesting is possible
//  |
//  any  comment
pub fn comment(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "*", tail: Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: chars } }, linfo) => {
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: Deref @ metamodelica::List::Cons { head: Deref @ "*", tail: chars } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = comment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = comment(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    (chars, linfo) = comment(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars @ Deref @ metamodelica::List::Cons { head: _, tail: charsRest }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        (_, _) = unwrap_break_err!(newLine(chars.clone(), linfo.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (chars, linfo) = comment(charsRest.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
//afterKeyword must not fail after every keyword to be considered as keyword
afterKeyword:
    [_0-9A-Za-z]  =>  fail  // if it can be an identifier/other keyword
    |
    _  => ()
*/
pub fn afterKeyword(mut inChars: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::List::Cons { head: c, tail: _ } => {
            let mut i: i32 = 0;
            i = stringCharInt((c.clone()).clone())?;
            let false = (i.clone() == 95 || 48 <= i.clone() && i.clone() <= 57 || 65 <= i.clone() && i.clone() <= 90 || 97 <= i.clone() && i.clone() <= 122) else { bail!("pattern mismatch") };
            ()
        },
        Deref @ metamodelica::List::Nil => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

/*
identifier:
  [_A-Za-z]:c  identifier_rest:rest     =>  stringCharListString(c::rest)
*/
pub static keywords: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("end")).clone(), (literal!("if")).clone(), (literal!("then")).clone(), (literal!("else")).clone(), (literal!("match")).clone(), (literal!("case")).clone(), (literal!("equation")).clone(), (literal!("equality")).clone(), (literal!("failure")).clone(), (literal!("algorithm")).clone(), (literal!("input")).clone(), (literal!("output")).clone(), (literal!("matchcontinue")).clone(), (literal!("local")).clone(), (literal!("constant")).clone(), (literal!("extends")).clone(), (literal!("external")).clone(), (literal!("for")).clone(), (literal!("function")).clone(), (literal!("import")).clone(), (literal!("package")).clone(), (literal!("partial")).clone(), (literal!("protected")).clone(), (literal!("public")).clone(), (literal!("record")).clone(), (literal!("as")).clone(), (literal!("uniontype")).clone(), (literal!("subtypeof")).clone()] });

pub fn identifier(mut inChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outIdent: ArcStr = arcstr::literal!("");
    (outChars, outIdent) = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::List::Cons { head: c, tail: chars } => {
            let mut restIdChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut ident: ArcStr = arcstr::literal!("");
            let mut i: i32 = 0;
            let mut chars = (*chars).clone();
            i = stringCharInt((c.clone()).clone())?;
            let true = (i.clone() == 95 || 65 <= i.clone() && i.clone() <= 90 || 97 <= i.clone() && i.clone() <= 122) else { bail!("pattern mismatch") };
            (chars, restIdChars) = identifier_rest(chars.clone())?;
            ident = (stringCharListString(cons((c.clone()).clone(), restIdChars.clone()))).clone();
            (chars.clone(), ident.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outChars, outIdent))
}

/*
identifier_rest:
    [_0-9A-Za-z]:c  identifier_rest:rest  =>  c::rest
    |
  _  =>  {}
*/
pub fn identifier_rest(mut inChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outRestIdentChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outChars, outRestIdentChars) = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::List::Cons { head: c, tail: chars } => {
            let mut restIdChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut i: i32 = 0;
            let mut chars = (*chars).clone();
            i = stringCharInt((c.clone()).clone())?;
            if i.clone() == 95 || 48 <= i.clone() && i.clone() <= 57 || 65 <= i.clone() && i.clone() <= 90 || 97 <= i.clone() && i.clone() <= 122 {
                (chars, restIdChars) = identifier_rest(chars.clone())?;
                restIdChars = cons((c.clone()).clone(), restIdChars.clone());
            } else {
                chars = inChars.clone();
                restIdChars = metamodelica::nil();
            }
            (chars.clone(), restIdChars.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outChars, outRestIdentChars))
}

/*
pathIdent:
  identifier:head  pathIdentPath(head):pid => pid
*/
pub fn pathIdent(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::PathIdent>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outPathIdent: Arc<TplAbsyn::PathIdent>;
    (outChars, outLineInfo, outPathIdent) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (chars, linfo) => {
            let mut head: ArcStr = arcstr::literal!("");
            let mut pid: Arc<TplAbsyn::PathIdent>;
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, head) = identifier(chars.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, pid) = pathIdentPath(chars.clone(), linfo.clone(), (head.clone()).clone())?;
            (chars.clone(), linfo.clone(), pid.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outPathIdent))
}

/*
pathIdentPath(head):
  '.' pathIdent:path  =>  PATH_IDENT(head, path)
  |
  '.' error "expecting identifier after dot."
    => PATH_IDENT(head, TplAbsyn.IDENT("#error#"))
  |
  _ =>  IDENT(head)
*/
pub fn pathIdentPath(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inHeadIdent: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::PathIdent>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outPathIdent: Arc<TplAbsyn::PathIdent>;
    (outChars, outLineInfo, outPathIdent) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inHeadIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ".", tail: chars }, linfo, head) => {
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, pid) = pathIdent(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (head.clone()).clone(), path: pid.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), Arc::new(TplAbsyn::PathIdent::IDENT { ident: (inHeadIdent.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outPathIdent))
}

pub fn identifierNoOpt(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, ArcStr)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outIdent: ArcStr = arcstr::literal!("");
    (outChars, outLineInfo, outIdent) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut ident: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    (chars, ident) = identifier(chars.clone())?;
                    Ok((chars.clone(), linfo.clone(), ident.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        (_, _) = unwrap_break_err!(identifier(chars.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected an identifier at the position.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), literal!("#error#")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outIdent))
}

pub fn pathIdentNoOpt(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::PathIdent>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outPathIdent: Arc<TplAbsyn::PathIdent>;
    (outChars, outLineInfo, outPathIdent) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (chars, linfo) => {
            let mut head: ArcStr = arcstr::literal!("");
            let mut pid: Arc<TplAbsyn::PathIdent>;
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, head) = identifierNoOpt(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, pid) = pathIdentPath(chars.clone(), linfo.clone(), (head.clone()).clone())?;
            (chars.clone(), linfo.clone(), pid.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outPathIdent))
}

/*
templPackage:
  'package'  pathIdent:pid  stringComment
    definitions(pid,{},{}):(astDefs,templDefs)
  endDefPathIdent(pid)
  =>   TEMPL_PACKAGE(pid, astDefs,templDefs)
*/
pub fn templPackage(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut cachedDefs: Arc<CacheTree::Tree>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, TplAbsyn::TemplPackage, Arc<CacheTree::Tree>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTemplPackage: TplAbsyn::TemplPackage;
    let mut cachedDefs: Arc<CacheTree::Tree> = cachedDefs;
    (outChars, outLineInfo, outTemplPackage) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut astDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = metamodelica::nil();
                    let mut templDefs: Arc<metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>> = metamodelica::nil();
                    let mut annotationFooter: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut cachedDefs: Arc<CacheTree::Tree> = cachedDefs.clone();
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![(literal!("p")).clone(), (literal!("a")).clone(), (literal!("c")).clone(), (literal!("k")).clone(), (literal!("a")).clone(), (literal!("g")).clone(), (literal!("e")).clone()], true)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, pid) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, astDefs, templDefs, cachedDefs) = definitions(chars.clone(), linfo.clone(), metamodelica::nil(), metamodelica::nil(), cachedDefs.clone())?;
                    astDefs = astDefs.clone().reverse();
                    templDefs = templDefs.clone().reverse();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, annotationFooter) = self::annotationFooter(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = endDefPathIdent(chars.clone(), linfo.clone(), pid.clone())?;
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplPackage { name: pid.clone(), astDefs: astDefs.clone(), templateDefs: templDefs.clone(), annotationFooter: (annotationFooter.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!!Parse error - TplParser.templPackage failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTemplPackage, cachedDefs))
}

/*
definitions(astDefs,templDefs):
  'import' 'interface' pathIdent:pid stringComment ';'
    { ads = typeviewDefsFromInterfaceFile(packageNameToFileName(pid,".mo"), astDefs) }
    definitions(ads, templDefs):(ads,tds)
    => (ads,tds)
  |
  'import' pathIdent:pid unqualImportPostfix:unq stringComment ';'
    { ads = typeviewDefsFromTemplateFile(pid, unq, astDefs) }
    definitions(ads, templDefs):(ads,tds)
    => (ads,tds)
//  |
//  absynDef:ad  definitions(ad::astDefs,templDefs):(ads,tds) => (ads,tds)
  |
  templDef:(name, td)  definitions(astDefs,(name,td)::templDefs):(ads,tds) => (ads,tds)
//  |
//  error "Expecting 'end' | ['public' | 'protected' ] 'package' definition | template definition starting with an identifier."
*/
pub fn definitions(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inAccASTDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>>, mut inAccTemplDefs: Arc<metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>>, mut cachedDefs: Arc<CacheTree::Tree>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<TplAbsyn::ASTDef>>, Arc<metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>>, Arc<CacheTree::Tree>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outASTDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = metamodelica::nil();
    let mut outTemplDefs: Arc<metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>> = metamodelica::nil();
    let mut cachedDefs: Arc<CacheTree::Tree> = cachedDefs;
    (outChars, outLineInfo, outASTDefs, outTemplDefs) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inAccASTDefs.clone(), inAccTemplDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } }, linfo, astDefs, templDefs) => {
                    afterKeyword(chars.clone())?;
                    Ok((startChars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "m", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } } } } }, linfo, astDefs, templDefs) => {
                    let mut startChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut errOptTV: Option<ArcStr> = None;
                    let mut startLinfo: LineInfo;
                    let mut linfoTV: LineInfo;
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut astDefs = (*astDefs).clone();
                    let mut templDefs = (*templDefs).clone();
                    let mut cachedDefs: Arc<CacheTree::Tree> = cachedDefs.clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "f", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: __pa0 } } } } } } } } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    afterKeyword(chars.clone())?;
                    (startChars, startLinfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, pid) = pathIdentNoOpt(startChars.clone(), startLinfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (astDefs, linfoTV, errOptTV, cachedDefs) = typeviewDefsFromInterfaceFile(pid.clone(), astDefs.clone(), cachedDefs.clone())?;
                    linfo = parseErrorPrevPositionOpt(startChars.clone(), startLinfo.clone(), linfo.clone(), errOptTV.clone(), false)?;
                    linfo = mergeErrors(linfo.clone(), linfoTV.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, astDefs, templDefs, cachedDefs) = definitions(chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone(), cachedDefs.clone())?;
                    Ok((chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "m", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } } } } }, linfo, astDefs, templDefs) => {
                    let mut startChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut errOptTV: Option<ArcStr> = None;
                    let mut startLinfo: LineInfo;
                    let mut linfoTV: LineInfo;
                    let mut isUnqual: bool = false;
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut astDefs = (*astDefs).clone();
                    let mut templDefs = (*templDefs).clone();
                    let mut cachedDefs: Arc<CacheTree::Tree> = cachedDefs.clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (startChars, startLinfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, pid) = pathIdentNoOpt(startChars.clone(), startLinfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, isUnqual) = unqualImportPostfix(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (astDefs, linfoTV, errOptTV, cachedDefs) = typeviewDefsFromTemplateFile(pid.clone(), isUnqual.clone(), astDefs.clone(), cachedDefs.clone())?;
                    linfo = parseErrorPrevPositionOpt(startChars.clone(), startLinfo.clone(), linfo.clone(), errOptTV.clone(), false)?;
                    linfo = mergeErrors(linfo.clone(), linfoTV.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, astDefs, templDefs, cachedDefs) = definitions(chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone(), cachedDefs.clone())?;
                    Ok((chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, astDefs, templDefs) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut td: TplAbsyn::TemplateDef;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut astDefs = (*astDefs).clone();
                    let mut templDefs = (*templDefs).clone();
                    let mut cachedDefs: Arc<CacheTree::Tree> = cachedDefs.clone();
                    (chars, linfo, name, td) = templDef(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, astDefs, templDefs, cachedDefs) = definitions(chars.clone(), linfo.clone(), astDefs.clone(), cons((name.clone(), td.clone()), templDefs.clone()), cachedDefs.clone())?;
                    Ok((chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inAccASTDefs.clone(), inAccTemplDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outASTDefs, outTemplDefs, cachedDefs))
}

/*
unqualImportPostfix:
  '.' '*' => true
  |
  _ => false
*/
pub fn unqualImportPostfix(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, bool)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outIsUnqual: bool = false;
    (outChars, outLineInfo, outIsUnqual) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ".", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "*", tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    Ok((chars.clone(), linfo.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outIsUnqual))
}

//optional, may fail
pub fn typeSig(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::TypeSignature>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTypeSignature: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    (outChars, outLineInfo, outTypeSignature) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (chars, linfo) => {
            let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, ts) = typeSig_base(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars.clone(), linfo.clone(), ts.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outTypeSignature))
}

//must not fail
pub fn typeSigNoOpt(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::TypeSignature>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTypeSignature: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    (outChars, outLineInfo, outTypeSignature) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, ts) = typeSig(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), ts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected a type signature at the position.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: (literal!("#parse error#")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTypeSignature))
}

/*
typeSig_base:
  'list' '<' typeSig:tof '>'  =>  LIST_TYPE(tof)
  |
  'Option' '<' typeSig '>'   =>  OPTION_TYPE(tof)
  |
  'tuple' '<' typeSig:ts  typeSig_restList:restLst  '>'  => TUPLE_TYPE(ts::restLst)
  |
  'array' '<' typeSig:tof '>'  =>  ARRAY_TYPE(tof)
  |
  pathIdent:pid  =>  NAMED_TYPE(pid)  // +specializations for String, Integer, .... => STRING_TYPE(), ...
*/
pub fn typeSig_base(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::TypeSignature>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTypeSignature: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    (outChars, outLineInfo, outTypeSignature) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } } }, linfo) => {
                    let mut tof: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("<")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, tof) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(">")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: tof.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "O", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: chars } } } } } }, linfo) => {
                    let mut tof: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("<")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, tof) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(">")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::TypeSignature::OPTION_TYPE { ofType: tof.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } } }, linfo) => {
                    let mut tof: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut restLst: Arc<metamodelica::List<Arc<TplAbsyn::TypeSignature>>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("<")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, tof) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, restLst) = typeSig_restList(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(">")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::TypeSignature::TUPLE_TYPE { ofTypes: cons(tof.clone(), restLst.clone()) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "y", tail: chars } } } } }, linfo) => {
                    let mut tof: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("<")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, tof) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(">")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::TypeSignature::ARRAY_TYPE { ofType: tof.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, pid) = pathIdent(chars.clone(), linfo.clone())?;
                    ts = typeSigFromPathIdent(pid.clone());
                    Ok((chars.clone(), linfo.clone(), ts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTypeSignature))
}

/*
typeSig_restList:
    ',' typeSig:ts  typeSig_restList:restLst  =>  ts::restLst
    |
    _  => {}
*/
pub fn typeSig_restList(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<Arc<TplAbsyn::TypeSignature>>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTypeSignatureList: Arc<metamodelica::List<Arc<TplAbsyn::TypeSignature>>> = metamodelica::nil();
    (outChars, outLineInfo, outTypeSignatureList) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut tsLst: Arc<metamodelica::List<Arc<TplAbsyn::TypeSignature>>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, tsLst) = typeSig_restList(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons(ts.clone(), tsLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTypeSignatureList))
}

pub fn typeSigFromPathIdent(mut inPathIdent: Arc<TplAbsyn::PathIdent>) -> Arc<TplAbsyn::TypeSignature> {
    let mut outTypeSignature: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    outTypeSignature = (::match_deref::match_deref! { match &(inPathIdent.clone()) {
        Deref @ TplAbsyn::PathIdent::IDENT { ident: Deref @ "String" } => Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE),
        Deref @ TplAbsyn::PathIdent::IDENT { ident: Deref @ "Integer" } => Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE),
        Deref @ TplAbsyn::PathIdent::IDENT { ident: Deref @ "Real" } => Arc::new(crate::TplAbsyn::TypeSignature::REAL_TYPE),
        Deref @ TplAbsyn::PathIdent::IDENT { ident: Deref @ "Boolean" } => Arc::new(crate::TplAbsyn::TypeSignature::BOOLEAN_TYPE),
        _ => Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: inPathIdent.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTypeSignature
}

/*
publicProtected:
  'public' => true
  |
  'protected' => false
  |
  _ => true
*/
pub fn publicProtected(mut inChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, bool)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outIsDefault: bool = false;
    (outChars, outIsDefault) = 'mc: {
        let __mc_input = inChars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "b", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: chars } } } } } } => {
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } } } } } } } } => {
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outIsDefault))
}

/*
stringComment:
  '"' stringCommentRest
  |
  _
*/
pub fn stringComment(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::List::Cons { head: Deref @ "\"", tail: chars }, startLinfo) => {
                    let mut linfo: LineInfo;
                    let mut optErr: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), startLinfo.clone())?;
                    linfo = parseErrorPrevPositionOpt(startChars.clone(), startLinfo.clone(), linfo.clone(), optErr.clone(), true)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
stringCommentRest:
  '\\"' stringCommentRest
  |
  '\\' stringCommentRest
  |
  ~'"' stringCommentRest
  |
  '"'
*/
pub fn stringCommentRest(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Option<ArcStr>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outError: Option<ArcStr> = None;
    (outChars, outLineInfo, outError) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\"", tail: chars }, linfo) => {
                    Ok((chars.clone(), linfo.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::List::Cons { head: Deref @ "\"", tail: chars } }, linfo) => {
                    let mut optErr: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), optErr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: chars } }, linfo) => {
                    let mut optErr: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), optErr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut optErr: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), optErr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::List::Cons { head: _, tail: chars }, linfo) => {
                    let mut optErr: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        (_, _) = unwrap_break_err!(newLine(startChars.clone(), linfo.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), optErr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, linfo) => {
                    let mut strErr: ArcStr = arcstr::literal!("");
                    strErr = (literal!("Unmatched \" \" comment - reached end of file.")).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.stringCommentRest - ")); __mm_s.push_str(&*strErr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((metamodelica::nil(), linfo.clone(), Some((strErr.clone()).clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outError))
}

pub fn semicolon(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ";", tail: chars }, linfo) => {
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected semicolon ';' at the position.")).clone(), false)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!! TplParser.semicolon failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
interfacePackage(astDefs):
  'interface' 'package'  pathIdent:pid  stringComment
    typeviewDefs(astDefs):ads
  endDefPathIdent(pid)
  =>   (pid, ads)
*/
pub fn interfacePackage(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inAccASTDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::PathIdent>, Arc<metamodelica::List<TplAbsyn::ASTDef>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outPid: Arc<TplAbsyn::PathIdent>;
    let mut outAccASTDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = metamodelica::nil();
    (outChars, outLineInfo, outPid, outAccASTDefs) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inAccASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _) => {
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut astDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![(literal!("i")).clone(), (literal!("n")).clone(), (literal!("t")).clone(), (literal!("e")).clone(), (literal!("r")).clone(), (literal!("f")).clone(), (literal!("a")).clone(), (literal!("c")).clone(), (literal!("e")).clone()], true)?;
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![(literal!("p")).clone(), (literal!("a")).clone(), (literal!("c")).clone(), (literal!("k")).clone(), (literal!("a")).clone(), (literal!("g")).clone(), (literal!("e")).clone()], true)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, pid) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, astDefs) = typeviewDefs(chars.clone(), linfo.clone(), inAccASTDefs.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = endDefPathIdent(chars.clone(), linfo.clone(), pid.clone())?;
                    Ok((chars.clone(), linfo.clone(), pid.clone(), astDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!!Parse error - TplParser.interfacePackage failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outPid, outAccASTDefs))
}

/*
typeviewDefs(astDefs):
  absynDef:ad  typeviewDefs(ad::astDefs):ads => ads
  |
  _ => astDefs
*/
pub fn typeviewDefs(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inAccASTDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<TplAbsyn::ASTDef>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outASTDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = metamodelica::nil();
    (outChars, outLineInfo, outASTDefs) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inAccASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, astDefs) => {
                    let mut ad: TplAbsyn::ASTDef;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut astDefs = (*astDefs).clone();
                    (chars, linfo, ad) = absynDef(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, astDefs) = typeviewDefs(chars.clone(), linfo.clone(), cons(ad.clone(), astDefs.clone()))?;
                    Ok((chars.clone(), linfo.clone(), astDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inAccASTDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outASTDefs))
}

/*
absynDef:
  publicProtected:isD  'package' pathIdent:pid  stringComment
    absynTypes:types
  endDefPathIdent(pid)
  =>  AST_DEF(pid, isD, types)
*/
pub fn absynDef(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, TplAbsyn::ASTDef)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outASTDef: TplAbsyn::ASTDef;
    (outChars, outLineInfo, outASTDef) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (chars, linfo) => {
            let mut isD: bool = false;
            let mut pid: Arc<TplAbsyn::PathIdent>;
            let mut types: Arc<metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>> = metamodelica::nil();
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, isD) = publicProtected(chars.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "k", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "g", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: __pa0 } } } } } } } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            chars = __pa0.clone();
            afterKeyword(chars.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, pid) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, types) = absynTypes(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = endDefPathIdent(chars.clone(), linfo.clone(), pid.clone())?;
            (chars.clone(), linfo.clone(), TplAbsyn::ASTDef { importPackage: pid.clone(), isDefault: isD.clone(), types: types.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outASTDef))
}

/*
//not optional, must not fail
endDefPathIdent(pid):
  'end' pathIdent:pidEnd ';' // pid == pidEnd | warning
*/
pub fn endDefPathIdent(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inPathIdentToMatch: Arc<TplAbsyn::PathIdent>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inPathIdentToMatch.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } }, linfo, pidToMatch) => {
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, pid) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    let true = (pid.clone() == pidToMatch.clone()) else { bail!("pattern mismatch") };
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } }, linfo, pidToMatch) => {
                    let mut startChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut startLinfo: LineInfo;
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (startChars, startLinfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, pid) = pathIdentNoOpt(startChars.clone(), startLinfo.clone())?;
                    let false = (pid.clone() == pidToMatch.clone()) else { bail!("pattern mismatch") };
                    linfo = parseErrorPrevPosition(startChars.clone(), startLinfo.clone(), linfo.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unmatched ident for 'end'. Expected '")); __mm_s.push_str(&*TplAbsyn::pathIdentString(pidToMatch.clone())?); __mm_s.push_str(&*literal!("', but '")); __mm_s.push_str(&*TplAbsyn::pathIdentString(pid.clone())?); __mm_s.push_str(&*literal!("' found instead.")); ArcStr::from(__mm_s) }).clone(), false)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _) => {
                    let mut linfo = (*linfo).clone();
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), cons((literal!("e")).clone(), cons((literal!("n")).clone(), cons((literal!("d")).clone(), metamodelica::nil()))))?) {
                        (_, false) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected 'end' keyword at the position.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _) => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("!!!Parse error - TplParser.endDefPathIdent failed.\n")).clone())?;
                    }
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
//not optional ... must not fail
endDefIdent(id):
  'end' identifier:idEnd ';' // id == idEnd | warning
*/
pub fn endDefIdent(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inIdentToMatch: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inIdentToMatch.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } }, linfo, idToMatch) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    let true = ((id.clone()).clone() == idToMatch.clone()) else { bail!("pattern mismatch") };
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } }, linfo, idToMatch) => {
                    let mut startChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut startLinfo: LineInfo;
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (startChars, startLinfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(startChars.clone(), startLinfo.clone())?;
                    let false = ((id.clone()).clone() == idToMatch.clone()) else { bail!("pattern mismatch") };
                    linfo = parseErrorPrevPosition(startChars.clone(), startLinfo.clone(), linfo.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unmatched ident for 'end'. Expected '")); __mm_s.push_str(&*idToMatch.clone()); __mm_s.push_str(&*literal!("', but '")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!("' found instead.")); ArcStr::from(__mm_s) }).clone(), false)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _) => {
                    let mut linfo = (*linfo).clone();
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), cons((literal!("e")).clone(), cons((literal!("n")).clone(), cons((literal!("d")).clone(), metamodelica::nil()))))?) {
                        (_, false) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected 'end' keyword at the position.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("!!!Parse error - TplParser.endDefIdent failed.\n")).clone())?;
                    }
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
absynTypes:
  absynType:(id,ti)  absynTypes:types  => (id,ti) :: types
  |
  _ => {}
*/
pub fn absynTypes(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTypes: Arc<metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>> = metamodelica::nil();
    (outChars, outLineInfo, outTypes) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut idti: (ArcStr, TplAbsyn::TypeInfo);
                    let mut types: Arc<metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, idti) = absynType(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, types) = absynTypes(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons(idti.clone(), types.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTypes))
}

/*
absynType:
  'uniontype' identifier:id  stringComment
      recordTags(id):rtags
  => (id, TI_UNION_TYPE(rtags))
  |
  recordType:(id,fields)
  => (id, TI_RECORD_TYPE(fields))
  |
  'function' identifier:id  stringComment
    inputFunArgs:inArgs
    outputFunArgs:outArgs
  endDefIdent(id)
  => (id, TI_FUN_TYPE(inArgs,outArgs))
  |
  'constant'  typeSig:ts  identifier:id  stringComment  ';'
  => (id, TI_CONST_TYPE(ts))
  |
  'type' identifier:id '=' typeSig:ts stringComment  ';'
  => (id, TI_ALIAS_TYPE(ts))
*/
pub fn absynType(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (ArcStr, TplAbsyn::TypeInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outType: (ArcStr, TplAbsyn::TypeInfo);
    (outChars, outLineInfo, outType) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "y", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } } } } } } }, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut rtags: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, rtags) = recordTags(chars.clone(), linfo.clone(), (id.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_UNION_TYPE { recTags: rtags.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut fields: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let (__pa0, __pa1, (__pa2, __pa3)) = recordType(chars.clone(), linfo.clone())?;
                    chars = __pa0.clone();
                    linfo = __pa1.clone();
                    id = __pa2.clone();
                    fields = __pa3.clone();
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_RECORD_TYPE { fields: fields.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "f", tail: Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: chars } } } } } } } }, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut inargs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut outargs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut tyvars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, tyvars) = typeVars(chars.clone(), linfo.clone(), metamodelica::nil())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, inargs) = inputFunArgs(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, outargs) = outputFunArgs(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, tyvars) = typeVars(chars.clone(), linfo.clone(), tyvars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = endDefIdent(chars.clone(), linfo.clone(), (id.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_FUN_TYPE { inArgs: inargs.clone(), outArgs: outargs.clone(), tyVars: tyvars.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } } } } } } }, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_CONST_TYPE { constType: ts.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "y", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } }, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("=")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: ts.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outType))
}

/*
recordType:
  'record' identifier:id  stringComment
      typeDecls:tids
  'end' identifier:idEnd ';' // id == idEnd
  => (id,tids)
*/
pub fn recordType(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (ArcStr, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outRecordType: (ArcStr, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>);
    (outChars, outLineInfo, outRecordType) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } } } } }, linfo) => {
            let mut id: ArcStr = arcstr::literal!("");
            let mut fields: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            afterKeyword(chars.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, fields) = typeDecls(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = endDefIdent(chars.clone(), linfo.clone(), (id.clone()).clone())?;
            (chars.clone(), linfo.clone(), (id.clone(), fields.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outRecordType))
}

/*
typeDecls:
  typeSig:ts  identifier:id  stringComment ';'
  typeDecls:tids
  => (id,ts) :: tids
  |
  _ => {}
*/
pub fn typeDecls(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTypeDecls: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
    (outChars, outLineInfo, outTypeDecls) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } }, linfo) => {
                    afterKeyword(chars.clone())?;
                    Ok((startChars.clone(), linfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut fields: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, ts) = typeSig(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, fields) = typeDecls(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons((id.clone(), ts.clone()), fields.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTypeDecls))
}

/*
recordTags:
  recordType:(id,tids)  recordTags:rtags  => (id,tids) :: rtags
  |
  _ => {}
*/
pub fn recordTags(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut id: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outRecordTags: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)>> = metamodelica::nil();
    (outChars, outLineInfo, outRecordTags) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut rtag: (ArcStr, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>);
                    let mut rtags: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, rtag) = recordType(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, rtags) = recordTags(chars.clone(), linfo.clone(), (id.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), cons(rtag.clone(), rtags.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = endDefIdent(chars.clone(), linfo.clone(), (id.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("!!!Parse error - TplParser.recordTags failed at ")); __mm_s.push_str(&*inLineInfo.parseInfo.fileName.clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", inLineInfo.lineNumber.clone()))); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outRecordTags))
}

/*
inputFunArgs:
  'input' typeSig:ts  identifier:id  stringComment
  inputFunArgs:iargs
  => (id,ts) :: iargs
  |
  _ => {}
*/
pub fn inputFunArgs(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTypedIdents: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
    (outChars, outLineInfo, outTypedIdents) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } } } }, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut inargs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, inargs) = inputFunArgs(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons((id.clone(), ts.clone()), inargs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTypedIdents))
}

/*
outputFunArgs:
  'output' typeSig:ts  identifier:id  stringComment ';'
  outputFunArgs:oargs
  => (id,ts) :: oargs
  |
  _ => {}
*/
pub fn outputFunArgs(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTypedIdents: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
    (outChars, outLineInfo, outTypedIdents) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } } } } }, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut outargs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, outargs) = outputFunArgs(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons((id.clone(), ts.clone()), outargs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTypedIdents))
}

/*
typeVars(tyvars):
  'replaceable' 'type'  identifier:id  'subtypeof' 'Any' ';'
  typeVars(id :: tyvars):tyvars
  => tyvars
  |
  _ => tyvars
*/
pub fn typeVars(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inTyVars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<ArcStr>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTyVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outChars, outLineInfo, outTyVars) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inTyVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "b", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } } } } } } } } }, linfo, tyvars) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut tyvars = (*tyvars).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![(literal!("t")).clone(), (literal!("y")).clone(), (literal!("p")).clone(), (literal!("e")).clone()], true)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![(literal!("s")).clone(), (literal!("u")).clone(), (literal!("b")).clone(), (literal!("t")).clone(), (literal!("y")).clone(), (literal!("p")).clone(), (literal!("e")).clone(), (literal!("o")).clone(), (literal!("f")).clone()], true)?;
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![(literal!("A")).clone(), (literal!("n")).clone(), (literal!("y")).clone()], true)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (chars, linfo, tyvars) = typeVars(chars.clone(), linfo.clone(), cons((id.clone()).clone(), tyvars.clone()))?;
                    Ok((chars.clone(), linfo.clone(), tyvars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTyVars))
}

/*
templDef:
    'template' identifier:name
      '(' templArgs:args ')' stringComment
      templDef_Templ:(exp,lesc,resc)
    endDefIdent(name)
      =>  (name, TEMPLATE_DEF(args,lesc,resc,exp))
    |
    'constant' constantType:ctype  identifier:name templDef_Const:td //check ctype
      stringComment ';'
      => (name, td)
*/
pub fn templDef(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, ArcStr, TplAbsyn::TemplateDef)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTemplName: ArcStr = arcstr::literal!("");
    let mut outTemplDef: TplAbsyn::TemplateDef;
    (outChars, outLineInfo, outTemplName, outTemplDef) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "m", tail: Deref @ metamodelica::List::Cons { head: Deref @ "p", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } } } } } }, linfo) => {
            let mut lesc: ArcStr = arcstr::literal!("");
            let mut resc: ArcStr = arcstr::literal!("");
            let mut name: ArcStr = arcstr::literal!("");
            let mut args: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
            let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            afterKeyword(chars.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("(")).clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, args) = templArgs(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, exp, lesc, resc) = templDef_Templ(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = endDefIdent(chars.clone(), linfo.clone(), (name.clone()).clone())?;
            (chars.clone(), linfo.clone(), name.clone(), TplAbsyn::TemplateDef::TEMPLATE_DEF { args: args.clone(), lesc: (lesc.clone()).clone(), resc: (resc.clone()).clone(), exp: exp.clone() })
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } } } } } } }, linfo) => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut td: TplAbsyn::TemplateDef;
            let mut ctype: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
            let mut ctypeLit: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            afterKeyword(chars.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, ctype) = constantType(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, td, ctypeLit) = templDef_Const(chars.clone(), linfo.clone())?;
            (chars, linfo) = checkConstantType(chars.clone(), linfo.clone(), ctype.clone(), ctypeLit.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = stringComment(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
            (chars.clone(), linfo.clone(), name.clone(), td.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outTemplName, outTemplDef))
}

/*
templDef_Const:
  '=' stringConstant:strRevList
    =>  STR_TOKEN_DEF(makeStrTokFromRevStrList(strRevList))
  |
  '=' literalConstant:(str,litType)
    =>  LITERAL_DEF(str, litType)
*/
pub fn templDef_Const(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, TplAbsyn::TemplateDef, Arc<TplAbsyn::TypeSignature>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTemplDef: TplAbsyn::TemplateDef;
    let mut outConstType: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    (outChars, outLineInfo, outTemplDef, outConstType) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "=", tail: chars }, linfo) => {
                    let mut strRevList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut st: Arc<Tpl::StringToken> = Arc::new(Tpl::StringToken::ST_NEW_LINE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, strRevList) = stringConstant(chars.clone(), linfo.clone())?;
                    st = makeStrTokFromRevStrList(strRevList.clone())?;
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplateDef::STR_TOKEN_DEF { value: st.clone() }, Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "=", tail: chars }, linfo) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut litType: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, r#str, litType) = literalConstant(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplateDef::LITERAL_DEF { value: (r#str.clone()).clone(), litType: litType.clone() }, litType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "=", tail: chars }, linfo) => {
                    let mut litType: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected a constant definition after the '='.")).clone(), true)?;
                    litType = Arc::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: (literal!("#Error#")).clone() });
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplateDef::LITERAL_DEF { value: (literal!("#error#")).clone(), litType: litType.clone() }, litType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut litType: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected a constant definition after the position.")).clone(), true)?;
                    litType = Arc::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: (literal!("#Error#")).clone() });
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplateDef::TEMPLATE_DEF { args: metamodelica::nil(), lesc: (literal!("")).clone(), resc: (literal!("")).clone(), exp: (Arc::new(crate::TplAbsyn::ExpressionBase::ERROR_EXP), dummySourceInfo.clone()) }, litType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTemplDef, outConstType))
}

/*
constantType:
  'String'  => STRING_TYPE()
  'Integer' => INTEGER_TYPE()
  'Real'    => REAL_TYPE()
  'Boolean' => BOOLEAN_TYPE()
*/
pub fn constantType(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::TypeSignature>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outConstType: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    (outChars, outLineInfo, outConstType) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "S", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "g", tail: chars } } } } } }, linfo) => {
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "I", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "g", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: chars } } } } } } }, linfo) => {
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "R", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: chars } } } }, linfo) => {
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(crate::TplAbsyn::TypeSignature::REAL_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "B", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: chars } } } } } } }, linfo) => {
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(crate::TplAbsyn::TypeSignature::BOOLEAN_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected 'String', 'Integer', 'Real' or 'Boolean' type specification for the constant definition after the position.")).clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: (literal!("#Error#")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outConstType))
}

pub fn checkConstantType(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inConstType: Arc<TplAbsyn::TypeSignature>, mut inConstTypeLiteral: Arc<TplAbsyn::TypeSignature>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone(), inConstType.clone(), inConstTypeLiteral.clone())) {
        (chars, linfo, Deref @ TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: _ }, _) => {
            (chars.clone(), linfo.clone())
        },
        (chars, linfo, _, Deref @ TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: _ }) => {
            (chars.clone(), linfo.clone())
        },
        (chars, linfo, ctype, litType) if (!(ctype.clone() == litType.clone())) => {
            let mut linfo = (*linfo).clone();
            linfo = parseError(chars.clone(), linfo.clone(), (literal!("Declared constant type and the type of the constant's definition literal are different.")).clone(), false)?;
            (chars.clone(), linfo.clone())
        },
        _ => {
            (inChars.clone(), inLineInfo.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo))
}

/*
templDef_Templ:
  '::='  expression(LEsc = '<',REsc = '>'):exp   => (exp,'<','>')
  ///|
  //'$$='  expression(LEsc = '$',REsc = '$'):exp   => (exp,'$','$')
*/
pub fn templDef_Templ(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo), ArcStr, ArcStr)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut outLeftEsc: ArcStr = arcstr::literal!("");
    let mut outRightEsc: ArcStr = arcstr::literal!("");
    (outChars, outLineInfo, outExpression, outLeftEsc, outRightEsc) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ":", tail: Deref @ metamodelica::List::Cons { head: Deref @ ":", tail: Deref @ metamodelica::List::Cons { head: Deref @ "=", tail: chars } } }, linfo) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), exp.clone(), literal!("<"), literal!(">")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(chars.clone()) {
                            Deref @ metamodelica::List::Cons { head: Deref @ ":", tail: Deref @ metamodelica::List::Cons { head: Deref @ ":", tail: Deref @ metamodelica::List::Cons { head: Deref @ "=", tail: _ } } } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected '::=' symbol before a template definition body at the position.")).clone(), false)?;
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), exp.clone(), literal!("<"), literal!(">")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!!Parse error - TplParser.templDef_Templ failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression, outLeftEsc, outRightEsc))
}

/*
templArgs:
    //TODO: to be TEXT_REF ... for now only syntax
    'Text' '&' identifier:name  templArgs_rest:args  =>  (name,TEXT_TYPE())::args
    |
    typeSig:ts  identifier:name  templArgs_rest:args  =>  (name,ts)::args
    |
    _  => {}
*/
pub fn templArgs(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outArgs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
    (outChars, outLineInfo, outArgs) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "T", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "x", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } } }, linfo) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut args: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "&", tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, args) = templArgs_rest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons((name.clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE)), args.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut args: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, ts) = typeSig(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, args) = templArgs_rest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons((name.clone(), ts.clone()), args.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outArgs))
}

/*
templArg0:
    typeSig:ts  implicitArgName:name  =>  (name,ts)

*/
/*
public function templArg0
  input list<String> inChars;
  input LineInfo inLineInfo;

  output list<String> outChars;
  output LineInfo outLineInfo;
  output tuple<TplAbsyn.Ident, TplAbsyn.TypeSignature> outArg;
algorithm
  (outChars, outLineInfo, outArg) := matchcontinue (inChars, inLineInfo)
    local
      String lesc, resc;
      list<String> chars;
      LineInfo linfo;
      Boolean isD;
      TplAbsyn.PathIdent pid;
      list<tuple<TplAbsyn.Ident, TplAbsyn.TypeInfo>> types;
      TplAbsyn.Ident name;
      TplAbsyn.TemplateDef td;
      TplAbsyn.TypedIdents args;
      TplAbsyn.Expression exp;
      TplAbsyn.TypeSignature ts;

    case (chars, linfo)
      algorithm
        (chars, linfo, ts) = typeSig(chars, linfo);
        (chars, linfo) = interleave(chars, linfo);
        (chars, name) = implicitArgName(chars);
      then (chars, linfo, (name,ts));

  end matchcontinue;
end templArg0;
*/
/*
implicitArgName:
      IDENT:id  => id  //maybe 'it' explicitly
      |
      _  => 'it'


public function implicitArgName
  input list<String> inChars;

  output list<String> outChars;
  output TplAbsyn.Ident outArgName;
algorithm
  (outChars, outArgName) := matchcontinue (inChars)
    local
      String lesc, resc;
      list<String> chars;
      Boolean isD;
      TplAbsyn.PathIdent pid;
      list<tuple<TplAbsyn.Ident, TplAbsyn.TypeInfo>> types;
      TplAbsyn.Ident name;
      TplAbsyn.TemplateDef td;
      TplAbsyn.TypedIdents args;
      TplAbsyn.Expression exp;
      TplAbsyn.TypeSignature ts;

    case (chars)
      algorithm
        (chars, name) = identifier(chars);
      then (chars, name);

    else (inChars, "it");

  end matchcontinue;
end implicitArgName;
*/
/*
templArgs_rest
  ',' typeSig:ts  argName_nonIt:name  templArgs_rest:rest  =>  (name,ts)::rest
  |
  _  => {}
*/
pub fn templArgs_rest(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outArgs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
    (outChars, outLineInfo, outArgs) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut args: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "T", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "x", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: __pa0 } } } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa2 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "&", tail: __pa2 } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa2.clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, args) = templArgs_rest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons((name.clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE)), args.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut rest: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::TypeSignature>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, rest) = templArgs_rest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons((name.clone(), ts.clone()), rest.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outArgs))
}

/*
argName_nonIt:
      'it'  =>  Error("Implicit argument 'it' appeared at non-fist position in the template argument list. 'it' can be explicitly only as the first argument.")
      |
      IDENT:id  => id
*/
/*
public function argName_nonIt
  input list<String> inChars;
  input LineInfo inLineInfo;

  output list<String> outChars;
  output LineInfo outLineInfo;
  output TplAbsyn.Ident outArgName;
algorithm
  (outChars, outLineInfo, outArgName) := matchcontinue (inChars, inLineInfo)
    local
      String lesc, resc;
      list<String> chars, startChars;
      LineInfo linfo;
      Boolean isD;
      TplAbsyn.PathIdent pid;
      list<tuple<TplAbsyn.Ident, TplAbsyn.TypeInfo>> types;
      TplAbsyn.Ident name;
      TplAbsyn.TemplateDef td;
      TplAbsyn.TypedIdents args;
      TplAbsyn.Expression exp;
      TplAbsyn.TypeSignature ts;
      TplAbsyn.TypedIdents rest;

    case (startChars as ("i"::"t":: chars), linfo)
      algorithm
        afterKeyword(chars);
        (linfo) = parseError(startChars, linfo, "Implicit argument 'it' appeared at non-first position in the template argument list. 'it' can be explicitly only as the first argument.",
        false);
        //true = Flags.isSet(Flags.FAILTRACE); Debug.trace("Parse error - implicit argument 'it' appeared at non-first position in the template argument list. 'it' can be explicitly only as the first argument.\n");
      then (chars, linfo, "#Error-displaced it#");

    case (chars, linfo)
      algorithm
        (chars, linfo, name) = identifierNoOpt(chars, linfo);
      then (chars, linfo, name);

  end matchcontinue;
end argName_nonIt;
*/
/*
expression(lesc,resc):
  expressionNoOptions(lesc,resc):exp  escapedOptions:opts
    => makeEscapedExp(exp, opts)
*/
pub fn expression(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr, mut isOptional: bool) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone(), isOptional.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc, _) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut opts: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = metamodelica::nil();
                    let mut indexOffsetOption: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp, indexOffsetOption) = expressionNoOptions(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo, opts) = escapedOptions(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    opts = listAppend(indexOffsetOption.clone(), opts.clone());
                    exp = makeEscapedExp(chars.clone(), linfo.clone(), exp.clone(), opts.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _, _, false) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expecting an expression - not able to parse from this point.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(crate::TplAbsyn::ExpressionBase::ERROR_EXP), dummySourceInfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression))
}

pub fn makeEscapedExp(mut inEndChars: Arc<metamodelica::List<ArcStr>>, mut inEndLineInfo: LineInfo, mut inExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo), mut inOptions: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>>) -> Result<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)> {
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    outExpression = (::match_deref::match_deref! { match &((inEndChars.clone(), inEndLineInfo.clone(), inExpression.clone(), inOptions.clone())) {
        (_, _, exp, Deref @ metamodelica::List::Nil) => {
            exp.clone()
        },
        (_, _, exp, opts @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            sinfo = tplSourceInfo(startPositionFromExp(exp.clone())?, inEndChars.clone(), inEndLineInfo.clone())?;
            (Arc::new(TplAbsyn::ExpressionBase::ESCAPED { exp: exp.clone(), options: opts.clone() }), sinfo.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpression)
}

/*
escapedOptions(lesc,resc):
  ';' identifier:id  escOptionExp(lesc,resc):expOpt  escapedOptions(lesc,resc):opts
  => (id, expOpt) :: opts
  |
  _ => {}

*/
pub fn escapedOptions(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outOptions: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = metamodelica::nil();
    (outChars, outLineInfo, outOptions) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ";", tail: chars }, linfo, lesc, resc) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut expOpt: Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)> = None;
                    let mut opts: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, expOpt) = escOptionExp(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, opts) = escapedOptions(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), cons((id.clone(), expOpt.clone()), opts.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outOptions))
}

/*
escOptionExp(lesc,resc):
  '=' expressionLet(lesc,resc):exp
    => SOME(exp)
  |
  _ => NONE
*/
pub fn escOptionExp(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpOption: Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)> = None;
    (outChars, outLineInfo, outExpOption) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "=", tail: chars }, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), Some(exp.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpOption))
}

/* not optional
expressionNoOptions(lesc,resc):
  expressionLet(lesc,resc):expLet  mapTailOpt(lesc,resc,expLet):exp
    => exp
*/
pub fn expressionNoOptions(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo), Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut outIndexOffsetOption: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = metamodelica::nil();
    (outChars, outLineInfo, outExpression, outIndexOffsetOption) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone())) {
        (chars, linfo, lesc, resc) => {
            let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, exp, outIndexOffsetOption) = mapTailOpt(chars.clone(), linfo.clone(), exp.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            (chars.clone(), linfo.clone(), exp.clone(), outIndexOffsetOption.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outExpression, outIndexOffsetOption))
}

/*
mapTailOpt(headExp,lesc,resc):
  '|>' matchBinding:mexp
  indexedByOpt:idxNmOpt
  '=>' expressionLet(lesc,resc):exp  =>  MAP(headExp,mexp,exp)
  |
  _ => headExp
*/
pub fn mapTailOpt(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inHeadExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo), mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo), Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut outIndexOffsetOption: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = metamodelica::nil();
    (outChars, outLineInfo, outExpression, outIndexOffsetOption) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inHeadExpression.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "|", tail: Deref @ metamodelica::List::Cons { head: Deref @ ">", tail: chars } }, linfo, headExp, lesc, resc) => {
                    let mut idxNmOpt: Option<ArcStr> = None;
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut outIndexOffsetOption: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = outIndexOffsetOption.clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, idxNmOpt, outIndexOffsetOption) = indexedByOpt(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("=")).clone())?;
                    (chars, linfo) = expectChar(chars.clone(), linfo.clone(), (literal!(">")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    sinfo = tplSourceInfo(startPositionFromExp(headExp.clone())?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::MAP { argExp: headExp.clone(), ofBinding: mexp.clone(), mapExp: exp.clone(), hasIndexIdentOpt: idxNmOpt.clone() }), sinfo.clone()), outIndexOffsetOption.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inHeadExpression.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression, outIndexOffsetOption))
}

/*
indexedByOpt:
  'hasindex' identifier:id
    => SOME(id)
  |
  _ => NONE
*/
pub fn indexedByOpt(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Option<ArcStr>, Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outIndexNameOpt: Option<ArcStr> = None;
    let mut outIndexOffsetOption: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = metamodelica::nil();
    (outChars, outLineInfo, outIndexNameOpt, outIndexOffsetOption) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "h", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "x", tail: chars } } } } } } } }, linfo, lesc, resc) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut outIndexOffsetOption: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = outIndexOffsetOption.clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, outIndexOffsetOption) = fromOpt(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), Some((id.clone()).clone()), outIndexOffsetOption.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), None, metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outIndexNameOpt, outIndexOffsetOption))
}

/*
fromOpt:
  'fromindex' expression_base:expFrom
    => { ("$indexOffset", SOME(expFrom)) }
  |
  _ => {}
*/
pub fn fromOpt(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outIndexOffsetOption: Arc<metamodelica::List<(ArcStr, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)>> = metamodelica::nil();
    (outChars, outLineInfo, outIndexOffsetOption) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "f", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "m", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "x", tail: chars } } } } } } } } }, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), list![(arcstr::literal!(TplAbsyn::indexOffsetOptionId), Some(exp.clone()))]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "f", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "m", tail: chars } } } }, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Keyword 'from' was changed to 'fromindex', please update your source code here.")).clone(), false)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), list![(arcstr::literal!(TplAbsyn::indexOffsetOptionId), Some(exp.clone()))]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outIndexOffsetOption))
}

/*
expressionLet(lesc,resc):
  'let' letExp(lesc,resc):lexp  concatLetExp_rest(lesc,resc):expLst
     => TEMPLATE(lexp::expLst}, "let", ""); //TODO: should be a LET_EXPRESSION()
  |
  expressionMatch(lesc,resc):exp
*/
pub fn expressionLet(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: startChars } } }, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut lexp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    afterKeyword(startChars.clone())?;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo, lexp) = letExp(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 3)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::LET { letExp: lexp.clone(), exp: exp.clone() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = expressionMatch(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
concatLetExp_rest(lesc,resc):
  'let' letExp(lesc,resc):lexp  concatLetExp_rest(lesc,resc):expLst
    =>  lexp::expLst
  |
  expression(lesc,resc):exp
    => {exp}
*/
/*
public function concatLetExp_rest
  input list<String> inChars;
  input LineInfo inLineInfo;
  input String inLeftEsc;
  input String inRightEsc;

  output list<String> outChars;
  output LineInfo outLineInfo;
  output list<TplAbsyn.Expression> outExpressionList;
algorithm
  (outChars, outLineInfo, outExpressionList) := matchcontinue (inChars, inLineInfo, inLeftEsc, inRightEsc)
    local
      list<String> chars;
      LineInfo linfo;
      String c, lesc, resc;
      Boolean isD;
      TplAbsyn.Ident id;
      TplAbsyn.PathIdent name;
      TplAbsyn.TypedIdents fields,inargs,outargs;
      TplAbsyn.TypeSignature ts;
      Tpl.StringToken st;
      TplAbsyn.Expression exp, lexp;
      list<TplAbsyn.Expression> expLst;
      TplAbsyn.MatchingExp mexp;

    case ("l"::"e"::"t":: chars, linfo, lesc, resc)
      algorithm
        afterKeyword(chars);
        (chars, linfo) = interleave(chars, linfo);
        (chars, linfo, lexp) = letExp(chars, linfo, lesc, resc);
        (chars, linfo) = interleave(chars, linfo);
        (chars, linfo, expLst) = concatLetExp_rest(chars, linfo, lesc, resc);
      then (chars, linfo, lexp::expLst);

    case (chars, linfo, lesc, resc)
      algorithm
        (chars, linfo, exp) = expressionMatch(chars, linfo, lesc, resc);
      then (chars, linfo, {exp});

  end matchcontinue;
end concatLetExp_rest;
*/
/*
must not fail - not optional, at least one must match
letExp(lesc,resc):
  '&' identifier:id '=' 'buffer' expression(lesc,resc):exp
       => TEXT_CREATE(id,exp)
  |
  '&' identifier:id '+=' expression(lesc,resc):exp
       => TEXT_ADD(id,exp)
  |
  '()' '=' pathIdent:name  funCall(name,lesc,resc):exp
       =>  exp //TODO: noRetCall expression should be here
  |
  identifier:id '=' expression(lesc,resc):exp
    => TEXT_CREATE(id,exp) //TODO: !! a HACK for now

  //TODO:
  |
  letBinding:bd '=' expression(lesc,resc):exp
    =>  LET_BINDING(bd, exp)
*/
pub fn letExp(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "&", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
                    (chars, id) = identifier(chars.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 1)?, chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "=", tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![(literal!("b")).clone(), (literal!("u")).clone(), (literal!("f")).clone(), (literal!("f")).clone(), (literal!("e")).clone(), (literal!("r")).clone()], false)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::TEXT_CREATE { name: (id.clone()).clone(), exp: exp.clone() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "&", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
                    (chars, id) = identifier(chars.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 1)?, chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "+", tail: Deref @ metamodelica::List::Cons { head: Deref @ "=", tail: __pa0 } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::TEXT_ADD { name: (id.clone()).clone(), exp: exp.clone() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "&", tail: startChars }, startLInfo, _, _) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo, _) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expecting a '=' or '+=' text variable creation/addition (&var = exp or &var += exp) at the position.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(crate::TplAbsyn::ExpressionBase::ERROR_EXP), dummySourceInfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: Deref @ metamodelica::List::Cons { head: Deref @ ")", tail: startChars } }, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut name: Arc<TplAbsyn::PathIdent>;
                    let mut args: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo) = interleaveExpectChar(startChars.clone(), startLInfo.clone(), (literal!("=")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, name) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(funCall(chars.clone(), linfo.clone(), name.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?) {
                        (__pa0, __pa1, Deref @ TplAbsyn::ExpressionBase::FUN_CALL { name: __pa2, args: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    linfo = __pa1.clone();
                    name = __pa2.clone();
                    args = __pa3.clone();
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 2)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::NORET_CALL { name: name.clone(), args: args.clone() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: Deref @ metamodelica::List::Cons { head: Deref @ ")", tail: startChars } }, startLInfo, _, _) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    (chars, linfo) = interleaveExpectChar(startChars.clone(), startLInfo.clone(), (literal!("=")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, _) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expecting a non-return function call( let () = [package.]funName(args,...) ) at the position.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(crate::TplAbsyn::ExpressionBase::ERROR_EXP), dummySourceInfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo, id) = identifierNoOpt(startChars.clone(), startLInfo.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 0)?, chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("=")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::TEXT_CREATE { name: (id.clone()).clone(), exp: exp.clone() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("!!!Parse error - TplParser.letExp failed.\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
expressionMatch(lesc,resc):
  matchExp(lesc,resc):exp
    => exp
  |
  expressionIf(lesc,resc):exp
    => exp
*/
pub fn expressionMatch(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = matchExp(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = expressionIf(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
expressionIf(lesc,resc):
  conditionExp(lesc,resc):exp
    => exp
  |
  expressionPlus(lesc,resc):exp
    => exp
*/
pub fn expressionIf(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = conditionExp(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = expressionPlus(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
expressionPlus(lesc,resc):
  expression_base(lesc,resc):bexp  plusTailOpt(lesc,resc,bexp):exp
    => exp
*/
pub fn expressionPlus(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone())) {
        (chars, linfo, lesc, resc) => {
            let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, exp) = plusTailOpt(chars.clone(), linfo.clone(), exp.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            (chars.clone(), linfo.clone(), exp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outExpression))
}

/*
plusTailOpt(lesc,resc,bexp):
  '+' expression_base(lesc,resc):exp  concatExp_rest(lesc,resc):expLst   //  concatenation same as "<expression><expression>"
    => TEMPLATE(bexp::exp::expLst, "+", "");
  |
  _ => bexp
*/
pub fn plusTailOpt(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inBaseExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo), mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inBaseExpression.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "+", tail: chars }, linfo, bexp, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, expLst) = concatExp_rest(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    sinfo = tplSourceInfo(startPositionFromExp(bexp.clone())?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::TEMPLATE { items: cons(bexp.clone(), cons(exp.clone(), expLst.clone())), lquote: (literal!("+")).clone(), rquote: (literal!("")).clone() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inBaseExpression.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
concatExp_rest(lesc,resc):
  '+' expression_base(lesc,resc):exp  concatExp_rest(lesc,resc):expLst  =>  exp::expLst
  |
  _ => {}
*/
pub fn concatExp_rest(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    (outChars, outLineInfo, outExpressionList) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "+", tail: chars }, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, expLst) = concatExp_rest(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), cons(exp.clone(), expLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpressionList))
}

/*
expression_base(lesc,resc):
  stringConstant:strRevList
    => STR_TOKEN(makeStrTokFromRevStrList(strRevList))
  |
  literalConstant:(str,litType)
    => LITERAL(str,litType)
  |
  templateExp(lesc,resc)
  |
  '{' '}'  => MAP_ARG_LIST({})
  |
  '{' expressionPlus(lesc,resc):exp  expressionList_rest(lesc,resc):expLst '}'   //  list construction with possible mixed scalars and lists
                                                             // useful in map/concatenation context
     => MAP_ARG_LIST(exp::expLst)
  |
  '(' expression(lesc,resc):exp ')'
     => exp
  |
  '&' identifier:id
    => BOUND_VALUE(IDENT(name))  //TODO: ref Text buffer
  |// TODO: create an optional/error reporting variant of pathIdent
  pathIdent:name  boundValueOrFunCall(name,lesc,resc):exp  =>  exp
*/
pub fn expression_base(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars, startLInfo, _, _) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strRevList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut st: Arc<Tpl::StringToken> = Arc::new(Tpl::StringToken::ST_NEW_LINE);
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo, strRevList) = stringConstant(startChars.clone(), startLInfo.clone())?;
                    st = makeStrTokFromRevStrList(strRevList.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 0)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: st.clone() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars, startLInfo, _, _) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo, r#str, ts) = literalConstant(startChars.clone(), startLInfo.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 0)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::LITERAL { value: (r#str.clone()).clone(), litType: ts.clone() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = templateExp(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "{", tail: startChars }, startLInfo, _, _) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "}", tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 1)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::MAP_ARG_LIST { parts: metamodelica::nil() }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "{", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo, exp) = expressionPlus(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, expLst) = expressionList_rest(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("}")).clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 1)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::MAP_ARG_LIST { parts: cons(exp.clone(), expLst.clone()) }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone(), false)?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "&", tail: startChars }, startLInfo, _, _) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 1)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (id.clone()).clone() }) }), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut name: Arc<TplAbsyn::PathIdent>;
                    let mut expB: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo, name) = pathIdent(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, expB) = boundValueOrFunCall(chars.clone(), linfo.clone(), name.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 0)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (expB.clone(), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
boundValueOrFunCall(name,lesc,resc):
  funCall(name,lesc,resc):exp  => exp
  |
  _ => BOUND_VALUE(name)
*/
pub fn boundValueOrFunCall(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inName: Arc<TplAbsyn::PathIdent>, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::ExpressionBase>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpressionBase: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
    (outChars, outLineInfo, outExpressionBase) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inName.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, name, lesc, resc) => {
                    let mut expB: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, expB) = funCall(chars.clone(), linfo.clone(), name.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), expB.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: inName.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpressionBase))
}

/*
//may fail
funCall(name,lesc,resc):
  '(' ')' => FUN_CALL(name,{})
  |
  '(' expression(lesc,resc):exp  expressionList_rest(lesc,resc):expLst ')'  //template  or  intrinsic function
    => FUN_CALL(name,exp::expLst)
*/
pub fn funCall(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inName: Arc<TplAbsyn::PathIdent>, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::ExpressionBase>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpressionBase: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
    (outChars, outLineInfo, outExpressionBase) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inName.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo, name, _, _) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ ")", tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::ExpressionBase::FUN_CALL { name: name.clone(), args: metamodelica::nil() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo, name, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expressionPlus(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, expLst) = expressionList_rest(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::ExpressionBase::FUN_CALL { name: name.clone(), args: cons(exp.clone(), expLst.clone()) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpressionBase))
}

/*
expressionList_rest(lesc,resc):
  ',' expressionPlus(lesc,resc):exp  expressionList_rest(lesc,resc):expLst => exp::expLst
  |
  _ => {}
*/
pub fn expressionList_rest(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    (outChars, outLineInfo, outExpressionList) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ",", tail: chars }, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expressionPlus(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, expLst) = expressionList_rest(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), cons(exp.clone(), expLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpressionList))
}

/*
stringConstant:
  '"' doubleQuoteConst({},{}):stRevLst
    => stRevLst
  |
  //'%'(lquot) stripFirstNewLine verbatimConst(Rquote(lquot),{},{}):stRevLst
  //  => stRevLst
  //|
  '\\n' escUnquotedChars({}, {"\n"}):stRevLst
    => stRevLst
  |
  '\\' escChar:c  escUnquotedChars({c}, {}):stRevLst
    => stRevLst
*/
pub fn stringConstant(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<ArcStr>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outStrRevList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outChars, outLineInfo, outStrRevList) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::List::Cons { head: Deref @ "\"", tail: chars }, startLinfo) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut optError: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(chars.clone(), startLinfo.clone(), metamodelica::nil(), metamodelica::nil())?;
                    linfo = parseErrorPrevPositionOpt(startChars.clone(), startLinfo.clone(), linfo.clone(), optError.clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: chars } }, linfo) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, stRevLst) = escUnquotedChars(chars.clone(), linfo.clone(), metamodelica::nil(), list![(literal!("\n")).clone()])?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::List::Cons { head: c, tail: chars } }, linfo) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut c = (*c).clone();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    c = (escChar((c.clone()).clone())?).clone();
                    (chars, linfo, stRevLst) = escUnquotedChars(chars.clone(), linfo.clone(), list![(c.clone()).clone()], metamodelica::nil())?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outStrRevList))
}

/*
//not optional, must not fail
literalConstant:
  //(+|-)?d*(.d+)?(('e'|'E')(+|-)?d+)?
  plusMinus:pm digits:ds dotNumber:(dn,ts) exponent(ts):(ex,ts)
  => (pm+ stringCharListString(ds)+dn+ex, ts)  //validate the number - must have integer part or dotpart
  |
  'true' => ("true", BOOLEAN_TYPE())
  |
  'false' => ("false", BOOLEAN_TYPE())
*/
pub fn literalConstant(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, ArcStr, Arc<TplAbsyn::TypeSignature>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outConstantValue: ArcStr = arcstr::literal!("");
    let mut outConstantType: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    (outChars, outLineInfo, outConstantValue, outConstantType) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut ds: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut pm: ArcStr = arcstr::literal!("");
                    let mut dn: ArcStr = arcstr::literal!("");
                    let mut ex: ArcStr = arcstr::literal!("");
                    let mut num: ArcStr = arcstr::literal!("");
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, pm) = plusMinus(chars.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, ds) = digits(chars.clone())?;
                    (chars, dn, ts) = dotNumber(chars.clone())?;
                    num = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringCharListString(ds.clone())); __mm_s.push_str(&*dn.clone()); ArcStr::from(__mm_s) }).clone();
                    let true = (((num.clone()).clone().len() as i32) > 0) else { bail!("pattern mismatch") };
                    (chars, ex, ts) = exponent(chars.clone(), ts.clone())?;
                    num = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*pm.clone()); __mm_s.push_str(&*num.clone()); __mm_s.push_str(&*ex.clone()); ArcStr::from(__mm_s) }).clone();
                    Ok((chars.clone(), linfo.clone(), num.clone(), ts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "r", tail: Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } }, linfo) => {
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), linfo.clone(), literal!("true"), Arc::new(crate::TplAbsyn::TypeSignature::BOOLEAN_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "f", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } } }, linfo) => {
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), linfo.clone(), literal!("false"), Arc::new(crate::TplAbsyn::TypeSignature::BOOLEAN_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outConstantValue, outConstantType))
}

pub fn stripFirstNewLine(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

pub fn rightVerbatimConstQuote(mut inLeftQuote: ArcStr) -> ArcStr {
    let mut outRightQuote: ArcStr = arcstr::literal!("");
    outRightQuote = ((::match_deref::match_deref! { match &(inLeftQuote.clone()) {
        Deref @ "(" => literal!(")"),
        Deref @ "{" => literal!("}"),
        Deref @ "<" => literal!(">"),
        Deref @ "[" => literal!("]"),
        _ => inLeftQuote.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outRightQuote
}

/*
doubleQuoteConst(accChars,accStrList):
  '"' => stringCharListString(listReverse(accChars)) :: accStrList
  |
  newLine doubleQuoteConst({}, stringCharListString(listReverse('\n'::accChars))::accStrList):stRevLst
  => stRevLst
  |
  '\\n' doubleQuoteConst({}, stringCharListString(listReverse('\n'::accChars))::accStrList):stRevLst
  => stRevLst
  |
  '\\'escChar:c doubleQuoteConst(c::accChars,accStrList):stRevLst
  => stRevLst
  |
  c doubleQuoteConst(c::accChars,accStrList):stRevLst
  => stRevLst
  |
  Error end of file
*/
pub fn doubleQuoteConst(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inAccChars: Arc<metamodelica::List<ArcStr>>, mut inAccStrList: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<ArcStr>>, Option<ArcStr>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outStrRevList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outError: Option<ArcStr> = None;
    (outChars, outLineInfo, outStrRevList, outError) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inAccChars.clone(), inAccStrList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\"", tail: chars }, linfo, accChars, accStrList) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    Ok((chars.clone(), linfo.clone(), cons((r#str.clone()).clone(), accStrList.clone()), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: chars } }, linfo, accChars, accStrList) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut optError: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    r#str = (stringCharListString(cons((literal!("\n")).clone(), accChars.clone()).reverse())).clone();
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(chars.clone(), linfo.clone(), metamodelica::nil(), cons((r#str.clone()).clone(), accStrList.clone()))?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::List::Cons { head: c, tail: chars } }, linfo, accChars, accStrList) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut optError: Option<ArcStr> = None;
                    let mut c = (*c).clone();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    c = (escChar((c.clone()).clone())?).clone();
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(chars.clone(), linfo.clone(), cons((c.clone()).clone(), accChars.clone()), accStrList.clone())?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, accChars, accStrList) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut optError: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    r#str = (stringCharListString(cons((literal!("\n")).clone(), accChars.clone()).reverse())).clone();
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(chars.clone(), linfo.clone(), metamodelica::nil(), cons((r#str.clone()).clone(), accStrList.clone()))?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars @ Deref @ metamodelica::List::Cons { head: c, tail: restChars }, linfo, accChars, accStrList) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut optError: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        (_, _) = unwrap_break_err!(newLine(chars.clone(), linfo.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(restChars.clone(), linfo.clone(), cons((c.clone()).clone(), accChars.clone()), accStrList.clone())?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, linfo, accChars, accStrList) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut errStr: ArcStr = arcstr::literal!("");
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    errStr = (literal!("Unmatched \" \" quotes for a string constant - reached end of file.")).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.doubleQuoteConst - ")); __mm_s.push_str(&*errStr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((metamodelica::nil(), linfo.clone(), cons((r#str.clone()).clone(), accStrList.clone()), Some((errStr.clone()).clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outStrRevList, outError))
}

/*
escChar:
  ( '\'' | '"' | '?' |  '\\' | 'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | ' ' )
  => the escaped char

*/
pub fn escChar(mut inEscChar: ArcStr) -> Result<ArcStr> {
    let mut outTheChar: ArcStr = arcstr::literal!("");
    outTheChar = ((::match_deref::match_deref! { match &(inEscChar.clone()) {
        Deref @ "'" => literal!("'"),
        Deref @ "\"" => literal!("\""),
        Deref @ "?" => literal!("?"),
        Deref @ "\\" => literal!("\\"),
        Deref @ "n" => literal!("\n"),
        Deref @ "t" => literal!("\t"),
        Deref @ " " => literal!(" "),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outTheChar)
}

/*
verbatimConst(rquot, accChars, accStrList):
  //strip a last inline new line
  newLine (rquot)'%' =>  stringCharListString(listReverse(accChars)) :: accStrList
  |
  (rquot)'%' =>  stringCharListString(listReverse(accChars)) :: accStrList
  |
  newLine verbatimConst(rquot, {}, stringCharListString(listReverse('\n'::accChars))::accStrList):stRevLst
    => stRevLst
  |
  c  verbatimConst(rquot, c::accChars,accStrList):stRevLst
    => stRevLst
  |
  Error end of file
*/
pub fn verbatimConst(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inRightQuote: ArcStr, mut inAccChars: Arc<metamodelica::List<ArcStr>>, mut inAccStrList: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<ArcStr>>, Option<ArcStr>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outStrRevList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outError: Option<ArcStr> = None;
    (outChars, outLineInfo, outStrRevList, outError) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inRightQuote.clone(), inAccChars.clone(), inAccStrList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, rquot, accChars, accStrList) => {
                    let mut c: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: Deref @ "%", tail: __pa1 } } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    c = __pa0.clone();
                    chars = __pa1.clone();
                    let true = (stringEq((c.clone()).clone(), (rquot.clone()).clone())) else { bail!("pattern mismatch") };
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    Ok((chars.clone(), linfo.clone(), cons((r#str.clone()).clone(), accStrList.clone()), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: c, tail: Deref @ metamodelica::List::Cons { head: Deref @ "%", tail: chars } }, linfo, rquot, accChars, accStrList) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (stringEq((c.clone()).clone(), (rquot.clone()).clone())) else { bail!("pattern mismatch") };
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    Ok((chars.clone(), linfo.clone(), cons((r#str.clone()).clone(), accStrList.clone()), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, rquot, accChars, accStrList) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut optError: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    r#str = (stringCharListString(cons((literal!("\n")).clone(), accChars.clone()).reverse())).clone();
                    (chars, linfo, stRevLst, optError) = verbatimConst(chars.clone(), linfo.clone(), (rquot.clone()).clone(), metamodelica::nil(), cons((r#str.clone()).clone(), accStrList.clone()))?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars @ Deref @ metamodelica::List::Cons { head: c, tail: restChars }, linfo, rquot, accChars, accStrList) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut optError: Option<ArcStr> = None;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        (_, _) = unwrap_break_err!(newLine(chars.clone(), linfo.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (chars, linfo, stRevLst, optError) = verbatimConst(restChars.clone(), linfo.clone(), (rquot.clone()).clone(), cons((c.clone()).clone(), accChars.clone()), accStrList.clone())?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, linfo, rquot, accChars, accStrList) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut errStr: ArcStr = arcstr::literal!("");
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    errStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unmatched %")); __mm_s.push_str(&*rquot.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*rquot.clone()); __mm_s.push_str(&*literal!("% quotes for a verbatim string constant - reached end of file.")); ArcStr::from(__mm_s) }).clone();
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.verbatimConst - ")); __mm_s.push_str(&*errStr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok((metamodelica::nil(), linfo.clone(), cons((r#str.clone()).clone(), accStrList.clone()), Some((errStr.clone()).clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outStrRevList, outError))
}

/*
escUnquotedChars(accChars,accStrList):
  '\\n' escUnquotedChars({}, stringCharListString(listReverse('\n'::accChars)) :: accStrList):stRevLst
  => stRevLst
  |
  '\\' escChar:c  escUnquotedChars(c::accChars, accStrList):stRevLst
  => stRevLst
  |
  _ => stringCharListString(listReverse(accChars)) :: accStrList

*/
pub fn escUnquotedChars(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inAccChars: Arc<metamodelica::List<ArcStr>>, mut inAccStrList: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<ArcStr>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outStrRevList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outChars, outLineInfo, outStrRevList) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inAccChars.clone(), inAccStrList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: chars } }, linfo, accChars, accStrList) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    r#str = (stringCharListString(cons((literal!("\n")).clone(), accChars.clone()).reverse())).clone();
                    (chars, linfo, stRevLst) = escUnquotedChars(chars.clone(), linfo.clone(), metamodelica::nil(), cons((r#str.clone()).clone(), accStrList.clone()))?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::List::Cons { head: c, tail: chars } }, linfo, accChars, accStrList) => {
                    let mut stRevLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut c = (*c).clone();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    c = (escChar((c.clone()).clone())?).clone();
                    (chars, linfo, stRevLst) = escUnquotedChars(chars.clone(), linfo.clone(), cons((c.clone()).clone(), accChars.clone()), accStrList.clone())?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, accChars, accStrList) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    Ok((chars.clone(), linfo.clone(), cons((r#str.clone()).clone(), accStrList.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outStrRevList))
}

pub fn makeStrTokFromRevStrList(mut inRevStrList: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Tpl::StringToken>> {
    let mut outStringToken: Arc<Tpl::StringToken> = Arc::new(Tpl::StringToken::ST_NEW_LINE);
    outStringToken = 'mc: {
        let __mc_input = inRevStrList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#str, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(Arc::new(Tpl::StringToken::ST_STRING { value: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Cons { head: Deref @ "\n", tail: Deref @ metamodelica::List::Nil } } => {
                    Ok(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Cons { head: r#str, tail: Deref @ metamodelica::List::Nil } } => {
                    Ok(Arc::new(Tpl::StringToken::ST_LINE { line: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "", tail: strList } => {
                    let mut strList = (*strList).clone();
                    strList = strList.clone().reverse();
                    Ok(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: strList.clone(), lastHasNewLine: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                strList @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
                    let mut strList = (*strList).clone();
                    strList = strList.clone().reverse();
                    Ok(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: strList.clone(), lastHasNewLine: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Parse invalid operation error - TplParser.makeStrTokFromRevStrList failed (an empty string list passed?) .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringToken)
}

/*
plusMinus:
  '+' => "+"
  |
  '-' => "-"
  |
  _ => ""
*/
pub fn plusMinus(mut inChars: Arc<metamodelica::List<ArcStr>>) -> (Arc<metamodelica::List<ArcStr>>, ArcStr) {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outSign: ArcStr = arcstr::literal!("");
    (outChars, outSign) = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::List::Cons { head: char, tail: chars } if (char.clone() == literal!("+") || char.clone() == literal!("-")) => {
            (chars.clone(), char.clone())
        },
        _ => {
            (inChars.clone(), literal!(""))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outChars, outSign)
}

/*
digits:
  [0-9]:d  digits:ds => d::ds
  |
  _ => {}
*/
pub fn digits(mut inChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outDigits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outChars, outDigits) = 'mc: {
        let __mc_input = inChars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: d, tail: chars } => {
                    let mut ds: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut i: i32 = 0;
                    let mut chars = (*chars).clone();
                    i = stringCharInt((d.clone()).clone())?;
                    let true = (48 <= i.clone() && i.clone() <= 57) else { bail!("pattern mismatch") };
                    (chars, ds) = digits(chars.clone())?;
                    Ok((chars.clone(), cons((d.clone()).clone(), ds.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                chars => {
                    Ok((chars.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outDigits))
}

/*
dotNumber:
  '.' digits:ds  =>  (stringCharListString(ds), REAL_TYPE())
  |
  _ => INTEGER_TYPE()
*/
pub fn dotNumber(mut inChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<TplAbsyn::TypeSignature>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outDotNumber: ArcStr = arcstr::literal!("");
    let mut outLitType: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    (outChars, outDotNumber, outLitType) = 'mc: {
        let __mc_input = inChars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ ".", tail: chars } => {
                    let mut dn: ArcStr = arcstr::literal!("");
                    let mut ds: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    (chars, ds) = digits(chars.clone())?;
                    ::match_deref::match_deref! { match &(ds.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: _ } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    dn = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*stringCharListString(ds.clone())); ArcStr::from(__mm_s) }).clone();
                    Ok((chars.clone(), dn.clone(), Arc::new(crate::TplAbsyn::TypeSignature::REAL_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), literal!(""), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outDotNumber, outLitType))
}

/*
exponent(typ):
  'e' plusMinus:pm  digits:ds => ("e"+pm+stringCharListString(ds), REAL_TYPE())
  |
  'E' plusMinus:pm  digits:ds => ("E"+pm+stringCharListString(ds), REAL_TYPE())
  |
  => ("",typ)
*/
pub fn exponent(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLitType: Arc<TplAbsyn::TypeSignature>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<TplAbsyn::TypeSignature>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outExponent: ArcStr = arcstr::literal!("");
    let mut outLitType: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
    (outChars, outExponent, outLitType) = 'mc: {
        let __mc_input = (inChars.clone(), inLitType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars }, _) => {
                    let mut ex: ArcStr = arcstr::literal!("");
                    let mut pm: ArcStr = arcstr::literal!("");
                    let mut ds: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    (chars, pm) = plusMinus(chars.clone());
                    (chars, ds) = digits(chars.clone())?;
                    ::match_deref::match_deref! { match &(ds.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: _ } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ex = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*pm.clone()); __mm_s.push_str(&*stringCharListString(ds.clone())); ArcStr::from(__mm_s) }).clone();
                    Ok((chars.clone(), ex.clone(), Arc::new(crate::TplAbsyn::TypeSignature::REAL_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "E", tail: chars }, _) => {
                    let mut ex: ArcStr = arcstr::literal!("");
                    let mut pm: ArcStr = arcstr::literal!("");
                    let mut ds: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    (chars, pm) = plusMinus(chars.clone());
                    (chars, ds) = digits(chars.clone())?;
                    ::match_deref::match_deref! { match &(ds.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: _ } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ex = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E")); __mm_s.push_str(&*pm.clone()); __mm_s.push_str(&*stringCharListString(ds.clone())); ArcStr::from(__mm_s) }).clone();
                    Ok((chars.clone(), ex.clone(), Arc::new(crate::TplAbsyn::TypeSignature::REAL_TYPE)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), literal!(""), inLitType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outExponent, outLitType))
}

/*
templateExp(lesc, resc):
  "'" stripFirstNewLine  templateBody(lesc, resc, isSingleQuote = true, {},{},0)
  |
  '<<' stripFirstNewLine templateBody(lesc, resc, isSingleQuote = false,{},{},0 )
*/
pub fn templateExp(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "'", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut expB: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (chars, linfo, expB) = templateBody(startChars.clone(), startLInfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone(), true, metamodelica::nil(), metamodelica::nil(), 0)?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 1)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (expB.clone(), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "<", tail: Deref @ metamodelica::List::Cons { head: Deref @ "<", tail: startChars } }, startLInfo @ LineInfo { startOfLineChars: solChars, .. }, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut expB: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
                    let mut baseInd: i32 = 0;
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (_, baseInd) = lineIndent(solChars.clone(), 0);
                    (chars, linfo) = takeSpaceAndNewLine(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo, expB) = templateBody(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone(), false, metamodelica::nil(), metamodelica::nil(), baseInd.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 2)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (expB.clone(), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "<", tail: Deref @ metamodelica::List::Cons { head: Deref @ "<", tail: startChars } }, startLInfo @ LineInfo { startOfLineChars: solChars, .. }, lesc, resc) => {
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut linfo: LineInfo;
                    let mut expB: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
                    let mut baseInd: i32 = 0;
                    let mut lineInd: i32 = 0;
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (_, baseInd) = lineIndent(solChars.clone(), 0);
                    if '__try0: {
                        (_, _) = unwrap_break_err!(takeSpaceAndNewLine(startChars.clone(), startLInfo.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (chars, lineInd) = lineIndent(startChars.clone(), 0);
                    lineInd = lineInd.clone() + baseInd.clone();
                    (chars, linfo, expB) = restOfTemplLine(chars.clone(), startLInfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone(), false, metamodelica::nil(), metamodelica::nil(), baseInd.clone(), lineInd.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 2)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (expB.clone(), sinfo.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
//optional, may fail
takeSpaceAndNewLine:
  newLine
  |
  ' ' takeSpaceAndNewLine
  |
  '\t' takeSpaceAndNewLine
*/
pub fn takeSpaceAndNewLine(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: char, tail: chars }, linfo) => {
                    if !((char.clone() == literal!(" ") || char.clone() == literal!("\t"))) { bail!("guard") }
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = takeSpaceAndNewLine(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
templateBody(lesc, resc, isSingleQuote, expList, indStack, actInd):
  lineIndent(0):lineInd
    restOfTemplLine(lesc, resc, isSingleQuote, expList, indStack, actInd, lineInd, {}):exp
  => exp
*/
pub fn templateBody(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr, mut inIsSingleQuote: bool, mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, mut inIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, mut inActualIndent: i32) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::ExpressionBase>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpressionBase: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
    let mut lindent: i32 = 0;
    (outChars, lindent) = lineIndent(inChars.clone(), 0);
    (outChars, outLineInfo, outExpressionBase) = restOfTemplLine(outChars.clone(), inLineInfo.clone(), (inLeftEsc.clone()).clone(), (inRightEsc.clone()).clone(), inIsSingleQuote.clone(), inExpressionList.clone(), inIndentStack.clone(), inActualIndent.clone(), lindent.clone())?;
    Ok((outChars, outLineInfo, outExpressionBase))
}

/*
lineIndent(ind):
  ' ' lineIndent(ind+1):n  =>  n
  |
  '\t' lineIndent(ind+4):n  =>  n
  |
  _  =>  ind

*/
pub fn lineIndent(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineIndent: i32) -> (Arc<metamodelica::List<ArcStr>>, i32) {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineIndent: i32 = 0;
    (outChars, outLineIndent) = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ " ", tail: outChars } => lineIndent(outChars.clone(), inLineIndent.clone() + 1),
        Deref @ metamodelica::List::Cons { head: Deref @ "\t", tail: outChars } => lineIndent(outChars.clone(), inLineIndent.clone() + TabSpaces.clone()),
        _ => (inChars.clone(), inLineIndent.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outChars, outLineIndent)
}

/*
// & ... no interleave
restOfTemplLine(lesc, resc, isSingleQuote, expList, indStack, actInd, lineInd, accStrChars):
  //(lesc)'#' nonTemplateExprWithOpts(lesc,resc):eexp  '#'(resc)
  //   { (expList, indStack, actInd) = onEscapedExp(eexp, expList, indStack, actInd, lineInd, accStrChars) }
  //   & restOfTemplLine(lesc,resc,isSingleQuote, expList, indStack, actInd, actInd, {}):exp
  //   => exp
  //
  //|
  (lesc)  (resc)  // a comment | empty expression ... ignore completely
     & restOfTemplLineAfterEmptyExp(lesc,resc,isSingleQuote, expList, indStack, actInd, lineInd, accStrChars):exp
     => exp
  |
  (lesc) '%' expression(lesc,resc):eexp (resc)
     { (expList, indStack, actInd) = onEscapedExp(eexp, expList, indStack, actInd, lineInd, accStrChars) }
     & restOfTemplLine(lesc,resc,isSingleQuote, expList, indStack, actInd, actInd, {}):exp
     => exp

  | // on \n
  newLine
   { (expList, indStack, actInd) = onNewLine(expList, indStack, actInd, lineInd, accStrChars) }
   & templateBody(lesc, resc, isSingleQuote, expList, indStack, actInd):exp
  => exp

  | //end
  (isSingleQuote = true) "'"
   =>
    onTemplEnd(expList, indStack, actInd, lineInd, accStrChars)

  | //end
  (isSingleQuote = false) '>>'
   =>
   onTemplEnd(expList, indStack, actInd, lineInd, accStrChars)

  |
  '\' & ( '\' | "'" | (lesc) | (resc) ):c
   & restOfTemplLine(lesc, resc, isSingleQuote, expList, indStack, actInd, lineInd, c :: accStrChars) : exp
    => exp
  |
  any:c
    & restOfTemplLine(lesc, resc, isSingleQuote, expList, indStack, actInd, lineInd, c :: accStrChars) : exp
    => exp
*/
pub fn restOfTemplLine(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr, mut inIsSingleQuote: bool, mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, mut inIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, mut inActualIndent: i32, mut inLineIndent: i32) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::ExpressionBase>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = inChars.clone();
    let mut outLineInfo: LineInfo = inLineInfo.clone();
    let mut outExpressionBase: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
    let mut expl: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = inExpressionList.clone();
    let mut lindent: i32 = inLineIndent.clone();
    let mut aindent: i32 = inActualIndent.clone();
    let mut ind_stack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>> = inIndentStack.clone();
    let mut char: ArcStr = arcstr::literal!("");
    let mut next_char: ArcStr = arcstr::literal!("");
    let mut remaining_chars: bool = true;
    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut sol_chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut acc_chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut err_opt: Option<ArcStr> = None;
    let mut linfo: LineInfo;
    if '__try0: {
        loop {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(outChars.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            char = __pa1.clone();
            outChars = __pa2.clone();
            if inIsSingleQuote.clone() && char.clone() == literal!("'") {
                expl = unwrap_break_err!(onTemplEnd(false, expl.clone(), ind_stack.clone(), aindent.clone(), lindent.clone(), acc_chars.clone()), '__try0);
                outExpressionBase = unwrap_break_err!(makeTemplateFromExpList(expl.clone(), (literal!("'")).clone(), (literal!("'")).clone()), '__try0);
                return Ok((outChars, outLineInfo, outExpressionBase));
            }
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(outChars.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            next_char = __pa3.clone();
            chars = __pa4.clone();
            if !(inIsSingleQuote.clone()) && char.clone() == literal!(">") && next_char.clone() == literal!(">") {
                expl = unwrap_break_err!(onTemplEnd(true, expl.clone(), ind_stack.clone(), aindent.clone(), lindent.clone(), acc_chars.clone()), '__try0);
                outExpressionBase = unwrap_break_err!(makeTemplateFromExpList(expl.clone(), (literal!("<<")).clone(), (literal!(">>")).clone()), '__try0);
                outChars = chars.clone();
                return Ok((outChars, outLineInfo, outExpressionBase));
            } else if char.clone() == literal!("\r") || char.clone() == literal!("\n") {
                (outChars, linfo) = unwrap_break_err!(newLine(cons((char.clone()).clone(), outChars.clone()), outLineInfo.clone()), '__try0);
                (expl, ind_stack, aindent, err_opt) = unwrap_break_err!(onNewLine(expl.clone(), ind_stack.clone(), aindent.clone(), lindent.clone(), acc_chars.clone()), '__try0);
                outLineInfo = unwrap_break_err!(parseErrorPrevPositionOptInfoChars(outLineInfo.clone(), linfo.clone(), err_opt.clone(), false), '__try0);
                (outChars, lindent) = lineIndent(outChars.clone(), 0);
                acc_chars = metamodelica::nil();
            } else if char.clone() == inLeftEsc.clone() && next_char.clone() == literal!("%") {
                (outChars, linfo) = unwrap_break_err!(interleave(chars.clone(), outLineInfo.clone()), '__try0);
                let (__pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(outChars.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } } => (__pa5.clone(), __pa6.clone(), __pa7.clone()),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                char = __pa5.clone();
                next_char = __pa6.clone();
                chars = __pa7.clone();
                if char.clone() == literal!("%") && next_char.clone() == inRightEsc.clone() {
                    (outChars, outLineInfo, lindent) = unwrap_break_err!(dropNewLineAfterEmptyExp(chars.clone(), linfo.clone(), lindent.clone(), acc_chars.clone()), '__try0);
                } else {
                    (outChars, linfo, exp) = unwrap_break_err!(expression(outChars.clone(), linfo.clone(), (inLeftEsc.clone()).clone(), (inRightEsc.clone()).clone(), false), '__try0);
                    (outChars, linfo) = unwrap_break_err!(interleaveExpectChar(outChars.clone(), linfo.clone(), (literal!("%")).clone()), '__try0);
                    (outChars, linfo) = unwrap_break_err!(expectChar(outChars.clone(), linfo.clone(), (inRightEsc.clone()).clone()), '__try0);
                    (expl, ind_stack, aindent, err_opt) = unwrap_break_err!(onEscapedExp(exp.clone(), expl.clone(), ind_stack.clone(), aindent.clone(), lindent.clone(), acc_chars.clone()), '__try0);
                    outLineInfo = unwrap_break_err!(parseErrorPrevPositionOptInfoChars(outLineInfo.clone(), linfo.clone(), err_opt.clone(), false), '__try0);
                    acc_chars = metamodelica::nil();
                }
            } else {
                acc_chars = cons((char.clone()).clone(), acc_chars.clone());
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    outChars = metamodelica::nil();
    outLineInfo = parseError(metamodelica::nil(), inLineInfo.clone(), (literal!("Not able to parse the text template expression from the point.")).clone(), true)?;
    outExpressionBase = Arc::new(crate::TplAbsyn::ExpressionBase::ERROR_EXP);
    Ok((outChars, outLineInfo, outExpressionBase))
}

pub fn dropNewLineAfterEmptyExp(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLineIndent: i32, mut inAccStringChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, i32)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outLineIndent: i32 = 0;
    (outChars, outLineInfo, outLineIndent) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLineIndent.clone(), inAccStringChars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _, Deref @ metamodelica::List::Nil) => {
                    let mut lineInd: i32 = 0;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = takeSpaceAndNewLine(chars.clone(), linfo.clone())?;
                    (chars, lineInd) = lineIndent(chars.clone(), 0);
                    Ok((chars.clone(), linfo.clone(), lineInd.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inLineIndent.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outLineIndent))
}

pub fn makeTemplateFromExpList(mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, mut inLeftQuote: ArcStr, mut inRightQuote: ArcStr) -> Result<Arc<TplAbsyn::ExpressionBase>> {
    let mut outExpressionBase: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
    outExpressionBase = 'mc: {
        let __mc_input = (inExpressionList.clone(), inLeftQuote.clone(), inRightQuote.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (expB, _), tail: Deref @ metamodelica::List::Nil }, _, _) => {
                    Ok(expB.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, lquote, rquote) => {
                    let mut expLst = (*expLst).clone();
                    expLst = expLst.clone().reverse();
                    Ok(Arc::new(TplAbsyn::ExpressionBase::TEMPLATE { items: expLst.clone(), lquote: (lquote.clone()).clone(), rquote: (rquote.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpressionBase)
}

pub fn onEscapedExp(mut inExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo), mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, mut inIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, mut inActualIndent: i32, mut inLineIndent: i32, mut inAccStringChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, i32, Option<ArcStr>)> {
    let mut outExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    let mut outIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>> = metamodelica::nil();
    let mut outActualIndent: i32 = 0;
    let mut outError: Option<ArcStr> = None;
    (outExpressionList, outIndentStack, outActualIndent, outError) = 'mc: {
        let __mc_input = (inExpression.clone(), inExpressionList.clone(), inIndentStack.clone(), inActualIndent.clone(), inLineIndent.clone(), inAccStringChars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, expLst, indStack, actInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let true = (intEq(lineInd.clone(), actInd.clone())) else { bail!("pattern mismatch") };
                    expLst = addAccStringChars(expLst.clone(), accChars.clone())?;
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    expLst = cons(exp.clone(), expLst.clone());
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, expLst, indStack, actInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let true = (lineInd.clone() > actInd.clone()) else { bail!("pattern mismatch") };
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    indStack = cons((actInd.clone(), expLst.clone()), indStack.clone());
                    expLst = addAccStringChars(metamodelica::nil(), accChars.clone())?;
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    expLst = cons(exp.clone(), expLst.clone());
                    Ok((expLst.clone(), indStack.clone(), lineInd.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, expLst, Deref @ metamodelica::List::Nil, baseInd, lineInd, accChars) => {
                    let mut errStr: ArcStr = arcstr::literal!("");
                    let mut errOpt: Option<ArcStr> = None;
                    let mut actInd: i32 = 0;
                    let mut indStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>> = metamodelica::nil();
                    let mut expLst = (*expLst).clone();
                    let true = (lineInd.clone() < baseInd.clone()) else { bail!("pattern mismatch") };
                    errStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Indent level is under the level of the '<<' determined level (by ")); __mm_s.push_str(&*intString(baseInd.clone() - lineInd.clone())); __mm_s.push_str(&*literal!(" chars).")); ArcStr::from(__mm_s) }).clone();
                    errOpt = Some((errStr.clone()).clone());
                    (expLst, indStack, actInd, _) = onEscapedExp(exp.clone(), expLst.clone(), metamodelica::nil(), baseInd.clone(), baseInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), errOpt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, expLst, indStack @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, actInd, lineInd, accChars) => {
                    let mut errOpt: Option<ArcStr> = None;
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (lineInd.clone() < actInd.clone()) else { bail!("pattern mismatch") };
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    (expLst, indStack, actInd) = popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone())?;
                    (expLst, indStack, actInd, errOpt) = onEscapedExp(exp.clone(), expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), lineInd.clone(), errOpt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Parse unexpected error - TplParser.onEscapedExp failed .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExpressionList, outIndentStack, outActualIndent, outError))
}

pub fn onNewLine(mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, mut inIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, mut inActualIndent: i32, mut inLineIndent: i32, mut inAccStringChars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, i32, Option<ArcStr>)> {
    let mut outExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    let mut outIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>> = metamodelica::nil();
    let mut outActualIndent: i32 = 0;
    let mut outError: Option<ArcStr> = None;
    (outExpressionList, outIndentStack, outActualIndent, outError) = 'mc: {
        let __mc_input = (inExpressionList.clone(), inIndentStack.clone(), inActualIndent.clone(), inLineIndent.clone(), inAccStringChars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, indStack, actInd, lineInd, Deref @ metamodelica::List::Cons { head: c, tail: accChars }) => {
                    let mut errOpt: Option<ArcStr> = None;
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (c.clone() == literal!(" ") || c.clone() == literal!("\t")) else { bail!("pattern mismatch") };
                    (expLst, indStack, actInd, errOpt) = onNewLine(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), errOpt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, indStack, actInd, _, Deref @ metamodelica::List::Nil) => {
                    let mut expLst: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    expLst = addAccStringChars(metamodelica::nil(), list![(literal!("\n")).clone()])?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst @ Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: _ }, .. } }, _), tail: _ }, indStack, actInd, _, Deref @ metamodelica::List::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = addAccStringChars(expLst.clone(), list![(literal!("\n")).clone()])?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_NEW_LINE }, _), tail: expLst }, indStack, actInd, _, Deref @ metamodelica::List::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = addAccStringChars(expLst.clone(), list![(literal!("\n")).clone()])?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst @ Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::SOFT_NEW_LINE, _), tail: _ }, indStack, actInd, _, Deref @ metamodelica::List::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = addAccStringChars(expLst.clone(), list![(literal!("\n")).clone()])?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, indStack, actInd, _, Deref @ metamodelica::List::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = cons((Arc::new(crate::TplAbsyn::ExpressionBase::SOFT_NEW_LINE), dummySourceInfo.clone()), expLst.clone());
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, indStack, actInd, lineInd, accChars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut strLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut expLst = (*expLst).clone();
                    let mut accChars = (*accChars).clone();
                    let true = (lineInd.clone() >= actInd.clone()) else { bail!("pattern mismatch") };
                    accChars = listAppend(accChars.clone(), List::fill((literal!(" ")).clone(), lineInd.clone() - actInd.clone()));
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(addAccStringChars(expLst.clone(), accChars.clone())?) {
                        Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: __pa0, lastHasNewLine: false } }, _), tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    strLst = __pa0.clone();
                    expLst = __pa1.clone();
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: strLst.clone(), lastHasNewLine: true }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, Deref @ metamodelica::List::Nil, baseInd, lineInd, accChars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut errStr: ArcStr = arcstr::literal!("");
                    let mut errOpt: Option<ArcStr> = None;
                    let mut actInd: i32 = 0;
                    let mut indStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>> = metamodelica::nil();
                    let mut expLst = (*expLst).clone();
                    let true = (lineInd.clone() < baseInd.clone()) else { bail!("pattern mismatch") };
                    errStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Indent level is under the level of the '<<' determined level (by ")); __mm_s.push_str(&*intString(baseInd.clone() - lineInd.clone())); __mm_s.push_str(&*literal!(" chars).")); ArcStr::from(__mm_s) }).clone();
                    errOpt = Some((errStr.clone()).clone());
                    (expLst, indStack, actInd, _) = onNewLine(expLst.clone(), metamodelica::nil(), baseInd.clone(), baseInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), errOpt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, indStack @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, actInd, lineInd, accChars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut errOpt: Option<ArcStr> = None;
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (lineInd.clone() < actInd.clone()) else { bail!("pattern mismatch") };
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    (expLst, indStack, actInd) = popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone())?;
                    (expLst, indStack, actInd, errOpt) = onNewLine(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), errOpt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Parse unexpected error - TplParser.onNewLine failed .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExpressionList, outIndentStack, outActualIndent, outError))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn onTemplEnd(mut inDropLastNewLine: bool, mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, mut inIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, mut inActualIndent: i32, mut inLineIndent: i32, mut inAccStringChars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>> {
    let mut outExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    outExpressionList = 'mc: {
        let __mc_input = (inDropLastNewLine.clone(), inExpressionList.clone(), inIndentStack.clone(), inActualIndent.clone(), inLineIndent.clone(), inAccStringChars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, baseInd, lineInd, Deref @ metamodelica::List::Nil) => {
                    let mut expLst: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    let true = (lineInd.clone() >= baseInd.clone()) else { bail!("pattern mismatch") };
                    expLst = addAccStringChars(metamodelica::nil(), List::fill((literal!(" ")).clone(), lineInd.clone() - baseInd.clone()))?;
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::SOFT_NEW_LINE, _), tail: expLst }, indStack, actInd, _, Deref @ metamodelica::List::Nil) => {
                    let mut expLst = (*expLst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), 0)?) {
                        (__pa0, Deref @ metamodelica::List::Nil, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLst = __pa0.clone();
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: true, strList: strLst @ Deref @ metamodelica::List::Cons { head: Deref @ "", tail: _ } } }, _), tail: expLst }, indStack, actInd, _, Deref @ metamodelica::List::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = finalizeLastStringToken(cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: strLst.clone(), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), 0)?) {
                        (__pa0, Deref @ metamodelica::List::Nil, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLst = __pa0.clone();
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, expLst, indStack, actInd, _, Deref @ metamodelica::List::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), 0)?) {
                        (__pa0, Deref @ metamodelica::List::Nil, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLst = __pa0.clone();
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, expLst, indStack, actInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let mut accChars = (*accChars).clone();
                    let true = (lineInd.clone() >= actInd.clone()) else { bail!("pattern mismatch") };
                    accChars = listAppend(accChars.clone(), List::fill((literal!(" ")).clone(), lineInd.clone() - actInd.clone()));
                    expLst = addAccStringChars(expLst.clone(), accChars.clone())?;
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), 0)?) {
                        (__pa0, Deref @ metamodelica::List::Nil, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLst = __pa0.clone();
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dropLastNL, expLst, Deref @ metamodelica::List::Nil, baseInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let true = (lineInd.clone() < baseInd.clone()) else { bail!("pattern mismatch") };
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Parse warning onTemplEnd() - indent level is under the level of the '<<' determined level.\n")).clone())?;
                    expLst = onTemplEnd(dropLastNL.clone(), expLst.clone(), metamodelica::nil(), baseInd.clone(), baseInd.clone(), accChars.clone())?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dropLastNL, expLst, indStack @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, actInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (lineInd.clone() < actInd.clone()) else { bail!("pattern mismatch") };
                    expLst = finalizeLastStringToken(expLst.clone())?;
                    (expLst, indStack, actInd) = popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone())?;
                    expLst = onTemplEnd(dropLastNL.clone(), expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone(), accChars.clone())?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!!Parse error - TplParser.onTemplEnd failed .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpressionList)
}

pub fn popIndentStack(mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, mut inIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, mut inActualIndent: i32, mut inLineIndent: i32) -> Result<(Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>>, i32)> {
    let mut outExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    let mut outIndentStack: Arc<metamodelica::List<(i32, Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>)>> = metamodelica::nil();
    let mut outActualIndent: i32 = 0;
    (outExpressionList, outIndentStack, outActualIndent) = 'mc: {
        let __mc_input = (inExpressionList.clone(), inIndentStack.clone(), inActualIndent.clone(), inLineIndent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, Deref @ metamodelica::List::Cons { head: (prevInd, prevExpLst), tail: indStack }, actInd, lineInd) => {
                    let mut d: i32 = 0;
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (lineInd.clone() < actInd.clone()) else { bail!("pattern mismatch") };
                    d = actInd.clone() - prevInd.clone();
                    expLst = expLst.clone().reverse();
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::INDENTATION { width: d.clone(), items: expLst.clone() }), dummySourceInfo.clone()), prevExpLst.clone());
                    (expLst, indStack, actInd) = popIndentStack(expLst.clone(), indStack.clone(), prevInd.clone(), lineInd.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, indStack, actInd, lineInd) => {
                    let true = (lineInd.clone() >= actInd.clone()) else { bail!("pattern mismatch") };
                    Ok((expLst.clone(), indStack.clone(), actInd.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, Deref @ metamodelica::List::Nil, baseInd, _) => {
                    Ok((expLst.clone(), metamodelica::nil(), baseInd.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!!Parse error - TplParser.popIndentStack failed .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExpressionList, outIndentStack, outActualIndent))
}

pub fn addAccStringChars(mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>, mut inAccStringChars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>> {
    let mut outExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    outExpressionList = 'mc: {
        let __mc_input = (inExpressionList.clone(), inAccStringChars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, Deref @ metamodelica::List::Nil) => {
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: true, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Cons { head: strNonNl, tail: strLst } } } }, _), tail: expLst }, accChars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut strNonNl = (*strNonNl).clone();
                    let mut expLst = (*expLst).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(stringGetStringChar((strNonNl.clone()).clone(), ((strNonNl.clone()).clone().len() as i32)), '__try0)) {
                            Deref @ "\n" => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    strNonNl = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*strNonNl.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: cons((literal!("")).clone(), cons((r#str.clone()).clone(), cons((strNonNl.clone()).clone(), strLst.clone()))), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: true, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: strLst } } }, _), tail: expLst }, accChars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut expLst = (*expLst).clone();
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: cons((literal!("")).clone(), cons((r#str.clone()).clone(), cons((literal!("\n")).clone(), strLst.clone()))), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: false, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: strLst } } }, _), tail: expLst }, accChars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut expLst = (*expLst).clone();
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: cons((literal!("")).clone(), cons((r#str.clone()).clone(), strLst.clone())), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, accChars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut expLst = (*expLst).clone();
                    r#str = (stringCharListString(accChars.clone().reverse())).clone();
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("")).clone(), (r#str.clone()).clone()], lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!!Parse error - TplParser.addAccStringChars failed .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpressionList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn finalizeLastStringToken(mut inExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>) -> Result<Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>>> {
    let mut outExpressionList: Arc<metamodelica::List<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    outExpressionList = 'mc: {
        let __mc_input = inExpressionList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: true, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Cons { head: strNonNl, tail: strLst } } } }, _), tail: expLst } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut expLst = (*expLst).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(stringGetStringChar((strNonNl.clone()).clone(), ((strNonNl.clone()).clone().len() as i32)), '__try0)) {
                            Deref @ "\n" => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*strNonNl.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    expLst = finalizeLastStringToken(cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: cons((literal!("")).clone(), cons((r#str.clone()).clone(), strLst.clone())), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone()))?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: true, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: strLst } } }, _), tail: expLst } => {
                    let mut expLst = (*expLst).clone();
                    expLst = finalizeLastStringToken(cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: cons((literal!("")).clone(), cons((literal!("\n")).clone(), strLst.clone())), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone()))?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: false, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Nil } } }, _), tail: expLst } => {
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: false, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Cons { head: Deref @ "\n", tail: Deref @ metamodelica::List::Nil } } } }, _), tail: expLst } => {
                    let mut expLst = (*expLst).clone();
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: false, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Cons { head: r#str, tail: Deref @ metamodelica::List::Nil } } } }, _), tail: expLst } => {
                    let mut expLst = (*expLst).clone();
                    ::match_deref::match_deref! { match &(stringGetStringChar((r#str.clone()).clone(), ((r#str.clone()).clone().len() as i32))?) {
                        Deref @ "\n" => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_LINE { line: (r#str.clone()).clone() }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: false, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Cons { head: r#str, tail: Deref @ metamodelica::List::Nil } } } }, _), tail: expLst } => {
                    let mut expLst = (*expLst).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(stringGetStringChar((r#str.clone()).clone(), ((r#str.clone()).clone().len() as i32)), '__try0)) {
                            Deref @ "\n" => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (r#str.clone()).clone() }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { lastHasNewLine: false, strList: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: strLst @ Deref @ metamodelica::List::Cons { head: r#str, tail: _ } } } }, _), tail: expLst } => {
                    let mut hasNL: bool = false;
                    let mut strLst = (*strLst).clone();
                    let mut expLst = (*expLst).clone();
                    hasNL = literal!("\n") == stringGetStringChar((r#str.clone()).clone(), ((r#str.clone()).clone().len() as i32))?;
                    strLst = strLst.clone().reverse();
                    expLst = cons((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: strLst.clone(), lastHasNewLine: hasNL.clone() }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExpressionList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpressionList)
}

/*
conditionExp(lesc,resc):
  'if' condArgExp(lesc,resc):(isNot, lhsExp, rhsMExpOpt)
  'then' expressionLet(lesc,resc):trueBr
  elseBranch(lesc,resc):elseBrOpt
   => CONDITION(isNot, lhsExp, rhsMExpOpt, trueBr, elseBrOpt)
*/
pub fn conditionExp(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "f", tail: startChars } }, startLInfo, lesc, resc) => {
            let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut linfo: LineInfo;
            let mut isNot: bool = false;
            let mut lhsExp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut trueBr: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut rhsMExpOpt: Option<Arc<TplAbsyn::MatchingExp>> = None;
            let mut elseBrOpt: Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)> = None;
            let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            afterKeyword(startChars.clone())?;
            (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
            (chars, linfo, isNot, lhsExp, rhsMExpOpt) = condArgExp(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, trueBr) = thenBranch(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, elseBrOpt) = elseBranch(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 2)?, chars.clone(), linfo.clone())?;
            (chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::CONDITION { isNot: isNot.clone(), lhsExp: lhsExp.clone(), rhsValue: rhsMExpOpt.clone(), trueBranch: trueBr.clone(), elseBranch: elseBrOpt.clone() }), sinfo.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outExpression))
}

pub fn thenBranch(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outTrueBranch: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outTrueBranch) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "h", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: chars } } } }, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), cons((literal!("t")).clone(), cons((literal!("h")).clone(), cons((literal!("e")).clone(), cons((literal!("n")).clone(), metamodelica::nil())))))?) {
                        (_, false) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected 'then' keyword at the position.")).clone(), false)?;
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- !!! TplParser.thenBranch failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outTrueBranch))
}

/*
elseBranch(lesc,resc):
  'else' expressionLet(lesc,resc):elseBr
    => SOME(elseBr)
  |
  _ => NONE

*/
pub fn elseBranch(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outElseBranchOpt: Option<(Arc<TplAbsyn::ExpressionBase>, SourceInfo)> = None;
    (outChars, outLineInfo, outElseBranchOpt) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } }, linfo, lesc, resc) => {
                    let mut elseBr: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, elseBr) = expressionLet(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), Some(elseBr.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outElseBranchOpt))
}

/*
must not fail
condArgExp:
  'not' expressionPlus(lesc,resc):lhsExp
    => (true, lhsExp,NONE())
  |
  expressionPlus(lesc,resc):lhsExp
  //  condArgRHS:(isNot, rshMExpOpt)
  { isNot = false }
   => (isNot,lhsExp, rhsMExpOpt)
*/
pub fn condArgExp(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, bool, (Arc<TplAbsyn::ExpressionBase>, SourceInfo), Option<Arc<TplAbsyn::MatchingExp>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outIsNot: bool = false;
    let mut outLHSExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut outRHSMExpOpt: Option<Arc<TplAbsyn::MatchingExp>> = None;
    (outChars, outLineInfo, outIsNot, outLHSExpression, outRHSMExpOpt) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: chars } } }, linfo, lesc, resc) => {
                    let mut lhsExp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, lhsExp) = expressionPlus(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), true, lhsExp.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut lhsExp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, lhsExp) = expressionPlus(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), false, lhsExp.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outIsNot, outLHSExpression, outRHSMExpOpt))
}

/*
condArgRHS:
  'is' 'not' matchBinding:rhsMExp  =>  (true, SOME(rhsMexp))
  |
  'is' matchBinding:rhsMExp  =>  (false, SOME(rhsMexp))
  |
  _ => (false,NONE())
*/
/*
public function condArgRHS
  input list<String> inChars;
  input LineInfo inLineInfo;

  output list<String> outChars;
  output LineInfo outLineInfo;
  output Boolean outIsNot;
  output Option<TplAbsyn.MatchingExp> outRHSMExpOpt;
algorithm
  (outChars, outLineInfo, outIsNot, outRHSMExpOpt) :=
  matchcontinue (inChars, inLineInfo)
    local
      list<String> chars;
      LineInfo linfo;
      String c, lesc, resc;
      Boolean isD, isNot;
      TplAbsyn.Ident id;
      TplAbsyn.PathIdent name;
      TplAbsyn.TypedIdents fields,inargs,outargs;
      TplAbsyn.TypeSignature ts;
      Tpl.StringToken st;
      TplAbsyn.Expression exp, bexp, lhsExp, elseBr;
      TplAbsyn.MatchingExp rhsMExp;
      Option<TplAbsyn.MatchingExp> rhsMExpOpt;
      Option<TplAbsyn.Expression> elseBrOpt;
      list<TplAbsyn.Expression> expLst;
      TplAbsyn.EscOption sopt;
      list<TplAbsyn.EscOption> opts;

    case ("i"::"s":: chars, linfo)
      algorithm
        afterKeyword(chars);
        (chars, linfo) = interleave(chars, linfo);
        ("n"::"o"::"t":: chars) = chars;
        afterKeyword(chars);
        (chars, linfo) = interleave(chars, linfo);
        (chars, linfo, rhsMExp) = matchBinding(chars, linfo);
      then (chars, linfo, true, SOME(rhsMExp));

    case ("i"::"s":: chars, linfo)
      algorithm
        afterKeyword(chars);
        (chars, linfo) = interleave(chars, linfo);
        (chars, linfo, rhsMExp) = matchBinding(chars, linfo);
      then (chars, linfo, false, SOME(rhsMExp));


    else (inChars, inLineInfo, false, NONE());

  end matchcontinue;
end condArgRHS;
*/
/*
optional, can fail
matchExp(lesc,resc):
  'match' expressionIf:exp
    matchCaseList(lesc,resc):mcaseLst  { (_::_) = mcaseLst }//not optional
    matchElseCase(lesc,resc):elseLst
    matchEndMatch
   => MATCH(exp, listAppend(mcaseLst, elseLst))
  //|
  //matchCaseList(lesc,resc):mcaseLst { (_::_) = mcaseLst }
  //=> MATCH(BOUND_VALUE(IDENT("it")), mcaseLst)
*/
pub fn matchExp(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ "m", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "h", tail: startChars } } } } }, startLInfo, lesc, resc) => {
            let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut linfo: LineInfo;
            let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut mcaseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
            let mut elseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
            let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            afterKeyword(startChars.clone())?;
            (chars, linfo) = interleave(startChars.clone(), startLInfo.clone())?;
            (chars, linfo, exp) = expressionIf(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            (chars, linfo, mcaseLst) = matchCaseListNoOpt(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, elseLst) = matchElseCase(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            mcaseLst = listAppend(mcaseLst.clone(), elseLst.clone());
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo) = matchEndMatch(chars.clone(), linfo.clone())?;
            sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), startLInfo.clone(), 5)?, chars.clone(), linfo.clone())?;
            (chars.clone(), linfo.clone(), (Arc::new(TplAbsyn::ExpressionBase::MATCH { matchExp: exp.clone(), cases: mcaseLst.clone() }), sinfo.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outExpression))
}

/*
matchCase(lesc,resc):
  'case'  matchBinding:mexp  matchCaseHeads(): mexpHeadLst
  'then'  expression:exp
     => makeMatchCaseLst(mexp::mexpHeadLst,exp)
*/
pub fn matchCase(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchCaseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
    (outChars, outLineInfo, outMatchCaseLst) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } }, linfo, lesc, resc) => {
            let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
            let mut mexpHeadList: Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>> = metamodelica::nil();
            let mut matchCaseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            afterKeyword(chars.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, mexpHeadList) = matchCaseHeads(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, exp) = thenBranch(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
            matchCaseLst = makeMatchCaseLst(cons(mexp.clone(), mexpHeadList.clone()), exp.clone())?;
            (chars.clone(), linfo.clone(), matchCaseLst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outMatchCaseLst))
}

/*
matchElseCase(lesc,resc):
  'else' expression:exp
    => {(REST_MATCH(), exp)}
  |
  _ => {}
*/
pub fn matchElseCase(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchCaseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
    (outChars, outLineInfo, outMatchCaseLst) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "l", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } }, linfo, lesc, resc) => {
                    let mut exp: (Arc<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    Ok((chars.clone(), linfo.clone(), list![(Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH), exp.clone())]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchCaseLst))
}

/*
matchEndMatch:
  'end' 'match'
  |
  _
*/
pub fn matchEndMatch(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: chars } } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "m", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "h", tail: __pa0 } } } } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    afterKeyword(chars.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
matchCaseHeads(lesc,resc):
  'case'  matchBinding:mexp  matchCaseHeads(): mexpHeadLst
     => mexp :: mexpHeadLst
  |
  _ => {}
*/
pub fn matchCaseHeads(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMExpHeadLst: Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>> = metamodelica::nil();
    (outChars, outLineInfo, outMExpHeadLst) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "c", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: Deref @ metamodelica::List::Cons { head: Deref @ "e", tail: chars } } } }, linfo) => {
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut mexpHeadList: Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexpHeadList) = matchCaseHeads(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons(mexp.clone(), mexpHeadList.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMExpHeadLst))
}

pub fn makeMatchCaseLst(mut inMExpHeadLst: Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>>, mut inExpression: (Arc<TplAbsyn::ExpressionBase>, SourceInfo)) -> Result<Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>>> {
    let mut outMatchCaseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
    outMatchCaseLst = (::match_deref::match_deref! { match &((inMExpHeadLst.clone(), inExpression.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: mexp, tail: mexpHeadList }, exp) => {
            let mut matchCaseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
            matchCaseLst = makeMatchCaseLst(mexpHeadList.clone(), exp.clone())?;
            cons((mexp.clone(), exp.clone()), matchCaseLst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMatchCaseLst)
}

/*
matchCaseList(lesc,resc):
  matchCase(lesc,resc):mcaseLst  matchCaseList(lesc,resc):mcrest
    => listAppend(mcaseLst, mcrest)
  |
  _ => {}
*/
pub fn matchCaseList(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchCases: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
    (outChars, outLineInfo, outMatchCases) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut mcaseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
                    let mut mcrest: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, mcaseLst) = matchCase(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mcrest) = matchCaseList(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    mcaseLst = listAppend(mcaseLst.clone(), mcrest.clone());
                    Ok((chars.clone(), linfo.clone(), mcaseLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchCases))
}

pub fn matchCaseListNoOpt(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inLeftEsc: ArcStr, mut inRightEsc: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchCases: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
    (outChars, outLineInfo, outMatchCases) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc.clone(), inRightEsc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut mcaseLst: Arc<metamodelica::List<(Arc<TplAbsyn::MatchingExp>, (Arc<TplAbsyn::ExpressionBase>, SourceInfo))>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, mcaseLst) = matchCaseList(chars.clone(), linfo.clone(), (lesc.clone()).clone(), (resc.clone()).clone())?;
                    ::match_deref::match_deref! { match &(mcaseLst.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: _ } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((chars.clone(), linfo.clone(), mcaseLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _, _) => {
                    let mut linfo = (*linfo).clone();
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), cons((literal!("c")).clone(), cons((literal!("a")).clone(), cons((literal!("s")).clone(), cons((literal!("e")).clone(), metamodelica::nil())))))?) {
                        (_, false) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected keyword 'case' at the position.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!! TplParser.matchCaseListNoOpt failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchCases))
}

/*
matchBinding:
  matchBinding_base:headMExp  matchBinding_tail(headMExp):mexp
    => mexp

*/
pub fn matchBinding(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::MatchingExp>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
    (outChars, outLineInfo, outMatchingExp) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (chars, linfo) => {
            let mut headMExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
            let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, headMExp) = matchBinding_base(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
            (chars, linfo, mexp) = matchBinding_tail(chars.clone(), linfo.clone(), headMExp.clone())?;
            (chars.clone(), linfo.clone(), mexp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
matchBinding_tail(headMExp):
  '::' matchBinding:restMExp
    => LIST_CONS_MATCH(headMExp, restMExp)
  |
  _ => headMExp
*/
pub fn matchBinding_tail(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inHeadMatchingExp: Arc<TplAbsyn::MatchingExp>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::MatchingExp>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inHeadMatchingExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ":", tail: Deref @ metamodelica::List::Cons { head: Deref @ ":", tail: chars } }, linfo, headMExp) => {
                    let mut restMExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, restMExp) = matchBinding(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::LIST_CONS_MATCH { head: headMExp.clone(), rest: restMExp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inHeadMatchingExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
matchBinding_base:
  'SOME' someBinding_rest:mexp
    => SOME_MATCH(mexp)
  |
  'NONE' takeEmptyBraces
    => NONE_MATCH()
  |
  '(' matchBinding:headMExp  tupleOrSingleMatch(headMExp):mexp ')'
    => mexp
  |
  '{' '}'
    => LIST_MATCH({})
  |
  '{' matchBinding:headMExp  listMatch_rest:mrest '}
    => LIST_MATCH(headMExp :: mrest)
  |
  stringConstant:strRevList
    => STRING_MATCH(stringAppendList(listReverse(strRevList))
  |
  literalConstant:(str,litType)
    => LITERAL_MATCH(str,litType)
  |
  '_'
    => REST_MATCH()
  |
  pathIdent:pid  afterIdentBinding(pid):mexp
    => mexp
*/
pub fn matchBinding_base(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::MatchingExp>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "S", tail: Deref @ metamodelica::List::Cons { head: Deref @ "O", tail: Deref @ metamodelica::List::Cons { head: Deref @ "M", tail: Deref @ metamodelica::List::Cons { head: Deref @ "E", tail: chars } } } }, linfo) => {
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = someBinding_rest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::SOME_MATCH { value: mexp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "N", tail: Deref @ metamodelica::List::Cons { head: Deref @ "O", tail: Deref @ metamodelica::List::Cons { head: Deref @ "N", tail: Deref @ metamodelica::List::Cons { head: Deref @ "E", tail: chars } } } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo) = takeEmptyBraces(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(crate::TplAbsyn::MatchingExp::NONE_MATCH)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo) => {
                    let mut headMExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, headMExp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = tupleOrSingleMatch(chars.clone(), linfo.clone(), headMExp.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
                    Ok((chars.clone(), linfo.clone(), mexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "{", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "}", tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::LIST_MATCH { listElts: metamodelica::nil() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "{", tail: chars }, linfo) => {
                    let mut headMExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut mrest: Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, headMExp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mrest) = listMatch_rest(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("}")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::LIST_MATCH { listElts: cons(headMExp.clone(), mrest.clone()) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "_", tail: chars }, linfo) => {
                    Ok((chars.clone(), linfo.clone(), Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut strRevList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, strRevList) = stringConstant(chars.clone(), linfo.clone())?;
                    r#str = stringAppendList(strRevList.clone().reverse());
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::STRING_MATCH { value: (r#str.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, r#str, ts) = literalConstant(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::LITERAL_MATCH { value: (r#str.clone()).clone(), litType: ts.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, pid) = pathIdent(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = afterIdentBinding(chars.clone(), linfo.clone(), pid.clone())?;
                    Ok((chars.clone(), linfo.clone(), mexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected a valid match binding expression at the position.")).clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::LITERAL_MATCH { value: (literal!("#Error#")).clone(), litType: Arc::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: (literal!("#Error#")).clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
someBinding_rest:
  '(' '__' ')'
    => SOME_MATCH(REST_MATCH())
  |
  '(' matchBinding:mexp ')'
    => SOME_MATCH(mexp)
  |
  _ => SOME_MATCH(REST_MATCH())
*/
pub fn someBinding_rest(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::MatchingExp>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "_", tail: Deref @ metamodelica::List::Cons { head: Deref @ "_", tail: __pa0 } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo) => {
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
                    Ok((chars.clone(), linfo.clone(), mexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
takeEmptyBraces:
  '(' ')'
  |
  _
*/
pub fn takeEmptyBraces(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo))
}

/*
tupleOrSingleMatch(headMExp):
  ',' matchBinding:secMExp  listMatch_rest:mrest
    => TUPLE_MATCH(headMExp :: secMExp :: mrest)
  |
  _ => headMExp

*/
pub fn tupleOrSingleMatch(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inHeadMatchingExp: Arc<TplAbsyn::MatchingExp>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::MatchingExp>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inHeadMatchingExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ",", tail: chars }, linfo, headMExp) => {
                    let mut secMExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut mrest: Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, secMExp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mrest) = listMatch_rest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::TUPLE_MATCH { tupleArgs: cons(headMExp.clone(), cons(secMExp.clone(), mrest.clone())) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inHeadMatchingExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
listMatch_rest:
  ',' matchBinding:mexp  listMatch_rest:mrest
    => mexp :: mrest
  |
  _ => {}

*/
pub fn listMatch_rest(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchingExpListRest: Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>> = metamodelica::nil();
    (outChars, outLineInfo, outMatchingExpListRest) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut mrest: Arc<metamodelica::List<Arc<TplAbsyn::MatchingExp>>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mrest) = listMatch_rest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons(mexp.clone(), mrest.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchingExpListRest))
}

/*
afterIdentBinding(pid):
  '(' ')'
    => RECORD_MATCH(pid, {})
  |
  '(' '__' ')'
    => RECORD_MATCH(pid, {}) //TODO: to be RECORD_TYPE_MATCH(pid)
  |
  '(' fieldBinding:fb  fieldBinding_rest:fbs ')'
    => RECORD_MATCH(pid, fb::fbs)
  |
  {pid is PATH_IDENT}
  => error "Expected '(' after the dot path."
  //RECORD_MATCH(pid, {})
  |
  {pid is IDENT(id)}
  'as' matchBinding:mexp
    => BIND_AS_MATCH(id, mexp)
  |
  {pid is IDENT(id)}
  _ => BIND_MATCH(id)
*/
pub fn afterIdentBinding(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo, mut inPathIdent: Arc<TplAbsyn::PathIdent>) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<TplAbsyn::MatchingExp>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inPathIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo, pid) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ ")", tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: pid.clone(), fieldMatchings: metamodelica::nil() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo, pid) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "_", tail: Deref @ metamodelica::List::Cons { head: Deref @ "_", tail: __pa0 } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa0.clone();
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: pid.clone(), fieldMatchings: metamodelica::nil() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: chars }, linfo, pid) => {
                    let mut fb: (ArcStr, Arc<TplAbsyn::MatchingExp>);
                    let mut fbs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::MatchingExp>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, fb) = fieldBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, fbs) = fieldBinding_rest(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!(")")).clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: pid.clone(), fieldMatchings: cons(fb.clone(), fbs.clone()) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, pid @ Deref @ TplAbsyn::PathIdent::PATH_IDENT { ident: _, path: _ }) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), (literal!("Expected '(' after the dot path.")).clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: pid.clone(), fieldMatchings: metamodelica::nil() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "s", tail: chars } }, linfo, Deref @ TplAbsyn::PathIdent::IDENT { ident: id }) => {
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(chars.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::BIND_AS_MATCH { bindIdent: (id.clone()).clone(), matchingExp: mexp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, Deref @ TplAbsyn::PathIdent::IDENT { ident: id }) => {
                    Ok((chars.clone(), linfo.clone(), Arc::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: (id.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!! TplParser.afterIdentBinding failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
must not fail
fieldBinding:
  identifier:fldId '=' matchBinding:mexp
    => (fldId, mexp)
*/
pub fn fieldBinding(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, (ArcStr, Arc<TplAbsyn::MatchingExp>))> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outFieldBinding: (ArcStr, Arc<TplAbsyn::MatchingExp>);
    (outChars, outLineInfo, outFieldBinding) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut mexp: Arc<TplAbsyn::MatchingExp> = Arc::new(TplAbsyn::MatchingExp::NONE_MATCH);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), (literal!("=")).clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), mexp.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- !!! TplParser.fieldBinding failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outFieldBinding))
}

/*
fieldBinding_rest:
  ',' fieldBinding:fb  fieldBinding_rest:fbs
    => fb :: fbs
  |
  _ => {}

*/
pub fn fieldBinding_rest(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::MatchingExp>)>>)> {
    let mut outChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLineInfo: LineInfo;
    let mut outFieldBindingsRest: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::MatchingExp>)>> = metamodelica::nil();
    (outChars, outLineInfo, outFieldBindingsRest) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut fb: (ArcStr, Arc<TplAbsyn::MatchingExp>);
                    let mut fbs: Arc<metamodelica::List<(ArcStr, Arc<TplAbsyn::MatchingExp>)>> = metamodelica::nil();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, fb) = fieldBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone())?;
                    (chars, linfo, fbs) = fieldBinding_rest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), cons(fb.clone(), fbs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChars, outLineInfo, outFieldBindingsRest))
}

/*
annotationFooter:
  'annotation(...)' => str
  |
  _ => ""

*/
fn annotationFooter(mut inChars: Arc<metamodelica::List<ArcStr>>, mut inLineInfo: LineInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, LineInfo, ArcStr)> {
    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut linfo: LineInfo;
    let mut footer: ArcStr = arcstr::literal!("");
    (chars, linfo, footer) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: chars } } } } } } } } } }, linfo) => {
            let mut footerChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut chars = (*chars).clone();
            (footerChars, chars) = List::split(inChars.clone(), List::position((literal!(";")).clone(), inChars.clone())? + 1)?;
            footer = stringAppendList(footerChars.clone());
            (chars.clone(), linfo.clone(), footer.clone())
        },
        _ => {
            (inChars.clone(), inLineInfo.clone(), literal!("annotation(__OpenModelica_generator=\"Susan\");"))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((chars, linfo, footer))
}

