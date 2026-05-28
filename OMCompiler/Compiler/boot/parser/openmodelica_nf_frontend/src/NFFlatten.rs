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

use crate::NFAlgorithm as Algorithm;
use crate::NFArrayConnections as ArrayConnections;
use crate::NFAttributes as Attributes;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCardinalityTable as CardinalityTable;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectEquations as ConnectEquations;
use crate::NFConnection as Connection;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFConnector::Face;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEvalConstants as EvalConstants;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpandableConnectors as ExpandableConnectors;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFlatModel as FlatModel;
use crate::NFFunction::Function;
use crate::NFInline as Inline;
use crate::NFInstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFInstUtil as InstUtil;
use crate::NFModifier::Modifier;
use crate::NFOCConnectionGraph;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Parallelism;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSimplifyModel as SimplifyModel;
use crate::NFStatement as Statement;
use crate::NFStreamFlowAlias as StreamFlowAlias;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub type FunctionTree = Arc<FunctionTreeImpl::Tree>;

pub type DeletedVariables = Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;

pub mod FunctionTreeImpl {
    use super::*;
    pub type Key = Arc<Path>;

    pub type Value = Arc<Function::Function>;

    pub fn keyStr(mut inKey: Key) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (AbsynUtil::pathString(inKey.clone(), (literal!(".")).clone(), true, false)?).clone();
        Ok(outString)
    }

    pub fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (literal!("")).clone();
        outString
    }

    pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
        let mut outResult: i32 = 0;
        outResult = AbsynUtil::pathCompareNoQual(inKey1.clone(), inKey2.clone())?;
        Ok(outResult)
    }

    pub use addConflictKeep as addConflictDefault;

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

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

    pub type ValueNode = Arc<Path>;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Result<Arc<Function::Function>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut key_comp: i32 = 0;
            key_comp = keyCompare(inKey.clone(), key.clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey.clone(), inValue.clone(), conflictFunc.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey.clone(), inValue.clone(), conflictFunc.clone())?);
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::NODE).clone(), key.clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::NODE).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::NODE; value = value.clone());
                }
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            let mut value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare(inKey.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }), right: Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }) });
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::LEAF).clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
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

    pub fn addConflictFail(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Result<Value> {
        let mut value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
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

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(Arc<Path>, Arc<Function::Function>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Result<Arc<Function::Function>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key;
        let mut value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Arc<Function::Function>>) -> Result<Arc<Function::Function>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Function::Function>>) -> Result<Value> + 'static>;

        let mut tree: Arc<Tree> = tree;
        let mut key_comp: i32 = 0;
        let mut new_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }),
        Deref @ Tree::NODE { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::NODE).clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), key.clone(), r#fn.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), key.clone(), r#fn.clone())?);
            } else {
                assign_variant_field!(tree => Tree::NODE; value = r#fn(Some(var_field!((*tree).value, Tree::NODE).clone()))?);
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }), right: Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }) });
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Function::Function>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

        let mut outResult: FT = inStartValue.clone();
        outResult = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            outResult = fold(var_field!((*inTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            outResult = fold(var_field!((*inTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            outResult = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            outResult.clone()
        },
        _ => {
            outResult.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outResult
    }

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Function::Function>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

        let mut value: FT = value;
        value = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), value.clone()).unwrap();
            if c.clone() {
                value = foldCond(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), value.clone());
                value = foldCond(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), value.clone());
            }
            value.clone()
        },
        Deref @ Tree::LEAF { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), value.clone()).unwrap();
            value.clone()
        },
        _ => {
            value.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value
    }

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Function::Function>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
        pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (foldArg1, foldArg2)
    }

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Function::Function>) -> Result<()> + 'static>) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let _ = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            forEach(var_field!((*tree).left, Tree::NODE).clone(), func.clone())?;
            func(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone())?;
            forEach(var_field!((*tree).right, Tree::NODE).clone(), func.clone())?;
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            func(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone())?;
            ()
        },
        Deref @ Tree::EMPTY { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
        Ok(())
    }

    pub fn fromList(mut inValues: Arc<metamodelica::List<(Arc<Path>, Arc<Function::Function>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Result<Arc<Function::Function>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY);
        let mut key: Key;
        let mut value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => var_field!((*tree).value, Tree::LEAF).clone(),
        (0, Deref @ Tree::NODE { .. }) => var_field!((*tree).value, Tree::NODE).clone(),
        (1, Deref @ Tree::NODE { .. }) => get(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(value)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Result<Option<Arc<Function::Function>>> {
        let mut value: Option<Arc<Function::Function>> = None;
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => key.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => Some(var_field!((*tree).value, Tree::LEAF).clone()),
        (0, Deref @ Tree::NODE { .. }) => Some(var_field!((*tree).value, Tree::NODE).clone()),
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Result<Arc<Function::Function>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ Tree::EMPTY { .. } => tree.clone(),
        Deref @ Tree::NODE { .. } => {
            tree = add(tree.clone(), var_field!((*treeToJoin).key, Tree::NODE).clone(), var_field!((*treeToJoin).value, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).left, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).right, Tree::NODE).clone(), conflictFunc.clone())?;
            tree.clone()
        },
        Deref @ Tree::LEAF { .. } => add(tree.clone(), var_field!((*treeToJoin).key, Tree::LEAF).clone(), var_field!((*treeToJoin).value, Tree::LEAF).clone(), conflictFunc.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn listKeys(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Path>>>) -> Arc<metamodelica::List<Arc<Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Path>>> = lst;
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

    pub fn listKeysReverse(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Path>>>) -> Arc<metamodelica::List<Arc<Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Path>>> = lst;
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

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Function::Function>>>) -> Arc<metamodelica::List<Arc<Function::Function>>> {
        let mut lst: Arc<metamodelica::List<Arc<Function::Function>>> = lst;
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

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Function::Function>) -> Result<Arc<Function::Function>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            new_left = map(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone());
            new_value = inFunc(key.clone(), value.clone()).unwrap();
            new_right = map(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
            new_value = inFunc(key.clone(), value.clone()).unwrap();
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

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Function::Function>, FT) -> Result<(Arc<Function::Function>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            (new_left, outResult) = mapFold(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            (new_right, outResult) = mapFold(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value = Arc::new(<Function::Function as ::std::default::Default>::default());
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
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
        let mut outTree: Arc<Tree> = Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY);
        outTree
    }

    pub fn printNodeStr(mut inNode: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::NODE).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::LEAF).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
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
        let mut val_node: Option<Arc<Path>> = None;
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::NFFlatten::FunctionTreeImpl::Tree::EMPTY), node.clone())?
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
        (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => Arc::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
        (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig.clone(),
        (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE).clone(), left.clone()) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE).clone(), right.clone())) {orig.clone()} else {Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() })},
        (Deref @ Tree::LEAF { .. }, _, _) => Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() }),
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

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(Arc<Path>, Arc<Function::Function>)>>) -> Arc<metamodelica::List<(Arc<Path>, Arc<Function::Function>)>> {
        let mut lst: Arc<metamodelica::List<(Arc<Path>, Arc<Function::Function>)>> = lst;
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
        let mut outTree: Arc<Tree> = add(tree.clone(), key.clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Arc<Function::Function>, Arc<Function::Function>, Arc<Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Result<Arc<Function::Function>> + 'static>)).unwrap();
        outTree
    }

}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlattenSettings {
    pub scalarize: bool,
    pub arrayConnect: bool,
    pub nfAPI: bool,
    pub relaxedErrorChecking: bool,
    pub newBackend: bool,
    pub vectorizeBindings: bool,
    pub implicitStartAttribute: bool,
    pub minimalEval: bool,
}

impl Default for FlattenSettings {
    fn default() -> Self {
        Self {
            scalarize: Default::default(),
            arrayConnect: Default::default(),
            nfAPI: Default::default(),
            relaxedErrorChecking: Default::default(),
            newBackend: Default::default(),
            vectorizeBindings: Default::default(),
            implicitStartAttribute: Default::default(),
            minimalEval: Default::default(),
        }
    }
}

pub type SETTINGS = FlattenSettings;


pub mod Prefix {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Prefix {
        PREFIX {
            root: Arc<InstNode::InstNode>,
            prefix: Arc<ComponentRef::NFComponentRef>,
        },
        INDEXED_PREFIX {
            root: Arc<InstNode::InstNode>,
            prefix: Arc<ComponentRef::NFComponentRef>,
            indexedPrefix: Arc<ComponentRef::NFComponentRef>,
        },
    }
    pub use self::Prefix::{PREFIX,INDEXED_PREFIX};
    pub fn new(mut root: Arc<InstNode::InstNode>, mut indexed: bool) -> Arc<Prefix> {
        let mut prefix: Arc<Prefix>;
        prefix = if (indexed.clone()) {Arc::new(Prefix::INDEXED_PREFIX { root: root.clone(), prefix: Arc::new(crate::NFComponentRef::EMPTY), indexedPrefix: Arc::new(crate::NFComponentRef::EMPTY) })} else {Arc::new(Prefix::PREFIX { root: root.clone(), prefix: Arc::new(crate::NFComponentRef::EMPTY) })};
        prefix
    }

    pub fn isEmpty(mut prefix: Arc<Prefix>) -> Result<bool> {
        let mut empty: bool = false;
        empty = (::match_deref::match_deref! { match &(prefix.clone()) {
        Deref @ PREFIX { .. } => ComponentRef::isEmpty(var_field!((*prefix).prefix, Prefix::PREFIX).clone()),
        Deref @ INDEXED_PREFIX { .. } => ComponentRef::isEmpty(var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX).clone()),
        _ => bail!("match: no arm matched"),
    } });
        Ok(empty)
    }

    pub fn isIndexed(mut prefix: Arc<Prefix>) -> bool {
        let mut indexed: bool = false;
        indexed = (::match_deref::match_deref! { match &(prefix.clone()) {
        Deref @ INDEXED_PREFIX { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        indexed
    }

    pub fn push(mut node: Arc<InstNode::InstNode>, mut ty: Arc<Type::NFType>, mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut prefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
        let mut prefix: Arc<Prefix> = prefix;
        let () = (::match_deref::match_deref! { match &(prefix.clone()) {
        Deref @ PREFIX { .. } => {
            assign_variant_field!(prefix => Prefix::PREFIX; prefix = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), var_field!((*prefix).prefix, Prefix::PREFIX).clone()));
            ()
        },
        Deref @ INDEXED_PREFIX { .. } => {
            assign_variant_field!(prefix => Prefix::INDEXED_PREFIX;
                prefix = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), var_field!((*prefix).prefix, Prefix::INDEXED_PREFIX).clone()),
                indexedPrefix = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX).clone()),
                indexedPrefix = ComponentRef::setSubscripts(makeBindingIterators(var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX).clone(), dims.clone())?, var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX).clone())?
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(prefix)
    }

    pub fn pop(mut prefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
        let mut prefix: Arc<Prefix> = prefix;
        let () = (::match_deref::match_deref! { match &(prefix.clone()) {
        Deref @ PREFIX { .. } => {
            assign_variant_field!(prefix => Prefix::PREFIX; prefix = ComponentRef::rest(var_field!((*prefix).prefix, Prefix::PREFIX).clone())?);
            ()
        },
        Deref @ INDEXED_PREFIX { .. } => {
            assign_variant_field!(prefix => Prefix::INDEXED_PREFIX;
                prefix = ComponentRef::rest(var_field!((*prefix).prefix, Prefix::INDEXED_PREFIX).clone())?,
                indexedPrefix = ComponentRef::rest(var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX).clone())?
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(prefix)
    }

    pub fn prefix(mut prefix: Arc<Prefix>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        cref = (::match_deref::match_deref! { match &(prefix.clone()) {
        Deref @ PREFIX { .. } => var_field!((*prefix).prefix, Prefix::PREFIX).clone(),
        Deref @ INDEXED_PREFIX { .. } => var_field!((*prefix).prefix, Prefix::INDEXED_PREFIX).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(cref)
    }

    pub fn indexedPrefix(mut prefix: Arc<Prefix>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        cref = (::match_deref::match_deref! { match &(prefix.clone()) {
        Deref @ PREFIX { .. } => var_field!((*prefix).prefix, Prefix::PREFIX).clone(),
        Deref @ INDEXED_PREFIX { .. } => var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(cref)
    }

    pub fn toNonIndexedPrefix(mut prefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
        let mut prefix: Arc<Prefix> = prefix;
        prefix = (::match_deref::match_deref! { match &(prefix.clone()) {
        Deref @ PREFIX { .. } => prefix.clone(),
        Deref @ INDEXED_PREFIX { .. } => Arc::new(Prefix::PREFIX { root: var_field!((*prefix).root, Prefix::INDEXED_PREFIX).clone(), prefix: var_field!((*prefix).prefix, Prefix::INDEXED_PREFIX).clone() }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(prefix)
    }

    pub fn apply(mut prefix: Arc<Prefix>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
        cref = ComponentRef::transferSubscripts(indexedPrefix(prefix.clone())?, cref.clone())?;
        Ok(cref)
    }

    pub fn subscript(mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut prefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
        let mut prefix: Arc<Prefix> = prefix;
        let () = (::match_deref::match_deref! { match &(prefix.clone()) {
        Deref @ PREFIX { .. } => {
            assign_variant_field!(prefix => Prefix::PREFIX; prefix = ComponentRef::setSubscripts(subs.clone(), var_field!((*prefix).prefix, Prefix::PREFIX).clone())?);
            ()
        },
        Deref @ INDEXED_PREFIX { .. } => {
            assign_variant_field!(prefix => Prefix::INDEXED_PREFIX; prefix = ComponentRef::setSubscripts(subs.clone(), var_field!((*prefix).prefix, Prefix::INDEXED_PREFIX).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(prefix)
    }

    pub fn toString(mut pre: Arc<Prefix>) -> ArcStr {
        let mut r#str: ArcStr = ComponentRef::toString(prefix(pre.clone()).unwrap()).unwrap();
        r#str
    }

    pub fn rootNode(mut pre: Arc<Prefix>) -> Result<Arc<InstNode::InstNode>> {
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        node = (::match_deref::match_deref! { match &(pre.clone()) {
        Deref @ PREFIX { .. } => var_field!((*pre).root, Prefix::PREFIX).clone(),
        Deref @ INDEXED_PREFIX { .. } => var_field!((*pre).root, Prefix::INDEXED_PREFIX).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn instanceName(mut pre: Arc<Prefix>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (InstNode::name(rootNode(pre.clone())?)?).clone();
        if !(ComponentRef::isEmpty(indexedPrefix(pre.clone())?)) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*toString(pre.clone())); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

}

thread_local! { static __EMPTY_PREFIX_TLS: Arc<Prefix::Prefix> = Arc::new(Prefix::Prefix::PREFIX { root: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), prefix: Arc::new(crate::NFComponentRef::EMPTY) }); }
pub fn EMPTY_PREFIX() -> Arc<Prefix::Prefix> { __EMPTY_PREFIX_TLS.with(|__t| __t.clone()) }

thread_local! { static __EMPTY_INDEXED_PREFIX_TLS: Arc<Prefix::Prefix> = Arc::new(Prefix::Prefix::INDEXED_PREFIX { root: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), prefix: Arc::new(crate::NFComponentRef::EMPTY), indexedPrefix: Arc::new(crate::NFComponentRef::EMPTY) }); }
pub fn EMPTY_INDEXED_PREFIX() -> Arc<Prefix::Prefix> { __EMPTY_INDEXED_PREFIX_TLS.with(|__t| __t.clone()) }

pub fn flatten(mut classInst: Arc<InstNode::InstNode>, mut classPath: Arc<Path>, mut getConnectionResolved: bool) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = Arc::new(<FlatModel::NFFlatModel as ::std::default::Default>::default());
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ieql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut alg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let mut ialg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut cmt: Option<Arc<SCode::Comment>> = None;
    let mut settings: FlattenSettings = <FlattenSettings as ::std::default::Default>::default();
    let mut deleted_vars: DeletedVariables;
    let mut prefix: Arc<Prefix::Prefix>;
    settings = FlattenSettings { scalarize: Flags::isSet(Flags::NF_SCALARIZE.clone())?, arrayConnect: Flags::isSet(Flags::ARRAY_CONNECT.clone())?, nfAPI: Flags::isSet(Flags::NF_API.clone())?, relaxedErrorChecking: Flags::isSet(Flags::NF_API.clone())? || Flags::getConfigBool(Flags::CHECK_MODEL.clone())?, newBackend: Flags::getConfigBool(Flags::NEW_BACKEND.clone())?, vectorizeBindings: Flags::isSet(Flags::VECTORIZE_BINDINGS.clone())?, implicitStartAttribute: Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("implicitParameterStartAttribute")).clone())?, minimalEval: Flags::getConfigString(Flags::EVALUATE_STRUCTURAL_PARAMETERS.clone())? != literal!("all") };
    prefix = Prefix::new(classInst.clone(), settings.vectorizeBindings.clone());
    sections = Arc::new(crate::NFSections::EMPTY);
    src = ElementSource::createElementSource(InstNode::info(classInst.clone())?, None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?;
    src = ElementSource::addCommentToSource(src.clone(), SCodeUtil::getElementComment(InstNode::definition(classInst.clone())?));
    deleted_vars = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    (vars, sections) = flattenClass(InstNode::getClass(classInst.clone())?, prefix.clone(), Visibility::PUBLIC.clone(), None, metamodelica::nil(), sections.clone(), deleted_vars.clone(), settings.clone())?;
    vars = vars.clone().reverse();
    flatModel = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { .. } => {
            eql = var_field!((*sections).equations, Sections::NFSections::SECTIONS).clone().reverse();
            ieql = var_field!((*sections).initialEquations, Sections::NFSections::SECTIONS).clone().reverse();
            alg = var_field!((*sections).algorithms, Sections::NFSections::SECTIONS).clone().reverse();
            ialg = var_field!((*sections).initialAlgorithms, Sections::NFSections::SECTIONS).clone().reverse();
            Arc::new(FlatModel::NFFlatModel { name: classPath.clone(), variables: vars.clone(), equations: eql.clone(), initialEquations: ieql.clone(), algorithms: alg.clone(), initialAlgorithms: ialg.clone(), source: src.clone() })
        },
        _ => Arc::new(FlatModel::NFFlatModel { name: classPath.clone(), variables: vars.clone(), equations: metamodelica::nil(), initialEquations: metamodelica::nil(), algorithms: metamodelica::nil(), initialAlgorithms: metamodelica::nil(), source: src.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    assign_field!(
        flatModel.algorithms = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut al in (flatModel.algorithms.clone()).into_iter().cloned() {
            let __x = Algorithm::setInputsOutputs(al.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.initialAlgorithms = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut al in (flatModel.initialAlgorithms.clone()).into_iter().cloned() {
            let __x = Algorithm::setInputsOutputs(al.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
    );
    execStat((literal!("NFFlatten.flatten")).clone())?;
    InstUtil::dumpFlatModelDebug((literal!("flatten")).clone(), flatModel.clone(), FunctionTreeImpl::new())?;
    if getConnectionResolved.clone() {
        if settings.newBackend.clone() {
            assign_field!(flatModel.equations = evaluateIfWithConnects(flatModel.equations.clone())?);
        }
        if settings.arrayConnect.clone() {
            flatModel = resolveArrayConnections(flatModel.clone())?;
        } else {
            flatModel = resolveConnections(flatModel.clone(), deleted_vars.clone(), settings.clone())?;
        }
        InstUtil::dumpFlatModelDebug((literal!("connections")).clone(), flatModel.clone(), FunctionTreeImpl::new())?;
    }
    assign_field!(flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut var in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = updateVariability(var.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if !(Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("illegalConditionalContext")).clone())?) {
        checkDeletedVarRefs(flatModel.clone(), deleted_vars.clone(), settings.clone())?;
    }
    Ok(flatModel)
}

pub fn flattenConnection(mut classInst: Arc<InstNode::InstNode>, mut classPath: Arc<Path>) -> Result<Arc<Connections::NFConnections>> {
    let mut conns: Arc<Connections::NFConnections> = Arc::new(<Connections::NFConnections as ::std::default::Default>::default());
    let mut flatModel: Arc<FlatModel::NFFlatModel> = Arc::new(<FlatModel::NFFlatModel as ::std::default::Default>::default());
    let mut deleted_vars: DeletedVariables;
    flatModel = flatten(classInst.clone(), classPath.clone(), false)?;
    deleted_vars = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    (flatModel, conns) = Connections::collectConnections(flatModel.clone(), Arc::new({ let __pe_b1 = deleted_vars.clone(); move |__pe_a0| isDeletedCref(__pe_a0, __pe_b1.clone()) }))?;
    (_, conns) = ExpandableConnectors::elaborate(flatModel.clone(), conns.clone())?;
    conns = Connections::collectFlows(flatModel.clone(), conns.clone())?;
    Ok(conns)
}

pub fn collectFunctions(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = Arc::new(FunctionTreeImpl::Tree::EMPTY);
    funcs = FunctionTreeImpl::new();
    funcs = List::fold(flatModel.variables.clone(), (std::sync::Arc::new(collectComponentFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
    funcs = List::fold(flatModel.equations.clone(), (std::sync::Arc::new(collectEquationFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
    funcs = List::fold(flatModel.initialEquations.clone(), (std::sync::Arc::new(collectEquationFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
    funcs = List::fold(flatModel.algorithms.clone(), (std::sync::Arc::new(fnptr!(collectAlgorithmFuncs, Arc<Algorithm::NFAlgorithm>, Arc<FunctionTreeImpl::Tree>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
    funcs = List::fold(flatModel.initialAlgorithms.clone(), (std::sync::Arc::new(fnptr!(collectAlgorithmFuncs, Arc<Algorithm::NFAlgorithm>, Arc<FunctionTreeImpl::Tree>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
    execStat((literal!("NFFlatten.collectFunctions")).clone())?;
    Ok(funcs)
}

pub fn fillVectorizedVariableBinding(mut var: Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    let mut ty_attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
    let mut attr_name: ArcStr = arcstr::literal!("");
    let mut attr_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    assign_field!(var.binding = fillVectorizedBinding(var.binding.clone(), var.ty.clone())?);
    for mut ty_attr in &*var.typeAttributes.clone() {
        let mut ty_attr = ty_attr.clone();
        (attr_name, attr_binding) = ty_attr.clone();
        attr_binding = fillVectorizedBinding(attr_binding.clone(), Type::copyDims(var.ty.clone(), Binding::getType(attr_binding.clone())?))?;
        ty_attrs = cons((attr_name.clone(), attr_binding.clone()), ty_attrs.clone());
    }
    assign_field!(var.typeAttributes = ty_attrs.clone().reverse());
    Ok(var)
}

fn flattenClass(mut cls: Arc<Class::NFClass>, mut prefix: Arc<Prefix::Prefix>, mut visibility: Visibility, mut binding: Option<Arc<Binding::NFBinding>>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut sections: Arc<Sections::NFSections>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<Sections::NFSections>)> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut sections: Arc<Sections::NFSections> = sections;
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut bindings: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = metamodelica::nil();
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::TYPE, .. } => (),
        Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: comps, .. }, .. } => {
            if isSome(binding.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(binding.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                b = __pa0.clone();
                if Binding::isBound(b.clone()) {
                    b = flattenBinding(b.clone(), Prefix::pop(prefix.clone())?, false)?;
                    bindings = getRecordBindings(b.clone(), comps.clone(), prefix.clone())?;
                }
            }
            if bindings.clone().is_empty() {
                let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range1 {
                    (vars, sections) = flattenComponent(c.clone(), prefix.clone(), visibility.clone(), binding.clone(), vars.clone(), sections.clone(), deletedVars.clone(), settings.clone())?;
                }
            } else {
                let __range2 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range2 {
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(bindings.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    b = __pa3.clone();
                    bindings = __pa4.clone();
                    (vars, sections) = flattenComponent(c.clone(), prefix.clone(), visibility.clone(), Some(b.clone()), vars.clone(), sections.clone(), deletedVars.clone(), settings.clone())?;
                }
            }
            sections = flattenSections(var_field!((*cls).sections, Class::NFClass::INSTANCED_CLASS).clone(), Prefix::toNonIndexedPrefix(prefix.clone())?, sections.clone(), settings.clone())?;
            ()
        },
        Deref @ Class::TYPED_DERIVED { .. } => {
            (vars, sections) = flattenClass(InstNode::getClass(var_field!((*cls).baseClass, Class::NFClass::TYPED_DERIVED).clone())?, prefix.clone(), visibility.clone(), binding.clone(), vars.clone(), sections.clone(), deletedVars.clone(), settings.clone())?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFlatten.flattenClass")); __mm_s.push_str(&*literal!(" got non-instantiated component ")); __mm_s.push_str(&*Prefix::toString(prefix.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((vars, sections))
}

fn flattenComponent(mut component: Arc<InstNode::InstNode>, mut prefix: Arc<Prefix::Prefix>, mut visibility: Visibility, mut outerBinding: Option<Arc<Binding::NFBinding>>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut sections: Arc<Sections::NFSections>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<Sections::NFSections>)> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut sections: Arc<Sections::NFSections> = sections;
    let mut comp_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut condition: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut vis: Visibility = Visibility::PUBLIC;
    let mut children: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    if InstNode::isEmpty(component.clone()) || InstNode::isOnlyOuter(component.clone())? {
        return Ok((vars, sections));
    }
    comp_node = InstNode::resolveOuter(component.clone());
    c = InstNode::component(comp_node.clone())?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { ty, condition, .. } => {
            if isDeletedComponent(condition.clone(), prefix.clone())? {
                deleteComponent(component.clone(), prefix.clone(), deletedVars.clone())?;
                return Ok((vars, sections));
            }
            cls = InstNode::getClass(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())?;
            vis = if (InstNode::isProtected(component.clone())) {Visibility::PROTECTED.clone()} else {visibility.clone()};
            (vars, sections) = (match getComponentType(ty.clone(), settings.clone()) {
        ComponentType::COMPLEX { .. } => flattenComplexComponent(comp_node.clone(), c.clone(), cls.clone(), ty.clone(), vis.clone(), outerBinding.clone(), prefix.clone(), vars.clone(), sections.clone(), deletedVars.clone(), settings.clone())?,
        ComponentType::NORMAL => flattenSimpleComponent(comp_node.clone(), c.clone(), vis.clone(), outerBinding.clone(), Class::getTypeAttributes(cls.clone()), prefix.clone(), vars.clone(), sections.clone(), settings.clone(), metamodelica::nil())?,
        ComponentType::RECORD { .. } => {
            (children, sections) = flattenComplexComponent(comp_node.clone(), c.clone(), cls.clone(), ty.clone(), vis.clone(), outerBinding.clone(), prefix.clone(), metamodelica::nil(), sections.clone(), deletedVars.clone(), settings.clone())?;
            flattenSimpleComponent(comp_node.clone(), c.clone(), vis.clone(), outerBinding.clone(), Class::getTypeAttributes(cls.clone()), prefix.clone(), vars.clone(), sections.clone(), settings.clone(), children.clone().reverse())?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFlatten.flattenComponent")); __mm_s.push_str(&*literal!(" got unknown component")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
            ()
        },
        _ if (Component::isDeleted(c.clone())?) => {
            deleteComponent(component.clone(), prefix.clone(), deletedVars.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFlatten.flattenComponent")); __mm_s.push_str(&*literal!(" got unknown component")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((vars, sections))
}

fn isDeletedComponent(mut condition: Arc<Binding::NFBinding>, mut prefix: Arc<Prefix::Prefix>) -> Result<bool> {
    let mut isDeleted: bool = false;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cond: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    if Binding::isBound(condition.clone()) {
        cond = flattenBinding(condition.clone(), prefix.clone(), false)?;
        exp = Binding::getTypedExp(cond.clone())?;
        exp = Ceval::evalExp(exp.clone(), Ceval::EvalTarget::new(Binding::getInfo(cond.clone()), NFInstContext::CONDITION.clone(), None))?;
        exp = Expression::expandSplitIndices(exp.clone())?;
        if Expression::arrayAllEqual(exp.clone())? {
            exp = Expression::arrayFirstScalar(exp.clone())?;
        }
        isDeleted = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BOOLEAN { .. } => !(var_field!((*exp).value, Expression::NFExpression::BOOLEAN).clone()),
        _ => {
            Error::addSourceMessage(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone(), list![(Expression::toString(exp.clone())?).clone()], Binding::getInfo(cond.clone()))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        isDeleted = false;
    }
    Ok(isDeleted)
}

fn deleteComponent(mut node: Arc<InstNode::InstNode>, mut prefix: Arc<Prefix::Prefix>, mut deletedVars: DeletedVariables) -> Result<()> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    cref = ComponentRef::prefixCref(node.clone(), Arc::new(crate::NFType::UNKNOWN), metamodelica::nil(), Prefix::prefix(prefix.clone())?);
    UnorderedSet::add(cref.clone(), deletedVars.clone())?;
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getComponentType(mut ty: Arc<Type::NFType>, mut settings: FlattenSettings) -> ComponentType {
    let mut compTy: ComponentType = ComponentType::NORMAL;
    compTy = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => ComponentType::NORMAL.clone(),
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } if (settings.newBackend.clone()) => ComponentType::RECORD.clone(),
        Deref @ Type::COMPLEX { .. } => ComponentType::COMPLEX.clone(),
        Deref @ Type::ARRAY { .. } => getComponentType(var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), settings.clone()),
        _ => ComponentType::NORMAL.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    compTy
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ComponentType {
    NORMAL = 1,
    COMPLEX = 2,
    RECORD = 3,
}
impl PartialOrd for ComponentType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ComponentType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

fn flattenSimpleComponent(mut node: Arc<InstNode::InstNode>, mut comp: Arc<Component::NFComponent>, mut visibility: Visibility, mut outerBinding: Option<Arc<Binding::NFBinding>>, mut typeAttrs: Arc<metamodelica::List<Arc<Modifier::Modifier>>>, mut prefix: Arc<Prefix::Prefix>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut sections: Arc<Sections::NFSections>, mut settings: FlattenSettings, mut children: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<Sections::NFSections>)> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut sections: Arc<Sections::NFSections> = sections;
    let mut comp_node: Arc<InstNode::InstNode> = node.clone();
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut comp_attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut vis: Visibility = Visibility::PUBLIC;
    let mut eq: Arc<Equation::NFEquation>;
    let mut ty_attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let mut unfix: bool = false;
    let mut pre: Arc<Prefix::Prefix>;
    let mut v: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut fillVectorizedBindingFails: bool = false;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT { info: __pa0, comment: __pa1, attributes: __pa2, binding: __pa3, ty: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    cmt = __pa1.clone();
    comp_attr = __pa2.clone();
    binding = __pa3.clone();
    ty = __pa4.clone();
    checkUnspecifiedEnumType(ty.clone(), node.clone(), info.clone())?;
    var = comp_attr.variability.clone();
    if isSome(outerBinding.clone()) {
        let __pa5 = ::match_deref::match_deref! { match &(outerBinding.clone()) {
            Some(__pa5) => __pa5.clone(),
            _ => bail!("pattern mismatch"),
        } };
        binding = __pa5.clone();
        unfix = Binding::isUnbound(binding.clone()) && var.clone() == Variability::PARAMETER.clone();
    } else {
        binding = flattenBinding(binding.clone(), prefix.clone(), false)?;
        unfix = false;
    }
    if !(settings.scalarize.clone()) && !(settings.vectorizeBindings.clone()) && Binding::isBound(binding.clone()) && !(Prefix::isEmpty(prefix.clone())?) && Type::isArray(ComponentRef::nodeType(Prefix::prefix(prefix.clone())?)?) {
        fillVectorizedBindingFails = containsPrefix(Binding::getExp(binding.clone())?, prefix.clone())?;
    }
    if !(settings.nfAPI.clone()) && settings.scalarize.clone() || fillVectorizedBindingFails.clone() {
        if var.clone() >= Variability::DISCRETE.clone() && Type::isArray(ty.clone()) && !(Type::isExternalObject(Type::arrayElementType(ty.clone()))) && Binding::isBound(binding.clone()) || fillVectorizedBindingFails.clone() {
            name = ComponentRef::prefixCref(comp_node.clone(), ty.clone(), metamodelica::nil(), Prefix::prefix(prefix.clone())?);
            eq = Equation::makeEquality(Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: name.clone() }), Binding::getTypedExp(binding.clone())?, ty.clone(), ElementSource::createElementSource(info.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?, Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::DONT_SCALARIZE.clone());
            sections = Sections::prependEquation(eq.clone(), sections.clone(), false)?;
            binding = Binding::EMPTY_BINDING().clone();
            if comp_attr.direction.clone() == Direction::INPUT.clone() && Prefix::isEmpty(prefix.clone())? {
                assign_field!(comp_attr.direction = Direction::NONE.clone());
                Error::addSourceMessage(Error::TOP_LEVEL_INPUT_WITH_BINDING.clone(), list![(ComponentRef::toString(name.clone())?).clone()], info.clone())?;
            }
        }
    }
    ty = flattenType(ty.clone(), prefix.clone(), info.clone());
    verifyDimensions(Type::arrayDims(ty.clone()), comp_node.clone())?;
    pre = Prefix::push(comp_node.clone(), ty.clone(), Type::arrayDims(ty.clone()), prefix.clone())?;
    ty_attrs = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut m in (typeAttrs.clone()).into_iter().cloned() {
            let __x = flattenTypeAttribute(m.clone(), prefix.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if unfix.clone() {
        ty_attrs = Binding::setAttr(ty_attrs.clone(), (literal!("fixed")).clone(), Binding::makeFlat(Arc::new(Expression::NFExpression::BOOLEAN { value: false }), Variability::CONSTANT.clone(), Binding::Source::GENERATED.clone()));
    }
    name = Prefix::prefix(pre.clone())?;
    v = Arc::new(Variable::NFVariable { name: name.clone(), ty: ty.clone(), binding: binding.clone(), visibility: visibility.clone(), attributes: comp_attr.clone(), typeAttributes: ty_attrs.clone(), children: children.clone(), comment: cmt.clone(), info: info.clone(), backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone() });
    if !(settings.relaxedErrorChecking.clone()) && var.clone() < Variability::DISCRETE.clone() && !(unfix.clone()) && !(Type::isComplex(Type::arrayElementType(ty.clone()))) {
        v = verifyBinding(v.clone(), var.clone(), binding.clone(), settings.clone())?;
    }
    vars = cons(v.clone(), vars.clone());
    Ok((vars, sections))
}

fn checkUnspecifiedEnumType(mut ty: Arc<Type::NFType>, mut node: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ENUMERATION { literals: Deref @ metamodelica::List::Nil, .. } => {
            Error::addSourceMessage(Error::UNSPECIFIED_ENUM_COMPONENT.clone(), list![(InstNode::name(node.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn flattenTypeAttribute(mut attr: Arc<Modifier::Modifier>, mut prefix: Arc<Prefix::Prefix>) -> Result<(ArcStr, Arc<Binding::NFBinding>)> {
    let mut outAttr: (ArcStr, Arc<Binding::NFBinding>);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    binding = flattenBinding(Modifier::binding(attr.clone()), prefix.clone(), true)?;
    outAttr = (Modifier::name(attr.clone())?, binding.clone());
    Ok(outAttr)
}

fn isTypeAttributeNamed(mut name: ArcStr, mut attr: (ArcStr, Arc<Binding::NFBinding>)) -> bool {
    let mut isNamed: bool = false;
    let mut attr_name: ArcStr = arcstr::literal!("");
    (attr_name, _) = attr.clone();
    isNamed = name.clone() == attr_name.clone();
    isNamed
}

fn verifyBinding(mut var: Arc<Variable::NFVariable>, mut variability: Variability, mut binding: Arc<Binding::NFBinding>, mut settings: FlattenSettings) -> Result<Arc<Variable::NFVariable>> {
    fn eval_binding(mut binding: Arc<Binding::NFBinding>) -> Result<Option<Arc<Expression::NFExpression>>> {
        let mut result: Option<Arc<Expression::NFExpression>> = None;
        if Binding::isBound(binding.clone()) {
            result = Some(Ceval::tryEvalExp(Binding::getExp(binding.clone())?, Ceval::noTarget().clone()));
        } else {
            result = None;
        }
        Ok(result)
    }

    let mut var: Arc<Variable::NFVariable> = var;
    let mut fixed_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut start_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut fixed_exp_opt: Option<Arc<Expression::NFExpression>> = None;
    let mut fixed_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut start_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fixed: bool = false;
    let mut min_exp_opt: Option<Arc<Expression::NFExpression>> = None;
    let mut max_exp_opt: Option<Arc<Expression::NFExpression>> = None;
    if variability.clone() > Variability::CONSTANT.clone() && Binding::isBound(binding.clone()) {
        return Ok(var);
    }
    fixed_binding = Variable::lookupTypeAttribute((literal!("fixed")).clone(), var.clone());
    fixed_exp_opt = eval_binding(fixed_binding.clone())?;
    if isSome(fixed_exp_opt.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(fixed_exp_opt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        fixed_exp = __pa0.clone();
        if !(Expression::isBoolean(fixed_exp.clone())) {
            return Ok(var);
        }
        fixed = Expression::isTrue(fixed_exp.clone());
    } else {
        fixed = true;
    }
    if variability.clone() == Variability::CONSTANT.clone() {
        if !(fixed.clone()) {
            Error::addSourceMessage(Error::NON_FIXED_CONSTANT.clone(), list![(ComponentRef::toString(var.name.clone())?).clone()], var.info.clone())?;
            if !(settings.relaxedErrorChecking.clone()) {
                bail!("fail");
            }
        }
        if Binding::isUnbound(binding.clone()) {
            Error::addSourceMessage(Error::NO_CONSTANT_BINDING.clone(), list![(ComponentRef::toString(var.name.clone())?).clone()], var.info.clone())?;
            bail!("fail");
        }
    } else {
        if fixed.clone() && Binding::isUnbound(binding.clone()) {
            start_binding = Variable::lookupTypeAttribute((literal!("start")).clone(), var.clone());
            if Binding::isUnbound(start_binding.clone()) {
                Error::addSourceMessage(Error::UNBOUND_PARAMETER_ERROR.clone(), list![(ComponentRef::toString(var.name.clone())?).clone()], var.info.clone())?;
                if settings.implicitStartAttribute.clone() {
                    min_exp_opt = eval_binding(Variable::lookupTypeAttribute((literal!("min")).clone(), var.clone()))?;
                    max_exp_opt = eval_binding(Variable::lookupTypeAttribute((literal!("max")).clone(), var.clone()))?;
                    start_exp = Expression::makeDefaultValue(var.ty.clone(), min_exp_opt.clone(), max_exp_opt.clone())?;
                    assign_field!(var.binding = Binding::makeFlat(start_exp.clone(), Expression::variability(start_exp.clone())?, Binding::Source::GENERATED.clone()));
                } else if !(settings.relaxedErrorChecking.clone()) {
                    bail!("fail");
                }
            } else {
                Error::addSourceMessage(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone(), list![(ComponentRef::toString(var.name.clone())?).clone(), (Binding::toString(start_binding.clone(), (literal!("")).clone())?).clone()], var.info.clone())?;
            }
        }
    }
    Ok(var)
}

fn getRecordBindings(mut binding: Arc<Binding::NFBinding>, mut comps: metamodelica::Array<Arc<InstNode::InstNode>>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<metamodelica::List<Arc<Binding::NFBinding>>>> {
    let mut recordBindings: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = metamodelica::nil();
    let mut binding_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Variability = Variability::CONSTANT;
    let mut bind_src: Binding::Source = Binding::Source::BINDING;
    binding_exp = Binding::getTypedExp(binding.clone())?;
    var = Binding::variability(binding.clone())?;
    bind_src = Binding::Source::GENERATED.clone();
    recordBindings = (::match_deref::match_deref! { match &(binding_exp.clone()) {
        Deref @ Expression::RECORD { .. } => {
        let mut __acc: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = metamodelica::nil();
        for mut e in (var_field!((*binding_exp).elements, Expression::NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = if (Expression::isEmpty(e.clone())) {Binding::EMPTY_BINDING().clone()} else {Binding::makeFlat(e.clone(), var.clone(), bind_src.clone())};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        Deref @ Expression::ARRAY { .. } if (Type::isRecord(Type::arrayElementType(Expression::typeOf(binding_exp.clone())))) => {
        let mut __acc: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = metamodelica::nil();
        for mut i in (1..=(comps.clone().borrow().len() as i32)).into_iter() {
            let __x = Binding::makeFlat(Expression::nthRecordElement(i.clone(), binding_exp.clone())?, var.clone(), bind_src.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFlatten.getRecordBindings")); __mm_s.push_str(&*literal!(" got non-record binding ")); __mm_s.push_str(&*Expression::toString(binding_exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Error::assertion((recordBindings.clone().len() as i32) == (comps.clone().borrow().len() as i32), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFlatten.getRecordBindings")); __mm_s.push_str(&*literal!(" got record binding with wrong number of elements for ")); __mm_s.push_str(&*Prefix::toString(prefix.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    Ok(recordBindings)
}

fn flattenComplexComponent(mut node: Arc<InstNode::InstNode>, mut comp: Arc<Component::NFComponent>, mut cls: Arc<Class::NFClass>, mut nodeTy: Arc<Type::NFType>, mut visibility: Visibility, mut outerBinding: Option<Arc<Binding::NFBinding>>, mut prefix: Arc<Prefix::Prefix>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut sections: Arc<Sections::NFSections>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<Sections::NFSections>)> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut sections: Arc<Sections::NFSections> = sections;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut opt_binding: Option<Arc<Binding::NFBinding>> = None;
    let mut binding_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut binding_exp_eval: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eq: Arc<Equation::NFEquation>;
    let mut comp_var: Variability = Variability::CONSTANT;
    let mut binding_var: Variability = Variability::CONSTANT;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut pre: Arc<Prefix::Prefix>;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = InstNode::info(node.clone())?;
    ty = flattenType(nodeTy.clone(), prefix.clone(), info.clone());
    dims = Type::arrayDims(ty.clone());
    binding = if (isSome(outerBinding.clone())) {Util::getOption(outerBinding.clone())?} else {Component::getBinding(comp.clone())};
    if Binding::isExplicitlyBound(binding.clone()) {
        binding = flattenBinding(binding.clone(), prefix.clone(), false)?;
        binding_exp = Binding::getTypedExp(binding.clone())?;
        binding_var = Binding::variability(binding.clone())?;
        comp_var = Component::variability(comp.clone());
        if comp_var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() || binding_var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
            binding_exp = Ceval::evalExp(binding_exp.clone(), Ceval::EvalTarget::new(info.clone(), NFInstContext::BINDING.clone(), None))?;
            binding_exp = flattenExp(binding_exp.clone(), prefix.clone(), Binding::getInfo(binding.clone()))?;
        } else if binding_var.clone() == Variability::PARAMETER.clone() && Component::isFinal(comp.clone())? {
            if '__try0: {
                binding_exp = unwrap_break_err!(Inline::inlineCallExp(binding_exp.clone(), true), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
            if !(Expression::isRecord(binding_exp.clone()) || Expression::isCref(binding_exp.clone())) {
                if '__try1: {
                    binding_exp_eval = Ceval::tryEvalExp(binding_exp.clone(), Ceval::noTarget().clone());
                    binding_exp_eval = unwrap_break_err!(flattenExp(binding_exp_eval.clone(), prefix.clone(), Binding::getInfo(binding.clone())), '__try1);
                    let 0 = (Type::dimensionDiff(ty.clone(), Expression::typeOf(binding_exp_eval.clone()))) else { break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                    binding_exp = binding_exp_eval.clone();
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
            }
        } else {
            binding_exp = SimplifyExp::simplify(binding_exp.clone(), false)?;
        }
        binding_exp = splitRecordCref(binding_exp.clone())?;
        if !(Expression::isRecordOrRecordArray(binding_exp.clone())) {
            if !(settings.newBackend.clone()) {
                name = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), Prefix::prefix(prefix.clone())?);
                eq = Equation::makeEquality(Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: name.clone() }), binding_exp.clone(), ty.clone(), ElementSource::createElementSource(info.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?, Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone());
                sections = Sections::prependEquation(eq.clone(), sections.clone(), comp_var.clone() <= Variability::PARAMETER.clone())?;
            }
            opt_binding = Some(Binding::EMPTY_BINDING().clone());
        } else {
            binding = Binding::setTypedExp(binding_exp.clone(), binding.clone())?;
            opt_binding = Some(binding.clone());
        }
    } else {
        opt_binding = None;
    }
    pre = Prefix::push(node.clone(), ty.clone(), dims.clone(), prefix.clone())?;
    if dims.clone().is_empty() {
        (vars, sections) = flattenClass(cls.clone(), pre.clone(), visibility.clone(), opt_binding.clone(), vars.clone(), sections.clone(), deletedVars.clone(), settings.clone())?;
    } else if settings.scalarize.clone() {
        dims = {
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = flattenDimension(d.clone(), pre.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        verifyDimensions(dims.clone(), node.clone())?;
        (vars, sections) = flattenArray(cls.clone(), dims.clone(), pre.clone(), visibility.clone(), opt_binding.clone(), vars.clone(), sections.clone(), metamodelica::nil(), deletedVars.clone(), info.clone(), settings.clone())?;
    } else {
        (vars, sections) = vectorizeArray(cls.clone(), ty.clone(), dims.clone(), pre.clone(), visibility.clone(), opt_binding.clone(), vars.clone(), sections.clone(), metamodelica::nil(), deletedVars.clone(), settings.clone())?;
    }
    Ok((vars, sections))
}

fn splitRecordCref(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut field_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut fields: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (outExp, _) = ExpandExp::expand(exp.clone(), false, false)?;
    outExp = (::match_deref::match_deref! { match &(outExp.clone()) {
        Deref @ Expression::CREF { cref: cr, ty: Deref @ Type::COMPLEX { cls, .. } } => {
            comps = ClassTree::getComponents(Class::classTree(InstNode::getClass(cls.clone())?)?)?;
            fields = metamodelica::nil();
            let __range0 = (1..=(comps.clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                ty = InstNode::getType(comps.borrow()[(i.clone()-1) as usize].clone())?;
                field_cr = ComponentRef::prefixCref(comps.borrow()[(i.clone()-1) as usize].clone(), ty.clone(), metamodelica::nil(), cr.clone());
                field_cr = flattenCref(field_cr.clone(), Arc::new(Prefix::Prefix::PREFIX { root: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), prefix: cr.clone() }), Absyn::dummyInfo.clone())?;
                fields = cons(Expression::fromCref(field_cr.clone(), false)?, fields.clone());
            }
            Expression::makeRecord(InstNode::scopePath(cls.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, var_field!((*outExp).ty, Expression::NFExpression::CREF).clone(), fields.clone())
        },
        Deref @ Expression::ARRAY { .. } => {
            assign_variant_field!(outExp => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*outExp).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new(splitRecordCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)));
            outExp.clone()
        },
        Deref @ Expression::IF { .. } if (Expression::variability(var_field!((*outExp).condition, Expression::NFExpression::IF).clone())? <= Variability::PARAMETER.clone()) => {
            cond = Ceval::tryEvalExp(var_field!((*outExp).condition, Expression::NFExpression::IF).clone(), Ceval::noTarget().clone());
            if !(referenceEq(&cond.clone(),&var_field!((*outExp).condition, Expression::NFExpression::IF).clone())) {
                Structural::markExp(var_field!((*outExp).condition, Expression::NFExpression::IF).clone())?;
            }
            (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Expression::BOOLEAN { .. } => splitRecordCref(if (var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone()) {var_field!((*outExp).trueBranch, Expression::NFExpression::IF).clone()} else {var_field!((*outExp).falseBranch, Expression::NFExpression::IF).clone()})?,
        _ => outExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn flattenArray(mut cls: Arc<Class::NFClass>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut prefix: Arc<Prefix::Prefix>, mut visibility: Visibility, mut binding: Option<Arc<Binding::NFBinding>>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut sections: Arc<Sections::NFSections>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut deletedVars: DeletedVariables, mut info: SourceInfo, mut settings: FlattenSettings) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<Sections::NFSections>)> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut sections: Arc<Sections::NFSections> = sections;
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rest_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut sub_pre: Arc<Prefix::Prefix>;
    let mut range_iter: Arc<RangeIterator::NFRangeIterator>;
    let mut sub_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    if dimensions.clone().is_empty() {
        subs = subscripts.clone().reverse();
        sub_pre = Prefix::subscript(subs.clone(), prefix.clone())?;
        (vars, sections) = flattenClass(cls.clone(), sub_pre.clone(), visibility.clone(), subscriptBindingOpt(subs.clone(), binding.clone())?, vars.clone(), sections.clone(), deletedVars.clone(), settings.clone())?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dimensions.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim = __pa0.clone();
        rest_dims = __pa1.clone();
        dim = flattenDimension(dim.clone(), prefix.clone(), info.clone())?;
        range_iter = RangeIterator::fromDim(dim.clone(), false)?;
        while RangeIterator::hasNext(range_iter.clone())? {
            (range_iter, sub_exp) = RangeIterator::next(range_iter.clone())?;
            (vars, sections) = flattenArray(cls.clone(), rest_dims.clone(), prefix.clone(), visibility.clone(), binding.clone(), vars.clone(), sections.clone(), cons(Arc::new(Subscript::NFSubscript::INDEX { index: sub_exp.clone() }), subscripts.clone()), deletedVars.clone(), info.clone(), settings.clone())?;
        }
    }
    Ok((vars, sections))
}

fn vectorizeArray(mut cls: Arc<Class::NFClass>, mut cls_ty: Arc<Type::NFType>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut prefix: Arc<Prefix::Prefix>, mut visibility: Visibility, mut binding: Option<Arc<Binding::NFBinding>>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut sections: Arc<Sections::NFSections>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<Sections::NFSections>)> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut sections: Arc<Sections::NFSections> = sections;
    let mut vrs: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut sects: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut eq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ieq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut alg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let mut ialg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let mut indexed_prefix: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    if List::any(dimensions.clone(), (std::sync::Arc::new(Dimension::isZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<bool> + 'static>)) {
        return Ok((vars, sections));
    }
    (vrs, sects) = flattenClass(cls.clone(), prefix.clone(), visibility.clone(), binding.clone(), metamodelica::nil(), Arc::new(Sections::NFSections::SECTIONS { equations: metamodelica::nil(), initialEquations: metamodelica::nil(), algorithms: metamodelica::nil(), initialAlgorithms: metamodelica::nil() }), deletedVars.clone(), settings.clone())?;
    for mut v in &*vrs.clone().reverse() {
        let mut v = v.clone();
        if !(settings.newBackend.clone() && Type::isRecord(Type::arrayElementType(cls_ty.clone()))) {
            assign_field!(v.ty = Type::liftArrayLeftList(v.ty.clone(), dimensions.clone()));
        }
        vars = cons(v.clone(), vars.clone());
    }
    let () = (::match_deref::match_deref! { match &(sects.clone()) {
        Deref @ Sections::SECTIONS { .. } => {
            eq = vectorizeEquations(var_field!((*sects).equations, Sections::NFSections::SECTIONS).clone(), dimensions.clone(), prefix.clone(), settings.clone())?;
            ieq = vectorizeEquations(var_field!((*sects).initialEquations, Sections::NFSections::SECTIONS).clone(), dimensions.clone(), prefix.clone(), settings.clone())?;
            alg = vectorizeAlgorithms(var_field!((*sects).algorithms, Sections::NFSections::SECTIONS).clone(), dimensions.clone(), prefix.clone())?;
            ialg = vectorizeAlgorithms(var_field!((*sects).initialAlgorithms, Sections::NFSections::SECTIONS).clone(), dimensions.clone(), prefix.clone())?;
            sections = Sections::prepend(eq.clone(), ieq.clone(), alg.clone(), ialg.clone(), sections.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((vars, sections))
}

fn makeBindingIterators(mut prefix: Arc<ComponentRef::NFComponentRef>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>> {
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut iter: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut name: ArcStr = arcstr::literal!("");
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*InstNode::name(ComponentRef::node(prefix.clone())?)?); ArcStr::from(__mm_s) }).clone();
    for mut d in &*dimensions.clone() {
        let mut d = d.clone();
        index = index.clone() + 1;
        iter = ComponentRef::makeIterator(InstNode::newIterator(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); ArcStr::from(__mm_s) }).clone(), Arc::new(crate::NFType::INTEGER), Absyn::dummyInfo.clone()), InstNode::getType(InstNode::newIterator(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); ArcStr::from(__mm_s) }).clone(), Arc::new(crate::NFType::INTEGER), Absyn::dummyInfo.clone()))?);
        subs = cons(Subscript::makeIndex(Expression::fromCref(iter.clone(), false)?)?, subs.clone());
    }
    subs = subs.clone().reverse();
    Ok(subs)
}

fn vectorizeBinding(mut binding: Arc<Binding::NFBinding>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut array_call: Arc<Call::NFCall>;
    let mut binding_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut prefix_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    if !(Binding::isBound(binding.clone())) {
        return Ok(binding);
    }
    prefix_cr = Prefix::indexedPrefix(prefix.clone())?;
    subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (ComponentRef::subscriptsAllFlat(prefix_cr.clone())).into_iter().cloned() {
            if !(Subscript::isIterator(s.clone())) { continue; }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if subs.clone().is_empty() {
        return Ok(binding);
    }
    exp = Binding::getExp(binding.clone())?;
    binding_ty = Binding::getType(binding.clone())?;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { .. } if (Subscript::isEqualList(var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), subs.clone())?) => {
            binding = Binding::makeFlat(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), Binding::variability(binding.clone())?, Binding::source(binding.clone()));
            return Ok(binding);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    nodes = ComponentRef::nodes(prefix_cr.clone(), metamodelica::nil());
    dims = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>>> = metamodelica::nil();
        for mut n in (nodes.clone()).into_iter().cloned() {
            let __x = Type::arrayDims(InstNode::getType(n.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    dims = List::lastN(dims.clone(), (subs.clone().len() as i32))?;
    binding_ty = Type::liftArrayLeftList(binding_ty.clone(), dims.clone());
    if !(dims.clone().is_empty()) {
        if Expression::isLiteral(exp.clone()) || !(Expression::contains(exp.clone(), (std::sync::Arc::new(fnptr!(Expression::isIterator, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) {
            array_call = Call::makeTypedCall(NFBuiltinFuncs::FILL_FUNC().clone(), cons(exp.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = Dimension::sizeExp(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), Binding::variability(binding.clone())?, Purity::PURE.clone(), binding_ty.clone());
        } else {
            iters = {
        let mut __acc: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        for (s, d) in (&(subs.clone())).into_iter().zip((&(dims.clone())).into_iter()) {
            let __x = (Subscript::toIterator(s.clone())?, Dimension::toRange(d.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc
    };
            array_call = Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: binding_ty.clone(), var: Expression::variability(exp.clone())?, purity: Expression::purity(exp.clone())?, exp: exp.clone(), iters: iters.clone() });
        }
        exp = Arc::new(Expression::NFExpression::CALL { call: array_call.clone() });
    }
    binding = Binding::makeFlat(exp.clone(), Binding::variability(binding.clone())?, Binding::source(binding.clone()));
    Ok(binding)
}

fn fillVectorizedBinding(mut binding: Arc<Binding::NFBinding>, mut varType: Arc<Type::NFType>) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let mut bind_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut bind_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dim_diff: i32 = 0;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim_expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { bindingExp: bind_exp, .. } => {
            bind_ty = (::match_deref::match_deref! { match &(bind_exp.clone()) {
        Deref @ Expression::CREF { .. } => ComponentRef::getSubscriptedType(var_field!((**bind_exp).cref, Expression::NFExpression::CREF).clone(), true)?,
        _ => Expression::typeOf(bind_exp.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            dim_diff = Type::dimensionDiff(varType.clone(), bind_ty.clone());
            if dim_diff.clone() > 0 {
                dim_expl = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut d in (List::firstN(Type::arrayDims(varType.clone()), dim_diff.clone())?).into_iter().cloned() {
            let __x = Dimension::sizeExp(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING;
                    bindingExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FILL_FUNC().clone(), cons(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), dim_expl.clone()), var_field!((*binding).variability, Binding::NFBinding::TYPED_BINDING).clone(), Purity::PURE.clone(), varType.clone()) }),
                    bindingType = Expression::typeOf(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone())
                );
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

fn vectorizeEquations(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut prefix: Arc<Prefix::Prefix>, mut settings: FlattenSettings) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        equations = vectorizeEquation(eq.clone(), dimensions.clone(), prefix.clone(), settings.clone(), equations.clone())?;
    }
    equations = equations.clone().reverse();
    Ok(equations)
}

fn vectorizeEquation(mut eqn: Arc<Equation::NFEquation>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut prefix: Arc<Prefix::Prefix>, mut settings: FlattenSettings, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    eql = flattenEquation(eqn.clone(), EMPTY_PREFIX().clone(), metamodelica::nil(), settings.clone())?;
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        equations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { rhs: rhs @ Deref @ Expression::CREF { .. }, lhs: lhs @ Deref @ Expression::CREF { .. }, .. } if (!(Flags::getConfigBool(Flags::NEW_BACKEND.clone())?) || List::all(ComponentRef::subscriptsAllWithWholeFlat(var_field!((**lhs).cref, Expression::NFExpression::CREF).clone()), (std::sync::Arc::new(fnptr!(Subscript::isSimple, Arc<Subscript::NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<bool> + 'static>)) && List::all(ComponentRef::subscriptsAllWithWholeFlat(var_field!((**rhs).cref, Expression::NFExpression::CREF).clone()), (std::sync::Arc::new(fnptr!(Subscript::isSimple, Arc<Subscript::NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<bool> + 'static>))) => {
            let mut rhs = (*rhs).clone();
            let mut lhs = (*lhs).clone();
            ty = Type::liftArrayLeftList(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone(), dimensions.clone());
            lhs = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: var_field!((*lhs).cref, Expression::NFExpression::CREF).clone() });
            rhs = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: var_field!((*rhs).cref, Expression::NFExpression::CREF).clone() });
            cons(Arc::new(Equation::NFEquation::EQUALITY { lhs: lhs.clone(), rhs: rhs.clone(), ty: ty.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone(), scalarizeMode: var_field!((*eq).scalarizeMode, Equation::NFEquation::EQUALITY).clone() }), equations.clone())
        },
        Deref @ Equation::NORETCALL { exp: lhs @ Deref @ Expression::CALL { .. }, .. } if (Call::isConnectionsOperator(var_field!((**lhs).call, Expression::NFExpression::CALL).clone())?) => cons(eq.clone(), equations.clone()),
        _ => {
            eq = vectorizeEquationGeneric(eq.clone(), dimensions.clone(), prefix.clone())?;
            splitForLoop(eq.clone(), EMPTY_PREFIX().clone(), equations.clone(), settings.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(equations)
}

fn vectorizeEquationGeneric(mut eqn: Arc<Equation::NFEquation>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<Equation::NFEquation>> {
    let mut vectorizedEqn: Arc<Equation::NFEquation>;
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut iters: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    (iters, ranges, subs) = makeIterators(Prefix::prefix(prefix.clone())?, dimensions.clone())?;
    subs = subs.clone().reverse();
    vectorizedEqn = Equation::mapExp(eqn.clone(), Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = subs.clone(); move |__pe_a0| addIterator(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    scope = Equation::scope(eqn.clone())?;
    src = Equation::source(eqn.clone())?;
    while !(iters.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iters.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        iter = __pa0.clone();
        iters = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ranges.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        range = __pa2.clone();
        ranges = __pa3.clone();
        vectorizedEqn = Arc::new(Equation::NFEquation::FOR { iterator: iter.clone(), range: Some(range.clone()), body: list![vectorizedEqn.clone()], scope: scope.clone(), source: src.clone() });
    }
    Ok(vectorizedEqn)
}

fn vectorizeAlgorithms(mut algs: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>> {
    let mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    for mut alg in &*algs.clone() {
        let mut alg = alg.clone();
        algorithms = cons(vectorizeAlgorithm(alg.clone(), dimensions.clone(), prefix.clone())?, algorithms.clone());
    }
    algorithms = algorithms.clone().reverse();
    Ok(algorithms)
}

fn vectorizeAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = flattenStatements(alg.statements.clone(), EMPTY_PREFIX().clone())?);
    alg = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Algorithm::ALGORITHM { statements: Deref @ metamodelica::List::Cons { head: Deref @ Statement::ASSIGNMENT { rhs: Deref @ Expression::CREF { .. }, lhs: Deref @ Expression::CREF { .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            alg.clone()
        },
        _ => {
            let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut iters: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
            let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            (iters, ranges, subs) = makeIterators(Prefix::prefix(prefix.clone())?, dimensions.clone())?;
            subs = subs.clone().reverse();
            body = Statement::mapExpList(alg.statements.clone(), Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = subs.clone(); move |__pe_a0| addIterator(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            while !(iters.clone().is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iters.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                iter = __pa0.clone();
                iters = __pa1.clone();
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ranges.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                range = __pa2.clone();
                ranges = __pa3.clone();
                body = list![Arc::new(Statement::NFStatement::FOR { iterator: iter.clone(), range: Some(range.clone()), body: body.clone(), forType: Arc::new(crate::NFStatement::ForType::NORMAL), source: alg.source.clone() })];
            }
            Arc::new(Algorithm::NFAlgorithm { statements: body.clone(), inputs: alg.inputs.clone(), outputs: alg.outputs.clone(), stmtDiffInfo: None, scope: alg.scope.clone(), source: alg.source.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(alg)
}

pub fn makeIterators(mut prefix: Arc<ComponentRef::NFComponentRef>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<(Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>)> {
    let mut iterators: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut iter_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut prefix_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    prefix_node = ComponentRef::node(prefix.clone())?;
    for mut dim in &*dimensions.clone() {
        let mut dim = dim.clone();
        iter = InstNode::newUniqueIterator(InstNode::info(prefix_node.clone())?, Arc::new(crate::NFType::INTEGER));
        iterators = cons(iter.clone(), iterators.clone());
        range = Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), None, Dimension::sizeExp(dim.clone())?)?;
        ranges = cons(range.clone(), ranges.clone());
        sub = Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::INTEGER), cref: ComponentRef::makeIterator(iter.clone(), Arc::new(crate::NFType::INTEGER)) }) });
        subscripts = cons(sub.clone(), subscripts.clone());
    }
    Ok((iterators, ranges, subscripts))
}

fn addIterator(mut exp: Arc<Expression::NFExpression>, mut prefix: Arc<Prefix::Prefix>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = subscripts.clone(); move |__pe_a0| addIterator_traverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    Ok(exp)
}

fn addIterator_traverse(mut exp: Arc<Expression::NFExpression>, mut prefix: Arc<Prefix::Prefix>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut r#ref: Arc<ComponentRef::NFComponentRef> = Prefix::prefix(prefix.clone())?;
    let mut restString: ArcStr = arcstr::literal!("");
    let mut prefixString: ArcStr = ComponentRef::toString(r#ref.clone())?;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { restCref, .. }, .. } => {
            restString = (ComponentRef::toString(restCref.clone())?).clone();
            if StringUtil::startsWith((restString.clone()).clone(), (prefixString.clone()).clone()) {
                assign_variant_field!(exp => Expression::NFExpression::CREF; cref = mergeIterator(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), r#ref.clone(), subscripts.clone())?);
            }
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn mergeIterator(mut cref: Arc<ComponentRef::NFComponentRef>, mut r#ref: Arc<ComponentRef::NFComponentRef>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } => {
            if ComponentRef::isEqual(cref.clone(), r#ref.clone())? {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; subscripts = listAppend(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone(), subscripts.clone()));
            } else {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = mergeIterator(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), r#ref.clone(), subscripts.clone())?);
            }
            cref.clone()
        },
        _ => cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

fn containsPrefix(mut exp: Arc<Expression::NFExpression>, mut prefix: Arc<Prefix::Prefix>) -> Result<bool> {
    let mut contains: bool = false;
    contains = Expression::fold(exp.clone(), Arc::new({ let __pe_b2 = prefix.clone(); move |__pe_a0, __pe_a1| containsPrefix_traverse(__pe_a0, __pe_a1, __pe_b2.clone()) }), false)?;
    Ok(contains)
}

fn containsPrefix_traverse(mut exp: Arc<Expression::NFExpression>, mut contains: bool, mut prefix: Arc<Prefix::Prefix>) -> Result<bool> {
    let mut contains: bool = contains;
    let mut restString: ArcStr = arcstr::literal!("");
    let mut prefixString: ArcStr = ComponentRef::toString(Prefix::prefix(prefix.clone())?)?;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { restCref, .. }, .. } => {
            restString = (ComponentRef::toString(restCref.clone())?).clone();
            if StringUtil::startsWith((restString.clone()).clone(), (prefixString.clone()).clone()) {
                contains = true;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(contains)
}

fn subscriptBindingOpt(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut binding: Option<Arc<Binding::NFBinding>>) -> Result<Option<Arc<Binding::NFBinding>>> {
    let mut binding: Option<Arc<Binding::NFBinding>> = binding;
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if isSome(binding.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(binding.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        b = __pa0.clone();
        binding = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Binding::TYPED_BINDING { bindingType: ty, bindingExp: exp, .. } => {
            assign_variant_field!(b => Binding::NFBinding::TYPED_BINDING;
                bindingExp = Expression::applySubscripts(subscripts.clone(), exp.clone(), false)?,
                bindingType = Type::arrayElementType(ty.clone())
            );
            Some(b.clone())
        },
        Deref @ Binding::FLAT_BINDING { bindingExp: exp, .. } => {
            assign_variant_field!(b => Binding::NFBinding::FLAT_BINDING; bindingExp = Expression::applySubscripts(subscripts.clone(), exp.clone(), false)?);
            Some(b.clone())
        },
        _ => binding.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(binding)
}

pub fn flattenBinding(mut binding: Arc<Binding::NFBinding>, mut prefix: Arc<Prefix::Prefix>, mut isTypeAttribute: bool) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut accum_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut binding_level: i32 = 0;
    let mut bind_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut pars: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut par: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    binding = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::UNBOUND => binding.clone(),
        Deref @ Binding::TYPED_BINDING { .. } => {
            if var_field!((*binding).isFlattened, Binding::NFBinding::TYPED_BINDING).clone() {
                return Ok(binding);
            }
            info = Binding::getInfo(binding.clone());
            assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING;
                bindingExp = flattenExp(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), prefix.clone(), info.clone())?,
                bindingType = flattenType(var_field!((*binding).bindingType, Binding::NFBinding::TYPED_BINDING).clone(), prefix.clone(), info.clone()),
                isFlattened = true
            );
            if (Prefix::isIndexed(prefix.clone())) {vectorizeBinding(binding.clone(), prefix.clone())?} else {binding.clone()}
        },
        Deref @ Binding::CEVAL_BINDING { .. } => Binding::EMPTY_BINDING().clone(),
        Deref @ Binding::FLAT_BINDING { .. } => binding.clone(),
        Deref @ Binding::INVALID_BINDING { .. } => {
            Error::addTotalMessages(var_field!((*binding).errors, Binding::NFBinding::INVALID_BINDING).clone())?;
            bail!("fail")
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFlatten.flattenBinding")); __mm_s.push_str(&*literal!(" got untyped binding.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

pub fn flattenExp(mut exp: Arc<Expression::NFExpression>, mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF;
                cref = ComponentRef::mapExpShallow(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| flattenExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) })),
                cref = flattenCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), prefix.clone(), info.clone())?,
                ty = flattenType(var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), prefix.clone(), info.clone())
            );
            exp.clone()
        },
        Deref @ Expression::SUBSCRIPTED_EXP { split: true, .. } => Expression::mapShallow(replaceSplitIndices(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), prefix.clone(), info.clone())?, Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| flattenExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?,
        Deref @ Expression::IF { ty: Deref @ Type::CONDITIONAL_ARRAY { .. }, .. } => flattenConditionalArrayIfExp(exp.clone(), prefix.clone(), info.clone())?,
        Deref @ Expression::INSTANCE_NAME { .. } => Arc::new(Expression::NFExpression::STRING { value: (Prefix::instanceName(prefix.clone())?).clone() }),
        _ => Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| flattenExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp = flattenExpType(exp.clone(), prefix.clone(), info.clone())?;
    Ok(exp)
}

pub fn replaceSplitIndices(mut exp: Arc<Expression::NFExpression>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = subscripts.clone();
    let mut cr_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut cr_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    for mut cr in &*ComponentRef::toListReverse(Prefix::indexedPrefix(prefix.clone())?, true, metamodelica::nil()) {
        let mut cr = cr.clone();
        cr_subs = ComponentRef::getSubscripts(cr.clone());
        if !(cr_subs.clone().is_empty()) {
            index = 1;
            cr_node = ComponentRef::node(cr.clone())?;
            for mut s in &*cr_subs.clone() {
                let mut s = s.clone();
                (subs, _) = List::replaceOnTrue(s.clone(), subs.clone(), Arc::new({ let __pe_b1 = cr_node.clone(); let __pe_b2 = index.clone(); move |__pe_a0| Ok(replaceSplitIndices2(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }))?;
                index = index.clone() + 1;
            }
        }
    }
    subs = Subscript::expandSplitIndices(subs.clone(), metamodelica::nil())?;
    exp = Expression::applySubscripts(subs.clone(), exp.clone(), false)?;
    exp = flattenExp(exp.clone(), prefix.clone(), info.clone())?;
    Ok(exp)
}

pub fn replaceSplitIndices2(mut sub: Arc<Subscript::NFSubscript>, mut node: Arc<InstNode::InstNode>, mut index: i32) -> bool {
    let mut replace: bool = false;
    replace = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => var_field!((*sub).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone() == index.clone() && InstNode::refEqual(var_field!((*sub).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), node.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    replace
}

pub fn flattenCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    cref = Prefix::apply(prefix.clone(), cref.clone())?;
    if ComponentRef::hasSplitSubscripts(cref.clone()) {
        cref = flattenCrefSplitSubscripts(cref.clone(), prefix.clone())?;
    }
    cref = ComponentRef::mapTypes(cref.clone(), Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| Ok(flattenType(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }));
    Ok(cref)
}

pub fn flattenCrefSplitSubscripts(mut cref: Arc<ComponentRef::NFComponentRef>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut sub_map: Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>;
    sub_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 1);
    for mut cr in &*ComponentRef::toListReverse(Prefix::indexedPrefix(prefix.clone())?, true, metamodelica::nil()) {
        let mut cr = cr.clone();
        if ComponentRef::hasSubscripts(cr.clone()) {
            UnorderedMap::addUnique(ComponentRef::node(cr.clone())?, ComponentRef::getSubscripts(cr.clone()), sub_map.clone())?;
        }
    }
    cref = ComponentRef::mapSubscripts(cref.clone(), Arc::new({ let __pe_b1 = sub_map.clone(); move |__pe_a0| Ok(flattenCrefSplitSubscripts2(__pe_a0, __pe_b1.clone())) }), false);
    cref = ComponentRef::simplifySubscripts(cref.clone(), true)?;
    Ok(cref)
}

pub fn flattenCrefSplitSubscripts2(mut sub: Arc<Subscript::NFSubscript>, mut subMap: Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>) -> Arc<Subscript::NFSubscript> {
    let mut sub: Arc<Subscript::NFSubscript> = sub;
    sub = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            subs = UnorderedMap::getOrDefault(var_field!((*sub).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), subMap.clone(), metamodelica::nil());
            if (var_field!((*sub).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone() > (subs.clone().len() as i32)) {Arc::new(crate::NFSubscript::WHOLE)} else {(subs.clone()).get(var_field!((*sub).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone()).unwrap()}
        },
        _ => {
            sub.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sub
}

pub fn flattenConditionalArrayIfExp(mut exp: Arc<Expression::NFExpression>, mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cond_var: Variability = Variability::CONSTANT;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::IF { falseBranch: __pa0, trueBranch: __pa1, condition: __pa2, ty: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    fb = __pa0.clone();
    tb = __pa1.clone();
    cond = __pa2.clone();
    ty = __pa3.clone();
    cond = flattenExp(cond.clone(), prefix.clone(), info.clone())?;
    cond_var = Expression::variability(cond.clone())?;
    if Type::isConditionalArray(ty.clone()) {
        Structural::markExp(cond.clone())?;
        cond = Ceval::tryEvalExp(cond.clone(), Ceval::noTarget().clone());
        exp = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Expression::BOOLEAN { .. } => {
            if !(Type::isMatchedBranch(var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone(), ty.clone())?) {
                (tb, fb) = Util::swap(var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone(), fb.clone(), tb.clone());
                Error::addSourceMessage(Error::ARRAY_DIMENSION_MISMATCH.clone(), list![(Expression::toString(tb.clone())?).clone(), (Type::toString(Expression::typeOf(tb.clone()))?).clone(), (Dimension::toStringList(Type::arrayDims(Expression::typeOf(fb.clone())), false)?).clone()], info.clone())?;
                bail!("fail");
            }
            flattenExp(if (var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone()) {tb.clone()} else {fb.clone()}, prefix.clone(), info.clone())?
        },
        _ => {
            Error::addSourceMessage(Error::TYPE_MISMATCH_IF_EXP.clone(), list![(literal!("")).clone(), (Expression::toString(tb.clone())?).clone(), (Type::toString(Expression::typeOf(tb.clone()))?).clone(), (Expression::toString(fb.clone())?).clone(), (Type::toString(Expression::typeOf(fb.clone()))?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else if Expression::variability(cond.clone())? == Variability::PARAMETER.clone() {
        Structural::markExp(cond.clone())?;
        tb = flattenExp(tb.clone(), prefix.clone(), info.clone())?;
        fb = flattenExp(fb.clone(), prefix.clone(), info.clone())?;
        ty = flattenType(ty.clone(), prefix.clone(), info.clone());
        exp = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: tb.clone(), falseBranch: fb.clone() });
    }
    Ok(exp)
}

pub fn flattenExpType(mut exp: Arc<Expression::NFExpression>, mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = Expression::typeOf(exp.clone());
    if Type::isArray(ty.clone()) {
        ty = flattenType(ty.clone(), prefix.clone(), info.clone());
        exp = Expression::setType(ty.clone(), exp.clone())?;
    }
    Ok(exp)
}

pub fn flattenType(mut ty: Arc<Type::NFType>, mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = ty;
    ty = Type::mapDims(ty.clone(), Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| flattenDimension(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    ty
}

pub fn flattenDimension(mut dim: Arc<Dimension::NFDimension>, mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = dim;
    dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { .. } => Dimension::fromExp(flattenExp(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone(), prefix.clone(), info.clone())?, var_field!((*dim).var, Dimension::NFDimension::EXP).clone())?,
        _ => dim.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub fn flattenSections(mut sections: Arc<Sections::NFSections>, mut prefix: Arc<Prefix::Prefix>, mut accumSections: Arc<Sections::NFSections>, mut settings: FlattenSettings) -> Result<Arc<Sections::NFSections>> {
    let mut accumSections: Arc<Sections::NFSections> = accumSections;
    let () = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { .. } => {
            let mut eq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut ieq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut alg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
            let mut ialg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
            eq = flattenEquations(var_field!((*sections).equations, Sections::NFSections::SECTIONS).clone(), prefix.clone(), settings.clone())?;
            ieq = flattenEquations(var_field!((*sections).initialEquations, Sections::NFSections::SECTIONS).clone(), prefix.clone(), settings.clone())?;
            alg = flattenAlgorithms(var_field!((*sections).algorithms, Sections::NFSections::SECTIONS).clone(), prefix.clone())?;
            ialg = flattenAlgorithms(var_field!((*sections).initialAlgorithms, Sections::NFSections::SECTIONS).clone(), prefix.clone())?;
            accumSections = Sections::prepend(eq.clone(), ieq.clone(), alg.clone(), ialg.clone(), accumSections.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(accumSections)
}

pub fn flattenEquations(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut prefix: Arc<Prefix::Prefix>, mut settings: FlattenSettings) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        equations = flattenEquation(eq.clone(), prefix.clone(), equations.clone(), settings.clone())?;
    }
    Ok(equations)
}

pub fn flattenEquation(mut eq: Arc<Equation::NFEquation>, mut prefix: Arc<Prefix::Prefix>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut settings: FlattenSettings) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut info: SourceInfo = Equation::info(eq.clone());
    equations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            e1 = flattenExp(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), prefix.clone(), info.clone())?;
            e2 = flattenExp(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), prefix.clone(), info.clone())?;
            ty = flattenType(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone(), prefix.clone(), info.clone());
            cons(Arc::new(Equation::NFEquation::EQUALITY { lhs: e1.clone(), rhs: e2.clone(), ty: ty.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone(), scalarizeMode: var_field!((*eq).scalarizeMode, Equation::NFEquation::EQUALITY).clone() }), equations.clone())
        },
        Deref @ Equation::FOR { .. } => {
            let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            if settings.scalarize.clone() {
                eql = unrollForLoop(eq.clone(), prefix.clone(), equations.clone(), settings.clone())?;
            } else {
                eql = splitForLoop(eq.clone(), prefix.clone(), equations.clone(), settings.clone())?;
            }
            eql.clone()
        },
        Deref @ Equation::CONNECT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*eq).lhs, Equation::NFEquation::CONNECT).clone(), prefix.clone(), info.clone())?;
            e2 = flattenExp(var_field!((*eq).rhs, Equation::NFEquation::CONNECT).clone(), prefix.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::CONNECT { lhs: e1.clone(), rhs: e2.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::CONNECT).clone(), source: var_field!((*eq).source, Equation::NFEquation::CONNECT).clone() }), equations.clone())
        },
        Deref @ Equation::IF { .. } => {
            flattenIfEquation(eq.clone(), prefix.clone(), equations.clone(), settings.clone())?
        },
        Deref @ Equation::WHEN { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::WHEN; branches = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, Equation::NFEquation::WHEN).clone()).into_iter().cloned() {
            let __x = flattenEqBranch(b.clone(), prefix.clone(), info.clone(), settings.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            cons(eq.clone(), equations.clone())
        },
        Deref @ Equation::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), prefix.clone(), info.clone())?;
            e2 = flattenExp(var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), prefix.clone(), info.clone())?;
            e3 = flattenExp(var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), prefix.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::ASSERT).clone(), source: var_field!((*eq).source, Equation::NFEquation::ASSERT).clone() }), equations.clone())
        },
        Deref @ Equation::TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), prefix.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::TERMINATE { message: e1.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::TERMINATE).clone(), source: var_field!((*eq).source, Equation::NFEquation::TERMINATE).clone() }), equations.clone())
        },
        Deref @ Equation::REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*eq).cref, Equation::NFEquation::REINIT).clone(), prefix.clone(), info.clone())?;
            e2 = flattenExp(var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), prefix.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::REINIT { cref: e1.clone(), reinitExp: e2.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::REINIT).clone(), source: var_field!((*eq).source, Equation::NFEquation::REINIT).clone() }), equations.clone())
        },
        Deref @ Equation::NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), prefix.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::NORETCALL { exp: e1.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::NORETCALL).clone(), source: var_field!((*eq).source, Equation::NFEquation::NORETCALL).clone() }), equations.clone())
        },
        _ => {
            cons(eq.clone(), equations.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub fn flattenIfEquation(mut eq: Arc<Equation::NFEquation>, mut prefix: Arc<Prefix::Prefix>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut settings: FlattenSettings) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut branch: Arc<Equation::Branch::Branch>;
    let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut bl: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let mut has_connect: bool = false;
    let mut should_eval: bool = false;
    let mut structural: bool = true;
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut target: Arc<Ceval::EvalTarget::EvalTarget> = Arc::new(<Ceval::EvalTarget::EvalTarget as ::std::default::Default>::default());
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::IF { source: __pa0, scope: __pa1, branches: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    src = __pa0.clone();
    scope = __pa1.clone();
    branches = __pa2.clone();
    has_connect = Equation::contains(eq.clone(), (std::sync::Arc::new(Equation::isConnection) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>));
    info = Equation::info(eq.clone());
    target = if (has_connect.clone()) {Ceval::EvalTarget::new(info.clone(), NFInstContext::NO_CONTEXT.clone(), None)} else {Ceval::noTarget().clone()};
    while !(branches.clone().is_empty()) {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(branches.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        branch = __pa3.clone();
        branches = __pa4.clone();
        bl = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { condition: cond, conditionVar: var, body: eql } => {
            let mut cond = (*cond).clone();
            let mut eql = (*eql).clone();
            cond = flattenExp(cond.clone(), prefix.clone(), info.clone())?;
            if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
                if Expression::isPure(cond.clone())? {
                    if has_connect.clone() {
                        should_eval = !(settings.newBackend.clone());
                        structural = true;
                    } else if settings.minimalEval.clone() {
                        should_eval = false;
                        structural = false;
                    } else if settings.scalarize.clone() {
                        should_eval = true;
                    } else if settings.newBackend.clone() || Expression::contains(cond.clone(), (std::sync::Arc::new(fnptr!(Expression::isIterator, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))? {
                        should_eval = false;
                        structural = settings.newBackend.clone();
                    } else {
                        should_eval = true;
                    }
                    if structural.clone() || should_eval.clone() {
                        Structural::markExp(cond.clone())?;
                    }
                    if should_eval.clone() {
                        cond = Ceval::tryEvalExp(cond.clone(), target.clone());
                        cond = flattenExp(cond.clone(), prefix.clone(), info.clone())?;
                    }
                }
                if !(Expression::isBoolean(cond.clone())) && has_connect.clone() && !(settings.newBackend.clone()) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to evaluate branch condition in if equation containing connect equations: `")); __mm_s.push_str(&*Expression::toString(cond.clone())?); __mm_s.push_str(&*literal!("`")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
                    bail!("fail");
                }
            }
            if Expression::isTrue(cond.clone()) {
                branches = metamodelica::nil();
                eql = flattenEquations(eql.clone(), prefix.clone(), settings.clone())?;
                if bl.clone().is_empty() {
                    equations = listAppend(eql.clone(), equations.clone());
                } else {
                    bl = cons(Equation::makeBranch(cond.clone(), eql.clone().reverse(), var.clone()), bl.clone());
                }
            } else if !(Expression::isFalse(cond.clone())) {
                eql = flattenEquations(eql.clone(), prefix.clone(), settings.clone())?;
                bl = cons(Equation::makeBranch(cond.clone(), eql.clone().reverse(), var.clone()), bl.clone());
            }
            bl.clone()
        },
        Deref @ Equation::Branch::INVALID_BRANCH { branch: Deref @ Equation::Branch::BRANCH { conditionVar: var, condition: cond, .. }, .. } if (has_connect.clone()) => {
            let mut cond = (*cond).clone();
            if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
                Structural::markExp(cond.clone())?;
                cond = Ceval::evalExp(cond.clone(), target.clone())?;
                cond = flattenExp(cond.clone(), prefix.clone(), info.clone())?;
            }
            if !(Expression::isFalse(cond.clone())) {
                Equation::Branch::triggerErrors(branch.clone())?;
            }
            bl.clone()
        },
        _ => cons(branch.clone(), bl.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if !(bl.clone().is_empty()) {
        equations = cons(Arc::new(Equation::NFEquation::IF { branches: bl.clone().reverse(), scope: scope.clone(), source: src.clone() }), equations.clone());
    }
    Ok(equations)
}

pub fn flattenEqBranch(mut branch: Arc<Equation::Branch::Branch>, mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo, mut settings: FlattenSettings) -> Result<Arc<Equation::Branch::Branch>> {
    let mut branch: Arc<Equation::Branch::Branch> = branch;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: __pa1, body: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    var = __pa1.clone();
    eql = __pa2.clone();
    exp = flattenExp(exp.clone(), prefix.clone(), info.clone())?;
    eql = flattenEquations(eql.clone(), prefix.clone(), settings.clone())?;
    branch = Equation::makeBranch(exp.clone(), eql.clone().reverse(), var.clone());
    Ok(branch)
}

pub fn unrollForLoop(mut forLoop: Arc<Equation::NFEquation>, mut prefix: Arc<Prefix::Prefix>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut settings: FlattenSettings) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut unrolled_body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut range_iter: Arc<RangeIterator::NFRangeIterator>;
    let mut val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(forLoop.clone()) {
        Deref @ Equation::FOR { body: __pa0, range: Some(__pa1), iterator: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    body = __pa0.clone();
    range = __pa1.clone();
    iter = __pa2.clone();
    info = Equation::info(forLoop.clone());
    range = flattenExp(range.clone(), prefix.clone(), info.clone())?;
    Structural::markExp(range.clone())?;
    range = Ceval::evalExp(range.clone(), Ceval::EvalTarget::new(info.clone(), NFInstContext::ITERATION_RANGE.clone(), None))?;
    range_iter = RangeIterator::fromExp(range.clone())?;
    while RangeIterator::hasNext(range_iter.clone())? {
        (range_iter, val) = RangeIterator::next(range_iter.clone())?;
        unrolled_body = Equation::replaceIteratorList(body.clone(), iter.clone(), val.clone());
        unrolled_body = flattenEquations(unrolled_body.clone(), prefix.clone(), settings.clone())?;
        equations = listAppend(unrolled_body.clone(), equations.clone());
    }
    Ok(equations)
}

pub fn splitForLoop(mut forLoop: Arc<Equation::NFEquation>, mut prefix: Arc<Prefix::Prefix>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut settings: FlattenSettings) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut opt_range: Option<Arc<Expression::NFExpression>> = None;
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut connects: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut non_connects: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut eq: Arc<Equation::NFEquation>;
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(forLoop.clone()) {
        Deref @ Equation::FOR { iterator: __pa0, range: __pa1, body: __pa2, scope: __pa3, source: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    iter = __pa0.clone();
    opt_range = __pa1.clone();
    body = __pa2.clone();
    scope = __pa3.clone();
    src = __pa4.clone();
    body = flattenEquations(body.clone(), EMPTY_PREFIX().clone(), settings.clone())?;
    (connects, non_connects) = splitForLoop2(body.clone(), settings.clone())?;
    if !(connects.clone().is_empty()) {
        if isSome(opt_range.clone()) {
            let __pa5 = ::match_deref::match_deref! { match &(opt_range.clone()) {
                Some(__pa5) => __pa5.clone(),
                _ => bail!("pattern mismatch"),
            } };
            range = __pa5.clone();
            range = Ceval::evalExp(range.clone(), Ceval::EvalTarget::new(Equation::info(forLoop.clone()), NFInstContext::ITERATION_RANGE.clone(), None))?;
            Structural::markExp(range.clone())?;
            opt_range = Some(range.clone());
        }
        eq = Arc::new(Equation::NFEquation::FOR { iterator: iter.clone(), range: opt_range.clone(), body: connects.clone(), scope: scope.clone(), source: src.clone() });
        if settings.arrayConnect.clone() {
            equations = cons(eq.clone(), equations.clone());
        } else {
            equations = unrollForLoop(eq.clone(), prefix.clone(), equations.clone(), settings.clone())?;
        }
    }
    if !(non_connects.clone().is_empty()) {
        equations = cons(Arc::new(Equation::NFEquation::FOR { iterator: iter.clone(), range: opt_range.clone(), body: non_connects.clone(), scope: scope.clone(), source: src.clone() }), equations.clone());
    }
    Ok(equations)
}

pub fn splitForLoop2(mut forBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut settings: FlattenSettings) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Equation::NFEquation>>>)> {
    let mut connects: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut nonConnects: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut conns: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut nconns: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    for mut eq in &*forBody.clone() {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::CONNECT { .. } => {
            connects = cons(eq.clone(), connects.clone());
            ()
        },
        Deref @ Equation::FOR { .. } => {
            (conns, nconns) = splitForLoop2(var_field!((*eq).body, Equation::NFEquation::FOR).clone(), settings.clone())?;
            if !(conns.clone().is_empty()) {
                connects = cons(Arc::new(Equation::NFEquation::FOR { iterator: var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), range: var_field!((*eq).range, Equation::NFEquation::FOR).clone(), body: conns.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::FOR).clone(), source: var_field!((*eq).source, Equation::NFEquation::FOR).clone() }), connects.clone());
            }
            if !(nconns.clone().is_empty()) {
                nonConnects = cons(Arc::new(Equation::NFEquation::FOR { iterator: var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), range: var_field!((*eq).range, Equation::NFEquation::FOR).clone(), body: nconns.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::FOR).clone(), source: var_field!((*eq).source, Equation::NFEquation::FOR).clone() }), nonConnects.clone());
            }
            ()
        },
        _ => {
            if Equation::contains(eq.clone(), (std::sync::Arc::new(fnptr!(Equation::isConnect, Arc<Equation::NFEquation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>)) || Equation::containsExp(eq.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static> = (std::sync::Arc::new(Expression::isConnectionCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>); move |__pe_a0| Expression::contains(__pe_a0, __pe_b1.clone()) }))? {
                connects = cons(eq.clone(), connects.clone());
            } else {
                nonConnects = cons(eq.clone(), nonConnects.clone());
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((connects, nonConnects))
}

pub fn unrollForStatementsInAlg(mut alg: Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = unrollForStatements(alg.statements.clone())?);
    Ok(alg)
}

pub fn unrollForStatements(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut outStmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    for mut s in &*stmts.clone() {
        let mut s = s.clone();
        outStmts = unrollForStatement(s.clone(), outStmts.clone())?;
    }
    outStmts = outStmts.clone().reverse();
    Ok(outStmts)
}

pub fn unrollForStatement(mut stmt: Arc<Statement::NFStatement>, mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = statements;
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut range_iter: Arc<RangeIterator::NFRangeIterator>;
    let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut has_for: bool = false;
    statements = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::FOR { range: Some(range), .. } => {
            let mut range = (*range).clone();
            info = Statement::info(stmt.clone());
            match '__try0: {
                range = unwrap_break_err!(Ceval::evalExp(range.clone(), Ceval::EvalTarget::new(info.clone(), NFInstContext::ITERATION_RANGE.clone(), None)), '__try0);
                range_iter = unwrap_break_err!(RangeIterator::fromExp(range.clone()), '__try0);
                Ok::<_, anyhow::Error>((range.clone(), range_iter.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    range = __try0_o0;
                    range_iter = __try0_o1;
                }
                Err(_) => {
                    Error::addSourceMessage(Error::UNROLL_FAILURE.clone(), list![(Statement::toString(stmt.clone(), (literal!("")).clone())?).clone()], info.clone())?;
                    bail!("fail");
                }
            }
            has_for = Statement::containsList(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), (std::sync::Arc::new(fnptr!(Statement::isFor, Arc<Statement::NFStatement>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<bool> + 'static>));
            while RangeIterator::hasNext(range_iter.clone())? {
                (range_iter, val) = RangeIterator::next(range_iter.clone())?;
                stmts = Statement::replaceIteratorList(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), var_field!((*stmt).iterator, Statement::NFStatement::FOR).clone(), val.clone());
                if has_for.clone() {
                    stmts = unrollForStatements(stmts.clone())?;
                }
                statements = List::append_reverse(stmts.clone(), statements.clone());
            }
            statements.clone()
        },
        _ => cons(stmt.clone(), statements.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(statements)
}

pub fn flattenAlgorithms(mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>> {
    let mut outAlgorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    for mut alg in &*algorithms.clone() {
        let mut alg = alg.clone();
        assign_field!(alg.statements = flattenStatements(alg.statements.clone(), prefix.clone())?);
        if ComponentRef::hasSubscripts(Prefix::prefix(prefix.clone())?) {
            assign_field!(alg.source = addElementSourceArrayPrefix(alg.source.clone(), prefix.clone())?);
        }
        outAlgorithms = cons(alg.clone(), outAlgorithms.clone());
    }
    Ok(outAlgorithms)
}

pub fn flattenStatements(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = stmts;
    stmts = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (stmts.clone()).into_iter().cloned() {
            let __x = flattenStatement(s.clone(), prefix.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(stmts)
}

pub fn flattenStatement(mut stmt: Arc<Statement::NFStatement>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement> = stmt;
    let mut info: SourceInfo = Statement::info(stmt.clone());
    stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            e1 = flattenExp(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), prefix.clone(), info.clone())?;
            e2 = flattenExp(var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), prefix.clone(), info.clone())?;
            ty = flattenType(var_field!((*stmt).ty, Statement::NFStatement::ASSIGNMENT).clone(), prefix.clone(), info.clone());
            Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: e1.clone(), rhs: e2.clone(), ty: ty.clone(), source: var_field!((*stmt).source, Statement::NFStatement::ASSIGNMENT).clone() })
        },
        Deref @ Statement::FOR { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::FOR;
                range = Util::applyOption(var_field!((*stmt).range, Statement::NFStatement::FOR).clone(), Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| flattenExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) })),
                body = flattenStatements(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), prefix.clone())?,
                forType = updateForType(var_field!((*stmt).forType, Statement::NFStatement::FOR).clone(), var_field!((*stmt).body, Statement::NFStatement::FOR).clone())?
            );
            stmt.clone()
        },
        Deref @ Statement::IF { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::IF; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, Statement::NFStatement::IF).clone()).into_iter().cloned() {
            let __x = flattenStmtBranch(b.clone(), prefix.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            stmt.clone()
        },
        Deref @ Statement::WHEN { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHEN; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone()).into_iter().cloned() {
            let __x = flattenStmtBranch(b.clone(), prefix.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            stmt.clone()
        },
        Deref @ Statement::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), prefix.clone(), info.clone())?;
            e2 = flattenExp(var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), prefix.clone(), info.clone())?;
            e3 = flattenExp(var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), prefix.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), source: var_field!((*stmt).source, Statement::NFStatement::ASSERT).clone() })
        },
        Deref @ Statement::TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*stmt).message, Statement::NFStatement::TERMINATE).clone(), prefix.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::TERMINATE { message: e1.clone(), source: var_field!((*stmt).source, Statement::NFStatement::TERMINATE).clone() })
        },
        Deref @ Statement::REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*stmt).cref, Statement::NFStatement::REINIT).clone(), prefix.clone(), info.clone())?;
            e2 = flattenExp(var_field!((*stmt).reinitExp, Statement::NFStatement::REINIT).clone(), prefix.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::REINIT { cref: e1.clone(), reinitExp: e2.clone(), source: var_field!((*stmt).source, Statement::NFStatement::REINIT).clone() })
        },
        Deref @ Statement::NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = flattenExp(var_field!((*stmt).exp, Statement::NFStatement::NORETCALL).clone(), prefix.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::NORETCALL { exp: e1.clone(), source: var_field!((*stmt).source, Statement::NFStatement::NORETCALL).clone() })
        },
        Deref @ Statement::WHILE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            e1 = flattenExp(var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), prefix.clone(), info.clone())?;
            body = flattenStatements(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone(), prefix.clone())?;
            Arc::new(Statement::NFStatement::WHILE { condition: e1.clone(), body: body.clone(), source: var_field!((*stmt).source, Statement::NFStatement::WHILE).clone() })
        },
        Deref @ Statement::FAILURE { .. } => {
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            body = flattenStatements(var_field!((*stmt).body, Statement::NFStatement::FAILURE).clone(), prefix.clone())?;
            Arc::new(Statement::NFStatement::FAILURE { body: body.clone(), source: var_field!((*stmt).source, Statement::NFStatement::FAILURE).clone() })
        },
        _ => {
            stmt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

pub fn flattenStmtBranch(mut branch: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), mut prefix: Arc<Prefix::Prefix>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)> {
    let mut branch: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>) = branch;
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    (cond, body) = branch.clone();
    cond = flattenExp(cond.clone(), prefix.clone(), info.clone())?;
    body = flattenStatements(body.clone(), prefix.clone())?;
    branch = (cond.clone(), body.clone());
    Ok(branch)
}

pub fn addElementSourceArrayPrefix(mut source: Arc<DAE::ElementSource>, mut prefix: Arc<Prefix::Prefix>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    let mut comp_pre: Arc<DAE::ComponentPrefix> = Arc::new(DAE::ComponentPrefix::NOCOMPPRE);
    comp_pre = Arc::new(DAE::ComponentPrefix::PRE { prefix: (ComponentRef::firstName(Prefix::prefix(prefix.clone())?, false)?).clone(), dimensions: metamodelica::nil(), subscripts: list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: -1 }) })], next: Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE), ci_state: ClassInf::State::UNKNOWN { path: Arc::new(Path::IDENT { name: (literal!("?")).clone() }) }, info: Absyn::dummyInfo.clone() });
    source = ElementSource::addElementSourceInstanceOpt(source.clone(), comp_pre.clone())?;
    Ok(source)
}

pub fn isDeletedCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut deletedVars: DeletedVariables) -> Result<bool> {
    let mut res: bool = false;
    let mut cr: Arc<ComponentRef::NFComponentRef> = cref.clone();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    (cr, _) = ComponentRef::stripSubscripts(cref.clone());
    while ComponentRef::isCref(cr.clone()) {
        node = ComponentRef::node(cr.clone())?;
        if InstNode::isComponent(node.clone()) && Component::hasCondition(InstNode::component(node.clone())?) {
            if UnorderedSet::contains(cr.clone(), deletedVars.clone())? {
                res = true;
                return Ok(res);
            }
        }
        (cr, _) = ComponentRef::stripSubscripts(ComponentRef::rest(cr.clone())?);
    }
    res = false;
    Ok(res)
}

pub fn resolveConnections(mut flatModel: Arc<FlatModel::NFFlatModel>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut conns: Arc<Connections::NFConnections> = Arc::new(<Connections::NFConnections as ::std::default::Default>::default());
    let mut conn_eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ec_eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut tlio_eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut tlio_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut csets: ConnectionSets::Sets = <ConnectionSets::Sets as ::std::default::Default>::default();
    let mut csets_array: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>;
    let mut unhandled_stream_sets: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>> = metamodelica::nil();
    let mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>;
    let mut broken: Arc<metamodelica::List<Connections::BrokenEdge>> = metamodelica::nil();
    let mut vars: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>;
    let mut connectedLocalIOs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut exposeLocalIOs: i32 = 0;
    vars = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), (flatModel.variables.clone().len() as i32));
    for mut v in &*flatModel.variables.clone() {
        let mut v = v.clone();
        UnorderedMap::addNew(v.name.clone(), v.clone(), vars.clone())?;
    }
    (flatModel, conns) = Connections::collectConnections(flatModel.clone(), Arc::new({ let __pe_b1 = deletedVars.clone(); move |__pe_a0| isDeletedCref(__pe_a0, __pe_b1.clone()) }))?;
    ctable = CardinalityTable::fromConnections(conns.clone())?;
    (flatModel, conns) = ExpandableConnectors::elaborate(flatModel.clone(), conns.clone())?;
    assign_field!(flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            if !(Variable::isPresent(v.clone())) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    conns = Connections::collectFlows(flatModel.clone(), conns.clone())?;
    if System::getHasOverconstrainedConnectors() {
        (flatModel, broken) = NFOCConnectionGraph::handleOverconstrainedConnections(flatModel.clone(), conns.clone(), Arc::new({ let __pe_b1 = deletedVars.clone(); move |__pe_a0| isDeletedCref(__pe_a0, __pe_b1.clone()) }))?;
    }
    conns = Connections::addBroken(broken.clone(), conns.clone());
    conns = Connections::split(conns.clone());
    conns = Connections::scalarize(conns.clone(), !(settings.scalarize.clone()))?;
    csets = ConnectionSets::fromConnections(conns.clone())?;
    (csets_array, _) = ConnectionSets::extractSets(csets.clone());
    (conn_eql, connectedLocalIOs, unhandled_stream_sets) = ConnectEquations::generateEquations(csets_array.clone(), vars.clone())?;
    if System::getHasOverconstrainedConnectors() {
        ec_eql = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Equation::NFEquation>>>>> = metamodelica::nil();
        for mut e in (broken.clone()).into_iter().cloned() {
            let __x = e.brokenEquations.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        assign_field!(flatModel.equations = listAppend(ec_eql.clone(), flatModel.equations.clone()));
    }
    assign_field!(flatModel.equations = listAppend(conn_eql.clone(), flatModel.equations.clone()));
    if !(unhandled_stream_sets.clone().is_empty()) {
        flatModel = StreamFlowAlias::eliminateAliases(flatModel.clone())?;
    }
    exposeLocalIOs = Flags::getConfigInt(Flags::EXPOSE_LOCAL_IOS.clone())?;
    if exposeLocalIOs.clone() > 0 {
        (tlio_vars, tlio_eql) = generateTopLevelIOs(vars.clone(), connectedLocalIOs.clone(), exposeLocalIOs.clone())?;
        assign_field!(
            flatModel.variables = List::append_reverse(flatModel.variables.clone(), tlio_vars.clone()),
            flatModel.equations = List::append_reverse(flatModel.equations.clone(), tlio_eql.clone())
        );
    }
    if System::getHasStreamConnectors() || System::getUsesCardinality() {
        flatModel = evaluateConnectionOperators(flatModel.clone(), csets.clone(), csets_array.clone(), vars.clone(), ctable.clone())?;
    }
    execStat((literal!("NFFlatten.resolveConnections")).clone())?;
    Ok(flatModel)
}

pub fn generateTopLevelIOs(mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut connectedLocalIOs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut exposeLocalIOs: i32) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<metamodelica::List<Arc<Equation::NFEquation>>>)> {
    let mut tlio_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut tlio_eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut attributes: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut tlio_var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut name: ArcStr = arcstr::literal!("");
    let mut tlio_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut level: i32 = 0;
    tlio_vars = metamodelica::nil();
    tlio_eql = metamodelica::nil();
    for mut variable in &*UnorderedMap::valueList(variables.clone()) {
        let mut variable = variable.clone();
        level = ComponentRef::depth(variable.name.clone()) - 1;
        attributes = variable.attributes.clone();
        if 0 < level.clone() && level.clone() <= exposeLocalIOs.clone() && variable.visibility.clone() == Visibility::PUBLIC.clone() && attributes.connectorType.clone() != ConnectorType::NON_CONNECTOR.clone() && (attributes.direction.clone() == Direction::INPUT.clone() || attributes.direction.clone() == Direction::OUTPUT.clone()) && !(UnorderedSet::contains(variable.name.clone(), connectedLocalIOs.clone())?) {
            tlio_var = Variable::removeNonTopLevelDirection(variable.clone())?;
            attributes = tlio_var.attributes.clone();
            if attributes.direction.clone() == Direction::NONE.clone() {
                tlio_var = variable.clone();
                assign_field!(tlio_var.binding = Arc::new(crate::NFBinding::UNBOUND));
                cref = tlio_var.name.clone();
                name = stringDelimitList(ComponentRef::toString_impl(cref.clone(), metamodelica::nil())?, (literal!(".")).clone());
                while UnorderedMap::contains(tlio_var.name.clone(), variables.clone()) {
                    tlio_node = Arc::new(InstNode::InstNode::NAME_NODE { name: (Util::makeQuotedIdentifier((name.clone()).clone())?).clone() });
                    assign_field!(tlio_var.name = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } => Arc::new(ComponentRef::NFComponentRef::CREF { node: tlio_node.clone(), subscripts: var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone(), ty: var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, ComponentRef::NFComponentRef::CREF).clone(), restCref: Arc::new(crate::NFComponentRef::EMPTY) }),
        _ => bail!("match: no arm matched"),
    } }));
                    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); ArcStr::from(__mm_s) }).clone();
                }
                tlio_vars = cons(tlio_var.clone(), tlio_vars.clone());
                tlio_eql = cons(Equation::makeCrefEquality(variable.name.clone(), tlio_var.name.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), ElementSource::createElementSource(variable.info.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?)?, tlio_eql.clone());
            }
        }
    }
    Ok((tlio_vars, tlio_eql))
}

pub fn evaluateConnectionOperators(mut flatModel: Arc<FlatModel::NFFlatModel>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut c in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = evaluateBindingConnOp(c.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.equations = evaluateEquationsConnOp(flatModel.equations.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?,
        flatModel.initialEquations = evaluateEquationsConnOp(flatModel.initialEquations.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?
    );
    Ok(flatModel)
}

pub fn evaluateBindingConnOp(mut var: Arc<Variable::NFVariable>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eval_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let () = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { .. } if (Binding::hasExp(var.binding.clone())) => {
            exp = Binding::getExp(var.binding.clone())?;
            eval_exp = ConnectEquations::evaluateOperators(exp.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
            if !(referenceEq(&exp.clone(),&eval_exp.clone())) {
                assign_field!(var.binding = Binding::setExp(eval_exp.clone(), var.binding.clone())?);
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn evaluateEquationsConnOp(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    equations = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (equations.clone()).into_iter().cloned() {
            let __x = evaluateEquationConnOp(eq.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(equations)
}

pub fn evaluateEquationConnOp(mut eq: Arc<Equation::NFEquation>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    eq = Equation::mapExp(eq.clone(), Arc::new({ let __pe_b1 = sets.clone(); let __pe_b2 = setsArray.clone(); let __pe_b3 = variables.clone(); let __pe_b4 = ctable.clone(); move |__pe_a0| ConnectEquations::evaluateOperators(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }));
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::IF { .. } => {
            for mut b in &*var_field!((*eq).branches, Equation::NFEquation::IF).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => {
            if var_field!((*b).conditionVar, Equation::Branch::Branch::BRANCH).clone() == Variability::PARAMETER.clone() && !(Structural::isExpressionNotFixed(var_field!((*b).condition, Equation::Branch::Branch::BRANCH).clone(), false, 100)?) {
                Structural::markExp(var_field!((*b).condition, Equation::Branch::Branch::BRANCH).clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub fn resolveArrayConnections(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    flatModel = ArrayConnections::resolve(flatModel.clone())?;
    execStat((literal!("NFFlatten.resolveArrayConnections")).clone())?;
    Ok(flatModel)
}

pub fn collectComponentFuncs(mut var: Arc<Variable::NFVariable>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { .. } => {
            funcs = collectTypeFuncs(var.ty.clone(), funcs.clone())?;
            funcs = collectBindingFuncs(var.binding.clone(), funcs.clone())?;
            for mut attr in &*var.typeAttributes.clone() {
                let mut attr = attr.clone();
                funcs = collectBindingFuncs(Util::tuple22(attr.clone()), funcs.clone())?;
            }
            for mut c in &*var.children.clone() {
                let mut c = c.clone();
                funcs = collectComponentFuncs(c.clone(), funcs.clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(funcs)
}

pub fn collectBindingFuncs(mut binding: Arc<Binding::NFBinding>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    if Binding::isExplicitlyBound(binding.clone()) {
        funcs = collectExpFuncs(Binding::getTypedExp(binding.clone())?, funcs.clone())?;
    }
    Ok(funcs)
}

pub fn collectTypeFuncs(mut ty: Arc<Type::NFType>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { .. } => {
            funcs = Dimension::foldExpList(var_field!((*ty).dimensions, Type::NFType::ARRAY).clone(), (std::sync::Arc::new(collectExpFuncs_traverse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone())?;
            funcs = collectTypeFuncs(var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), funcs.clone())?;
            ()
        },
        Deref @ Type::FUNCTION { r#fn, .. } => {
            funcs = flattenFunction(r#fn.clone(), funcs.clone())?;
            ()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { destructor: de, constructor: con }, .. } => {
            funcs = collectStructor(con.clone(), funcs.clone())?;
            funcs = collectStructor(de.clone(), funcs.clone())?;
            ()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: con, .. }, .. } => {
            funcs = collectStructor(con.clone(), funcs.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub fn collectStructor(mut node: Arc<InstNode::InstNode>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
    let mut r#fn: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    cache = InstNode::getFuncCache(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::FUNCTION { .. } => {
            for mut r#fn in &*var_field!((*cache).funcs, CachedData::CachedData::FUNCTION).clone() {
                let mut r#fn = r#fn.clone();
                funcs = flattenFunction(r#fn.clone(), funcs.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub fn collectEquationFuncs(mut eq: Arc<Equation::NFEquation>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            funcs = collectExpFuncs(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), funcs.clone())?;
            funcs = collectExpFuncs(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), funcs.clone())?;
            funcs = collectTypeFuncs(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone(), funcs.clone())?;
            ()
        },
        Deref @ Equation::FOR { .. } => {
            funcs = List::fold(var_field!((*eq).body, Equation::NFEquation::FOR).clone(), (std::sync::Arc::new(collectEquationFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            ()
        },
        Deref @ Equation::IF { .. } => {
            funcs = List::fold(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), (std::sync::Arc::new(collectEqBranchFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Branch::Branch>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            ()
        },
        Deref @ Equation::WHEN { .. } => {
            funcs = List::fold(var_field!((*eq).branches, Equation::NFEquation::WHEN).clone(), (std::sync::Arc::new(collectEqBranchFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Branch::Branch>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            ()
        },
        Deref @ Equation::ASSERT { .. } => {
            funcs = collectExpFuncs(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), funcs.clone())?;
            funcs = collectExpFuncs(var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), funcs.clone())?;
            funcs = collectExpFuncs(var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), funcs.clone())?;
            ()
        },
        Deref @ Equation::TERMINATE { .. } => {
            funcs = collectExpFuncs(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), funcs.clone())?;
            ()
        },
        Deref @ Equation::REINIT { .. } => {
            funcs = collectExpFuncs(var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), funcs.clone())?;
            ()
        },
        Deref @ Equation::NORETCALL { .. } => {
            funcs = collectExpFuncs(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), funcs.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub fn collectEqBranchFuncs(mut branch: Arc<Equation::Branch::Branch>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => {
            funcs = collectExpFuncs(var_field!((*branch).condition, Equation::Branch::Branch::BRANCH).clone(), funcs.clone())?;
            funcs = List::fold(var_field!((*branch).body, Equation::Branch::Branch::BRANCH).clone(), (std::sync::Arc::new(collectEquationFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub fn collectAlgorithmFuncs(mut alg: Arc<Algorithm::NFAlgorithm>, mut funcs: FunctionTree) -> FunctionTree {
    let mut funcs: FunctionTree = funcs;
    funcs = List::fold(alg.statements.clone(), (std::sync::Arc::new(collectStatementFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
    funcs
}

pub fn collectStatementFuncs(mut stmt: Arc<Statement::NFStatement>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            funcs = collectExpFuncs(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), funcs.clone())?;
            funcs = collectExpFuncs(var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), funcs.clone())?;
            funcs = collectTypeFuncs(var_field!((*stmt).ty, Statement::NFStatement::ASSIGNMENT).clone(), funcs.clone())?;
            ()
        },
        Deref @ Statement::FOR { .. } => {
            funcs = List::fold(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), (std::sync::Arc::new(collectStatementFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            funcs = collectExpFuncs(Util::getOption(var_field!((*stmt).range, Statement::NFStatement::FOR).clone())?, funcs.clone())?;
            ()
        },
        Deref @ Statement::IF { .. } => {
            funcs = List::fold(var_field!((*stmt).branches, Statement::NFStatement::IF).clone(), (std::sync::Arc::new(collectStmtBranchFuncs) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            ()
        },
        Deref @ Statement::WHEN { .. } => {
            funcs = List::fold(var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone(), (std::sync::Arc::new(collectStmtBranchFuncs) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            ()
        },
        Deref @ Statement::ASSERT { .. } => {
            funcs = collectExpFuncs(var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), funcs.clone())?;
            funcs = collectExpFuncs(var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), funcs.clone())?;
            funcs = collectExpFuncs(var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), funcs.clone())?;
            ()
        },
        Deref @ Statement::TERMINATE { .. } => {
            funcs = collectExpFuncs(var_field!((*stmt).message, Statement::NFStatement::TERMINATE).clone(), funcs.clone())?;
            ()
        },
        Deref @ Statement::REINIT { .. } => {
            funcs = collectExpFuncs(var_field!((*stmt).cref, Statement::NFStatement::REINIT).clone(), funcs.clone())?;
            funcs = collectExpFuncs(var_field!((*stmt).reinitExp, Statement::NFStatement::REINIT).clone(), funcs.clone())?;
            ()
        },
        Deref @ Statement::NORETCALL { .. } => {
            funcs = collectExpFuncs(var_field!((*stmt).exp, Statement::NFStatement::NORETCALL).clone(), funcs.clone())?;
            ()
        },
        Deref @ Statement::WHILE { .. } => {
            funcs = collectExpFuncs(var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), funcs.clone())?;
            funcs = List::fold(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone(), (std::sync::Arc::new(collectStatementFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub fn collectStmtBranchFuncs(mut branch: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    funcs = collectExpFuncs(Util::tuple21(branch.clone()), funcs.clone())?;
    funcs = List::fold(Util::tuple22(branch.clone()), (std::sync::Arc::new(collectStatementFuncs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
    Ok(funcs)
}

pub fn collectExpFuncs(mut exp: Arc<Expression::NFExpression>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    funcs = Expression::fold(exp.clone(), (std::sync::Arc::new(collectExpFuncs_traverse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone())?;
    Ok(funcs)
}

pub fn collectExpFuncs_traverse(mut exp: Arc<Expression::NFExpression>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } => {
            funcs = flattenFunction(Call::typedFunction(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?, funcs.clone())?;
            ()
        },
        Deref @ Expression::CREF { .. } => {
            funcs = collectTypeFuncs(var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), funcs.clone())?;
            ()
        },
        Deref @ Expression::RECORD { .. } => {
            funcs = collectTypeFuncs(var_field!((*exp).ty, Expression::NFExpression::RECORD).clone(), funcs.clone())?;
            ()
        },
        Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. } => {
            for mut f in &*Function::getRefCache(var_field!((*exp).r#fn, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())? {
                let mut f = f.clone();
                funcs = flattenFunction(f.clone(), funcs.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub fn flattenFunction(mut func: Arc<Function::Function>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let mut r#fn: Arc<Function::Function> = func.clone();
    if !(Function::isCollected(r#fn.clone())) {
        r#fn = Function::mapExp(r#fn.clone(), (std::sync::Arc::new(Expression::expandSplitIndices) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), (std::sync::Arc::new(Expression::expandSplitIndices) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), true, true)?;
        r#fn = EvalConstants::evaluateFunction(r#fn.clone())?;
        SimplifyModel::simplifyFunction(r#fn.clone())?;
        Function::collect(r#fn.clone());
        if !(InstNode::isPartial(r#fn.node.clone())) {
            funcs = FunctionTreeImpl::add(funcs.clone(), Function::name(r#fn.clone()), r#fn.clone(), (std::sync::Arc::new(fnptr!(FunctionTreeImpl::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            funcs = collectClassFunctions(r#fn.node.clone(), funcs.clone())?;
            for mut fn_der in &*r#fn.derivatives.clone() {
                let mut fn_der = fn_der.clone();
                for mut der_fn in &*Function::getCachedFuncs(fn_der.derivativeFn.clone())? {
                    let mut der_fn = der_fn.clone();
                    funcs = flattenFunction(der_fn.clone(), funcs.clone())?;
                }
            }
            let __range0 = r#fn.inverses.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut fn_inv in __range0 {
                funcs = collectExpFuncs(fn_inv.inverseCall.clone(), funcs.clone())?;
            }
            if Function::isPartialDerivative(r#fn.clone()) {
                for mut f in &*Function::getCachedFuncs(Class::lastBaseClass(r#fn.node.clone()))? {
                    let mut f = f.clone();
                    flattenFunction(f.clone(), funcs.clone())?;
                }
            }
        }
    }
    Ok(funcs)
}

pub fn collectClassFunctions(mut clsNode: Arc<InstNode::InstNode>, mut funcs: FunctionTree) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    cls = InstNode::getClass(clsNode.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { sections, elements: cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            let __range0 = var_field!((**cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                comp = InstNode::component(c.clone())?;
                funcs = collectTypeFuncs(Component::getType(comp.clone())?, funcs.clone())?;
                binding = Component::getBinding(comp.clone());
                if Binding::isExplicitlyBound(binding.clone()) {
                    funcs = collectExpFuncs(Binding::getTypedExp(binding.clone())?, funcs.clone())?;
                }
            }
            let () = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { .. } => {
            funcs = List::fold(var_field!((**sections).algorithms, Sections::NFSections::SECTIONS).clone(), (std::sync::Arc::new(fnptr!(collectAlgorithmFuncs, Arc<Algorithm::NFAlgorithm>, Arc<FunctionTreeImpl::Tree>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, Arc<FunctionTreeImpl::Tree>) -> Result<Arc<FunctionTreeImpl::Tree>> + 'static>), funcs.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Class::TYPED_DERIVED { .. } => {
            funcs = collectClassFunctions(var_field!((*cls).baseClass, Class::NFClass::TYPED_DERIVED).clone(), funcs.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub fn updateForType(mut forType: Arc<Statement::ForType>, mut forBody: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<Statement::ForType>> {
    let mut forType: Arc<Statement::ForType> = forType;
    let mut vars: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>>;
    let () = (::match_deref::match_deref! { match &(forType.clone()) {
        Deref @ Statement::ForType::NORMAL => (),
        Deref @ Statement::ForType::PARALLEL { .. } => {
            vars = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            for mut s in &*forBody.clone() {
                let mut s = s.clone();
                vars = Statement::fold(s.clone(), (std::sync::Arc::new(collectParallelVariables) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>>> + 'static>), vars.clone());
            }
            assign_variant_field!(forType => Statement::ForType::PARALLEL; vars = UnorderedMap::toList(vars.clone()));
            for mut v in &*var_field!((*forType).vars, Statement::ForType::PARALLEL).clone() {
                let mut v = v.clone();
                checkParGlobalCref(v.clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(forType)
}

pub fn collectParallelVariables(mut stmt: Arc<Statement::NFStatement>, mut vars: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>>> {
    let mut vars: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>> = vars;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = Statement::info(stmt.clone());
    vars = Statement::foldExp(stmt.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, _) -> Result<_> + 'static> = Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0, __pe_a2| collectParallelVariablesExp(__pe_a0, __pe_b1.clone(), __pe_a2) }); move |__pe_a0, __pe_a2| Expression::fold(__pe_a0, __pe_b1.clone(), __pe_a2) }), vars.clone())?;
    Ok(vars)
}

pub fn collectParallelVariablesExp(mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo, mut vars: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>>> {
    let mut vars: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, SourceInfo>> = vars;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()) && !(ComponentRef::isIterator(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) && InstNode::isComponent(ComponentRef::node(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?)) => {
            cref = ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone());
            UnorderedMap::tryAdd(cref.clone(), info.clone(), vars.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vars)
}

pub fn checkParGlobalCref(mut crefInfo: (Arc<ComponentRef::NFComponentRef>, SourceInfo)) -> Result<()> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut errorString: ArcStr = arcstr::literal!("");
    (cref, info) = crefInfo.clone();
    node = ComponentRef::node(cref.clone())?;
    if Component::parallelism(InstNode::component(node.clone())?) != Parallelism::GLOBAL.clone() {
        errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Component '")); __mm_s.push_str(&*AbsynUtil::pathString(ComponentRef::toPath(cref.clone())?, (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' is used in a parallel for loop.")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel for loops can only contain references to parglobal variables")); ArcStr::from(__mm_s) }).clone();
        Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn verifyDimensions(mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut component: Arc<InstNode::InstNode>) -> Result<()> {
    for mut d in &*dimensions.clone() {
        let mut d = d.clone();
        verifyDimension(d.clone(), component.clone())?;
    }
    Ok(())
}

pub fn verifyDimension(mut dimension: Arc<Dimension::NFDimension>, mut component: Arc<InstNode::InstNode>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(dimension.clone()) {
        Deref @ Dimension::INTEGER { .. } => {
            if var_field!((*dimension).size, Dimension::NFDimension::INTEGER).clone() < 0 {
                Error::addSourceMessage(Error::NEGATIVE_DIMENSION_INDEX.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*dimension).size, Dimension::NFDimension::INTEGER).clone())), (InstNode::name(component.clone())?).clone()], InstNode::info(component.clone())?)?;
                bail!("fail");
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn updateVariability(mut var: Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    let mut v: Variability = Variability::CONSTANT;
    if var.attributes.variability.clone() == Variability::PARAMETER.clone() {
        v = Component::variability(InstNode::component(ComponentRef::node(var.name.clone())?)?);
        if v.clone() < Variability::PARAMETER.clone() {
            var = Variable::setVariability(var.clone(), v.clone());
        }
    }
    Ok(var)
}

pub fn evaluateIfWithConnects(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut outEql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        outEql = evaluateIfWithConnects2(eq.clone(), outEql.clone())?;
    }
    outEql = outEql.clone().reverse();
    Ok(outEql)
}

pub fn evaluateIfWithConnects2(mut eq: Arc<Equation::NFEquation>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Variability = Variability::CONSTANT;
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut target: Arc<Ceval::EvalTarget::EvalTarget> = Arc::new(<Ceval::EvalTarget::EvalTarget as ::std::default::Default>::default());
    let mut bl: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    equations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::IF { .. } if (Equation::contains(eq.clone(), (std::sync::Arc::new(fnptr!(Equation::isConnect, Arc<Equation::NFEquation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>)) || Equation::containsExp(eq.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static> = (std::sync::Arc::new(Expression::isConnectionCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>); move |__pe_a0| Expression::contains(__pe_a0, __pe_b1.clone()) }))?) => {
            target = Ceval::EvalTarget::new(Equation::info(eq.clone()), NFInstContext::NO_CONTEXT.clone(), None);
            for mut branch in &*var_field!((*eq).branches, Equation::NFEquation::IF).clone() {
                let mut branch = branch.clone();
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(branch.clone()) {
                    Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: __pa1, body: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                cond = __pa0.clone();
                var = __pa1.clone();
                eql = __pa2.clone();
                if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
                    if Expression::isPure(cond.clone())? {
                        Structural::markExp(cond.clone())?;
                        cond = Ceval::evalExp(cond.clone(), target.clone())?;
                    }
                    if !(Expression::isBoolean(cond.clone())) {
                        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to evaluate branch condition in if equation containing connect equations: `")); __mm_s.push_str(&*Expression::toString(cond.clone())?); __mm_s.push_str(&*literal!("`")); ArcStr::from(__mm_s) }).clone(), Equation::info(eq.clone()))?;
                        bail!("fail");
                    }
                }
                if Expression::isTrue(cond.clone()) {
                    if bl.clone().is_empty() {
                        eql = evaluateIfWithConnects(eql.clone())?;
                        equations = listAppend(eql.clone(), equations.clone());
                        bl = metamodelica::nil();
                    } else {
                        bl = cons(Equation::makeBranch(cond.clone(), eql.clone(), var.clone()), bl.clone());
                    }
                    break;
                } else if !(Expression::isFalse(cond.clone())) {
                    bl = cons(Equation::makeBranch(cond.clone(), eql.clone(), var.clone()), bl.clone());
                }
            }
            if !(bl.clone().is_empty()) {
                equations = cons(Arc::new(Equation::NFEquation::IF { branches: bl.clone().reverse(), scope: var_field!((*eq).scope, Equation::NFEquation::IF).clone(), source: var_field!((*eq).source, Equation::NFEquation::IF).clone() }), equations.clone());
            }
            equations.clone()
        },
        _ => cons(eq.clone(), equations.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub fn checkDeletedVarRefs(mut flatModel: Arc<FlatModel::NFFlatModel>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> Result<()> {
    for mut var in &*flatModel.variables.clone() {
        let mut var = var.clone();
        checkDeletedVarRefsInVar(var.clone(), deletedVars.clone(), settings.clone());
    }
    for mut eq in &*flatModel.equations.clone() {
        let mut eq = eq.clone();
        checkDeletedVarRefsInEq(eq.clone(), deletedVars.clone(), settings.clone())?;
    }
    for mut eq in &*flatModel.initialEquations.clone() {
        let mut eq = eq.clone();
        checkDeletedVarRefsInEq(eq.clone(), deletedVars.clone(), settings.clone())?;
    }
    for mut alg in &*flatModel.algorithms.clone() {
        let mut alg = alg.clone();
        checkDeletedVarRefsInAlg(alg.clone(), deletedVars.clone(), settings.clone())?;
    }
    for mut alg in &*flatModel.initialAlgorithms.clone() {
        let mut alg = alg.clone();
        checkDeletedVarRefsInAlg(alg.clone(), deletedVars.clone(), settings.clone())?;
    }
    Ok(())
}

pub fn checkDeletedVarRefsInVar(mut var: Arc<Variable::NFVariable>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> () {
    Variable::applyExpShallow(var.clone(), Arc::new({ let __pe_b1 = deletedVars.clone(); let __pe_b2 = settings.clone(); let __pe_b3 = var.info.clone(); move |__pe_a0| checkDeletedVarRefsInExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }));
    ()
}

pub fn checkDeletedVarRefsInExp(mut exp: Arc<Expression::NFExpression>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings, mut info: SourceInfo) -> Result<()> {
    Expression::apply(exp.clone(), Arc::new({ let __pe_b1 = deletedVars.clone(); let __pe_b2 = settings.clone(); let __pe_b3 = info.clone(); move |__pe_a0| checkDeletedVarRefsInExp_traverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    Ok(())
}

pub fn checkDeletedVarRefsInExp_traverser(mut exp: Arc<Expression::NFExpression>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (isDeletedCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), deletedVars.clone())?) => {
            Error::addSourceMessage(Error::INVALID_DELETED_COMPONENT_CONTEXT.clone(), list![(ComponentRef::toString(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?).clone()], info.clone())?;
            if !(settings.relaxedErrorChecking.clone()) {
                bail!("fail");
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkDeletedVarRefsInEq(mut eq: Arc<Equation::NFEquation>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> Result<()> {
    Equation::applyExp(eq.clone(), Arc::new({ let __pe_b1 = deletedVars.clone(); let __pe_b2 = settings.clone(); let __pe_b3 = Equation::info(eq.clone()); move |__pe_a0| checkDeletedVarRefsInExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    Ok(())
}

pub fn checkDeletedVarRefsInAlg(mut alg: Arc<Algorithm::NFAlgorithm>, mut deletedVars: DeletedVariables, mut settings: FlattenSettings) -> Result<()> {
    for mut stmt in &*alg.statements.clone() {
        let mut stmt = stmt.clone();
        Statement::applyExp(stmt.clone(), Arc::new({ let __pe_b1 = deletedVars.clone(); let __pe_b2 = settings.clone(); let __pe_b3 = Statement::info(stmt.clone()); move |__pe_a0| checkDeletedVarRefsInExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    }
    Ok(())
}

