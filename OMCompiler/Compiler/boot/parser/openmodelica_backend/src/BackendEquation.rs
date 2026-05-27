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

use crate::AvlSetInt;
use crate::BackendDAE;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendVariable;
use crate::ExpressionSolve;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashTable;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn emptyEqns() -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = emptyEqnsSized(0);
    equationArray
}

pub fn emptyEqnsSized(mut size: i32) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ExpandableArray::new(size.clone(), Arc::new(crate::BackendDAE::Equation::DUMMY_EQUATION));
    outEquationArray
}

pub fn add(mut inEquation: Arc<BackendDAE::Equation>, mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    ExpandableArray::add(inEquation.clone(), equationArray.clone())?;
    Ok(equationArray)
}

pub fn addList(mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    ExpandableArray::expandToSize(ExpandableArray::getLastUsedIndex(equationArray.clone()) + (eqnlst.clone().len() as i32), equationArray.clone())?;
    for mut e in &*eqnlst.clone() {
        let mut e = e.clone();
        equationArray = add(e.clone(), equationArray.clone())?;
    }
    Ok(equationArray)
}

pub fn delete(mut inPos: i32, mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    ExpandableArray::delete(inPos.clone(), equationArray.clone())?;
    Ok(equationArray)
}

pub fn deleteList(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inIndices: Arc<metamodelica::List<i32>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    for mut index in &*inIndices.clone() {
        let mut index = index.clone();
        ExpandableArray::delete(index.clone(), equationArray.clone())?;
    }
    Ok(equationArray)
}

pub fn merge(mut inEqns1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inEqns2: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    outEqns = copyEquationArray(inEqns2.clone());
    outEqns = addList(equationList(inEqns1.clone()), outEqns.clone())?;
    Ok(outEqns)
}

pub fn listEquation(mut inEquationList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    outEquationArray = ExpandableArray::new((inEquationList.clone().len() as i32), Arc::new(crate::BackendDAE::Equation::DUMMY_EQUATION));
    for mut eq in &*inEquationList.clone() {
        let mut eq = eq.clone();
        ExpandableArray::add(eq.clone(), outEquationArray.clone())?;
    }
    Ok(outEquationArray)
}

pub fn equationList(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = ExpandableArray::toList(equationArray.clone()).unwrap();
    outEquationLst
}

pub fn copyEquationArray(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ExpandableArray::copy(inEquationArray.clone(), Arc::new(crate::BackendDAE::Equation::DUMMY_EQUATION));
    outEquationArray
}

pub fn setAtIndex(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inPos: i32, mut inEquation: Arc<BackendDAE::Equation>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    if ExpandableArray::occupied(inPos.clone(), equationArray.clone()) {
        ExpandableArray::update(inPos.clone(), inEquation.clone(), equationArray.clone())?;
    } else {
        ExpandableArray::set(inPos.clone(), inEquation.clone(), equationArray.clone())?;
    }
    Ok(equationArray)
}

pub fn setAtIndexFirst(mut inPos: i32, mut inEquation: Arc<BackendDAE::Equation>, mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = setAtIndex(inEquationArray.clone(), inPos.clone(), inEquation.clone()).unwrap();
    outEquationArray
}

pub fn get(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inPos: i32) -> Arc<BackendDAE::Equation> {
    let mut outEquation: Arc<BackendDAE::Equation> = ExpandableArray::get(inPos.clone(), inEquationArray.clone()).unwrap();
    outEquation
}

pub fn has(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inPos: i32) -> bool {
    let mut b: bool = ExpandableArray::occupied(inPos.clone(), inEquationArray.clone());
    b
}

pub fn getList(mut inIndices: Arc<metamodelica::List<i32>>, mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEqns = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut index in (inIndices.clone()).into_iter().cloned() {
            let __x = get(inEquationArray.clone(), index.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outEqns
}

pub fn equationArraySize(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut outSize: i32 = 0;
    let mut nfScalarize: bool = Flags::isSet(Flags::NF_SCALARIZE.clone())?;
    outSize = 0;
    for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
        if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
            if nfScalarize.clone() {
                outSize = outSize.clone() + equationSize(ExpandableArray::get(i.clone(), equationArray.clone())?)?;
            } else {
                outSize = outSize.clone() + 1;
            }
        }
    }
    Ok(outSize)
}

pub fn getNumberOfEquations(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> i32 {
    let mut outSize: i32 = ExpandableArray::getNumberOfElements(inEquationArray.clone());
    outSize
}

pub fn traverseEquationArray<T: Clone + 'static>(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, T)> + 'static>, mut extraArg: T) -> Result<T> {
    pub type Func<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, T)> + 'static>;

    let mut extraArg: T = extraArg;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
        if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
            eqn = ExpandableArray::get(i.clone(), equationArray.clone())?;
            (_, extraArg) = inFunc(eqn.clone(), extraArg.clone())?;
        }
    }
    Ok(extraArg)
}

pub fn traverseEquationArray_WithStop<T: Clone + 'static>(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inFuncWithStop: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, bool, T)> + 'static>, mut extraArg: T) -> Result<T> {
    pub type FuncWithStop<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, bool, T)> + 'static>;

    let mut extraArg: T = extraArg;
    let mut continue_: bool = false;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
        if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
            eqn = ExpandableArray::get(i.clone(), equationArray.clone())?;
            (_, continue_, extraArg) = inFuncWithStop(eqn.clone(), extraArg.clone())?;
            if !(continue_.clone()) {
                break;
            }
        }
    }
    Ok(extraArg)
}

pub fn traverseEquationArray_WithUpdate<T: Clone + 'static>(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inFuncWithUpdate: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, T)> + 'static>, mut extraArg: T) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, T)> {
    pub type FuncWithUpdate<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, T)> + 'static>;

    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    let mut extraArg: T = extraArg;
    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut new_e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
        if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
            e = ExpandableArray::get(i.clone(), equationArray.clone())?;
            (new_e, extraArg) = inFuncWithUpdate(e.clone(), extraArg.clone())?;
            if !(referenceEq(&e.clone(),&new_e.clone())) {
                ExpandableArray::update(i.clone(), new_e.clone(), equationArray.clone())?;
            }
        }
    }
    Ok((equationArray, extraArg))
}

pub fn sortInitialEqns(mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = eqns;
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut init_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut sim_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    eqn_lst = equationList(eqns.clone());
    (init_eqns, sim_eqns) = List::splitOnTrue(eqn_lst.clone(), (std::sync::Arc::new(isInitialEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>));
    eqn_lst = listAppend(init_eqns.clone(), sim_eqns.clone());
    eqns = listEquation(eqn_lst.clone())?;
    Ok(eqns)
}

pub fn getForEquationIterIdent(mut inEquation: Arc<BackendDAE::Equation>) -> Option<ArcStr> {
    let mut forIter: Option<ArcStr> = None;
    forIter = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::FOR_EQUATION { iter: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: iter, .. }, .. }, .. } => {
            Some((iter.clone()).clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    forIter
}

pub fn getWhenEquationExpr(mut inWhenEquation: Arc<BackendDAE::WhenEquation>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outExp: Arc<DAE::Exp>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inWhenEquation.clone()) {
            Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: __pa1, left: Deref @ DAE::Exp::CREF { componentRef: __pa2, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        outExp = __pa1.clone();
        outComponentRef = __pa2.clone();
        Ok::<_, anyhow::Error>((outComponentRef.clone(), outExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outComponentRef = __try0_o0;
            outExp = __try0_o1;
        }
        Err(_) => {
            Error::addInternalError((literal!("BackendEquation.getWhenEquationExpr failed\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((outComponentRef, outExp))
}

pub fn setWhenElsePart(mut inWhenEquation: Arc<BackendDAE::WhenEquation>, mut inElseWhenEquation: Arc<BackendDAE::WhenEquation>) -> Result<Arc<BackendDAE::WhenEquation>> {
    let mut outWhenEquation: Arc<BackendDAE::WhenEquation>;
    outWhenEquation = (::match_deref::match_deref! { match &(inWhenEquation.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: None, whenStmtLst, condition: cond } => {
            Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: Some(inElseWhenEquation.clone()) })
        },
        Deref @ BackendDAE::WhenEquation { elsewhenPart: Some(elsewhenPart), whenStmtLst, condition: cond } => {
            Arc::new(BackendDAE::WhenEquation { elsewhenPart: Some(setWhenElsePart(elsewhenPart.clone(), inElseWhenEquation.clone())?), whenStmtLst: whenStmtLst.clone(), condition: cond.clone() })
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outWhenEquation)
}

pub fn equationsLstVars(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut indexes: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    let mut keys: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if inEquationLst.clone().is_empty() {
        outVars = metamodelica::nil();
        return Ok(outVars);
    }
    (_, indexes) = traverseExpsOfEquationList(inEquationLst.clone(), Arc::new({ let __pe_b2 = inVars.clone(); move |__pe_a0, __pe_a1| checkEquationsVarsExpTopDownTraverseHelper(__pe_a0, __pe_a1, __pe_b2.clone()) }), AvlSetInt::new())?;
    keys = AvlSetInt::listKeys(indexes.clone(), metamodelica::nil());
    outVars = List::map1r(keys.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVars.clone());
    Ok(outVars)
}

pub fn equationsVars(mut inEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut indexes: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    let mut keys: Arc<metamodelica::List<i32>> = metamodelica::nil();
    indexes = BackendDAEUtil::traverseBackendDAEExpsEqns(inEquations.clone(), Arc::new({ let __pe_b2 = inVars.clone(); move |__pe_a0, __pe_a1| checkEquationsVarsExpTopDownTraverseHelper(__pe_a0, __pe_a1, __pe_b2.clone()) }), AvlSetInt::new())?;
    keys = AvlSetInt::listKeys(indexes.clone(), metamodelica::nil());
    outVars = List::map1r(keys.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVars.clone());
    Ok(outVars)
}

pub fn equationVars(mut inEquation: Arc<BackendDAE::Equation>, mut inVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut indexes: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    let mut keys: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (_, indexes) = traverseExpsOfEquation(inEquation.clone(), Arc::new({ let __pe_b2 = inVars.clone(); move |__pe_a0, __pe_a1| checkEquationsVarsExpTopDownTraverseHelper(__pe_a0, __pe_a1, __pe_b2.clone()) }), AvlSetInt::new())?;
    keys = AvlSetInt::listKeys(indexes.clone(), metamodelica::nil());
    outVars = List::map1r(keys.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVars.clone());
    Ok(outVars)
}

pub fn expressionVars(mut inExp: Arc<DAE::Exp>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut indexes: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    let mut keys: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (_, indexes) = Expression::traverseExpTopDown(inExp.clone(), Arc::new({ let __pe_b2 = vars.clone(); move |__pe_a0, __pe_a1| Ok(checkEquationsVarsExpTopDown(__pe_a0, __pe_a1, __pe_b2.clone())) }), AvlSetInt::new())?;
    keys = AvlSetInt::listKeys(indexes.clone(), metamodelica::nil());
    outVars = List::map1r(keys.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
    Ok(outVars)
}

pub fn expressionVarsIndexes(mut exp: Arc<DAE::Exp>, mut indexes: Arc<AvlSetInt::Tree>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>) -> Result<Arc<AvlSetInt::Tree>> {
    pub type CheckEquationsVarsExpTopDownFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>;

    let mut indexes: Arc<AvlSetInt::Tree> = indexes;
    (_, indexes) = Expression::traverseExpTopDown(exp.clone(), func.clone(), indexes.clone())?;
    Ok(indexes)
}

pub fn checkEquationsVarsExpTopDownTraverseHelper(mut exp: Arc<DAE::Exp>, mut tree: Arc<AvlSetInt::Tree>, mut vars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut tree: Arc<AvlSetInt::Tree> = tree;
    (exp, tree) = Expression::traverseExpTopDown(exp.clone(), Arc::new({ let __pe_b2 = vars.clone(); move |__pe_a0, __pe_a1| Ok(checkEquationsVarsExpTopDown(__pe_a0, __pe_a1, __pe_b2.clone())) }), tree.clone())?;
    Ok((exp, tree))
}

pub fn checkEquationsVarsExpTopDown(mut exp: Arc<DAE::Exp>, mut tree: Arc<AvlSetInt::Tree>, mut vars: BackendDAE::Variables) -> (Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>) {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cont: bool = false;
    let mut tree: Arc<AvlSetInt::Tree> = tree;
    (cont, tree) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. } => {
            (true, tree.clone())
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. } => {
            (true, tree.clone())
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, .. } if (idn.clone() == literal!("pre") || idn.clone() == literal!("previous")) => {
            (false, tree.clone())
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            match '__try0: {
                (_, ilst) = unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars.clone()), '__try0);
                tree = unwrap_break_err!(AvlSetInt::addList(tree.clone(), ilst.clone()), '__try0);
                Ok::<_, anyhow::Error>((ilst.clone(), tree.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    ilst = __try0_o0;
                    tree = __try0_o1;
                }
                Err(_) => {
                    panic!("try/else: outputs not set in else branch");
                }
            }
            (true, tree.clone())
        },
        _ => {
            (true, tree.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, cont, tree)
}

pub fn assertWithCondTrue(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { cond: Deref @ DAE::Exp::BCONST { bool: true }, .. }, tail: Deref @ metamodelica::List::Nil } }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn equationsParams(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>)> {
    let mut outParamVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outParamVarsIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (_, (_, (__pa0, __pa1, _))) = traverseExpsOfEquationList(inEquationLst.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traversingParamRefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables))> + 'static>), (metamodelica::nil(), metamodelica::nil(), inVars.clone())))?;
    outParamVars = __pa0.clone();
    outParamVarsIdc = __pa1.clone();
    Ok((outParamVars, outParamVarsIdc))
}

fn traversingParamRefFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables);
    outExp = inExp.clone();
    outTpl = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
            inTpl.clone()
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, varIdc, allVars)) => {
            let mut foundVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut foundVarsIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vars = (*vars).clone();
            let mut varIdc = (*varIdc).clone();
            (foundVars, foundVarsIdc) = BackendVariable::getVar(cr.clone(), allVars.clone())?;
            (vars, varIdc) = traversingParamRefFinder0(foundVars.clone(), foundVarsIdc.clone(), vars.clone(), varIdc.clone())?;
            (vars.clone(), varIdc.clone(), allVars.clone())
        },
        _ => {
            inTpl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn traversingParamRefFinder0(mut iVars: Arc<metamodelica::List<BackendDAE::Var>>, mut iVarIdc: Arc<metamodelica::List<i32>>, mut iParamVarsList: Arc<metamodelica::List<BackendDAE::Var>>, mut iParamVarsIdc: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>)> {
    let mut oParamVarsList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oParamVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var: BackendDAE::Var;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = iParamVarsList.clone();
    let mut varIdc: Arc<metamodelica::List<i32>> = iParamVarsIdc.clone();
    let mut varIdx: i32 = 0;
    let mut rest: Arc<metamodelica::List<i32>> = iVarIdc.clone();
    for mut var in &*iVars.clone() {
        let mut var = var.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        varIdx = __pa0.clone();
        rest = __pa1.clone();
        if BackendVariable::isParam(var.clone()) {
            vars = List::unionEltOnTrue(var.clone(), vars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::varEqual, BackendDAE::Var, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Var) -> Result<bool> + 'static>));
            varIdc = List::unionEltOnTrue(varIdx.clone(), varIdc.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        }
    }
    oParamVarsList = vars.clone();
    oParamVarIdc = varIdc.clone();
    Ok((oParamVarsList, oParamVarIdc))
}

pub fn iterationVarsinRelations(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables) -> Result<(bool, Arc<metamodelica::List<i32>>)> {
    let mut mixedSystem: bool = false;
    let mut indexes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (_, (_, (__pa0, _))) = traverseExpsOfEquationList(inEquationLst.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traversingRelationsforIterationVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables))> + 'static>), (metamodelica::nil(), inVars.clone())))?;
    indexes = __pa0.clone();
    mixedSystem = !(indexes.clone().is_empty());
    Ok((mixedSystem, indexes))
}

fn traversingRelationsforIterationVars(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<i32>>, BackendDAE::Variables);
    outExp = inExp.clone();
    outTpl = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::RELATION { index, exp2: e2, exp1: e1, .. }, (indexes, vars)) => {
            let mut vlst1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut vlst2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            vlst1 = expressionVars(e1.clone(), vars.clone())?;
            vlst2 = expressionVars(e2.clone(), vars.clone())?;
            if !(vlst1.clone().is_empty() && vlst2.clone().is_empty()) {
                outTpl = (cons(index.clone(), indexes.clone()), vars.clone());
            } else {
                outTpl = inTpl.clone();
            }
            outTpl.clone()
        },
        _ => {
            inTpl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

pub fn equationsCrefs(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let (_, (_, __pa0)) = traverseExpsOfEquationList(inEquationLst.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinder, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil()))?;
    outExpComponentRefLst = __pa0.clone();
    Ok(outExpComponentRefLst)
}

pub fn equationCrefs(mut inEquation: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let (_, (_, __pa0)) = traverseExpsOfEquation(inEquation.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinder, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil()))?;
    outExpComponentRefLst = __pa0.clone();
    Ok(outExpComponentRefLst)
}

pub fn equationCrefsSolved(mut inEquation: Arc<BackendDAE::Equation>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut lhs_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut rhs_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    lhs = getEquationLHS(inEquation.clone())?;
    rhs = getEquationRHS(inEquation.clone())?;
    lhs_lst = Expression::extractCrefsFromExp(lhs.clone())?;
    rhs_lst = Expression::extractCrefsFromExp(rhs.clone())?;
    Ok((lhs_lst, rhs_lst))
}

pub fn getAllCrefFromEquations(mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    cr_lst = traverseEquationArray(inEqns.clone(), (std::sync::Arc::new(traversingEquationCrefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
    Ok(cr_lst)
}

fn traversingEquationCrefFinder(mut inEq: Arc<BackendDAE::Equation>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    e = inEq.clone();
    cr_lst = inCrefs.clone();
    let (_, (_, __pa0)) = traverseExpsOfEquation(e.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinder, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), cr_lst.clone()))?;
    cr_lst = __pa0.clone();
    Ok((e, cr_lst))
}

pub fn getCrefsFromEquations(mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inKnVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut vars: BackendDAE::Variables;
    let mut knownVars: BackendDAE::Variables;
    ht = HashTable::emptyHashTable();
    (_, _, ht) = traverseEquationArray(inEqns.clone(), (std::sync::Arc::new(findUnknownCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))))> + 'static>), (inVars.clone(), inKnVars.clone(), ht.clone()))?;
    cr_lst = BaseHashTable::hashTableKeyList(ht.clone());
    Ok(cr_lst)
}

pub fn findUnknownCrefs(mut inEq: Arc<BackendDAE::Equation>, mut extraArgs: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))))> {
    let mut inEq: Arc<BackendDAE::Equation> = inEq;
    let mut extraArgs: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) = extraArgs;
    let (_, (_, __pa0)) = traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(checkEquationsUnknownCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))))> + 'static>), extraArgs.clone()))?;
    extraArgs = __pa0.clone();
    Ok((inEq, extraArgs))
}

pub fn equationUnknownCrefs(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inKnVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    ht = HashTable::emptyHashTable();
    let (_, (_, (_, _, __pa0))) = traverseExpsOfEquationList(inEquationLst.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(checkEquationsUnknownCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))))> + 'static>), (inVars.clone(), inKnVars.clone(), ht.clone())))?;
    ht = __pa0.clone();
    cr_lst = BaseHashTable::hashTableKeyList(ht.clone());
    Ok(cr_lst)
}

fn checkEquationsUnknownCrefsExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)));
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst, .. }, componentRef: cr }, _) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)));
                    expl = List::map1(varLst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone());
                    (_, outTuple) = Expression::traverseExpList(expl.clone(), (std::sync::Arc::new(checkEquationsUnknownCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))))> + 'static>), inTuple.clone())?;
                    Ok((e.clone(), outTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)));
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (_, outTuple) = Expression::traverseExpBottomUp(e1.clone(), (std::sync::Arc::new(checkEquationsUnknownCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))))> + 'static>), inTuple.clone())?;
                    Ok((e.clone(), outTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, _, ht)) => {
                    if !((BaseHashTable::hasKey(cr.clone(), ht.clone()))) { bail!("guard") }
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _, _)) => {
                    (_, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, knvars, _)) => {
                    (_, _) = BackendVariable::getVar(cr.clone(), knvars.clone())?;
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, knvars, ht)) => {
                    let mut ht = (*ht).clone();
                    (ht, _, _, _) = BaseHashTable::add((cr.clone(), 0), ht.clone())?;
                    Ok((inExp.clone(), (vars.clone(), knvars.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

pub fn traverseExpsOfEquationList<ArgT: Clone + 'static>(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outArg: ArgT = inArg.clone();
    for mut eq in &*inEquations.clone() {
        let mut eq = eq.clone();
        (eq, outArg) = traverseExpsOfEquation(eq.clone(), func.clone(), outArg.clone())?;
        outEquations = cons(eq.clone(), outEquations.clone());
    }
    outEquations = metamodelica::Dangerous::listReverseInPlace(outEquations.clone());
    Ok((outEquations, outArg))
}

pub fn traverseExpsOfEquationList_WithStop<Type_a: Clone + 'static>(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(bool, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outBoolean: bool = true;
    let mut outTypeA: Type_a = inTypeA.clone();
    for mut eqn in &*inEquations.clone() {
        let mut eqn = eqn.clone();
        (outBoolean, outTypeA) = traverseExpsOfEquation_WithStop(eqn.clone(), inFunc.clone(), outTypeA.clone())?;
        if !(outBoolean.clone()) {
            break;
        }
    }
    Ok((outBoolean, outTypeA))
}

pub fn traverseExpsOfEquationList_WithoutChange<ArgT: Clone + 'static>(mut inEquation: Arc<BackendDAE::Equation>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<ArgT> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outArg: ArgT = inArg.clone();
    (_, outArg) = traverseExpsOfEquation(inEquation.clone(), func.clone(), outArg.clone())?;
    Ok(outArg)
}

fn traverseExpsOfEquationListList_WithStop<T: Clone + 'static>(mut inEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inTypeA: T) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outBoolean: bool = true;
    let mut outTypeA: T = inTypeA.clone();
    for mut eqn in &*inEquations.clone() {
        let mut eqn = eqn.clone();
        (outBoolean, outTypeA) = traverseExpsOfEquationList_WithStop(eqn.clone(), func.clone(), outTypeA.clone())?;
        if !(outBoolean.clone()) {
            break;
        }
    }
    Ok((outBoolean, outTypeA))
}

pub fn traverseExpsOfEquation<T: Clone + 'static>(mut inEquation: Arc<BackendDAE::Equation>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inTypeA: T) -> Result<(Arc<BackendDAE::Equation>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTypeA: T;
    (outEquation, outTypeA) = (::match_deref::match_deref! { match &(inEquation.clone()) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (e1, extArg) = inFunc(var_field!((*eqn).exp, BackendDAE::Equation::EQUATION).clone(), inTypeA.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; exp = e1.clone());
            (e1, extArg) = inFunc(var_field!((*eqn).scalar, BackendDAE::Equation::EQUATION).clone(), extArg.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; scalar = e1.clone());
            (eqn.clone(), extArg.clone())
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (e1, extArg) = inFunc(var_field!((*eqn).left, BackendDAE::Equation::ARRAY_EQUATION).clone(), inTypeA.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; left = e1.clone());
            (e1, extArg) = inFunc(var_field!((*eqn).right, BackendDAE::Equation::ARRAY_EQUATION).clone(), extArg.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; right = e1.clone());
            (eqn.clone(), extArg.clone())
        },
        eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            e1 = Expression::makeCrefExp(cr.clone(), Expression::r#typeof(var_field!((*eqn).exp, BackendDAE::Equation::SOLVED_EQUATION).clone())?)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inFunc(e1.clone(), inTypeA.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr1 = __pa0.clone();
            extArg = __pa1.clone();
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; componentRef = cr1.clone());
            (e1, extArg) = inFunc(var_field!((*eqn).exp, BackendDAE::Equation::SOLVED_EQUATION).clone(), extArg.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; exp = e1.clone());
            (eqn.clone(), extArg.clone())
        },
        eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (e1, extArg) = inFunc(var_field!((*eqn).exp, BackendDAE::Equation::RESIDUAL_EQUATION).clone(), inTypeA.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::RESIDUAL_EQUATION; exp = e1.clone());
            (eqn.clone(), extArg.clone())
        },
        eqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            let mut whenEquation: Arc<BackendDAE::WhenEquation>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (whenEquation, extArg) = traverseExpsOfWhenEquation(var_field!((*eqn).whenEquation, BackendDAE::Equation::WHEN_EQUATION).clone(), inFunc.clone(), inTypeA.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::WHEN_EQUATION; whenEquation = whenEquation.clone());
            (eqn.clone(), extArg.clone())
        },
        eqn @ Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            let mut stmts = (*stmts).clone();
            (stmts, extArg) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), inFunc.clone(), inTypeA.clone());
            assign_variant_field!(eqn => BackendDAE::Equation::ALGORITHM; alg = Arc::new(DAE::Algorithm { statementLst: stmts.clone() }));
            (eqn.clone(), extArg.clone())
        },
        eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (e1, extArg) = inFunc(var_field!((*eqn).left, BackendDAE::Equation::COMPLEX_EQUATION).clone(), inTypeA.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::COMPLEX_EQUATION; left = e1.clone());
            (e1, extArg) = inFunc(var_field!((*eqn).right, BackendDAE::Equation::COMPLEX_EQUATION).clone(), extArg.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::COMPLEX_EQUATION; right = e1.clone());
            (eqn.clone(), extArg.clone())
        },
        eqn @ Deref @ BackendDAE::Equation::IF_EQUATION { .. } => {
            let mut extArg: T;
            let mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut eqnstrue: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
            let mut eqnsfalse: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqn = (*eqn).clone();
            (conditions, extArg) = traverseExpsOfExpList(var_field!((*eqn).conditions, BackendDAE::Equation::IF_EQUATION).clone(), inFunc.clone(), inTypeA.clone());
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION; conditions = conditions.clone());
            (eqnstrue, extArg) = List::map1Fold(var_field!((*eqn).eqnstrue, BackendDAE::Equation::IF_EQUATION).clone(), (std::sync::Arc::new(traverseExpsOfEquationList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, _, _) -> Result<_> + 'static>), inFunc.clone(), extArg.clone());
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION; eqnstrue = eqnstrue.clone());
            (eqnsfalse, extArg) = List::map1Fold(var_field!((*eqn).eqnsfalse, BackendDAE::Equation::IF_EQUATION).clone(), (std::sync::Arc::new(traverseExpsOfEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), extArg.clone());
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION; eqnsfalse = eqnsfalse.clone());
            (eqn.clone(), extArg.clone())
        },
        eqn @ Deref @ BackendDAE::Equation::FOR_EQUATION { .. } => {
            let mut eqn1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (eqn1, extArg) = traverseExpsOfEquation(var_field!((*eqn).body, BackendDAE::Equation::FOR_EQUATION).clone(), inFunc.clone(), inTypeA.clone())?;
            assign_variant_field!(eqn => BackendDAE::Equation::FOR_EQUATION; body = eqn1.clone());
            (eqn.clone(), extArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outEquation, outTypeA))
}

pub fn traverseExpsOfEquation_WithStop<T: Clone + 'static>(mut inEquation: Arc<BackendDAE::Equation>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inTypeA: T) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outBoolean: bool = false;
    let mut outTypeA: T;
    (outBoolean, outTypeA) = (::match_deref::match_deref! { match &((inEquation.clone(), func.clone(), inTypeA.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, _, _) => {
            let mut ext_arg: T;
            let mut b: bool = false;
            (_, b, ext_arg) = func(e1.clone(), inTypeA.clone())?;
            if b.clone() {
                (_, b, ext_arg) = func(e2.clone(), ext_arg.clone())?;
            }
            (b.clone(), ext_arg.clone())
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }, _, _) => {
            let mut ext_arg: T;
            let mut b: bool = false;
            (_, b, ext_arg) = func(e1.clone(), inTypeA.clone())?;
            if b.clone() {
                (_, b, ext_arg) = func(e2.clone(), ext_arg.clone())?;
            }
            (b.clone(), ext_arg.clone())
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. }, _, _) => {
            let mut e1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ext_arg: T;
            let mut b: bool = false;
            tp = Expression::r#typeof(e2.clone())?;
            e1 = Expression::makeCrefExp(cr.clone(), tp.clone())?;
            (_, b, ext_arg) = func(e1.clone(), inTypeA.clone())?;
            if b.clone() {
                (_, b, ext_arg) = func(e2.clone(), ext_arg.clone())?;
            }
            (b.clone(), ext_arg.clone())
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, .. }, _, _) => {
            let mut ext_arg: T;
            let mut b: bool = false;
            (_, b, ext_arg) = func(e1.clone(), inTypeA.clone())?;
            (b.clone(), ext_arg.clone())
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: we, .. }, _, _) => {
            let mut ext_arg: T;
            let mut b: bool = false;
            (b, ext_arg) = traverseExpsOfWhenEquation_WithStop(we.clone(), func.clone(), inTypeA.clone())?;
            (b.clone(), ext_arg.clone())
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { .. }, .. }, _, _) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("not implemented error - BackendDAE.ALGORITHM - BackendEquation.traverseExpsOfEquation_WithStop\n")).clone())?;
            bail!("fail")
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. }, _, _) => {
            let mut ext_arg: T;
            let mut b: bool = false;
            (_, b, ext_arg) = func(e1.clone(), inTypeA.clone())?;
            if b.clone() {
                (_, b, ext_arg) = func(e2.clone(), ext_arg.clone())?;
            }
            (b.clone(), ext_arg.clone())
        },
        (Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: eqns, eqnstrue: eqnslst, conditions: expl, .. }, _, _) => {
            let mut ext_arg: T;
            let mut b: bool = false;
            (b, ext_arg) = traverseExpsOfExpList_WithStop(expl.clone(), func.clone(), inTypeA.clone());
            if b.clone() {
                (b, ext_arg) = traverseExpsOfEquationListList_WithStop(eqnslst.clone(), func.clone(), ext_arg.clone())?;
            }
            if b.clone() {
                (b, ext_arg) = traverseExpsOfEquationList_WithStop(eqns.clone(), func.clone(), ext_arg.clone())?;
            }
            (b.clone(), ext_arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outBoolean, outTypeA))
}

fn traverseExpsOfWhenEquation<T: Clone + 'static>(mut inWhenEquation: Arc<BackendDAE::WhenEquation>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inTypeA: T) -> Result<(Arc<BackendDAE::WhenEquation>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outWhenEquation: Arc<BackendDAE::WhenEquation>;
    let mut outTypeA: T;
    (outWhenEquation, outTypeA) = (::match_deref::match_deref! { match &(inWhenEquation.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: oelsewe, whenStmtLst, condition: cond } => {
            let mut elsewe: Arc<BackendDAE::WhenEquation>;
            let mut extArg: T;
            let mut oelsewe = (*oelsewe).clone();
            let mut whenStmtLst = (*whenStmtLst).clone();
            let mut cond = (*cond).clone();
            (cond, extArg) = inFunc(cond.clone(), inTypeA.clone())?;
            (whenStmtLst, extArg) = traverseExpsOfWhenOps(whenStmtLst.clone(), inFunc.clone(), extArg.clone(), metamodelica::nil())?;
            if isSome(oelsewe.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oelsewe.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                elsewe = __pa0.clone();
                (elsewe, extArg) = traverseExpsOfWhenEquation(elsewe.clone(), inFunc.clone(), extArg.clone())?;
                oelsewe = Some(elsewe.clone());
            } else {
                oelsewe = None;
            }
            (Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: oelsewe.clone() }), extArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outWhenEquation, outTypeA))
}

fn traverseExpsOfWhenOps<T: Clone + 'static>(mut inWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inTypeA: T, mut inAccum: Arc<metamodelica::List<BackendDAE::WhenOperator>>) -> Result<(Arc<metamodelica::List<BackendDAE::WhenOperator>>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut outTypeA: T;
    (outWhenOps, outTypeA) = (::match_deref::match_deref! { match &(inWhenOps.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inAccum.clone().reverse(), inTypeA.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { source, right: e2, left: e1 }, tail: rest } => {
            let mut extArg: T;
            let mut e2 = (*e2).clone();
            let mut e1 = (*e1).clone();
            let _ = Expression::r#typeof(e2.clone())?;
            (e1, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            (e2, extArg) = inFunc(e2.clone(), extArg.clone())?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg.clone(), cons(BackendDAE::WhenOperator::ASSIGN { left: e1.clone(), right: e2.clone(), source: source.clone() }, inAccum.clone()))?;
            (outWhenOps.clone(), extArg.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { source, value: e2, stateVar: cr }, tail: rest } => {
            let mut e1: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut extArg: T;
            let mut e2 = (*e2).clone();
            e1 = Expression::crefExp(cr.clone())?;
            (e1, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            if Expression::isCref(e1.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(e1.clone()) {
                    Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                cr1 = __pa0.clone();
            } else {
                cr1 = cr.clone();
            }
            (e2, extArg) = inFunc(e2.clone(), extArg.clone())?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg.clone(), cons(BackendDAE::WhenOperator::REINIT { stateVar: cr1.clone(), value: e2.clone(), source: source.clone() }, inAccum.clone()))?;
            (outWhenOps.clone(), extArg.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSERT { source, level, message: e2, condition: e1 }, tail: rest } => {
            let mut extArg: T;
            let mut e2 = (*e2).clone();
            let mut e1 = (*e1).clone();
            (e1, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            (e2, extArg) = inFunc(e2.clone(), extArg.clone())?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg.clone(), cons(BackendDAE::WhenOperator::ASSERT { condition: e1.clone(), message: e2.clone(), level: level.clone(), source: source.clone() }, inAccum.clone()))?;
            (outWhenOps.clone(), extArg.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::TERMINATE { source, message: e1 }, tail: rest } => {
            let mut extArg: T;
            let mut e1 = (*e1).clone();
            (e1, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg.clone(), cons(BackendDAE::WhenOperator::TERMINATE { message: e1.clone(), source: source.clone() }, inAccum.clone()))?;
            (outWhenOps.clone(), extArg.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { source, exp: e1 }, tail: rest } => {
            let mut extArg: T;
            let mut e1 = (*e1).clone();
            (e1, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg.clone(), cons(BackendDAE::WhenOperator::NORETCALL { exp: e1.clone(), source: source.clone() }, inAccum.clone()))?;
            (outWhenOps.clone(), extArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outWhenOps, outTypeA))
}

fn traverseExpsOfWhenEquation_WithStop<T: Clone + 'static>(mut inWhenEquation: Arc<BackendDAE::WhenEquation>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inTypeA: T) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outCont: bool = false;
    let mut outTypeA: T;
    (outCont, outTypeA) = (::match_deref::match_deref! { match &(inWhenEquation.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: oelsewe, whenStmtLst, condition: cond } => {
            let mut elsewe: Arc<BackendDAE::WhenEquation>;
            let mut extArg: T;
            let mut b: bool = false;
            (_, b, extArg) = inFunc(cond.clone(), inTypeA.clone())?;
            if b.clone() {
                (b, extArg) = traverseExpsOfWhenOps_WithStop(whenStmtLst.clone(), inFunc.clone(), extArg.clone(), b.clone())?;
            }
            if b.clone() {
                if isSome(oelsewe.clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(oelsewe.clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    elsewe = __pa0.clone();
                    (b, extArg) = traverseExpsOfWhenEquation_WithStop(elsewe.clone(), inFunc.clone(), extArg.clone())?;
                }
            }
            (b.clone(), extArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCont, outTypeA))
}

pub fn statementEq(mut iStmts: Arc<DAE::Statement>) -> Result<Arc<BackendDAE::Equation>> {
    let mut oEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    oEq = (::match_deref::match_deref! { match &(iStmts.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { exp, exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            generateEquation(Expression::crefExp(cr.clone())?, exp.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp, lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            generateEquation(Expression::crefExp(cr.clone())?, exp.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp, expExpLst: explst, .. } => {
            generateEquation(Expression::makeTuple(explst.clone())?, exp.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oEq)
}

fn traverseExpsOfWhenOps_WithStop<T: Clone + 'static>(mut inWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inTypeA: T, mut inCont: bool) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outCont: bool = false;
    let mut outTypeA: T;
    (outCont, outTypeA) = ({
        let mut b: bool = false;
        (::match_deref::match_deref! { match &(inWhenOps.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inCont.clone(), inTypeA.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e2, left: e1, .. }, tail: rest } => {
            let mut extArg: T;
            let _ = Expression::r#typeof(e2.clone())?;
            if inCont.clone() {
                (_, b, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            }
            if b.clone() {
                (_, b, extArg) = inFunc(e2.clone(), extArg.clone())?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg.clone(), b.clone())?;
            (b.clone(), extArg.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { value: e2, stateVar: cr, .. }, tail: rest } => {
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            tp = Expression::r#typeof(e2.clone())?;
            e1 = Expression::makeCrefExp(cr.clone(), tp.clone())?;
            if inCont.clone() {
                (_, b, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            }
            if b.clone() {
                (_, b, extArg) = inFunc(e2.clone(), extArg.clone())?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg.clone(), b.clone())?;
            (b.clone(), extArg.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSERT { message: e2, condition: e1, .. }, tail: rest } => {
            let mut extArg: T;
            if inCont.clone() {
                (_, b, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            }
            if b.clone() {
                (_, b, extArg) = inFunc(e2.clone(), extArg.clone())?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg.clone(), b.clone())?;
            (b.clone(), extArg.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::TERMINATE { message: e1, .. }, tail: rest } => {
            let mut extArg: T;
            if inCont.clone() {
                (_, b, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg.clone(), b.clone())?;
            (b.clone(), extArg.clone())
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: e1, .. }, tail: rest } => {
            let mut extArg: T;
            if inCont.clone() {
                (_, b, extArg) = inFunc(e1.clone(), inTypeA.clone())?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg.clone(), b.clone())?;
            (b.clone(), extArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok((outCont, outTypeA))
}

fn traverseExpsOfExpList<T: Clone + 'static>(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inExtArg: T) -> (Arc<metamodelica::List<Arc<DAE::Exp>>>, T) {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypeA: T = inExtArg.clone();
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        (e, outTypeA) = rel(e.clone(), outTypeA.clone()).unwrap();
        outExpl = cons(e.clone(), outExpl.clone());
    }
    outExpl = metamodelica::Dangerous::listReverseInPlace(outExpl.clone());
    (outExpl, outTypeA)
}

fn traverseExpsOfExpList_WithStop<T: Clone + 'static>(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inExtArg: T) -> (bool, T) {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outBoolean: bool = true;
    let mut outTypeA: T = inExtArg.clone();
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        (_, outBoolean, outTypeA) = rel(e.clone(), outTypeA.clone()).unwrap();
        if !(outBoolean.clone()) {
            break;
        }
    }
    (outBoolean, outTypeA)
}

pub fn equationEqual(mut e1: Arc<BackendDAE::Equation>, mut e2: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut res: bool = true;
    if referenceEq(&e1.clone(),&e2.clone()) {
        return Ok(res);
    }
    res = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { scalar: e12, exp: e11, .. }, Deref @ BackendDAE::Equation::EQUATION { scalar: e22, exp: e21, .. }) => {
            res = boolAnd(ExpressionBasics::expEqual(e11.clone(), e21.clone())?, ExpressionBasics::expEqual(e12.clone(), e22.clone())?);
            res.clone()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e12, left: e11, .. }, Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e22, left: e21, .. }) => {
            res = boolAnd(ExpressionBasics::expEqual(e11.clone(), e21.clone())?, ExpressionBasics::expEqual(e12.clone(), e22.clone())?);
            res.clone()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e12, left: e11, .. }, Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e22, left: e21, .. }) => {
            res = boolAnd(ExpressionBasics::expEqual(e11.clone(), e21.clone())?, ExpressionBasics::expEqual(e12.clone(), e22.clone())?);
            res.clone()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: exp1, componentRef: cr1, .. }, Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: exp2, componentRef: cr2, .. }) => {
            res = boolAnd(ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?, ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?);
            res.clone()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp1, .. }, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp2, .. }) => {
            res = ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?;
            res.clone()
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { alg: alg1, .. }, Deref @ BackendDAE::Equation::ALGORITHM { alg: alg2, .. }) => {
            let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            explst1 = Algorithm::getAllExps(alg1.clone())?;
            explst2 = Algorithm::getAllExps(alg2.clone())?;
            res = List::isEqualOnTrue(explst1.clone(), explst2.clone(), (std::sync::Arc::new(ExpressionBasics::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>));
            res.clone()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e12, left: e11, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. }, Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e22, left: e21, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. }) => {
            res = boolAnd(ExpressionBasics::expEqual(e11.clone(), e21.clone())?, ExpressionBasics::expEqual(e12.clone(), e22.clone())?);
            res.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn equationAddDAE(mut inEquation: Arc<BackendDAE::Equation>, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    outEqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem.clone(), add(inEquation.clone(), inEqSystem.orderedEqs.clone())?);
    assign_field!(outEqSystem.matching = Arc::new(crate::BackendDAE::Matching::NO_MATCHING));
    Ok(outEqSystem)
}

pub fn equationsAddDAE(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = inEqSystem.clone();
    assign_field!(
        outEqSystem.orderedEqs = addList(inEquations.clone(), outEqSystem.orderedEqs.clone())?,
        outEqSystem.matching = Arc::new(crate::BackendDAE::Matching::NO_MATCHING)
    );
    Ok(outEqSystem)
}

pub fn requationsAddDAE(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inSyst: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem>;
    outSyst = if (inEquations.clone().is_empty()) {inSyst.clone()} else {BackendDAEUtil::setEqSystRemovedEqns(inSyst.clone(), addList(inEquations.clone(), inSyst.removedEqs.clone())?)};
    Ok(outSyst)
}

pub fn removeRemovedEqs(mut eqSystem: Arc<BackendDAE::EqSystem>) -> Arc<BackendDAE::EqSystem> {
    let mut eqSystem: Arc<BackendDAE::EqSystem> = eqSystem;
    ExpandableArray::clear(eqSystem.removedEqs.clone());
    eqSystem
}

fn equationTupleToScalarResidualForm(mut cr: Arc<DAE::Exp>, mut exp: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes, mut inTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    outTpl = (::match_deref::match_deref! { match &((cr.clone(), exp.clone(), inSource.clone(), inEqAttr.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _, _, _, (i, eqs)) => {
            (i.clone() + 1, eqs.clone())
        },
        (Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, _, _, _, (i, eqs)) => {
            (i.clone() + 1, eqs.clone())
        },
        (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, _, _, _, (i, eqs)) => {
            let mut eqs = (*eqs).clone();
            eqs = cons(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: Arc::new(DAE::Exp::TSUB { exp: exp.clone(), ix: i.clone(), ty: DAE::T_REAL_DEFAULT().clone() }), source: inSource.clone(), attr: inEqAttr.clone() }), eqs.clone());
            (i.clone() + 1, eqs.clone())
        },
        (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, .. }, _, _, _, (i, eqs)) => {
            let mut e: Arc<DAE::Exp>;
            let mut eqs = (*eqs).clone();
            e = Expression::makePureBuiltinCall((literal!("sum")).clone(), list![Arc::new(DAE::Exp::TSUB { exp: exp.clone(), ix: i.clone(), ty: DAE::T_REAL_DEFAULT().clone() })], DAE::T_REAL_DEFAULT().clone());
            eqs = cons(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: inSource.clone(), attr: inEqAttr.clone() }), eqs.clone());
            (i.clone() + 1, eqs.clone())
        },
        (_, _, _, _, (i, _)) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.equationTupleToScalarResidualForm failed: ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(cr.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(inSource.clone()))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

pub fn equationToResidualForm(mut inEquation: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outEquation = 'mc: {
        let __mc_input = inEquation.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: e2, exp: e1 } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
                    (e, _) = ExpressionSimplify::simplify(exp.clone())?;
                    Ok(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: eqAttr, source, exp: e2, componentRef: cr } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    e1 = Expression::crefExp(cr.clone())?;
                    exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
                    (e, _) = ExpressionSimplify::simplify(exp.clone())?;
                    Ok(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: eqAttr, source, right: e2, left: e1, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
                    (e, _) = ExpressionSimplify::simplify(exp.clone())?;
                    Ok(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: eqAttr, source, right: e2, left: e1, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
                    (e, _) = ExpressionSimplify::simplify(exp.clone())?;
                    Ok(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                backendEq @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
                    Ok(backendEq.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                backendEq @ Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
                    Ok(backendEq.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                backendEq @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
                    Ok(backendEq.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- BackendDAE.equationToResidualForm failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEquation)
}

pub fn convertResidualsIntoSolvedEquations(mut inResidualList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inName: ArcStr, mut inIndex: i32, mut isResidual: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outEquationList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outVariableList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outVarIndex: i32 = inIndex.clone();
    for mut eq in &*inResidualList.clone() {
        let mut eq = eq.clone();
        let _ = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: eqAttr, source, exp } => {
            let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut currEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut currVariable: BackendDAE::Var;
            componentRef = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*intString(outVarIndex.clone())); ArcStr::from(__mm_s) }).clone(), identType: Expression::r#typeof(exp.clone())?, subscriptLst: metamodelica::nil() });
            currEquation = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: componentRef.clone(), exp: exp.clone(), source: source.clone(), attr: eqAttr.clone() });
            currVariable = BackendVariable::makeVar(componentRef.clone());
            if isResidual.clone() {
                currVariable = BackendVariable::setVarKind(currVariable.clone(), crate::BackendDAE::VarKind::DAE_RESIDUAL_VAR)?;
            }
            outVarIndex = outVarIndex.clone() + 1;
            outEquationList = cons(currEquation.clone(), outEquationList.clone());
            outVariableList = cons(currVariable.clone(), outVariableList.clone());
            ()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.convertResidualsIntoSolvedEquations")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outEquationList = metamodelica::Dangerous::listReverseInPlace(outEquationList.clone());
    outVariableList = metamodelica::Dangerous::listReverseInPlace(outVariableList.clone());
    Ok((outEquationList, outVariableList, outVarIndex))
}

pub fn equationInfo(mut eq: Arc<BackendDAE::Equation>) -> Result<SourceInfo> {
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = ElementSource::getElementSourceFileInfo(equationSource(eq.clone())?);
    Ok(info)
}

pub fn markedEquationSource(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inPos: i32) -> Result<Arc<DAE::ElementSource>> {
    let mut outSource: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    outSource = equationSource(get(inEqSystem.orderedEqs.clone(), inPos.clone()))?;
    Ok(outSource)
}

pub fn equationSource(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    source = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { source, .. } => source.clone(),
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { source, .. } => source.clone(),
        Deref @ BackendDAE::Equation::FOR_EQUATION { source, .. } => source.clone(),
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { source, .. } => source.clone(),
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { source, .. } => source.clone(),
        Deref @ BackendDAE::Equation::WHEN_EQUATION { source, .. } => source.clone(),
        Deref @ BackendDAE::Equation::ALGORITHM { source, .. } => source.clone(),
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { source, .. } => source.clone(),
        Deref @ BackendDAE::Equation::IF_EQUATION { source, .. } => source.clone(),
        _ => {
            Error::addInternalError((literal!("BackendEquation.equationSource failed!")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub fn equationSizeKeepAlgorithmAsOne(mut eq: Arc<BackendDAE::Equation>) -> Result<i32> {
    let mut osize: i32 = 0;
    osize = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::ALGORITHM { size: _, .. } => 1,
        _ => equationSize(eq.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osize)
}

pub fn equationSize(mut eq: Arc<BackendDAE::Equation>) -> Result<i32> {
    let mut osize: i32 = 0;
    osize = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { .. } => {
            1
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { recordSize: Some(recordSize), dimSize: ds, .. } => {
            let mut size: i32 = 0;
            size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1) * recordSize.clone();
            size.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { recordSize: None, dimSize: ds, .. } => {
            let mut size: i32 = 0;
            size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1);
            size.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { .. } => {
            1
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            1
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { size, .. } => {
            size.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { size, .. } => {
            size.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, .. } => {
            size.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse, .. } => {
            let mut size: i32 = 0;
            size = equationLstSize(eqnsfalse.clone())?;
            size.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { stop: Deref @ DAE::Exp::ICONST { integer: stop }, start: Deref @ DAE::Exp::ICONST { integer: start }, .. } => {
            let mut size: i32 = 0;
            size = (stop.clone() - start.clone() + 1) * equationSize(var_field!((*eq).body, BackendDAE::Equation::FOR_EQUATION).clone())?;
            size.clone()
        },
        _ => {
            Error::addInternalError((literal!("BackendEquation.equationSize failed!")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osize)
}

pub fn isInitialEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool = false;
    let mut eqKind: BackendDAE::EquationKind = BackendDAE::EquationKind::AUX_EQUATION;
    eqKind = equationKind(inEquation.clone())?;
    outBool = isInitialEqKind(eqKind.clone());
    Ok(outBool)
}

pub fn isInitialEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool = false;
    outBool = (match inEqKind.clone() {
        BackendDAE::EquationKind::INITIAL_EQUATION => true,
        _ => false,
    });
    outBool
}

pub fn isDynamicEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool = false;
    outBool = isDynamicEqKind(equationKind(inEquation.clone())?);
    Ok(outBool)
}

pub fn isDynamicEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool = false;
    outBool = (match inEqKind.clone() {
        BackendDAE::EquationKind::DYNAMIC_EQUATION => true,
        _ => false,
    });
    outBool
}

pub fn isBindingEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool = false;
    outBool = isBindingEqKind(equationKind(inEquation.clone())?);
    Ok(outBool)
}

pub fn isBindingEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool = false;
    outBool = (match inEqKind.clone() {
        BackendDAE::EquationKind::BINDING_EQUATION => true,
        _ => false,
    });
    outBool
}

pub fn isDiscreteEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool = false;
    outBool = isDiscreteEqKind(equationKind(inEquation.clone())?);
    Ok(outBool)
}

pub fn isDiscreteEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool = false;
    outBool = (match inEqKind.clone() {
        BackendDAE::EquationKind::DISCRETE_EQUATION => true,
        _ => false,
    });
    outBool
}

pub fn isAuxEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool = false;
    outBool = isAuxEqKind(equationKind(inEquation.clone())?);
    Ok(outBool)
}

pub fn isAuxEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool = false;
    outBool = (match inEqKind.clone() {
        BackendDAE::EquationKind::AUX_EQUATION => true,
        _ => false,
    });
    outBool
}

pub fn defaultClockedEqAttr(mut clockIndex: i32) -> BackendDAE::EquationAttributes {
    let mut outEqAttr: BackendDAE::EquationAttributes;
    outEqAttr = BackendDAE::EquationAttributes { differentiated: false, kind: BackendDAE::EquationKind::CLOCKED_EQUATION { clk: clockIndex.clone() }, evalStages: BackendDAE::defaultEvalStages.clone() };
    outEqAttr
}

pub fn equationKind(mut inEquation: Arc<BackendDAE::Equation>) -> Result<BackendDAE::EquationKind> {
    let mut outEqKind: BackendDAE::EquationKind = BackendDAE::EquationKind::AUX_EQUATION;
    outEqKind = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.equationKind")); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqKind)
}

pub fn setEquationKind(mut eq: Arc<BackendDAE::Equation>, mut k: BackendDAE::EquationKind) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::EquationKind)> {
    let mut eq: Arc<BackendDAE::Equation> = eq;
    let mut k: BackendDAE::EquationKind = k;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::EQUATION; attr = a.clone());
            eq.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::ARRAY_EQUATION; attr = a.clone());
            eq.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::FOR_EQUATION; attr = a.clone());
            eq.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::SOLVED_EQUATION; attr = a.clone());
            eq.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::RESIDUAL_EQUATION; attr = a.clone());
            eq.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::WHEN_EQUATION; attr = a.clone());
            eq.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::ALGORITHM; attr = a.clone());
            eq.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::COMPLEX_EQUATION; attr = a.clone());
            eq.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k.clone();
            assign_variant_field!(eq => BackendDAE::Equation::IF_EQUATION; attr = a.clone());
            eq.clone()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.setEquationKind")); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, k))
}

pub fn setEvalStageDynamic(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.dynamicEval = true;
    evalStage
}

pub fn setEvalStageAlgebraic(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.algebraicEval = true;
    evalStage
}

pub fn setEvalStageZeroCross(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.zerocrossEval = true;
    evalStage
}

pub fn setEvalStageDiscrete(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.discreteEval = true;
    evalStage
}

pub fn setEvalStageOnlyDiscrete(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage = setEvalStage(evalStage.clone(), false, false, false, true);
    evalStage
}

pub fn setEvalStageAll(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage = setEvalStage(evalStage.clone(), true, true, true, true);
    evalStage
}

pub fn setEvalStage(mut evalStage: BackendDAE::EvaluationStages, mut dynamicEval: bool, mut algebraicEval: bool, mut zerocrossEval: bool, mut discreteEval: bool) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.dynamicEval = dynamicEval.clone();
    evalStage.algebraicEval = algebraicEval.clone();
    evalStage.zerocrossEval = zerocrossEval.clone();
    evalStage.discreteEval = discreteEval.clone();
    evalStage
}

pub fn setEquationEvalStage(mut eqn: Arc<BackendDAE::Equation>, mut func: Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>) -> Result<Arc<BackendDAE::Equation>> {
    pub type setEvalStage = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>;

    let mut eqn: Arc<BackendDAE::Equation> = eqn;
    let mut attr: BackendDAE::EquationAttributes;
    attr = getEquationAttributes(eqn.clone())?;
    attr.evalStages = func(attr.evalStages.clone())?;
    eqn = setEquationAttributes(eqn.clone(), attr.clone())?;
    Ok(eqn)
}

pub fn equationLstSize(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut size: i32 = 0;
    for mut eqn in &*inEqns.clone() {
        let mut eqn = eqn.clone();
        size = size.clone() + equationSize(eqn.clone())?;
    }
    Ok(size)
}

pub fn equationLstSizeKeepAlgorithmAsOne(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut size: i32 = 0;
    for mut eqn in &*inEqns.clone() {
        let mut eqn = eqn.clone();
        size = size.clone() + equationSizeKeepAlgorithmAsOne(eqn.clone())?;
    }
    Ok(size)
}

pub fn generateEquation(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Expression::r#typeof(lhs.clone())?;
    outEqn = (match () {
        () if (DAEUtil::expTypeComplex(ty.clone()) || DAEUtil::expTypeTuple(ty.clone())) => {
            let mut size: i32 = 0;
            size = Expression::sizeOf(ty.clone())?;
            Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: inEqAttr.clone() })
        },
        () if (DAEUtil::expTypeArray(ty.clone())) => {
            let mut recordSize: Option<i32> = None;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut ds: Arc<metamodelica::List<i32>> = metamodelica::nil();
            tp = Expression::r#typeof(lhs.clone())?;
            tp = DAEUtil::expTypeElementType(tp.clone());
            if DAEUtil::expTypeComplex(tp.clone()) {
                recordSize = Some(Expression::sizeOf(tp.clone())?);
            } else {
                recordSize = None;
            }
            dims = Expression::arrayDimension(ty.clone());
            ds = Expression::dimensionsSizes(dims.clone());
            Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: inEqAttr.clone(), recordSize: recordSize.clone() })
        },
        () if (!(DAEUtil::expTypeComplex(ty.clone())) && !(DAEUtil::expTypeArray(ty.clone()))) => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: inEqAttr.clone() })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendEquation.generateEquation failed on: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
    });
    Ok(outEqn)
}

pub fn getEquationArraySubsetLst(mut eqnArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iLst: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut subset: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    for mut i in &*iLst.clone() {
        let mut i = i.clone();
        subset = cons(ExpandableArray::get(i.clone(), eqnArr.clone())?, subset.clone());
    }
    Ok(subset)
}

pub fn getEquationAttributes(mut inEqn: Arc<BackendDAE::Equation>) -> Result<BackendDAE::EquationAttributes> {
    let mut outAttr: BackendDAE::EquationAttributes;
    outAttr = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { attr, .. } => {
            attr.clone()
        },
        _ => {
            Error::addInternalError((literal!("function getEquationAttributes failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAttr)
}

pub fn setEquationAttributes(mut inEqn: Arc<BackendDAE::Equation>, mut inAttr: BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outEqn = (::match_deref::match_deref! { match &((inEqn.clone(), inAttr.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { source, scalar: rhs, exp: lhs, .. }, _) => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: inAttr.clone() })
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { recordSize, source, right: rhs, left: lhs, dimSize, .. }, _) => {
            Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: inAttr.clone(), recordSize: recordSize.clone() })
        },
        (Deref @ BackendDAE::Equation::FOR_EQUATION { .. }, _) => {
            Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: var_field!((*inEqn).iter, BackendDAE::Equation::FOR_EQUATION).clone(), start: var_field!((*inEqn).start, BackendDAE::Equation::FOR_EQUATION).clone(), stop: var_field!((*inEqn).stop, BackendDAE::Equation::FOR_EQUATION).clone(), body: var_field!((*inEqn).body, BackendDAE::Equation::FOR_EQUATION).clone(), source: var_field!((*inEqn).source, BackendDAE::Equation::FOR_EQUATION).clone(), attr: inAttr.clone() })
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { source, exp: rhs, componentRef, .. }, _) => {
            Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: componentRef.clone(), exp: rhs.clone(), source: source.clone(), attr: inAttr.clone() })
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { source, exp: rhs, .. }, _) => {
            Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: rhs.clone(), source: source.clone(), attr: inAttr.clone() })
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { expand, source, alg, size, .. }, _) => {
            Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: expand.clone(), attr: inAttr.clone() })
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { source, whenEquation, size, .. }, _) => {
            Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEquation.clone(), source: source.clone(), attr: inAttr.clone() })
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { source, right: rhs, left: lhs, size, .. }, _) => {
            Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: inAttr.clone() })
        },
        (Deref @ BackendDAE::Equation::IF_EQUATION { source, eqnsfalse, eqnstrue, conditions, .. }, _) => {
            Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: conditions.clone(), eqnstrue: eqnstrue.clone(), eqnsfalse: eqnsfalse.clone(), source: source.clone(), attr: inAttr.clone() })
        },
        _ => {
            Error::addInternalError((literal!("function setEquationAttributes failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

pub fn setEquationLHS(mut inEqn: Arc<BackendDAE::Equation>, mut lhs: Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outEqn = (::match_deref::match_deref! { match &(inEqn.clone()) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; exp = lhs.clone());
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; left = lhs.clone());
            eqn.clone()
        },
        _ => {
            Error::addInternalError((literal!("function setEquationLHS failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

pub fn setEquationRHS(mut inEqn: Arc<BackendDAE::Equation>, mut rhs: Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outEqn = (::match_deref::match_deref! { match &(inEqn.clone()) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; scalar = rhs.clone());
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; right = rhs.clone());
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; exp = rhs.clone());
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::RESIDUAL_EQUATION; exp = rhs.clone());
            eqn.clone()
        },
        _ => {
            Error::addInternalError((literal!("function setEquationRHS failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

pub fn generateSolvedEqnsfromOption(mut inLhs: Arc<DAE::ComponentRef>, mut inRhs: Option<Arc<DAE::Exp>>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outEqn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEqn = (::match_deref::match_deref! { match &((inLhs.clone(), inRhs.clone(), inSource.clone(), inEqAttr.clone())) {
        (_, Some(rhs), _, _) => {
            list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: inLhs.clone(), exp: rhs.clone(), source: inSource.clone(), attr: inEqAttr.clone() })]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEqn
}

pub fn generateResidualFromRelation(mut conCrefName: ArcStr, mut iRhs: Arc<DAE::Exp>, mut Source: Arc<DAE::ElementSource>, mut inVars: BackendDAE::Variables, mut knvars: BackendDAE::Variables, mut conKind: BackendDAE::VarKind) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Var)> {
    let mut outEqn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vout: BackendDAE::Var;
    (outEqn, vout) = (::match_deref::match_deref! { match &((conCrefName.clone(), iRhs.clone())) {
        (_, Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::LESS { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ }) => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lowBound: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e1.clone(), e2.clone())?;
            (rhs, _) = ExpressionSimplify::simplify1(rhs.clone())?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            lowBound = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e21_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar.clone(), Some(lowBound.clone()), Some(expNull.clone()))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs.clone(), exp: rhs.clone(), source: Source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar.clone())
        },
        (_, Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::LESSEQ { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ }) => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lowBound: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e1.clone(), e2.clone())?;
            (rhs, _) = ExpressionSimplify::simplify1(rhs.clone())?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            lowBound = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e21_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar.clone(), Some(lowBound.clone()), Some(expNull.clone()))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs.clone(), exp: rhs.clone(), source: Source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar.clone())
        },
        (_, Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::GREATER { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ }) => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lowBound: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e2.clone(), e1.clone())?;
            (rhs, _) = ExpressionSimplify::simplify1(rhs.clone())?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            lowBound = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e21_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar.clone(), Some(lowBound.clone()), Some(expNull.clone()))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs.clone(), exp: rhs.clone(), source: Source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar.clone())
        },
        (_, Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::GREATEREQ { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ }) => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lowBound: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e2.clone(), e1.clone())?;
            (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            lowBound = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e21_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar.clone(), Some(lowBound.clone()), Some(expNull.clone()))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs.clone(), exp: rhs.clone(), source: Source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar.clone())
        },
        (_, Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::EQUAL { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ }) => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e2.clone(), e1.clone())?;
            (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar.clone(), Some(expNull.clone()), Some(expNull.clone()))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs.clone(), exp: rhs.clone(), source: Source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar.clone())
        },
        (_, e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            let mut lhs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dummyVar: BackendDAE::Var;
            let mut v: BackendDAE::Var;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            match '__try0: {
                (v, _) = unwrap_break_err!(BackendVariable::getVarSingle(cr.clone(), inVars.clone()), '__try0);
                Ok::<_, anyhow::Error>((v.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    v = __try0_o0;
                }
                Err(_) => {
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), knvars.clone())?;
                }
            }
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            dummyVar = BackendVariable::mergeAliasVars(dummyVar.clone(), v.clone(), false, knvars.clone())?;
            eqn = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs.clone(), exp: e1.clone(), source: Source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
            (list![eqn.clone()], dummyVar.clone())
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEqn, vout))
}

pub fn makeTmpEqnForExp(mut iExp: Arc<DAE::Exp>, mut name: ArcStr, mut offset: i32, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut noPara: bool) -> Result<(Arc<DAE::Exp>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>, bool, bool)> {
    let mut oExp: Arc<DAE::Exp>;
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut update: bool = false;
    let mut para: bool = false;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tmpvar: BackendDAE::Var;
    let mut name_: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("__OMC__")); __mm_s.push_str(&*intString(offset.clone())); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) };
    let mut y: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut eqnVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqnKnVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut inputsKnVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut knowVars: BackendDAE::Variables;
    let mut b: bool = false;
    (y, _) = ExpressionSimplify::simplify(iExp.clone())?;
    if makeTmpEqnForExp_rule(y.clone())? {
        update = true;
        cr = ComponentReferenceBasics::makeCrefIdent((name_.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
        oExp = Expression::crefExp(cr.clone())?;
        tmpvar = BackendVariable::makeVar(cr.clone());
        tmpvar = BackendVariable::setVarTS(tmpvar.clone(), Some(crate::BackendDAE::TearingSelect::AVOID));
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: oExp.clone(), scalar: y.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); __mm_s.push_str(&*literal!(" -- new eqn--\n")); ArcStr::from(__mm_s) }).clone());
        }
        eqnVars = equationVars(eqn.clone(), ivars.clone())?;
        b = eqnVars.clone().is_empty() && !(Expression::expHasCref(y.clone(), DAE::crefTime().clone())?);
        if b.clone() {
            knowVars = BackendVariable::daeGlobalKnownVars(oshared.clone());
            eqnKnVars = equationVars(eqn.clone(), knowVars.clone())?;
            (inputsKnVars, _) = List::splitOnTrue(eqnKnVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isInput, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
            b = inputsKnVars.clone().is_empty();
        }
        b = false;
        if b.clone() {
            if noPara.clone() {
                (oExp, _) = ExpressionSimplify::simplify(iExp.clone())?;
                update = false;
            } else {
                tmpvar = BackendVariable::setBindExp(tmpvar.clone(), Some(y.clone()));
                tmpvar = BackendVariable::setVarKind(tmpvar.clone(), crate::BackendDAE::VarKind::PARAM)?;
                oshared = BackendVariable::addGlobalKnownVarDAE(tmpvar.clone(), oshared.clone())?;
                para = true;
            }
        } else {
            oeqns = add(eqn.clone(), oeqns.clone())?;
            ovars = BackendVariable::addVar(tmpvar.clone(), ovars.clone())?;
        }
    } else {
        oExp = y.clone();
        update = false;
    }
    Ok((oExp, oeqns, ovars, oshared, update, para))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeTmpEqnForExp_rule(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut allowed: bool = false;
    if Expression::isCref(inExp.clone()) || Expression::isConst(inExp.clone())? || Expression::isUnaryCref(inExp.clone()) {
        allowed = false;
        return Ok(allowed);
    }
    allowed = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } if ((Expression::isOne(e1.clone()) || Expression::isConstMinusOne(e1.clone())) && (Expression::isCref(e2.clone()) || Expression::isUnaryCref(e2.clone()))) => {
            false
        },
        Deref @ DAE::Exp::CAST { exp: e1, .. } => {
            makeTmpEqnForExp_rule(e1.clone())?
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(allowed)
}

pub fn normalizationVec(mut vec: metamodelica::Array<Arc<DAE::Exp>>, mut name: ArcStr, mut offset: i32, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<DAE::Exp>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>)> {
    let mut nvec: metamodelica::Array<Arc<DAE::Exp>>;
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut ovars: BackendDAE::Variables;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut len: Arc<DAE::Exp> = Expression::lenVec(vec.clone());
    (len, oeqns, ovars, oshared, _, _) = makeTmpEqnForExp(len.clone(), (name.clone()).clone(), offset.clone(), ieqns.clone(), ivars.clone(), ishared.clone(), false)?;
    if Expression::isZero(len.clone()) {
        bail!("fail");
    }
    nvec = Array::map1(vec.clone(), (std::sync::Arc::new(Expression::makeDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), len.clone())?;
    Ok((nvec, oeqns, ovars, oshared))
}

pub fn generateRESIDUAL_EQUATION(mut inExp: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Arc<BackendDAE::Equation> {
    let mut outEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outEqn = Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: inExp.clone(), source: inSource.clone(), attr: inEqAttr.clone() });
    outEqn
}

pub fn generateRESIDUAL_EQUATION1(mut inTpl: (Arc<DAE::Exp>, Arc<DAE::Exp>), mut source: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    (e1, e2) = inTpl.clone();
    e = Expression::createResidualExp(e1.clone(), e2.clone())?;
    outEqn = Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: inEqAttr.clone() });
    Ok(outEqn)
}

pub fn equationSystemsEqnsLst(mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eq: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    for mut es in &*systs.clone() {
        let mut es = es.clone();
        let __pa0 = ::match_deref::match_deref! { match &(es.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        eq = __pa0.clone();
        eqns = equationList(eq.clone());
        outEqns = List::append_reverse(eqns.clone(), outEqns.clone());
    }
    outEqns = metamodelica::Dangerous::listReverseInPlace(outEqns.clone());
    Ok(outEqns)
}

pub fn getEqnsFromEqSystems(mut inEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outOrderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    outOrderedEqs = listEquation(equationSystemsEqnsLst(inEqSystems.clone())?)?;
    Ok(outOrderedEqs)
}

pub fn getEqnsFromEqSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outOrderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqSystem.orderedEqs.clone();
    outOrderedEqs
}

pub fn getInitialEqnsFromShared(mut inShared: Arc<BackendDAE::Shared>) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outInitialEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inShared.initialEqs.clone();
    outInitialEqs
}

pub fn aliasEquation(mut inEqn: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>> = metamodelica::nil();
    outTpls = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. } => {
            aliasEquation1(e1.clone(), e2.clone(), metamodelica::nil())?
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. } => {
            aliasEquation1(e1.clone(), e2.clone(), metamodelica::nil())?
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. } => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::crefExp(cr.clone())?;
            aliasEquation1(e.clone(), e2.clone(), metamodelica::nil())?
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, .. } => {
            aliasExpression(e1.clone(), metamodelica::nil())?
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. } => {
            aliasEquation1(e1.clone(), e2.clone(), metamodelica::nil())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpls)
}

fn aliasEquation1(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>> = metamodelica::nil();
    outTpls = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone(), inTpls.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _) => {
            cons((cr1.clone(), cr2.clone(), lhs.clone(), rhs.clone(), false), inTpls.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: lhs.clone() }), rhs.clone(), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: lhs.clone() }), rhs.clone(), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _) => {
            cons((cr1.clone(), cr2.clone(), lhs.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: rhs.clone() }), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _) => {
            cons((cr1.clone(), cr2.clone(), lhs.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: rhs.clone() }), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: lhs.clone() }), rhs.clone(), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _) => {
            cons((cr1.clone(), cr2.clone(), lhs.clone(), Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: rhs.clone() }), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls.clone())
        },
        (Deref @ DAE::Exp::ARRAY { array: elst1, .. }, Deref @ DAE::Exp::ARRAY { array: elst2, .. }, _) => {
            List::threadFold(elst1.clone(), elst2.clone(), (std::sync::Arc::new(aliasEquation1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls.clone())?
        },
        (Deref @ DAE::Exp::MATRIX { matrix: elstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: elstlst2, .. }, _) => {
            List::threadFold(elstlst1.clone(), elstlst2.clone(), (std::sync::Arc::new(aliasEquationLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls.clone())?
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: pathb1 }, varLst: varLst2, .. }, .. }, expLst: elst2, path: pathb }, _) if (AbsynUtil::pathEqual(pathb.clone(), pathb1.clone())) => {
            aliasRecord(cr1.clone(), varLst2.clone(), elst2.clone(), inTpls.clone())?
        },
        (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: patha1 }, varLst: varLst1, .. }, .. }, expLst: elst1, path: patha }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _) if (AbsynUtil::pathEqual(patha.clone(), patha1.clone())) => {
            aliasRecord(cr2.clone(), varLst1.clone(), elst1.clone(), inTpls.clone())?
        },
        (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: patha1 }, .. }, .. }, expLst: elst1, path: patha }, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: pathb1 }, .. }, .. }, expLst: elst2, path: pathb }, _) if (AbsynUtil::pathEqual(patha.clone(), patha1.clone()) && AbsynUtil::pathEqual(pathb.clone(), pathb1.clone())) => {
            List::threadFold(elst1.clone(), elst2.clone(), (std::sync::Arc::new(aliasEquation1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls.clone())?
        },
        _ => {
            aliasEquation2(lhs.clone(), rhs.clone(), inTpls.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpls)
}

fn aliasEquationLst(mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut elst2: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>> = metamodelica::nil();
    outTpls = List::threadFold(elst1.clone(), elst2.clone(), (std::sync::Arc::new(aliasEquation1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls.clone())?;
    Ok(outTpls)
}

fn aliasEquation2(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>> = metamodelica::nil();
    outTpls = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone(), inTpls.clone())) {
        (Deref @ DAE::Exp::ARRAY { array: elst1, .. }, _, _) if (Expression::isZero(rhs.clone())) => {
            List::fold(elst1.clone(), (std::sync::Arc::new(aliasExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls.clone())
        },
        (_, Deref @ DAE::Exp::ARRAY { array: elst2, .. }, _) if (Expression::isZero(lhs.clone())) => {
            List::fold(elst2.clone(), (std::sync::Arc::new(aliasExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls.clone())
        },
        (_, _, _) if (Expression::isZero(rhs.clone())) => {
            aliasExpression(lhs.clone(), inTpls.clone())?
        },
        (_, _, _) if (Expression::isZero(lhs.clone())) => {
            aliasExpression(rhs.clone(), inTpls.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpls)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn aliasRecord(mut cr: Arc<DAE::ComponentRef>, mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>> = metamodelica::nil();
    outTpls = (::match_deref::match_deref! { match &((cr.clone(), varLst.clone(), explst.clone(), inTpls.clone())) {
        (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            inTpls.clone()
        },
        (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, name: ident, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, tail: elst }, _) => {
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e1: Arc<DAE::Exp>;
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (ident.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e1 = Arc::new(DAE::Exp::CREF { componentRef: cr1.clone(), ty: ty.clone() });
            aliasRecord(cr.clone(), vlst.clone(), elst.clone(), cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls.clone()))?
        },
        (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, name: ident, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: e2 @ Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, tail: elst }, _) => {
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e1: Arc<DAE::Exp>;
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (ident.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e1 = Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: Arc::new(DAE::Exp::CREF { componentRef: cr1.clone(), ty: ty.clone() }) });
            aliasRecord(cr.clone(), vlst.clone(), elst.clone(), cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), true), inTpls.clone()))?
        },
        (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, name: ident, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: e2 @ Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, tail: elst }, _) => {
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e1: Arc<DAE::Exp>;
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (ident.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e1 = Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: Arc::new(DAE::Exp::CREF { componentRef: cr1.clone(), ty: ty.clone() }) });
            aliasRecord(cr.clone(), vlst.clone(), elst.clone(), cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), true), inTpls.clone()))?
        },
        (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, name: ident, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: e2 @ Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, tail: elst }, _) => {
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e1: Arc<DAE::Exp>;
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (ident.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e1 = Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: Arc::new(DAE::Exp::CREF { componentRef: cr1.clone(), ty: ty.clone() }) });
            aliasRecord(cr.clone(), vlst.clone(), elst.clone(), cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), true), inTpls.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpls)
}

fn aliasExpression(mut exp: Arc<DAE::Exp>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>> = metamodelica::nil();
    outTpls = (::match_deref::match_deref! { match &((exp.clone(), inTpls.clone())) {
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::ADD { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() }), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() }), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::ADD_ARR { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e1.clone() }), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::SUB { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::ADD { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::SUB { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() }), true), inTpls.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::SUB_ARR { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _) => {
            cons((cr1.clone(), cr2.clone(), e1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), true), inTpls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpls)
}

pub fn derivativeEquation(mut eqn: Arc<BackendDAE::Equation>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)> {
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    let mut de: Arc<DAE::Exp>;
    let mut negate: bool = false;
    (cr, dcr, e, de, negate) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: de @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, exp: e @ Deref @ DAE::Exp::CREF { componentRef: dcr, .. }, .. } => {
            (cr.clone(), dcr.clone(), e.clone(), de.clone(), false)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: e @ Deref @ DAE::Exp::CREF { componentRef: dcr, .. }, exp: de @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, .. } => {
            (cr.clone(), dcr.clone(), e.clone(), de.clone(), false)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, exp: e @ Deref @ DAE::Exp::CREF { componentRef: dcr, .. }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            (cr.clone(), dcr.clone(), ne.clone(), de.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, exp: e @ Deref @ DAE::Exp::CREF { componentRef: dcr, .. }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            (cr.clone(), dcr.clone(), ne.clone(), de.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: e @ Deref @ DAE::Exp::CREF { componentRef: dcr, .. }, exp: de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            (cr.clone(), dcr.clone(), ne.clone(), de.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: e @ Deref @ DAE::Exp::CREF { componentRef: dcr, .. }, exp: de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            (cr.clone(), dcr.clone(), ne.clone(), de.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: de @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, exp: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: dcr, .. } }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), e.clone(), ne.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: de @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, exp: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: dcr, .. } }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), e.clone(), ne.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: dcr, .. } }, exp: de @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), e.clone(), ne.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: dcr, .. } }, exp: de @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), e.clone(), ne.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, exp: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: dcr, .. } }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            let mut ne2: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            ne2 = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), ne.clone(), ne2.clone(), false)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, exp: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: dcr, .. } }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            let mut ne2: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            ne2 = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), ne.clone(), ne2.clone(), false)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: dcr, .. } }, exp: de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            let mut ne2: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            ne2 = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), ne.clone(), ne2.clone(), false)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: dcr, .. } }, exp: de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, .. } => {
            let mut ne: Arc<DAE::Exp>;
            let mut ne2: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            ne2 = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), ne.clone(), ne2.clone(), false)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cr, dcr, e, de, negate))
}

pub fn addOperation(mut inEqn: Arc<BackendDAE::Equation>, mut inSymOp: Arc<DAE::SymbolicOperation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outEqn = (::match_deref::match_deref! { match &(inEqn.clone()) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::EQUATION).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::ARRAY_EQUATION).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::SOLVED_EQUATION).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::RESIDUAL_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::RESIDUAL_EQUATION).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ALGORITHM; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::ALGORITHM).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::WHEN_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::WHEN_EQUATION).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::COMPLEX_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::COMPLEX_EQUATION).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::IF_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::IF_EQUATION).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::FOR_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::FOR_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::FOR_EQUATION).clone(), inSymOp.clone())?);
            eqn.clone()
        },
        _ => {
            Error::addInternalError((literal!("BackendEquation.addOperation failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

pub fn isEquationsSystem(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isTornSystem(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isWhenEquation(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isWhenEquationOrDiscreteAlgorithm(mut inEqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = inEqn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
                    let mut b1: bool = false;
                    let mut lhsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    b1 = true;
                    for mut s in &*stmts.clone() {
                        let mut s = s.clone();
                        (lhsCrefs, _) = Expression::extractCrefsStatment(s.clone())?;
                        for mut c in &*lhsCrefs.clone() {
                            let mut c = c.clone();
                            b1 = b1.clone() && BackendVariable::isDiscrete(c.clone(), vars.clone())?;
                        }
                    }
                    Ok(b1.clone())
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
    Ok(b)
}

pub fn isArrayEquation(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isAlgorithm(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isComplexEquation(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isEquation(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isNotAlgorithm(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = !(isAlgorithm(inEqn.clone()));
    b
}

pub fn markDifferentiated(mut inEqn: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outEqn = (::match_deref::match_deref! { match &(inEqn.clone()) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::EQUATION).clone()));
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::ARRAY_EQUATION).clone()));
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::SOLVED_EQUATION).clone()));
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::RESIDUAL_EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::RESIDUAL_EQUATION).clone()));
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::COMPLEX_EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::COMPLEX_EQUATION).clone()));
            eqn.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            inEqn.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            inEqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::IF_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION;
                attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::IF_EQUATION).clone()),
                eqnstrue = List::mapList(var_field!((*eqn).eqnstrue, BackendDAE::Equation::IF_EQUATION).clone(), (std::sync::Arc::new(markDifferentiated) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>)),
                eqnsfalse = List::map(var_field!((*eqn).eqnsfalse, BackendDAE::Equation::IF_EQUATION).clone(), (std::sync::Arc::new(markDifferentiated) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))
            );
            eqn.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqn)
}

fn markDifferentiated2(mut attr: BackendDAE::EquationAttributes) -> BackendDAE::EquationAttributes {
    let mut attr: BackendDAE::EquationAttributes = attr;
    attr.differentiated = true;
    attr
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isDifferentiated(mut inEqn: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut diffed: bool = false;
    diffed = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            b.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            b.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            b.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            b.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            b.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            b.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            b.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: Deref @ metamodelica::List::Cons { head: eqn, tail: _ }, .. } => {
            isDifferentiated(eqn.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(diffed)
}

pub fn replaceDerOpInEquationList(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (outEqns, _) = traverseExpsOfEquationList(inEqns.clone(), (std::sync::Arc::new(Expression::replaceDerOpInExpCond) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>)> + 'static>), None)?;
    Ok(outEqns)
}

pub fn getEquationRHS(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut rhs: Arc<DAE::Exp>;
    rhs = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: exp1, .. }, tail: Deref @ metamodelica::List::Nil }, condition: Deref @ DAE::Exp::BCONST { bool: true }, .. }, .. } => {
            exp1.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rhs)
}

pub fn getEquationLHS(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut lhs: Arc<DAE::Exp>;
    lhs = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, .. } => {
            Expression::crefExp(cref.clone())?
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: exp1, .. }, tail: Deref @ metamodelica::List::Nil }, condition: Deref @ DAE::Exp::BCONST { bool: true }, .. }, .. } => {
            exp1.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lhs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn allAlgorithmsLst(mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(eqn_lst.clone()) {
        Deref @ metamodelica::List::Nil => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { .. }, tail: Deref @ metamodelica::List::Nil } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { .. }, tail: rest } => {
            allAlgorithmsLst(rest.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn createResidualExp(mut eqn: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    res = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. } => {
            Expression::createResidualExp(e1.clone(), e2.clone())?
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. } => {
            Expression::createResidualExp(e1.clone(), e2.clone())?
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. } => {
            Expression::createResidualExp(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ComponentReference::crefTypeFull(cr.clone())? }), e2.clone())?
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, .. } => {
            e1.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. } => {
            Expression::createResidualExp(e1.clone(), e2.clone())?
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn hasAnyUnknown(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut b: bool = false;
    b = !(equationVars(eqn.clone(), vars.clone())?.is_empty());
    Ok(b)
}

