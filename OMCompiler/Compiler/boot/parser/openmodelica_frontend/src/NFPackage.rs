// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree as ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::List;

pub type Constants = Arc<BaseAvlSet::Tree>;

pub mod ConstantsSetImpl {
    use super::*;
    pub type Key = Arc<ComponentRef::NFComponentRef>;

    pub enum Tree {
        NODE {
            key: Arc<ComponentRef::NFComponentRef>,
            height: i32,
            left: Arc<Tree>,
            right: Arc<Tree>,
        },
        LEAF {
            key: Arc<ComponentRef::NFComponentRef>,
        },
        EMPTY,
    }
    pub use Tree::*;

    pub type ValueNode = Arc<ComponentRef::NFComponentRef>;

    pub fn add(inTree: Arc<Tree>, inKey: Arc<ComponentRef::NFComponentRef>) -> Arc<Tree> {
        todo!()
    }

    pub fn addList(tree: Arc<Tree>, inValues: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> Arc<Tree> {
        todo!()
    }

    fn balance(inTree: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    fn calculateBalance(inNode: Arc<Tree>) -> i32 {
        todo!()
    }

    pub fn hasKey(inTree: Arc<Tree>, inKey: Arc<ComponentRef::NFComponentRef>) -> bool {
        todo!()
    }

    fn height(inNode: Arc<Tree>) -> i32 {
        todo!()
    }

    pub fn intersection(tree1: Arc<Tree>, tree2: Arc<Tree>) -> (Arc<Tree>, Arc<Tree>, Arc<Tree>) {
        todo!()
    }

    pub fn isEmpty(tree: Arc<Tree>) -> bool {
        todo!()
    }

    pub fn join(tree: Arc<Tree>, treeToJoin: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    pub fn keyCompare(inKey1: Arc<ComponentRef::NFComponentRef>, inKey2: Arc<ComponentRef::NFComponentRef>) -> i32 {
        todo!()
    }

    pub fn keyStr(inKey: Arc<ComponentRef::NFComponentRef>) -> String {
        todo!()
    }

    pub fn listKeys(inTree: Arc<Tree>, lst: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
        todo!()
    }

    pub fn listKeysReverse(inTree: Arc<Tree>, lst: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
        todo!()
    }

    pub fn new() -> Arc<Tree> {
        todo!()
    }

    pub fn printNodeStr(inNode: Arc<Tree>) -> String {
        todo!()
    }

    pub fn printTreeStr(inTree: Arc<Tree>) -> String {
        todo!()
    }

    fn printTreeStr2(inTree: Arc<Tree>, isLeft: bool, inIndent: String) -> String {
        todo!()
    }

    fn referenceEqOrEmpty(t1: Arc<Tree>, t2: Arc<Tree>) -> bool {
        todo!()
    }

    fn rotateLeft(inNode: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    fn rotateRight(inNode: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    pub fn setTreeLeftRight(orig: Arc<Tree>, left: Arc<Tree>, right: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    pub fn smallestKey(tree: Arc<Tree>) -> Arc<ComponentRef::NFComponentRef> {
        todo!()
    }

}

pub fn collectBindingConstants(binding: Arc<Binding::NFBinding>, constants: Arc<BaseAvlSet::Tree>) -> Arc<BaseAvlSet::Tree> {
    todo!()
}

pub fn collectConstants(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn collectExpConstants(exp: Arc<Expression::NFExpression>, constants: Arc<BaseAvlSet::Tree>) -> Arc<BaseAvlSet::Tree> {
    todo!()
}

pub fn collectExpConstants_traverser(exp: Arc<Expression::NFExpression>, constants: Arc<BaseAvlSet::Tree>) -> Arc<BaseAvlSet::Tree> {
    todo!()
}

pub fn collectFuncConstants(name: Arc<Absyn::Path>, func: Arc<Function::Function>, constants: Arc<BaseAvlSet::Tree>) -> Arc<BaseAvlSet::Tree> {
    todo!()
}

pub fn collectVariableConstants(var: Arc<Variable::NFVariable>, constants: Arc<BaseAvlSet::Tree>) -> Arc<BaseAvlSet::Tree> {
    todo!()
}

pub fn getPackageConstantBinding(cref: Arc<ComponentRef::NFComponentRef>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn getPackageConstantBinding2(fieldNode: Arc<InstNode::InstNode>, cref: Arc<ComponentRef::NFComponentRef>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn replaceBindingConstants(binding: Arc<Binding::NFBinding>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn replaceConstants(flatModel: Arc<FlatModel::NFFlatModel>, functions: Arc<BaseAvlTree::Tree>) -> (Arc<FlatModel::NFFlatModel>, Arc<BaseAvlTree::Tree>) {
    todo!()
}

pub fn replaceExpConstants(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn replaceExpConstants_traverser(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn replaceFuncConstants(name: Arc<Absyn::Path>, func: Arc<Function::Function>) -> Arc<Function::Function> {
    todo!()
}

pub fn replaceVariableConstants(var: Arc<Variable::NFVariable>) -> Arc<Variable::NFVariable> {
    todo!()
}

