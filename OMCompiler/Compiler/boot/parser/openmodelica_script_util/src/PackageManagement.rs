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

use crate::Curl;
use crate::Unzip;
use openmodelica_util::Autoconf;
use openmodelica_util::AvlSetString;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::JSON;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub mod AvailableLibraries {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = Arc<VersionMap::Tree>;

    pub fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (inKey.clone()).clone();
        outString
    }

    pub fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (VersionMap::printTreeStr(inValue.clone())?).clone();
        Ok(outString)
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

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<VersionMap::Tree>, Arc<VersionMap::Tree>, ArcStr) -> Result<Arc<VersionMap::Tree>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value = Arc::new(VersionMap::Tree::EMPTY);
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
            let mut value: Value = Arc::new(VersionMap::Tree::EMPTY);
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare((inKey.clone()).clone(), (var_field!((*tree).key, Tree::LEAF).clone()).clone());
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }), right: Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }) });
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
        let mut value: Value = Arc::new(VersionMap::Tree::EMPTY);
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

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(ArcStr, Arc<VersionMap::Tree>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<VersionMap::Tree>, Arc<VersionMap::Tree>, ArcStr) -> Result<Arc<VersionMap::Tree>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key = arcstr::literal!("");
        let mut value: Value = Arc::new(VersionMap::Tree::EMPTY);
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Arc<VersionMap::Tree>>) -> Result<Arc<VersionMap::Tree>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<VersionMap::Tree>>) -> Result<Value> + 'static>;

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
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }), right: Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }) });
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<VersionMap::Tree>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
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

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<VersionMap::Tree>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
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

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<VersionMap::Tree>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
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

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(ArcStr, Arc<VersionMap::Tree>) -> Result<()> + 'static>) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let () = (::match_deref::match_deref! { match &(tree.clone()) {
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

    pub fn fromList(mut inValues: Arc<metamodelica::List<(ArcStr, Arc<VersionMap::Tree>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<VersionMap::Tree>, Arc<VersionMap::Tree>, ArcStr) -> Result<Arc<VersionMap::Tree>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY);
        let mut key: Key = arcstr::literal!("");
        let mut value: Value = Arc::new(VersionMap::Tree::EMPTY);
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value = Arc::new(VersionMap::Tree::EMPTY);
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
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Option<Arc<VersionMap::Tree>> {
        let mut value: Option<Arc<VersionMap::Tree>> = None;
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
            return Ok(comp.clone());
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<VersionMap::Tree>, Arc<VersionMap::Tree>, ArcStr) -> Result<Arc<VersionMap::Tree>> + 'static>) -> Result<Arc<Tree>> {
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

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<VersionMap::Tree>>>) -> Arc<metamodelica::List<Arc<VersionMap::Tree>>> {
        let mut lst: Arc<metamodelica::List<Arc<VersionMap::Tree>>> = lst;
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

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<VersionMap::Tree>) -> Result<Arc<VersionMap::Tree>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = Arc::new(VersionMap::Tree::EMPTY);
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
            let mut new_value: Value = Arc::new(VersionMap::Tree::EMPTY);
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

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<VersionMap::Tree>, FT) -> Result<(Arc<VersionMap::Tree>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = Arc::new(VersionMap::Tree::EMPTY);
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
            let mut new_value: Value = Arc::new(VersionMap::Tree::EMPTY);
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
        let mut outTree: Arc<Tree> = Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY);
        outTree
    }

    pub fn printNodeStr(mut inNode: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr((var_field!((*inNode).key, Tree::NODE).clone()).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr((var_field!((*inNode).key, Tree::LEAF).clone()).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
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
        let mut left: Option<Arc<Tree>> = None;
        let mut right: Option<Arc<Tree>> = None;
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::PackageManagement::AvailableLibraries::Tree::EMPTY), node.clone())?
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

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(ArcStr, Arc<VersionMap::Tree>)>>) -> Arc<metamodelica::List<(ArcStr, Arc<VersionMap::Tree>)>> {
        let mut lst: Arc<metamodelica::List<(ArcStr, Arc<VersionMap::Tree>)>> = lst;
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
        let mut outTree: Arc<Tree> = add(tree.clone(), (key.clone()).clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Arc<VersionMap::Tree>, Arc<VersionMap::Tree>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<VersionMap::Tree>, Arc<VersionMap::Tree>, ArcStr) -> Result<Arc<VersionMap::Tree>> + 'static>)).unwrap();
        outTree
    }

}

pub mod VersionMap {
    use super::*;
    pub type Key = SemanticVersion::Version;

    pub type Value = ArcStr;

    pub fn keyStr(mut inKey: Key) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (SemanticVersion::toString(inKey.clone())?).clone();
        Ok(outString)
    }

    pub fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (inValue.clone()).clone();
        outString
    }

    pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
        let mut outResult: i32 = 0;
        outResult = SemanticVersion::compare(inKey1.clone(), inKey2.clone(), true, true)?;
        Ok(outResult)
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

    pub type ValueNode = SemanticVersion::Version;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: inKey.clone(), value: (inValue.clone()).clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value = arcstr::literal!("");
            let mut key_comp: i32 = 0;
            key_comp = keyCompare(inKey.clone(), key.clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey.clone(), (inValue.clone()).clone(), conflictFunc.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey.clone(), (inValue.clone()).clone(), conflictFunc.clone())?);
            } else {
                value = (conflictFunc((inValue.clone()).clone(), (var_field!((*tree).value, Tree::NODE).clone()).clone(), key.clone())?).clone();
                if !(referenceEq(&var_field!((*tree).value, Tree::NODE).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::NODE; value = value.clone());
                }
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            let mut value: Value = arcstr::literal!("");
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare(inKey.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: (var_field!((*tree).value, Tree::LEAF).clone()).clone(), height: 2, left: Arc::new(Tree::LEAF { key: inKey.clone(), value: (inValue.clone()).clone() }), right: Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: (var_field!((*tree).value, Tree::LEAF).clone()).clone(), height: 2, left: Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: inKey.clone(), value: (inValue.clone()).clone() }) });
            } else {
                value = (conflictFunc((inValue.clone()).clone(), (var_field!((*tree).value, Tree::LEAF).clone()).clone(), var_field!((*tree).key, Tree::LEAF).clone())?).clone();
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
        let mut value: Value = arcstr::literal!("");
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

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(SemanticVersion::Version, ArcStr)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key;
        let mut value: Value = arcstr::literal!("");
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), (value.clone()).clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<ArcStr>) -> Result<ArcStr> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<ArcStr>) -> Result<Value> + 'static>;

        let mut tree: Arc<Tree> = tree;
        let mut key_comp: i32 = 0;
        let mut new_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => Arc::new(Tree::LEAF { key: key.clone(), value: (r#fn(None)?).clone() }),
        Deref @ Tree::NODE { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::NODE).clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), key.clone(), r#fn.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), key.clone(), r#fn.clone())?);
            } else {
                assign_variant_field!(tree => Tree::NODE; value = r#fn(Some((var_field!((*tree).value, Tree::NODE).clone()).clone()))?);
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: (var_field!((*tree).value, Tree::LEAF).clone()).clone(), height: 2, left: Arc::new(Tree::LEAF { key: key.clone(), value: (r#fn(None)?).clone() }), right: Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: (var_field!((*tree).value, Tree::LEAF).clone()).clone(), height: 2, left: Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: key.clone(), value: (r#fn(None)?).clone() }) });
            } else {
                assign_variant_field!(tree => Tree::LEAF; value = r#fn(Some((var_field!((*tree).value, Tree::LEAF).clone()).clone()))?);
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

        let mut outResult: FT = inStartValue.clone();
        outResult = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            outResult = fold(var_field!((*inTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult = inFunc(key.clone(), (value.clone()).clone(), outResult.clone()).unwrap();
            outResult = fold(var_field!((*inTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            outResult = inFunc(key.clone(), (value.clone()).clone(), outResult.clone()).unwrap();
            outResult.clone()
        },
        _ => {
            outResult.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outResult
    }

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

        let mut value: FT = value;
        value = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), (var_field!((*tree).value, Tree::NODE).clone()).clone(), value.clone()).unwrap();
            if c.clone() {
                value = foldCond(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), value.clone());
                value = foldCond(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), value.clone());
            }
            value.clone()
        },
        Deref @ Tree::LEAF { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), (var_field!((*tree).value, Tree::LEAF).clone()).clone(), value.clone()).unwrap();
            value.clone()
        },
        _ => {
            value.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value
    }

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
        pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), (var_field!((*tree).value, Tree::NODE).clone()).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), (var_field!((*tree).value, Tree::LEAF).clone()).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (foldArg1, foldArg2)
    }

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr) -> Result<()> + 'static>) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            forEach(var_field!((*tree).left, Tree::NODE).clone(), func.clone())?;
            func(var_field!((*tree).key, Tree::NODE).clone(), (var_field!((*tree).value, Tree::NODE).clone()).clone())?;
            forEach(var_field!((*tree).right, Tree::NODE).clone(), func.clone())?;
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            func(var_field!((*tree).key, Tree::LEAF).clone(), (var_field!((*tree).value, Tree::LEAF).clone()).clone())?;
            ()
        },
        Deref @ Tree::EMPTY { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
        Ok(())
    }

    pub fn fromList(mut inValues: Arc<metamodelica::List<(SemanticVersion::Version, ArcStr)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY);
        let mut key: Key;
        let mut value: Value = arcstr::literal!("");
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), (value.clone()).clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value = arcstr::literal!("");
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        value = ((::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => var_field!((*tree).value, Tree::LEAF).clone(),
        (0, Deref @ Tree::NODE { .. }) => var_field!((*tree).value, Tree::NODE).clone(),
        (1, Deref @ Tree::NODE { .. }) => get(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(value)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Result<Option<ArcStr>> {
        let mut value: Option<ArcStr> = None;
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => key.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => Some((var_field!((*tree).value, Tree::LEAF).clone()).clone()),
        (0, Deref @ Tree::NODE { .. }) => Some((var_field!((*tree).value, Tree::NODE).clone()).clone()),
        (1, Deref @ Tree::NODE { .. }) => getOpt(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ Tree::NODE { .. }) => getOpt(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(value)
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
            return Ok(comp.clone());
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ Tree::EMPTY { .. } => tree.clone(),
        Deref @ Tree::NODE { .. } => {
            tree = add(tree.clone(), var_field!((*treeToJoin).key, Tree::NODE).clone(), (var_field!((*treeToJoin).value, Tree::NODE).clone()).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).left, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).right, Tree::NODE).clone(), conflictFunc.clone())?;
            tree.clone()
        },
        Deref @ Tree::LEAF { .. } => add(tree.clone(), var_field!((*treeToJoin).key, Tree::LEAF).clone(), (var_field!((*treeToJoin).value, Tree::LEAF).clone()).clone(), conflictFunc.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn listKeys(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<SemanticVersion::Version>>) -> Arc<metamodelica::List<SemanticVersion::Version>> {
        let mut lst: Arc<metamodelica::List<SemanticVersion::Version>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { key, .. } => {
            lst = listKeys(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(key.clone(), lst.clone());
            lst = listKeys(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { key, .. } => {
            cons(key.clone(), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn listKeysReverse(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<SemanticVersion::Version>>) -> Arc<metamodelica::List<SemanticVersion::Version>> {
        let mut lst: Arc<metamodelica::List<SemanticVersion::Version>> = lst;
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

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
        let mut lst: Arc<metamodelica::List<ArcStr>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { value, .. } => {
            lst = listValues(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons((value.clone()).clone(), lst.clone());
            lst = listValues(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { value, .. } => {
            cons((value.clone()).clone(), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr) -> Result<ArcStr> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = arcstr::literal!("");
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            new_left = map(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone());
            new_value = (inFunc(key.clone(), (value.clone()).clone()).unwrap()).clone();
            new_right = map(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: (new_value.clone()).clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value = arcstr::literal!("");
            new_value = (inFunc(key.clone(), (value.clone()).clone()).unwrap()).clone();
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

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr, FT) -> Result<(ArcStr, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = arcstr::literal!("");
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            (new_left, outResult) = mapFold(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            (new_value, outResult) = inFunc(key.clone(), (value.clone()).clone(), outResult.clone()).unwrap();
            (new_right, outResult) = mapFold(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: (new_value.clone()).clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value = arcstr::literal!("");
            (new_value, outResult) = inFunc(key.clone(), (value.clone()).clone(), outResult.clone()).unwrap();
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
        let mut outTree: Arc<Tree> = Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY);
        outTree
    }

    pub fn printNodeStr(mut inNode: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr((var_field!((*inNode).value, Tree::NODE).clone()).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr((var_field!((*inNode).value, Tree::LEAF).clone()).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
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
        let mut left: Option<Arc<Tree>> = None;
        let mut right: Option<Arc<Tree>> = None;
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::PackageManagement::VersionMap::Tree::EMPTY), node.clone())?
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
        (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => Arc::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: (var_field!((*orig).value, Tree::NODE).clone()).clone() }),
        (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig.clone(),
        (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE).clone(), left.clone()) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE).clone(), right.clone())) {orig.clone()} else {Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: (var_field!((*orig).value, Tree::NODE).clone()).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() })},
        (Deref @ Tree::LEAF { .. }, _, _) => Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: (var_field!((*orig).value, Tree::LEAF).clone()).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() }),
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

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(SemanticVersion::Version, ArcStr)>>) -> Arc<metamodelica::List<(SemanticVersion::Version, ArcStr)>> {
        let mut lst: Arc<metamodelica::List<(SemanticVersion::Version, ArcStr)>> = lst;
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
        let mut outTree: Arc<Tree> = add(tree.clone(), key.clone(), (value.clone()).clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, ArcStr, ArcStr, SemanticVersion::Version)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr> + 'static>)).unwrap();
        outTree
    }

}

pub const metaDataFileName: &'static str = "openmodelica.metadata.json";

pub fn getInstalledLibraries() -> Result<Arc<AvailableLibraries::Tree>> {
    let mut tree: Arc<AvailableLibraries::Tree> = Arc::new(AvailableLibraries::Tree::EMPTY);
    let mut mp: ArcStr = arcstr::literal!("");
    let mut gd: ArcStr = arcstr::literal!("");
    let mut first: ArcStr = arcstr::literal!("");
    let mut ver: ArcStr = arcstr::literal!("");
    let mut lib: ArcStr = arcstr::literal!("");
    let mut mps: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut dirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut versions: Arc<VersionMap::Tree> = Arc::new(VersionMap::Tree::EMPTY);
    mp = (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone();
    gd = (arcstr::literal!(Autoconf::groupDelimiter)).clone();
    mps = System::strtok((mp.clone()).clone(), (gd.clone()).clone());
    tree = AvailableLibraries::new();
    files = metamodelica::nil();
    dirs = metamodelica::nil();
    for mut mp in &*mps.clone() {
        let mut mp = mp.clone();
        files = listAppend(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut file in (System::moFiles((mp.clone()).clone())).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*file.clone()); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), files.clone());
        dirs = listAppend(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut dir in (getLibrarySubdirectories((mp.clone()).clone())).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*dir.clone()); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), dirs.clone());
    }
    for mut path in &*listAppend(files.clone(), dirs.clone()) {
        let mut path = path.clone();
        lib = (System::basename((path.clone()).clone())).clone();
        if StringUtil::endsWith((lib.clone()).clone(), (literal!(".mo")).clone()) {
            lib = (Util::removeLast3Char((lib.clone()).clone())?).clone();
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok((lib.clone()).clone(), (literal!(" ")).clone())) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        first = __pa0.clone();
        rest = __pa1.clone();
        ver = stringDelimitList(rest.clone(), (literal!(" ")).clone());
        versions = if (AvailableLibraries::hasKey(tree.clone(), (first.clone()).clone())?) {AvailableLibraries::get(tree.clone(), (first.clone()).clone())?} else {VersionMap::new()};
        versions = VersionMap::add(versions.clone(), SemanticVersion::parse((ver.clone()).clone(), false)?, (path.clone()).clone(), (std::sync::Arc::new(fnptr!(VersionMap::addConflictReplace, ArcStr, ArcStr, SemanticVersion::Version)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr> + 'static>))?;
        tree = AvailableLibraries::add(tree.clone(), (first.clone()).clone(), versions.clone(), (std::sync::Arc::new(fnptr!(AvailableLibraries::addConflictReplace, Arc<VersionMap::Tree>, Arc<VersionMap::Tree>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<VersionMap::Tree>, Arc<VersionMap::Tree>, ArcStr) -> Result<Arc<VersionMap::Tree>> + 'static>))?;
    }
    Ok(tree)
}

pub fn getInstalledLibraryVersions(mut libraryName: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut libraryVersions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut tree: Arc<AvailableLibraries::Tree> = Arc::new(AvailableLibraries::Tree::EMPTY);
    let mut versionTree: Arc<VersionMap::Tree> = Arc::new(VersionMap::Tree::EMPTY);
    let mut versions: Arc<metamodelica::List<SemanticVersion::Version>> = metamodelica::nil();
    let mut versionStr: ArcStr = arcstr::literal!("");
    tree = getInstalledLibraries()?;
    versionTree = AvailableLibraries::get(tree.clone(), (libraryName.clone()).clone())?;
    versions = VersionMap::listKeys(versionTree.clone(), metamodelica::nil());
    for mut version in &*versions.clone() {
        let mut version = version.clone();
        versionStr = (VersionMap::keyStr(version.clone())?).clone();
        if stringCompare((versionStr.clone()).clone(), (literal!("")).clone()) > 0 {
            libraryVersions = cons((versionStr.clone()).clone(), libraryVersions.clone());
        }
    }
    Ok(libraryVersions)
}

pub fn getLibrarySubdirectories(mut inPath: ArcStr) -> Arc<metamodelica::List<ArcStr>> {
    let mut outSubdirectories: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut allSubdirectories: Arc<metamodelica::List<ArcStr>> = System::subDirectories((inPath.clone()).clone());
    let mut pd: ArcStr = arcstr::literal!(Autoconf::pathDelimiter);
    for mut dir in &*allSubdirectories.clone() {
        let mut dir = dir.clone();
        if System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPath.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*literal!("package.mo")); ArcStr::from(__mm_s) }).clone()) {
            outSubdirectories = cons((dir.clone()).clone(), outSubdirectories.clone());
        }
    }
    outSubdirectories
}

pub fn providesExpectedVersion(mut version: ArcStr, mut provides: Arc<JSON::JSON>, mut wantedVersion: SemanticVersion::Version) -> Result<bool> {
    let mut matches: bool = false;
    let mut providedVersions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut thisVersion: SemanticVersion::Version;
    let () = (::match_deref::match_deref! { match &(wantedVersion.clone()) {
        SemanticVersion::Version::NONSEMVER { version: r#str } if (r#str.clone() == literal!("default") || r#str.clone() == literal!("")) => {
            matches = true;
            return Ok(matches.clone());
            bail!("fail")
        },
        SemanticVersion::Version::SEMVER { prerelease: Deref @ metamodelica::List::Cons { head: Deref @ "default", tail: Deref @ metamodelica::List::Nil }, patch: 0, minor: 0, major: 0, .. } => {
            matches = true;
            return Ok(matches.clone());
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    providedVersions = JSON::getStringList(provides.clone())?;
    matches = false;
    for mut v in &*cons((version.clone()).clone(), providedVersions.clone()) {
        let mut v = v.clone();
        thisVersion = SemanticVersion::parse((v.clone()).clone(), true)?;
        if SemanticVersion::compare(thisVersion.clone(), wantedVersion.clone(), SemanticVersion::isPrerelease(wantedVersion.clone()) && SemanticVersion::isPrerelease(wantedVersion.clone()), false)? == 0 {
            matches = true;
            return Ok(matches.clone());
        }
    }
    Ok(matches)
}

pub static supportLevels: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("fullSupport")).clone(), (literal!("support")).clone(), (literal!("experimental")).clone(), (literal!("obsolete")).clone(), (literal!("unknown")).clone(), (literal!("noSupport")).clone()] });

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SupportLevel {
    noSupport = 1,
    unknown = 2,
    obsolete = 3,
    experimental = 4,
    support = 5,
    fullSupport = 6,
}
impl PartialOrd for SupportLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SupportLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn getSupportLevel(mut obj: Arc<JSON::JSON>) -> Result<SupportLevel> {
    let mut support: SupportLevel = SupportLevel::noSupport;
    support = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ JSON::STRING { r#str: Deref @ "fullSupport" } => SupportLevel::fullSupport.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "support" } => SupportLevel::support.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "experimental" } => SupportLevel::experimental.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "obsolete" } => SupportLevel::obsolete.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "unknown" } => SupportLevel::unknown.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "noSupport" } => SupportLevel::noSupport.clone(),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown support level ")); __mm_s.push_str(&*JSON::toString(obj.clone(), false)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(support)
}

pub fn compareVersionsAndSupportLevel(mut x1: (ArcStr, SemanticVersion::Version, SupportLevel), mut x2: (ArcStr, SemanticVersion::Version, SupportLevel)) -> Result<bool> {
    let mut c: bool = false;
    let mut s1: SupportLevel = SupportLevel::noSupport;
    let mut s2: SupportLevel = SupportLevel::noSupport;
    let mut v1: SemanticVersion::Version;
    let mut v2: SemanticVersion::Version;
    (_, v1, s1) = x1.clone();
    (_, v2, s2) = x2.clone();
    if s1.clone() < s2.clone() {
        c = true;
        return Ok(c.clone());
    } else if s1.clone() > s2.clone() {
        c = false;
        return Ok(c.clone());
    }
    if SemanticVersion::isPrerelease(v1.clone()) != SemanticVersion::isPrerelease(v2.clone()) {
        c = SemanticVersion::isPrerelease(v2.clone());
        return Ok(c.clone());
    }
    c = SemanticVersion::compare(v1.clone(), v2.clone(), true, true)? < 0;
    Ok(c)
}

pub fn updateIndex() -> Result<bool> {
    let mut success: bool = false;
    let mut userLibraries: ArcStr = arcstr::literal!("");
    let mut packageIndex: ArcStr = arcstr::literal!("");
    let url: ArcStr = literal!("https://libraries.openmodelica.org/index/v1/index.json");
    userLibraries = (getUserLibraryPath()?).clone();
    Util::createDirectoryTree((userLibraries.clone()).clone())?;
    packageIndex = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*userLibraries.clone()); __mm_s.push_str(&*literal!("index.json")); ArcStr::from(__mm_s) }).clone();
    if !(Curl::multiDownload(list![(list![(url.clone()).clone()], packageIndex.clone())], Config::noProc()?)) {
        Error::addMessage(Error::ERROR_PKG_INDEX_FAILED_DOWNLOAD.clone(), list![(url.clone()).clone(), (packageIndex.clone()).clone()])?;
        success = false;
    } else {
        Error::addSourceMessage(Error::NOTIFY_PKG_INDEX_DOWNLOAD.clone(), list![(url.clone()).clone()], makeSourceInfo((getIndexPath()?).clone()))?;
        success = true;
    }
    { let __v = None; openmodelica_util::Globals::packageIndexCacheIndex.with(|__root| *__root.borrow_mut() = __v) };
    Ok(success)
}

pub fn upgradeInstalledPackages(mut installNewestVersions: bool) -> Result<bool> {
    let mut success: bool = false;
    let mut installedLibraries: Arc<AvailableLibraries::Tree> = Arc::new(AvailableLibraries::Tree::EMPTY);
    let mut versions: Arc<VersionMap::Tree> = Arc::new(VersionMap::Tree::EMPTY);
    success = true;
    installedLibraries = getInstalledLibraries()?;
    for mut pkg in &*AvailableLibraries::listKeys(installedLibraries.clone(), metamodelica::nil()) {
        let mut pkg = pkg.clone();
        versions = AvailableLibraries::get(installedLibraries.clone(), (pkg.clone()).clone())?;
        for mut version in &*VersionMap::listKeys(versions.clone(), metamodelica::nil()) {
            let mut version = version.clone();
            success = success.clone() && installPackage((pkg.clone()).clone(), (SemanticVersion::toString(version.clone())?).clone(), true, false)?;
        }
        if installNewestVersions.clone() {
            success = success.clone() && installPackage((pkg.clone()).clone(), (literal!("")).clone(), false, false)?;
        }
    }
    Ok(success)
}

pub fn getPackageIndex(mut printError: bool) -> Result<Arc<JSON::JSON>> {
    let mut obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut userLibraries: ArcStr = arcstr::literal!("");
    let mut packageIndex: ArcStr = arcstr::literal!("");
    let mut gd: ArcStr = arcstr::literal!("");
    let mut mp: ArcStr = arcstr::literal!("");
    let mut mps: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    if '__try0: {
        obj = unwrap_break_err!(openmodelica_util::Globals::packageIndexCacheIndex.with(|__root| __root.borrow().clone()).ok_or_else(|| anyhow::anyhow!("getGlobalRoot: empty slot packageIndexCacheIndex")), '__try0);
        return Ok(obj.clone());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    mp = (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone();
    gd = (arcstr::literal!(Autoconf::groupDelimiter)).clone();
    mps = System::strtok((mp.clone()).clone(), (gd.clone()).clone());
    userLibraries = (getUserLibraryPath()?).clone();
    packageIndex = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*userLibraries.clone()); __mm_s.push_str(&*literal!("index.json")); ArcStr::from(__mm_s) }).clone();
    obj = JSON::emptyObject();
    if !(listMember((userLibraries.clone()).clone(), mps.clone())) && !(listMember((Util::removeLastNChar((userLibraries.clone()).clone(), 1)?).clone(), mps.clone())) {
        if printError.clone() {
            Error::addMessage(Error::ERROR_PKG_INDEX_NOT_ON_PATH.clone(), list![(mp.clone()).clone(), (userLibraries.clone()).clone()])?;
        }
        return Ok(obj.clone());
    }
    if !(System::regularFileExists((packageIndex.clone()).clone())) {
        if !(updateIndex()?) {
            return Ok(obj.clone());
        }
    }
    match '__try1: {
        obj = unwrap_break_err!(JSON::parseFile((packageIndex.clone()).clone()), '__try1);
        { let __v = Some(obj.clone()); openmodelica_util::Globals::packageIndexCacheIndex.with(|__root| *__root.borrow_mut() = __v) };
        Ok::<_, anyhow::Error>((obj.clone(),))
    } {
        Ok((__try1_o0,)) => {
            obj = __try1_o0;
        }
        Err(_) => {
            Error::addSourceMessage(Error::ERROR_PKG_INDEX_NOT_PARSED.clone(), list![(packageIndex.clone()).clone()], makeSourceInfo((getIndexPath()?).clone()))?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(obj)
}

pub fn getAllProvidedVersionsForLibrary(mut lib: ArcStr, mut printError: bool) -> Arc<metamodelica::List<ArcStr>> {
    let mut result: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut libobject: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut vers: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut provides: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut tree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    result = metamodelica::nil();
    tree = AvlSetString::new();
    match '__try0: {
        obj = unwrap_break_err!(getPackageIndex(printError.clone()), '__try0);
        libobject = unwrap_break_err!(JSON::get(JSON::get(obj.clone(), (literal!("libs")).clone()).unwrap(), (lib.clone()).clone()), '__try0);
        vers = unwrap_break_err!(JSON::get(libobject.clone(), (literal!("versions")).clone()), '__try0);
        for mut version in &*unwrap_break_err!(JSON::getKeys(vers.clone()), '__try0) {
            let mut version = version.clone();
            tree = unwrap_break_err!(AvlSetString::add(tree.clone(), (version.clone()).clone()), '__try0);
            provides = JSON::getOrDefault(JSON::get(vers.clone(), (version.clone()).clone()).unwrap(), (literal!("provides")).clone(), JSON::emptyArray(0));
            for mut i in 1..=JSON::size(provides.clone()) {
                tree = unwrap_break_err!(AvlSetString::add(tree.clone(), (JSON::getString(JSON::at(provides.clone(), i.clone()).unwrap()).unwrap()).clone()), '__try0);
            }
        }
        result = AvlSetString::listKeys(tree.clone(), metamodelica::nil());
        Ok::<_, anyhow::Error>((libobject.clone(), obj.clone(), result.clone(), vers.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            libobject = __try0_o0;
            obj = __try0_o1;
            result = __try0_o2;
            vers = __try0_o3;
        }
        Err(_) => {
            return result.clone();
        }
    }
    result
}

pub fn versionsThatProvideTheWanted(mut id: ArcStr, mut version: ArcStr, mut printError: bool) -> Arc<metamodelica::List<ArcStr>> {
    let mut result: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut libobject: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut vers: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut wantedVersion: SemanticVersion::Version;
    result = metamodelica::nil();
    match '__try0: {
        obj = unwrap_break_err!(getPackageIndex(printError.clone()), '__try0);
        libobject = unwrap_break_err!(JSON::get(JSON::get(obj.clone(), (literal!("libs")).clone()).unwrap(), (id.clone()).clone()), '__try0);
        vers = unwrap_break_err!(JSON::get(libobject.clone(), (literal!("versions")).clone()), '__try0);
        wantedVersion = unwrap_break_err!(SemanticVersion::parse((version.clone()).clone(), true), '__try0);
        result = List::map(List::sort(({
        let mut __acc: Arc<metamodelica::List<(ArcStr, SemanticVersion::Version, SupportLevel)>> = metamodelica::nil();
        for mut version in (JSON::getKeys(vers.clone()).unwrap()).into_iter().cloned() {
            if !(providesExpectedVersion((version.clone()).clone(), JSON::getOrDefault(JSON::get(vers.clone(), (version.clone()).clone()).unwrap(), (literal!("provides")).clone(), JSON::emptyArray(0)), wantedVersion.clone()).unwrap()) { continue; }
            let __x = (version.clone(), SemanticVersion::parse((version.clone()).clone(), true).unwrap(), getSupportLevel(JSON::get(JSON::get(vers.clone(), (version.clone()).clone()).unwrap(), (literal!("support")).clone()).unwrap()).unwrap());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (std::sync::Arc::new(compareVersionsAndSupportLevel) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, SemanticVersion::Version, SupportLevel), (ArcStr, SemanticVersion::Version, SupportLevel)) -> Result<bool> + 'static>)).unwrap(), std::sync::Arc::new(fnptr!(Util::tuple31, _)));
        Ok::<_, anyhow::Error>((libobject.clone(), obj.clone(), result.clone(), vers.clone(), wantedVersion.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            libobject = __try0_o0;
            obj = __try0_o1;
            result = __try0_o2;
            vers = __try0_o3;
            wantedVersion = __try0_o4;
        }
        Err(_) => {
            return result.clone();
        }
    }
    result
}

pub fn versionsThatConvertFromTheWanted(mut id: ArcStr, mut version: ArcStr, mut printError: bool) -> Arc<metamodelica::List<ArcStr>> {
    let mut result: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut libobject: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut vers: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut wantedVersion: SemanticVersion::Version;
    let mut convertVersion: SemanticVersion::Version;
    let mut convertFrom: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut versionStr: ArcStr = arcstr::literal!("");
    result = metamodelica::nil();
    match '__try0: {
        obj = unwrap_break_err!(getPackageIndex(printError.clone()), '__try0);
        libobject = unwrap_break_err!(JSON::get(JSON::get(obj.clone(), (literal!("libs")).clone()).unwrap(), (id.clone()).clone()), '__try0);
        vers = unwrap_break_err!(JSON::get(libobject.clone(), (literal!("versions")).clone()), '__try0);
        wantedVersion = unwrap_break_err!(SemanticVersion::parse((version.clone()).clone(), true), '__try0);
        for mut v in &*unwrap_break_err!(JSON::getKeys(vers.clone()), '__try0) {
            let mut v = v.clone();
            convertFrom = JSON::getOrDefault(JSON::get(vers.clone(), (v.clone()).clone()).unwrap(), (literal!("convertFromVersion")).clone(), JSON::emptyArray(0));
            for mut i in 1..=JSON::size(convertFrom.clone()) {
                let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(JSON::at(convertFrom.clone(), i.clone()), '__try0)) {
                    Deref @ JSON::STRING { r#str: __pa1 } => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                versionStr = __pa1.clone();
                convertVersion = unwrap_break_err!(SemanticVersion::parse((versionStr.clone()).clone(), true), '__try0);
                if unwrap_break_err!(SemanticVersion::compare(wantedVersion.clone(), convertVersion.clone(), true, false), '__try0) == 0 {
                    result = cons((v.clone()).clone(), result.clone());
                    continue;
                }
            }
        }
        Ok::<_, anyhow::Error>((libobject.clone(), obj.clone(), vers.clone(), wantedVersion.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            libobject = __try0_o0;
            obj = __try0_o1;
            vers = __try0_o2;
            wantedVersion = __try0_o3;
        }
        Err(_) => {
            return result.clone();
        }
    }
    result
}

pub fn versionsThatConvertToTheWanted(mut id: ArcStr, mut version: ArcStr, mut printError: bool) -> Arc<metamodelica::List<ArcStr>> {
    let mut result: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut libobject: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut vers: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut wantedVersion: SemanticVersion::Version;
    let mut libVersion: SemanticVersion::Version;
    result = metamodelica::nil();
    match '__try0: {
        obj = unwrap_break_err!(getPackageIndex(printError.clone()), '__try0);
        libobject = unwrap_break_err!(JSON::get(JSON::get(obj.clone(), (literal!("libs")).clone()).unwrap(), (id.clone()).clone()), '__try0);
        vers = unwrap_break_err!(JSON::get(libobject.clone(), (literal!("versions")).clone()), '__try0);
        wantedVersion = unwrap_break_err!(SemanticVersion::parse((version.clone()).clone(), true), '__try0);
        for mut v in &*unwrap_break_err!(JSON::getKeys(vers.clone()), '__try0) {
            let mut v = v.clone();
            libVersion = unwrap_break_err!(SemanticVersion::parse((v.clone()).clone(), true), '__try0);
            if unwrap_break_err!(SemanticVersion::compare(wantedVersion.clone(), libVersion.clone(), true, false), '__try0) == 0 {
                result = unwrap_break_err!(JSON::getStringList(JSON::get(JSON::get(vers.clone(), (v.clone()).clone()).unwrap(), (literal!("convertFromVersion")).clone()).unwrap()), '__try0);
                return result.clone();
            }
        }
        Ok::<_, anyhow::Error>((libobject.clone(), obj.clone(), vers.clone(), wantedVersion.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            libobject = __try0_o0;
            obj = __try0_o1;
            vers = __try0_o2;
            wantedVersion = __try0_o3;
        }
        Err(_) => {
            return result.clone();
        }
    }
    result
}

pub fn installPackage(mut pkg: ArcStr, mut version: ArcStr, mut exactMatch: bool, mut skipDownload: bool) -> Result<bool> {
    let mut success: bool = false;
    let mut packageList: Arc<metamodelica::List<PackageInstallInfo>> = metamodelica::nil();
    let mut packagesToInstall: Arc<metamodelica::List<PackageInstallInfo>> = metamodelica::nil();
    let mut urlPathList: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, ArcStr)>> = metamodelica::nil();
    let mut urlPathListToDownload: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, ArcStr)>> = metamodelica::nil();
    let mut path: ArcStr = arcstr::literal!("");
    let mut destPath: ArcStr = arcstr::literal!("");
    let mut destPathPkgMo: ArcStr = arcstr::literal!("");
    let mut destPathPkgInfo: ArcStr = arcstr::literal!("");
    let mut oldSha: ArcStr = arcstr::literal!("");
    let mut dirOfPath: ArcStr = arcstr::literal!("");
    let mut expectedLocation: ArcStr = arcstr::literal!("");
    let mut cachePath: ArcStr = getCachePath()?;
    let mut installCachePath: ArcStr = getInstallationCachePath()?;
    let mut curCachePath: ArcStr = arcstr::literal!("");
    let mut mirrors: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (success, packageList) = installPackageWork((pkg.clone()).clone(), (version.clone()).clone(), exactMatch.clone(), false, metamodelica::nil())?;
    for mut p in &*packageList.clone() {
        let mut p = p.clone();
        if p.pkg.clone() == pkg.clone() && !(p.needsInstall.clone()) {
            if version.clone() == SemanticVersion::toString(p.version.clone())? {
                Error::addSourceMessage(Error::NOTIFY_PKG_ALREADY_INSTALLED.clone(), list![(pkg.clone()).clone(), (SemanticVersion::toString(p.version.clone())?).clone()], makeSourceInfo((p.path.clone()).clone()))?;
            } else {
                Error::addSourceMessage(Error::NOTIFY_PKG_NO_INSTALL.clone(), list![(pkg.clone()).clone(), (version.clone()).clone(), (SemanticVersion::toString(p.version.clone())?).clone()], makeSourceInfo((p.path.clone()).clone()))?;
            }
        }
    }
    packagesToInstall = ({
        let mut __acc: Arc<metamodelica::List<PackageInstallInfo>> = metamodelica::nil();
        for mut p in (packageList.clone()).into_iter().cloned() {
            if !(p.needsInstall.clone()) { continue; }
            let __x = p.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut pack in &*packagesToInstall.clone() {
        let mut pack = pack.clone();
        Util::createDirectoryTree((cachePath.clone()).clone())?;
    }
    if !(skipDownload.clone()) {
        mirrors = getMirrors()?;
        urlPathList = List::sort(({
        let mut __acc: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, ArcStr)>> = metamodelica::nil();
        for mut p in (packagesToInstall.clone()).into_iter().cloned() {
            let __x = (getAllUrls((p.urlToZipFile.clone()).clone(), mirrors.clone())?, if (System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*installCachePath.clone()); __mm_s.push_str(&*System::basename((p.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*installCachePath.clone()); __mm_s.push_str(&*System::basename((p.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*cachePath.clone()); __mm_s.push_str(&*System::basename((p.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (std::sync::Arc::new(compareUrlBool) as std::sync::Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<ArcStr>>, ArcStr), (Arc<metamodelica::List<ArcStr>>, ArcStr)) -> Result<bool> + 'static>))?;
        urlPathList = List::unique(urlPathList.clone());
        urlPathListToDownload = ({
        let mut __acc: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, ArcStr)>> = metamodelica::nil();
        for mut tpl in (urlPathList.clone()).into_iter().cloned() {
            if !(!(System::regularFileExists((Util::tuple22(tpl.clone())).clone()))) { continue; }
            let __x = tpl.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if !(Curl::multiDownload(urlPathListToDownload.clone(), Config::noProc()?)) {
            bail!("fail");
        }
    }
    for mut pack in &*packagesToInstall.clone() {
        let mut pack = pack.clone();
        destPath = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*getUserLibraryPath()?); __mm_s.push_str(&*pack.pkg.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*SemanticVersion::toString(pack.version.clone())?); ArcStr::from(__mm_s) }).clone();
        System::removeDirectory((destPath.clone()).clone());
        System::createDirectory((destPath.clone()).clone());
        destPathPkgMo = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*destPath.clone()); __mm_s.push_str(&*literal!("/package.mo")); ArcStr::from(__mm_s) }).clone();
        destPathPkgInfo = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*destPath.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*arcstr::literal!(metaDataFileName)); ArcStr::from(__mm_s) }).clone();
        oldSha = (literal!("")).clone();
        if System::regularFileExists((destPathPkgInfo.clone()).clone()) {
            if '__try0: {
                oldSha = (unwrap_break_err!(getShaOrZipfile(JSON::parseFile((destPathPkgInfo.clone()).clone())?), '__try0)).clone();
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
        curCachePath = (if (System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*installCachePath.clone()); __mm_s.push_str(&*System::basename((pack.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }).clone())) {installCachePath.clone()} else {cachePath.clone()}).clone();
        if StringUtil::endsWith((pack.path.clone()).clone(), (literal!(".mo")).clone()) {
            dirOfPath = (System::dirname((pack.path.clone()).clone())).clone();
            if pack.singleFileStructureCopyAllFiles.clone() {
                Unzip::unzipPath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*curCachePath.clone()); __mm_s.push_str(&*System::basename((pack.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }).clone(), (if (dirOfPath.clone() == literal!(".")) {literal!("")} else {dirOfPath.clone()}).clone(), (destPath.clone()).clone());
                expectedLocation = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*destPath.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::basename((pack.path.clone()).clone())); ArcStr::from(__mm_s) }).clone();
                if !(System::rename((expectedLocation.clone()).clone(), (destPathPkgMo.clone()).clone())) {
                    Error::addMessage(Error::ERROR_PKG_INSTALL_NO_PACKAGE_MO.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*curCachePath.clone()); __mm_s.push_str(&*System::basename((pack.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }).clone(), (expectedLocation.clone()).clone()])?;
                    bail!("fail");
                }
            } else {
                Unzip::unzipPath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*curCachePath.clone()); __mm_s.push_str(&*System::basename((pack.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }).clone(), (pack.path.clone()).clone(), (destPathPkgMo.clone()).clone());
            }
        } else {
            Unzip::unzipPath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*curCachePath.clone()); __mm_s.push_str(&*System::basename((pack.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }).clone(), (pack.path.clone()).clone(), (destPath.clone()).clone());
        }
        if System::regularFileExists((destPathPkgMo.clone()).clone()) {
            if oldSha.clone() == literal!("") {
                Error::addSourceMessage(Error::NOTIFY_PKG_INSTALL_DONE.clone(), list![(pack.sha.clone()).clone()], makeSourceInfo((destPathPkgMo.clone()).clone()))?;
            } else {
                Error::addSourceMessage(Error::NOTIFY_PKG_UPGRADE_DONE.clone(), list![(pack.sha.clone()).clone(), (oldSha.clone()).clone()], makeSourceInfo((destPathPkgMo.clone()).clone()))?;
            }
        } else {
            Error::addMessage(Error::ERROR_PKG_INSTALL_NO_PACKAGE_MO.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*curCachePath.clone()); __mm_s.push_str(&*System::basename((pack.urlToZipFile.clone()).clone())); ArcStr::from(__mm_s) }).clone(), (destPathPkgMo.clone()).clone()])?;
            System::removeDirectory((destPath.clone()).clone());
            bail!("fail");
        }
        System::writeFile((destPathPkgInfo.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*JSON::toString(pack.json.clone(), false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(success)
}

pub fn installCachedPackages() -> Result<()> {
    let mut packageIndex: ArcStr = arcstr::literal!("");
    let mut homeDir: ArcStr = arcstr::literal!("");
    let mut obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut libs_obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut lib_obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut versions_obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    homeDir = (Settings::getHomeDir(Testsuite::isRunning()?)).clone();
    if !(System::subDirectories((getUserLibraryPath()?).clone()).is_empty()) || homeDir.clone() == literal!("") || homeDir.clone() == literal!("/") {
        return Ok(());
    }
    packageIndex = (getInstallationIndexPath()?).clone();
    if !(System::regularFileExists((packageIndex.clone()).clone())) {
        return Ok(());
    }
    obj = JSON::makeNull();
    if '__try0: {
        obj = unwrap_break_err!(JSON::parseFile((packageIndex.clone()).clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addSourceMessage(Error::ERROR_PKG_INDEX_NOT_PARSED.clone(), list![(packageIndex.clone()).clone()], makeSourceInfo((packageIndex.clone()).clone()))?;
    }
    match '__try1: {
        libs_obj = unwrap_break_err!(JSON::get(obj.clone(), (literal!("libs")).clone()), '__try1);
        libs = unwrap_break_err!(JSON::getKeys(libs_obj.clone()), '__try1);
        Ok::<_, anyhow::Error>((libs.clone(), libs_obj.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            libs = __try1_o0;
            libs_obj = __try1_o1;
        }
        Err(_) => {
            return Ok(());
        }
    }
    if !(libs.clone().is_empty()) {
        Error::addSourceMessage(Error::NOTIFY_INITIALIZING_USER_LIBRARIES.clone(), list![(getUserLibraryPath()?).clone()], makeSourceInfo((packageIndex.clone()).clone()))?;
    }
    if !(System::regularFileExists((getIndexPath()?).clone())) {
        Util::createDirectoryTree((getUserLibraryPath()?).clone())?;
        System::copyFile((packageIndex.clone()).clone(), (getIndexPath()?).clone());
    }
    for mut lib in &*libs.clone() {
        let mut lib = lib.clone();
        lib_obj = JSON::get(libs_obj.clone(), (lib.clone()).clone())?;
        versions_obj = JSON::getOrDefault(lib_obj.clone(), (literal!("versions")).clone(), JSON::emptyObject());
        for mut version in &*JSON::getKeys(versions_obj.clone())? {
            let mut version = version.clone();
            installPackage((lib.clone()).clone(), (version.clone()).clone(), true, true)?;
        }
    }
    updateIndex()?;
    Ok(())
}

fn compareUrlBool(mut tpl1: (Arc<metamodelica::List<ArcStr>>, ArcStr), mut tpl2: (Arc<metamodelica::List<ArcStr>>, ArcStr)) -> Result<bool> {
    let mut b: bool = false;
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(tpl1.clone()) {
        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    s1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(tpl2.clone()) {
        (Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, _) => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    s2 = __pa1.clone();
    b = stringCompare((s1.clone()).clone(), (s2.clone()).clone()) > 0;
    Ok(b)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackageInstallInfo {
    pub needsInstall: bool,
    pub pkg: ArcStr,
    pub version: SemanticVersion::Version,
    pub urlToZipFile: ArcStr,
    pub path: ArcStr,
    pub sha: ArcStr,
    pub singleFileStructureCopyAllFiles: bool,
    pub json: Arc<JSON::JSON>,
}

impl Default for PackageInstallInfo {
    fn default() -> Self {
        Self {
            needsInstall: Default::default(),
            pkg: Default::default(),
            version: Default::default(),
            urlToZipFile: Default::default(),
            path: Default::default(),
            sha: Default::default(),
            singleFileStructureCopyAllFiles: Default::default(),
            json: Default::default(),
        }
    }
}

pub type PKG_INSTALL_INFO = PackageInstallInfo;


fn installPackageWork(mut pkg: ArcStr, mut version: ArcStr, mut exactMatch: bool, mut fallbackOnNonExactMatch: bool, mut packagesToInstall: Arc<metamodelica::List<PackageInstallInfo>>) -> Result<(bool, Arc<metamodelica::List<PackageInstallInfo>>)> {
    let mut success: bool = false;
    let mut packagesToInstall: Arc<metamodelica::List<PackageInstallInfo>> = packagesToInstall;
    let mut installedLibraries: Arc<AvailableLibraries::Tree> = Arc::new(AvailableLibraries::Tree::EMPTY);
    let mut installedVersions: Arc<VersionMap::Tree> = Arc::new(VersionMap::Tree::EMPTY);
    let mut candidates: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut candidatesSemver: Arc<metamodelica::List<SemanticVersion::Version>> = metamodelica::nil();
    let mut exactMatches: Arc<metamodelica::List<SemanticVersion::Version>> = metamodelica::nil();
    let mut versionToInstall: ArcStr = arcstr::literal!("");
    let mut usedVersion: ArcStr = arcstr::literal!("");
    let mut path: ArcStr = arcstr::literal!("");
    let mut sha: ArcStr = arcstr::literal!("");
    let mut jsonPath: ArcStr = arcstr::literal!("");
    let mut zip: ArcStr = arcstr::literal!("");
    let mut semverToInstall: SemanticVersion::Version;
    let mut semver: SemanticVersion::Version;
    let mut index: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut versionObj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut versionsObj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut usesObj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut indexHasPkg: bool = false;
    let mut packageToInstall: PackageInstallInfo = <PackageInstallInfo as ::std::default::Default>::default();
    candidates = versionsThatProvideTheWanted((pkg.clone()).clone(), (version.clone()).clone(), true);
    candidatesSemver = ({
        let mut __acc: Arc<metamodelica::List<SemanticVersion::Version>> = metamodelica::nil();
        for mut candidate in (candidates.clone()).into_iter().cloned() {
            let __x = SemanticVersion::parse((candidate.clone()).clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    semver = SemanticVersion::parse((version.clone()).clone(), false)?;
    exactMatches = ({
        let mut __acc: Arc<metamodelica::List<SemanticVersion::Version>> = metamodelica::nil();
        for mut candidate in (candidatesSemver.clone()).into_iter().cloned() {
            if !(0 == SemanticVersion::compare(candidate.clone(), semver.clone(), true, SemanticVersion::hasMetaInformation(semver.clone()))?) { continue; }
            let __x = candidate.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    success = false;
    for mut pkgInfo in &*packagesToInstall.clone() {
        let mut pkgInfo = pkgInfo.clone();
        if pkgInfo.pkg.clone() == pkg.clone() {
            if SemanticVersion::compare(pkgInfo.version.clone(), semver.clone(), true, false)? == 0 || ({
        let mut __acc: Option<bool> = None;
        for mut candidate in (candidatesSemver.clone()).into_iter().cloned() {
            let __x = 0 == SemanticVersion::compare(pkgInfo.version.clone(), candidate.clone(), true, false)?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    }) {
                success = true;
                return Ok((success.clone(), packagesToInstall.clone()));
            }
            Error::addMessage(Error::WARNING_PKG_CONFLICTING_VERSIONS.clone(), list![(pkg.clone()).clone(), (SemanticVersion::toString(pkgInfo.version.clone())?).clone(), (version.clone()).clone()])?;
            success = true;
            return Ok((success.clone(), packagesToInstall.clone()));
        }
    }
    installedLibraries = getInstalledLibraries()?;
    if candidates.clone().is_empty() {
        versionToInstall = (version.clone()).clone();
        semverToInstall = semver.clone();
    } else if exactMatch.clone() && !(exactMatches.clone().is_empty()) {
        semverToInstall = listHead(exactMatches.clone())?;
        versionToInstall = (SemanticVersion::toString(semverToInstall.clone())?).clone();
    } else {
        versionToInstall = (listHead(candidates.clone())?).clone();
        semverToInstall = listHead(candidatesSemver.clone())?;
    }
    index = getPackageIndex(true)?;
    indexHasPkg = true;
    sha = (literal!("")).clone();
    if AvailableLibraries::hasKey(installedLibraries.clone(), (pkg.clone()).clone())? {
        installedVersions = AvailableLibraries::get(installedLibraries.clone(), (pkg.clone()).clone())?;
        if VersionMap::hasKey(installedVersions.clone(), semverToInstall.clone())? || version.clone() == literal!("") && !(indexHasPkg.clone()) {
            success = true;
            path = (if (VersionMap::hasKey(installedVersions.clone(), semverToInstall.clone())?) {VersionMap::get(installedVersions.clone(), semverToInstall.clone())?} else {literal!("#DUMMY#")}).clone();
            jsonPath = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*arcstr::literal!(metaDataFileName)); ArcStr::from(__mm_s) }).clone();
            if System::regularFileExists((jsonPath.clone()).clone()) {
                versionObj = JSON::parseFile((jsonPath.clone()).clone())?;
                zip = (JSON::getString(JSON::get(versionObj.clone(), (literal!("zipfile")).clone())?)?).clone();
                if '__try0: {
                    sha = (unwrap_break_err!(JSON::getString(JSON::get(versionObj.clone(), (literal!("sha")).clone())?), '__try0)).clone();
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
            } else {
                zip = (literal!("")).clone();
            }
            packageToInstall = PackageInstallInfo { needsInstall: false, pkg: (pkg.clone()).clone(), version: semverToInstall.clone(), urlToZipFile: (zip.clone()).clone(), path: (path.clone()).clone(), sha: (sha.clone()).clone(), singleFileStructureCopyAllFiles: false, json: JSON::emptyObject() };
            indexHasPkg = JSON::hasKey(JSON::get(index.clone(), (literal!("libs")).clone())?, (pkg.clone()).clone())?;
        }
    }
    if !(success.clone()) {
        if candidates.clone().is_empty() {
            Error::addSourceMessage(Error::ERROR_PKG_NOT_FOUND_VERSION.clone(), list![(pkg.clone()).clone(), (version.clone()).clone(), stringDelimitList(getAllProvidedVersionsForLibrary((pkg.clone()).clone(), true), (literal!("\n")).clone())], makeSourceInfo((getIndexPath()?).clone()))?;
            return Ok((success.clone(), packagesToInstall.clone()));
        }
        if exactMatch.clone() && !(({
        let mut __acc: Option<bool> = None;
        for mut candidate in (candidatesSemver.clone()).into_iter().cloned() {
            let __x = 0 == SemanticVersion::compare(semver.clone(), candidate.clone(), true, false)?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    })) {
            if !(fallbackOnNonExactMatch.clone()) {
                Error::addSourceMessage(Error::ERROR_PKG_NOT_EXACT_MATCH.clone(), list![(pkg.clone()).clone(), (version.clone()).clone(), stringDelimitList(candidates.clone(), (literal!(", ")).clone())], makeSourceInfo((getIndexPath()?).clone()))?;
                return Ok((success.clone(), packagesToInstall.clone()));
            }
            versionToInstall = (listHead(candidates.clone())?).clone();
            semverToInstall = listHead(candidatesSemver.clone())?;
        }
    }
    if !(indexHasPkg.clone()) {
        packagesToInstall = cons(packageToInstall.clone(), packagesToInstall.clone());
        return Ok((success.clone(), packagesToInstall.clone()));
    }
    versionsObj = JSON::get(JSON::get(JSON::get(index.clone(), (literal!("libs")).clone())?, (pkg.clone()).clone())?, (literal!("versions")).clone())?;
    if success.clone() && !(JSON::hasKey(versionsObj.clone(), (versionToInstall.clone()).clone())?) {
        packagesToInstall = cons(packageToInstall.clone(), packagesToInstall.clone());
        return Ok((success.clone(), packagesToInstall.clone()));
    }
    versionObj = JSON::get(versionsObj.clone(), (versionToInstall.clone()).clone())?;
    if !(success.clone()) || sha.clone() != literal!("") && sha.clone() != getShaOrZipfile(versionObj.clone())? {
        success = true;
        packageToInstall = PackageInstallInfo { needsInstall: true, pkg: (pkg.clone()).clone(), version: semverToInstall.clone(), urlToZipFile: (JSON::getString(JSON::get(versionObj.clone(), (literal!("zipfile")).clone())?)?).clone(), path: (JSON::getString(JSON::get(versionObj.clone(), (literal!("path")).clone())?)?).clone(), sha: (getShaOrZipfile(versionObj.clone())?).clone(), singleFileStructureCopyAllFiles: JSON::getBoolean(JSON::getOrDefault(versionObj.clone(), (literal!("singleFileStructureCopyAllFiles")).clone(), Arc::new(openmodelica_util::JSON::FALSE)))?, json: versionObj.clone() };
    }
    usesObj = JSON::getOrDefault(versionObj.clone(), (literal!("uses")).clone(), JSON::emptyObject());
    packagesToInstall = cons(packageToInstall.clone(), packagesToInstall.clone());
    for mut usesPackage in &*JSON::getKeys(usesObj.clone())? {
        let mut usesPackage = usesPackage.clone();
        let __pa1 = ::match_deref::match_deref! { match &(JSON::get(usesObj.clone(), (usesPackage.clone()).clone())?) {
            Deref @ JSON::STRING { r#str: __pa1 } => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        usedVersion = __pa1.clone();
        (success, packagesToInstall) = installPackageWork((usesPackage.clone()).clone(), (usedVersion.clone()).clone(), exactMatch.clone(), true, packagesToInstall.clone())?;
        if !(success.clone()) {
            return Ok((success.clone(), packagesToInstall.clone()));
        }
    }
    Ok((success, packagesToInstall))
}

fn getShaOrZipfile(mut obj: Arc<JSON::JSON>) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = (if (JSON::hasKey(obj.clone(), (literal!("sha")).clone())?) {JSON::getString(JSON::get(obj.clone(), (literal!("sha")).clone())?)?} else {System::basename((JSON::getString(JSON::get(obj.clone(), (literal!("zipfile")).clone())?)?).clone())}).clone();
    Ok(res)
}

fn getAllUrls(mut url: ArcStr, mut mirrors: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut urls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut urlWithoutProtocol: ArcStr = arcstr::literal!("");
    let mut newUrl: ArcStr = arcstr::literal!("");
    urls = list![(url.clone()).clone()];
    if !(StringUtil::startsWith((url.clone()).clone(), (literal!("https://")).clone())) {
        return Ok(urls.clone());
    }
    urlWithoutProtocol = substring((url.clone()).clone(), 9, ((url.clone()).clone().len() as i32))?;
    for mut mirror in &*mirrors.clone() {
        let mut mirror = mirror.clone();
        newUrl = (if (StringUtil::endsWith((mirror.clone()).clone(), (literal!("/")).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*mirror.clone()); __mm_s.push_str(&*urlWithoutProtocol.clone()); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*mirror.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*urlWithoutProtocol.clone()); ArcStr::from(__mm_s) }}).clone();
        urls = cons((newUrl.clone()).clone(), urls.clone());
    }
    Ok(urls)
}

fn getMirrors() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut mirrors: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    obj = getPackageIndex(false)?;
    if !(JSON::hasKey(obj.clone(), (literal!("mirrors")).clone())?) {
        mirrors = metamodelica::nil();
        return Ok(mirrors.clone());
    }
    obj = JSON::get(obj.clone(), (literal!("mirrors")).clone())?;
    mirrors = JSON::getStringList(obj.clone())?;
    Ok(mirrors)
}

fn getUserLibraryPath() -> Result<ArcStr> {
    let mut path: ArcStr = arcstr::literal!("");
    path = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getHomeDir(Testsuite::isRunning()?)); __mm_s.push_str(&*literal!("/.openmodelica/libraries/")); ArcStr::from(__mm_s) }).clone();
    Ok(path)
}

fn getIndexPath() -> Result<ArcStr> {
    let mut path: ArcStr = arcstr::literal!("");
    path = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getHomeDir(Testsuite::isRunning()?)); __mm_s.push_str(&*literal!("/.openmodelica/libraries/index.json")); ArcStr::from(__mm_s) }).clone();
    Ok(path)
}

pub fn getCachePath() -> Result<ArcStr> {
    let mut path: ArcStr = arcstr::literal!("");
    path = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getHomeDir(Testsuite::isRunning()?)); __mm_s.push_str(&*literal!("/.openmodelica/cache/")); ArcStr::from(__mm_s) }).clone();
    Ok(path)
}

fn getInstallationIndexPath() -> Result<ArcStr> {
    let mut path: ArcStr = arcstr::literal!("");
    path = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/share/omlibrary/cache/index.json")); ArcStr::from(__mm_s) }).clone();
    Ok(path)
}

fn getInstallationCachePath() -> Result<ArcStr> {
    let mut path: ArcStr = arcstr::literal!("");
    path = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/share/omlibrary/cache/")); ArcStr::from(__mm_s) }).clone();
    Ok(path)
}

fn makeSourceInfo(mut fileName: ArcStr) -> SourceInfo {
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = SourceInfo { fileName: (fileName.clone()).clone(), isReadOnly: true, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };
    info
}

