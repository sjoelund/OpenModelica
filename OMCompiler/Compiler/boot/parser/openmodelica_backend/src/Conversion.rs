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

use crate::GlobalScriptDump;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_frontend::Parser;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub enum ConversionRule {
    /// convertClass
    CLASS {
        oldPath: metamodelica::Array<ArcStr>,
        newPath: Arc<Path>,
    },
    /// convertClassIf (not yet implemented)
    CLASS_IF,
    /// convertElement
    ELEMENT {
        oldPath: metamodelica::Array<ArcStr>,
        oldName: ArcStr,
        newName: ArcStr,
    },
    /// convertModifiers
    MODIFIERS {
        oldMods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>,
        newMods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>,
        info: SourceInfo,
    },
    /// convertMessage
    MESSAGE {
        message: ArcStr,
    },
}
impl Default for ConversionRule {
    fn default() -> Self { Self::CLASS_IF }
}
pub use self::ConversionRule::{CLASS,CLASS_IF,ELEMENT,MODIFIERS,MESSAGE};

pub mod ConversionRules {
    use super::*;
    /// Structure used to store conversion rules. Each node corresponds to one
    ///     element, and each node has a map of child nodes and a list of rules. So
    ///     e.g. convertClass('A.B', 'A.C') becomes
    ///     A(nodes = {B(nodes = {}, rules = {convertClass(A.C)})}, rules = {})
#[derive(Clone, Debug, PartialEq)]
    pub struct ConversionRules {
        pub nodes: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<ConversionRules>>>,
        pub rules: Arc<metamodelica::List<ConversionRule>>,
    }

    impl Default for ConversionRules {
        fn default() -> Self {
            Self {
                nodes: Default::default(),
                rules: Default::default(),
            }
        }
    }

    pub type CONVERSION_RULES = ConversionRules;

    pub fn newNode() -> Arc<ConversionRules> {
        let mut node: Arc<ConversionRules>;
        node = Arc::new(ConversionRules { nodes: UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), stringEq, 1), rules: metamodelica::nil() });
        node
    }

}

pub type RuleList = Arc<metamodelica::List<ConversionRule>>;

pub type RuleTable = Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<metamodelica::List<ConversionRule>>>>;

pub type TypeTable = Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Path>>>;

// Used to specify which arguments to the conversion functions can be vectorized.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ArgType {
    SCALAR = 1,
    ARRAY = 2,
}
impl PartialOrd for ArgType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ArgType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub static CONVERT_CLASS_TYPE: std::sync::LazyLock<Arc<metamodelica::List<ArgType>>> = std::sync::LazyLock::new(|| { list![ArgType::SCALAR.clone(), ArgType::SCALAR.clone()] });

pub static CONVERT_CLASS_IF_TYPE: std::sync::LazyLock<Arc<metamodelica::List<ArgType>>> = std::sync::LazyLock::new(|| { list![ArgType::SCALAR.clone(), ArgType::SCALAR.clone(), ArgType::SCALAR.clone(), ArgType::SCALAR.clone()] });

pub static CONVERT_ELEMENT_TYPE: std::sync::LazyLock<Arc<metamodelica::List<ArgType>>> = std::sync::LazyLock::new(|| { list![ArgType::SCALAR.clone(), ArgType::SCALAR.clone(), ArgType::SCALAR.clone()] });

pub static CONVERT_MODIFIER_TYPE: std::sync::LazyLock<Arc<metamodelica::List<ArgType>>> = std::sync::LazyLock::new(|| { list![ArgType::SCALAR.clone(), ArgType::ARRAY.clone(), ArgType::ARRAY.clone(), ArgType::SCALAR.clone()] });

pub static CONVERT_MESSAGE_TYPE: std::sync::LazyLock<Arc<metamodelica::List<ArgType>>> = std::sync::LazyLock::new(|| { list![ArgType::SCALAR.clone(), ArgType::SCALAR.clone(), ArgType::SCALAR.clone()] });

/// Struct for storing import data.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImportData {
    /// The import before conversion
    pub originalPath: Arc<Path>,
    /// The import after conversion
    pub convertedPath: Arc<Path>,
    /// The import name after conversion (same as before for
    ///                         named imports, possibly different for qualified imports)
    pub importName: ArcStr,
    /// Shadowed by another element or not
    pub shadowed: bool,
}

pub type IMPORT_DATA = ImportData;


pub type ImportTree = Arc<ImportTreeImpl::Tree>;

pub mod ImportTreeImpl {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = ImportData;

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

    pub use addConflictReplace as addConflictDefault;

    pub type ConflictFunc = fn(Value, Value, Key) -> Result<Value>;

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

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: ConflictFunc) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value;
            let mut key_comp: i32 = 0;
            key_comp = keyCompare((inKey.clone()).clone(), (key.clone()).clone());
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), (inKey.clone()).clone(), inValue.clone(), conflictFunc)?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), (inKey.clone()).clone(), inValue.clone(), conflictFunc)?);
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::NODE).clone(), (key.clone()).clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::NODE).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::NODE; value = value.clone());
                }
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            let mut value: Value;
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare((inKey.clone()).clone(), (var_field!((*tree).key, Tree::LEAF).clone()).clone());
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }), right: Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }) });
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

    pub fn addConflictFail(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Result<Value> {
        let mut value: Value;
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

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(ArcStr, ImportData)>>, mut conflictFunc: ConflictFunc) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key = arcstr::literal!("");
        let mut value: Value;
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc)?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<ImportData>) -> Result<ImportData> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = fn(Option<ImportData>) -> Result<Value>;

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
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }), right: Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }) });
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, ImportData, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
        pub type FoldFunc<FT: Clone> = fn(Key, Value, FT) -> Result<FT>;

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

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, ImportData, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
        pub type FoldFunc<FT: Clone> = fn(Key, Value, FT) -> Result<(FT, bool)>;

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

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, ImportData, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
        pub type FoldFunc<FT1: Clone, FT2: Clone> = fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)>;

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

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(ArcStr, ImportData) -> Result<()> + 'static>) -> Result<()> {
        pub type EachFunc = fn(Key, Value) -> Result<()>;

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

    pub fn fromList(mut inValues: Arc<metamodelica::List<(ArcStr, ImportData)>>, mut conflictFunc: ConflictFunc) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY);
        let mut key: Key = arcstr::literal!("");
        let mut value: Value;
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc)?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value;
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
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Option<ImportData> {
        let mut value: Option<ImportData> = None;
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: ConflictFunc) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ Tree::EMPTY { .. } => tree.clone(),
        Deref @ Tree::NODE { .. } => {
            tree = add(tree.clone(), (var_field!((*treeToJoin).key, Tree::NODE).clone()).clone(), var_field!((*treeToJoin).value, Tree::NODE).clone(), conflictFunc)?;
            tree = join(tree.clone(), var_field!((*treeToJoin).left, Tree::NODE).clone(), conflictFunc)?;
            tree = join(tree.clone(), var_field!((*treeToJoin).right, Tree::NODE).clone(), conflictFunc)?;
            tree.clone()
        },
        Deref @ Tree::LEAF { .. } => add(tree.clone(), (var_field!((*treeToJoin).key, Tree::LEAF).clone()).clone(), var_field!((*treeToJoin).value, Tree::LEAF).clone(), conflictFunc)?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn listKeys(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
        let mut lst: Arc<metamodelica::List<ArcStr>> = lst;
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

    pub fn listKeysReverse(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
        let mut lst: Arc<metamodelica::List<ArcStr>> = lst;
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

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<ImportData>>) -> Arc<metamodelica::List<ImportData>> {
        let mut lst: Arc<metamodelica::List<ImportData>> = lst;
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

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, ImportData) -> Result<ImportData> + 'static>) -> Arc<Tree> {
        pub type MapFunc = fn(Key, Value) -> Result<Value>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value;
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
            let mut new_value: Value;
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

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, ImportData, FT) -> Result<(ImportData, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone> = fn(Key, Value, FT) -> Result<Value>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value;
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
            let mut new_value: Value;
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
        let mut outTree: Arc<Tree> = Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY);
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::Conversion::ImportTreeImpl::Tree::EMPTY), node.clone())?
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

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(ArcStr, ImportData)>>) -> Arc<metamodelica::List<(ArcStr, ImportData)>> {
        let mut lst: Arc<metamodelica::List<(ArcStr, ImportData)>> = lst;
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
        let mut outTree: Arc<Tree> = add(tree.clone(), (key.clone()).clone(), value.clone(), fnptr!(addConflictReplace, ImportData, ImportData, ArcStr)).unwrap();
        outTree
    }

}

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub components: TypeTable,
    pub imports: ImportTree,
}

pub type ENV = Env;


pub fn convertPackage(mut cls: Arc<Absyn::Class>, mut scriptFile: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut rules: Arc<ConversionRules::ConversionRules>;
    let mut env: Env;
    let mut stmts: Arc<metamodelica::List<GlobalScript::Statement>> = metamodelica::nil();
    stmts = loadScript((scriptFile.clone()).clone())?;
    rules = ConversionRules::newNode();
    rules = parseRules(stmts.clone(), rules.clone())?;
    if Flags::isSet(Flags::DUMP_CONVERSION_RULES.clone())? {
        dumpRules(rules.clone(), (literal!("")).clone())?;
    }
    cls = convertClass(cls.clone(), rules.clone(), newEnv(), metamodelica::nil())?;
    Ok(cls)
}

fn loadScript(mut scriptFile: ArcStr) -> Result<Arc<metamodelica::List<GlobalScript::Statement>>> {
    let mut stmts: Arc<metamodelica::List<GlobalScript::Statement>> = metamodelica::nil();
    let mut script: ArcStr = arcstr::literal!("");
    script = (System::readFile((scriptFile.clone()).clone())?).clone();
    script = (System::stringReplace((script.clone()).clone(), (literal!(")\n")).clone(), (literal!(");\n")).clone())?).clone();
    let GlobalScript::Statements { interactiveStmtLst: __pa0, .. } = (Parser::parsestringexp((script.clone()).clone(), (scriptFile.clone()).clone())?) else { bail!("pattern mismatch") };
    stmts = __pa0.clone();
    Ok(stmts)
}

fn parseRules(mut stmts: Arc<metamodelica::List<GlobalScript::Statement>>, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    for mut stmt in &*stmts.clone() {
        let mut stmt = stmt.clone();
        rules = parseRule(stmt.clone(), rules.clone())?;
    }
    Ok(rules)
}

fn parseRule(mut stmt: GlobalScript::Statement, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    type ParseFn = fn(Arc<metamodelica::List<Arc<Absyn::Exp>>>, SourceInfo, Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>>;

    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    let mut fn_name: ArcStr = arcstr::literal!("");
    let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut parse_fn: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::Exp>>>, SourceInfo, Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> + 'static>;
    let mut fn_type: Arc<metamodelica::List<ArgType>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: fn_name, .. }, .. }, .. } => {
            let mut args = (*args).clone();
            (parse_fn, fn_type) = (::match_deref::match_deref! { match &(fn_name.clone()) {
        Deref @ "convertClass" => (parseConvertClass, CONVERT_CLASS_TYPE.clone()),
        Deref @ "convertClassIf" => (parseConvertClassIf, CONVERT_CLASS_IF_TYPE.clone()),
        Deref @ "convertElement" => (parseConvertElement, CONVERT_ELEMENT_TYPE.clone()),
        Deref @ "convertModifiers" => (parseConvertModifiers, CONVERT_MODIFIER_TYPE.clone()),
        Deref @ "convertMessage" => (parseConvertMessage, CONVERT_MESSAGE_TYPE.clone()),
        _ => {
            printConversionRuleError(stmt.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut a in (args.clone()).into_iter().cloned() {
            let __x = expandArg(a.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            for mut a in &*vectorizeArgs(args.clone(), fn_type.clone(), stmt.clone())? {
                let mut a = a.clone();
                rules = parse_fn(a.clone(), var_field!(stmt.info, GlobalScript::Statement::IEXP).clone(), rules.clone())?;
            }
            ()
        },
        _ => {
            printConversionRuleError(stmt.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rules)
}

fn expandArg(mut exp: Arc<Absyn::Exp>) -> Arc<Absyn::Exp> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::INTEGER { value: 0 }, tail: Deref @ metamodelica::List::Nil } }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "fill", .. }, .. } => Arc::new(Absyn::Exp::ARRAY { arrayExp: metamodelica::nil() }),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn vectorizeArgs(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut fnType: Arc<metamodelica::List<ArgType>>, mut stmt: GlobalScript::Statement) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>> {
    let mut vargs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
    let mut vdim: i32 = -1;
    let mut dim: i32 = 0;
    let mut fn_ty: Arc<metamodelica::List<ArgType>> = fnType.clone();
    let mut arg_ty: ArgType = ArgType::SCALAR;
    let mut is_varg: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    if (args.clone().len() as i32) > (fnType.clone().len() as i32) {
        printConversionRuleError(stmt.clone())?;
    }
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(fn_ty.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg_ty = __pa0.clone();
        fn_ty = __pa1.clone();
        (vdim, is_varg) = (::match_deref::match_deref! { match &((arg.clone(), arg_ty.clone())) {
        (Deref @ Absyn::Exp::ARRAY { .. }, ArgType::SCALAR) => {
            dim = (var_field!((*arg).arrayExp, Absyn::Exp::ARRAY).clone().len() as i32);
            if vdim.clone() >= 0 && dim.clone() != vdim.clone() {
                printConversionRuleError(stmt.clone())?;
            }
            (dim.clone(), cons(true, is_varg.clone()))
        },
        (Deref @ Absyn::Exp::ARRAY { .. }, ArgType::ARRAY { .. }) => (vdim.clone(), cons(false, is_varg.clone())),
        (_, ArgType::ARRAY { .. }) => {
            printConversionRuleError(stmt.clone())?;
            bail!("fail")
        },
        _ => (vdim.clone(), cons(false, is_varg.clone())),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if vdim.clone() == 0 {
        vargs = metamodelica::nil();
    } else if vdim.clone() == -1 {
        vargs = list![args.clone()];
    } else {
        vargs = metamodelica::nil();
        for mut arg in &*args.clone().reverse() {
            let mut arg = arg.clone();
            if listHead(is_varg.clone())? {
                let __pa2 = ::match_deref::match_deref! { match &(arg.clone()) {
                    Deref @ Absyn::Exp::ARRAY { arrayExp: __pa2 } => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                expl = __pa2.clone();
                vargs = cons(expl.clone(), vargs.clone());
            } else {
                vargs = cons(List::fill(arg.clone(), vdim.clone()), vargs.clone());
            }
            is_varg = listRest(is_varg.clone())?;
        }
        vargs = List::transposeList(vargs.clone())?;
    }
    Ok(vargs)
}

fn statementInfo(mut stmt: GlobalScript::Statement) -> SourceInfo {
    let mut info: SourceInfo;
    info = (match stmt.clone() {
        GlobalScript::Statement::IEXP { .. } => var_field!(stmt.info, GlobalScript::Statement::IEXP).clone(),
        _ => Absyn::dummyInfo.clone(),
    });
    info
}

fn printConversionRuleError(mut stmt: GlobalScript::Statement) -> Result<()> {
    Error::addSourceMessage(Error::INVALID_CONVERSION_RULE.clone(), list![(GlobalScriptDump::printIstmtStr(stmt.clone())?).clone()], statementInfo(stmt.clone()))?;
    bail!("fail");
    Ok(())
}

fn parseConvertClass(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut info: SourceInfo, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: old_cls }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: new_cls }, tail: Deref @ metamodelica::List::Nil } } => {
            parseConvertClassStr((old_cls.clone()).clone(), (new_cls.clone()).clone(), rules.clone())?;
            ()
        },
        _ => {
            Error::addSourceMessage(Error::INVALID_CONVERSION_RULE.clone(), list![(List::toString(args.clone(), Arc::new(Dump::printExpStr), (literal!("convertClass")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rules)
}

fn parseConvertClassStr(mut oldName: ArcStr, mut newName: ArcStr, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    let mut old_path: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut rule: ConversionRule = ConversionRule::CLASS_IF;
    old_path = parsePathList((oldName.clone()).clone());
    rule = ConversionRule::CLASS { oldPath: metamodelica::arrayFromVec(old_path.clone().into_iter().cloned().collect()), newPath: parsePath((newName.clone()).clone()) };
    rules = addRule(old_path.clone(), rule.clone(), rules.clone())?;
    Ok(rules)
}

fn parseConvertClassIf(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut info: SourceInfo, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Conversion.parseConvertClassIf")); __mm_s.push_str(&*literal!(": not implemented")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
    Ok(rules)
}

fn parseConvertElement(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut info: SourceInfo, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: cls_name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: old_name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: new_name }, tail: Deref @ metamodelica::List::Nil } } } => {
            let mut old_path: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut rule: ConversionRule = ConversionRule::CLASS_IF;
            old_path = parsePathList((cls_name.clone()).clone());
            rule = ConversionRule::ELEMENT { oldPath: metamodelica::arrayFromVec(old_path.clone().into_iter().cloned().collect()), oldName: (old_name.clone()).clone(), newName: (new_name.clone()).clone() };
            rules = addRule(old_path.clone(), rule.clone(), rules.clone())?;
            ()
        },
        _ => {
            Error::addSourceMessage(Error::INVALID_CONVERSION_RULE.clone(), list![(List::toString(args.clone(), Arc::new(Dump::printExpStr), (literal!("convertElement")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rules)
}

fn parseConvertModifiers(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut info: SourceInfo, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    rules = 'mc: {
        let __mc_input = args.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: cls_name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: old_mods }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: new_mods }, tail: Deref @ metamodelica::List::Nil } } } => {
                    Ok(parseConvertModifiers2((cls_name.clone()).clone(), old_mods.clone(), new_mods.clone(), false, info.clone(), rules.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: cls_name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: old_mods }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: new_mods }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::BOOL { value: simplify }, tail: Deref @ metamodelica::List::Nil } } } } => {
                    Ok(parseConvertModifiers2((cls_name.clone()).clone(), old_mods.clone(), new_mods.clone(), simplify.clone(), info.clone(), rules.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INVALID_CONVERSION_RULE.clone(), list![(List::toString(args.clone(), Arc::new(Dump::printExpStr), (literal!("convertModifiers")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(rules)
}

fn parseConvertModifiers2(mut className: ArcStr, mut oldMods: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut newMods: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut simplify: bool, mut info: SourceInfo, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    let mut cls_path: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut old_mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut new_mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    cls_path = parsePathList((className.clone()).clone());
    old_mods = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut m in (oldMods.clone()).into_iter().cloned() {
            let __x = parseModifier(m.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    new_mods = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut m in (newMods.clone()).into_iter().cloned() {
            let __x = parseModifier(m.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    rules = addRule(cls_path.clone(), ConversionRule::MODIFIERS { oldMods: old_mods.clone(), newMods: new_mods.clone(), info: info.clone() }, rules.clone())?;
    Ok(rules)
}

fn parseModifier(mut r#mod: Arc<Absyn::Exp>, mut info: SourceInfo) -> Result<Arc<Absyn::ElementArg>> {
    let mut outMod: Arc<Absyn::ElementArg>;
    let mut r#str: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ Absyn::Exp::STRING { value: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#str = __pa0.clone();
    outMod = Parser::stringMod((quotePlaceholders((r#str.clone()).clone(), info.clone())?).clone(), (literal!("<internal>")).clone())?;
    Ok(outMod)
}

fn quotePlaceholders(mut r#str: ArcStr, mut info: SourceInfo) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut in_ident: bool = false;
    strl = System::strtokIncludingDelimiters((r#str.clone()).clone(), (literal!("%")).clone());
    if (strl.clone().len() as i32) <= 1 {
        return Ok(r#str);
    }
    for mut s in &*strl.clone() {
        let mut s = s.clone();
        if s.clone() == literal!("%") {
            s = (if (in_ident.clone()) {literal!("%'")} else {literal!("'%")}).clone();
            in_ident = !(in_ident.clone());
        }
        res = cons(s.clone(), res.clone());
    }
    if in_ident.clone() {
        Error::addSourceMessage(Error::CONVERSION_MISMATCHED_PLACEHOLDER.clone(), list![(r#str.clone()).clone()], info.clone())?;
        bail!("fail");
    }
    r#str = stringAppendList(res.clone().reverse());
    Ok(r#str)
}

fn parseConvertMessage(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut info: SourceInfo, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: cls_name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: msg }, tail: Deref @ metamodelica::List::Nil } } => {
            let mut rule: ConversionRule = ConversionRule::CLASS_IF;
            rule = ConversionRule::MESSAGE { message: (msg.clone()).clone() };
            rules = addRule(parsePathList((cls_name.clone()).clone()), rule.clone(), rules.clone())?;
            ()
        },
        _ => {
            Error::addSourceMessage(Error::INVALID_CONVERSION_RULE.clone(), list![(List::toString(args.clone(), Arc::new(Dump::printExpStr), (literal!("convertMessage")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rules)
}

fn parsePath(mut r#str: ArcStr) -> Arc<Path> {
    let mut path: Arc<Path> = AbsynUtil::stringPath((r#str.clone()).clone()).unwrap();
    path
}

fn parsePathList(mut r#str: ArcStr) -> Arc<metamodelica::List<ArcStr>> {
    let mut path: Arc<metamodelica::List<ArcStr>> = Util::stringSplitAtChar((r#str.clone()).clone(), (literal!(".")).clone()).unwrap();
    path
}

fn addRule(mut path: Arc<metamodelica::List<ArcStr>>, mut rule: ConversionRule, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut rules: Arc<ConversionRules::ConversionRules> = rules;
    updateNode(Some(rules.clone()), path.clone(), rule.clone())?;
    Ok(rules)
}

fn updateNode(mut onode: Option<Arc<ConversionRules::ConversionRules>>, mut path: Arc<metamodelica::List<ArcStr>>, mut rule: ConversionRule) -> Result<Arc<ConversionRules::ConversionRules>> {
    let mut node: Arc<ConversionRules::ConversionRules>;
    if isSome(onode.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(onode.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        node = __pa0.clone();
    } else {
        node = ConversionRules::newNode();
    }
    if path.clone().is_empty() {
        assign_field!(node.rules = cons(rule.clone(), node.rules.clone()));
    } else {
        UnorderedMap::addUpdate((listHead(path.clone())?).clone(), Arc::new({ let __pe_b1 = listRest(path.clone())?; let __pe_b2 = rule.clone(); move |__pe_a0| updateNode(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), node.nodes.clone())?;
    }
    Ok(node)
}

fn lookupRuleNode(mut path: Arc<Path>, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Option<Arc<ConversionRules::ConversionRules>>> {
    let mut outNode: Option<Arc<ConversionRules::ConversionRules>> = None;
    let mut node: Arc<ConversionRules::ConversionRules> = rules.clone();
    for mut name in &*AbsynUtil::pathToStringList(path.clone())? {
        let mut name = name.clone();
        outNode = UnorderedMap::get((name.clone()).clone(), node.nodes.clone());
        if isNone(outNode.clone()) {
            return Ok(outNode);
        }
        let __pa0 = ::match_deref::match_deref! { match &(outNode.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        node = __pa0.clone();
    }
    Ok(outNode)
}

fn lookupRules(mut path: Arc<Path>, mut rules: Arc<ConversionRules::ConversionRules>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ConversionRule>>>>> {
    let mut outRules: Arc<metamodelica::List<Arc<metamodelica::List<ConversionRule>>>> = metamodelica::nil();
    let mut onode: Option<Arc<ConversionRules::ConversionRules>> = None;
    let mut node: Arc<ConversionRules::ConversionRules> = rules.clone();
    for mut name in &*AbsynUtil::pathToStringList(path.clone())? {
        let mut name = name.clone();
        onode = UnorderedMap::get((name.clone()).clone(), node.nodes.clone());
        if isNone(onode.clone()) {
            outRules = cons(metamodelica::nil(), outRules.clone());
            return Ok(outRules);
        }
        let __pa0 = ::match_deref::match_deref! { match &(onode.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        node = __pa0.clone();
        if !(node.rules.clone().is_empty()) {
            outRules = cons(node.rules.clone(), outRules.clone());
        }
    }
    Ok(outRules)
}

fn lookupTypeRules(mut typePath: Arc<Path>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<(Option<ConversionRule>, RuleTable, Arc<metamodelica::List<ConversionRule>>)> {
    let mut typeRule: Option<ConversionRule> = None;
    let mut localRules: RuleTable = newRuleTable();
    let mut modifierRules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
    let mut found_rules: Arc<metamodelica::List<Arc<metamodelica::List<ConversionRule>>>> = metamodelica::nil();
    found_rules = lookupRules(typePath.clone(), rules.clone())?;
    if found_rules.clone().is_empty() {
        return Ok((typeRule, localRules, modifierRules));
    }
    modifierRules = sortLocalRules(listHead(found_rules.clone())?, localRules.clone())?;
    for mut rl in &*found_rules.clone() {
        let mut rl = rl.clone();
        for mut r in &*rl.clone() {
            let mut r = r.clone();
            let () = (match r.clone() {
        ConversionRule::CLASS { .. } => {
            if isNone(typeRule.clone()) {
                typeRule = Some(r.clone());
            }
            ()
        },
        _ => (),
    });
        }
    }
    Ok((typeRule, localRules, modifierRules))
}

fn newRuleTable() -> RuleTable {
    let mut table: RuleTable;
    table = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), stringEq, 1);
    table
}

fn newTypeTable() -> TypeTable {
    let mut table: TypeTable;
    table = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), stringEq, 1);
    table
}

fn newEnv() -> Env {
    let mut env: Env = Env { components: newTypeTable(), imports: ImportTreeImpl::new() };
    env
}

fn sortLocalRules(mut rules: Arc<metamodelica::List<ConversionRule>>, mut localRules: RuleTable) -> Result<Arc<metamodelica::List<ConversionRule>>> {
    let mut modifierRules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
    for mut rule in &*rules.clone() {
        let mut rule = rule.clone();
        let () = (match rule.clone() {
        ConversionRule::ELEMENT { .. } => {
            UnorderedMap::addUpdate((var_field!(rule.oldName, ConversionRule::ELEMENT).clone()).clone(), Arc::new({ let __pe_b1 = rule.clone(); move |__pe_a0| mergeRuleList(__pe_a0, __pe_b1.clone()) }), localRules.clone())?;
            ()
        },
        ConversionRule::MODIFIERS { .. } => {
            modifierRules = cons(rule.clone(), modifierRules.clone());
            ()
        },
        _ => (),
    });
    }
    Ok(modifierRules)
}

fn mergeRuleList(mut oldRules: Option<Arc<metamodelica::List<ConversionRule>>>, mut newRule: ConversionRule) -> Result<Arc<metamodelica::List<ConversionRule>>> {
    let mut outRules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
    if isNone(oldRules.clone()) {
        outRules = list![newRule.clone()];
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(oldRules.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        outRules = __pa0.clone();
        outRules = cons(newRule.clone(), outRules.clone());
    }
    Ok(outRules)
}

fn lookupClassExtendsRules(mut name: ArcStr, mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>) -> Result<(RuleTable, Arc<metamodelica::List<ConversionRule>>)> {
    let mut localRules: RuleTable = newRuleTable();
    let mut modificationRules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
    let mut onode: Option<Arc<ConversionRules::ConversionRules>> = None;
    let mut node: Arc<ConversionRules::ConversionRules>;
    for mut ext in &*extendsRules.clone() {
        let mut ext = ext.clone();
        onode = UnorderedMap::get((name.clone()).clone(), ext.nodes.clone());
        if isSome(onode.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(onode.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            modificationRules = sortLocalRules(node.rules.clone(), localRules.clone())?;
            return Ok((localRules, modificationRules));
        }
    }
    Ok((localRules, modificationRules))
}

fn dumpRules(mut rules: Arc<ConversionRules::ConversionRules>, mut indent: ArcStr) -> Result<()> {
    let mut keys: metamodelica::Array<ArcStr>;
    let mut values: metamodelica::Array<Arc<ConversionRules::ConversionRules>>;
    let mut rule: ConversionRule = ConversionRule::CLASS_IF;
    let mut rest_rules: Arc<metamodelica::List<ConversionRule>> = rules.rules.clone();
    keys = UnorderedMap::keyArray(rules.nodes.clone());
    values = UnorderedMap::valueArray(rules.nodes.clone());
    while !(rest_rules.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_rules.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        rule = __pa0.clone();
        rest_rules = __pa1.clone();
        if rest_rules.clone().is_empty() && keys.clone().borrow().is_empty() {
            dumpRule(rule.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("└─")); ArcStr::from(__mm_s) }).clone())?;
        } else {
            dumpRule(rule.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("├─")); ArcStr::from(__mm_s) }).clone())?;
        }
    }
    let __range2 = 1..=(keys.clone().borrow().len() as i32);
    for mut i in __range2 {
        if i.clone() == (keys.clone().borrow().len() as i32) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("└─")); ArcStr::from(__mm_s) }).clone());
            println!("{}", (keys.borrow()[(i.clone()-1) as usize].clone()).clone());
            println!("{}", (literal!("\n")).clone());
            dumpRules(values.borrow()[(i.clone()-1) as usize].clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?;
        } else {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("├─")); ArcStr::from(__mm_s) }).clone());
            println!("{}", (keys.borrow()[(i.clone()-1) as usize].clone()).clone());
            println!("{}", (literal!("\n")).clone());
            dumpRules(values.borrow()[(i.clone()-1) as usize].clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("│ ")); ArcStr::from(__mm_s) }).clone())?;
        }
    }
    Ok(())
}

fn dumpRule(mut rule: ConversionRule, mut indent: ArcStr) -> Result<()> {
    println!("{}", (indent.clone()).clone());
    let () = (match rule.clone() {
        ConversionRule::CLASS { .. } => {
            println!("{}", (literal!("convertClass: ")).clone());
            println!("{}", (AbsynUtil::pathString(var_field!(rule.newPath, ConversionRule::CLASS).clone(), (literal!(".")).clone(), true, false)?).clone());
            ()
        },
        ConversionRule::CLASS_IF => {
            println!("{}", (literal!("convertClassIf: ")).clone());
            ()
        },
        ConversionRule::ELEMENT { .. } => {
            println!("{}", (literal!("convertElement: ")).clone());
            println!("{}", (var_field!(rule.oldName, ConversionRule::ELEMENT).clone()).clone());
            println!("{}", (literal!(" => ")).clone());
            println!("{}", (var_field!(rule.newName, ConversionRule::ELEMENT).clone()).clone());
            ()
        },
        ConversionRule::MODIFIERS { .. } => {
            println!("{}", (literal!("convertModifiers: ")).clone());
            println!("{}", (List::toString(var_field!(rule.oldMods, ConversionRule::MODIFIERS).clone(), Arc::new(Dump::unparseElementArgStr), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?).clone());
            println!("{}", (literal!(" => ")).clone());
            println!("{}", (List::toString(var_field!(rule.newMods, ConversionRule::MODIFIERS).clone(), Arc::new(Dump::unparseElementArgStr), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?).clone());
            ()
        },
        ConversionRule::MESSAGE { .. } => {
            println!("{}", (literal!("convertMessage: \"")).clone());
            println!("{}", (var_field!(rule.message, ConversionRule::MESSAGE).clone()).clone());
            println!("{}", (literal!("\"")).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn convertProgram(mut program: Absyn::Program, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Absyn::Program> {
    let mut program: Absyn::Program = program;
    program.classes = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
        for mut c in (program.classes.clone()).into_iter().cloned() {
            let __x = convertClass(c.clone(), rules.clone(), env.clone(), metamodelica::nil())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(program)
}

fn convertClass(mut cls: Arc<Absyn::Class>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    assign_field!(cls.body = convertClassDef(cls.body.clone(), rules.clone(), env.clone(), extendsRules.clone(), cls.info.clone())?);
    Ok(cls)
}

fn convertClassDef(mut cdef: Arc<Absyn::ClassDef>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>, mut info: SourceInfo) -> Result<Arc<Absyn::ClassDef>> {
    let mut cdef: Arc<Absyn::ClassDef> = cdef;
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = convertClassParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), newRuleTable(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            let mut local_rules: RuleTable;
            let mut mod_rules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
            let mut ty: Arc<Absyn::TypeSpec>;
            (ty, local_rules, mod_rules) = convertTypeSpec(var_field!((*cdef).typeSpec, Absyn::ClassDef::DERIVED).clone(), rules.clone(), env.clone(), info.clone())?;
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED;
                typeSpec = ty.clone(),
                arguments = convertModification2(mod_rules.clone(), var_field!((*cdef).arguments, Absyn::ClassDef::DERIVED).clone())?,
                arguments = convertElementArgs(var_field!((*cdef).arguments, Absyn::ClassDef::DERIVED).clone(), local_rules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            let mut local_rules: RuleTable;
            let mut mod_rules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
            (local_rules, mod_rules) = lookupClassExtendsRules((var_field!((*cdef).baseClassName, Absyn::ClassDef::CLASS_EXTENDS).clone()).clone(), extendsRules.clone())?;
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS;
                modifications = convertModification2(mod_rules.clone(), var_field!((*cdef).modifications, Absyn::ClassDef::CLASS_EXTENDS).clone())?,
                modifications = convertElementArgs(var_field!((*cdef).modifications, Absyn::ClassDef::CLASS_EXTENDS).clone(), local_rules.clone(), rules.clone(), env.clone())?,
                parts = convertClassParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), local_rules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cdef)
}

fn convertClassParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = parts;
    let mut extends_rules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>> = metamodelica::nil();
    let mut imps: ImportTree = Arc::new(ImportTreeImpl::Tree::EMPTY);
    let mut cls_env: Env;
    cls_env = addImportNamesToEnv(getImportsInParts(parts.clone()), rules.clone(), env.clone())?;
    addComponentTypesToEnv(parts.clone(), env.components.clone())?;
    cls_env.imports = shadowImportsInParts(parts.clone(), cls_env.imports.clone())?;
    extends_rules = getExtendsRules(parts.clone(), rules.clone(), cls_env.clone())?;
    parts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (parts.clone()).into_iter().cloned() {
            let __x = convertClassPart(p.clone(), localRules.clone(), rules.clone(), cls_env.clone(), extends_rules.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(parts)
}

fn convertClassPart(mut part: Arc<Absyn::ClassPart>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>, mut info: SourceInfo) -> Result<Arc<Absyn::ClassPart>> {
    let mut part: Arc<Absyn::ClassPart> = part;
    let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = convertElementItems(var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone(), rules.clone(), env.clone(), extendsRules.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = convertElementItems(var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone(), rules.clone(), env.clone(), extendsRules.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = convertEquationItems(var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone(), localRules.clone(), rules.clone(), env.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = convertEquationItems(var_field!((*part).contents, Absyn::ClassPart::INITIALEQUATIONS).clone(), localRules.clone(), rules.clone(), env.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::ALGORITHMS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::ALGORITHMS; contents = convertAlgorithmItems(var_field!((*part).contents, Absyn::ClassPart::ALGORITHMS).clone(), localRules.clone(), rules.clone(), env.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALALGORITHMS; contents = convertAlgorithmItems(var_field!((*part).contents, Absyn::ClassPart::INITIALALGORITHMS).clone(), localRules.clone(), rules.clone(), env.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::EXTERNAL { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EXTERNAL; externalDecl = convertExternalDecl(var_field!((*part).externalDecl, Absyn::ClassPart::EXTERNAL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(part)
}

fn convertElementArgs(mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = args;
    args = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (args.clone()).into_iter().cloned() {
            let __x = convertElementArg(a.clone(), localRules.clone(), rules.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(args)
}

fn convertElementArg(mut arg: Arc<Absyn::ElementArg>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Arc<Absyn::ElementArg>> {
    let mut arg: Arc<Absyn::ElementArg> = arg;
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            let mut mod_rules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
            mod_rules = UnorderedMap::getOrDefault((AbsynUtil::pathString(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), (literal!(".")).clone(), true, false)?).clone(), localRules.clone(), metamodelica::nil());
            for mut rule in &*mod_rules.clone() {
                let mut rule = rule.clone();
                let () = (match rule.clone() {
        ConversionRule::ELEMENT { .. } => {
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; path = Arc::new(Path::IDENT { name: (var_field!(rule.newName, ConversionRule::ELEMENT).clone()).clone() }));
            ()
        },
        _ => (),
    });
            }
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = convertModificationExps(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone(), localRules.clone(), rules.clone(), env.clone(), var_field!((*arg).info, Absyn::ElementArg::MODIFICATION).clone()));
            ()
        },
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => {
            assign_variant_field!(arg => Absyn::ElementArg::REDECLARATION;
                elementSpec = convertElementSpec(var_field!((*arg).elementSpec, Absyn::ElementArg::REDECLARATION).clone(), rules.clone(), env.clone(), metamodelica::nil(), var_field!((*arg).info, Absyn::ElementArg::REDECLARATION).clone())?,
                constrainClass = convertOption(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone(), Arc::new(convertConstrainClass), rules.clone(), env.clone(), var_field!((*arg).info, Absyn::ElementArg::REDECLARATION).clone())
            );
            ()
        },
        Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. } => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(arg)
}

fn convertModificationExps(mut r#mod: Option<Arc<Absyn::Modification>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Option<Arc<Absyn::Modification>> {
    let mut r#mod: Option<Arc<Absyn::Modification>> = r#mod;
    r#mod = convertOption(r#mod.clone(), Arc::new({ let __pe_b1 = localRules.clone(); move |__pe_a0, __pe_a2, __pe_a3, __pe_a4| convertModificationExps2(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3, __pe_a4) }), rules.clone(), env.clone(), info.clone());
    r#mod
}

fn convertModificationExps2(mut r#mod: Arc<Absyn::Modification>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::Modification>> {
    let mut r#mod: Arc<Absyn::Modification> = r#mod;
    assign_field!(
        r#mod.elementArgLst = convertElementArgs(r#mod.elementArgLst.clone(), localRules.clone(), rules.clone(), env.clone())?,
        r#mod.eqMod = convertEqMod(r#mod.eqMod.clone(), localRules.clone(), rules.clone(), env.clone())?
    );
    Ok(r#mod)
}

fn convertEqMod(mut r#mod: Arc<Absyn::EqMod>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Arc<Absyn::EqMod>> {
    let mut r#mod: Arc<Absyn::EqMod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => {
            assign_variant_field!(r#mod => Absyn::EqMod::EQMOD; exp = convertExp(var_field!((*r#mod).exp, Absyn::EqMod::EQMOD).clone(), localRules.clone(), rules.clone(), env.clone(), var_field!((*r#mod).info, Absyn::EqMod::EQMOD).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

fn convertModification(mut r#mod: Option<Arc<Absyn::Modification>>, mut modifierRules: Arc<metamodelica::List<ConversionRule>>) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut r#mod: Option<Arc<Absyn::Modification>> = r#mod;
    let mut elem_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut eq_mod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    if isSome(r#mod.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(r#mod.clone()) {
            Some(Deref @ Absyn::Modification { elementArgLst: __pa0, eqMod: __pa1 }) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        elem_args = __pa0.clone();
        eq_mod = __pa1.clone();
    } else {
        elem_args = metamodelica::nil();
        eq_mod = Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD);
    }
    elem_args = convertModification2(modifierRules.clone(), elem_args.clone())?;
    r#mod = (::match_deref::match_deref! { match &((elem_args.clone(), eq_mod.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ Absyn::EqMod::NOMOD { .. }) => None,
        _ => Some(Arc::new(Absyn::Modification { elementArgLst: elem_args.clone(), eqMod: eq_mod.clone() })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

fn convertModification2(mut modifierRules: Arc<metamodelica::List<ConversionRule>>, mut elemArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut elemArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = elemArgs;
    for mut rule in &*modifierRules.clone() {
        let mut rule = rule.clone();
        elemArgs = convertModifier(rule.clone(), elemArgs.clone())?;
    }
    Ok(elemArgs)
}

fn convertModifier(mut rule: ConversionRule, mut elemArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut elemArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = elemArgs;
    let mut old_mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut new_mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut matching_mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut rest_mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut placeholders: Arc<UnorderedMap::UnorderedMap<ArcStr, Option<Arc<Absyn::Exp>>>>;
    let mut info: SourceInfo;
    let ConversionRule::MODIFIERS { info: __pa0, newMods: __pa1, oldMods: __pa2 } = (rule.clone()) else { bail!("pattern mismatch") };
    info = __pa0.clone();
    new_mods = __pa1.clone();
    old_mods = __pa2.clone();
    if old_mods.clone().is_empty() {
        elemArgs = mergeModifiers(elemArgs.clone(), new_mods.clone());
    } else {
        (matching_mods, rest_mods) = List::splitOnTrue(elemArgs.clone(), Arc::new({ let __pe_b1 = old_mods.clone(); move |__pe_a0| Ok(isModifierInList(__pe_a0, __pe_b1.clone())) }));
        if !(matching_mods.clone().is_empty()) {
            placeholders = makePlaceholderTable(listAppend(old_mods.clone(), matching_mods.clone()))?;
            new_mods = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut m in (new_mods.clone()).into_iter().cloned() {
            let __x = replacePlaceholders(m.clone(), placeholders.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            elemArgs = mergeModifiers(rest_mods.clone(), new_mods.clone());
        }
    }
    Ok(elemArgs)
}

fn isModifierInList(mut r#mod: Arc<Absyn::ElementArg>, mut mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> bool {
    let mut res: bool = List::any(mods.clone(), Arc::new({ let __pe_b1 = r#mod.clone(); move |__pe_a0| Ok(isEqualNameMod(__pe_a0, __pe_b1.clone())) }));
    res
}

fn isEqualNameMod(mut mod1: Arc<Absyn::ElementArg>, mut mod2: Arc<Absyn::ElementArg>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ Absyn::ElementArg::MODIFICATION { .. }, Deref @ Absyn::ElementArg::MODIFICATION { .. }) => AbsynUtil::pathEqual(var_field!((*mod1).path, Absyn::ElementArg::MODIFICATION).clone(), var_field!((*mod2).path, Absyn::ElementArg::MODIFICATION).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn makePlaceholderTable(mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<UnorderedMap::UnorderedMap<ArcStr, Option<Arc<Absyn::Exp>>>>> {
    let mut placeholders: Arc<UnorderedMap::UnorderedMap<ArcStr, Option<Arc<Absyn::Exp>>>>;
    placeholders = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), stringEq, 1);
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        UnorderedMap::add((AbsynUtil::pathString(AbsynUtil::elementArgName(arg.clone())?, (literal!(".")).clone(), true, false)?).clone(), getElementArgBinding(arg.clone()), placeholders.clone())?;
    }
    Ok(placeholders)
}

fn getElementArgBinding(mut arg: Arc<Absyn::ElementArg>) -> Option<Arc<Absyn::Exp>> {
    let mut exp: Option<Arc<Absyn::Exp>> = None;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    exp = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }), .. } => Some(e.clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

fn replacePlaceholders(mut arg: Arc<Absyn::ElementArg>, mut placeholders: Arc<UnorderedMap::UnorderedMap<ArcStr, Option<Arc<Absyn::Exp>>>>, mut info: SourceInfo) -> Result<Arc<Absyn::ElementArg>> {
    let mut arg: Arc<Absyn::ElementArg> = arg;
    let mut r#mod: Arc<Absyn::Modification>;
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut eq_mod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(r#mod), .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(r#mod.clone()) {
                Deref @ Absyn::Modification { elementArgLst: __pa0, eqMod: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            args = __pa0.clone();
            eq_mod = __pa1.clone();
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (args.clone()).into_iter().cloned() {
            let __x = replacePlaceholders(a.clone(), placeholders.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            eq_mod = replacePlaceholdersEqMod(eq_mod.clone(), placeholders.clone(), list![var_field!((*arg).info, Absyn::ElementArg::MODIFICATION).clone(), info.clone()])?;
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eq_mod.clone() })));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

fn replacePlaceholdersEqMod(mut eqMod: Arc<Absyn::EqMod>, mut placeholders: Arc<UnorderedMap::UnorderedMap<ArcStr, Option<Arc<Absyn::Exp>>>>, mut info: Arc<metamodelica::List<SourceInfo>>) -> Result<Arc<Absyn::EqMod>> {
    let mut eqMod: Arc<Absyn::EqMod> = eqMod;
    let () = (::match_deref::match_deref! { match &(eqMod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => {
            assign_variant_field!(eqMod => Absyn::EqMod::EQMOD; exp = AbsynUtil::traverseExp(var_field!((*eqMod).exp, Absyn::EqMod::EQMOD).clone(), Arc::new({ let __pe_b2 = info.clone(); move |__pe_a0, __pe_a1| replacePlaceholdersExp(__pe_a0, __pe_a1, __pe_b2.clone()) }), placeholders.clone())?.0);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqMod)
}

fn replacePlaceholdersExp(mut exp: Arc<Absyn::Exp>, mut placeholders: Arc<UnorderedMap::UnorderedMap<ArcStr, Option<Arc<Absyn::Exp>>>>, mut info: Arc<metamodelica::List<SourceInfo>>) -> Result<(Arc<Absyn::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, Option<Arc<Absyn::Exp>>>>)> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outPlaceholders: Arc<UnorderedMap::UnorderedMap<ArcStr, Option<Arc<Absyn::Exp>>>> = placeholders.clone();
    let mut name: ArcStr = arcstr::literal!("");
    let mut len: i32 = 0;
    let mut new_exp: Option<Arc<Absyn::Exp>> = None;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name } } => {
            let mut name = (*name).clone();
            len = ((name.clone()).clone().len() as i32);
            if len.clone() > 4 && stringGet((name.clone()).clone(),1)? == 39 && stringGet((name.clone()).clone(),2)? == 37 && stringGet((name.clone()).clone(),len.clone() - 1)? == 37 && stringGet((name.clone()).clone(),len.clone())? == 39 {
                name = substring((name.clone()).clone(), 3, len.clone() - 2)?;
                new_exp = UnorderedMap::getOrDefault((name.clone()).clone(), placeholders.clone(), None);
                if isNone(new_exp.clone()) {
                    Error::addMultiSourceMessage(Error::CONVERSION_MISSING_PLACEHOLDER_VALUE.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("%")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                }
                let __pa0 = ::match_deref::match_deref! { match &(new_exp.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                outExp = __pa0.clone();
            } else {
                outExp = exp.clone();
            }
            outExp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outPlaceholders))
}

fn mergeModifiers(mut outerMods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut innerMods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementArg>>> {
    let mut mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = outerMods.clone();
    for mut m in &*innerMods.clone().reverse() {
        let mut m = m.clone();
        if !(isModifierInList(m.clone(), outerMods.clone())) {
            mods = cons(m.clone(), mods.clone());
        }
    }
    mods
}

fn convertTypeSpec(mut ty: Arc<Absyn::TypeSpec>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<(Arc<Absyn::TypeSpec>, RuleTable, Arc<metamodelica::List<ConversionRule>>)> {
    let mut ty: Arc<Absyn::TypeSpec> = ty;
    let mut localRules: RuleTable;
    let mut modifierRules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
    let mut ty_rule: Option<ConversionRule> = None;
    let mut ty_path: Arc<Path>;
    let mut import_path: Option<(Arc<Path>, ArcStr)> = None;
    (ty_path, import_path) = applyImportsToPath(AbsynUtil::typeSpecPath(ty.clone())?, env.imports.clone())?;
    (ty_rule, localRules, modifierRules) = lookupTypeRules(ty_path.clone(), rules.clone(), env.clone())?;
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { .. } => {
            if isSome(ty_rule.clone()) {
                assign_variant_field!(ty => Absyn::TypeSpec::TPATH; path = convertTypePath(ty_path.clone(), Util::getOption(ty_rule.clone())?, import_path.clone(), info.clone())?);
            }
            assign_variant_field!(ty => Absyn::TypeSpec::TPATH; arrayDim = convertOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone(), Arc::new({ let __pe_b1 = localRules.clone(); move |__pe_a0, __pe_a2, __pe_a3, __pe_a4| convertSubscripts(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3, __pe_a4) }), rules.clone(), env.clone(), info.clone()));
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { .. } => {
            if isSome(ty_rule.clone()) {
                assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX; path = convertTypePath(ty_path.clone(), Util::getOption(ty_rule.clone())?, import_path.clone(), info.clone())?);
            }
            assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX;
                typeSpecs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>> = metamodelica::nil();
        for mut t in (var_field!((*ty).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone()).into_iter().cloned() {
            let __x = (convertTypeSpec(t.clone(), rules.clone(), env.clone(), info.clone())?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                arrayDim = convertOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone(), Arc::new({ let __pe_b1 = localRules.clone(); move |__pe_a0, __pe_a2, __pe_a3, __pe_a4| convertSubscripts(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3, __pe_a4) }), rules.clone(), env.clone(), info.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ty, localRules, modifierRules))
}

fn convertTypePath(mut path: Arc<Path>, mut rule: ConversionRule, mut importPath: Option<(Arc<Path>, ArcStr)>, mut info: SourceInfo) -> Result<Arc<Path>> {
    let mut path: Arc<Path> = path;
    let () = (match rule.clone() {
        ConversionRule::CLASS { .. } => {
            if AbsynUtil::pathPartCount(path.clone(), 0)? == (var_field!(rule.oldPath, ConversionRule::CLASS).clone().borrow().len() as i32) {
                path = var_field!(rule.newPath, ConversionRule::CLASS).clone();
            } else {
                path = Util::foldcallN((var_field!(rule.oldPath, ConversionRule::CLASS).clone().borrow().len() as i32), Arc::new(AbsynUtil::pathRest), path.clone());
                path = AbsynUtil::joinPaths(var_field!(rule.newPath, ConversionRule::CLASS).clone(), path.clone())?;
            }
            ()
        },
        ConversionRule::MESSAGE { .. } => {
            Error::addSourceMessage(Error::CONVERSION_MESSAGE.clone(), list![(var_field!(rule.message, ConversionRule::MESSAGE).clone()).clone()], info.clone())?;
            ()
        },
        _ => (),
    });
    path = stripImportPath(path.clone(), importPath.clone())?;
    Ok(path)
}

fn convertElementItems(mut elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = elements;
    elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut e in (elements.clone()).into_iter().cloned() {
            let __x = convertElementItem(e.clone(), rules.clone(), env.clone(), extendsRules.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    elements = filterDuplicateImports(elements.clone())?;
    Ok(elements)
}

fn convertElementItem(mut element: Arc<Absyn::ElementItem>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>) -> Result<Arc<Absyn::ElementItem>> {
    let mut element: Arc<Absyn::ElementItem> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => {
            assign_variant_field!(element => Absyn::ElementItem::ELEMENTITEM; element = convertElement(var_field!((*element).element, Absyn::ElementItem::ELEMENTITEM).clone(), rules.clone(), env.clone(), extendsRules.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

fn convertElement(mut element: Arc<Absyn::Element>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT;
                specification = convertElementSpec(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), rules.clone(), env.clone(), extendsRules.clone(), var_field!((*element).info, Absyn::Element::ELEMENT).clone())?,
                constrainClass = convertOption(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone(), Arc::new(convertConstrainClass), rules.clone(), env.clone(), var_field!((*element).info, Absyn::Element::ELEMENT).clone())
            );
            ()
        },
        Deref @ Absyn::Element::DEFINEUNIT { .. } => {
            let mut local_rules: RuleTable;
            local_rules = newRuleTable();
            assign_variant_field!(element => Absyn::Element::DEFINEUNIT; args = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
        for mut a in (var_field!((*element).args, Absyn::Element::DEFINEUNIT).clone()).into_iter().cloned() {
            let __x = convertNamedArg(a.clone(), local_rules.clone(), rules.clone(), env.clone(), var_field!((*element).info, Absyn::Element::DEFINEUNIT).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

fn convertConstrainClass(mut cc: Arc<Absyn::ConstrainClass>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::ConstrainClass>> {
    let mut cc: Arc<Absyn::ConstrainClass> = cc;
    assign_field!(cc.elementSpec = convertElementSpec(cc.elementSpec.clone(), rules.clone(), env.clone(), metamodelica::nil(), info.clone())?);
    Ok(cc)
}

fn convertElementSpec(mut spec: Arc<Absyn::ElementSpec>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>, mut info: SourceInfo) -> Result<Arc<Absyn::ElementSpec>> {
    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let () = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = convertClass(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), rules.clone(), env.clone(), extendsRules.clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::EXTENDS { .. } => {
            let mut local_rules: RuleTable;
            let mut mod_rules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
            let mut ty_path: Arc<Path>;
            let mut import_path: Option<(Arc<Path>, ArcStr)> = None;
            (ty_path, import_path) = applyImportsToPath(var_field!((*spec).path, Absyn::ElementSpec::EXTENDS).clone(), env.imports.clone())?;
            (_, local_rules, mod_rules) = lookupTypeRules(ty_path.clone(), rules.clone(), env.clone())?;
            ty_path = convertPath(ty_path.clone(), rules.clone(), env.imports.clone(), info.clone())?;
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS;
                path = stripImportPath(ty_path.clone(), import_path.clone())?,
                elementArg = convertModification2(mod_rules.clone(), var_field!((*spec).elementArg, Absyn::ElementSpec::EXTENDS).clone())?,
                elementArg = convertElementArgs(var_field!((*spec).elementArg, Absyn::ElementSpec::EXTENDS).clone(), local_rules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::ElementSpec::IMPORT { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::IMPORT; import_ = convertImport(var_field!((*spec).import_, Absyn::ElementSpec::IMPORT).clone(), rules.clone(), info.clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            let mut local_rules: RuleTable;
            let mut mod_rules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
            let mut ty: Arc<Absyn::TypeSpec>;
            (ty, local_rules, mod_rules) = convertTypeSpec(var_field!((*spec).typeSpec, Absyn::ElementSpec::COMPONENTS).clone(), rules.clone(), env.clone(), info.clone())?;
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS;
                typeSpec = ty.clone(),
                components = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
        for mut c in (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
            let __x = convertComponentItem(c.clone(), local_rules.clone(), mod_rules.clone(), rules.clone(), env.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(spec)
}

fn convertImport(mut imp: Absyn::Import, mut rules: Arc<ConversionRules::ConversionRules>, mut info: SourceInfo) -> Result<Absyn::Import> {
    let mut imp: Absyn::Import = imp;
    let () = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => {
            let __owned_variant_path_0 = convertPath(var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone(), rules.clone(), ImportTreeImpl::new(), info.clone())?;
            if let Absyn::Import::NAMED_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::NAMED_IMPORT"); }
            ()
        },
        Absyn::Import::QUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = convertPath(var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(), rules.clone(), ImportTreeImpl::new(), info.clone())?;
            if let Absyn::Import::QUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::QUAL_IMPORT"); }
            ()
        },
        Absyn::Import::UNQUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = convertPath(var_field!(imp.path, Absyn::Import::UNQUAL_IMPORT).clone(), rules.clone(), ImportTreeImpl::new(), info.clone())?;
            if let Absyn::Import::UNQUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::UNQUAL_IMPORT"); }
            ()
        },
        Absyn::Import::GROUP_IMPORT { .. } => {
            let __owned_variant_prefix_0 = convertPath(var_field!(imp.prefix, Absyn::Import::GROUP_IMPORT).clone(), rules.clone(), ImportTreeImpl::new(), info.clone())?;
            if let Absyn::Import::GROUP_IMPORT { prefix, .. } = &mut imp {
                *prefix = __owned_variant_prefix_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::GROUP_IMPORT"); }
            ()
        },
        _ => (),
    });
    imp = simplifyImport(imp.clone())?;
    Ok(imp)
}

fn simplifyImport(mut imp: Absyn::Import) -> Result<Absyn::Import> {
    let mut imp: Absyn::Import = imp;
    imp = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } if (var_field!(imp.name, Absyn::Import::NAMED_IMPORT).clone() == AbsynUtil::pathLastIdent(var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone())?) => Absyn::Import::QUAL_IMPORT { path: var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone() },
        _ => imp.clone(),
    });
    Ok(imp)
}

fn filterDuplicateImports(mut elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut imports: Arc<UnorderedSet::UnorderedSet<Arc<Path>>>;
    imports = UnorderedSet::new(AbsynUtil::pathHash, fnptr!(AbsynUtil::pathEqual, Arc<Path>, Arc<Path>), 1);
    outElements = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut e in (elements.clone()).into_iter().cloned() {
            if !(!(importExists(e.clone(), imports.clone())?)) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outElements)
}

fn importExists(mut element: Arc<Absyn::ElementItem>, mut imports: Arc<UnorderedSet::UnorderedSet<Arc<Path>>>) -> Result<bool> {
    let mut exists: bool = false;
    let mut path: Arc<Path>;
    exists = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::IMPORT { import_: Absyn::Import::QUAL_IMPORT { path }, .. }, .. } } => {
            exists = UnorderedSet::contains(path.clone(), imports.clone())?;
            if !(exists.clone()) {
                UnorderedSet::add(path.clone(), imports.clone())?;
            }
            exists.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exists)
}

fn convertComponentItem(mut comp: Arc<Absyn::ComponentItem>, mut localRules: RuleTable, mut modifierRules: Arc<metamodelica::List<ConversionRule>>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::ComponentItem>> {
    let mut comp: Arc<Absyn::ComponentItem> = comp;
    assign_field!(
        comp.component = convertComponent(comp.component.clone(), localRules.clone(), modifierRules.clone(), rules.clone(), env.clone(), info.clone())?,
        comp.condition = convertOptExp(comp.condition.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
    );
    Ok(comp)
}

fn convertComponent(mut comp: Absyn::Component, mut localRules: RuleTable, mut modifierRules: Arc<metamodelica::List<ConversionRule>>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Absyn::Component> {
    let mut comp: Absyn::Component = comp;
    comp.arrayDim = convertSubscripts(comp.arrayDim.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?;
    if !(modifierRules.clone().is_empty()) {
        comp.modification = convertModification(comp.modification.clone(), modifierRules.clone())?;
    }
    comp.modification = convertModificationExps(comp.modification.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone());
    Ok(comp)
}

fn convertEquationItems(mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = eqs;
    eqs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
        for mut eq in (eqs.clone()).into_iter().cloned() {
            let __x = convertEquationItem(eq.clone(), localRules.clone(), rules.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(eqs)
}

fn convertEquationItem(mut eq: Arc<Absyn::EquationItem>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Arc<Absyn::EquationItem>> {
    let mut eq: Arc<Absyn::EquationItem> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { .. } => {
            assign_variant_field!(eq => Absyn::EquationItem::EQUATIONITEM; equation_ = convertEquation(var_field!((*eq).equation_, Absyn::EquationItem::EQUATIONITEM).clone(), localRules.clone(), rules.clone(), env.clone(), var_field!((*eq).info, Absyn::EquationItem::EQUATIONITEM).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

fn convertEquation(mut eq: Arc<Absyn::Equation>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::Equation>> {
    let mut eq: Arc<Absyn::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_IF;
                ifExp = convertExp(var_field!((*eq).ifExp, Absyn::Equation::EQ_IF).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                equationTrueItems = convertEquationItems(var_field!((*eq).equationTrueItems, Absyn::Equation::EQ_IF).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseIfBranches = convertBranches(var_field!((*eq).elseIfBranches, Absyn::Equation::EQ_IF).clone(), Arc::new({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), Arc::new(convertEquationItems), localRules.clone(), rules.clone(), env.clone()),
                equationElseItems = convertEquationItems(var_field!((*eq).equationElseItems, Absyn::Equation::EQ_IF).clone(), localRules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_EQUALS { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_EQUALS;
                leftSide = convertExp(var_field!((*eq).leftSide, Absyn::Equation::EQ_EQUALS).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                rightSide = convertExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_EQUALS).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_PDE { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_PDE;
                leftSide = convertExp(var_field!((*eq).leftSide, Absyn::Equation::EQ_PDE).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                rightSide = convertExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_PDE).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_CONNECT;
                connector1 = convertCref(var_field!((*eq).connector1, Absyn::Equation::EQ_CONNECT).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                connector2 = convertCref(var_field!((*eq).connector2, Absyn::Equation::EQ_CONNECT).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_FOR { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_FOR;
                iterators = convertForIterators(var_field!((*eq).iterators, Absyn::Equation::EQ_FOR).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                forEquations = convertEquationItems(var_field!((*eq).forEquations, Absyn::Equation::EQ_FOR).clone(), localRules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_WHEN_E;
                whenExp = convertExp(var_field!((*eq).whenExp, Absyn::Equation::EQ_WHEN_E).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                whenEquations = convertEquationItems(var_field!((*eq).whenEquations, Absyn::Equation::EQ_WHEN_E).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseWhenEquations = convertBranches(var_field!((*eq).elseWhenEquations, Absyn::Equation::EQ_WHEN_E).clone(), Arc::new({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), Arc::new(convertEquationItems), localRules.clone(), rules.clone(), env.clone())
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_NORETCALL;
                functionName = convertCref(var_field!((*eq).functionName, Absyn::Equation::EQ_NORETCALL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                functionArgs = convertFunctionArgs(var_field!((*eq).functionArgs, Absyn::Equation::EQ_NORETCALL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_FAILURE { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_FAILURE; equ = convertEquationItem(var_field!((*eq).equ, Absyn::Equation::EQ_FAILURE).clone(), localRules.clone(), rules.clone(), env.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

fn convertAlgorithmItems(mut algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>> {
    let mut algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = algs;
    algs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
        for mut alg in (algs.clone()).into_iter().cloned() {
            let __x = convertAlgorithmItem(alg.clone(), localRules.clone(), rules.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(algs)
}

fn convertAlgorithmItem(mut alg: Arc<Absyn::AlgorithmItem>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Arc<Absyn::AlgorithmItem>> {
    let mut alg: Arc<Absyn::AlgorithmItem> = alg;
    let () = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { .. } => {
            assign_variant_field!(alg => Absyn::AlgorithmItem::ALGORITHMITEM; algorithm_ = convertAlgorithm(var_field!((*alg).algorithm_, Absyn::AlgorithmItem::ALGORITHMITEM).clone(), localRules.clone(), rules.clone(), env.clone(), var_field!((*alg).info, Absyn::AlgorithmItem::ALGORITHMITEM).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(alg)
}

fn convertAlgorithm(mut alg: Arc<Absyn::Algorithm>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::Algorithm>> {
    let mut alg: Arc<Absyn::Algorithm> = alg;
    let () = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_ASSIGN;
                assignComponent = convertExp(var_field!((*alg).assignComponent, Absyn::Algorithm::ALG_ASSIGN).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                value = convertExp(var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_IF { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_IF;
                ifExp = convertExp(var_field!((*alg).ifExp, Absyn::Algorithm::ALG_IF).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                trueBranch = convertAlgorithmItems(var_field!((*alg).trueBranch, Absyn::Algorithm::ALG_IF).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseIfAlgorithmBranch = convertBranches(var_field!((*alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone(), Arc::new({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), Arc::new(convertAlgorithmItems), localRules.clone(), rules.clone(), env.clone()),
                elseBranch = convertAlgorithmItems(var_field!((*alg).elseBranch, Absyn::Algorithm::ALG_IF).clone(), localRules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_FOR { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_FOR;
                iterators = convertForIterators(var_field!((*alg).iterators, Absyn::Algorithm::ALG_FOR).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                forBody = convertAlgorithmItems(var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone(), localRules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_PARFOR { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_PARFOR;
                iterators = convertForIterators(var_field!((*alg).iterators, Absyn::Algorithm::ALG_PARFOR).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                parforBody = convertAlgorithmItems(var_field!((*alg).parforBody, Absyn::Algorithm::ALG_PARFOR).clone(), localRules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_WHILE;
                boolExpr = convertExp(var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                whileBody = convertAlgorithmItems(var_field!((*alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone(), localRules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_WHEN_A;
                boolExpr = convertExp(var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHEN_A).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                whenBody = convertAlgorithmItems(var_field!((*alg).whenBody, Absyn::Algorithm::ALG_WHEN_A).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseWhenAlgorithmBranch = convertBranches(var_field!((*alg).elseWhenAlgorithmBranch, Absyn::Algorithm::ALG_WHEN_A).clone(), Arc::new({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), Arc::new(convertAlgorithmItems), localRules.clone(), rules.clone(), env.clone())
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_NORETCALL;
                functionCall = convertCref(var_field!((*alg).functionCall, Absyn::Algorithm::ALG_NORETCALL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                functionArgs = convertFunctionArgs(var_field!((*alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_FAILURE { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_FAILURE; equ = convertAlgorithmItems(var_field!((*alg).equ, Absyn::Algorithm::ALG_FAILURE).clone(), localRules.clone(), rules.clone(), env.clone())?);
            ()
        },
        Deref @ Absyn::Algorithm::ALG_TRY { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_TRY;
                body = convertAlgorithmItems(var_field!((*alg).body, Absyn::Algorithm::ALG_TRY).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseBody = convertAlgorithmItems(var_field!((*alg).elseBody, Absyn::Algorithm::ALG_TRY).clone(), localRules.clone(), rules.clone(), env.clone())?
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(alg)
}

fn convertBranches<CondT: Clone + 'static, BodyT: Clone + 'static>(mut branches: Arc<metamodelica::List<(CondT, BodyT)>>, mut condFunc: Arc<dyn ::std::ops::Fn(CondT, Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<metamodelica::List<ConversionRule>>>>, Arc<ConversionRules::ConversionRules>, Env) -> Result<CondT> + 'static>, mut bodyFunc: Arc<dyn ::std::ops::Fn(BodyT, Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<metamodelica::List<ConversionRule>>>>, Arc<ConversionRules::ConversionRules>, Env) -> Result<BodyT> + 'static>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Arc<metamodelica::List<(CondT, BodyT)>> {
    pub type CondFunc<CondT: Clone> = fn(CondT, RuleTable, Arc<ConversionRules::ConversionRules>, Env) -> Result<CondT>;

    pub type BodyFunc<BodyT: Clone> = fn(BodyT, RuleTable, Arc<ConversionRules::ConversionRules>, Env) -> Result<BodyT>;

    let mut branches: Arc<metamodelica::List<(CondT, BodyT)>> = branches;
    branches = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut b in (branches.clone()).into_iter().cloned() {
            let __x = (condFunc(Util::tuple21(b.clone()), localRules.clone(), rules.clone(), env.clone()).unwrap(), bodyFunc(Util::tuple22(b.clone()), localRules.clone(), rules.clone(), env.clone()).unwrap());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    branches
}

fn convertForIterators(mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<Absyn::ForIterator>>>> {
    let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = iters;
    iters = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
        for mut i in (iters.clone()).into_iter().cloned() {
            let __x = convertForIterator(i.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(iters)
}

fn convertForIterator(mut iter: Arc<Absyn::ForIterator>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::ForIterator>> {
    let mut iter: Arc<Absyn::ForIterator> = iter;
    assign_field!(
        iter.guardExp = convertOptExp(iter.guardExp.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
        iter.range = convertOptExp(iter.range.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
    );
    Ok(iter)
}

fn convertExternalDecl(mut extDecl: Arc<Absyn::ExternalDecl>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::ExternalDecl>> {
    let mut extDecl: Arc<Absyn::ExternalDecl> = extDecl;
    assign_field!(extDecl.args = convertExps(extDecl.args.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
    Ok(extDecl)
}

fn convertExps(mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = exps;
    exps = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (exps.clone()).into_iter().cloned() {
            let __x = convertExp(e.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(exps)
}

fn convertOptExp(mut exp: Option<Arc<Absyn::Exp>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut exp: Option<Arc<Absyn::Exp>> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(e) => {
            Some(convertExp(e.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?)
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn convertExp(mut exp: Arc<Absyn::Exp>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = convertCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        Deref @ Absyn::Exp::BINARY { .. } => {
            assign_variant_field!(exp => Absyn::Exp::BINARY;
                exp1 = convertExp(var_field!((*exp).exp1, Absyn::Exp::BINARY).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                exp2 = convertExp(var_field!((*exp).exp2, Absyn::Exp::BINARY).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Exp::UNARY { .. } => {
            assign_variant_field!(exp => Absyn::Exp::UNARY; exp = convertExp(var_field!((*exp).exp, Absyn::Exp::UNARY).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        Deref @ Absyn::Exp::LBINARY { .. } => {
            assign_variant_field!(exp => Absyn::Exp::LBINARY;
                exp1 = convertExp(var_field!((*exp).exp1, Absyn::Exp::LBINARY).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                exp2 = convertExp(var_field!((*exp).exp2, Absyn::Exp::LBINARY).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Exp::LUNARY { .. } => {
            assign_variant_field!(exp => Absyn::Exp::LUNARY; exp = convertExp(var_field!((*exp).exp, Absyn::Exp::LUNARY).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        Deref @ Absyn::Exp::RELATION { .. } => {
            assign_variant_field!(exp => Absyn::Exp::RELATION;
                exp1 = convertExp(var_field!((*exp).exp1, Absyn::Exp::RELATION).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                exp2 = convertExp(var_field!((*exp).exp2, Absyn::Exp::RELATION).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Exp::IFEXP { .. } => {
            assign_variant_field!(exp => Absyn::Exp::IFEXP;
                ifExp = convertExp(var_field!((*exp).ifExp, Absyn::Exp::IFEXP).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                trueBranch = convertExp(var_field!((*exp).trueBranch, Absyn::Exp::IFEXP).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                elseBranch = convertExp(var_field!((*exp).elseBranch, Absyn::Exp::IFEXP).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                elseIfBranch = convertBranches(var_field!((*exp).elseIfBranch, Absyn::Exp::IFEXP).clone(), Arc::new({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), Arc::new({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), localRules.clone(), rules.clone(), env.clone())
            );
            ()
        },
        Deref @ Absyn::Exp::CALL { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CALL;
                function_ = convertCref(var_field!((*exp).function_, Absyn::Exp::CALL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                functionArgs = convertFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::CALL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { .. } => {
            assign_variant_field!(exp => Absyn::Exp::PARTEVALFUNCTION;
                function_ = convertCref(var_field!((*exp).function_, Absyn::Exp::PARTEVALFUNCTION).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                functionArgs = convertFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::PARTEVALFUNCTION).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Exp::ARRAY { .. } => {
            assign_variant_field!(exp => Absyn::Exp::ARRAY; arrayExp = convertExps(var_field!((*exp).arrayExp, Absyn::Exp::ARRAY).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        Deref @ Absyn::Exp::MATRIX { .. } => {
            assign_variant_field!(exp => Absyn::Exp::MATRIX; matrix = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).matrix, Absyn::Exp::MATRIX).clone()).into_iter().cloned() {
            let __x = convertExps(e.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::Exp::RANGE { .. } => {
            assign_variant_field!(exp => Absyn::Exp::RANGE;
                start = convertExp(var_field!((*exp).start, Absyn::Exp::RANGE).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                step = convertOptExp(var_field!((*exp).step, Absyn::Exp::RANGE).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                stop = convertExp(var_field!((*exp).stop, Absyn::Exp::RANGE).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::Exp::TUPLE { .. } => {
            assign_variant_field!(exp => Absyn::Exp::TUPLE; expressions = convertExps(var_field!((*exp).expressions, Absyn::Exp::TUPLE).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        Deref @ Absyn::Exp::EXPRESSIONCOMMENT { .. } => {
            assign_variant_field!(exp => Absyn::Exp::EXPRESSIONCOMMENT; exp = convertExp(var_field!((*exp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        Deref @ Absyn::Exp::SUBSCRIPTED_EXP { .. } => {
            assign_variant_field!(exp => Absyn::Exp::SUBSCRIPTED_EXP;
                exp = convertExp(var_field!((*exp).exp, Absyn::Exp::SUBSCRIPTED_EXP).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                subscripts = convertSubscripts(var_field!((*exp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn convertCref(mut cref: Arc<Absyn::ComponentRef>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::WILD { .. } => cref.clone(),
        Deref @ Absyn::ComponentRef::ALLWILD { .. } => cref.clone(),
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => Arc::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: convertCref2(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())? }),
        _ => convertCref2(cref.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

fn convertCref2(mut cref: Arc<Absyn::ComponentRef>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut path: Arc<Path>;
    let mut cref_rules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
    let mut rule: ConversionRule = ConversionRule::CLASS_IF;
    let mut has_subs: bool = false;
    let mut converted: bool = false;
    has_subs = AbsynUtil::crefHasSubscripts(cref.clone());
    if has_subs.clone() {
        cref = convertCrefSubscripts(cref.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?;
    }
    cref_rules = UnorderedMap::getOrDefault((AbsynUtil::crefFirstIdent(cref.clone())?).clone(), localRules.clone(), metamodelica::nil());
    if !(cref_rules.clone().is_empty()) {
        rule = listHead(cref_rules.clone())?;
        cref = (match rule.clone() {
        ConversionRule::ELEMENT { .. } => AbsynUtil::crefSetFirstIdent(cref.clone(), (var_field!(rule.newName, ConversionRule::ELEMENT).clone()).clone()),
        _ => cref.clone(),
    });
        converted = true;
    } else {
        (cref, converted) = convertCrefFromType(cref.clone(), rules.clone(), env.clone())?;
    }
    if !(converted.clone()) && !(has_subs.clone()) {
        path = AbsynUtil::crefToPath(cref.clone())?;
        path = convertPath(path.clone(), rules.clone(), env.imports.clone(), info.clone())?;
        cref = AbsynUtil::pathToCref(path.clone())?;
    }
    Ok(cref)
}

fn convertCrefFromType(mut cref: Arc<Absyn::ComponentRef>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<(Arc<Absyn::ComponentRef>, bool)> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut converted: bool = false;
    let mut id: ArcStr = arcstr::literal!("");
    let mut first_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut rest_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut opt_ty: Option<Arc<Path>> = None;
    let mut cref_rules: Arc<metamodelica::List<ConversionRule>> = metamodelica::nil();
    if !(AbsynUtil::crefIsQual(cref.clone())) {
        return Ok((cref, converted));
    }
    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
    opt_ty = UnorderedMap::get((id.clone()).clone(), env.components.clone());
    if isSome(opt_ty.clone()) {
        cref_rules = listHead(lookupRules(Util::getOption(opt_ty.clone())?, rules.clone())?)?;
    } else {
        cref_rules = metamodelica::nil();
    }
    if cref_rules.clone().is_empty() {
        return Ok((cref, converted));
    }
    first_cref = AbsynUtil::crefFirstCref(cref.clone());
    rest_cref = AbsynUtil::crefStripFirst(cref.clone())?;
    id = (AbsynUtil::crefFirstIdent(rest_cref.clone())?).clone();
    for mut rule in &*cref_rules.clone() {
        let mut rule = rule.clone();
        let () = (match rule.clone() {
        ConversionRule::ELEMENT { .. } if (var_field!(rule.oldName, ConversionRule::ELEMENT).clone() == id.clone()) => {
            rest_cref = AbsynUtil::crefSetFirstIdent(rest_cref.clone(), (var_field!(rule.newName, ConversionRule::ELEMENT).clone()).clone());
            cref = AbsynUtil::joinCrefs(first_cref.clone(), rest_cref.clone())?;
            converted = true;
            return Ok((cref, converted));
            ()
        },
        _ => (),
    });
    }
    Ok((cref, converted))
}

fn convertCrefSubscripts(mut cref: Arc<Absyn::ComponentRef>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL;
                subscripts = convertSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                componentRef = convertCrefSubscripts(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = convertSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

fn convertSubscripts(mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = subs;
    subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = convertSubscript(s.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(subs)
}

fn convertSubscript(mut sub: Arc<Absyn::Subscript>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::Subscript>> {
    let mut sub: Arc<Absyn::Subscript> = sub;
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = convertExp(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sub)
}

fn convertPath(mut path: Arc<Path>, mut rules: Arc<ConversionRules::ConversionRules>, mut imports: ImportTree, mut info: SourceInfo) -> Result<Arc<Path>> {
    let mut path: Arc<Path> = path;
    let mut import_path: Option<(Arc<Path>, ArcStr)> = None;
    (path, import_path) = applyImportsToPath(path.clone(), imports.clone())?;
    path = applyRulesPath(path.clone(), lookupRules(path.clone(), rules.clone())?, info.clone())?;
    path = stripImportPath(path.clone(), import_path.clone())?;
    Ok(path)
}

fn applyRulesPath(mut path: Arc<Path>, mut rules: Arc<metamodelica::List<Arc<metamodelica::List<ConversionRule>>>>, mut info: SourceInfo) -> Result<Arc<Path>> {
    let mut path: Arc<Path> = path;
    let mut path_len: i32 = AbsynUtil::pathPartCount(path.clone(), 0)?;
    let mut found: bool = false;
    for mut rl in &*rules.clone() {
        let mut rl = rl.clone();
        for mut rule in &*rl.clone() {
            let mut rule = rule.clone();
            found = (match rule.clone() {
        ConversionRule::CLASS { .. } => {
            if path_len.clone() == (var_field!(rule.oldPath, ConversionRule::CLASS).clone().borrow().len() as i32) {
                path = var_field!(rule.newPath, ConversionRule::CLASS).clone();
            } else {
                path = Util::foldcallN((var_field!(rule.oldPath, ConversionRule::CLASS).clone().borrow().len() as i32), Arc::new(AbsynUtil::pathRest), path.clone());
                path = AbsynUtil::joinPaths(var_field!(rule.newPath, ConversionRule::CLASS).clone(), path.clone())?;
            }
            true
        },
        ConversionRule::ELEMENT { .. } if (path_len.clone() > (var_field!(rule.oldPath, ConversionRule::ELEMENT).clone().borrow().len() as i32) && AbsynUtil::pathNthIdent(path.clone(), (var_field!(rule.oldPath, ConversionRule::ELEMENT).clone().borrow().len() as i32) + 1)? == var_field!(rule.oldName, ConversionRule::ELEMENT).clone()) => {
            if path_len.clone() == (var_field!(rule.oldPath, ConversionRule::ELEMENT).clone().borrow().len() as i32) - 1 {
                path = AbsynUtil::pathSetLastIdent(path.clone(), (var_field!(rule.newName, ConversionRule::ELEMENT).clone()).clone())?;
            } else {
                path = AbsynUtil::pathSetNthIdent(path.clone(), (var_field!(rule.newName, ConversionRule::ELEMENT).clone()).clone(), (var_field!(rule.oldPath, ConversionRule::ELEMENT).clone().borrow().len() as i32) + 1)?;
            }
            true
        },
        ConversionRule::MESSAGE { .. } => {
            Error::addSourceMessage(Error::CONVERSION_MESSAGE.clone(), list![(var_field!(rule.message, ConversionRule::MESSAGE).clone()).clone()], info.clone())?;
            true
        },
        _ => false,
    });
            if found.clone() {
                return Ok(path);
            }
        }
    }
    Ok(path)
}

fn convertFunctionArgs(mut args: Arc<Absyn::FunctionArgs>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::FunctionArgs>> {
    let mut args: Arc<Absyn::FunctionArgs> = args;
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. } => {
            assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS;
                args = convertExps(var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                argNames = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
        for mut a in (var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone()).into_iter().cloned() {
            let __x = convertNamedArg(a.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. } => {
            assign_variant_field!(args => Absyn::FunctionArgs::FOR_ITER_FARG;
                exp = convertExp(var_field!((*args).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?,
                iterators = convertForIterators(var_field!((*args).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(args)
}

fn convertNamedArg(mut arg: Arc<Absyn::NamedArg>, mut localRules: RuleTable, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Result<Arc<Absyn::NamedArg>> {
    let mut arg: Arc<Absyn::NamedArg> = arg;
    assign_field!(arg.argValue = convertExp(arg.argValue.clone(), localRules.clone(), rules.clone(), env.clone(), info.clone())?);
    Ok(arg)
}

fn convertOption<T: Clone + 'static>(mut opt: Option<T>, mut optFunc: Arc<dyn ::std::ops::Fn(T, Arc<ConversionRules::ConversionRules>, Env, SourceInfo) -> Result<T> + 'static>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env, mut info: SourceInfo) -> Option<T> {
    pub type OptFunc<T: Clone> = fn(T, Arc<ConversionRules::ConversionRules>, Env, SourceInfo) -> Result<T>;

    let mut opt: Option<T> = opt;
    let mut e: T;
    opt = (match opt.clone() {
        Some(mut e) => Some(optFunc(e.clone(), rules.clone(), env.clone(), info.clone()).unwrap()),
        _ => opt.clone(),
    });
    opt
}

fn getExtendsRules(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>>> {
    let mut extendsRules: Arc<metamodelica::List<Arc<ConversionRules::ConversionRules>>> = metamodelica::nil();
    let mut onode: Option<Arc<ConversionRules::ConversionRules>> = None;
    for mut ext in &*getExtendsPathsInParts(parts.clone()) {
        let mut ext = ext.clone();
        onode = lookupRuleNode(ext.clone(), rules.clone())?;
        if isSome(onode.clone()) {
            extendsRules = cons(Util::getOption(onode.clone())?, extendsRules.clone());
        }
    }
    Ok(extendsRules)
}

fn getExtendsPathsInParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Path>>> {
    let mut extendsPaths: Arc<metamodelica::List<Arc<Path>>> = metamodelica::nil();
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone() {
                let mut e = e.clone();
                extendsPaths = getExtendsPathsInElementItem(e.clone(), extendsPaths.clone());
            }
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone() {
                let mut e = e.clone();
                extendsPaths = getExtendsPathsInElementItem(e.clone(), extendsPaths.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    extendsPaths
}

fn getExtendsPathsInElementItem(mut element: Arc<Absyn::ElementItem>, mut extendsPaths: Arc<metamodelica::List<Arc<Path>>>) -> Arc<metamodelica::List<Arc<Path>>> {
    let mut extendsPaths: Arc<metamodelica::List<Arc<Path>>> = extendsPaths;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path: ext_path, .. }, .. } } => {
            extendsPaths = cons(ext_path.clone(), extendsPaths.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    extendsPaths
}

fn getImportsInParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> {
    let mut imports: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone() {
                let mut e = e.clone();
                imports = getImportsInElementItem(e.clone(), imports.clone());
            }
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone() {
                let mut e = e.clone();
                imports = getImportsInElementItem(e.clone(), imports.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    imports
}

fn getImportsInElementItem(mut element: Arc<Absyn::ElementItem>, mut imports: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> {
    let mut imports: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = imports;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: imp @ Deref @ Absyn::ElementSpec::IMPORT { .. }, .. } } => {
            imports = cons(imp.clone(), imports.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    imports
}

fn addImportNamesToEnv(mut elements: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>>, mut rules: Arc<ConversionRules::ConversionRules>, mut env: Env) -> Result<Env> {
    let mut env: Env = env;
    let mut imp: Absyn::Import;
    let mut info: SourceInfo;
    let mut imps: ImportTree = Arc::new(ImportTreeImpl::Tree::EMPTY);
    if elements.clone().is_empty() {
        return Ok(env);
    }
    imps = env.imports.clone();
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(e.clone()) {
            Deref @ Absyn::ElementSpec::IMPORT { info: __pa0, import_: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        info = __pa0.clone();
        imp = __pa1.clone();
        imps = addImportName(imp.clone(), rules.clone(), info.clone(), imps.clone())?;
    }
    env.imports = imps.clone();
    Ok(env)
}

fn addImportName(mut imp: Absyn::Import, mut rules: Arc<ConversionRules::ConversionRules>, mut info: SourceInfo, mut imports: ImportTree) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let mut name: ArcStr = arcstr::literal!("");
    let mut imp_name: ArcStr = arcstr::literal!("");
    let mut old_path: Arc<Path>;
    let mut new_path: Arc<Path>;
    let () = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { path: ref old_path, name: mut name } => {
            new_path = convertPath(old_path.clone(), rules.clone(), ImportTreeImpl::new(), info.clone())?;
            imports = ImportTreeImpl::add(imports.clone(), (name.clone()).clone(), ImportData { originalPath: old_path.clone(), convertedPath: new_path.clone(), importName: (name.clone()).clone(), shadowed: false }, ImportTreeImpl::addConflictDefault)?;
            ()
        },
        Absyn::Import::QUAL_IMPORT { path: ref old_path } => {
            new_path = convertPath(old_path.clone(), rules.clone(), ImportTreeImpl::new(), info.clone())?;
            name = (AbsynUtil::pathLastIdent(old_path.clone())?).clone();
            imp_name = (AbsynUtil::pathLastIdent(new_path.clone())?).clone();
            imports = ImportTreeImpl::add(imports.clone(), (name.clone()).clone(), ImportData { originalPath: old_path.clone(), convertedPath: new_path.clone(), importName: (imp_name.clone()).clone(), shadowed: false }, ImportTreeImpl::addConflictDefault)?;
            ()
        },
        Absyn::Import::GROUP_IMPORT { prefix: ref old_path, .. } => {
            for mut group in &*var_field!(imp.groups, Absyn::Import::GROUP_IMPORT).clone() {
                let mut group = group.clone();
                imports = addGroupImportName(old_path.clone(), group.clone(), rules.clone(), info.clone(), imports.clone())?;
            }
            ()
        },
        _ => (),
    });
    Ok(imports)
}

fn addGroupImportName(mut prefix: Arc<Path>, mut imp: Absyn::GroupImport, mut rules: Arc<ConversionRules::ConversionRules>, mut info: SourceInfo, mut imports: ImportTree) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let mut rename: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut imp_name: ArcStr = arcstr::literal!("");
    let mut old_path: Arc<Path>;
    let mut new_path: Arc<Path>;
    (rename, name) = (match imp.clone() {
        Absyn::GroupImport::GROUP_IMPORT_NAME { name: mut name } => (name.clone(), name.clone()),
        Absyn::GroupImport::GROUP_IMPORT_RENAME { name: mut name, rename: mut rename } => (rename.clone(), name.clone()),
        _ => bail!("match: no arm matched"),
    });
    old_path = AbsynUtil::suffixPath(prefix.clone(), (name.clone()).clone())?;
    new_path = convertPath(old_path.clone(), rules.clone(), ImportTreeImpl::new(), info.clone())?;
    imp_name = ((match imp.clone() {
        Absyn::GroupImport::GROUP_IMPORT_NAME { .. } => AbsynUtil::pathLastIdent(new_path.clone())?,
        Absyn::GroupImport::GROUP_IMPORT_RENAME { .. } => rename.clone(),
        _ => bail!("match: no arm matched"),
    })).clone();
    imports = ImportTreeImpl::add(imports.clone(), (rename.clone()).clone(), ImportData { originalPath: old_path.clone(), convertedPath: new_path.clone(), importName: (imp_name.clone()).clone(), shadowed: false }, ImportTreeImpl::addConflictDefault)?;
    Ok(imports)
}

fn shadowImportsInParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut imports: ImportTree) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone() {
                let mut e = e.clone();
                imports = shadowImportsInElementItem(e.clone(), imports.clone())?;
            }
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone() {
                let mut e = e.clone();
                imports = shadowImportsInElementItem(e.clone(), imports.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(imports)
}

fn shadowImportsInElementItem(mut element: Arc<Absyn::ElementItem>, mut imports: ImportTree) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: spec, .. } } => {
            imports = shadowImportsInElementSpec(spec.clone(), imports.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(imports)
}

fn shadowImportsInElementSpec(mut spec: Arc<Absyn::ElementSpec>, mut imports: ImportTree) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let () = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name, .. }, .. } => {
            imports = shadowImport((name.clone()).clone(), imports.clone())?;
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            for mut c in &*var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone() {
                let mut c = c.clone();
                imports = shadowImport((AbsynUtil::componentName(c.clone())?).clone(), imports.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(imports)
}

fn shadowImport(mut name: ArcStr, mut imports: ImportTree) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let mut imp_data: ImportData;
    if !(ImportTreeImpl::hasKey(imports.clone(), (name.clone()).clone())?) {
        return Ok(imports);
    }
    imp_data = ImportTreeImpl::get(imports.clone(), (name.clone()).clone())?;
    imp_data.shadowed = true;
    imports = ImportTreeImpl::update(imports.clone(), (name.clone()).clone(), imp_data.clone());
    Ok(imports)
}

fn applyImportsToPath(mut path: Arc<Path>, mut imports: ImportTree) -> Result<(Arc<Path>, Option<(Arc<Path>, ArcStr)>)> {
    let mut path: Arc<Path> = path;
    let mut importPath: Option<(Arc<Path>, ArcStr)> = None;
    let mut imp_data_opt: Option<ImportData> = None;
    let mut imp_data: ImportData;
    imp_data_opt = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::QUALIFIED { .. } => ImportTreeImpl::getOpt(imports.clone(), (var_field!((*path).name, Path::QUALIFIED).clone()).clone()),
        Deref @ Absyn::Path::IDENT { .. } => ImportTreeImpl::getOpt(imports.clone(), (var_field!((*path).name, Path::IDENT).clone()).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if isSome(imp_data_opt.clone()) {
        let Some(__pa0) = (imp_data_opt.clone()) else { bail!("pattern mismatch") };
        imp_data = __pa0.clone();
        if !(imp_data.shadowed.clone()) {
            importPath = Some((imp_data.convertedPath.clone(), imp_data.importName.clone()));
            path = AbsynUtil::pathReplaceFirst(path.clone(), imp_data.originalPath.clone())?;
        } else {
            importPath = None;
        }
    } else {
        importPath = None;
    }
    Ok((path, importPath))
}

fn stripImportPath(mut path: Arc<Path>, mut importPath: Option<(Arc<Path>, ArcStr)>) -> Result<Arc<Path>> {
    let mut path: Arc<Path> = path;
    let mut import_path: Arc<Path>;
    let mut import_name: ArcStr = arcstr::literal!("");
    let mut imp_len: i32 = 0;
    let mut path_len: i32 = 0;
    if isNone(importPath.clone()) {
        return Ok(path);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(importPath.clone()) {
        Some((__pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    import_path = __pa0.clone();
    import_name = __pa1.clone();
    if AbsynUtil::pathPrefixOf(import_path.clone(), path.clone()) {
        imp_len = AbsynUtil::pathPartCount(import_path.clone(), 0)?;
        path_len = AbsynUtil::pathPartCount(path.clone(), 0)?;
        if imp_len.clone() == path_len.clone() {
            path = Arc::new(Path::IDENT { name: (import_name.clone()).clone() });
        } else {
            path = Util::foldcallN(AbsynUtil::pathPartCount(import_path.clone(), 0)?, Arc::new(AbsynUtil::pathRest), path.clone());
            path = AbsynUtil::prefixPath((import_name.clone()).clone(), path.clone());
        }
    }
    Ok(path)
}

fn addComponentTypesToEnv(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut components: TypeTable) -> Result<()> {
    UnorderedMap::clear(components.clone());
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone() {
                let mut e = e.clone();
                addComponentTypesToEnv2(e.clone(), components.clone())?;
            }
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone() {
                let mut e = e.clone();
                addComponentTypesToEnv2(e.clone(), components.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

fn addComponentTypesToEnv2(mut element: Arc<Absyn::ElementItem>, mut components: TypeTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: comps @ Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } } => {
            let mut ty_path: Arc<Path>;
            ty_path = AbsynUtil::typeSpecPath(var_field!((**comps).typeSpec, Absyn::ElementSpec::COMPONENTS).clone())?;
            for mut c in &*var_field!((**comps).components, Absyn::ElementSpec::COMPONENTS).clone() {
                let mut c = c.clone();
                UnorderedMap::add((AbsynUtil::componentName(c.clone())?).clone(), ty_path.clone(), components.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

