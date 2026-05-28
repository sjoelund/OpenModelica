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

use crate::BaseModelica;
use crate::NFBinding as Binding;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFSubscript as Subscript;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util_datatypes_basic::List;

thread_local! { static __EMPTY_MOD_TLS: Arc<Modifier::Modifier> = Arc::new(crate::NFModifier::Modifier::NOMOD); }
pub fn EMPTY_MOD() -> Arc<Modifier::Modifier> { __EMPTY_MOD_TLS.with(|__t| __t.clone()) }

pub mod ModTable {
    use super::*;
    pub fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (inKey.clone()).clone();
        outString
    }

    pub fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (Modifier::toString(inValue.clone(), true)?).clone();
        Ok(outString)
    }

    pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
        let mut outResult: i32 = 0;
        outResult = stringCompare((inKey1.clone()).clone(), (inKey2.clone()).clone());
        outResult
    }

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    pub type Key = ArcStr;

    /// The binary tree data structure.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub type Value = Arc<Modifier::Modifier>;

    pub type ValueNode = ArcStr;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Modifier::Modifier>, Arc<Modifier::Modifier>, ArcStr) -> Result<Arc<Modifier::Modifier>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value = Arc::new(Modifier::NOMOD);
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
            let mut value: Value = Arc::new(Modifier::NOMOD);
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare((inKey.clone()).clone(), (var_field!((*tree).key, Tree::LEAF).clone()).clone());
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }), right: Arc::new(crate::NFModifier::ModTable::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::NFModifier::ModTable::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }) });
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
        let mut value: Value = Arc::new(Modifier::NOMOD);
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

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(ArcStr, Arc<Modifier::Modifier>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Modifier::Modifier>, Arc<Modifier::Modifier>, ArcStr) -> Result<Arc<Modifier::Modifier>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key = arcstr::literal!("");
        let mut value: Value = Arc::new(Modifier::NOMOD);
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Arc<Modifier::Modifier>>) -> Result<Arc<Modifier::Modifier>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Modifier::Modifier>>) -> Result<Value> + 'static>;

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
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }), right: Arc::new(crate::NFModifier::ModTable::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::NFModifier::ModTable::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }) });
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Modifier::Modifier>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
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

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Modifier::Modifier>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
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

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Modifier::Modifier>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
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

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Modifier::Modifier>) -> Result<()> + 'static>) -> Result<()> {
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

    pub fn fromList(mut inValues: Arc<metamodelica::List<(ArcStr, Arc<Modifier::Modifier>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Modifier::Modifier>, Arc<Modifier::Modifier>, ArcStr) -> Result<Arc<Modifier::Modifier>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::NFModifier::ModTable::Tree::EMPTY);
        let mut key: Key = arcstr::literal!("");
        let mut value: Value = Arc::new(Modifier::NOMOD);
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value = Arc::new(Modifier::NOMOD);
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
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Option<Arc<Modifier::Modifier>> {
        let mut value: Option<Arc<Modifier::Modifier>> = None;
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Modifier::Modifier>, Arc<Modifier::Modifier>, ArcStr) -> Result<Arc<Modifier::Modifier>> + 'static>) -> Result<Arc<Tree>> {
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

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Modifier::Modifier>>>) -> Arc<metamodelica::List<Arc<Modifier::Modifier>>> {
        let mut lst: Arc<metamodelica::List<Arc<Modifier::Modifier>>> = lst;
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

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Modifier::Modifier>) -> Result<Arc<Modifier::Modifier>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = Arc::new(Modifier::NOMOD);
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
            let mut new_value: Value = Arc::new(Modifier::NOMOD);
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

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Modifier::Modifier>, FT) -> Result<(Arc<Modifier::Modifier>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = Arc::new(Modifier::NOMOD);
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
            let mut new_value: Value = Arc::new(Modifier::NOMOD);
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
        let mut outTree: Arc<Tree> = Arc::new(crate::NFModifier::ModTable::Tree::EMPTY);
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::NFModifier::ModTable::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::NFModifier::ModTable::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::NFModifier::ModTable::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::NFModifier::ModTable::Tree::EMPTY), node.clone())?
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

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(ArcStr, Arc<Modifier::Modifier>)>>) -> Arc<metamodelica::List<(ArcStr, Arc<Modifier::Modifier>)>> {
        let mut lst: Arc<metamodelica::List<(ArcStr, Arc<Modifier::Modifier>)>> = lst;
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
        let mut outTree: Arc<Tree> = add(tree.clone(), (key.clone()).clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Arc<Modifier::Modifier>, Arc<Modifier::Modifier>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Modifier::Modifier>, Arc<Modifier::Modifier>, ArcStr) -> Result<Arc<Modifier::Modifier>> + 'static>)).unwrap();
        outTree
    }

}

pub mod ModifierScope {
    use super::*;
    /// Structure that represents where a modifier comes from.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ModifierScope {
        COMPONENT {
            name: ArcStr,
        },
        CLASS {
            name: ArcStr,
        },
        EXTENDS {
            path: Arc<Absyn::Path>,
        },
    }
    pub use self::ModifierScope::{COMPONENT,CLASS,EXTENDS};
    pub fn fromElement(mut element: Arc<SCode::Element>) -> Result<Arc<ModifierScope>> {
        let mut scope: Arc<ModifierScope>;
        scope = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => Arc::new(ModifierScope::COMPONENT { name: (var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone() }),
        Deref @ SCode::Element::CLASS { .. } => Arc::new(ModifierScope::CLASS { name: (var_field!((*element).name, SCode::Element::CLASS).clone()).clone() }),
        Deref @ SCode::Element::EXTENDS { .. } => Arc::new(ModifierScope::EXTENDS { path: var_field!((*element).baseClassPath, SCode::Element::EXTENDS).clone() }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(scope)
    }

    pub fn name(mut scope: Arc<ModifierScope>) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        name = ((::match_deref::match_deref! { match &(scope.clone()) {
        Deref @ COMPONENT { .. } => var_field!((*scope).name, ModifierScope::COMPONENT).clone(),
        Deref @ CLASS { .. } => var_field!((*scope).name, ModifierScope::CLASS).clone(),
        Deref @ EXTENDS { .. } => AbsynUtil::pathString(var_field!((*scope).path, ModifierScope::EXTENDS).clone(), (literal!(".")).clone(), true, false)?,
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(name)
    }

    pub fn isClass(mut scope: Arc<ModifierScope>) -> bool {
        let mut res: bool = false;
        res = (::match_deref::match_deref! { match &(scope.clone()) {
        Deref @ CLASS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    pub fn toString(mut scope: Arc<ModifierScope>) -> Result<ArcStr> {
        let mut string: ArcStr = arcstr::literal!("");
        string = ((::match_deref::match_deref! { match &(scope.clone()) {
        Deref @ COMPONENT { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*var_field!((*scope).name, ModifierScope::COMPONENT).clone()); ArcStr::from(__mm_s) },
        Deref @ CLASS { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("class ")); __mm_s.push_str(&*var_field!((*scope).name, ModifierScope::CLASS).clone()); ArcStr::from(__mm_s) },
        Deref @ EXTENDS { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("extends ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*scope).path, ModifierScope::EXTENDS).clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(string)
    }

}

pub mod Modifier {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Modifier {
        MODIFIER {
            name: ArcStr,
            finalPrefix: SCode::Final,
            eachPrefix: SCode::Each,
            binding: Arc<Binding::NFBinding>,
            subModifiers: Arc<ModTable::Tree>,
            info: SourceInfo,
        },
        REDECLARE {
            finalPrefix: SCode::Final,
            eachPrefix: SCode::Each,
            element: Arc<InstNode::InstNode>,
            innerMod: Arc<Modifier>,
            outerMod: Arc<Modifier>,
            constrainingMod: Arc<Modifier>,
            propagatedSubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>,
        },
        NOMOD,
    }
    pub use self::Modifier::{MODIFIER,REDECLARE,NOMOD};
    pub fn create(mut r#mod: Arc<SCode::Mod>, mut name: ArcStr, mut modScope: Arc<ModifierScope::ModifierScope>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<Modifier>> {
        let mut newMod: Arc<Modifier> = Arc::new(Modifier::NOMOD);
        newMod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::NOMOD => {
            Arc::new(crate::NFModifier::Modifier::NOMOD)
        },
        Deref @ SCode::Mod::MOD { .. } => {
            let mut submod_lst: Arc<metamodelica::List<(ArcStr, Arc<Modifier>)>> = metamodelica::nil();
            let mut submod_table: Arc<ModTable::Tree> = Arc::new(ModTable::Tree::EMPTY);
            let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
            let mut is_each: bool = false;
            is_each = SCodeUtil::eachBool(var_field!((*r#mod).eachPrefix, SCode::Mod::MOD).clone())?;
            binding = Binding::fromAbsyn(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), is_each.clone(), ModifierScope::isClass(modScope.clone()), scope.clone(), var_field!((*r#mod).info, SCode::Mod::MOD).clone());
            submod_lst = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Modifier>)>> = metamodelica::nil();
        for mut m in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            if !(!(SCodeUtil::isBreakSubMod(m.clone()))) { continue; }
            let __x = (m.ident.clone(), createSubMod(m.clone(), modScope.clone(), scope.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            submod_table = ModTable::fromList(submod_lst.clone(), Arc::new({ let __pe_b3 = modScope.clone(); let __pe_b4 = metamodelica::nil(); move |__pe_a0, __pe_a1, __pe_a2| mergeLocal(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone()) }))?;
            Arc::new(Modifier::MODIFIER { name: (name.clone()).clone(), finalPrefix: var_field!((*r#mod).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*r#mod).eachPrefix, SCode::Mod::MOD).clone(), binding: binding.clone(), subModifiers: submod_table.clone(), info: var_field!((*r#mod).info, SCode::Mod::MOD).clone() })
        },
        Deref @ SCode::Mod::REDECL { element: elem, .. } => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut cc_mod: Arc<Modifier> = Arc::new(Modifier::NOMOD);
            node = InstNode::new(elem.clone(), scope.clone())?;
            if InstNode::isClass(node.clone()) {
                Inst::partialInstClass(node.clone())?;
            }
            cc_mod = createConstrainingMod(elem.clone(), scope.clone())?;
            Arc::new(Modifier::REDECLARE { finalPrefix: var_field!((*r#mod).finalPrefix, SCode::Mod::REDECL).clone(), eachPrefix: var_field!((*r#mod).eachPrefix, SCode::Mod::REDECL).clone(), element: node.clone(), innerMod: Arc::new(crate::NFModifier::Modifier::NOMOD), outerMod: Arc::new(crate::NFModifier::Modifier::NOMOD), constrainingMod: cc_mod.clone(), propagatedSubs: metamodelica::nil() })
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(newMod)
    }

    pub fn createConstrainingMod(mut element: Arc<SCode::Element>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<Modifier>> {
        let mut r#mod: Arc<Modifier> = Arc::new(Modifier::NOMOD);
        let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        r#mod = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: smod, .. }) }, .. }, .. } => create(smod.clone(), (var_field!((*element).name, SCode::Element::CLASS).clone()).clone(), Arc::new(ModifierScope::ModifierScope::CLASS { name: (var_field!((*element).name, SCode::Element::CLASS).clone()).clone() }), scope.clone())?,
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: smod, .. }) }, .. }, .. } => create(smod.clone(), (var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone(), Arc::new(ModifierScope::ModifierScope::COMPONENT { name: (var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone() }), scope.clone())?,
        _ => Arc::new(crate::NFModifier::Modifier::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(r#mod)
    }

    pub fn stripSCodeMod(mut elem: Arc<SCode::Element>) -> (Arc<SCode::Element>, Arc<SCode::Mod>) {
        let mut elem: Arc<SCode::Element> = elem;
        let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        r#mod = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::CLASS { classDef: cdef @ Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, .. }, .. } => {
            let mut cdef = (*cdef).clone();
            if !(SCodeUtil::isEmptyMod(r#mod.clone())) {
                assign_variant_field!(cdef => SCode::ClassDef::DERIVED; modifications = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD));
                assign_variant_field!(elem => SCode::Element::CLASS; classDef = cdef.clone());
            }
            r#mod.clone()
        },
        Deref @ SCode::Element::COMPONENT { modifications: r#mod, .. } => {
            if !(SCodeUtil::isEmptyMod(r#mod.clone())) {
                assign_variant_field!(elem => SCode::Element::COMPONENT; modifications = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD));
            }
            r#mod.clone()
        },
        _ => {
            Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (elem, r#mod)
    }

    pub fn fromElement(mut element: Arc<SCode::Element>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<Modifier>> {
        let mut r#mod: Arc<Modifier> = Arc::new(Modifier::NOMOD);
        r#mod = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => {
            create(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), (literal!("")).clone(), Arc::new(ModifierScope::ModifierScope::EXTENDS { path: var_field!((*element).baseClassPath, SCode::Element::EXTENDS).clone() }), scope.clone())?
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            smod = patchElementModFinal(var_field!((*element).prefixes, SCode::Element::COMPONENT).clone(), var_field!((*element).info, SCode::Element::COMPONENT).clone(), var_field!((*element).modifications, SCode::Element::COMPONENT).clone())?;
            create(smod.clone(), (var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone(), Arc::new(ModifierScope::ModifierScope::COMPONENT { name: (var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone() }), scope.clone())?
        },
        Deref @ SCode::Element::CLASS { classDef: def @ Deref @ SCode::ClassDef::DERIVED { .. }, .. } => {
            create(var_field!((**def).modifications, SCode::ClassDef::DERIVED).clone(), (var_field!((*element).name, SCode::Element::CLASS).clone()).clone(), Arc::new(ModifierScope::ModifierScope::CLASS { name: (var_field!((*element).name, SCode::Element::CLASS).clone()).clone() }), scope.clone())?
        },
        Deref @ SCode::Element::CLASS { classDef: def @ Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            create(var_field!((**def).modifications, SCode::ClassDef::CLASS_EXTENDS).clone(), (var_field!((*element).name, SCode::Element::CLASS).clone()).clone(), Arc::new(ModifierScope::ModifierScope::CLASS { name: (var_field!((*element).name, SCode::Element::CLASS).clone()).clone() }), scope.clone())?
        },
        _ => {
            Arc::new(crate::NFModifier::Modifier::NOMOD)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(r#mod)
    }

    pub fn patchElementModFinal(mut prefixes: Arc<SCode::Prefixes>, mut info: SourceInfo, mut r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
        let mut r#mod: Arc<SCode::Mod> = r#mod;
        if SCodeUtil::finalBool(SCodeUtil::prefixesFinal(prefixes.clone())?)? {
            r#mod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; finalPrefix = openmodelica_frontend_types::SCode::Final::FINAL);
            r#mod.clone()
        },
        Deref @ SCode::Mod::REDECL { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::REDECL; finalPrefix = openmodelica_frontend_types::SCode::Final::FINAL);
            r#mod.clone()
        },
        _ => Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: None, comment: None, info: info.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(r#mod)
    }

    pub fn lookupModifier(mut modName: ArcStr, mut modifier: Arc<Modifier>) -> Result<Arc<Modifier>> {
        let mut subMod: Arc<Modifier> = Arc::new(Modifier::NOMOD);
        subMod = 'mc: {
        let __mc_input = modifier.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ MODIFIER { .. } => {
                    Ok(ModTable::get(var_field!((*modifier).subModifiers, Modifier::MODIFIER).clone(), (modName.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(EMPTY_MOD().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        Ok(subMod)
    }

    pub fn name(mut modifier: Arc<Modifier>) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        name = ((::match_deref::match_deref! { match &(modifier.clone()) {
        Deref @ MODIFIER { .. } => var_field!((*modifier).name, Modifier::MODIFIER).clone(),
        Deref @ REDECLARE { .. } => InstNode::name(var_field!((*modifier).element, Modifier::REDECLARE).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(name)
    }

    pub fn info(mut modifier: Arc<Modifier>) -> Result<SourceInfo> {
        let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
        info = (::match_deref::match_deref! { match &(modifier.clone()) {
        Deref @ MODIFIER { .. } => var_field!((*modifier).info, Modifier::MODIFIER).clone(),
        Deref @ REDECLARE { .. } => InstNode::info(var_field!((*modifier).element, Modifier::REDECLARE).clone())?,
        _ => Absyn::dummyInfo.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(info)
    }

    pub fn hasBinding(mut modifier: Arc<Modifier>) -> bool {
        let mut hasBinding: bool = false;
        hasBinding = (::match_deref::match_deref! { match &(modifier.clone()) {
        Deref @ MODIFIER { .. } => Binding::isBound(var_field!((*modifier).binding, Modifier::MODIFIER).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        hasBinding
    }

    pub fn binding(mut modifier: Arc<Modifier>) -> Arc<Binding::NFBinding> {
        let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        binding = (::match_deref::match_deref! { match &(modifier.clone()) {
        Deref @ MODIFIER { .. } => var_field!((*modifier).binding, Modifier::MODIFIER).clone(),
        _ => Binding::EMPTY_BINDING().clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        binding
    }

    pub fn setBinding(mut binding: Arc<Binding::NFBinding>, mut modifier: Arc<Modifier>) -> Result<Arc<Modifier>> {
        let mut modifier: Arc<Modifier> = modifier;
        let () = (::match_deref::match_deref! { match &(modifier.clone()) {
        Deref @ MODIFIER { .. } => {
            assign_variant_field!(modifier => Modifier::MODIFIER; binding = binding.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(modifier)
    }

    pub fn merge(mut outerMod: Arc<Modifier>, mut innerMod: Arc<Modifier>, mut name: ArcStr) -> Result<Arc<Modifier>> {
        let mut mergedMod: Arc<Modifier> = Arc::new(Modifier::NOMOD);
        mergedMod = (::match_deref::match_deref! { match &((outerMod.clone(), innerMod.clone())) {
        (Deref @ NOMOD { .. }, _) => {
            innerMod.clone()
        },
        (_, Deref @ NOMOD { .. }) => {
            outerMod.clone()
        },
        (Deref @ MODIFIER { .. }, Deref @ MODIFIER { .. }) => {
            let mut submods: Arc<ModTable::Tree> = Arc::new(ModTable::Tree::EMPTY);
            let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
            checkFinalOverride(var_field!((*innerMod).finalPrefix, Modifier::MODIFIER).clone(), outerMod.clone(), var_field!((*innerMod).info, Modifier::MODIFIER).clone())?;
            binding = if (Binding::isBound(var_field!((*outerMod).binding, Modifier::MODIFIER).clone())) {var_field!((*outerMod).binding, Modifier::MODIFIER).clone()} else {var_field!((*innerMod).binding, Modifier::MODIFIER).clone()};
            submods = ModTable::join(var_field!((*innerMod).subModifiers, Modifier::MODIFIER).clone(), var_field!((*outerMod).subModifiers, Modifier::MODIFIER).clone(), (std::sync::Arc::new(merge) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Modifier>, Arc<Modifier>, ArcStr) -> Result<Arc<Modifier>> + 'static>))?;
            Arc::new(Modifier::MODIFIER { name: (var_field!((*outerMod).name, Modifier::MODIFIER).clone()).clone(), finalPrefix: var_field!((*outerMod).finalPrefix, Modifier::MODIFIER).clone(), eachPrefix: var_field!((*outerMod).eachPrefix, Modifier::MODIFIER).clone(), binding: binding.clone(), subModifiers: submods.clone(), info: var_field!((*outerMod).info, Modifier::MODIFIER).clone() })
        },
        (Deref @ REDECLARE { .. }, Deref @ MODIFIER { .. }) => {
            assign_variant_field!(outerMod => Modifier::REDECLARE; innerMod = merge(var_field!((*outerMod).innerMod, Modifier::REDECLARE).clone(), innerMod.clone(), (literal!("")).clone())?);
            outerMod.clone()
        },
        (Deref @ MODIFIER { .. }, Deref @ REDECLARE { .. }) => {
            assign_variant_field!(innerMod => Modifier::REDECLARE; outerMod = merge(outerMod.clone(), var_field!((*innerMod).outerMod, Modifier::REDECLARE).clone(), (literal!("")).clone())?);
            innerMod.clone()
        },
        (Deref @ REDECLARE { constrainingMod: Deref @ NOMOD { .. }, .. }, Deref @ REDECLARE { constrainingMod: Deref @ MODIFIER { .. }, .. }) => {
            Arc::new(Modifier::REDECLARE { finalPrefix: var_field!((*outerMod).finalPrefix, Modifier::REDECLARE).clone(), eachPrefix: var_field!((*outerMod).eachPrefix, Modifier::REDECLARE).clone(), element: var_field!((*outerMod).element, Modifier::REDECLARE).clone(), innerMod: var_field!((*outerMod).innerMod, Modifier::REDECLARE).clone(), outerMod: var_field!((*outerMod).outerMod, Modifier::REDECLARE).clone(), constrainingMod: var_field!((*innerMod).constrainingMod, Modifier::REDECLARE).clone(), propagatedSubs: var_field!((*outerMod).propagatedSubs, Modifier::REDECLARE).clone() })
        },
        (Deref @ REDECLARE { .. }, _) => {
            outerMod.clone()
        },
        (_, Deref @ REDECLARE { .. }) => {
            innerMod.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Mod.mergeMod failed on unknown mod.")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(mergedMod)
    }

    pub fn propagate(mut r#mod: Arc<Modifier>, mut origin: Arc<InstNode::InstNode>, mut parent: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        let mut outMod: Arc<Modifier> = propagateSubs(r#mod.clone(), list![Arc::new(Subscript::NFSubscript::SPLIT_PROXY { origin: origin.clone(), parent: parent.clone() })]);
        outMod
    }

    pub fn propagateSubs(mut r#mod: Arc<Modifier>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Arc<Modifier> {
        let mut r#mod: Arc<Modifier> = r#mod;
        let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ MODIFIER { .. } => {
            assign_variant_field!(r#mod => Modifier::MODIFIER; subModifiers = ModTable::map(var_field!((*r#mod).subModifiers, Modifier::MODIFIER).clone(), Arc::new({ let __pe_b2 = subs.clone(); move |__pe_a0, __pe_a1| Ok(propagateSubMod(__pe_a0, __pe_a1, __pe_b2.clone())) })));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        r#mod
    }

    pub fn propagateBinding(mut r#mod: Arc<Modifier>, mut origin: Arc<InstNode::InstNode>, mut parent: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        let mut r#mod: Arc<Modifier> = r#mod;
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ MODIFIER { .. } => {
            subs = list![Arc::new(Subscript::NFSubscript::SPLIT_PROXY { origin: origin.clone(), parent: parent.clone() })];
            assign_variant_field!(r#mod => Modifier::MODIFIER; binding = Binding::propagate(var_field!((*r#mod).binding, Modifier::MODIFIER).clone(), subs.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        r#mod
    }

    pub fn propagateSubMod(mut name: ArcStr, mut submod: Arc<Modifier>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Arc<Modifier> {
        let mut submod: Arc<Modifier> = submod;
        let () = (::match_deref::match_deref! { match &(submod.clone()) {
        Deref @ MODIFIER { eachPrefix: SCode::Each::NOT_EACH, .. } => {
            assign_variant_field!(submod => Modifier::MODIFIER;
                binding = Binding::propagate(var_field!((*submod).binding, Modifier::MODIFIER).clone(), subs.clone()),
                subModifiers = ModTable::map(var_field!((*submod).subModifiers, Modifier::MODIFIER).clone(), Arc::new({ let __pe_b2 = subs.clone(); move |__pe_a0, __pe_a1| Ok(propagateSubMod(__pe_a0, __pe_a1, __pe_b2.clone())) }))
            );
            ()
        },
        Deref @ REDECLARE { eachPrefix: SCode::Each::NOT_EACH, .. } => {
            assign_variant_field!(submod => Modifier::REDECLARE;
                innerMod = propagateSubMod((name.clone()).clone(), var_field!((*submod).innerMod, Modifier::REDECLARE).clone(), subs.clone()),
                propagatedSubs = listAppend(subs.clone(), var_field!((*submod).propagatedSubs, Modifier::REDECLARE).clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        submod
    }

    pub fn isEmpty(mut r#mod: Arc<Modifier>) -> bool {
        let mut isEmpty: bool = false;
        isEmpty = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ NOMOD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isEmpty
    }

    pub fn isRedeclare(mut r#mod: Arc<Modifier>) -> bool {
        let mut isRedeclare: bool = false;
        isRedeclare = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ REDECLARE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isRedeclare
    }

    pub fn toList(mut r#mod: Arc<Modifier>) -> Arc<metamodelica::List<Arc<Modifier>>> {
        let mut modList: Arc<metamodelica::List<Arc<Modifier>>> = metamodelica::nil();
        modList = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ MODIFIER { .. } => ModTable::listValues(var_field!((*r#mod).subModifiers, Modifier::MODIFIER).clone(), metamodelica::nil()),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        modList
    }

    pub fn isEach(mut r#mod: Arc<Modifier>) -> bool {
        let mut isEach: bool = false;
        isEach = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ MODIFIER { eachPrefix: SCode::Each::EACH, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isEach
    }

    pub fn isFinal(mut r#mod: Arc<Modifier>) -> bool {
        let mut isFinal: bool = false;
        isFinal = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ MODIFIER { finalPrefix: SCode::Final::FINAL, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isFinal
    }

    pub fn map(mut r#mod: Arc<Modifier>, mut func: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Modifier>) -> Result<Arc<Modifier>> + 'static>) -> Arc<Modifier> {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Modifier>) -> Result<Arc<Modifier>> + 'static>;

        let mut r#mod: Arc<Modifier> = r#mod;
        let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ MODIFIER { .. } => {
            assign_variant_field!(r#mod => Modifier::MODIFIER; subModifiers = ModTable::map(var_field!((*r#mod).subModifiers, Modifier::MODIFIER).clone(), func.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        r#mod
    }

    pub fn toString(mut r#mod: Arc<Modifier>, mut printName: bool) -> Result<ArcStr> {
        let mut string: ArcStr = arcstr::literal!("");
        string = ((::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ MODIFIER { .. } => {
            let mut submods: Arc<metamodelica::List<Arc<Modifier>>> = metamodelica::nil();
            let mut subs_str: ArcStr = arcstr::literal!("");
            let mut binding_str: ArcStr = arcstr::literal!("");
            let mut binding_sep: ArcStr = arcstr::literal!("");
            submods = ModTable::listValues(var_field!((*r#mod).subModifiers, Modifier::MODIFIER).clone(), metamodelica::nil());
            if !(submods.clone().is_empty()) {
                subs_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut s in (submods.clone()).into_iter().cloned() {
            let __x = toString(s.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                binding_sep = (literal!(" = ")).clone();
            } else {
                subs_str = (literal!("")).clone();
                binding_sep = (if (printName.clone()) {literal!(" = ")} else {literal!("= ")}).clone();
            }
            binding_str = (Binding::toString(var_field!((*r#mod).binding, Modifier::MODIFIER).clone(), (binding_sep.clone()).clone())?).clone();
            if (printName.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*r#mod).name, Modifier::MODIFIER).clone()); __mm_s.push_str(&*subs_str.clone()); __mm_s.push_str(&*binding_str.clone()); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*subs_str.clone()); __mm_s.push_str(&*binding_str.clone()); ArcStr::from(__mm_s) }}
        },
        Deref @ REDECLARE { .. } => {
            InstNode::toString(var_field!((*r#mod).element, Modifier::REDECLARE).clone())?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(string)
    }

    pub fn toFlatStreamList(mut modifiers: Arc<metamodelica::List<Arc<Modifier>>>, mut format: BaseModelica::OutputFormat, mut s: IOStream::IOStream, mut delimiter: ArcStr) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        let mut mods: Arc<metamodelica::List<Arc<Modifier>>> = modifiers.clone();
        if mods.clone().is_empty() {
            return Ok(s);
        }
        loop {
            s = toFlatStream(listHead(mods.clone())?, format.clone(), s.clone(), true)?;
            mods = listRest(mods.clone())?;
            if mods.clone().is_empty() {
                break;
            } else {
                s = IOStream::append(s.clone(), (delimiter.clone()).clone())?;
            }
        }
        Ok(s)
    }

    pub fn toFlatStream(mut r#mod: Arc<Modifier>, mut format: BaseModelica::OutputFormat, mut s: IOStream::IOStream, mut printName: bool) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        let mut submods: Arc<metamodelica::List<Arc<Modifier>>> = metamodelica::nil();
        let mut subs_str: ArcStr = arcstr::literal!("");
        let mut binding_str: ArcStr = arcstr::literal!("");
        let mut binding_sep: ArcStr = arcstr::literal!("");
        let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ MODIFIER { .. } => {
            if printName.clone() {
                s = IOStream::append(s.clone(), (var_field!((*r#mod).name, Modifier::MODIFIER).clone()).clone())?;
            }
            submods = ModTable::listValues(var_field!((*r#mod).subModifiers, Modifier::MODIFIER).clone(), metamodelica::nil());
            if !(submods.clone().is_empty()) {
                s = IOStream::append(s.clone(), (literal!("(")).clone())?;
                s = toFlatStreamList(submods.clone(), format.clone(), s.clone(), (literal!(", ")).clone())?;
                s = IOStream::append(s.clone(), (literal!(")")).clone())?;
                binding_sep = (literal!(" = ")).clone();
            } else {
                binding_sep = (if (printName.clone()) {literal!(" = ")} else {literal!("= ")}).clone();
            }
            s = IOStream::append(s.clone(), (Binding::toFlatString(var_field!((*r#mod).binding, Modifier::MODIFIER).clone(), format.clone(), (binding_sep.clone()).clone())?).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(s)
    }

    pub fn toFlatString(mut r#mod: Arc<Modifier>, mut format: BaseModelica::OutputFormat, mut printName: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut s: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
        s = IOStream::create((literal!("NFModifier.Modifier.toFlatString")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
        s = toFlatStream(r#mod.clone(), format.clone(), s.clone(), printName.clone())?;
        r#str = (IOStream::string(s.clone())?).clone();
        IOStream::delete(s.clone())?;
        Ok(r#str)
    }

    fn createSubMod(mut subMod: Arc<SCode::SubMod>, mut modScope: Arc<ModifierScope::ModifierScope>, mut scope: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        let mut r#mod: Arc<Modifier> = create(subMod.r#mod.clone(), (subMod.ident.clone()).clone(), modScope.clone(), scope.clone()).unwrap();
        r#mod
    }

    fn checkFinalOverride(mut innerFinal: SCode::Final, mut outerMod: Arc<Modifier>, mut innerInfo: SourceInfo) -> Result<()> {
        let _ = (match innerFinal.clone() {
        SCode::Final::FINAL => {
            Error::addMultiSourceMessage(Error::FINAL_COMPONENT_OVERRIDE.clone(), list![(name(outerMod.clone())?).clone(), (toString(outerMod.clone(), false)?).clone()], list![info(outerMod.clone())?, innerInfo.clone()])?;
            bail!("fail")
        },
        _ => (),
    });
        Ok(())
    }

    fn mergeLocal(mut mod1: Arc<Modifier>, mut mod2: Arc<Modifier>, mut name: ArcStr, mut scope: Arc<ModifierScope::ModifierScope>, mut prefix: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Modifier>> {
        let mut r#mod: Arc<Modifier> = Arc::new(Modifier::NOMOD);
        let mut comp_name: ArcStr = arcstr::literal!("");
        r#mod = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ MODIFIER { .. }, Deref @ MODIFIER { binding: Deref @ Binding::UNBOUND, .. }) => {
            assign_variant_field!(mod1 => Modifier::MODIFIER; subModifiers = ModTable::join(var_field!((*mod1).subModifiers, Modifier::MODIFIER).clone(), var_field!((*mod2).subModifiers, Modifier::MODIFIER).clone(), Arc::new({ let __pe_b3 = scope.clone(); let __pe_b4 = cons((var_field!((*mod1).name, Modifier::MODIFIER).clone()).clone(), prefix.clone()); move |__pe_a0, __pe_a1, __pe_a2| mergeLocal(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone()) }))?);
            mod1.clone()
        },
        (Deref @ MODIFIER { binding: Deref @ Binding::UNBOUND, .. }, Deref @ MODIFIER { .. }) => {
            assign_variant_field!(mod2 => Modifier::MODIFIER; subModifiers = ModTable::join(var_field!((*mod2).subModifiers, Modifier::MODIFIER).clone(), var_field!((*mod1).subModifiers, Modifier::MODIFIER).clone(), Arc::new({ let __pe_b3 = scope.clone(); let __pe_b4 = cons((var_field!((*mod1).name, Modifier::MODIFIER).clone()).clone(), prefix.clone()); move |__pe_a0, __pe_a1, __pe_a2| mergeLocal(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone()) }))?);
            mod2.clone()
        },
        _ => {
            comp_name = stringDelimitList(cons((self::name(mod1.clone())?).clone(), prefix.clone()).reverse(), (literal!(".")).clone());
            Error::addMultiSourceMessage(Error::DUPLICATE_MODIFICATIONS.clone(), list![(comp_name.clone()).clone(), (ModifierScope::toString(scope.clone())?).clone()], list![info(mod1.clone())?, info(mod2.clone())?])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(r#mod)
    }

}

