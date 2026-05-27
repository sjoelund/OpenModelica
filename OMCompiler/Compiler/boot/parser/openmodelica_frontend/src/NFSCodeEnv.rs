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

use crate::FBuiltin;
use crate::NFEnvExtends;
use crate::NFSCodeCheck;
use crate::NFSCodeFlattenRedeclare;
use crate::NFSCodeLookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Error;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub type Import = Absyn::Import;

pub const tmpTickIndex: i32 = 2;

pub const extendsTickIndex: i32 = 3;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImportTable {
    /// If true means that the imports are hidden.
    pub hidden: bool,
    pub qualifiedImports: Arc<metamodelica::List<Absyn::Import>>,
    pub unqualifiedImports: Arc<metamodelica::List<Absyn::Import>>,
}

pub type IMPORT_TABLE = ImportTable;


/// This uniontype stores a redeclare modifier (which might be derived from an
///  element redeclare). The RAW_MODIFIER stores a 'raw' modifier, i.e. the raw
///  element stored in the SCode representation. These are processed when they are
///  used, i.e. when replacements are done, and converted into PROCESSED_MODIFIERs
///  which are environment items ready to be replaced in the environment.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Redeclaration {
    RAW_MODIFIER {
        modifier: Arc<SCode::Element>,
    },
    PROCESSED_MODIFIER {
        modifier: Arc<Item>,
    },
}
pub use self::Redeclaration::{RAW_MODIFIER,PROCESSED_MODIFIER};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Extends {
    pub baseClass: Arc<Absyn::Path>,
    pub redeclareModifiers: Arc<metamodelica::List<Arc<Redeclaration>>>,
    pub index: i32,
    pub info: SourceInfo,
}

pub type EXTENDS = Extends;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtendsTable {
    pub baseClasses: Arc<metamodelica::List<Arc<Extends>>>,
    pub redeclaredElements: Arc<metamodelica::List<Arc<SCode::Element>>>,
    pub classExtendsInfo: Option<Arc<SCode::Element>>,
}

pub type EXTENDS_TABLE = ExtendsTable;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FrameType {
    NORMAL_SCOPE,
    ENCAPSULATED_SCOPE,
    /// This scope contains one or more iterators; they are made unique by the following index (plus their name)
    IMPLICIT_SCOPE {
        iterIndex: i32,
    },
}
pub use self::FrameType::{NORMAL_SCOPE,ENCAPSULATED_SCOPE,IMPLICIT_SCOPE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    pub name: Option<ArcStr>,
    pub frameType: FrameType,
    pub clsAndVars: Arc<EnvTree::Tree>,
    pub extendsTable: Arc<ExtendsTable>,
    pub importTable: ImportTable,
    /// Used by SCodeDependency.
    pub isUsed: Option<Mutable::Mutable<bool>>,
}

pub type FRAME = Frame;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClassType {
    USERDEFINED,
    BUILTIN,
    CLASS_EXTENDS,
    BASIC_TYPE,
}
pub use self::ClassType::{USERDEFINED,BUILTIN,CLASS_EXTENDS,BASIC_TYPE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    VAR {
        var: Arc<SCode::Element>,
        /// Used by SCodeDependency.
        isUsed: Option<Mutable::Mutable<bool>>,
    },
    CLASS {
        cls: Arc<SCode::Element>,
        env: Env,
        classType: ClassType,
    },
    /// An alias for another Item, see comment in SCodeFlattenRedeclare package.
    ALIAS {
        name: ArcStr,
        path: Option<Arc<Absyn::Path>>,
        info: SourceInfo,
    },
    REDECLARED_ITEM {
        item: Arc<Item>,
        declaredEnv: Env,
    },
}
pub use self::Item::{VAR,CLASS,ALIAS,REDECLARED_ITEM};

pub mod EnvTree {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = Arc<Item>;

    pub fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (inKey.clone()).clone();
        outString
    }

    pub fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (literal!("$item")).clone();
        outString
    }

    pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
        let mut outResult: i32 = 0;
        outResult = stringCompare((inKey1.clone()).clone(), (inKey2.clone()).clone());
        outResult
    }

    pub use addConflictReplace as addConflictDefault;

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

    pub type ValueNode = ArcStr;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Item>, Arc<Item>, ArcStr) -> Result<Arc<Item>> + 'static>) -> Result<Arc<Tree>> {
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
            let mut value: Value;
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare((inKey.clone()).clone(), (var_field!((*tree).key, Tree::LEAF).clone()).clone());
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }), right: Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }) });
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

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(ArcStr, Arc<Item>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Item>, Arc<Item>, ArcStr) -> Result<Arc<Item>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key = arcstr::literal!("");
        let mut value: Value;
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Arc<Item>>) -> Result<Arc<Item>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Item>>) -> Result<Value> + 'static>;

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
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }), right: Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }) });
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Item>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
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

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Item>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
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

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Item>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
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

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Item>) -> Result<()> + 'static>) -> Result<()> {
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

    pub fn fromList(mut inValues: Arc<metamodelica::List<(ArcStr, Arc<Item>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Item>, Arc<Item>, ArcStr) -> Result<Arc<Item>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY);
        let mut key: Key = arcstr::literal!("");
        let mut value: Value;
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
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
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Option<Arc<Item>> {
        let mut value: Option<Arc<Item>> = None;
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Item>, Arc<Item>, ArcStr) -> Result<Arc<Item>> + 'static>) -> Result<Arc<Tree>> {
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

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Item>>>) -> Arc<metamodelica::List<Arc<Item>>> {
        let mut lst: Arc<metamodelica::List<Arc<Item>>> = lst;
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

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Item>) -> Result<Arc<Item>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

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

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<Item>, FT) -> Result<(Arc<Item>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

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
        let mut outTree: Arc<Tree> = Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY);
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY), node.clone())?
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

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(ArcStr, Arc<Item>)>>) -> Arc<metamodelica::List<(ArcStr, Arc<Item>)>> {
        let mut lst: Arc<metamodelica::List<(ArcStr, Arc<Item>)>> = lst;
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
        let mut outTree: Arc<Tree> = add(tree.clone(), (key.clone()).clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Arc<Item>, Arc<Item>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Item>, Arc<Item>, ArcStr) -> Result<Arc<Item>> + 'static>)).unwrap();
        outTree
    }

}

pub type Env = Arc<metamodelica::List<Arc<Frame>>>;

pub static emptyEnv: std::sync::LazyLock<Arc<metamodelica::List<Arc<Frame>>>> = std::sync::LazyLock::new(|| { metamodelica::nil() });

pub const BASE_CLASS_SUFFIX: &'static str = "$base";

pub fn newEnvironment(mut inName: Option<ArcStr>) -> Env {
    let mut outEnv: Env = metamodelica::nil();
    let mut new_frame: Arc<Frame>;
    new_frame = newFrame(inName.clone(), crate::NFSCodeEnv::FrameType::NORMAL_SCOPE);
    outEnv = list![new_frame.clone()];
    outEnv
}

fn openScope(mut inEnv: Env, mut inClass: Arc<SCode::Element>) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut encapsulatedPrefix: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
    let mut new_frame: Arc<Frame>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { encapsulatedPrefix: __pa0, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    encapsulatedPrefix = __pa0.clone();
    name = __pa1.clone();
    new_frame = newFrame(Some((name.clone()).clone()), getFrameType(encapsulatedPrefix.clone()));
    outEnv = cons(new_frame.clone(), inEnv.clone());
    Ok(outEnv)
}

pub fn enterScope(mut inEnv: Env, mut inName: ArcStr) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = 'mc: {
        let __mc_input = (inEnv.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut cls_env: Arc<Frame>;
                    let mut item: Arc<Item>;
                    let mut outEnv: Arc<metamodelica::List<Arc<Frame>>> = outEnv.clone();
                    (item, _) = NFSCodeLookup::lookupInClass((inName.clone()).clone(), inEnv.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(getItemEnv(item.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls_env = __pa0.clone();
                    outEnv = enterFrame(cls_env.clone(), inEnv.clone());
                    Ok(outEnv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to enterScope: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*printEnvStr(inEnv.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn enterScopePath(mut inEnv: Env, mut inPath: Arc<Absyn::Path>) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inEnv.clone(), inPath.clone())) {
        (_, Deref @ Absyn::Path::QUALIFIED { path, name }) => {
            let mut env: Env = metamodelica::nil();
            env = enterScope(inEnv.clone(), (name.clone()).clone())?;
            enterScopePath(env.clone(), path.clone())?
        },
        (_, Deref @ Absyn::Path::IDENT { name }) => {
            enterScope(inEnv.clone(), (name.clone()).clone())?
        },
        (_, Deref @ Absyn::Path::FULLYQUALIFIED { path }) => {
            let mut env: Env = metamodelica::nil();
            env = getEnvTopScope(inEnv.clone())?;
            enterScopePath(env.clone(), path.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

pub fn enterFrame(mut inFrame: Arc<Frame>, mut inEnv: Env) -> Env {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = cons(inFrame.clone(), inEnv.clone());
    outEnv
}

pub fn getEnvTopScope(mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut top_scope: Arc<Frame>;
    let mut env: Env = metamodelica::nil();
    env = inEnv.clone().reverse();
    let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    top_scope = __pa0.clone();
    outEnv = list![top_scope.clone()];
    Ok(outEnv)
}

fn getFrameType(mut encapsulatedPrefix: SCode::Encapsulated) -> FrameType {
    let mut outType: FrameType = FrameType::ENCAPSULATED_SCOPE;
    outType = (match encapsulatedPrefix.clone() {
        SCode::Encapsulated::ENCAPSULATED => crate::NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE,
        _ => crate::NFSCodeEnv::FrameType::NORMAL_SCOPE,
    });
    outType
}

fn newFrame(mut inName: Option<ArcStr>, mut inType: FrameType) -> Arc<Frame> {
    let mut outFrame: Arc<Frame>;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut exts: Arc<ExtendsTable>;
    let mut imps: ImportTable;
    let mut is_used: Mutable::Mutable<bool>;
    tree = EnvTree::new();
    exts = newExtendsTable();
    imps = newImportTable();
    is_used = Mutable::create(false);
    outFrame = Arc::new(Frame { name: inName.clone(), frameType: inType.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: imps.clone(), isUsed: Some(is_used.clone()) });
    outFrame
}

fn newImportTable() -> ImportTable {
    let mut outImports: ImportTable;
    outImports = ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() };
    outImports
}

fn newExtendsTable() -> Arc<ExtendsTable> {
    let mut outExtends: Arc<ExtendsTable>;
    outExtends = Arc::new(ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None });
    outExtends
}

pub fn newItem(mut inElement: Arc<SCode::Element>) -> Result<Arc<Item>> {
    let mut outItem: Arc<Item>;
    outItem = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            let mut class_env: Env = metamodelica::nil();
            let mut item: Arc<Item>;
            class_env = makeClassEnvironment(inElement.clone(), true)?;
            item = newClassItem(inElement.clone(), class_env.clone(), crate::NFSCodeEnv::ClassType::USERDEFINED);
            item.clone()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            newVarItem(inElement.clone(), false)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outItem)
}

pub fn newClassItem(mut inClass: Arc<SCode::Element>, mut inEnv: Env, mut inClassType: ClassType) -> Arc<Item> {
    let mut outClassItem: Arc<Item>;
    outClassItem = Arc::new(Item::CLASS { cls: inClass.clone(), env: inEnv.clone(), classType: inClassType.clone() });
    outClassItem
}

pub fn newVarItem(mut inVar: Arc<SCode::Element>, mut inIsUsed: bool) -> Arc<Item> {
    let mut outVarItem: Arc<Item>;
    let mut is_used: Mutable::Mutable<bool>;
    is_used = Mutable::create(inIsUsed.clone());
    outVarItem = Arc::new(Item::VAR { var: inVar.clone(), isUsed: Some(is_used.clone()) });
    outVarItem
}

pub fn extendEnvWithClasses(mut inClasses: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnv: Env) -> Env {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = List::fold(inClasses.clone(), (std::sync::Arc::new(extendEnvWithClass) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<Arc<Frame>>>) -> Result<Arc<metamodelica::List<Arc<Frame>>>> + 'static>), inEnv.clone());
    outEnv
}

fn extendEnvWithClass(mut inClass: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = extendEnvWithClassDef(inClass.clone(), inEnv.clone())?;
    Ok(outEnv)
}

pub fn getClassType(mut inClassDef: Arc<SCode::ClassDef>) -> ClassType {
    let mut outType: ClassType = ClassType::BASIC_TYPE;
    outType = (::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), .. }), .. } => crate::NFSCodeEnv::ClassType::BUILTIN,
        _ => crate::NFSCodeEnv::ClassType::USERDEFINED,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn printClassType(mut inClassType: ClassType) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inClassType.clone() {
        ClassType::BUILTIN { .. } => literal!("BUILTIN"),
        ClassType::CLASS_EXTENDS { .. } => literal!("CLASS_EXTENDS"),
        ClassType::USERDEFINED { .. } => literal!("USERDEFINED"),
        ClassType::BASIC_TYPE { .. } => literal!("BASIC_TYPE"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn removeExtendsFromLocalScope(mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut imps: ImportTable;
    let mut exts: Arc<ExtendsTable>;
    let mut rest: Env = metamodelica::nil();
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { isUsed: __pa0, importTable: __pa1, clsAndVars: __pa2, frameType: __pa3, name: __pa4, .. }, tail: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    is_used = __pa0.clone();
    imps = __pa1.clone();
    tree = __pa2.clone();
    ty = __pa3.clone();
    name = __pa4.clone();
    rest = __pa5.clone();
    exts = newExtendsTable();
    outEnv = cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: imps.clone(), isUsed: is_used.clone() }), rest.clone());
    Ok(outEnv)
}

pub fn removeExtendFromLocalScope(mut inExtend: Arc<Absyn::Path>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut imps: ImportTable;
    let mut rest: Env = metamodelica::nil();
    let mut iu: Option<Mutable::Mutable<bool>> = None;
    let mut bcl: Arc<metamodelica::List<Arc<Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { isUsed: __pa0, importTable: __pa1, extendsTable: Deref @ ExtendsTable { classExtendsInfo: __pa2, redeclaredElements: __pa3, baseClasses: __pa4 }, clsAndVars: __pa5, frameType: __pa6, name: __pa7 }, tail: __pa8 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    iu = __pa0.clone();
    imps = __pa1.clone();
    cei = __pa2.clone();
    re = __pa3.clone();
    bcl = __pa4.clone();
    tree = __pa5.clone();
    ty = __pa6.clone();
    name = __pa7.clone();
    rest = __pa8.clone();
    (bcl, _) = List::deleteMemberOnTrue(inExtend.clone(), bcl.clone(), (std::sync::Arc::new(isExtendNamed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Extends>) -> Result<bool> + 'static>))?;
    outEnv = cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: Arc::new(ExtendsTable { baseClasses: bcl.clone(), redeclaredElements: re.clone(), classExtendsInfo: cei.clone() }), importTable: imps.clone(), isUsed: iu.clone() }), rest.clone());
    Ok(outEnv)
}

fn isExtendNamed(mut inName: Arc<Absyn::Path>, mut inExtends: Arc<Extends>) -> Result<bool> {
    let mut outIsNamed: bool = false;
    let mut bc: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(inExtends.clone()) {
        Deref @ Extends { baseClass: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    bc = __pa0.clone();
    outIsNamed = AbsynUtil::pathEqual(inName.clone(), bc.clone());
    Ok(outIsNamed)
}

pub fn removeRedeclaresFromLocalScope(mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut imps: ImportTable;
    let mut exts: Arc<ExtendsTable>;
    let mut rest: Env = metamodelica::nil();
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let mut bc: Arc<metamodelica::List<Arc<Extends>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { isUsed: __pa0, importTable: __pa1, extendsTable: Deref @ ExtendsTable { classExtendsInfo: __pa2, baseClasses: __pa3, .. }, clsAndVars: __pa4, frameType: __pa5, name: __pa6 }, tail: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    is_used = __pa0.clone();
    imps = __pa1.clone();
    cei = __pa2.clone();
    bc = __pa3.clone();
    tree = __pa4.clone();
    ty = __pa5.clone();
    name = __pa6.clone();
    rest = __pa7.clone();
    bc = List::map(bc.clone(), (std::sync::Arc::new(removeRedeclaresFromExtend) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Extends>) -> Result<Arc<Extends>> + 'static>));
    exts = Arc::new(ExtendsTable { baseClasses: bc.clone(), redeclaredElements: metamodelica::nil(), classExtendsInfo: cei.clone() });
    outEnv = cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: imps.clone(), isUsed: is_used.clone() }), rest.clone());
    Ok(outEnv)
}

fn removeRedeclaresFromExtend(mut inExtend: Arc<Extends>) -> Result<Arc<Extends>> {
    let mut outExtend: Arc<Extends>;
    let mut bc: Arc<Absyn::Path>;
    let mut index: i32 = 0;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExtend.clone()) {
        Deref @ Extends { baseClass: __pa0, redeclareModifiers: _, index: __pa1, info: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    bc = __pa0.clone();
    index = __pa1.clone();
    info = __pa2.clone();
    outExtend = Arc::new(Extends { baseClass: bc.clone(), redeclareModifiers: metamodelica::nil(), index: index.clone(), info: info.clone() });
    Ok(outExtend)
}

pub fn removeClsAndVarsFromFrame(mut inFrame: Arc<Frame>) -> Result<(Arc<Frame>, Arc<EnvTree::Tree>)> {
    let mut outFrame: Arc<Frame>;
    let mut outClsAndVars: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut name: Option<ArcStr> = None;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut imps: ImportTable;
    let mut exts: Arc<ExtendsTable>;
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(inFrame.clone()) {
        Deref @ Frame { isUsed: __pa0, importTable: __pa1, extendsTable: __pa2, clsAndVars: __pa3, frameType: __pa4, name: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    is_used = __pa0.clone();
    imps = __pa1.clone();
    exts = __pa2.clone();
    outClsAndVars = __pa3.clone();
    ty = __pa4.clone();
    name = __pa5.clone();
    tree = EnvTree::new();
    outFrame = Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: imps.clone(), isUsed: is_used.clone() });
    Ok((outFrame, outClsAndVars))
}

pub fn setImportTableHidden(mut inEnv: Env, mut inHidden: bool) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut imps: ImportTable;
    let mut exts: Arc<ExtendsTable>;
    let mut rest: Env = metamodelica::nil();
    let mut qi: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut uqi: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { isUsed: __pa0, importTable: ImportTable { unqualifiedImports: __pa1, qualifiedImports: __pa2, .. }, extendsTable: __pa3, clsAndVars: __pa4, frameType: __pa5, name: __pa6 }, tail: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    is_used = __pa0.clone();
    uqi = __pa1.clone();
    qi = __pa2.clone();
    exts = __pa3.clone();
    tree = __pa4.clone();
    ty = __pa5.clone();
    name = __pa6.clone();
    rest = __pa7.clone();
    outEnv = cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: ImportTable { hidden: inHidden.clone(), qualifiedImports: qi.clone(), unqualifiedImports: uqi.clone() }, isUsed: is_used.clone() }), rest.clone());
    Ok(outEnv)
}

pub fn setImportsInItemHidden(mut inItem: Arc<Item>, mut inHidden: bool) -> Result<Arc<Item>> {
    let mut outItem: Arc<Item>;
    outItem = (::match_deref::match_deref! { match &((inItem.clone(), inHidden.clone())) {
        (Deref @ Item::CLASS { classType: cls_ty, env, cls }, _) => {
            let mut env = (*env).clone();
            env = setImportTableHidden(env.clone(), inHidden.clone())?;
            Arc::new(Item::CLASS { cls: cls.clone(), env: env.clone(), classType: cls_ty.clone() })
        },
        _ => {
            inItem.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outItem)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isItemUsed(mut inItem: Arc<Item>) -> bool {
    let mut isUsed: bool = false;
    isUsed = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Item::CLASS { env: Deref @ metamodelica::List::Cons { head: Deref @ Frame { isUsed: Some(is_used), .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            Mutable::access(is_used.clone())
        },
        Deref @ Item::VAR { isUsed: Some(is_used), .. } => {
            Mutable::access(is_used.clone())
        },
        Deref @ Item::ALIAS { .. } => {
            true
        },
        Deref @ Item::REDECLARED_ITEM { item, .. } => {
            isItemUsed(item.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isUsed
}

pub fn linkItemUsage(mut inSrcItem: Arc<Item>, mut inDestItem: Arc<Item>) -> Arc<Item> {
    let mut outDestItem: Arc<Item>;
    outDestItem = (::match_deref::match_deref! { match &((inSrcItem.clone(), inDestItem.clone())) {
        (Deref @ Item::VAR { isUsed: is_used, .. }, Deref @ Item::VAR { var: elem, .. }) => {
            Arc::new(Item::VAR { var: elem.clone(), isUsed: is_used.clone() })
        },
        (Deref @ Item::CLASS { env: Deref @ metamodelica::List::Cons { head: Deref @ Frame { isUsed: is_used, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ Item::CLASS { env: Deref @ metamodelica::List::Cons { head: Deref @ Frame { name, frameType: ft, clsAndVars: cv, extendsTable: exts, importTable: imps, isUsed: _ }, tail: Deref @ metamodelica::List::Nil }, classType: cls_ty, cls: elem }) => {
            Arc::new(Item::CLASS { cls: elem.clone(), env: list![Arc::new(Frame { name: name.clone(), frameType: ft.clone(), clsAndVars: cv.clone(), extendsTable: exts.clone(), importTable: imps.clone(), isUsed: is_used.clone() })], classType: cls_ty.clone() })
        },
        (_, Deref @ Item::REDECLARED_ITEM { item, declaredEnv: env }) => {
            let mut item = (*item).clone();
            item = linkItemUsage(inSrcItem.clone(), item.clone());
            Arc::new(Item::REDECLARED_ITEM { item: item.clone(), declaredEnv: env.clone() })
        },
        _ => {
            inDestItem.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDestItem
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isClassItem(mut inItem: Arc<Item>) -> bool {
    let mut outIsClass: bool = false;
    outIsClass = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Item::CLASS { .. } => {
            true
        },
        Deref @ Item::REDECLARED_ITEM { item, .. } => {
            isClassItem(item.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClass
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isVarItem(mut inItem: Arc<Item>) -> bool {
    let mut outIsVar: bool = false;
    outIsVar = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Item::VAR { .. } => {
            true
        },
        Deref @ Item::REDECLARED_ITEM { item, .. } => {
            isVarItem(item.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsVar
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isClassExtendsItem(mut inItem: Arc<Item>) -> bool {
    let mut outIsClassExtends: bool = false;
    outIsClassExtends = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Item::CLASS { classType: ClassType::CLASS_EXTENDS { .. }, .. } => {
            true
        },
        Deref @ Item::REDECLARED_ITEM { item, .. } => {
            isClassExtendsItem(item.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClassExtends
}

fn extendEnvWithClassDef(mut inClassDefElement: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inClassDefElement.clone(), inEnv.clone())) {
        (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, _) => {
            NFEnvExtends::extendEnvWithClassExtends(inClassDefElement.clone(), inEnv.clone())?
        },
        (Deref @ SCode::Element::CLASS { info, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, classDef: cdef, name: cls_name, .. }, _) => {
            let mut alias_name: ArcStr = arcstr::literal!("");
            let mut class_env: Env = metamodelica::nil();
            let mut env: Env = metamodelica::nil();
            let mut cls_type: ClassType = ClassType::BASIC_TYPE;
            class_env = makeClassEnvironment(inClassDefElement.clone(), false)?;
            cls_type = getClassType(cdef.clone());
            alias_name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cls_name.clone()); __mm_s.push_str(&*arcstr::literal!(BASE_CLASS_SUFFIX)); ArcStr::from(__mm_s) }).clone();
            env = extendEnvWithItem(newClassItem(inClassDefElement.clone(), class_env.clone(), cls_type.clone()), inEnv.clone(), (alias_name.clone()).clone())?;
            env = extendEnvWithItem(Arc::new(Item::ALIAS { name: (alias_name.clone()).clone(), path: None, info: info.clone() }), env.clone(), (cls_name.clone()).clone())?;
            env.clone()
        },
        (Deref @ SCode::Element::CLASS { classDef: cdef, name: cls_name, .. }, _) => {
            let mut class_env: Env = metamodelica::nil();
            let mut env: Env = metamodelica::nil();
            let mut cls_type: ClassType = ClassType::BASIC_TYPE;
            class_env = makeClassEnvironment(inClassDefElement.clone(), false)?;
            cls_type = getClassType(cdef.clone());
            env = extendEnvWithItem(newClassItem(inClassDefElement.clone(), class_env.clone(), cls_type.clone()), inEnv.clone(), (cls_name.clone()).clone())?;
            env.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

pub fn makeClassEnvironment(mut inClassDefElement: Arc<SCode::Element>, mut inInModifierScope: bool) -> Result<Env> {
    let mut outClassEnv: Env = metamodelica::nil();
    let mut cdef: Arc<SCode::ClassDef>;
    let mut cls: Arc<SCode::Element>;
    let mut cls_name: ArcStr = arcstr::literal!("");
    let mut env: Env = metamodelica::nil();
    let mut enclosing_env: Env = metamodelica::nil();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inClassDefElement.clone()) {
        Deref @ SCode::Element::CLASS { info: __pa0, classDef: __pa1, name: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    cdef = __pa1.clone();
    cls_name = __pa2.clone();
    env = openScope(emptyEnv.clone(), inClassDefElement.clone())?;
    enclosing_env = if (inInModifierScope.clone()) {emptyEnv.clone()} else {env.clone()};
    outClassEnv = extendEnvWithClassComponents((cls_name.clone()).clone(), cdef.clone(), env.clone(), enclosing_env.clone(), info.clone())?;
    Ok(outClassEnv)
}

fn extendEnvWithVar(mut inVar: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut var_name: ArcStr = arcstr::literal!("");
    let mut is_used: Mutable::Mutable<bool>;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ SCode::Element::COMPONENT { info: __pa0, typeSpec: __pa1, name: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    ty = __pa1.clone();
    var_name = __pa2.clone();
    is_used = Mutable::create(false);
    outEnv = extendEnvWithItem(Arc::new(Item::VAR { var: inVar.clone(), isUsed: Some(is_used.clone()) }), inEnv.clone(), (var_name.clone()).clone())?;
    Ok(outEnv)
}

pub fn extendEnvWithItem(mut inItem: Arc<Item>, mut inEnv: Env, mut inItemName: ArcStr) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut exts: Arc<ExtendsTable>;
    let mut imps: ImportTable;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut rest: Env = metamodelica::nil();
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: __pa0, frameType: __pa1, clsAndVars: __pa2, extendsTable: __pa3, importTable: __pa4, isUsed: __pa5 }, tail: __pa6 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    tree = __pa2.clone();
    exts = __pa3.clone();
    imps = __pa4.clone();
    is_used = __pa5.clone();
    rest = __pa6.clone();
    tree = EnvTree::add(tree.clone(), (inItemName.clone()).clone(), inItem.clone(), (std::sync::Arc::new(fnptr!(extendEnvWithItemConflict, Arc<Item>, Arc<Item>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Item>, Arc<Item>, ArcStr) -> Result<Arc<Item>> + 'static>))?;
    outEnv = cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: imps.clone(), isUsed: is_used.clone() }), rest.clone());
    Ok(outEnv)
}

pub fn extendEnvWithItemConflict(mut newItem: Arc<Item>, mut oldItem: Arc<Item>, mut name: ArcStr) -> Arc<Item> {
    let mut item: Arc<Item>;
    item = linkItemUsage(oldItem.clone(), newItem.clone());
    item
}

pub fn updateItemInEnv(mut inItem: Arc<Item>, mut inEnv: Env, mut inItemName: ArcStr) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut exts: Arc<ExtendsTable>;
    let mut imps: ImportTable;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut rest: Env = metamodelica::nil();
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: __pa0, frameType: __pa1, clsAndVars: __pa2, extendsTable: __pa3, importTable: __pa4, isUsed: __pa5 }, tail: __pa6 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    tree = __pa2.clone();
    exts = __pa3.clone();
    imps = __pa4.clone();
    is_used = __pa5.clone();
    rest = __pa6.clone();
    tree = EnvTree::add(tree.clone(), (inItemName.clone()).clone(), inItem.clone(), (std::sync::Arc::new(fnptr!(EnvTree::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
    outEnv = cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: imps.clone(), isUsed: is_used.clone() }), rest.clone());
    Ok(outEnv)
}

fn extendEnvWithImport(mut inImport: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inImport.clone(), inEnv.clone())) {
        (Deref @ SCode::Element::IMPORT { imp: imp @ Absyn::Import::UNQUAL_IMPORT { .. }, .. }, Deref @ metamodelica::List::Cons { head: Deref @ Frame { name, frameType: ty, clsAndVars: tree, extendsTable: exts, importTable: ImportTable { hidden, qualifiedImports: qual_imps, unqualifiedImports: unqual_imps }, isUsed: is_used }, tail: rest }) => {
            let mut unqual_imps = (*unqual_imps).clone();
            unqual_imps = cons(imp.clone(), unqual_imps.clone());
            cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: ImportTable { hidden: hidden.clone(), qualifiedImports: qual_imps.clone(), unqualifiedImports: unqual_imps.clone() }, isUsed: is_used.clone() }), rest.clone())
        },
        (Deref @ SCode::Element::IMPORT { imp, .. }, Deref @ metamodelica::List::Cons { head: Deref @ Frame { name, frameType: ty, clsAndVars: tree, extendsTable: exts, importTable: ImportTable { hidden, qualifiedImports: qual_imps, unqualifiedImports: unqual_imps }, isUsed: is_used }, tail: rest }) => {
            let mut imp = (*imp).clone();
            let mut qual_imps = (*qual_imps).clone();
            imp = translateQualifiedImportToNamed(imp.clone())?;
            qual_imps = cons(imp.clone(), qual_imps.clone());
            cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: ImportTable { hidden: hidden.clone(), qualifiedImports: qual_imps.clone(), unqualifiedImports: unqual_imps.clone() }, isUsed: is_used.clone() }), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

fn translateQualifiedImportToNamed(mut inImport: Import) -> Result<Import> {
    let mut outImport: Import;
    outImport = (match inImport.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => {
            inImport.clone()
        },
        Absyn::Import::QUAL_IMPORT { path: mut path } => {
            let mut name: ArcStr = arcstr::literal!("");
            name = (AbsynUtil::pathLastIdent(path.clone())?).clone();
            Absyn::Import::NAMED_IMPORT { name: (name.clone()).clone(), path: path.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outImport)
}

pub fn extendEnvWithExtends(mut inExtends: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut bc: Arc<Absyn::Path>;
    let mut mods: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut redecls: Arc<metamodelica::List<Arc<Redeclaration>>> = metamodelica::nil();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut env: Env = metamodelica::nil();
    let mut index: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExtends.clone()) {
        Deref @ SCode::Element::EXTENDS { info: __pa0, modifications: __pa1, baseClassPath: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    mods = __pa1.clone();
    bc = __pa2.clone();
    redecls = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(mods.clone());
    index = System::tmpTickIndex(extendsTickIndex.clone());
    outEnv = addExtendsToEnvExtendsTable(Arc::new(Extends { baseClass: bc.clone(), redeclareModifiers: redecls.clone(), index: index.clone(), info: info.clone() }), inEnv.clone())?;
    Ok(outEnv)
}

fn addExtendsToEnvExtendsTable(mut inExtends: Arc<Extends>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut exts: Arc<metamodelica::List<Arc<Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(getEnvExtendsTable(inEnv.clone())?) {
        Deref @ ExtendsTable { baseClasses: __pa0, redeclaredElements: __pa1, classExtendsInfo: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exts = __pa0.clone();
    re = __pa1.clone();
    cei = __pa2.clone();
    exts = cons(inExtends.clone(), exts.clone());
    outEnv = setEnvExtendsTable(Arc::new(ExtendsTable { baseClasses: exts.clone(), redeclaredElements: re.clone(), classExtendsInfo: cei.clone() }), inEnv.clone())?;
    Ok(outEnv)
}

fn addElementRedeclarationToEnvExtendsTable(mut inRedeclare: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut exts: Arc<metamodelica::List<Arc<Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(getEnvExtendsTable(inEnv.clone())?) {
        Deref @ ExtendsTable { baseClasses: __pa0, redeclaredElements: __pa1, classExtendsInfo: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exts = __pa0.clone();
    re = __pa1.clone();
    cei = __pa2.clone();
    re = cons(inRedeclare.clone(), re.clone());
    outEnv = setEnvExtendsTable(Arc::new(ExtendsTable { baseClasses: exts.clone(), redeclaredElements: re.clone(), classExtendsInfo: cei.clone() }), inEnv.clone())?;
    Ok(outEnv)
}

fn extendEnvWithClassComponents(mut inClassName: ArcStr, mut inClassDef: Arc<SCode::ClassDef>, mut inEnv: Env, mut inEnclosingScope: Env, mut inInfo: SourceInfo) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inClassName.clone(), inClassDef.clone(), inEnv.clone(), inEnclosingScope.clone(), inInfo.clone())) {
        (_, Deref @ SCode::ClassDef::PARTS { elementLst: el, .. }, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            env = List::fold(el.clone(), (std::sync::Arc::new(extendEnvWithElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<Arc<Frame>>>) -> Result<Arc<metamodelica::List<Arc<Frame>>>> + 'static>), inEnv.clone());
            env.clone()
        },
        (_, Deref @ SCode::ClassDef::DERIVED { modifications: mods, typeSpec: ty @ Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            NFSCodeCheck::checkRecursiveShortDefinition(ty.clone(), (inClassName.clone()).clone(), inEnclosingScope.clone(), inInfo.clone())?;
            env = extendEnvWithExtends(Arc::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, modifications: mods.clone(), ann: None, info: inInfo.clone() }), inEnv.clone())?;
            env.clone()
        },
        (_, Deref @ SCode::ClassDef::ENUMERATION { enumLst: enums }, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            let mut path: Arc<Absyn::Path>;
            path = Arc::new(Absyn::Path::IDENT { name: (inClassName.clone()).clone() });
            env = extendEnvWithEnumLiterals(enums.clone(), path.clone(), 1, inEnv.clone(), inInfo.clone())?;
            env.clone()
        },
        _ => {
            inEnv.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn extendEnvWithElement(mut inElement: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = 'mc: {
        let __mc_input = (inElement.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE, .. }, .. }, _) => {
                    let mut env: Env = metamodelica::nil();
                    env = addElementRedeclarationToEnvExtendsTable(inElement.clone(), inEnv.clone())?;
                    env = extendEnvWithVar(inElement.clone(), env.clone())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { .. }, _) => {
                    let mut env: Env = metamodelica::nil();
                    env = extendEnvWithVar(inElement.clone(), inEnv.clone())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE, .. }, .. }, _) => {
                    let mut env: Env = metamodelica::nil();
                    env = addElementRedeclarationToEnvExtendsTable(inElement.clone(), inEnv.clone())?;
                    env = extendEnvWithClassDef(inElement.clone(), env.clone())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, _) => {
                    let mut env: Env = metamodelica::nil();
                    env = extendEnvWithClassDef(inElement.clone(), inEnv.clone())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::EXTENDS { .. }, _) => {
                    let mut env: Env = metamodelica::nil();
                    env = extendEnvWithExtends(inElement.clone(), inEnv.clone())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::IMPORT { .. }, _) => {
                    let mut env: Env = metamodelica::nil();
                    env = extendEnvWithImport(inElement.clone(), inEnv.clone())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::DEFINEUNIT { .. }, _) => {
                    Ok(inEnv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

pub fn checkUniqueQualifiedImport(mut inImport: Import, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inImport.clone(), inImports.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let false = (List::isMemberOnTrue(inImport.clone(), inImports.clone(), (std::sync::Arc::new(fnptr!(compareQualifiedImportNames, Absyn::Import, Absyn::Import)) as std::sync::Arc<dyn ::std::ops::Fn(Absyn::Import, Absyn::Import) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::Import::NAMED_IMPORT { name, .. }, _, _) => {
                    Error::addSourceMessage(Error::MULTIPLE_QUALIFIED_IMPORTS_WITH_SAME_NAME.clone(), list![(name.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn compareQualifiedImportNames(mut inImport1: Import, mut inImport2: Import) -> bool {
    let mut outEqual: bool = false;
    outEqual = (match (inImport1.clone(), inImport2.clone()) {
        (Absyn::Import::NAMED_IMPORT { name: mut name1, .. }, Absyn::Import::NAMED_IMPORT { name: mut name2, .. }) if (stringEqual((name1.clone()).clone(), (name2.clone()).clone())) => {
            true
        },
        _ => {
            false
        },
    });
    outEqual
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn extendEnvWithEnumLiterals(mut inEnum: Arc<metamodelica::List<Arc<SCode::Enum>>>, mut inEnumPath: Arc<Absyn::Path>, mut inNextValue: i32, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inEnum.clone(), inEnumPath.clone(), inNextValue.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ metamodelica::List::Cons { head: lit, tail: rest_lits }, _, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            env = extendEnvWithEnum(lit.clone(), inEnumPath.clone(), inNextValue.clone(), inEnv.clone(), inInfo.clone())?;
            extendEnvWithEnumLiterals(rest_lits.clone(), inEnumPath.clone(), inNextValue.clone() + 1, env.clone(), inInfo.clone())?
        },
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            inEnv.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

fn extendEnvWithEnum(mut inEnum: Arc<SCode::Enum>, mut inEnumPath: Arc<Absyn::Path>, mut inValue: i32, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut enum_lit: Arc<SCode::Element>;
    let mut lit_name: ArcStr = arcstr::literal!("");
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut index: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inEnum.clone()) {
        Deref @ SCode::Enum { literal: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lit_name = __pa0.clone();
    index = (intString(inValue.clone())).clone();
    ty = Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("$EnumType")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (index.clone()).clone(), path: inEnumPath.clone() }) }), arrayDim: None });
    enum_lit = Arc::new(SCode::Element::COMPONENT { name: (lit_name.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: ty.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: inInfo.clone() });
    outEnv = extendEnvWithElement(enum_lit.clone(), inEnv.clone())?;
    Ok(outEnv)
}

pub fn extendEnvWithIterators(mut inIterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut iterIndex: i32, mut inEnv: Env) -> Env {
    let mut outEnv: Env = metamodelica::nil();
    let mut frame: Arc<Frame>;
    frame = newFrame(Some((literal!("$for$")).clone()), FrameType::IMPLICIT_SCOPE { iterIndex: iterIndex.clone() });
    outEnv = List::fold(inIterators.clone(), (std::sync::Arc::new(extendEnvWithIterator) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ForIterator>, Arc<metamodelica::List<Arc<Frame>>>) -> Result<Arc<metamodelica::List<Arc<Frame>>>> + 'static>), cons(frame.clone(), inEnv.clone()));
    outEnv
}

fn extendEnvWithIterator(mut inIterator: Arc<Absyn::ForIterator>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut iter_name: ArcStr = arcstr::literal!("");
    let mut iter: Arc<SCode::Element>;
    let __pa0 = ::match_deref::match_deref! { match &(inIterator.clone()) {
        Deref @ Absyn::ForIterator { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    iter_name = __pa0.clone();
    iter = Arc::new(SCode::Element::COMPONENT { name: (iter_name.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
    outEnv = extendEnvWithElement(iter.clone(), inEnv.clone())?;
    Ok(outEnv)
}

pub fn extendEnvWithMatch(mut inMatchExp: Arc<Absyn::Exp>, mut iterIndex: i32, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut frame: Arc<Frame>;
    let mut local_decls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    frame = newFrame(Some((literal!("$match$")).clone()), FrameType::IMPLICIT_SCOPE { iterIndex: iterIndex.clone() });
    let __pa0 = ::match_deref::match_deref! { match &(inMatchExp.clone()) {
        Deref @ Absyn::Exp::MATCHEXP { localDecls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    local_decls = __pa0.clone();
    outEnv = List::fold(local_decls.clone(), (std::sync::Arc::new(extendEnvWithElementItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementItem>, Arc<metamodelica::List<Arc<Frame>>>) -> Result<Arc<metamodelica::List<Arc<Frame>>>> + 'static>), cons(frame.clone(), inEnv.clone()));
    Ok(outEnv)
}

fn extendEnvWithElementItem(mut inElementItem: Arc<Absyn::ElementItem>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inElementItem.clone(), inEnv.clone())) {
        (Deref @ Absyn::ElementItem::ELEMENTITEM { element }, _) => {
            let mut el: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut env: Env = metamodelica::nil();
            el = AbsynToSCode::translateElement(element.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
            env = List::fold(el.clone(), (std::sync::Arc::new(extendEnvWithElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<Arc<Frame>>>) -> Result<Arc<metamodelica::List<Arc<Frame>>>> + 'static>), inEnv.clone());
            env.clone()
        },
        _ => {
            inEnv.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

pub fn getEnvName(mut inEnv: Env) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (AbsynUtil::pathString(getEnvPath(inEnv.clone())?, (literal!(".")).clone(), true, false)?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getEnvPath(mut inEnv: Env) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { frameType: FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: rest } => {
            getEnvPath(rest.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: Some(name), .. }, tail: Deref @ metamodelica::List::Nil } => {
            Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: Some(name), .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: None, .. }, tail: Deref @ metamodelica::List::Nil } } => {
            Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: Some(name), .. }, tail: rest } => {
            let mut path: Arc<Absyn::Path>;
            path = getEnvPath(rest.clone())?;
            path = AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
            path.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getScopeName(mut inEnv: Env) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: Some(name), .. }, tail: _ } => {
            name.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            getScopeName(rest.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn envPrefixOf(mut inPrefixEnv: Env, mut inEnv: Env) -> Result<bool> {
    let mut outIsPrefix: bool = false;
    outIsPrefix = envPrefixOf2(inPrefixEnv.clone().reverse(), inEnv.clone().reverse())?;
    Ok(outIsPrefix)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn envPrefixOf2(mut inPrefixEnv: Env, mut inEnv: Env) -> Result<bool> {
    let mut outIsPrefix: bool = false;
    outIsPrefix = 'mc: {
        let __mc_input = (inPrefixEnv.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: None, .. }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: None, .. }, tail: rest2 }) => {
                    Ok(envPrefixOf2(rest1.clone(), rest2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: Some(n1), .. }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: Some(n2), .. }, tail: rest2 }) => {
                    let true = (stringEqual((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(envPrefixOf2(rest1.clone(), rest2.clone())?)
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
    Ok(outIsPrefix)
}

pub fn envScopeNames(mut inEnv: Env) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outNames = envScopeNames2(inEnv.clone(), metamodelica::nil())?;
    Ok(outNames)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn envScopeNames2(mut inEnv: Env, mut inAccumNames: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outNames = (::match_deref::match_deref! { match &((inEnv.clone(), inAccumNames.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: Some(name), .. }, tail: rest_env }, _) => {
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            names = envScopeNames2(rest_env.clone(), cons((name.clone()).clone(), inAccumNames.clone()))?;
            names.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: None, .. }, tail: rest_env }, _) => {
            envScopeNames2(rest_env.clone(), inAccumNames.clone())?
        },
        (Deref @ metamodelica::List::Nil, _) => {
            inAccumNames.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNames)
}

pub fn envEqualPrefix(mut inEnv1: Env, mut inEnv2: Env) -> Result<Env> {
    let mut outPrefix: Env = metamodelica::nil();
    outPrefix = envEqualPrefix2(inEnv1.clone().reverse(), inEnv2.clone().reverse(), metamodelica::nil())?;
    Ok(outPrefix)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn envEqualPrefix2(mut inEnv1: Env, mut inEnv2: Env, mut inAccumEnv: Env) -> Result<Env> {
    let mut outPrefix: Env = metamodelica::nil();
    outPrefix = 'mc: {
        let __mc_input = (inEnv1.clone(), inEnv2.clone(), inAccumEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: frame @ Deref @ Frame { name: Some(name1), .. }, tail: rest_env1 }, Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: Some(name2), .. }, tail: rest_env2 }, _) => {
                    let mut env: Env = metamodelica::nil();
                    let true = (stringEq((name1.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    env = envEqualPrefix2(rest_env1.clone(), rest_env2.clone(), cons(frame.clone(), inAccumEnv.clone()))?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: None, .. }, tail: rest_env1 }, Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: None, .. }, tail: rest_env2 }, _) => {
                    Ok(envEqualPrefix2(rest_env1.clone(), rest_env2.clone(), inAccumEnv.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inAccumEnv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPrefix)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getItemInfo(mut inItem: Arc<Item>) -> Result<SourceInfo> {
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outInfo = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Item::VAR { var: Deref @ SCode::Element::COMPONENT { info, .. }, .. } => {
            info.clone()
        },
        Deref @ Item::CLASS { cls: Deref @ SCode::Element::CLASS { info, .. }, .. } => {
            info.clone()
        },
        Deref @ Item::ALIAS { info, .. } => {
            info.clone()
        },
        Deref @ Item::REDECLARED_ITEM { item, .. } => {
            getItemInfo(item.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInfo)
}

pub fn itemStr(mut inItem: Arc<Item>) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    outName = ('mc: {
        let __mc_input = inItem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Item::VAR { var: el, .. } => {
                    Ok(SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Item::CLASS { cls: el, .. } => {
                    Ok(SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Item::ALIAS { path: Some(path), name, .. } => {
                    let mut alias_str: ArcStr = arcstr::literal!("");
                    alias_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("alias ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" -> (")); __mm_s.push_str(&*alias_str.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Item::ALIAS { path: None, name, .. } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("alias ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" -> ()")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Item::REDECLARED_ITEM { item, .. } => {
                    let mut name: ArcStr = arcstr::literal!("");
                    name = (itemStr(item.clone())?).clone();
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("redeclared ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("UNHANDLED ITEM"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outName)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getItemName(mut inItem: Arc<Item>) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    outName = ((::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Item::VAR { var: Deref @ SCode::Element::COMPONENT { name, .. }, .. } => {
            name.clone()
        },
        Deref @ Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, .. }, .. } => {
            name.clone()
        },
        Deref @ Item::ALIAS { name, .. } => {
            name.clone()
        },
        Deref @ Item::REDECLARED_ITEM { item, .. } => {
            getItemName(item.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outName)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getItemEnv(mut inItem: Arc<Item>) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Item::CLASS { env, .. } => {
            env.clone()
        },
        Deref @ Item::REDECLARED_ITEM { item, .. } => {
            getItemEnv(item.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getItemEnvNoFail(mut inItem: Arc<Item>) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = 'mc: {
        let __mc_input = inItem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Item::CLASS { env, .. } => {
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Item::REDECLARED_ITEM { item, .. } => {
                    Ok(getItemEnvNoFail(item.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env: Env = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut f: Arc<Frame>;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NO ENV FOR ITEM: ")); __mm_s.push_str(&*getItemName(inItem.clone())?); ArcStr::from(__mm_s) }).clone();
                    f = newFrame(Some((r#str.clone()).clone()), crate::NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE);
                    env = list![f.clone()];
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn setItemEnv(mut inItem: Arc<Item>, mut inNewEnv: Env) -> Result<Arc<Item>> {
    let mut outItem: Arc<Item>;
    outItem = (::match_deref::match_deref! { match &((inItem.clone(), inNewEnv.clone())) {
        (Deref @ Item::CLASS { cls, env: _, classType: ct }, _) => {
            Arc::new(Item::CLASS { cls: cls.clone(), env: inNewEnv.clone(), classType: ct.clone() })
        },
        (Deref @ Item::REDECLARED_ITEM { item, .. }, _) => {
            setItemEnv(item.clone(), inNewEnv.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outItem)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn mergeItemEnv(mut inItem: Arc<Item>, mut inEnv: Env) -> Env {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
        (Deref @ Item::CLASS { env: Deref @ metamodelica::List::Cons { head: cls_env, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            enterFrame(cls_env.clone(), inEnv.clone())
        },
        (Deref @ Item::REDECLARED_ITEM { item, .. }, _) => {
            mergeItemEnv(item.clone(), inEnv.clone())
        },
        _ => {
            inEnv.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEnv
}

pub fn unmergeItemEnv(mut inItem: Arc<Item>, mut inEnv: Env) -> Env {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: _, tail: env }) => {
            env.clone()
        },
        _ => {
            inEnv.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEnv
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getItemPrefixes(mut inItem: Arc<Item>) -> Result<Arc<SCode::Prefixes>> {
    let mut outPrefixes: Arc<SCode::Prefixes>;
    outPrefixes = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Item::CLASS { cls: Deref @ SCode::Element::CLASS { prefixes: pf, .. }, .. } => {
            pf.clone()
        },
        Deref @ Item::VAR { var: Deref @ SCode::Element::COMPONENT { prefixes: pf, .. }, .. } => {
            pf.clone()
        },
        Deref @ Item::REDECLARED_ITEM { item, .. } => {
            getItemPrefixes(item.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPrefixes)
}

pub fn resolveRedeclaredItem(mut inItem: Arc<Item>, mut inEnv: Env) -> (Arc<Item>, Env, Arc<metamodelica::List<(Arc<Item>, Arc<metamodelica::List<Arc<Frame>>>)>>) {
    let mut outItem: Arc<Item>;
    let mut outEnv: Env = metamodelica::nil();
    let mut outPreviousItem: Arc<metamodelica::List<(Arc<Item>, Arc<metamodelica::List<Arc<Frame>>>)>> = metamodelica::nil();
    (outItem, outEnv, outPreviousItem) = (::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
        (Deref @ Item::REDECLARED_ITEM { declaredEnv: env, item }, _) => {
            (item.clone(), env.clone(), list![(inItem.clone(), inEnv.clone())])
        },
        _ => {
            (inItem.clone(), inEnv.clone(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outItem, outEnv, outPreviousItem)
}

pub fn getEnvExtendsTable(mut inEnv: Env) -> Result<Arc<ExtendsTable>> {
    let mut outExtendsTable: Arc<ExtendsTable>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { extendsTable: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outExtendsTable = __pa0.clone();
    Ok(outExtendsTable)
}

pub fn getEnvExtendsFromTable(mut inEnv: Env) -> Result<Arc<metamodelica::List<Arc<Extends>>>> {
    let mut outExtends: Arc<metamodelica::List<Arc<Extends>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(getEnvExtendsTable(inEnv.clone())?) {
        Deref @ ExtendsTable { baseClasses: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outExtends = __pa0.clone();
    Ok(outExtends)
}

pub fn getDerivedClassRedeclares(mut inDerivedName: ArcStr, mut inTypeSpec: Arc<Absyn::TypeSpec>, mut inEnv: Env) -> Result<Arc<metamodelica::List<Arc<Redeclaration>>>> {
    let mut outRedeclarations: Arc<metamodelica::List<Arc<Redeclaration>>> = metamodelica::nil();
    outRedeclarations = 'mc: {
        let __mc_input = (inDerivedName.clone(), inTypeSpec.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: _ }, _) => {
                    let mut bc: Arc<Absyn::Path>;
                    let mut rm: Arc<metamodelica::List<Arc<Redeclaration>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getEnvExtendsFromTable(inEnv.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ Extends { redeclareModifiers: __pa0, baseClass: __pa1, .. }, tail: Deref @ metamodelica::List::Nil } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rm = __pa0.clone();
                    bc = __pa1.clone();
                    let true = (AbsynUtil::pathSuffixOf(path.clone(), bc.clone())?) else { bail!("pattern mismatch") };
                    Ok(rm.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: _ }, _) => {
                    let mut bc: Arc<Absyn::Path>;
                    let mut rm: Arc<metamodelica::List<Arc<Redeclaration>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getEnvExtendsFromTable(inEnv.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ Extends { redeclareModifiers: __pa0, baseClass: __pa1, .. }, tail: Deref @ metamodelica::List::Nil } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rm = __pa0.clone();
                    bc = __pa1.clone();
                    let false = (AbsynUtil::pathSuffixOf(path.clone(), bc.clone())?) else { bail!("pattern mismatch") };
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Derived paths are not the same: ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" != ")); __mm_s.push_str(&*AbsynUtil::pathString(bc.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(rm.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRedeclarations)
}

pub fn setEnvExtendsTable(mut inExtendsTable: Arc<ExtendsTable>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut imps: ImportTable;
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let mut rest_env: Env = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: __pa0, frameType: __pa1, clsAndVars: __pa2, extendsTable: _, importTable: __pa3, isUsed: __pa4 }, tail: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    tree = __pa2.clone();
    imps = __pa3.clone();
    is_used = __pa4.clone();
    rest_env = __pa5.clone();
    outEnv = cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: inExtendsTable.clone(), importTable: imps.clone(), isUsed: is_used.clone() }), rest_env.clone());
    Ok(outEnv)
}

pub fn setEnvClsAndVars(mut inTree: Arc<EnvTree::Tree>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut ty: FrameType = FrameType::ENCAPSULATED_SCOPE;
    let mut ext: Arc<ExtendsTable>;
    let mut imps: ImportTable;
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let mut rest_env: Env = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: __pa0, frameType: __pa1, clsAndVars: _, extendsTable: __pa2, importTable: __pa3, isUsed: __pa4 }, tail: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    ext = __pa2.clone();
    imps = __pa3.clone();
    is_used = __pa4.clone();
    rest_env = __pa5.clone();
    outEnv = cons(Arc::new(Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: inTree.clone(), extendsTable: ext.clone(), importTable: imps.clone(), isUsed: is_used.clone() }), rest_env.clone());
    Ok(outEnv)
}

pub fn mergePathWithEnvPath(mut inPath: Arc<Absyn::Path>, mut inEnv: Env) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = (inPath.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut env_path: Arc<Absyn::Path>;
                    let mut id: ArcStr = arcstr::literal!("");
                    env_path = getEnvPath(inEnv.clone())?;
                    id = (AbsynUtil::pathLastIdent(inPath.clone())?).clone();
                    Ok(AbsynUtil::joinPaths(env_path.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inPath.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

pub fn mergeTypeSpecWithEnvPath(mut inTS: Arc<Absyn::TypeSpec>, mut inEnv: Env) -> Result<Arc<Absyn::TypeSpec>> {
    let mut outTS: Arc<Absyn::TypeSpec>;
    outTS = 'mc: {
        let __mc_input = (inTS.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: ad }, _) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut path = (*path).clone();
                    id = (AbsynUtil::pathLastIdent(path.clone())?).clone();
                    path = AbsynUtil::joinPaths(getEnvPath(inEnv.clone())?, Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
                    Ok(Arc::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: ad.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inTS.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTS)
}

pub fn prefixIdentWithEnv(mut inIdent: ArcStr, mut inEnv: Env) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inIdent.clone(), inEnv.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: Deref @ Frame { name: None, .. }, tail: Deref @ metamodelica::List::Nil }) => {
            Arc::new(Absyn::Path::IDENT { name: (inIdent.clone()).clone() })
        },
        _ => {
            let mut path: Arc<Absyn::Path>;
            path = getEnvPath(inEnv.clone())?;
            path = AbsynUtil::suffixPath(path.clone(), (inIdent.clone()).clone())?;
            path.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getRedeclarationElement(mut inRedeclare: Arc<Redeclaration>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &(inRedeclare.clone()) {
        Deref @ Redeclaration::RAW_MODIFIER { modifier: e } => {
            e.clone()
        },
        Deref @ Redeclaration::PROCESSED_MODIFIER { modifier: Deref @ Item::CLASS { cls: e, .. } } => {
            e.clone()
        },
        Deref @ Redeclaration::PROCESSED_MODIFIER { modifier: Deref @ Item::VAR { var: e, .. } } => {
            e.clone()
        },
        Deref @ Redeclaration::PROCESSED_MODIFIER { modifier: Deref @ Item::REDECLARED_ITEM { item, .. } } => {
            getRedeclarationElement(Arc::new(Redeclaration::PROCESSED_MODIFIER { modifier: item.clone() }))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

pub fn getRedeclarationNameInfo(mut inRedeclare: Arc<Redeclaration>) -> Result<(ArcStr, SourceInfo)> {
    let mut outName: ArcStr = arcstr::literal!("");
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    (outName, outInfo) = (::match_deref::match_deref! { match &(inRedeclare.clone()) {
        Deref @ Redeclaration::PROCESSED_MODIFIER { modifier: Deref @ Item::ALIAS { info, name, .. } } => {
            (name.clone(), info.clone())
        },
        _ => {
            let mut el: Arc<SCode::Element>;
            let mut name: ArcStr = arcstr::literal!("");
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            el = getRedeclarationElement(inRedeclare.clone())?;
            (name, info) = SCodeUtil::elementNameInfo(el.clone())?;
            (name.clone(), info.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outName, outInfo))
}

pub fn buildInitialEnv() -> Result<Env> {
    let mut outInitialEnv: Env = metamodelica::nil();
    let mut tree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut exts: Arc<ExtendsTable>;
    let mut imps: ImportTable;
    let mut is_used: Mutable::Mutable<bool>;
    let mut p: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut initialClasses: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    tree = EnvTree::new();
    exts = newExtendsTable();
    imps = newImportTable();
    is_used = Mutable::create(false);
    tree = addDummyClassToTree((literal!("String")).clone(), tree.clone())?;
    tree = addDummyClassToTree((literal!("Integer")).clone(), tree.clone())?;
    tree = addDummyClassToTree((literal!("spliceFunction")).clone(), tree.clone())?;
    outInitialEnv = list![Arc::new(Frame { name: None, frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: tree.clone(), extendsTable: exts.clone(), importTable: imps.clone(), isUsed: Some(is_used.clone()) })];
    (_, p) = FBuiltin::getInitialFunctions()?;
    outInitialEnv = extendEnvWithClasses(p.clone(), outInitialEnv.clone());
    Ok(outInitialEnv)
}

fn addDummyClassToTree(mut inName: ArcStr, mut inTree: Arc<EnvTree::Tree>) -> Result<Arc<EnvTree::Tree>> {
    let mut outTree: Arc<EnvTree::Tree> = Arc::new(EnvTree::Tree::EMPTY);
    let mut cls: Arc<SCode::Element>;
    cls = Arc::new(SCode::Element::CLASS { name: (inName.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() });
    outTree = EnvTree::add(inTree.clone(), (inName.clone()).clone(), Arc::new(Item::CLASS { cls: cls.clone(), env: emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::BUILTIN }), (std::sync::Arc::new(fnptr!(EnvTree::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
    Ok(outTree)
}

pub fn printEnvStr(mut inEnv: Env) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut env: Env = metamodelica::nil();
    env = inEnv.clone().reverse();
    outString = stringDelimitList(List::map(env.clone(), (std::sync::Arc::new(printFrameStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Frame>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone());
    outString
}

fn printFrameStr(mut inFrame: Arc<Frame>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inFrame.clone()) {
        Deref @ Frame { name, frameType: ty, clsAndVars: tree, extendsTable: exts, importTable: imps, isUsed: _ } => {
            let mut name_str: ArcStr = arcstr::literal!("");
            let mut ty_str: ArcStr = arcstr::literal!("");
            let mut tree_str: ArcStr = arcstr::literal!("");
            let mut ext_str: ArcStr = arcstr::literal!("");
            let mut imp_str: ArcStr = arcstr::literal!("");
            let mut out: ArcStr = arcstr::literal!("");
            name_str = (printFrameNameStr(name.clone())).clone();
            ty_str = (printFrameTypeStr(ty.clone())?).clone();
            tree_str = (EnvTree::printTreeStr(tree.clone())?).clone();
            ext_str = (printExtendsTableStr(exts.clone())?).clone();
            imp_str = (printImportTableStr(imps.clone())?).clone();
            name_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<<<")); __mm_s.push_str(&*ty_str.clone()); __mm_s.push_str(&*literal!(" frame ")); __mm_s.push_str(&*name_str.clone()); __mm_s.push_str(&*literal!(">>>\n")); ArcStr::from(__mm_s) }).clone();
            out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name_str.clone()); __mm_s.push_str(&*literal!("\tImports:\n")); __mm_s.push_str(&*imp_str.clone()); __mm_s.push_str(&*literal!("\n\tExtends:\n")); __mm_s.push_str(&*ext_str.clone()); __mm_s.push_str(&*literal!("\n\tComponents:\n")); __mm_s.push_str(&*tree_str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            out.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn printFrameNameStr(mut inFrame: Option<ArcStr>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inFrame.clone() {
        None => {
            literal!("global")
        },
        Some(mut name) => {
            name.clone()
        },
    })).clone();
    outString
}

fn printFrameTypeStr(mut inFrame: FrameType) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inFrame.clone() {
        FrameType::NORMAL_SCOPE { .. } => literal!("Normal"),
        FrameType::ENCAPSULATED_SCOPE { .. } => literal!("Encapsulated"),
        FrameType::IMPLICIT_SCOPE { .. } => literal!("Implicit"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn printExtendsTableStr(mut inExtendsTable: Arc<ExtendsTable>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut bcl: Arc<metamodelica::List<Arc<Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExtendsTable.clone()) {
        Deref @ ExtendsTable { classExtendsInfo: __pa0, redeclaredElements: __pa1, baseClasses: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cei = __pa0.clone();
    re = __pa1.clone();
    bcl = __pa2.clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(bcl.clone(), (std::sync::Arc::new(printExtendsStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Extends>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n\t\tRedeclare elements:\n\t\t\t")); __mm_s.push_str(&*stringDelimitList(List::map1(re.clone(), (std::sync::Arc::new(SCodeDump::unparseElementStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, SCodeDump::SCodeDumpOptions) -> Result<ArcStr> + 'static>), SCodeDump::defaultOptions.clone()), (literal!("\n\t\t\t")).clone())); __mm_s.push_str(&*literal!("\n\t\tClass extends:\n\t\t\t")); __mm_s.push_str(&*Util::applyOptionOrDefault(cei.clone(), Arc::new({ let __pe_b1 = SCodeDump::defaultOptions.clone(); move |__pe_a0| SCodeDump::unparseElementStr(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone())); ArcStr::from(__mm_s) }).clone();
    Ok(outString)
}

pub fn printExtendsStr(mut inExtends: Arc<Extends>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut bc: Arc<Absyn::Path>;
    let mut mods: Arc<metamodelica::List<Arc<Redeclaration>>> = metamodelica::nil();
    let mut mods_str: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExtends.clone()) {
        Deref @ Extends { redeclareModifiers: __pa0, baseClass: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    mods = __pa0.clone();
    bc = __pa1.clone();
    mods_str = stringDelimitList(List::map(mods.clone(), (std::sync::Arc::new(printRedeclarationStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Redeclaration>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone());
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t\t")); __mm_s.push_str(&*AbsynUtil::pathString(bc.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*mods_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(outString)
}

pub fn printRedeclarationStr(mut inRedeclare: Arc<Redeclaration>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inRedeclare.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Redeclaration::PROCESSED_MODIFIER { modifier: Deref @ Item::ALIAS { path: Some(p), name, .. } } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ALIAS(")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Redeclaration::PROCESSED_MODIFIER { modifier: Deref @ Item::ALIAS { name, .. } } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ALIAS(")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(SCodeDump::unparseElementStr(getRedeclarationElement(inRedeclare.clone())?, SCodeDump::defaultOptions.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn printImportTableStr(mut inImports: ImportTable) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut qual_imps: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut unqual_imps: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut qual_str: ArcStr = arcstr::literal!("");
    let mut unqual_str: ArcStr = arcstr::literal!("");
    let ImportTable { unqualifiedImports: __pa0, qualifiedImports: __pa1, .. } = (inImports.clone()) else { bail!("pattern mismatch") };
    unqual_imps = __pa0.clone();
    qual_imps = __pa1.clone();
    qual_str = stringDelimitList(List::map(qual_imps.clone(), (std::sync::Arc::new(AbsynUtil::printImportString) as std::sync::Arc<dyn ::std::ops::Fn(Absyn::Import) -> Result<ArcStr> + 'static>)), (literal!("\n\t\t")).clone());
    unqual_str = stringDelimitList(List::map(unqual_imps.clone(), (std::sync::Arc::new(AbsynUtil::printImportString) as std::sync::Arc<dyn ::std::ops::Fn(Absyn::Import) -> Result<ArcStr> + 'static>)), (literal!("\n\t\t")).clone());
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t\t")); __mm_s.push_str(&*qual_str.clone()); __mm_s.push_str(&*unqual_str.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(outString)
}

