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

use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedSet;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NFAlgorithm {
    pub statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>,
    pub inputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
    pub outputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
    pub stmtDiffInfo: Option<Arc<UnorderedSet::UnorderedSet<Arc<Statement::NFStatement>>>>,
    pub scope: Arc<InstNode::InstNode>,
    pub source: Arc<DAE::ElementSource>,
}

pub type ALGORITHM = NFAlgorithm;

pub type ApplyFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<()> + 'static>;

pub fn applyList(mut algs: Arc<metamodelica::List<Arc<NFAlgorithm>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<()> + 'static>) -> () {
    for mut alg in &*algs.clone() {
        let mut alg = alg.clone();
        for mut s in &*alg.statements.clone() {
            let mut s = s.clone();
            Statement::apply(s.clone(), func.clone());
        }
    }
    ()
}

pub fn apply(mut alg: Arc<NFAlgorithm>, mut func: Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<()> + 'static>) -> () {
    for mut s in &*alg.statements.clone() {
        let mut s = s.clone();
        Statement::apply(s.clone(), func.clone());
    }
    ()
}

pub fn applyExp(mut alg: Arc<NFAlgorithm>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    for mut s in &*alg.statements.clone() {
        let mut s = s.clone();
        Statement::applyExp(s.clone(), func.clone())?;
    }
    Ok(())
}

pub fn applyExpList(mut algs: Arc<metamodelica::List<Arc<NFAlgorithm>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    for mut alg in &*algs.clone() {
        let mut alg = alg.clone();
        applyExp(alg.clone(), func.clone())?;
    }
    Ok(())
}

pub fn map(mut alg: Arc<NFAlgorithm>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> + 'static>) -> Arc<NFAlgorithm> {
    pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> + 'static>;

    let mut alg: Arc<NFAlgorithm> = alg;
    assign_field!(alg.statements = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (alg.statements.clone()).into_iter().cloned() {
            let __x = Statement::map(s.clone(), r#fn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    alg
}

pub fn mapExp(mut alg: Arc<NFAlgorithm>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFAlgorithm> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut alg: Arc<NFAlgorithm> = alg;
    assign_field!(alg.statements = Statement::mapExpList(alg.statements.clone(), func.clone()));
    alg
}

pub fn mapExpList(mut algs: Arc<metamodelica::List<Arc<NFAlgorithm>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<metamodelica::List<Arc<NFAlgorithm>>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut algs: Arc<metamodelica::List<Arc<NFAlgorithm>>> = algs;
    algs = {
        let mut __acc: Arc<metamodelica::List<Arc<NFAlgorithm>>> = metamodelica::nil();
        for mut alg in (algs.clone()).into_iter().cloned() {
            let __x = mapExp(alg.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    algs
}

pub fn foldExp<ArgT: Clone + 'static>(mut alg: Arc<NFAlgorithm>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut s in &*alg.statements.clone() {
        let mut s = s.clone();
        arg = Statement::foldExp(s.clone(), func.clone(), arg.clone())?;
    }
    Ok(arg)
}

pub fn foldExpList<ArgT: Clone + 'static>(mut algs: Arc<metamodelica::List<Arc<NFAlgorithm>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut alg in &*algs.clone() {
        let mut alg = alg.clone();
        arg = foldExp(alg.clone(), func.clone(), arg.clone())?;
    }
    Ok(arg)
}

pub fn toString(mut alg: Arc<NFAlgorithm>, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (Statement::toStringList(alg.statements.clone(), (indent.clone()).clone())?).clone();
    Ok(r#str)
}

pub fn setInputsOutputs(mut alg: Arc<NFAlgorithm>) -> Result<Arc<NFAlgorithm>> {
    let mut alg: Arc<NFAlgorithm> = alg;
    let mut inputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut outputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    (inputs, outputs) = getInputsOutputs(alg.statements.clone())?;
    assign_field!(
        alg.inputs = inputs.clone(),
        alg.outputs = outputs.clone()
    );
    Ok(alg)
}

pub fn getInputsOutputs(mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)> {
    let mut inputs_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut outputs_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut inputs_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut outputs_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    match '__try0: {
        for mut statement in &*statements.clone() {
            let mut statement = statement.clone();
            unwrap_break_err!(statementInputsOutputs(statement.clone(), inputs_set.clone(), outputs_set.clone()), '__try0);
        }
        inputs_lst = UnorderedSet::toList(inputs_set.clone());
        outputs_lst = UnorderedSet::toList(outputs_set.clone());
        Ok::<_, anyhow::Error>((inputs_lst.clone(), outputs_lst.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            inputs_lst = __try0_o0;
            outputs_lst = __try0_o1;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFAlgorithm.getInputsOutputs")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((inputs_lst, outputs_lst))
}

pub fn isEqual(mut alg1: Arc<NFAlgorithm>, mut alg2: Arc<NFAlgorithm>) -> bool {
    let mut b: bool = false;
    b = List::isEqualOnTrue(alg1.inputs.clone(), alg2.inputs.clone(), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>)) && List::isEqualOnTrue(alg1.outputs.clone(), alg2.outputs.clone(), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>)) && List::isEqualOnTrue(alg1.statements.clone(), alg2.statements.clone(), (std::sync::Arc::new(Statement::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<Statement::NFStatement>) -> Result<bool> + 'static>));
    b
}

pub fn isEmpty(mut alg: Arc<NFAlgorithm>) -> bool {
    let mut b: bool = alg.statements.clone().is_empty();
    b
}

pub fn isDiscrete(mut alg: Arc<NFAlgorithm>) -> bool {
    let mut b: bool = false;
    b = List::any(alg.outputs.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::isDiscrete, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>));
    b = if (b.clone()) {b.clone()} else {List::any(alg.statements.clone(), (std::sync::Arc::new(fnptr!(Statement::isDiscrete, Arc<Statement::NFStatement>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<bool> + 'static>))};
    b
}

fn statementInputsOutputs(mut statement: Arc<Statement::NFStatement>, mut inputs_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut outputs_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(statement.clone()) {
        Deref @ Statement::ASSIGNMENT { rhs, lhs: lhs @ Deref @ Expression::CREF { .. }, .. } => {
            Expression::apply(rhs.clone(), Arc::new({ let __pe_b1 = inputs_set.clone(); let __pe_b2 = outputs_set.clone(); move |__pe_a0| expressionInputs(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            expressionOutput(lhs.clone(), inputs_set.clone(), outputs_set.clone())?;
            ()
        },
        Deref @ Statement::ASSIGNMENT { rhs, lhs: Deref @ Expression::TUPLE { elements, .. }, .. } => {
            Expression::apply(rhs.clone(), Arc::new({ let __pe_b1 = inputs_set.clone(); let __pe_b2 = outputs_set.clone(); move |__pe_a0| expressionInputs(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            for mut exp in &*elements.clone() {
                let mut exp = exp.clone();
                expressionOutput(exp.clone(), inputs_set.clone(), outputs_set.clone())?;
            }
            ()
        },
        Deref @ Statement::FOR { body: stmts, .. } => {
            for mut stmt in &*stmts.clone() {
                let mut stmt = stmt.clone();
                statementInputsOutputs(stmt.clone(), inputs_set.clone(), outputs_set.clone())?;
            }
            ()
        },
        Deref @ Statement::IF { branches, .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            for mut branch in &*branches.clone() {
                let mut branch = branch.clone();
                (_, stmts) = branch.clone();
                for mut stmt in &*stmts.clone() {
                    let mut stmt = stmt.clone();
                    statementInputsOutputs(stmt.clone(), inputs_set.clone(), outputs_set.clone())?;
                }
            }
            ()
        },
        Deref @ Statement::WHEN { branches, .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            for mut branch in &*branches.clone() {
                let mut branch = branch.clone();
                (_, stmts) = branch.clone();
                for mut stmt in &*stmts.clone() {
                    let mut stmt = stmt.clone();
                    statementInputsOutputs(stmt.clone(), inputs_set.clone(), outputs_set.clone())?;
                }
            }
            ()
        },
        Deref @ Statement::WHILE { body: stmts, .. } => {
            for mut stmt in &*stmts.clone() {
                let mut stmt = stmt.clone();
                statementInputsOutputs(stmt.clone(), inputs_set.clone(), outputs_set.clone())?;
            }
            ()
        },
        Deref @ Statement::ASSERT { .. } => {
            ()
        },
        Deref @ Statement::TERMINATE { .. } => {
            ()
        },
        Deref @ Statement::REINIT { .. } => {
            ()
        },
        Deref @ Statement::NORETCALL { .. } => {
            ()
        },
        Deref @ Statement::RETURN { .. } => {
            ()
        },
        Deref @ Statement::BREAK { .. } => {
            ()
        },
        Deref @ Statement::FAILURE { .. } => {
            ()
        },
        Deref @ Statement::FUNCTION_ARRAY_INIT { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFAlgorithm.statementInputsOutputs")); __mm_s.push_str(&*literal!(" failed due to wrong Statement Type: FUNCTION_ARRAY_INIT.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFAlgorithm.statementInputsOutputs")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*Statement::toString(statement.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            }
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn expressionInputs(mut exp: Arc<Expression::NFExpression>, mut inputs_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut outputs_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: cr, .. } if (ComponentRef::isTime(cr.clone())) => {
            ()
        },
        Deref @ Expression::CREF { cref: cr, .. } if (ComponentRef::isIterator(cr.clone())) => {
            ()
        },
        Deref @ Expression::CREF { ty, .. } if (Type::isExternalObject(ty.clone())) => {
            ()
        },
        Deref @ Expression::CREF { cref: cr, .. } => {
            let mut cr = (*cr).clone();
            cr = ComponentRef::stripSubscriptsAll(cr.clone());
            if !(UnorderedSet::contains(cr.clone(), outputs_set.clone())?) {
                UnorderedSet::add(cr.clone(), inputs_set.clone())?;
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

fn expressionOutput(mut exp: Arc<Expression::NFExpression>, mut inputs_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut outputs_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. } => {
            ()
        },
        Deref @ Expression::CREF { cref: cr, .. } if (ComponentRef::isTime(cr.clone())) => {
            Error::addMessage(Error::COMPILER_ERROR.clone(), list![(literal!("Trying to assign to time.")).clone()])?;
            bail!("fail")
        },
        Deref @ Expression::CREF { cref: cr, .. } if (ComponentRef::isIterator(cr.clone())) => {
            Error::addMessage(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Trying to assign to iterator ")); __mm_s.push_str(&*ComponentRef::toString(cr.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        Deref @ Expression::CREF { ty, .. } if (Type::isExternalObject(ty.clone())) => {
            ()
        },
        Deref @ Expression::CREF { cref: cr, .. } => {
            let mut cr = (*cr).clone();
            cr = ComponentRef::stripSubscriptsAll(cr.clone());
            if UnorderedSet::remove(cr.clone(), inputs_set.clone())? {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using output variable in RHS before it is assigned (former occurences will be set to initial value): ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
                }
            }
            UnorderedSet::add(cr.clone(), outputs_set.clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFAlgorithm.expressionOutput")); __mm_s.push_str(&*literal!(" failed due to wrong expression type in LHS of algorithm statement: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}


