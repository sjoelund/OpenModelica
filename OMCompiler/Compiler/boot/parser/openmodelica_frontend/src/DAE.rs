// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::AbsynUtil;
use crate::ClassInf;
use crate::SCode;
use crate::Values;
use openmodelica_ast::Absyn;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;

pub static ASSERTIONLEVEL_ERROR: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::ENUM_LITERAL { name: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("AssertionLevel")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("error")).clone() }) }), index: 2 }) });

pub static ASSERTIONLEVEL_WARNING: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::ENUM_LITERAL { name: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("AssertionLevel")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("warning")).clone() }) }), index: 1 }) });

#[derive(Clone, Debug, PartialEq)]
pub struct Algorithm {
    pub statementLst: Arc<metamodelica::List<Arc<Statement>>>,
}

pub type ALGORITHM_STMTS = Algorithm;


#[derive(Clone, Debug, PartialEq)]
pub struct Attributes {
    pub connectorType: Arc<ConnectorType>,
    pub parallelism: SCode::Parallelism,
    pub variability: SCode::Variability,
    pub direction: Absyn::Direction,
    pub innerOuter: Absyn::InnerOuter,
    pub visibility: SCode::Visibility,
}

pub type ATTR = Attributes;


pub mod AvlTreePathFunction {
    use super::*;
    pub type ConflictFunc = fn(Value, Value, Key) -> Result<Value>;

    pub type Key = Arc<Absyn::Path>;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Tree {
        NODE {
            key: Key,
            value: Value,
            height: i32,
            left: Arc<Tree>,
            right: Arc<Tree>,
        },
        LEAF {
            key: Key,
            value: Value,
        },
        EMPTY,
    }
    pub use self::Tree::{NODE,LEAF,EMPTY};

    pub type Value = Option<Function>;

    pub type ValueNode = Arc<Absyn::Path>;

    pub fn add(inTree: Arc<Tree>, inKey: Key, inValue: Value, conflictFunc: ConflictFunc) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ EMPTY => Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }),
        Deref @ NODE { key, .. } => {
            let mut value: Value;
            let mut key_comp: i32;
            let mut outTree: Arc<Tree>;
            key_comp = keyCompare(inKey.clone(), key.clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey.clone(), inValue.clone(), conflictFunc)?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey.clone(), inValue.clone(), conflictFunc)?);
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::NODE).clone(), key.clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::NODE).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::NODE; value = value.clone());
                }
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ LEAF { .. } => {
            let mut key: Key;
            let mut value: Value;
            let mut key_comp: i32;
            let mut outTree: Arc<Tree>;
            key_comp = keyCompare(inKey.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }), right: Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }) });
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

    pub use addConflictReplace as addConflictDefault;

    pub fn addConflictFail(newValue: Value, oldValue: Value, key: Key) -> Result<Value> {
        let mut value: Value;
        bail!("fail");
        Ok(value)
    }

    pub fn addConflictKeep(newValue: Value, oldValue: Value, key: Key) -> Value {
        let mut value: Value = oldValue.clone();
        value
    }

    pub fn addConflictReplace(newValue: Value, oldValue: Value, key: Key) -> Value {
        let mut value: Value = newValue.clone();
        value
    }

    pub fn addList(tree: Arc<Tree>, inValues: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<Function>)>>, conflictFunc: ConflictFunc) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key;
        let mut value: Value;
        for t in &*inValues.clone() {
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc)?;
        }
        Ok(tree)
    }

    pub fn addUpdate(tree: Arc<Tree>, key: Key, r#fn: Arc<dyn ::std::ops::Fn(Option<Option<Function>>) -> Result<Option<Function>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = fn(Option<Option<Function>>) -> Result<Value>;

        let mut tree: Arc<Tree> = tree;
        let mut key_comp: i32;
        let mut new_tree: Arc<Tree>;
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ EMPTY => Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }),
        Deref @ NODE { .. } => {
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
        Deref @ LEAF { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }), right: Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }) });
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

    fn balance(inTree: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ LEAF { .. } => inTree.clone(),
        Deref @ NODE { .. } => {
            let mut lh: i32;
            let mut rh: i32;
            let mut diff: i32;
            let mut child: Arc<Tree>;
            let mut balanced_tree: Arc<Tree>;
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

    fn calculateBalance(inNode: Arc<Tree>) -> i32 {
        let mut outBalance: i32;
        outBalance = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ NODE { .. } => height(var_field!((*inNode).left, Tree::NODE).clone()) - height(var_field!((*inNode).right, Tree::NODE).clone()),
        Deref @ LEAF { .. } => 0,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outBalance
    }

    pub fn fold<FT: Clone + 'static>(inTree: Arc<Tree>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<Function>, FT) -> Result<FT> + 'static>, inStartValue: FT) -> FT {
        pub type FoldFunc<FT: Clone> = fn(Key, Value, FT) -> Result<FT>;

        let mut outResult: FT = inStartValue.clone();
        outResult = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ NODE { value, key, .. } => {
            outResult = fold(var_field!((*inTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            outResult = fold(var_field!((*inTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult.clone()
        },
        Deref @ LEAF { value, key } => {
            outResult = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            outResult.clone()
        },
        _ => outResult.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outResult
    }

    pub fn foldCond<FT: Clone + 'static>(tree: Arc<Tree>, foldFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<Function>, FT) -> Result<(FT, bool)> + 'static>, value: FT) -> FT {
        pub type FoldFunc<FT: Clone> = fn(Key, Value, FT) -> Result<(FT, bool)>;

        let mut value: FT = value;
        value = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ NODE { .. } => {
            let mut c: bool;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), value.clone()).unwrap();
            if c.clone() {
                value = foldCond(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), value.clone());
                value = foldCond(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), value.clone());
            }
            value.clone()
        },
        Deref @ LEAF { .. } => {
            let mut c: bool;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), value.clone()).unwrap();
            value.clone()
        },
        _ => value.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value
    }

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(tree: Arc<Tree>, foldFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<Function>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, foldArg1: FT1, foldArg2: FT2) -> (FT1, FT2) {
        pub type FoldFunc<FT1: Clone, FT2: Clone> = fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ NODE { .. } => {
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            ()
        },
        Deref @ LEAF { .. } => {
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (foldArg1, foldArg2)
    }

    pub fn forEach(tree: Arc<Tree>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<Function>) -> Result<()> + 'static>) -> Result<()> {
        pub type EachFunc = fn(Key, Value) -> Result<()>;

        let _ = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ NODE { .. } => {
            forEach(var_field!((*tree).left, Tree::NODE).clone(), func.clone())?;
            func(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone())?;
            forEach(var_field!((*tree).right, Tree::NODE).clone(), func.clone())?;
            ()
        },
        Deref @ LEAF { .. } => {
            func(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone())?;
            ()
        },
        Deref @ EMPTY => (),
        _ => bail!("match: no arm matched"),
    } });
        Ok(())
    }

    pub fn fromList(inValues: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<Function>)>>, conflictFunc: ConflictFunc) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY);
        let mut key: Key;
        let mut value: Value;
        for t in &*inValues.clone() {
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc)?;
        }
        Ok(tree)
    }

    pub fn get(tree: Arc<Tree>, key: Key) -> Result<Value> {
        let mut value: Value;
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ LEAF { .. }) => var_field!((*tree).value, Tree::LEAF).clone(),
        (0, Deref @ NODE { .. }) => var_field!((*tree).value, Tree::NODE).clone(),
        (1, Deref @ NODE { .. }) => get(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ NODE { .. }) => get(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(value)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getOpt(tree: Arc<Tree>, key: Key) -> Result<Option<Option<Function>>> {
        let mut value: Option<Option<Function>>;
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => key.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ LEAF { .. }) => Some(var_field!((*tree).value, Tree::LEAF).clone()),
        (0, Deref @ NODE { .. }) => Some(var_field!((*tree).value, Tree::NODE).clone()),
        (1, Deref @ NODE { .. }) => getOpt(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ NODE { .. }) => getOpt(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(value)
    }

    pub fn hasKey(inTree: Arc<Tree>, inKey: Key) -> Result<bool> {
        let mut comp: bool = false;
        let mut key: Key;
        let mut key_comp: i32;
        let mut tree: Arc<Tree>;
        key = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ NODE { .. } => var_field!((*inTree).key, Tree::NODE).clone(),
        Deref @ LEAF { .. } => var_field!((*inTree).key, Tree::LEAF).clone(),
        Deref @ EMPTY => {
            return Ok(comp);
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
        key_comp = keyCompare(inKey.clone(), key.clone())?;
        comp = (::match_deref::match_deref! { match &((key_comp.clone(), inTree.clone())) {
        (0, _) => true,
        (1, Deref @ NODE { right: tree, .. }) => hasKey(tree.clone(), inKey.clone())?,
        ((-1), Deref @ NODE { left: tree, .. }) => hasKey(tree.clone(), inKey.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(comp)
    }

    fn height(inNode: Arc<Tree>) -> i32 {
        let mut outHeight: i32;
        outHeight = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ NODE { .. } => var_field!((*inNode).height, Tree::NODE).clone(),
        Deref @ LEAF { .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outHeight
    }

    pub fn intersection() -> Result<()> {
        bail!("fail");
        Ok(())
    }

    pub fn isEmpty(tree: Arc<Tree>) -> bool {
        let mut isEmpty: bool;
        isEmpty = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ EMPTY => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isEmpty
    }

    pub fn join(tree: Arc<Tree>, treeToJoin: Arc<Tree>, conflictFunc: ConflictFunc) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ EMPTY => tree.clone(),
        Deref @ NODE { .. } => {
            tree = add(tree.clone(), var_field!((*treeToJoin).key, Tree::NODE).clone(), var_field!((*treeToJoin).value, Tree::NODE).clone(), conflictFunc)?;
            tree = join(tree.clone(), var_field!((*treeToJoin).left, Tree::NODE).clone(), conflictFunc)?;
            tree = join(tree.clone(), var_field!((*treeToJoin).right, Tree::NODE).clone(), conflictFunc)?;
            tree.clone()
        },
        Deref @ LEAF { .. } => add(tree.clone(), var_field!((*treeToJoin).key, Tree::LEAF).clone(), var_field!((*treeToJoin).value, Tree::LEAF).clone(), conflictFunc)?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn keyCompare(inKey1: Key, inKey2: Key) -> Result<i32> {
        let mut outResult: i32;
        outResult = AbsynUtil::pathCompareNoQual(inKey1.clone(), inKey2.clone())?;
        Ok(outResult)
    }

    pub fn keyStr(inKey: Key) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = (AbsynUtil::pathString(inKey.clone(), (literal!(".")).clone(), true, false)?).clone();
        Ok(outString)
    }

    pub fn listKeys(tree: Arc<Tree>, lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ NODE { key, .. } => {
            lst = listKeys(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(key.clone(), lst.clone());
            lst = listKeys(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ LEAF { key, .. } => cons(key.clone(), lst.clone()),
        _ => lst.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn listKeysReverse(inTree: Arc<Tree>, lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
        lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ LEAF { .. } => cons(var_field!((*inTree).key, Tree::LEAF).clone(), lst.clone()),
        Deref @ NODE { .. } => {
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

    pub fn listValues(tree: Arc<Tree>, lst: Arc<metamodelica::List<Option<Function>>>) -> Arc<metamodelica::List<Option<Function>>> {
        let mut lst: Arc<metamodelica::List<Option<Function>>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ NODE { value, .. } => {
            lst = listValues(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(value.clone(), lst.clone());
            lst = listValues(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ LEAF { value, .. } => cons(value.clone(), lst.clone()),
        _ => lst.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn map(inTree: Arc<Tree>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<Function>) -> Result<Option<Function>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = fn(Key, Value) -> Result<Value>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ NODE { value, key, .. } => {
            let mut new_value: Value;
            let mut branch: Arc<Tree>;
            let mut new_left: Arc<Tree>;
            let mut new_right: Arc<Tree>;
            new_left = map(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone());
            new_value = inFunc(key.clone(), value.clone()).unwrap();
            new_right = map(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ LEAF { value, key } => {
            let mut new_value: Value;
            let mut branch: Arc<Tree>;
            let mut new_left: Arc<Tree>;
            let mut new_right: Arc<Tree>;
            new_value = inFunc(key.clone(), value.clone()).unwrap();
            if !(referenceEq(&value.clone(),&new_value.clone())) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value.clone());
            }
            outTree.clone()
        },
        _ => inTree.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outTree
    }

    pub fn mapFold<FT: Clone + 'static>(inTree: Arc<Tree>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<Function>, FT) -> Result<(Option<Function>, FT)> + 'static>, inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone> = fn(Key, Value, FT) -> Result<Value>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ NODE { value, key, .. } => {
            let mut new_value: Value;
            let mut branch: Arc<Tree>;
            let mut new_left: Arc<Tree>;
            let mut new_right: Arc<Tree>;
            (new_left, outResult) = mapFold(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            (new_right, outResult) = mapFold(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ LEAF { value, key } => {
            let mut new_value: Value;
            let mut branch: Arc<Tree>;
            let mut new_left: Arc<Tree>;
            let mut new_right: Arc<Tree>;
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            if !(referenceEq(&value.clone(),&new_value.clone())) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value.clone());
            }
            outTree.clone()
        },
        _ => inTree.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (outTree, outResult)
    }

    pub fn new() -> Arc<Tree> {
        let mut outTree: Arc<Tree> = Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY);
        outTree
    }

    pub fn printNodeStr(inNode: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(outString)
    }

    pub fn printTreeStr(inTree: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr;
        let mut left: Arc<Tree>;
        let mut right: Arc<Tree>;
        outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ EMPTY => literal!("EMPTY()"),
        Deref @ LEAF { .. } => printNodeStr(inTree.clone())?,
        Deref @ NODE { right, left, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(left.clone(), true, (literal!("")).clone())?); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\\n")); __mm_s.push_str(&*printTreeStr2(right.clone(), false, (literal!("")).clone())?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(outString)
    }

    fn printTreeStr2(inTree: Arc<Tree>, isLeft: bool, inIndent: ArcStr) -> Result<ArcStr> {
        let mut outString: ArcStr;
        let mut val_node: Option<Arc<Absyn::Path>>;
        let mut left: Option<Arc<Tree>>;
        let mut right: Option<Arc<Tree>>;
        let mut left_str: ArcStr;
        let mut right_str: ArcStr;
        outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).left, Tree::NODE).clone(), true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!("     ")} else {literal!(" │   ")}); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\\n")); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).right, Tree::NODE).clone(), false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" │   ")} else {literal!("     ")}); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) },
        Deref @ LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\\n")); ArcStr::from(__mm_s) },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(outString)
    }

    fn referenceEqOrEmpty(t1: Arc<Tree>, t2: Arc<Tree>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ EMPTY, Deref @ EMPTY) => true,
        _ => referenceEq(&t1.clone(),&t2.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    fn rotateLeft(inNode: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outNode: Arc<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ NODE { right: child @ Deref @ NODE { .. }, .. } => {
            let mut node: Arc<Tree>;
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), var_field!((**child).left, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), node.clone(), var_field!((**child).right, Tree::NODE).clone())?
        },
        Deref @ NODE { right: child @ Deref @ LEAF { .. }, .. } => {
            let mut node: Arc<Tree>;
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY))?
        },
        _ => inNode.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outNode)
    }

    fn rotateRight(inNode: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outNode: Arc<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ NODE { left: child @ Deref @ NODE { .. }, .. } => {
            let mut node: Arc<Tree>;
            node = setTreeLeftRight(outNode.clone(), var_field!((**child).right, Tree::NODE).clone(), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), var_field!((**child).left, Tree::NODE).clone(), node.clone())?
        },
        Deref @ NODE { left: child @ Deref @ LEAF { .. }, .. } => {
            let mut node: Arc<Tree>;
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::DAE::AvlTreePathFunction::Tree::EMPTY), node.clone())?
        },
        _ => inNode.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outNode)
    }

    pub fn setTreeLeftRight(orig: Arc<Tree>, left: Arc<Tree>, right: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut res: Arc<Tree>;
        res = (::match_deref::match_deref! { match &((orig.clone(), left.clone(), right.clone())) {
        (Deref @ NODE { .. }, Deref @ EMPTY, Deref @ EMPTY) => Arc::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
        (Deref @ LEAF { .. }, Deref @ EMPTY, Deref @ EMPTY) => orig.clone(),
        (Deref @ NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE).clone(), left.clone()) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE).clone(), right.clone())) {orig.clone()} else {Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() })},
        (Deref @ LEAF { .. }, _, _) => Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn smallestKey(tree: Arc<Tree>) -> Result<Key> {
        let mut key: Key;
        key = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ NODE { right: Deref @ EMPTY, .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ NODE { .. } => smallestKey(var_field!((*tree).right, Tree::NODE).clone())?,
        Deref @ LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(key)
    }

    pub fn toList(inTree: Arc<Tree>, lst: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<Function>)>>) -> Arc<metamodelica::List<(Arc<Absyn::Path>, Option<Function>)>> {
        let mut lst: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<Function>)>> = lst;
        lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ NODE { value, key, .. } => {
            lst = toList(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst = cons((key.clone(), value.clone()), lst.clone());
            lst = toList(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ LEAF { value, key } => cons((key.clone(), value.clone()), lst.clone()),
        _ => lst.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn update(tree: Arc<Tree>, key: Key, value: Value) -> Arc<Tree> {
        let mut outTree: Arc<Tree> = add(tree.clone(), key.clone(), value.clone(), fnptr!(addConflictReplace, Option<Function>, Option<Function>, Arc<Absyn::Path>)).unwrap();
        outTree
    }

    pub fn valueStr(inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = ((match inValue.clone() {
        Some(FUNCTION { path: mut path, .. }) => AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?,
        Some(RECORD_CONSTRUCTOR { path: mut path, .. }) => AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?,
        Some(RECORD_CONSTRUCTOR { path: mut path, .. }) => literal!("<SOME_FUNCTION>"),
        _ => literal!("<NO_FUNCTION>"),
    })).clone();
        Ok(outString)
    }

}

#[derive(Clone, Debug, PartialEq)]
pub enum Binding {
    UNBOUND,
    EQBOUND {
        exp: Arc<Exp>,
        evaluatedExp: Option<Arc<Values::Value>>,
        constant_: Const,
        source: BindingSource,
    },
    VALBOUND {
        valBound: Arc<Values::Value>,
        source: BindingSource,
    },
}
pub use self::Binding::{UNBOUND,EQBOUND,VALBOUND};

#[derive(Clone, Debug, PartialEq)]
pub enum BindingSource {
    BINDING_FROM_DEFAULT_VALUE,
    BINDING_FROM_START_VALUE,
    BINDING_FROM_RECORD_SUBMODS,
    BINDING_FROM_DERIVED_RECORD_DECL,
}
pub use self::BindingSource::{BINDING_FROM_DEFAULT_VALUE,BINDING_FROM_START_VALUE,BINDING_FROM_RECORD_SUBMODS,BINDING_FROM_DERIVED_RECORD_DECL};

#[derive(Clone, Debug, PartialEq)]
pub struct CallAttributes {
    pub ty: Arc<Type>,
    pub tuple_: bool,
    pub builtin: bool,
    pub isImpure: bool,
    pub isFunctionPointerCall: bool,
    pub inlineType: InlineType,
    pub tailCall: TailCall,
}

pub type CALL_ATTR = CallAttributes;


#[derive(Clone, Debug, PartialEq)]
pub struct ClassAttributes {
    pub objetiveE: Option<Arc<Exp>>,
    pub objectiveIntegrandE: Option<Arc<Exp>>,
    pub startTimeE: Option<Arc<Exp>>,
    pub finalTimeE: Option<Arc<Exp>>,
}

pub type OPTIMIZATION_ATTRS = ClassAttributes;


#[derive(Clone, Debug, PartialEq)]
pub struct ClassPrefix {
    pub variability: SCode::Variability,
}

pub type CLASSPRE = ClassPrefix;


#[derive(Clone, Debug, PartialEq)]
pub enum ClockKind {
    INFERRED_CLOCK,
    RATIONAL_CLOCK {
        intervalCounter: Arc<Exp>,
        resolution: Arc<Exp>,
    },
    REAL_CLOCK {
        interval: Arc<Exp>,
    },
    EVENT_CLOCK {
        condition: Arc<Exp>,
        startInterval: Arc<Exp>,
    },
    SOLVER_CLOCK {
        c: Arc<Exp>,
        solverMethod: Arc<Exp>,
    },
}
pub use self::ClockKind::{INFERRED_CLOCK,RATIONAL_CLOCK,REAL_CLOCK,EVENT_CLOCK,SOLVER_CLOCK};

#[derive(Clone, Debug, PartialEq)]
pub enum CodeType {
    C_EXPRESSION,
    C_EXPRESSION_OR_MODIFICATION,
    C_MODIFICATION,
    C_TYPENAME,
    C_VARIABLENAME,
    C_VARIABLENAMES,
}
pub use self::CodeType::{C_EXPRESSION,C_EXPRESSION_OR_MODIFICATION,C_MODIFICATION,C_TYPENAME,C_VARIABLENAME,C_VARIABLENAMES};

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentPrefix {
    PRE {
        prefix: ArcStr,
        dimensions: Arc<metamodelica::List<Arc<Dimension>>>,
        subscripts: Arc<metamodelica::List<Arc<Subscript>>>,
        next: Arc<ComponentPrefix>,
        ci_state: ClassInf::State,
        info: SourceInfo,
    },
    NOCOMPPRE,
}
pub use self::ComponentPrefix::{PRE,NOCOMPPRE};

pub type ComponentPrefixOpt = Option<Arc<ComponentPrefix>>;

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentRef {
    CREF_QUAL {
        ident: Ident,
        identType: Arc<Type>,
        subscriptLst: Arc<metamodelica::List<Arc<Subscript>>>,
        componentRef: Arc<ComponentRef>,
    },
    CREF_IDENT {
        ident: Ident,
        identType: Arc<Type>,
        subscriptLst: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    OPTIMICA_ATTR_INST_CREF {
        componentRef: Arc<ComponentRef>,
        instant: ArcStr,
    },
    WILD,
}
pub use self::ComponentRef::{CREF_QUAL,CREF_IDENT,OPTIMICA_ATTR_INST_CREF,WILD};

pub mod Connect {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct ConnectorElement {
        pub name: Arc<ComponentRef>,
        pub face: Face,
        pub ty: ConnectorType,
        pub source: Arc<ElementSource>,
        pub set: i32,
    }

    pub type CONNECTOR_ELEMENT = ConnectorElement;


    #[derive(Clone, Debug, PartialEq)]
    pub enum ConnectorType {
        EQU,
        FLOW,
        STREAM {
            associatedFlow: Option<Arc<ComponentRef>>,
        },
        NO_TYPE,
    }
    pub use self::ConnectorType::{EQU,FLOW,STREAM,NO_TYPE};

    #[derive(Clone, Debug, PartialEq)]
    pub enum Face {
        INSIDE,
        OUTSIDE,
        NO_FACE,
    }
    pub use self::Face::{INSIDE,OUTSIDE,NO_FACE};

    pub const NEW_SET: i32 = -1;

#[derive(Clone, Debug, PartialEq)]
    pub struct OuterConnect {
        pub scope: Prefix,
        pub cr1: Arc<ComponentRef>,
        pub io1: Absyn::InnerOuter,
        pub f1: Face,
        pub cr2: Arc<ComponentRef>,
        pub io2: Absyn::InnerOuter,
        pub f2: Face,
        pub source: Arc<ElementSource>,
    }

    pub type OUTERCONNECT = OuterConnect;


    #[derive(Clone, Debug, PartialEq)]
    pub enum Set {
        SET {
            ty: ConnectorType,
            elements: Arc<metamodelica::List<ConnectorElement>>,
        },
        SET_POINTER {
            index: i32,
        },
    }
    pub use self::Set::{SET,SET_POINTER};

    pub type SetConnection = (i32, i32);

    pub type SetTrie = Arc<SetTrieNode>;

    #[derive(Clone, Debug, PartialEq)]
    pub enum SetTrieNode {
        SET_TRIE_NODE {
            name: ArcStr,
            cref: Arc<ComponentRef>,
            nodes: Arc<metamodelica::List<Arc<SetTrieNode>>>,
            connectCount: i32,
        },
        SET_TRIE_LEAF {
            name: ArcStr,
            insideElement: Option<ConnectorElement>,
            outsideElement: Option<ConnectorElement>,
            flowAssociation: Option<Arc<ComponentRef>>,
            connectCount: i32,
        },
    }
    pub use self::SetTrieNode::{SET_TRIE_NODE,SET_TRIE_LEAF};

#[derive(Clone, Debug, PartialEq)]
    pub struct Sets {
        pub sets: SetTrie,
        pub setCount: i32,
        pub connections: Arc<metamodelica::List<(i32, i32)>>,
        pub outerConnects: Arc<metamodelica::List<OuterConnect>>,
    }

    pub type SETS = Sets;


    pub static emptySet: std::sync::LazyLock<Sets> = std::sync::LazyLock::new(|| { Sets { sets: Arc::new(SetTrieNode::SET_TRIE_NODE { name: (literal!("")).clone(), cref: Arc::new(crate::DAE::ComponentRef::WILD), nodes: metamodelica::nil(), connectCount: 0 }), setCount: 0, connections: metamodelica::nil(), outerConnects: metamodelica::nil() } });

}

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectorType {
    POTENTIAL,
    FLOW,
    STREAM {
        associatedFlow: Option<Arc<ComponentRef>>,
    },
    NON_CONNECTOR,
}
pub use self::ConnectorType::{POTENTIAL,FLOW,STREAM,NON_CONNECTOR};

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    C_CONST,
    C_PARAM,
    C_VAR,
    C_UNKNOWN,
}
pub use self::Const::{C_CONST,C_PARAM,C_VAR,C_UNKNOWN};

#[derive(Clone, Debug, PartialEq)]
pub enum Constraint {
    CONSTRAINT_EXPS {
        constraintLst: Arc<metamodelica::List<Arc<Exp>>>,
    },
    CONSTRAINT_DT {
        constraint: Arc<Exp>,
        localCon: bool,
    },
}
pub use self::Constraint::{CONSTRAINT_EXPS,CONSTRAINT_DT};

#[derive(Clone, Debug, PartialEq)]
pub struct DAElist {
    pub elementLst: Arc<metamodelica::List<Arc<Element>>>,
}

pub type DAE = DAElist;


#[derive(Clone, Debug, PartialEq)]
pub enum Dimension {
    DIM_INTEGER {
        integer: i32,
    },
    DIM_BOOLEAN,
    DIM_ENUM {
        enumTypeName: Arc<Absyn::Path>,
        literals: Arc<metamodelica::List<ArcStr>>,
        size: i32,
    },
    DIM_EXP {
        exp: Arc<Exp>,
    },
    DIM_UNKNOWN,
}
pub use self::Dimension::{DIM_INTEGER,DIM_BOOLEAN,DIM_ENUM,DIM_EXP,DIM_UNKNOWN};

#[derive(Clone, Debug, PartialEq)]
pub enum DimensionBinding {
    DIM_UNBOUND,
    DIM_BOUND {
        binding: Arc<Exp>,
        constrains: Dimensions,
    },
}
pub use self::DimensionBinding::{DIM_UNBOUND,DIM_BOUND};

pub type Dimensions = Arc<metamodelica::List<Arc<Dimension>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Distribution {
    pub name: Arc<Exp>,
    pub params: Arc<Exp>,
    pub paramNames: Arc<Exp>,
}

pub type DISTRIBUTION = Distribution;


#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    VAR {
        componentRef: Arc<ComponentRef>,
        kind: VarKind,
        direction: VarDirection,
        parallelism: VarParallelism,
        protection: VarVisibility,
        ty: Arc<Type>,
        binding: Option<Arc<Exp>>,
        dims: InstDims,
        connectorType: Arc<ConnectorType>,
        source: Arc<ElementSource>,
        variableAttributesOption: Option<Arc<VariableAttributes>>,
        comment: Option<Arc<SCode::Comment>>,
        innerOuter: Absyn::InnerOuter,
        encrypted: bool,
    },
    DEFINE {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIALDEFINE {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    EQUATION {
        exp: Arc<Exp>,
        scalar: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    EQUEQUATION {
        cr1: Arc<ComponentRef>,
        cr2: Arc<ComponentRef>,
        source: Arc<ElementSource>,
    },
    ARRAY_EQUATION {
        dimension: Dimensions,
        exp: Arc<Exp>,
        array: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_ARRAY_EQUATION {
        dimension: Dimensions,
        exp: Arc<Exp>,
        array: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    CONNECT_EQUATION {
        lhsElement: Arc<Element>,
        lhsFace: Connect::Face,
        rhsElement: Arc<Element>,
        rhsFace: Connect::Face,
        source: Arc<ElementSource>,
    },
    COMPLEX_EQUATION {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_COMPLEX_EQUATION {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    WHEN_EQUATION {
        condition: Arc<Exp>,
        equations: Arc<metamodelica::List<Arc<Element>>>,
        elsewhen_: Option<Arc<Element>>,
        source: Arc<ElementSource>,
    },
    INITIAL_FOR_EQUATION {
        type_: Arc<Type>,
        iterIsArray: bool,
        iter: Ident,
        index: i32,
        range: Arc<Exp>,
        equations: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
    },
    FOR_EQUATION {
        type_: Arc<Type>,
        iterIsArray: bool,
        iter: Ident,
        index: i32,
        range: Arc<Exp>,
        equations: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
    },
    IF_EQUATION {
        condition1: Arc<metamodelica::List<Arc<Exp>>>,
        equations2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Element>>>>>,
        equations3: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
    },
    INITIAL_IF_EQUATION {
        condition1: Arc<metamodelica::List<Arc<Exp>>>,
        equations2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Element>>>>>,
        equations3: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
    },
    INITIALEQUATION {
        exp1: Arc<Exp>,
        exp2: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    ALGORITHM {
        algorithm_: Arc<Algorithm>,
        source: Arc<ElementSource>,
    },
    INITIALALGORITHM {
        algorithm_: Arc<Algorithm>,
        source: Arc<ElementSource>,
    },
    COMP {
        ident: Ident,
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
        comment: Option<Arc<SCode::Comment>>,
    },
    EXTOBJECTCLASS {
        path: Arc<Absyn::Path>,
        source: Arc<ElementSource>,
    },
    ASSERT {
        condition: Arc<Exp>,
        message: Arc<Exp>,
        level: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_ASSERT {
        condition: Arc<Exp>,
        message: Arc<Exp>,
        level: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    TERMINATE {
        message: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_TERMINATE {
        message: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    REINIT {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    NORETCALL {
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_NORETCALL {
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    CONSTRAINT {
        constraints: Arc<Constraint>,
        source: Arc<ElementSource>,
    },
    CLASS_ATTRIBUTES {
        classAttrs: Arc<ClassAttributes>,
    },
    FLAT_SM {
        ident: Ident,
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
    },
    SM_COMP {
        componentRef: Arc<ComponentRef>,
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
    },
    COMMENT {
        cmt: Arc<SCode::Comment>,
    },
}
pub use self::Element::{VAR,DEFINE,INITIALDEFINE,EQUATION,EQUEQUATION,ARRAY_EQUATION,INITIAL_ARRAY_EQUATION,CONNECT_EQUATION,COMPLEX_EQUATION,INITIAL_COMPLEX_EQUATION,WHEN_EQUATION,INITIAL_FOR_EQUATION,FOR_EQUATION,IF_EQUATION,INITIAL_IF_EQUATION,INITIALEQUATION,ALGORITHM,INITIALALGORITHM,COMP,EXTOBJECTCLASS,ASSERT,INITIAL_ASSERT,TERMINATE,INITIAL_TERMINATE,REINIT,NORETCALL,INITIAL_NORETCALL,CONSTRAINT,CLASS_ATTRIBUTES,FLAT_SM,SM_COMP,COMMENT};

#[derive(Clone, Debug, PartialEq)]
pub struct ElementSource {
    pub info: SourceInfo,
    pub partOfLst: Arc<metamodelica::List<Absyn::Within>>,
    pub instance: Arc<ComponentPrefix>,
    pub connectEquationOptLst: Arc<metamodelica::List<(Arc<ComponentRef>, Arc<ComponentRef>)>>,
    pub typeLst: Arc<metamodelica::List<Arc<Absyn::Path>>>,
    pub operations: Arc<metamodelica::List<Arc<SymbolicOperation>>>,
    pub comment: Arc<metamodelica::List<Arc<SCode::Comment>>>,
}

pub type SOURCE = ElementSource;


#[derive(Clone, Debug, PartialEq)]
pub enum Else {
    NOELSE,
    ELSEIF {
        exp: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        else_: Arc<Else>,
    },
    ELSE {
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
    },
}
pub use self::Else::{NOELSE,ELSEIF,ELSE};

#[derive(Clone, Debug, PartialEq)]
pub enum EqMod {
    TYPED {
        modifierAsExp: Arc<Exp>,
        modifierAsValue: Option<Arc<Values::Value>>,
        properties: Properties,
        modifierAsAbsynExp: Arc<Absyn::Exp>,
        info: SourceInfo,
    },
    UNTYPED {
        exp: Arc<Absyn::Exp>,
    },
}
pub use self::EqMod::{TYPED,UNTYPED};

pub type EqualityConstraint = Option<(Arc<Absyn::Path>, i32, InlineType)>;

#[derive(Clone, Debug, PartialEq)]
pub enum EquationExp {
    PARTIAL_EQUATION {
        exp: Arc<Exp>,
    },
    RESIDUAL_EXP {
        exp: Arc<Exp>,
    },
    EQUALITY_EXPS {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
    },
}
pub use self::EquationExp::{PARTIAL_EQUATION,RESIDUAL_EXP,EQUALITY_EXPS};

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluateSingletonType {
    EVAL_SINGLETON_TYPE_FUNCTION {
        fun: EvaluateSingletonTypeFunction,
    },
    EVAL_SINGLETON_KNOWN_TYPE {
        ty: Arc<Type>,
    },
    NOT_SINGLETON,
}
pub use self::EvaluateSingletonType::{EVAL_SINGLETON_TYPE_FUNCTION,EVAL_SINGLETON_KNOWN_TYPE,NOT_SINGLETON};

pub type EvaluateSingletonTypeFunction = fn() -> Result<Arc<Type>>;

#[derive(Clone, Debug, PartialEq)]
pub enum Exp {
    ICONST {
        integer: i32,
    },
    RCONST {
        real: f64,
    },
    SCONST {
        string: ArcStr,
    },
    BCONST {
        bool: bool,
    },
    CLKCONST {
        clk: Arc<ClockKind>,
    },
    ENUM_LITERAL {
        name: Arc<Absyn::Path>,
        index: i32,
    },
    CREF {
        componentRef: Arc<ComponentRef>,
        ty: Arc<Type>,
    },
    BINARY {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
    },
    UNARY {
        operator: Operator,
        exp: Arc<Exp>,
    },
    LBINARY {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
    },
    LUNARY {
        operator: Operator,
        exp: Arc<Exp>,
    },
    RELATION {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
        index: i32,
        optionExpisASUB: Option<(Arc<Exp>, i32, i32)>,
    },
    IFEXP {
        expCond: Arc<Exp>,
        expThen: Arc<Exp>,
        expElse: Arc<Exp>,
    },
    CALL {
        path: Arc<Absyn::Path>,
        expLst: Arc<metamodelica::List<Arc<Exp>>>,
        attr: Arc<CallAttributes>,
    },
    RECORD {
        path: Arc<Absyn::Path>,
        exps: Arc<metamodelica::List<Arc<Exp>>>,
        comp: Arc<metamodelica::List<ArcStr>>,
        ty: Arc<Type>,
    },
    PARTEVALFUNCTION {
        path: Arc<Absyn::Path>,
        expList: Arc<metamodelica::List<Arc<Exp>>>,
        ty: Arc<Type>,
        origType: Arc<Type>,
    },
    ARRAY {
        ty: Arc<Type>,
        scalar: bool,
        array: Arc<metamodelica::List<Arc<Exp>>>,
    },
    MATRIX {
        ty: Arc<Type>,
        integer: i32,
        matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Exp>>>>>,
    },
    RANGE {
        ty: Arc<Type>,
        start: Arc<Exp>,
        step: Option<Arc<Exp>>,
        stop: Arc<Exp>,
    },
    TUPLE {
        PR: Arc<metamodelica::List<Arc<Exp>>>,
    },
    CAST {
        ty: Arc<Type>,
        exp: Arc<Exp>,
    },
    ASUB {
        exp: Arc<Exp>,
        sub: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    TSUB {
        exp: Arc<Exp>,
        ix: i32,
        ty: Arc<Type>,
    },
    RSUB {
        exp: Arc<Exp>,
        ix: i32,
        fieldName: ArcStr,
        ty: Arc<Type>,
    },
    SIZE {
        exp: Arc<Exp>,
        sz: Option<Arc<Exp>>,
    },
    CODE {
        code: Arc<Absyn::CodeNode>,
        ty: Arc<Type>,
    },
    EMPTY {
        scope: ArcStr,
        name: Arc<ComponentRef>,
        ty: Arc<Type>,
        tyStr: ArcStr,
    },
    REDUCTION {
        reductionInfo: Arc<ReductionInfo>,
        expr: Arc<Exp>,
        iterators: ReductionIterators,
    },
    LIST {
        valList: Arc<metamodelica::List<Arc<Exp>>>,
    },
    CONS {
        car: Arc<Exp>,
        cdr: Arc<Exp>,
    },
    META_TUPLE {
        listExp: Arc<metamodelica::List<Arc<Exp>>>,
    },
    META_OPTION {
        exp: Option<Arc<Exp>>,
    },
    METARECORDCALL {
        path: Arc<Absyn::Path>,
        args: Arc<metamodelica::List<Arc<Exp>>>,
        fieldNames: Arc<metamodelica::List<ArcStr>>,
        index: i32,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
    },
    MATCHEXPRESSION {
        matchType: MatchType,
        inputs: Arc<metamodelica::List<Arc<Exp>>>,
        aliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>,
        localDecls: Arc<metamodelica::List<Arc<Element>>>,
        cases: Arc<metamodelica::List<Arc<MatchCase>>>,
        et: Arc<Type>,
    },
    BOX {
        exp: Arc<Exp>,
    },
    UNBOX {
        exp: Arc<Exp>,
        ty: Arc<Type>,
    },
    SHARED_LITERAL {
        index: i32,
        exp: Arc<Exp>,
    },
    PATTERN {
        pattern: Arc<Pattern>,
    },
}
pub use self::Exp::{ICONST,RCONST,SCONST,BCONST,CLKCONST,ENUM_LITERAL,CREF,BINARY,UNARY,LBINARY,LUNARY,RELATION,IFEXP,CALL,RECORD,PARTEVALFUNCTION,ARRAY,MATRIX,RANGE,TUPLE,CAST,ASUB,TSUB,RSUB,SIZE,CODE,EMPTY,REDUCTION,LIST,CONS,META_TUPLE,META_OPTION,METARECORDCALL,MATCHEXPRESSION,BOX,UNBOX,SHARED_LITERAL,PATTERN};

#[derive(Clone, Debug, PartialEq)]
pub enum Expand {
    EXPAND,
    NOT_EXPAND,
}
pub use self::Expand::{EXPAND,NOT_EXPAND};

#[derive(Clone, Debug, PartialEq)]
pub enum ExtArg {
    EXTARG {
        componentRef: Arc<ComponentRef>,
        direction: Absyn::Direction,
        type_: Arc<Type>,
    },
    EXTARGEXP {
        exp: Arc<Exp>,
        type_: Arc<Type>,
    },
    EXTARGSIZE {
        componentRef: Arc<ComponentRef>,
        type_: Arc<Type>,
        exp: Arc<Exp>,
    },
    NOEXTARG,
}
pub use self::ExtArg::{EXTARG,EXTARGEXP,EXTARGSIZE,NOEXTARG};

#[derive(Clone, Debug, PartialEq)]
pub struct ExternalDecl {
    pub name: ArcStr,
    pub args: Arc<metamodelica::List<ExtArg>>,
    pub returnArg: ExtArg,
    pub language: ArcStr,
    pub ann: Option<Arc<SCode::Annotation>>,
}

pub type EXTERNALDECL = ExternalDecl;


pub static FUNCTION_ATTRIBUTES_BUILTIN: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::PURE.clone(), isFunctionPointer: false, isBuiltin: FunctionBuiltin::FUNCTION_BUILTIN { name: None, unboxArgs: false }, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_BUILTIN_IMPURE: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::IMPURE.clone(), isFunctionPointer: false, isBuiltin: FunctionBuiltin::FUNCTION_BUILTIN { name: None, unboxArgs: false }, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_DEFAULT: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::DEFAULT_INLINE, generateEvents: false, purity: Purity::PURE.clone(), isFunctionPointer: false, isBuiltin: crate::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_IMPURE: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::IMPURE.clone(), isFunctionPointer: false, isBuiltin: crate::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

#[derive(Clone, Debug, PartialEq)]
pub struct FuncArg {
    pub name: ArcStr,
    pub ty: Arc<Type>,
    pub r#const: Const,
    pub par: VarParallelism,
    pub defaultBinding: Option<Arc<Exp>>,
}

pub type FUNCARG = FuncArg;


#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    FUNCTION {
        path: Arc<Absyn::Path>,
        functions: Arc<metamodelica::List<FunctionDefinition>>,
        type_: Arc<Type>,
        visibility: SCode::Visibility,
        partialPrefix: bool,
        isImpure: bool,
        inlineType: InlineType,
        unusedInputs: Arc<metamodelica::List<i32>>,
        source: Arc<ElementSource>,
        comment: Option<Arc<SCode::Comment>>,
    },
    RECORD_CONSTRUCTOR {
        path: Arc<Absyn::Path>,
        type_: Arc<Type>,
        source: Arc<ElementSource>,
    },
}
pub use self::Function::{FUNCTION,RECORD_CONSTRUCTOR};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionAttributes {
    pub inline: InlineType,
    pub generateEvents: bool,
    pub purity: Purity,
    pub isFunctionPointer: bool,
    pub isBuiltin: FunctionBuiltin,
    pub functionParallelism: FunctionParallelism,
}

pub type FUNCTION_ATTRIBUTES = FunctionAttributes;


#[derive(Clone, Debug, PartialEq)]
pub enum FunctionBuiltin {
    FUNCTION_NOT_BUILTIN,
    FUNCTION_BUILTIN {
        name: Option<ArcStr>,
        unboxArgs: bool,
    },
    FUNCTION_BUILTIN_PTR,
}
pub use self::FunctionBuiltin::{FUNCTION_NOT_BUILTIN,FUNCTION_BUILTIN,FUNCTION_BUILTIN_PTR};

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionDefinition {
    FUNCTION_DEF {
        body: Arc<metamodelica::List<Arc<Element>>>,
    },
    FUNCTION_EXT {
        body: Arc<metamodelica::List<Arc<Element>>>,
        externalDecl: ExternalDecl,
    },
    FUNCTION_DER_MAPPER {
        derivedFunction: Arc<Absyn::Path>,
        derivativeFunction: Arc<Absyn::Path>,
        derivativeOrder: i32,
        conditionRefs: Arc<metamodelica::List<(i32, derivativeCond)>>,
        defaultDerivative: Option<Arc<Absyn::Path>>,
        lowerOrderDerivatives: Arc<metamodelica::List<Arc<Absyn::Path>>>,
    },
    FUNCTION_INVERSE {
        inputParam: Arc<ComponentRef>,
        inverseCall: Arc<Exp>,
    },
    FUNCTION_PARTIAL_DERIVATIVE {
        derivedFunction: Arc<Absyn::Path>,
        derivedVars: Arc<metamodelica::List<ArcStr>>,
    },
}
pub use self::FunctionDefinition::{FUNCTION_DEF,FUNCTION_EXT,FUNCTION_DER_MAPPER,FUNCTION_INVERSE,FUNCTION_PARTIAL_DERIVATIVE};

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionParallelism {
    FP_NON_PARALLEL,
    FP_PARALLEL_FUNCTION,
    FP_KERNEL_FUNCTION,
}
pub use self::FunctionParallelism::{FP_NON_PARALLEL,FP_PARALLEL_FUNCTION,FP_KERNEL_FUNCTION};

pub type FunctionTree = Arc<AvlTreePathFunction::Tree>;

pub type Ident = ArcStr;

#[derive(Clone, Debug, PartialEq)]
pub enum InlineType {
    NORM_INLINE,
    BUILTIN_EARLY_INLINE,
    EARLY_INLINE,
    DEFAULT_INLINE,
    NO_INLINE,
    AFTER_INDEX_RED_INLINE,
}
pub use self::InlineType::{NORM_INLINE,BUILTIN_EARLY_INLINE,EARLY_INLINE,DEFAULT_INLINE,NO_INLINE,AFTER_INDEX_RED_INLINE};

pub type InstDims = Arc<metamodelica::List<Arc<Dimension>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase {
    pub patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    pub patternGuard: Option<Arc<Exp>>,
    pub localDecls: Arc<metamodelica::List<Arc<Element>>>,
    pub body: Arc<metamodelica::List<Arc<Statement>>>,
    pub result: Option<Arc<Exp>>,
    pub resultInfo: SourceInfo,
    pub jump: i32,
    pub info: SourceInfo,
}

pub type CASE = MatchCase;


#[derive(Clone, Debug, PartialEq)]
pub enum MatchType {
    MATCHCONTINUE,
    TRY_STACKOVERFLOW,
    MATCH {
        switch: Option<(i32, Arc<Type>, i32)>,
    },
}
pub use self::MatchType::{MATCHCONTINUE,TRY_STACKOVERFLOW,MATCH};

#[derive(Clone, Debug, PartialEq)]
pub enum Mod {
    MOD {
        finalPrefix: SCode::Final,
        eachPrefix: SCode::Each,
        subModLst: Arc<metamodelica::List<Arc<SubMod>>>,
        binding: Option<EqMod>,
        info: SourceInfo,
    },
    REDECL {
        finalPrefix: SCode::Final,
        eachPrefix: SCode::Each,
        element: Arc<SCode::Element>,
        r#mod: Arc<Mod>,
    },
    NOMOD,
}
pub use self::Mod::{MOD,REDECL,NOMOD};

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    ADD {
        ty: Arc<Type>,
    },
    SUB {
        ty: Arc<Type>,
    },
    MUL {
        ty: Arc<Type>,
    },
    DIV {
        ty: Arc<Type>,
    },
    POW {
        ty: Arc<Type>,
    },
    UMINUS {
        ty: Arc<Type>,
    },
    UMINUS_ARR {
        ty: Arc<Type>,
    },
    ADD_ARR {
        ty: Arc<Type>,
    },
    SUB_ARR {
        ty: Arc<Type>,
    },
    MUL_ARR {
        ty: Arc<Type>,
    },
    DIV_ARR {
        ty: Arc<Type>,
    },
    MUL_ARRAY_SCALAR {
        ty: Arc<Type>,
    },
    ADD_ARRAY_SCALAR {
        ty: Arc<Type>,
    },
    SUB_SCALAR_ARRAY {
        ty: Arc<Type>,
    },
    MUL_SCALAR_PRODUCT {
        ty: Arc<Type>,
    },
    MUL_MATRIX_PRODUCT {
        ty: Arc<Type>,
    },
    DIV_ARRAY_SCALAR {
        ty: Arc<Type>,
    },
    DIV_SCALAR_ARRAY {
        ty: Arc<Type>,
    },
    POW_ARRAY_SCALAR {
        ty: Arc<Type>,
    },
    POW_SCALAR_ARRAY {
        ty: Arc<Type>,
    },
    POW_ARR {
        ty: Arc<Type>,
    },
    POW_ARR2 {
        ty: Arc<Type>,
    },
    AND {
        ty: Arc<Type>,
    },
    OR {
        ty: Arc<Type>,
    },
    NOT {
        ty: Arc<Type>,
    },
    LESS {
        ty: Arc<Type>,
    },
    LESSEQ {
        ty: Arc<Type>,
    },
    GREATER {
        ty: Arc<Type>,
    },
    GREATEREQ {
        ty: Arc<Type>,
    },
    EQUAL {
        ty: Arc<Type>,
    },
    NEQUAL {
        ty: Arc<Type>,
    },
    USERDEFINED {
        fqName: Arc<Absyn::Path>,
    },
}
pub use self::Operator::{ADD,SUB,MUL,DIV,POW,UMINUS,UMINUS_ARR,ADD_ARR,SUB_ARR,MUL_ARR,DIV_ARR,MUL_ARRAY_SCALAR,ADD_ARRAY_SCALAR,SUB_SCALAR_ARRAY,MUL_SCALAR_PRODUCT,MUL_MATRIX_PRODUCT,DIV_ARRAY_SCALAR,DIV_SCALAR_ARRAY,POW_ARRAY_SCALAR,POW_SCALAR_ARRAY,POW_ARR,POW_ARR2,AND,OR,NOT,LESS,LESSEQ,GREATER,GREATEREQ,EQUAL,NEQUAL,USERDEFINED};

pub static PI: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::RCONST { real: 3.1415926535897932384626433832795028841971693993751058 }) });

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    PAT_WILD,
    PAT_CONSTANT {
        ty: Option<Arc<Type>>,
        exp: Arc<Exp>,
    },
    PAT_AS {
        id: ArcStr,
        ty: Option<Arc<Type>>,
        attr: Arc<Attributes>,
        pat: Arc<Pattern>,
    },
    PAT_AS_FUNC_PTR {
        id: ArcStr,
        pat: Arc<Pattern>,
    },
    PAT_META_TUPLE {
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    },
    PAT_CALL_TUPLE {
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    },
    PAT_CONS {
        head: Arc<Pattern>,
        tail: Arc<Pattern>,
    },
    PAT_CALL {
        name: Arc<Absyn::Path>,
        index: i32,
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
        fields: Arc<metamodelica::List<Arc<Var>>>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        knownSingleton: bool,
    },
    PAT_CALL_NAMED {
        name: Arc<Absyn::Path>,
        patterns: Arc<metamodelica::List<(Arc<Pattern>, ArcStr, Arc<Type>)>>,
    },
    PAT_SOME {
        pat: Arc<Pattern>,
    },
}
pub use self::Pattern::{PAT_WILD,PAT_CONSTANT,PAT_AS,PAT_AS_FUNC_PTR,PAT_META_TUPLE,PAT_CALL_TUPLE,PAT_CONS,PAT_CALL,PAT_CALL_NAMED,PAT_SOME};

#[derive(Clone, Debug, PartialEq)]
pub enum Prefix {
    NOPRE,
    PREFIX {
        compPre: Arc<ComponentPrefix>,
        classPre: ClassPrefix,
    },
}
pub use self::Prefix::{NOPRE,PREFIX};

#[derive(Clone, Debug, PartialEq)]
pub enum Properties {
    PROP {
        type_: Arc<Type>,
        constFlag: Const,
    },
    PROP_TUPLE {
        type_: Arc<Type>,
        tupleConst: Arc<TupleConst>,
    },
}
pub use self::Properties::{PROP,PROP_TUPLE};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Purity {
    PURE = 1,
    IMPURE = 2,
    UNDEFINED = 3,
    OM_IMPURE = 4,
}
impl PartialOrd for Purity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Purity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReductionInfo {
    pub path: Arc<Absyn::Path>,
    pub iterType: Absyn::ReductionIterType,
    pub exprType: Arc<Type>,
    pub defaultValue: Option<Arc<Values::Value>>,
    pub foldName: ArcStr,
    pub resultName: ArcStr,
    pub foldExp: Option<Arc<Exp>>,
}

pub type REDUCTIONINFO = ReductionInfo;


#[derive(Clone, Debug, PartialEq)]
pub struct ReductionIterator {
    pub id: ArcStr,
    pub exp: Arc<Exp>,
    pub guardExp: Option<Arc<Exp>>,
    pub ty: Arc<Type>,
}

pub type REDUCTIONITER = ReductionIterator;


pub type ReductionIterators = Arc<metamodelica::List<Arc<ReductionIterator>>>;

pub type StartValue = Option<Arc<Exp>>;

#[derive(Clone, Debug, PartialEq)]
pub enum StateSelect {
    NEVER,
    AVOID,
    DEFAULT,
    PREFER,
    ALWAYS,
}
pub use self::StateSelect::{NEVER,AVOID,DEFAULT,PREFER,ALWAYS};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    STMT_ASSIGN {
        type_: Arc<Type>,
        exp1: Arc<Exp>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_TUPLE_ASSIGN {
        type_: Arc<Type>,
        expExpLst: Arc<metamodelica::List<Arc<Exp>>>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_ASSIGN_ARR {
        type_: Arc<Type>,
        lhs: Arc<Exp>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_IF {
        exp: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        else_: Arc<Else>,
        source: Arc<ElementSource>,
    },
    STMT_FOR {
        type_: Arc<Type>,
        iterIsArray: bool,
        iter: Ident,
        range: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        source: Arc<ElementSource>,
    },
    STMT_PARFOR {
        type_: Arc<Type>,
        iterIsArray: bool,
        iter: Ident,
        range: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        loopPrlVars: Arc<metamodelica::List<(Arc<ComponentRef>, SourceInfo)>>,
        source: Arc<ElementSource>,
    },
    STMT_WHILE {
        exp: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        source: Arc<ElementSource>,
    },
    STMT_WHEN {
        exp: Arc<Exp>,
        conditions: Arc<metamodelica::List<Arc<ComponentRef>>>,
        initialCall: bool,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        elseWhen: Option<Arc<Statement>>,
        source: Arc<ElementSource>,
    },
    STMT_ASSERT {
        cond: Arc<Exp>,
        msg: Arc<Exp>,
        level: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_TERMINATE {
        msg: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_REINIT {
        var: Arc<Exp>,
        value: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_NORETCALL {
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_RETURN {
        source: Arc<ElementSource>,
    },
    STMT_BREAK {
        source: Arc<ElementSource>,
    },
    STMT_CONTINUE {
        source: Arc<ElementSource>,
    },
    STMT_ARRAY_INIT {
        name: ArcStr,
        ty: Arc<Type>,
        source: Arc<ElementSource>,
    },
    STMT_FAILURE {
        body: Arc<metamodelica::List<Arc<Statement>>>,
        source: Arc<ElementSource>,
    },
}
pub use self::Statement::{STMT_ASSIGN,STMT_TUPLE_ASSIGN,STMT_ASSIGN_ARR,STMT_IF,STMT_FOR,STMT_PARFOR,STMT_WHILE,STMT_WHEN,STMT_ASSERT,STMT_TERMINATE,STMT_REINIT,STMT_NORETCALL,STMT_RETURN,STMT_BREAK,STMT_CONTINUE,STMT_ARRAY_INIT,STMT_FAILURE};

#[derive(Clone, Debug, PartialEq)]
pub struct SubMod {
    pub ident: Ident,
    pub r#mod: Arc<Mod>,
}

pub type NAMEMOD = SubMod;


#[derive(Clone, Debug, PartialEq)]
pub enum Subscript {
    WHOLEDIM,
    SLICE {
        exp: Arc<Exp>,
    },
    INDEX {
        exp: Arc<Exp>,
    },
    WHOLE_NONEXP {
        exp: Arc<Exp>,
    },
}
pub use self::Subscript::{WHOLEDIM,SLICE,INDEX,WHOLE_NONEXP};

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolicOperation {
    FLATTEN {
        scode: Arc<SCode::Equation>,
        dae: Option<Arc<Element>>,
    },
    SIMPLIFY {
        before: Arc<EquationExp>,
        after: Arc<EquationExp>,
    },
    SUBSTITUTION {
        substitutions: Arc<metamodelica::List<Arc<Exp>>>,
        source: Arc<Exp>,
    },
    OP_INLINE {
        before: Arc<EquationExp>,
        after: Arc<EquationExp>,
    },
    OP_SCALARIZE {
        before: Arc<EquationExp>,
        index: i32,
        after: Arc<EquationExp>,
    },
    OP_DIFFERENTIATE {
        cr: Arc<ComponentRef>,
        before: Arc<Exp>,
        after: Arc<Exp>,
    },
    SOLVE {
        cr: Arc<ComponentRef>,
        exp1: Arc<Exp>,
        exp2: Arc<Exp>,
        res: Arc<Exp>,
        assertConds: Arc<metamodelica::List<Arc<Exp>>>,
    },
    SOLVED {
        cr: Arc<ComponentRef>,
        exp: Arc<Exp>,
    },
    LINEAR_SOLVED {
        vars: Arc<metamodelica::List<Arc<ComponentRef>>>,
        jac: Arc<metamodelica::List<Arc<metamodelica::List<f64>>>>,
        rhs: Arc<metamodelica::List<f64>>,
        result: Arc<metamodelica::List<f64>>,
    },
    NEW_DUMMY_DER {
        chosen: Arc<ComponentRef>,
        candidates: Arc<metamodelica::List<Arc<ComponentRef>>>,
    },
    OP_RESIDUAL {
        e1: Arc<Exp>,
        e2: Arc<Exp>,
        e: Arc<Exp>,
    },
}
pub use self::SymbolicOperation::{FLATTEN,SIMPLIFY,SUBSTITUTION,OP_INLINE,OP_SCALARIZE,OP_DIFFERENTIATE,SOLVE,SOLVED,LINEAR_SOLVED,NEW_DUMMY_DER,OP_RESIDUAL};

pub static T_ANYTYPE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ANYTYPE { anyClassType: None }) });

pub static T_ARRAY_BOOL_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_BOOL_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_INT_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_INTEGER_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_REAL_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_REAL_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_STRING_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_STRING_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ASSERTIONLEVEL: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("AssertionLevel")).clone() }) }), names: list![(literal!("warning")).clone(), (literal!("error")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }) });

pub static T_BOOL_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_BOOL_DEFAULT.clone() }) });

pub static T_BOOL_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_BOOL { varLst: metamodelica::nil() }) });

pub static T_CLOCK_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_CLOCK { varLst: metamodelica::nil() }) });

pub static T_COMPLEX_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }) });

pub static T_COMPLEX_DEFAULT_RECORD: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }) });

pub static T_ENUMERATION_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }) });

pub static T_INTEGER_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_INTEGER_DEFAULT.clone() }) });

pub static T_INTEGER_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_INTEGER { varLst: metamodelica::nil() }) });

pub static T_METABOXED_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_METALIST_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METALIST { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_METATYPE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METATYPE { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_NONE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METAOPTION { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_NORETCALL_DEFAULT: Arc<Type> = crate::DAE::Type::T_NORETCALL;

pub static T_REAL_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_REAL_DEFAULT.clone() }) });

pub static T_REAL_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_REAL { varLst: metamodelica::nil() }) });

pub static T_SOURCEINFO_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METAUNIONTYPE { paths: list![Arc::new(Absyn::Path::QUALIFIED { name: (literal!("SourceInfo")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SOURCEINFO")).clone() }) })], typeVars: metamodelica::nil(), knownSingleton: true, singletonType: Arc::new(EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: T_SOURCEINFO_DEFAULT_METARECORD.clone() }), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SourceInfo")).clone() }) }) });

pub static T_SOURCEINFO_DEFAULT_METARECORD: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METARECORD { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("SourceInfo")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SOURCEINFO")).clone() }) }), utPath: Arc::new(Absyn::Path::IDENT { name: (literal!("SourceInfo")).clone() }), typeVars: metamodelica::nil(), index: 1, fields: list![Arc::new(Var { name: (literal!("fileName")).clone(), attributes: dummyAttrVar.clone(), ty: T_STRING_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("isReadOnly")).clone(), attributes: dummyAttrVar.clone(), ty: T_BOOL_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lineNumberStart")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("columnNumberStart")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lineNumberEnd")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("columnNumberEnd")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lastModification")).clone(), attributes: dummyAttrVar.clone(), ty: T_REAL_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], knownSingleton: true }) });

pub static T_STRING_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_STRING_DEFAULT.clone() }) });

pub static T_STRING_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_STRING { varLst: metamodelica::nil() }) });

pub static T_UNKNOWN_DEFAULT: Arc<Type> = crate::DAE::Type::T_UNKNOWN;

#[derive(Clone, Debug, PartialEq)]
pub enum TailCall {
    NO_TAIL,
    TAIL {
        vars: Arc<metamodelica::List<ArcStr>>,
        outVars: Arc<metamodelica::List<ArcStr>>,
    },
}
pub use self::TailCall::{NO_TAIL,TAIL};

#[derive(Clone, Debug, PartialEq)]
pub enum TupleConst {
    SINGLE_CONST {
        r#const: Const,
    },
    TUPLE_CONST {
        tupleConstLst: Arc<metamodelica::List<Arc<TupleConst>>>,
    },
}
pub use self::TupleConst::{SINGLE_CONST,TUPLE_CONST};

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    T_INTEGER {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_REAL {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_STRING {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_BOOL {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_CLOCK {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_ENUMERATION {
        index: Option<i32>,
        path: Arc<Absyn::Path>,
        names: Arc<metamodelica::List<ArcStr>>,
        literalVarLst: Arc<metamodelica::List<Arc<Var>>>,
        attributeLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_ARRAY {
        ty: Arc<Type>,
        dims: Dimensions,
    },
    T_NORETCALL,
    T_UNKNOWN,
    T_COMPLEX {
        complexClassType: ClassInf::State,
        varLst: Arc<metamodelica::List<Arc<Var>>>,
        equalityConstraint: EqualityConstraint,
        usedExternally: bool,
    },
    T_SUBTYPE_BASIC {
        complexClassType: ClassInf::State,
        varLst: Arc<metamodelica::List<Arc<Var>>>,
        complexType: Arc<Type>,
        equalityConstraint: EqualityConstraint,
    },
    T_FUNCTION {
        funcArg: Arc<metamodelica::List<Arc<FuncArg>>>,
        funcResultType: Arc<Type>,
        functionAttributes: FunctionAttributes,
        path: Arc<Absyn::Path>,
    },
    T_FUNCTION_REFERENCE_VAR {
        functionType: Arc<Type>,
    },
    T_FUNCTION_REFERENCE_FUNC {
        builtin: bool,
        functionType: Arc<Type>,
    },
    T_TUPLE {
        types: Arc<metamodelica::List<Arc<Type>>>,
        names: Option<Arc<metamodelica::List<ArcStr>>>,
    },
    T_CODE {
        ty: CodeType,
    },
    T_ANYTYPE {
        anyClassType: Option<ClassInf::State>,
    },
    T_METALIST {
        ty: Arc<Type>,
    },
    T_METATUPLE {
        types: Arc<metamodelica::List<Arc<Type>>>,
    },
    T_METAOPTION {
        ty: Arc<Type>,
    },
    T_METAUNIONTYPE {
        paths: Arc<metamodelica::List<Arc<Absyn::Path>>>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        knownSingleton: bool,
        singletonType: Arc<EvaluateSingletonType>,
        path: Arc<Absyn::Path>,
    },
    T_METARECORD {
        path: Arc<Absyn::Path>,
        utPath: Arc<Absyn::Path>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        index: i32,
        fields: Arc<metamodelica::List<Arc<Var>>>,
        knownSingleton: bool,
    },
    T_METAARRAY {
        ty: Arc<Type>,
    },
    T_METABOXED {
        ty: Arc<Type>,
    },
    T_METAPOLYMORPHIC {
        name: ArcStr,
    },
    T_METATYPE {
        ty: Arc<Type>,
    },
}
pub use self::Type::{T_INTEGER,T_REAL,T_STRING,T_BOOL,T_CLOCK,T_ENUMERATION,T_ARRAY,T_NORETCALL,T_UNKNOWN,T_COMPLEX,T_SUBTYPE_BASIC,T_FUNCTION,T_FUNCTION_REFERENCE_VAR,T_FUNCTION_REFERENCE_FUNC,T_TUPLE,T_CODE,T_ANYTYPE,T_METALIST,T_METATUPLE,T_METAOPTION,T_METAUNIONTYPE,T_METARECORD,T_METAARRAY,T_METABOXED,T_METAPOLYMORPHIC,T_METATYPE};

pub const UNIQUEIO: &'static str = "$unique$outer$";

#[derive(Clone, Debug, PartialEq)]
pub enum Uncertainty {
    GIVEN,
    SOUGHT,
    REFINE,
    PROPAGATE,
}
pub use self::Uncertainty::{GIVEN,SOUGHT,REFINE,PROPAGATE};

#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    pub name: Ident,
    pub attributes: Arc<Attributes>,
    pub ty: Arc<Type>,
    pub binding: Arc<Binding>,
    pub bind_from_outside: bool,
    pub constOfForIteratorRange: Option<Const>,
}

pub type TYPES_VAR = Var;


#[derive(Clone, Debug, PartialEq)]
pub enum VarDirection {
    INPUT,
    OUTPUT,
    BIDIR,
}
pub use self::VarDirection::{INPUT,OUTPUT,BIDIR};

#[derive(Clone, Debug, PartialEq)]
pub enum VarInnerOuter {
    INNER,
    OUTER,
    INNER_OUTER,
    NOT_INNER_OUTER,
}
pub use self::VarInnerOuter::{INNER,OUTER,INNER_OUTER,NOT_INNER_OUTER};

#[derive(Clone, Debug, PartialEq)]
pub enum VarKind {
    VARIABLE,
    DISCRETE,
    PARAM,
    CONST,
}
pub use self::VarKind::{VARIABLE,DISCRETE,PARAM,CONST};

#[derive(Clone, Debug, PartialEq)]
pub enum VarParallelism {
    PARGLOBAL,
    PARLOCAL,
    NON_PARALLEL,
}
pub use self::VarParallelism::{PARGLOBAL,PARLOCAL,NON_PARALLEL};

#[derive(Clone, Debug, PartialEq)]
pub enum VarVisibility {
    PUBLIC,
    PROTECTED,
}
pub use self::VarVisibility::{PUBLIC,PROTECTED};

#[derive(Clone, Debug, PartialEq)]
pub enum VariableAttributes {
    VAR_ATTR_REAL {
        quantity: Option<Arc<Exp>>,
        unit: Option<Arc<Exp>>,
        displayUnit: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        nominal: Option<Arc<Exp>>,
        stateSelectOption: Option<StateSelect>,
        uncertainOption: Option<Uncertainty>,
        distributionOption: Option<Arc<Distribution>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_INT {
        quantity: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        uncertainOption: Option<Uncertainty>,
        distributionOption: Option<Arc<Distribution>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_BOOL {
        quantity: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_CLOCK {
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
    },
    VAR_ATTR_STRING {
        quantity: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_ENUMERATION {
        quantity: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
}
pub use self::VariableAttributes::{VAR_ATTR_REAL,VAR_ATTR_INT,VAR_ATTR_BOOL,VAR_ATTR_CLOCK,VAR_ATTR_STRING,VAR_ATTR_ENUMERATION};

pub const auxNamePrefix: &'static str = "$AUX";

pub static callAttrBuiltinBool: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_BOOL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureBool: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_BOOL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureInteger: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_INTEGER_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureReal: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_REAL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureString: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_STRING_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinInteger: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_INTEGER_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinOther: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_UNKNOWN_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinReal: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_REAL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinString: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_STRING_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrOther: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_UNKNOWN_DEFAULT.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static crefTime: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("time")).clone(), identType: T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

pub static crefTimeState: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("$time")).clone(), identType: T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

#[derive(Clone, Debug, PartialEq)]
pub enum derivativeCond {
    ZERO_DERIVATIVE,
    NO_DERIVATIVE {
        binding: Arc<Exp>,
    },
}
pub use self::derivativeCond::{ZERO_DERIVATIVE,NO_DERIVATIVE};

pub const derivativeNamePrefix: &'static str = "$DER";

pub static dummyAttrConst: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrInput: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrParam: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrVar: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static emptyCref: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("")).clone(), identType: T_UNKNOWN_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

pub static emptyDae: std::sync::LazyLock<DAElist> = std::sync::LazyLock::new(|| { DAE(metamodelica::nil()).unwrap() });

pub static emptyElementSource: std::sync::LazyLock<Arc<ElementSource>> = std::sync::LazyLock::new(|| { Arc::new(ElementSource { info: AbsynUtil::dummyInfo.clone(), partOfLst: metamodelica::nil(), instance: Arc::new(crate::DAE::ComponentPrefix::NOCOMPPRE), connectEquationOptLst: metamodelica::nil(), typeLst: metamodelica::nil(), operations: metamodelica::nil(), comment: metamodelica::nil() }) });

pub static emptyVarAttrBool: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_BOOL { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrClock: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_CLOCK { isProtected: None, finalPrefix: None }) });

pub static emptyVarAttrEnum: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_ENUMERATION { quantity: None, min: None, max: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrInt: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_INT { quantity: None, min: None, max: None, start: None, fixed: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrReal: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrString: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_STRING { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub const partialDerivativeNamePrefix: &'static str = "$pDER";

pub const preNamePrefix: &'static str = "$PRE";

pub const previousNamePrefix: &'static str = "$CLKPRE";

pub const startNamePrefix: &'static str = "$START";

