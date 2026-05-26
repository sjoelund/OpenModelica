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
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInstNode::InstNode;
use crate::NFType as Type;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::IOStream;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub enum NFStatement {
    ASSIGNMENT {
        /// The asignee
        lhs: Arc<Expression::NFExpression>,
        /// The expression
        rhs: Arc<Expression::NFExpression>,
        ty: Arc<Type::NFType>,
        source: Arc<DAE::ElementSource>,
    },
    /// Used to mark in which order local array variables in functions should be initialized
    FUNCTION_ARRAY_INIT {
        name: ArcStr,
        ty: Arc<Type::NFType>,
        source: Arc<DAE::ElementSource>,
    },
    FOR {
        iterator: Arc<InstNode::InstNode>,
        range: Option<Arc<Expression::NFExpression>>,
        /// The body of the for loop.
        body: Arc<metamodelica::List<Arc<NFStatement>>>,
        forType: Arc<ForType>,
        source: Arc<DAE::ElementSource>,
    },
    IF {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>>,
        source: Arc<DAE::ElementSource>,
    },
    WHEN {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>>,
        source: Arc<DAE::ElementSource>,
    },
    ASSERT {
        /// The assert condition.
        condition: Arc<Expression::NFExpression>,
        /// The message to display if the assert fails.
        message: Arc<Expression::NFExpression>,
        level: Arc<Expression::NFExpression>,
        source: Arc<DAE::ElementSource>,
    },
    TERMINATE {
        /// The message to display if the terminate triggers.
        message: Arc<Expression::NFExpression>,
        source: Arc<DAE::ElementSource>,
    },
    REINIT {
        cref: Arc<Expression::NFExpression>,
        reinitExp: Arc<Expression::NFExpression>,
        source: Arc<DAE::ElementSource>,
    },
    NORETCALL {
        exp: Arc<Expression::NFExpression>,
        source: Arc<DAE::ElementSource>,
    },
    WHILE {
        condition: Arc<Expression::NFExpression>,
        body: Arc<metamodelica::List<Arc<NFStatement>>>,
        source: Arc<DAE::ElementSource>,
    },
    RETURN {
        source: Arc<DAE::ElementSource>,
    },
    BREAK {
        source: Arc<DAE::ElementSource>,
    },
    FAILURE {
        body: Arc<metamodelica::List<Arc<NFStatement>>>,
        source: Arc<DAE::ElementSource>,
    },
}
pub use self::NFStatement::{ASSIGNMENT,FUNCTION_ARRAY_INIT,FOR,IF,WHEN,ASSERT,TERMINATE,REINIT,NORETCALL,WHILE,RETURN,BREAK,FAILURE};
#[derive(Clone, Debug, PartialEq)]
pub enum ForType {
    NORMAL,
    PARALLEL {
        vars: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, SourceInfo)>>,
    },
}
impl Default for ForType {
    fn default() -> Self { Self::NORMAL }
}
pub use self::ForType::{NORMAL,PARALLEL};

pub fn isDiscrete(mut stmt: Arc<NFStatement>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => Type::isDiscrete(var_field!((*stmt).ty, NFStatement::ASSIGNMENT).clone()),
        Deref @ FUNCTION_ARRAY_INIT { .. } => Type::isDiscrete(var_field!((*stmt).ty, NFStatement::FUNCTION_ARRAY_INIT).clone()),
        Deref @ FOR { .. } => List::any(var_field!((*stmt).body, NFStatement::FOR).clone(), Arc::new(fnptr!(isDiscrete, Arc<NFStatement>))),
        Deref @ IF { .. } => {
            for mut branch in &*var_field!((*stmt).branches, NFStatement::IF).clone() {
                let mut branch = branch.clone();
                b = List::any(Util::tuple22(branch.clone()), Arc::new(fnptr!(isDiscrete, Arc<NFStatement>)));
                if b.clone() {
                    break;
                }
            }
            b.clone()
        },
        Deref @ WHEN { .. } => true,
        Deref @ WHILE { .. } => List::any(var_field!((*stmt).body, NFStatement::WHILE).clone(), Arc::new(fnptr!(isDiscrete, Arc<NFStatement>))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn filterDiscrete(mut stmts: Arc<metamodelica::List<Arc<NFStatement>>>, mut out_stmts: Arc<metamodelica::List<Arc<NFStatement>>>) -> Arc<metamodelica::List<Arc<NFStatement>>> {
    let mut out_stmts: Arc<metamodelica::List<Arc<NFStatement>>> = out_stmts;
    let mut stmt: Arc<NFStatement>;
    let mut rest: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
    out_stmts = (::match_deref::match_deref! { match &(stmts.clone()) {
        Deref @ metamodelica::List::Cons { head: stmt @ Deref @ FOR { .. }, tail: rest } => {
            let mut stmt = (*stmt).clone();
            assign_variant_field!(stmt => NFStatement::FOR; body = filterDiscrete(var_field!((*stmt).body, NFStatement::FOR).clone(), metamodelica::nil()));
            out_stmts = if ((var_field!((*stmt).body, NFStatement::FOR).clone().len() as i32) == 0) {out_stmts.clone()} else {cons(stmt.clone(), out_stmts.clone())};
            filterDiscrete(rest.clone(), out_stmts.clone())
        },
        Deref @ metamodelica::List::Cons { head: stmt @ Deref @ IF { .. }, tail: rest } => {
            let mut stmt = (*stmt).clone();
            assign_variant_field!(stmt => NFStatement::IF; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
        for mut tpl in (var_field!((*stmt).branches, NFStatement::IF).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(tpl.clone()), filterDiscrete(Util::tuple22(tpl.clone()), metamodelica::nil()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            filterDiscrete(rest.clone(), cons(stmt.clone(), out_stmts.clone()))
        },
        Deref @ metamodelica::List::Cons { head: stmt, tail: rest } if (isDiscrete(stmt.clone())) => filterDiscrete(rest.clone(), out_stmts.clone()),
        Deref @ metamodelica::List::Cons { head: stmt, tail: rest } => filterDiscrete(rest.clone(), cons(stmt.clone(), out_stmts.clone())),
        _ => out_stmts.clone().reverse(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_stmts
}

pub fn hash(mut stmt: Arc<NFStatement>) -> i32 {
    let mut hash: i32 = stringHashDjb2((toString(stmt.clone(), (literal!("")).clone()).unwrap()).clone());
    hash
}

pub fn isEqual(mut stmt1: Arc<NFStatement>, mut stmt2: Arc<NFStatement>) -> Result<bool> {
    fn branchEqual(mut branch1: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>), mut branch2: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)) -> Result<bool> {
        let mut b: bool = false;
        let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut b1: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
        let mut b2: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
        (e1, b1) = branch1.clone();
        (e2, b2) = branch2.clone();
        b = Expression::isEqual(e1.clone(), e2.clone())? && List::isEqualOnTrue(b1.clone(), b2.clone(), Arc::new(isEqual));
        Ok(b)
    }

    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((stmt1.clone(), stmt2.clone())) {
        (Deref @ ASSIGNMENT { .. }, Deref @ ASSIGNMENT { .. }) => Expression::isEqual(var_field!((*stmt1).lhs, NFStatement::ASSIGNMENT).clone(), var_field!((*stmt2).lhs, NFStatement::ASSIGNMENT).clone())? && Expression::isEqual(var_field!((*stmt1).rhs, NFStatement::ASSIGNMENT).clone(), var_field!((*stmt2).rhs, NFStatement::ASSIGNMENT).clone())?,
        (Deref @ FUNCTION_ARRAY_INIT { .. }, Deref @ FUNCTION_ARRAY_INIT { .. }) => stringEqual((var_field!((*stmt1).name, NFStatement::FUNCTION_ARRAY_INIT).clone()).clone(), (var_field!((*stmt2).name, NFStatement::FUNCTION_ARRAY_INIT).clone()).clone()),
        (Deref @ FOR { .. }, Deref @ FOR { .. }) => InstNode::nameEqual(var_field!((*stmt1).iterator, NFStatement::FOR).clone(), var_field!((*stmt2).iterator, NFStatement::FOR).clone()) && Util::optionEqual(var_field!((*stmt1).range, NFStatement::FOR).clone(), var_field!((*stmt2).range, NFStatement::FOR).clone(), Arc::new(Expression::isEqual)) && List::isEqualOnTrue(var_field!((*stmt1).body, NFStatement::FOR).clone(), var_field!((*stmt2).body, NFStatement::FOR).clone(), Arc::new(isEqual)),
        (Deref @ IF { .. }, Deref @ IF { .. }) => List::isEqualOnTrue(var_field!((*stmt1).branches, NFStatement::IF).clone(), var_field!((*stmt2).branches, NFStatement::IF).clone(), Arc::new(branchEqual)),
        (Deref @ WHEN { .. }, Deref @ WHEN { .. }) => List::isEqualOnTrue(var_field!((*stmt1).branches, NFStatement::WHEN).clone(), var_field!((*stmt2).branches, NFStatement::WHEN).clone(), Arc::new(branchEqual)),
        (Deref @ ASSERT { .. }, Deref @ ASSERT { .. }) => Expression::isEqual(var_field!((*stmt1).condition, NFStatement::ASSERT).clone(), var_field!((*stmt2).condition, NFStatement::ASSERT).clone())? && Expression::isEqual(var_field!((*stmt1).message, NFStatement::ASSERT).clone(), var_field!((*stmt2).message, NFStatement::ASSERT).clone())? && Expression::isEqual(var_field!((*stmt1).level, NFStatement::ASSERT).clone(), var_field!((*stmt2).level, NFStatement::ASSERT).clone())?,
        (Deref @ TERMINATE { .. }, Deref @ TERMINATE { .. }) => Expression::isEqual(var_field!((*stmt1).message, NFStatement::TERMINATE).clone(), var_field!((*stmt2).message, NFStatement::TERMINATE).clone())?,
        (Deref @ REINIT { .. }, Deref @ REINIT { .. }) => Expression::isEqual(var_field!((*stmt1).cref, NFStatement::REINIT).clone(), var_field!((*stmt2).cref, NFStatement::REINIT).clone())? && Expression::isEqual(var_field!((*stmt1).reinitExp, NFStatement::REINIT).clone(), var_field!((*stmt2).reinitExp, NFStatement::REINIT).clone())?,
        (Deref @ NORETCALL { .. }, Deref @ NORETCALL { .. }) => Expression::isEqual(var_field!((*stmt1).exp, NFStatement::NORETCALL).clone(), var_field!((*stmt2).exp, NFStatement::NORETCALL).clone())?,
        (Deref @ WHILE { .. }, Deref @ WHILE { .. }) => Expression::isEqual(var_field!((*stmt1).condition, NFStatement::WHILE).clone(), var_field!((*stmt2).condition, NFStatement::WHILE).clone())? && List::isEqualOnTrue(var_field!((*stmt1).body, NFStatement::WHILE).clone(), var_field!((*stmt2).body, NFStatement::WHILE).clone(), Arc::new(isEqual)),
        (Deref @ RETURN { .. }, Deref @ RETURN { .. }) => true,
        (Deref @ BREAK { .. }, Deref @ BREAK { .. }) => true,
        (Deref @ FAILURE { .. }, Deref @ FAILURE { .. }) => List::isEqualOnTrue(var_field!((*stmt1).body, NFStatement::FAILURE).clone(), var_field!((*stmt2).body, NFStatement::FAILURE).clone(), Arc::new(isEqual)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn makeAssignment(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>, mut src: Arc<DAE::ElementSource>) -> Arc<NFStatement> {
    let mut stmt: Arc<NFStatement>;
    stmt = Arc::new(NFStatement::ASSIGNMENT { lhs: lhs.clone(), rhs: rhs.clone(), ty: ty.clone(), source: src.clone() });
    stmt
}

pub fn isAssignment(mut stmt: Arc<NFStatement>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isFor(mut stmt: Arc<NFStatement>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ FOR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isReturn(mut stmt: Arc<NFStatement>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ RETURN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn makeIf(mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>>, mut src: Arc<DAE::ElementSource>) -> Arc<NFStatement> {
    let mut stmt: Arc<NFStatement>;
    stmt = Arc::new(NFStatement::IF { branches: branches.clone(), source: src.clone() });
    stmt
}

pub fn source(mut stmt: Arc<NFStatement>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource>;
    source = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => var_field!((*stmt).source, NFStatement::ASSIGNMENT).clone(),
        Deref @ FUNCTION_ARRAY_INIT { .. } => var_field!((*stmt).source, NFStatement::FUNCTION_ARRAY_INIT).clone(),
        Deref @ FOR { .. } => var_field!((*stmt).source, NFStatement::FOR).clone(),
        Deref @ IF { .. } => var_field!((*stmt).source, NFStatement::IF).clone(),
        Deref @ WHEN { .. } => var_field!((*stmt).source, NFStatement::WHEN).clone(),
        Deref @ ASSERT { .. } => var_field!((*stmt).source, NFStatement::ASSERT).clone(),
        Deref @ TERMINATE { .. } => var_field!((*stmt).source, NFStatement::TERMINATE).clone(),
        Deref @ REINIT { .. } => var_field!((*stmt).source, NFStatement::REINIT).clone(),
        Deref @ NORETCALL { .. } => var_field!((*stmt).source, NFStatement::NORETCALL).clone(),
        Deref @ WHILE { .. } => var_field!((*stmt).source, NFStatement::WHILE).clone(),
        Deref @ RETURN { .. } => var_field!((*stmt).source, NFStatement::RETURN).clone(),
        Deref @ BREAK { .. } => var_field!((*stmt).source, NFStatement::BREAK).clone(),
        Deref @ FAILURE { .. } => var_field!((*stmt).source, NFStatement::FAILURE).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn setSource(mut source: Arc<DAE::ElementSource>, mut stmt: Arc<NFStatement>) -> Result<Arc<NFStatement>> {
    let mut stmt: Arc<NFStatement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => {
            assign_variant_field!(stmt => NFStatement::ASSIGNMENT; source = source.clone());
            ()
        },
        Deref @ FUNCTION_ARRAY_INIT { .. } => {
            assign_variant_field!(stmt => NFStatement::FUNCTION_ARRAY_INIT; source = source.clone());
            ()
        },
        Deref @ FOR { .. } => {
            assign_variant_field!(stmt => NFStatement::FOR; source = source.clone());
            ()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(stmt => NFStatement::IF; source = source.clone());
            ()
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(stmt => NFStatement::WHEN; source = source.clone());
            ()
        },
        Deref @ ASSERT { .. } => {
            assign_variant_field!(stmt => NFStatement::ASSERT; source = source.clone());
            ()
        },
        Deref @ TERMINATE { .. } => {
            assign_variant_field!(stmt => NFStatement::TERMINATE; source = source.clone());
            ()
        },
        Deref @ NORETCALL { .. } => {
            assign_variant_field!(stmt => NFStatement::NORETCALL; source = source.clone());
            ()
        },
        Deref @ WHILE { .. } => {
            assign_variant_field!(stmt => NFStatement::WHILE; source = source.clone());
            ()
        },
        Deref @ RETURN { .. } => {
            assign_variant_field!(stmt => NFStatement::RETURN; source = source.clone());
            ()
        },
        Deref @ BREAK { .. } => {
            assign_variant_field!(stmt => NFStatement::BREAK; source = source.clone());
            ()
        },
        Deref @ FAILURE { .. } => {
            assign_variant_field!(stmt => NFStatement::FAILURE; source = source.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(stmt)
}

pub fn info(mut stmt: Arc<NFStatement>) -> SourceInfo {
    let mut info: SourceInfo = ElementSource::getInfo(source(stmt.clone()).unwrap());
    info
}

pub type ApplyFn = fn(Arc<NFStatement>) -> Result<()>;

pub fn apply(mut stmt: Arc<NFStatement>, mut func: ApplyFn) -> () {
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ FOR { .. } => {
            for mut e in &*var_field!((*stmt).body, NFStatement::FOR).clone() {
                let mut e = e.clone();
                apply(e.clone(), func);
            }
            ()
        },
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::IF).clone() {
                let mut b = b.clone();
                for mut e in &*Util::tuple22(b.clone()) {
                    let mut e = e.clone();
                    apply(e.clone(), func);
                }
            }
            ()
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::WHEN).clone() {
                let mut b = b.clone();
                for mut e in &*Util::tuple22(b.clone()) {
                    let mut e = e.clone();
                    apply(e.clone(), func);
                }
            }
            ()
        },
        Deref @ WHILE { .. } => {
            for mut e in &*var_field!((*stmt).body, NFStatement::WHILE).clone() {
                let mut e = e.clone();
                apply(e.clone(), func);
            }
            ()
        },
        Deref @ FAILURE { .. } => {
            for mut e in &*var_field!((*stmt).body, NFStatement::FAILURE).clone() {
                let mut e = e.clone();
                apply(e.clone(), func);
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    func(stmt.clone()).unwrap();
    ()
}

pub fn map(mut stmt: Arc<NFStatement>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFStatement>) -> Result<Arc<NFStatement>> + 'static>) -> Arc<NFStatement> {
    pub type MapFn = fn(Arc<NFStatement>) -> Result<Arc<NFStatement>>;

    let mut stmt: Arc<NFStatement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ FOR { .. } => {
            assign_variant_field!(stmt => NFStatement::FOR; body = {
        let mut __acc: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).body, NFStatement::FOR).clone()).into_iter().cloned() {
            let __x = map(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(stmt => NFStatement::IF; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, NFStatement::IF).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(b.clone()), {
        let mut __acc: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
        for mut s in (Util::tuple22(b.clone())).into_iter().cloned() {
            let __x = map(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(stmt => NFStatement::WHEN; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, NFStatement::WHEN).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(b.clone()), {
        let mut __acc: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
        for mut s in (Util::tuple22(b.clone())).into_iter().cloned() {
            let __x = map(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ WHILE { .. } => {
            assign_variant_field!(stmt => NFStatement::WHILE; body = {
        let mut __acc: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).body, NFStatement::WHILE).clone()).into_iter().cloned() {
            let __x = map(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stmt = func(stmt.clone()).unwrap();
    stmt
}

pub fn fold<ArgT: Clone + 'static>(mut stmt: Arc<NFStatement>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFStatement>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> ArgT {
    pub type MapFn<ArgT: Clone> = fn(Arc<NFStatement>, ArgT) -> Result<ArgT>;

    let mut arg: ArgT = arg;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ FOR { .. } => {
            for mut s in &*var_field!((*stmt).body, NFStatement::FOR).clone() {
                let mut s = s.clone();
                arg = fold(s.clone(), func.clone(), arg.clone());
            }
            ()
        },
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::IF).clone() {
                let mut b = b.clone();
                for mut s in &*Util::tuple22(b.clone()) {
                    let mut s = s.clone();
                    arg = fold(s.clone(), func.clone(), arg.clone());
                }
            }
            ()
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::WHEN).clone() {
                let mut b = b.clone();
                for mut s in &*Util::tuple22(b.clone()) {
                    let mut s = s.clone();
                    arg = fold(s.clone(), func.clone(), arg.clone());
                }
            }
            ()
        },
        Deref @ WHILE { .. } => {
            for mut s in &*var_field!((*stmt).body, NFStatement::WHILE).clone() {
                let mut s = s.clone();
                arg = fold(s.clone(), func.clone(), arg.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    arg = func(stmt.clone(), arg.clone()).unwrap();
    arg
}

pub fn applyExpList(mut stmt: Arc<metamodelica::List<Arc<NFStatement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type FoldFunc = fn(Arc<Expression::NFExpression>) -> Result<()>;

    for mut s in &*stmt.clone() {
        let mut s = s.clone();
        applyExp(s.clone(), func.clone())?;
    }
    Ok(())
}

pub fn applyExp(mut stmt: Arc<NFStatement>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<Expression::NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => {
            func(var_field!((*stmt).lhs, NFStatement::ASSIGNMENT).clone())?;
            func(var_field!((*stmt).rhs, NFStatement::ASSIGNMENT).clone())?;
            ()
        },
        Deref @ FOR { .. } => {
            applyExpList(var_field!((*stmt).body, NFStatement::FOR).clone(), func.clone())?;
            if isSome(var_field!((*stmt).range, NFStatement::FOR).clone()) {
                func(Util::getOption(var_field!((*stmt).range, NFStatement::FOR).clone())?)?;
            }
            ()
        },
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::IF).clone() {
                let mut b = b.clone();
                func(Util::tuple21(b.clone()))?;
                applyExpList(Util::tuple22(b.clone()), func.clone())?;
            }
            ()
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::WHEN).clone() {
                let mut b = b.clone();
                func(Util::tuple21(b.clone()))?;
                applyExpList(Util::tuple22(b.clone()), func.clone())?;
            }
            ()
        },
        Deref @ ASSERT { .. } => {
            func(var_field!((*stmt).condition, NFStatement::ASSERT).clone())?;
            func(var_field!((*stmt).message, NFStatement::ASSERT).clone())?;
            func(var_field!((*stmt).level, NFStatement::ASSERT).clone())?;
            ()
        },
        Deref @ TERMINATE { .. } => {
            func(var_field!((*stmt).message, NFStatement::TERMINATE).clone())?;
            ()
        },
        Deref @ REINIT { .. } => {
            func(var_field!((*stmt).cref, NFStatement::REINIT).clone())?;
            func(var_field!((*stmt).reinitExp, NFStatement::REINIT).clone())?;
            ()
        },
        Deref @ NORETCALL { .. } => {
            func(var_field!((*stmt).exp, NFStatement::NORETCALL).clone())?;
            ()
        },
        Deref @ WHILE { .. } => {
            func(var_field!((*stmt).condition, NFStatement::WHILE).clone())?;
            applyExpList(var_field!((*stmt).body, NFStatement::WHILE).clone(), func.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn mapExpList(mut stmtl: Arc<metamodelica::List<Arc<NFStatement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<metamodelica::List<Arc<NFStatement>>> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut stmtl: Arc<metamodelica::List<Arc<NFStatement>>> = stmtl;
    stmtl = {
        let mut __acc: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
        for mut s in (stmtl.clone()).into_iter().cloned() {
            let __x = mapExp(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    stmtl
}

pub fn mapExp(mut stmt: Arc<NFStatement>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFStatement> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut stmt: Arc<NFStatement> = stmt;
    stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).lhs, NFStatement::ASSIGNMENT).clone()).unwrap();
            e2 = func(var_field!((*stmt).rhs, NFStatement::ASSIGNMENT).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).lhs, NFStatement::ASSIGNMENT).clone()) && referenceEq(&e2.clone(),&var_field!((*stmt).rhs, NFStatement::ASSIGNMENT).clone())) {stmt.clone()} else {Arc::new(NFStatement::ASSIGNMENT { lhs: e1.clone(), rhs: e2.clone(), ty: var_field!((*stmt).ty, NFStatement::ASSIGNMENT).clone(), source: var_field!((*stmt).source, NFStatement::ASSIGNMENT).clone() })}
        },
        Deref @ FOR { .. } => {
            assign_variant_field!(stmt => NFStatement::FOR;
                body = mapExpList(var_field!((*stmt).body, NFStatement::FOR).clone(), func.clone()),
                range = Util::applyOption(var_field!((*stmt).range, NFStatement::FOR).clone(), func.clone())
            );
            stmt.clone()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(stmt => NFStatement::IF; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, NFStatement::IF).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(b.clone())).unwrap(), mapExpList(Util::tuple22(b.clone()), func.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            stmt.clone()
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(stmt => NFStatement::WHEN; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, NFStatement::WHEN).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(b.clone())).unwrap(), mapExpList(Util::tuple22(b.clone()), func.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            stmt.clone()
        },
        Deref @ ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).condition, NFStatement::ASSERT).clone()).unwrap();
            e2 = func(var_field!((*stmt).message, NFStatement::ASSERT).clone()).unwrap();
            e3 = func(var_field!((*stmt).level, NFStatement::ASSERT).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).condition, NFStatement::ASSERT).clone()) && referenceEq(&e2.clone(),&var_field!((*stmt).message, NFStatement::ASSERT).clone()) && referenceEq(&e3.clone(),&var_field!((*stmt).level, NFStatement::ASSERT).clone())) {stmt.clone()} else {Arc::new(NFStatement::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), source: var_field!((*stmt).source, NFStatement::ASSERT).clone() })}
        },
        Deref @ TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).message, NFStatement::TERMINATE).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).message, NFStatement::TERMINATE).clone())) {stmt.clone()} else {Arc::new(NFStatement::TERMINATE { message: e1.clone(), source: var_field!((*stmt).source, NFStatement::TERMINATE).clone() })}
        },
        Deref @ REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).cref, NFStatement::REINIT).clone()).unwrap();
            e2 = func(var_field!((*stmt).reinitExp, NFStatement::REINIT).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).cref, NFStatement::REINIT).clone()) && referenceEq(&e2.clone(),&var_field!((*stmt).reinitExp, NFStatement::REINIT).clone())) {stmt.clone()} else {Arc::new(NFStatement::REINIT { cref: e1.clone(), reinitExp: e2.clone(), source: var_field!((*stmt).source, NFStatement::REINIT).clone() })}
        },
        Deref @ NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).exp, NFStatement::NORETCALL).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).exp, NFStatement::NORETCALL).clone())) {stmt.clone()} else {Arc::new(NFStatement::NORETCALL { exp: e1.clone(), source: var_field!((*stmt).source, NFStatement::NORETCALL).clone() })}
        },
        Deref @ WHILE { .. } => {
            Arc::new(NFStatement::WHILE { condition: func(var_field!((*stmt).condition, NFStatement::WHILE).clone()).unwrap(), body: mapExpList(var_field!((*stmt).body, NFStatement::WHILE).clone(), func.clone()), source: var_field!((*stmt).source, NFStatement::WHILE).clone() })
        },
        _ => {
            stmt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stmt
}

pub fn mapExpShallow(mut stmt: Arc<NFStatement>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFStatement> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut stmt: Arc<NFStatement> = stmt;
    stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).lhs, NFStatement::ASSIGNMENT).clone()).unwrap();
            e2 = func(var_field!((*stmt).rhs, NFStatement::ASSIGNMENT).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).lhs, NFStatement::ASSIGNMENT).clone()) && referenceEq(&e2.clone(),&var_field!((*stmt).rhs, NFStatement::ASSIGNMENT).clone())) {stmt.clone()} else {Arc::new(NFStatement::ASSIGNMENT { lhs: e1.clone(), rhs: e2.clone(), ty: var_field!((*stmt).ty, NFStatement::ASSIGNMENT).clone(), source: var_field!((*stmt).source, NFStatement::ASSIGNMENT).clone() })}
        },
        Deref @ FOR { .. } => {
            assign_variant_field!(stmt => NFStatement::FOR; range = Util::applyOption(var_field!((*stmt).range, NFStatement::FOR).clone(), func.clone()));
            stmt.clone()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(stmt => NFStatement::IF; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, NFStatement::IF).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(b.clone())).unwrap(), Util::tuple22(b.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            stmt.clone()
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(stmt => NFStatement::WHEN; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, NFStatement::WHEN).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(b.clone())).unwrap(), Util::tuple22(b.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            stmt.clone()
        },
        Deref @ ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).condition, NFStatement::ASSERT).clone()).unwrap();
            e2 = func(var_field!((*stmt).message, NFStatement::ASSERT).clone()).unwrap();
            e3 = func(var_field!((*stmt).level, NFStatement::ASSERT).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).condition, NFStatement::ASSERT).clone()) && referenceEq(&e2.clone(),&var_field!((*stmt).message, NFStatement::ASSERT).clone()) && referenceEq(&e3.clone(),&var_field!((*stmt).level, NFStatement::ASSERT).clone())) {stmt.clone()} else {Arc::new(NFStatement::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), source: var_field!((*stmt).source, NFStatement::ASSERT).clone() })}
        },
        Deref @ TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).message, NFStatement::TERMINATE).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).message, NFStatement::TERMINATE).clone())) {stmt.clone()} else {Arc::new(NFStatement::TERMINATE { message: e1.clone(), source: var_field!((*stmt).source, NFStatement::TERMINATE).clone() })}
        },
        Deref @ REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).cref, NFStatement::REINIT).clone()).unwrap();
            e2 = func(var_field!((*stmt).reinitExp, NFStatement::REINIT).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).cref, NFStatement::REINIT).clone()) && referenceEq(&e2.clone(),&var_field!((*stmt).reinitExp, NFStatement::REINIT).clone())) {stmt.clone()} else {Arc::new(NFStatement::REINIT { cref: e1.clone(), reinitExp: e2.clone(), source: var_field!((*stmt).source, NFStatement::REINIT).clone() })}
        },
        Deref @ NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = func(var_field!((*stmt).exp, NFStatement::NORETCALL).clone()).unwrap();
            if (referenceEq(&e1.clone(),&var_field!((*stmt).exp, NFStatement::NORETCALL).clone())) {stmt.clone()} else {Arc::new(NFStatement::NORETCALL { exp: e1.clone(), source: var_field!((*stmt).source, NFStatement::NORETCALL).clone() })}
        },
        Deref @ WHILE { .. } => {
            Arc::new(NFStatement::WHILE { condition: func(var_field!((*stmt).condition, NFStatement::WHILE).clone()).unwrap(), body: var_field!((*stmt).body, NFStatement::WHILE).clone(), source: var_field!((*stmt).source, NFStatement::WHILE).clone() })
        },
        _ => {
            stmt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stmt
}

pub fn foldExpList<ArgT: Clone + 'static>(mut stmt: Arc<metamodelica::List<Arc<NFStatement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut arg: ArgT = arg;
    for mut s in &*stmt.clone() {
        let mut s = s.clone();
        arg = foldExp(s.clone(), func.clone(), arg.clone())?;
    }
    Ok(arg)
}

pub fn foldExp<ArgT: Clone + 'static>(mut stmt: Arc<NFStatement>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut arg: ArgT = arg;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => {
            arg = func(var_field!((*stmt).lhs, NFStatement::ASSIGNMENT).clone(), arg.clone())?;
            arg = func(var_field!((*stmt).rhs, NFStatement::ASSIGNMENT).clone(), arg.clone())?;
            ()
        },
        Deref @ FOR { .. } => {
            arg = foldExpList(var_field!((*stmt).body, NFStatement::FOR).clone(), func.clone(), arg.clone())?;
            if isSome(var_field!((*stmt).range, NFStatement::FOR).clone()) {
                arg = func(Util::getOption(var_field!((*stmt).range, NFStatement::FOR).clone())?, arg.clone())?;
            }
            ()
        },
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::IF).clone() {
                let mut b = b.clone();
                arg = func(Util::tuple21(b.clone()), arg.clone())?;
                arg = foldExpList(Util::tuple22(b.clone()), func.clone(), arg.clone())?;
            }
            ()
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::WHEN).clone() {
                let mut b = b.clone();
                arg = func(Util::tuple21(b.clone()), arg.clone())?;
                arg = foldExpList(Util::tuple22(b.clone()), func.clone(), arg.clone())?;
            }
            ()
        },
        Deref @ ASSERT { .. } => {
            arg = func(var_field!((*stmt).condition, NFStatement::ASSERT).clone(), arg.clone())?;
            arg = func(var_field!((*stmt).message, NFStatement::ASSERT).clone(), arg.clone())?;
            arg = func(var_field!((*stmt).level, NFStatement::ASSERT).clone(), arg.clone())?;
            ()
        },
        Deref @ TERMINATE { .. } => {
            arg = func(var_field!((*stmt).message, NFStatement::TERMINATE).clone(), arg.clone())?;
            ()
        },
        Deref @ REINIT { .. } => {
            arg = func(var_field!((*stmt).cref, NFStatement::REINIT).clone(), arg.clone())?;
            arg = func(var_field!((*stmt).reinitExp, NFStatement::REINIT).clone(), arg.clone())?;
            ()
        },
        Deref @ NORETCALL { .. } => {
            arg = func(var_field!((*stmt).exp, NFStatement::NORETCALL).clone(), arg.clone())?;
            ()
        },
        Deref @ WHILE { .. } => {
            arg = func(var_field!((*stmt).condition, NFStatement::WHILE).clone(), arg.clone())?;
            arg = foldExpList(var_field!((*stmt).body, NFStatement::WHILE).clone(), func.clone(), arg.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

pub fn contains(mut stmt: Arc<NFStatement>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<NFStatement>) -> Result<bool> + 'static>) -> bool {
    pub type PredFn = fn(Arc<NFStatement>) -> Result<bool>;

    let mut res: bool = false;
    if r#fn(stmt.clone()).unwrap() {
        res = true;
        return res;
    }
    res = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ FOR { .. } => containsList(var_field!((*stmt).body, NFStatement::FOR).clone(), r#fn.clone()),
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::IF).clone() {
                let mut b = b.clone();
                if containsList(Util::tuple22(b.clone()), r#fn.clone()) {
                    res = true;
                    return res;
                }
            }
            false
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*stmt).branches, NFStatement::WHEN).clone() {
                let mut b = b.clone();
                if containsList(Util::tuple22(b.clone()), r#fn.clone()) {
                    res = true;
                    return res;
                }
            }
            false
        },
        Deref @ WHILE { .. } => containsList(var_field!((*stmt).body, NFStatement::WHILE).clone(), r#fn.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn containsList(mut eql: Arc<metamodelica::List<Arc<NFStatement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFStatement>) -> Result<bool> + 'static>) -> bool {
    pub type PredFn = fn(Arc<NFStatement>) -> Result<bool>;

    let mut res: bool = false;
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        if contains(eq.clone(), func.clone()) {
            res = true;
            return res;
        }
    }
    res = false;
    res
}

pub fn replaceIteratorList(mut stmtl: Arc<metamodelica::List<Arc<NFStatement>>>, mut iterator: Arc<InstNode::InstNode>, mut value: Arc<Expression::NFExpression>) -> Arc<metamodelica::List<Arc<NFStatement>>> {
    let mut stmtl: Arc<metamodelica::List<Arc<NFStatement>>> = stmtl;
    stmtl = mapExpList(stmtl.clone(), Arc::new({ let __pe_b1 = iterator.clone(); let __pe_b2 = value.clone(); move |__pe_a0| Expression::replaceIterator(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    stmtl
}

pub fn toString(mut stmt: Arc<NFStatement>, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: IOStream::IOStream;
    s = IOStream::create((literal!("NFStatement.toString")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toStream(stmt.clone(), (indent.clone()).clone(), s.clone())?;
    r#str = (IOStream::string(s.clone())?).clone();
    IOStream::delete(s.clone())?;
    Ok(r#str)
}

pub fn toStringList(mut stmtl: Arc<metamodelica::List<Arc<NFStatement>>>, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: IOStream::IOStream;
    s = IOStream::create((literal!("NFStatement.toStringList")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toStreamList(stmtl.clone(), (indent.clone()).clone(), s.clone())?;
    r#str = (IOStream::string(s.clone())?).clone();
    IOStream::delete(s.clone())?;
    Ok(r#str)
}

pub fn toStream(mut stmt: Arc<NFStatement>, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
    let mut first: bool = false;
    s = IOStream::append(s.clone(), (indent.clone()).clone())?;
    s = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => {
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).lhs, NFStatement::ASSIGNMENT).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(" := ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).rhs, NFStatement::ASSIGNMENT).clone())?).clone())?;
            s.clone()
        },
        Deref @ FUNCTION_ARRAY_INIT { .. } => {
            s = IOStream::append(s.clone(), (literal!("array init")).clone())?;
            s = IOStream::append(s.clone(), (var_field!((*stmt).name, NFStatement::FUNCTION_ARRAY_INIT).clone()).clone())?;
            s.clone()
        },
        Deref @ FOR { .. } => {
            s = IOStream::append(s.clone(), (literal!("for ")).clone())?;
            s = IOStream::append(s.clone(), (InstNode::name(var_field!((*stmt).iterator, NFStatement::FOR).clone())?).clone())?;
            if isSome(var_field!((*stmt).range, NFStatement::FOR).clone()) {
                s = IOStream::append(s.clone(), (literal!(" in ")).clone())?;
                s = IOStream::append(s.clone(), (Expression::toString(Util::getOption(var_field!((*stmt).range, NFStatement::FOR).clone())?)?).clone())?;
            }
            s = IOStream::append(s.clone(), (literal!(" loop\n")).clone())?;
            s = toStreamList(var_field!((*stmt).body, NFStatement::FOR).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("end for")).clone())?;
            s.clone()
        },
        Deref @ IF { .. } => {
            first = true;
            branches = var_field!((*stmt).branches, NFStatement::IF).clone();
            while !(branches.clone().is_empty()) {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(branches.clone()) {
                    Deref @ metamodelica::List::Cons { head: (__pa0, __pa1), tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                cond = __pa0.clone();
                body = __pa1.clone();
                branches = __pa2.clone();
                if !(first.clone()) && branches.clone().is_empty() && Expression::isTrue(cond.clone()) {
                    s = IOStream::append(s.clone(), (literal!("else\n")).clone())?;
                } else {
                    s = IOStream::append(s.clone(), (if (first.clone()) {literal!("if ")} else {literal!("elseif ")}).clone())?;
                    s = IOStream::append(s.clone(), (Expression::toString(cond.clone())?).clone())?;
                    s = IOStream::append(s.clone(), (literal!(" then\n")).clone())?;
                }
                s = toStreamList(body.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                first = false;
            }
            s = IOStream::append(s.clone(), (literal!("end if")).clone())?;
            s.clone()
        },
        Deref @ WHEN { .. } => {
            first = true;
            for mut b in &*var_field!((*stmt).branches, NFStatement::WHEN).clone() {
                let mut b = b.clone();
                (cond, body) = b.clone();
                s = IOStream::append(s.clone(), (if (first.clone()) {literal!("when ")} else {literal!("elsewhen ")}).clone())?;
                s = IOStream::append(s.clone(), (Expression::toString(cond.clone())?).clone())?;
                s = IOStream::append(s.clone(), (literal!(" then\n")).clone())?;
                s = toStreamList(body.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                first = false;
            }
            s = IOStream::append(s.clone(), (literal!("end when")).clone())?;
            s.clone()
        },
        Deref @ ASSERT { .. } => {
            s = IOStream::append(s.clone(), (literal!("assert(")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).condition, NFStatement::ASSERT).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).message, NFStatement::ASSERT).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).level, NFStatement::ASSERT).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            s.clone()
        },
        Deref @ TERMINATE { .. } => {
            s = IOStream::append(s.clone(), (literal!("terminate(")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).message, NFStatement::TERMINATE).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            s.clone()
        },
        Deref @ REINIT { .. } => {
            s = IOStream::append(s.clone(), (literal!("reinit(")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).cref, NFStatement::REINIT).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).reinitExp, NFStatement::REINIT).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            s.clone()
        },
        Deref @ NORETCALL { .. } => IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).exp, NFStatement::NORETCALL).clone())?).clone())?,
        Deref @ WHILE { .. } => {
            s = IOStream::append(s.clone(), (literal!("while ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toString(var_field!((*stmt).condition, NFStatement::WHILE).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(" then\n")).clone())?;
            s = toStreamList(var_field!((*stmt).body, NFStatement::WHILE).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("end while")).clone())?;
            s.clone()
        },
        Deref @ RETURN { .. } => IOStream::append(s.clone(), (literal!("return")).clone())?,
        Deref @ RETURN { .. } => IOStream::append(s.clone(), (literal!("break")).clone())?,
        _ => IOStream::append(s.clone(), (literal!("#UNKNOWN STATEMENT#")).clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn toStreamList(mut stmtl: Arc<metamodelica::List<Arc<NFStatement>>>, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut prev_multi_line: bool = false;
    let mut multi_line: bool = false;
    let mut first: bool = true;
    for mut stmt in &*stmtl.clone() {
        let mut stmt = stmt.clone();
        multi_line = isMultiLine(stmt.clone());
        if first.clone() {
            first = false;
        } else if prev_multi_line.clone() || multi_line.clone() {
            s = IOStream::append(s.clone(), (literal!("\n")).clone())?;
        }
        prev_multi_line = multi_line.clone();
        s = toStream(stmt.clone(), (indent.clone()).clone(), s.clone())?;
        s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
    }
    Ok(s)
}

pub fn toFlatStream(mut stmt: Arc<NFStatement>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<NFStatement>>>)>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<NFStatement>>> = metamodelica::nil();
    let mut first: bool = false;
    s = IOStream::append(s.clone(), (indent.clone()).clone())?;
    s = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGNMENT { .. } => {
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).lhs, NFStatement::ASSIGNMENT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(" := ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).rhs, NFStatement::ASSIGNMENT).clone(), format.clone())?).clone())?;
            s.clone()
        },
        Deref @ FUNCTION_ARRAY_INIT { .. } => {
            s = IOStream::append(s.clone(), (literal!("array init")).clone())?;
            s = IOStream::append(s.clone(), (var_field!((*stmt).name, NFStatement::FUNCTION_ARRAY_INIT).clone()).clone())?;
            s.clone()
        },
        Deref @ FOR { .. } => {
            s = IOStream::append(s.clone(), (literal!("for ")).clone())?;
            s = IOStream::append(s.clone(), (Util::makeQuotedIdentifier((InstNode::name(var_field!((*stmt).iterator, NFStatement::FOR).clone())?).clone())?).clone())?;
            if isSome(var_field!((*stmt).range, NFStatement::FOR).clone()) {
                s = IOStream::append(s.clone(), (literal!(" in ")).clone())?;
                s = IOStream::append(s.clone(), (Expression::toFlatString(Util::getOption(var_field!((*stmt).range, NFStatement::FOR).clone())?, format.clone())?).clone())?;
            }
            s = IOStream::append(s.clone(), (literal!(" loop\n")).clone())?;
            s = toFlatStreamList(var_field!((*stmt).body, NFStatement::FOR).clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("end for")).clone())?;
            s.clone()
        },
        Deref @ IF { .. } => {
            first = true;
            branches = var_field!((*stmt).branches, NFStatement::IF).clone();
            while !(branches.clone().is_empty()) {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(branches.clone()) {
                    Deref @ metamodelica::List::Cons { head: (__pa0, __pa1), tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                cond = __pa0.clone();
                body = __pa1.clone();
                branches = __pa2.clone();
                if !(first.clone()) && branches.clone().is_empty() && Expression::isTrue(cond.clone()) {
                    s = IOStream::append(s.clone(), (literal!("else\n")).clone())?;
                } else {
                    s = IOStream::append(s.clone(), (if (first.clone()) {literal!("if ")} else {literal!("elseif ")}).clone())?;
                    s = IOStream::append(s.clone(), (Expression::toFlatString(cond.clone(), format.clone())?).clone())?;
                    s = IOStream::append(s.clone(), (literal!(" then\n")).clone())?;
                }
                s = toFlatStreamList(body.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                first = false;
            }
            s = IOStream::append(s.clone(), (literal!("end if")).clone())?;
            s.clone()
        },
        Deref @ WHEN { .. } => {
            first = true;
            for mut b in &*var_field!((*stmt).branches, NFStatement::WHEN).clone() {
                let mut b = b.clone();
                (cond, body) = b.clone();
                s = IOStream::append(s.clone(), (if (first.clone()) {literal!("when ")} else {literal!("elsewhen ")}).clone())?;
                s = IOStream::append(s.clone(), (Expression::toFlatString(cond.clone(), format.clone())?).clone())?;
                s = IOStream::append(s.clone(), (literal!(" then\n")).clone())?;
                s = toFlatStreamList(body.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                first = false;
            }
            s = IOStream::append(s.clone(), (literal!("end when")).clone())?;
            s.clone()
        },
        Deref @ ASSERT { .. } => {
            s = IOStream::append(s.clone(), (literal!("assert(")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).condition, NFStatement::ASSERT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).message, NFStatement::ASSERT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).level, NFStatement::ASSERT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            s.clone()
        },
        Deref @ TERMINATE { .. } => {
            s = IOStream::append(s.clone(), (literal!("terminate(")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).message, NFStatement::TERMINATE).clone(), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            s.clone()
        },
        Deref @ REINIT { .. } => {
            s = IOStream::append(s.clone(), (literal!("reinit(")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).cref, NFStatement::REINIT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).reinitExp, NFStatement::REINIT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            s.clone()
        },
        Deref @ NORETCALL { .. } => IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).exp, NFStatement::NORETCALL).clone(), format.clone())?).clone())?,
        Deref @ WHILE { .. } => {
            s = IOStream::append(s.clone(), (literal!("while ")).clone())?;
            s = IOStream::append(s.clone(), (Expression::toFlatString(var_field!((*stmt).condition, NFStatement::WHILE).clone(), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(" loop\n")).clone())?;
            s = toFlatStreamList(var_field!((*stmt).body, NFStatement::WHILE).clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("end while")).clone())?;
            s.clone()
        },
        Deref @ RETURN { .. } => IOStream::append(s.clone(), (literal!("return")).clone())?,
        Deref @ BREAK { .. } => IOStream::append(s.clone(), (literal!("break")).clone())?,
        _ => IOStream::append(s.clone(), (literal!("#UNKNOWN STATEMENT#")).clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    s = FlatModelicaUtil::appendElementSourceComment(source(stmt.clone())?, FlatModelicaUtil::ElementType::ALGORITHM.clone(), s.clone())?;
    Ok(s)
}

pub fn toFlatStreamList(mut stmtl: Arc<metamodelica::List<Arc<NFStatement>>>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut prev_multi_line: bool = false;
    let mut multi_line: bool = false;
    let mut first: bool = true;
    for mut stmt in &*stmtl.clone() {
        let mut stmt = stmt.clone();
        multi_line = isMultiLine(stmt.clone());
        if first.clone() {
            first = false;
        } else if prev_multi_line.clone() || multi_line.clone() {
            s = IOStream::append(s.clone(), (literal!("\n")).clone())?;
        }
        prev_multi_line = multi_line.clone();
        s = toFlatStream(stmt.clone(), format.clone(), (indent.clone()).clone(), s.clone())?;
        s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
    }
    Ok(s)
}

pub fn isMultiLine(mut stmt: Arc<NFStatement>) -> bool {
    let mut multiLine: bool = false;
    multiLine = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ FOR { .. } => true,
        Deref @ IF { .. } => true,
        Deref @ WHEN { .. } => true,
        Deref @ WHILE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    multiLine
}


