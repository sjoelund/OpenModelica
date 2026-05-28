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

use crate::AvlSetCR;
use crate::DAEUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Config;
use openmodelica_util_datatypes_basic::Mutable;

// ************************ FNode structures ***************************
// ************************ FNode structures ***************************
// ************************ FNode structures ***************************
// ************************ FNode structures ***************************
/// an identifier is just a string
pub type Name = ArcStr;

/// list of names
pub type Names = Arc<metamodelica::List<ArcStr>>;

pub type Import = Absyn::Import;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub static emptyImportTable: std::sync::LazyLock<ImportTable> = std::sync::LazyLock::new(|| { ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() } });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImportTable {
    /// If true means that the imports are hidden.
    pub hidden: bool,
    pub qualifiedImports: Arc<metamodelica::List<Absyn::Import>>,
    pub unqualifiedImports: Arc<metamodelica::List<Absyn::Import>>,
}

impl Default for ImportTable {
    fn default() -> Self {
        Self {
            hidden: Default::default(),
            qualifiedImports: Default::default(),
            unqualifiedImports: Default::default(),
        }
    }
}

pub type IMPORT_TABLE = ImportTable;


/// array of 1
pub type Ref = metamodelica::Array<Node>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    /// node name, class/component/extends name, etc. see also *NodeName in above
    pub name: Name,
    /// Unique node id
    pub id: Id,
    /// A node can have several parents depending on the context
    pub parents: Parents,
    /// List of uniquely named classes and variables
    pub children: Children,
    /// More data for this node, Class, Var, etc
    pub data: Data,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: Default::default(),
            id: Default::default(),
            parents: Default::default(),
            children: Default::default(),
            data: Default::default(),
        }
    }
}

pub type N = Node;


/// Used to know where a modifier came from, for error reporting.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModScope {
    MS_COMPONENT {
        name: ArcStr,
    },
    MS_EXTENDS {
        path: Arc<Absyn::Path>,
    },
    MS_DERIVED {
        path: Arc<Absyn::Path>,
    },
    MS_CLASS_EXTENDS {
        name: ArcStr,
    },
    MS_CONSTRAINEDBY {
        path: Arc<Absyn::Path>,
    },
}
pub use self::ModScope::{MS_COMPONENT,MS_EXTENDS,MS_DERIVED,MS_CLASS_EXTENDS,MS_CONSTRAINEDBY};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Data {
    /// top
    TOP,
    IT {
        /// instantiated component
        i: Arc<DAE::Var>,
    },
    /// import
    IM {
        /// imports
        i: ImportTable,
    },
    /// class
    CL {
        e: Arc<SCode::Element>,
        pre: DAE::Prefix,
        /// modification
        r#mod: Arc<DAE::Mod>,
        /// usedefined, builtin, basic type
        kind: Kind,
        /// if it is untyped, typed or fully instantiated (dae)
        status: Status,
    },
    /// component
    CO {
        e: Arc<SCode::Element>,
        /// modification
        r#mod: Arc<DAE::Mod>,
        /// usedefined, builtin, basic type
        kind: Kind,
        /// if it is untyped, typed or fully instantiated (dae)
        status: Status,
    },
    /// extends
    EX {
        e: Arc<SCode::Element>,
        /// modification
        r#mod: Arc<DAE::Mod>,
    },
    /// units
    DU {
        els: Arc<metamodelica::List<Arc<SCode::Element>>>,
    },
    /// function type nodes
    FT {
        /// list since several types with the same name can exist in the same scope (overloading)
        tys: Arc<metamodelica::List<Arc<DAE::Type>>>,
    },
    /// algorithm section
    AL {
        /// al or ial (initial)
        name: Name,
        a: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>,
    },
    /// equation section
    EQ {
        /// eq or ieq (initial)
        name: Name,
        e: Arc<metamodelica::List<Arc<SCode::Equation>>>,
    },
    /// optimization
    OT {
        constrainLst: Arc<metamodelica::List<SCode::ConstraintSection>>,
        clsAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>,
    },
    /// external declaration
    ED {
        ed: Arc<SCode::ExternalDecl>,
    },
    /// for iterators scope
    FS {
        fis: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>,
    },
    /// for iterator
    FI {
        fi: Arc<Absyn::ForIterator>,
    },
    /// match scope
    MS {
        e: Arc<Absyn::Exp>,
    },
    /// mod
    MO {
        m: Arc<SCode::Mod>,
    },
    /// binding, condition, array dim, etc
    EXP {
        /// what is the expression for
        name: ArcStr,
        e: Arc<Absyn::Exp>,
    },
    /// component reference
    CR {
        r: Arc<Absyn::ComponentRef>,
    },
    /// dimensions
    DIMS {
        /// what are the dimensions for, type or component
        name: ArcStr,
        dims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>,
    },
    /// constrainedby class
    CC {
        cc: Arc<SCode::ConstrainClass>,
    },
    /// reference node
    REF {
        target: Scope,
    },
    /// no data
    ND {
        scopeType: Option<ScopeType>,
    },
    /// version node, contains the node that decided the generation of the clone
    VR {
        source: Scope,
        p: DAE::Prefix,
        m: Arc<DAE::Mod>,
        scopeType: Option<ScopeType>,
    },
    /// an assertion node, to be used in places
    ///    where we want to assert things in the graph.
    ///    for example if we looked up A.B from A.B.C.D
    ///    but could not find C then we add an assertion
    ///    node. we have just a message here but might
    ///    add new info later on.
    ASSERT {
        message: ArcStr,
    },
    /// status node
    STATUS {
        isInstantiating: bool,
    },
}
impl Default for Data {
    fn default() -> Self { Self::TOP }
}
pub use self::Data::{TOP,IT,IM,CL,CO,EX,DU,FT,AL,EQ,OT,ED,FS,FI,MS,MO,EXP,CR,DIMS,CC,REF,ND,VR,ASSERT,STATUS};

pub type Refs = Arc<metamodelica::List<metamodelica::Array<Node>>>;

pub type Parents = Arc<metamodelica::List<metamodelica::Array<Node>>>;

pub type Scope = Arc<metamodelica::List<metamodelica::Array<Node>>>;

pub type Children = Arc<RefTree::Tree>;

thread_local! { static __emptyScope_TLS: Arc<metamodelica::List<metamodelica::Array<Node>>> = metamodelica::nil(); }
pub fn emptyScope() -> Arc<metamodelica::List<metamodelica::Array<Node>>> { __emptyScope_TLS.with(|__t| __t.clone()) }

pub mod RefTree {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = metamodelica::Array<Node>;

    pub fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (inKey.clone()).clone();
        outString
    }

    pub fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        let Node { name: __pa0, .. } = (inValue.clone().borrow()[(1-1) as usize].clone()) else { bail!("pattern mismatch") };
        outString = __pa0.clone();
        Ok(outString)
    }

    pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
        let mut outResult: i32 = 0;
        outResult = stringCompare((inKey1.clone()).clone(), (inKey2.clone()).clone());
        outResult
    }

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
    impl Default for Tree {
        fn default() -> Self { Self::EMPTY }
    }
    pub use self::Tree::{NODE,LEAF,EMPTY};

    pub type ValueNode = ArcStr;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(metamodelica::Array<Node>, metamodelica::Array<Node>, ArcStr) -> Result<metamodelica::Array<Node>> + 'static>) -> Result<Arc<Tree>> {
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
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }), right: Arc::new(crate::FCore::RefTree::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::FCore::RefTree::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }) });
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

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(ArcStr, metamodelica::Array<Node>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(metamodelica::Array<Node>, metamodelica::Array<Node>, ArcStr) -> Result<metamodelica::Array<Node>> + 'static>) -> Result<Arc<Tree>> {
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

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<metamodelica::Array<Node>>) -> Result<metamodelica::Array<Node>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::Array<Node>>) -> Result<Value> + 'static>;

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
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }), right: Arc::new(crate::FCore::RefTree::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::FCore::RefTree::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }) });
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<Node>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
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

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<Node>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
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

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<Node>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
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

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<Node>) -> Result<()> + 'static>) -> Result<()> {
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

    pub fn fromList(mut inValues: Arc<metamodelica::List<(ArcStr, metamodelica::Array<Node>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(metamodelica::Array<Node>, metamodelica::Array<Node>, ArcStr) -> Result<metamodelica::Array<Node>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::FCore::RefTree::Tree::EMPTY);
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
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Option<metamodelica::Array<Node>> {
        let mut value: Option<metamodelica::Array<Node>> = None;
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(metamodelica::Array<Node>, metamodelica::Array<Node>, ArcStr) -> Result<metamodelica::Array<Node>> + 'static>) -> Result<Arc<Tree>> {
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

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<metamodelica::Array<Node>>>) -> Arc<metamodelica::List<metamodelica::Array<Node>>> {
        let mut lst: Arc<metamodelica::List<metamodelica::Array<Node>>> = lst;
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

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<Node>) -> Result<metamodelica::Array<Node>> + 'static>) -> Arc<Tree> {
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

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<Node>, FT) -> Result<(metamodelica::Array<Node>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
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
        let mut outTree: Arc<Tree> = Arc::new(crate::FCore::RefTree::Tree::EMPTY);
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::FCore::RefTree::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::FCore::RefTree::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::FCore::RefTree::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::FCore::RefTree::Tree::EMPTY), node.clone())?
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

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(ArcStr, metamodelica::Array<Node>)>>) -> Arc<metamodelica::List<(ArcStr, metamodelica::Array<Node>)>> {
        let mut lst: Arc<metamodelica::List<(ArcStr, metamodelica::Array<Node>)>> = lst;
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
        let mut outTree: Arc<Tree> = add(tree.clone(), (key.clone()).clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, metamodelica::Array<Node>, metamodelica::Array<Node>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Node>, metamodelica::Array<Node>, ArcStr) -> Result<metamodelica::Array<Node>> + 'static>)).unwrap();
        outTree
    }

}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    USERDEFINED,
    BUILTIN,
    BASIC_TYPE,
}
pub use self::Kind::{USERDEFINED,BUILTIN,BASIC_TYPE};

/// Used to distinguish between different phases of the instantiation of a component
/// A component is first added to environment untyped. It can thereafter be instantiated to get its type
/// and finally instantiated to produce the DAE. These three states are indicated by this datatype.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    /// Untyped variables, initially added to env
    VAR_UNTYPED,
    /// Typed variables, when instantiation to get type has been performed
    VAR_TYPED,
    /// Typed variables that also have been instantiated to generate dae. Required to distinguish
    ///                  between typed variables without DAE to know when to skip multiply declared dae elements
    VAR_DAE,
    /// A conditional variable that was deleted.
    VAR_DELETED,
    /// just added to the env
    CLS_UNTYPED,
    /// partially instantiated
    CLS_PARTIAL,
    /// fully instantiated
    CLS_FULL,
    /// a class that was generated for a component
    CLS_INSTANCE {
        instanceOf: ArcStr,
    },
}
pub use self::Status::{VAR_UNTYPED,VAR_TYPED,VAR_DAE,VAR_DELETED,CLS_UNTYPED,CLS_PARTIAL,CLS_FULL,CLS_INSTANCE};

// ************************ FVisit structures ***************************
// ************************ FVisit structures ***************************
// ************************ FVisit structures ***************************
// ************************ FVisit structures ***************************
/// Visit Node Info
/// Visit Node Info
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Visit {
    /// which node it is
    pub r#ref: Ref,
    /// order in which was visited
    pub seq: Seq,
}

impl Default for Visit {
    fn default() -> Self {
        Self {
            r#ref: Default::default(),
            seq: Default::default(),
        }
    }
}

pub type VN = Visit;


/// Visited structure is an AvlTree Id <-> Visit
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Visited {
    pub tree: Arc<VAvlTree>,
    /// the next visit node id
    pub next: Next,
}

impl Default for Visited {
    fn default() -> Self {
        Self {
            tree: Default::default(),
            next: Default::default(),
        }
    }
}

pub type V = Visited;


pub type VAvlKey = i32;

pub type VAvlValue = Visit;

/// The binary tree data structure for visited
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VAvlTree {
    /// Value
    pub value: Option<VAvlTreeValue>,
    /// heigth of tree, used for balancing
    pub height: i32,
    /// left subtree
    pub left: Option<Arc<VAvlTree>>,
    /// right subtree
    pub right: Option<Arc<VAvlTree>>,
}

impl Default for VAvlTree {
    fn default() -> Self {
        Self {
            value: Default::default(),
            height: Default::default(),
            left: Default::default(),
            right: Default::default(),
        }
    }
}

pub type VAVLTREENODE = VAvlTree;


/// Each node in the binary tree can have a value associated with it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VAvlTreeValue {
    /// Key
    pub key: VAvlKey,
    /// Value
    pub value: VAvlValue,
}

impl Default for VAvlTreeValue {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
        }
    }
}

pub type VAVLTREEVALUE = VAvlTreeValue;


thread_local! { static __emptyVAvlTree_TLS: Arc<VAvlTree> = Arc::new(VAvlTree { value: None, height: 0, left: None, right: None }); }
pub fn emptyVAvlTree() -> Arc<VAvlTree> { __emptyVAvlTree_TLS.with(|__t| __t.clone()) }

// ************************ FGraph structures ***************************
// ************************ FGraph structures ***************************
// ************************ FGraph structures ***************************
// ************************ FGraph structures ***************************
pub static dummyTopModel: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::IDENT { name: (literal!("$EMPTY")).clone() }) });

pub static dummyExtra: std::sync::LazyLock<Extra> = std::sync::LazyLock::new(|| { Extra { topModel: dummyTopModel.clone() } });

pub const recordConstructorSuffix: &'static str = "$recordconstructor";

pub const forScopeName: &'static str = "$for loop scope$";

pub const forIterScopeName: &'static str = "$foriter loop scope$";

pub const parForScopeName: &'static str = "$pafor loop scope$";

pub const parForIterScopeName: &'static str = "$parforiter loop scope$";

pub const matchScopeName: &'static str = "$match scope$";

pub const caseScopeName: &'static str = "$case scope$";

pub const patternTypeScope: &'static str = "$pattern type scope$";

pub static implicitScopeNames: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(arcstr::literal!(forScopeName)).clone(), (arcstr::literal!(forIterScopeName)).clone(), (arcstr::literal!(parForScopeName)).clone(), (arcstr::literal!(parForIterScopeName)).clone(), (arcstr::literal!(matchScopeName)).clone(), (arcstr::literal!(caseScopeName)).clone(), (arcstr::literal!(patternTypeScope)).clone()] });

/// propagate more info into env if needed
/// propagate more info into env if needed
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Extra {
    pub topModel: Arc<Absyn::Path>,
}

impl Default for Extra {
    fn default() -> Self {
        Self {
            topModel: Default::default(),
        }
    }
}

pub type EXTRA = Extra;


/// graph
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Graph {
    /// graph
    G {
        /// the top node
        top: Top,
        /// current scope
        scope: Scope,
    },
    /// empty graph
    EG {
        name: Name,
    },
}
impl Default for Graph {
    fn default() -> Self {
        Self::EG {
            name: Default::default(),
        }
    }
}
pub use self::Graph::{G,EG};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Top {
    pub graph: metamodelica::Array<Graph>,
    /// name of the graph
    pub name: Name,
    /// the top node
    pub node: Ref,
    /// extra information
    pub extra: Extra,
}

impl Default for Top {
    fn default() -> Self {
        Self {
            graph: Default::default(),
            name: Default::default(),
            node: Default::default(),
            extra: Default::default(),
        }
    }
}

pub type GTOP = Top;


pub const firstId: i32 = 0;

// ************************ Cache structures ***************************
// ************************ Cache structures ***************************
// ************************ Cache structures ***************************
// ************************ Cache structures ***************************
pub type StructuralParameters = (Arc<AvlSetCR::Tree>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cache {
    CACHE {
        /// and the initial environment
        initialGraph: Option<Graph>,
        /// set of Option<DAE.Function>; NONE() means instantiation started; SOME() means it's finished
        functions: Mutable::Mutable<Arc<AvlTreePathFunction::Tree>>,
        /// ht of prefixed crefs and a stack of evaluated but not yet prefix crefs
        evaluatedParams: StructuralParameters,
        /// name of the model being instantiated
        modelName: Arc<Absyn::Path>,
    },
    /// no cache
    NO_CACHE,
}
impl Default for Cache {
    fn default() -> Self { Self::NO_CACHE }
}
pub use self::Cache::{CACHE,NO_CACHE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeType {
    FUNCTION_SCOPE,
    CLASS_SCOPE,
    PARALLEL_SCOPE,
}
pub use self::ScopeType::{FUNCTION_SCOPE,CLASS_SCOPE,PARALLEL_SCOPE};

// ************************ functions ***************************
pub fn next(mut inext: Next) -> Next {
    let mut onext: Next = 0;
    onext = inext.clone() + 1;
    onext
}

pub fn emptyCache() -> Cache {
    let mut cache: Cache = Cache::NO_CACHE;
    let mut instFuncs: Mutable::Mutable<Arc<AvlTreePathFunction::Tree>>;
    let mut ht: StructuralParameters;
    instFuncs = Mutable::create(Arc::new(openmodelica_frontend_dump::AvlTreePathFunction::Tree::EMPTY));
    ht = (Arc::new(crate::AvlSetCR::Tree::EMPTY), metamodelica::nil());
    cache = Cache::CACHE { initialGraph: None, functions: instFuncs.clone(), evaluatedParams: ht.clone(), modelName: Arc::new(Absyn::Path::IDENT { name: (literal!("##UNDEFINED##")).clone() }) };
    cache
}

pub fn noCache() -> Cache {
    let mut cache: Cache = Cache::NO_CACHE;
    cache = crate::FCore::Cache::NO_CACHE;
    cache
}

pub fn addEvaluatedCref(mut cache: Cache, mut var: SCode::Variability, mut cr: Arc<DAE::ComponentRef>) -> Cache {
    let mut ocache: Cache = Cache::NO_CACHE;
    ocache = (::match_deref::match_deref! { match &((cache.clone(), var.clone())) {
        (Cache::CACHE { initialGraph, functions, evaluatedParams: (ht, Deref @ metamodelica::List::Cons { head: crs, tail: st }), modelName: p }, SCode::Variability::PARAM) => {
            Cache::CACHE { initialGraph: initialGraph.clone(), functions: functions.clone(), evaluatedParams: (ht.clone(), cons(cons(cr.clone(), crs.clone()), st.clone())), modelName: p.clone() }
        },
        (Cache::CACHE { initialGraph, functions, evaluatedParams: (ht, Deref @ metamodelica::List::Nil), modelName: p }, SCode::Variability::PARAM) => {
            Cache::CACHE { initialGraph: initialGraph.clone(), functions: functions.clone(), evaluatedParams: (ht.clone(), cons(list![cr.clone()], metamodelica::nil())), modelName: p.clone() }
        },
        _ => {
            cache.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ocache
}

pub fn getEvaluatedParams(mut cache: Cache) -> Result<Arc<AvlSetCR::Tree>> {
    let mut ht: Arc<AvlSetCR::Tree> = Arc::new(AvlSetCR::Tree::EMPTY);
    let Cache::CACHE { evaluatedParams: (__pa0, _), .. } = (cache.clone()) else { bail!("pattern mismatch") };
    ht = __pa0.clone();
    Ok(ht)
}

pub fn printNumStructuralParameters(mut cache: Cache) -> Result<()> {
    let mut crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(cache.clone()) {
        Cache::CACHE { evaluatedParams: (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }), .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crs = __pa0.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("printNumStructuralParameters: ")); __mm_s.push_str(&*intString((crs.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn setCacheClassName(mut inCache: Cache, mut p: Arc<Absyn::Path>) -> Cache {
    let mut outCache: Cache = Cache::NO_CACHE;
    outCache = (match inCache.clone() {
        Cache::CACHE { initialGraph: mut igraph, functions: mut ef, evaluatedParams: mut ht, modelName: _ } => {
            Cache::CACHE { initialGraph: igraph.clone(), functions: ef.clone(), evaluatedParams: ht.clone(), modelName: p.clone() }
        },
        _ => {
            inCache.clone()
        },
    });
    outCache
}

pub fn isImplicitScope(mut inName: Name) -> Result<bool> {
    let mut isImplicit: bool = false;
    isImplicit = 'mc: {
        let __mc_input = inName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut id = __mc_input.clone() else { bail!("nomatch") };
            Ok(stringGet((id.clone()).clone(),1)? == 36)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isImplicit)
}

pub fn getCachedInstFunc(mut inCache: Cache, mut path: Arc<Absyn::Path>) -> Result<DAE::Function> {
    let mut func: DAE::Function;
    func = (match inCache.clone() {
        Cache::CACHE { functions: mut ef, .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(Mutable::access(ef.clone()), path.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            func = __pa0.clone();
            func.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(func)
}

pub fn checkCachedInstFuncGuard(mut inCache: Cache, mut path: Arc<Absyn::Path>) -> Result<()> {
    let () = (match inCache.clone() {
        Cache::CACHE { functions: mut ef, .. } => {
            AvlTreePathFunction::get(Mutable::access(ef.clone()), path.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn getFunctionTree(mut cache: Cache) -> Arc<AvlTreePathFunction::Tree> {
    let mut ft: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    ft = (match cache.clone() {
        Cache::CACHE { functions: mut ef, .. } => {
            Mutable::access(ef.clone())
        },
        _ => {
            Arc::new(openmodelica_frontend_dump::AvlTreePathFunction::Tree::EMPTY)
        },
    });
    ft
}

pub fn addCachedInstFuncGuard(mut cache: Cache, mut func: Arc<Absyn::Path>) -> Result<Cache> {
    let mut outCache: Cache = Cache::NO_CACHE;
    outCache = 'mc: {
        let __mc_input = (cache.clone(), func.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    checkCachedInstFuncGuard(cache.clone(), func.clone())?;
                    Ok(cache.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Cache::CACHE { functions: ef, .. }, Deref @ Absyn::Path::FULLYQUALIFIED { path: _ }) => {
                    Mutable::update(ef.clone(), AvlTreePathFunction::add(Mutable::access(ef.clone()), func.clone(), None, (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?);
                    Ok(cache.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(cache.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCache)
}

pub fn addDaeFunction(mut inCache: Cache, mut funcs: Arc<metamodelica::List<DAE::Function>>) -> Result<Cache> {
    let mut outCache: Cache = Cache::NO_CACHE;
    outCache = (match inCache.clone() {
        Cache::CACHE { initialGraph: _, functions: mut ef, evaluatedParams: _, modelName: _ } => {
            Mutable::update(ef.clone(), DAEUtil::addDaeFunction(funcs.clone(), Mutable::access(ef.clone()))?);
            inCache.clone()
        },
        _ => {
            inCache.clone()
        },
    });
    Ok(outCache)
}

pub fn addDaeExtFunction(mut inCache: Cache, mut funcs: Arc<metamodelica::List<DAE::Function>>) -> Result<Cache> {
    let mut outCache: Cache = Cache::NO_CACHE;
    outCache = (match inCache.clone() {
        Cache::CACHE { initialGraph: _, functions: mut ef, evaluatedParams: _, modelName: _ } => {
            Mutable::update(ef.clone(), DAEUtil::addDaeExtFunction(funcs.clone(), Mutable::access(ef.clone()))?);
            inCache.clone()
        },
        _ => {
            inCache.clone()
        },
    });
    Ok(outCache)
}

pub fn setCachedFunctionTree(mut inCache: Cache, mut inFunctions: Arc<AvlTreePathFunction::Tree>) -> () {
    let () = (match inCache.clone() {
        Cache::CACHE { .. } => {
            Mutable::update(var_field!(inCache.functions, Cache::CACHE).clone(), inFunctions.clone());
            ()
        },
        _ => (),
    });
    ()
}

pub fn isTyped(mut is: Status) -> bool {
    let mut b: bool = false;
    b = (match is.clone() {
        Status::VAR_UNTYPED { .. } => false,
        _ => true,
    });
    b
}

pub fn isDeletedComp(mut status: Status) -> bool {
    let mut isDeleted: bool = false;
    isDeleted = (match status.clone() {
        Status::VAR_DELETED { .. } => true,
        _ => false,
    });
    isDeleted
}

pub fn getCachedInitialGraph(mut cache: Cache) -> Result<Graph> {
    let mut g: Graph;
    g = (match cache.clone() {
        Cache::CACHE { initialGraph: Some(mut g), .. } => g.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(g)
}

pub fn setCachedInitialGraph(mut cache: Cache, mut g: Graph) -> Cache {
    let mut cache: Cache = cache;
    cache = (match cache.clone() {
        Cache::CACHE { .. } => {
            let __owned_variant_initialGraph_0 = Some(g.clone());
            if let Cache::CACHE { initialGraph, .. } = &mut cache {
                *initialGraph = __owned_variant_initialGraph_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Cache::CACHE"); }
            cache.clone()
        },
        _ => cache.clone(),
    });
    cache
}

pub fn getRecordConstructorName(mut inName: Name) -> Result<Name> {
    let mut outName: Name = arcstr::literal!("");
    outName = (if (Config::acceptMetaModelicaGrammar()?) {inName.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*arcstr::literal!(recordConstructorSuffix)); ArcStr::from(__mm_s) }}).clone();
    Ok(outName)
}

pub fn getRecordConstructorPath(mut inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut lastId: Name = arcstr::literal!("");
    if Config::acceptMetaModelicaGrammar()? {
        outPath = inPath.clone();
    } else {
        lastId = (AbsynUtil::pathLastIdent(inPath.clone())?).clone();
        lastId = (getRecordConstructorName((lastId.clone()).clone())?).clone();
        outPath = AbsynUtil::pathSetLastIdent(inPath.clone(), (lastId.clone()).clone())?;
    }
    Ok(outPath)
}

