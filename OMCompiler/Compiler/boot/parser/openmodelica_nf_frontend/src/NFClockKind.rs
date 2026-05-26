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
use crate::NFExpression as Expression;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::JSON;
use openmodelica_util::Util;

#[derive(Clone, Debug, PartialEq)]
pub enum NFClockKind {
    /// Clock()
    INFERRED_CLOCK {
        /// unique index to correctly associate equal inferred clocks
        idx: i32,
    },
    RATIONAL_CLOCK {
        /// integer type >= 0
        intervalCounter: Arc<Expression::NFExpression>,
        /// integer type >= 1, defaults to 1
        resolution: Arc<Expression::NFExpression>,
    },
    REAL_CLOCK {
        /// real type > 0
        interval: Arc<Expression::NFExpression>,
    },
    EVENT_CLOCK {
        /// boolean type
        condition: Arc<Expression::NFExpression>,
        /// real type >= 0.0
        startInterval: Arc<Expression::NFExpression>,
    },
    SOLVER_CLOCK {
        /// clock type
        c: Arc<Expression::NFExpression>,
        /// string type
        solverMethod: Arc<Expression::NFExpression>,
    },
}
impl Default for NFClockKind {
    fn default() -> Self {
        Self::INFERRED_CLOCK {
            idx: Default::default(),
        }
    }
}
pub use self::NFClockKind::{INFERRED_CLOCK,RATIONAL_CLOCK,REAL_CLOCK,EVENT_CLOCK,SOLVER_CLOCK};
pub fn isInferred(mut ck: Arc<NFClockKind>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ INFERRED_CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn compare(mut ck1: Arc<NFClockKind>, mut ck2: Arc<NFClockKind>) -> Result<i32> {
    fn compareInt(mut kind: Arc<NFClockKind>) -> i32 {
        let mut i: i32 = 0;
        i = (::match_deref::match_deref! { match &(kind.clone()) {
        Deref @ INFERRED_CLOCK { .. } => 0,
        Deref @ RATIONAL_CLOCK { .. } => 1,
        Deref @ REAL_CLOCK { .. } => 2,
        Deref @ EVENT_CLOCK { .. } => 3,
        Deref @ SOLVER_CLOCK { .. } => 4,
        _ => 5,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        i
    }

    let mut comp: i32 = 0;
    comp = (::match_deref::match_deref! { match &((ck1.clone(), ck2.clone())) {
        (Deref @ INFERRED_CLOCK { .. }, Deref @ INFERRED_CLOCK { .. }) => {
            Util::intCompare(var_field!((*ck1).idx, NFClockKind::INFERRED_CLOCK).clone(), var_field!((*ck2).idx, NFClockKind::INFERRED_CLOCK).clone())
        },
        (Deref @ RATIONAL_CLOCK { intervalCounter: i1, resolution: r1 }, Deref @ RATIONAL_CLOCK { intervalCounter: i2, resolution: r2 }) => {
            comp = Expression::compare(i1.clone(), i2.clone())?;
            if comp.clone() == 0 {
                comp = Expression::compare(r1.clone(), r2.clone())?;
            }
            comp.clone()
        },
        (Deref @ REAL_CLOCK { interval: i1 }, Deref @ REAL_CLOCK { interval: i2 }) => {
            Expression::compare(i1.clone(), i2.clone())?
        },
        (Deref @ EVENT_CLOCK { condition: c1, startInterval: si1 }, Deref @ EVENT_CLOCK { condition: c2, startInterval: si2 }) => {
            comp = Expression::compare(c1.clone(), c2.clone())?;
            if comp.clone() == 0 {
                comp = Expression::compare(si1.clone(), si2.clone())?;
            }
            comp.clone()
        },
        (Deref @ SOLVER_CLOCK { c: c1, solverMethod: sm2 }, Deref @ SOLVER_CLOCK { c: c2, solverMethod: sm1 }) => {
            comp = Expression::compare(c1.clone(), c2.clone())?;
            if comp.clone() == 0 {
                comp = Expression::compare(sm1.clone(), sm2.clone())?;
            }
            comp.clone()
        },
        _ => {
            if (compareInt(ck1.clone()) < compareInt(ck2.clone())) {-1} else {1}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub fn containsExp(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<Expression::NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { .. } => Expression::contains(var_field!((*ck).intervalCounter, NFClockKind::RATIONAL_CLOCK).clone(), func.clone())? || Expression::contains(var_field!((*ck).resolution, NFClockKind::RATIONAL_CLOCK).clone(), func.clone())?,
        Deref @ REAL_CLOCK { .. } => Expression::contains(var_field!((*ck).interval, NFClockKind::REAL_CLOCK).clone(), func.clone())?,
        Deref @ EVENT_CLOCK { .. } => Expression::contains(var_field!((*ck).condition, NFClockKind::EVENT_CLOCK).clone(), func.clone())? || Expression::contains(var_field!((*ck).startInterval, NFClockKind::EVENT_CLOCK).clone(), func.clone())?,
        Deref @ SOLVER_CLOCK { .. } => Expression::contains(var_field!((*ck).c, NFClockKind::SOLVER_CLOCK).clone(), func.clone())? || Expression::contains(var_field!((*ck).solverMethod, NFClockKind::SOLVER_CLOCK).clone(), func.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn containsExpShallow(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> bool {
    pub type ContainsPred = fn(Arc<Expression::NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { .. } => func(var_field!((*ck).intervalCounter, NFClockKind::RATIONAL_CLOCK).clone()).unwrap() || func(var_field!((*ck).resolution, NFClockKind::RATIONAL_CLOCK).clone()).unwrap(),
        Deref @ REAL_CLOCK { .. } => func(var_field!((*ck).interval, NFClockKind::REAL_CLOCK).clone()).unwrap(),
        Deref @ EVENT_CLOCK { .. } => func(var_field!((*ck).condition, NFClockKind::EVENT_CLOCK).clone()).unwrap() || func(var_field!((*ck).startInterval, NFClockKind::EVENT_CLOCK).clone()).unwrap(),
        Deref @ SOLVER_CLOCK { .. } => func(var_field!((*ck).c, NFClockKind::SOLVER_CLOCK).clone()).unwrap() || func(var_field!((*ck).solverMethod, NFClockKind::SOLVER_CLOCK).clone()).unwrap(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn applyExp(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<Expression::NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { .. } => {
            Expression::apply(var_field!((*ck).intervalCounter, NFClockKind::RATIONAL_CLOCK).clone(), func.clone())?;
            Expression::apply(var_field!((*ck).resolution, NFClockKind::RATIONAL_CLOCK).clone(), func.clone())?;
            ()
        },
        Deref @ REAL_CLOCK { .. } => {
            Expression::apply(var_field!((*ck).interval, NFClockKind::REAL_CLOCK).clone(), func.clone())?;
            ()
        },
        Deref @ EVENT_CLOCK { .. } => {
            Expression::apply(var_field!((*ck).condition, NFClockKind::EVENT_CLOCK).clone(), func.clone())?;
            Expression::apply(var_field!((*ck).startInterval, NFClockKind::EVENT_CLOCK).clone(), func.clone())?;
            ()
        },
        Deref @ SOLVER_CLOCK { .. } => {
            Expression::apply(var_field!((*ck).c, NFClockKind::SOLVER_CLOCK).clone(), func.clone())?;
            Expression::apply(var_field!((*ck).solverMethod, NFClockKind::SOLVER_CLOCK).clone(), func.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn applyExpShallow(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> () {
    pub type ApplyFunc = fn(Arc<Expression::NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { .. } => {
            func(var_field!((*ck).intervalCounter, NFClockKind::RATIONAL_CLOCK).clone()).unwrap();
            func(var_field!((*ck).resolution, NFClockKind::RATIONAL_CLOCK).clone()).unwrap();
            ()
        },
        Deref @ REAL_CLOCK { .. } => {
            func(var_field!((*ck).interval, NFClockKind::REAL_CLOCK).clone()).unwrap();
            ()
        },
        Deref @ EVENT_CLOCK { .. } => {
            func(var_field!((*ck).condition, NFClockKind::EVENT_CLOCK).clone()).unwrap();
            func(var_field!((*ck).startInterval, NFClockKind::EVENT_CLOCK).clone()).unwrap();
            ()
        },
        Deref @ SOLVER_CLOCK { .. } => {
            func(var_field!((*ck).c, NFClockKind::SOLVER_CLOCK).clone()).unwrap();
            func(var_field!((*ck).solverMethod, NFClockKind::SOLVER_CLOCK).clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn foldExp<ArgT: Clone + 'static>(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut result: ArgT;
    result = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { .. } => {
            result = Expression::fold(var_field!((*ck).intervalCounter, NFClockKind::RATIONAL_CLOCK).clone(), func.clone(), arg.clone())?;
            Expression::fold(var_field!((*ck).resolution, NFClockKind::RATIONAL_CLOCK).clone(), func.clone(), result.clone())?
        },
        Deref @ REAL_CLOCK { .. } => Expression::fold(var_field!((*ck).interval, NFClockKind::REAL_CLOCK).clone(), func.clone(), arg.clone())?,
        Deref @ EVENT_CLOCK { .. } => {
            result = Expression::fold(var_field!((*ck).condition, NFClockKind::EVENT_CLOCK).clone(), func.clone(), arg.clone())?;
            Expression::fold(var_field!((*ck).startInterval, NFClockKind::EVENT_CLOCK).clone(), func.clone(), result.clone())?
        },
        Deref @ SOLVER_CLOCK { .. } => {
            result = Expression::fold(var_field!((*ck).c, NFClockKind::SOLVER_CLOCK).clone(), func.clone(), arg.clone())?;
            Expression::fold(var_field!((*ck).solverMethod, NFClockKind::SOLVER_CLOCK).clone(), func.clone(), result.clone())?
        },
        _ => arg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn mapExp(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFClockKind>> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outCk: Arc<NFClockKind>;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e4: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outCk = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { intervalCounter: e1, resolution: e2 } => {
            e3 = Expression::map(e1.clone(), func.clone())?;
            e4 = Expression::map(e2.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::RATIONAL_CLOCK { intervalCounter: e3.clone(), resolution: e4.clone() })}
        },
        Deref @ REAL_CLOCK { interval: e1 } => {
            e3 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone())) {ck.clone()} else {Arc::new(NFClockKind::REAL_CLOCK { interval: e3.clone() })}
        },
        Deref @ EVENT_CLOCK { condition: e1, startInterval: e2 } => {
            e3 = Expression::map(e1.clone(), func.clone())?;
            e4 = Expression::map(e2.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::EVENT_CLOCK { condition: e3.clone(), startInterval: e4.clone() })}
        },
        Deref @ SOLVER_CLOCK { c: e1, solverMethod: e2 } => {
            e3 = Expression::map(e1.clone(), func.clone())?;
            e4 = Expression::map(e2.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::SOLVER_CLOCK { c: e3.clone(), solverMethod: e4.clone() })}
        },
        _ => ck.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCk)
}

pub fn mapExpShallow(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFClockKind> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outCk: Arc<NFClockKind>;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e4: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outCk = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { intervalCounter: e1, resolution: e2 } => {
            e3 = func(e1.clone()).unwrap();
            e4 = func(e2.clone()).unwrap();
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::RATIONAL_CLOCK { intervalCounter: e3.clone(), resolution: e4.clone() })}
        },
        Deref @ REAL_CLOCK { interval: e1 } => {
            e3 = func(e1.clone()).unwrap();
            if (referenceEq(&e1.clone(),&e3.clone())) {ck.clone()} else {Arc::new(NFClockKind::REAL_CLOCK { interval: e3.clone() })}
        },
        Deref @ EVENT_CLOCK { condition: e1, startInterval: e2 } => {
            e3 = func(e1.clone()).unwrap();
            e4 = func(e2.clone()).unwrap();
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::EVENT_CLOCK { condition: e3.clone(), startInterval: e4.clone() })}
        },
        Deref @ SOLVER_CLOCK { c: e1, solverMethod: e2 } => {
            e3 = func(e1.clone()).unwrap();
            e4 = func(e2.clone()).unwrap();
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::SOLVER_CLOCK { c: e3.clone(), solverMethod: e4.clone() })}
        },
        _ => ck.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCk
}

pub fn mapFoldExp<ArgT: Clone + 'static>(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<NFClockKind>, ArgT)> {
    pub type MapFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)>;

    let mut outCk: Arc<NFClockKind>;
    let mut arg: ArgT = arg;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e4: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outCk = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { intervalCounter: e1, resolution: e2 } => {
            (e3, arg) = Expression::mapFold(e1.clone(), func.clone(), arg.clone())?;
            (e4, arg) = Expression::mapFold(e2.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::RATIONAL_CLOCK { intervalCounter: e3.clone(), resolution: e4.clone() })}
        },
        Deref @ REAL_CLOCK { interval: e1 } => {
            (e3, arg) = Expression::mapFold(e1.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone())) {ck.clone()} else {Arc::new(NFClockKind::REAL_CLOCK { interval: e3.clone() })}
        },
        Deref @ EVENT_CLOCK { condition: e1, startInterval: e2 } => {
            (e3, arg) = Expression::mapFold(e1.clone(), func.clone(), arg.clone())?;
            (e4, arg) = Expression::mapFold(e2.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::EVENT_CLOCK { condition: e3.clone(), startInterval: e4.clone() })}
        },
        Deref @ SOLVER_CLOCK { c: e1, solverMethod: e2 } => {
            (e3, arg) = Expression::mapFold(e1.clone(), func.clone(), arg.clone())?;
            (e4, arg) = Expression::mapFold(e2.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::SOLVER_CLOCK { c: e3.clone(), solverMethod: e4.clone() })}
        },
        _ => ck.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCk, arg))
}

pub fn mapFoldExpShallow<ArgT: Clone + 'static>(mut ck: Arc<NFClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<NFClockKind>, ArgT)> {
    pub type MapFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)>;

    let mut outCk: Arc<NFClockKind>;
    let mut arg: ArgT = arg;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e4: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outCk = (::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ RATIONAL_CLOCK { intervalCounter: e1, resolution: e2 } => {
            (e3, arg) = Expression::mapFoldShallow(e1.clone(), func.clone(), arg.clone())?;
            (e4, arg) = Expression::mapFoldShallow(e2.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::RATIONAL_CLOCK { intervalCounter: e3.clone(), resolution: e4.clone() })}
        },
        Deref @ REAL_CLOCK { interval: e1 } => {
            (e3, arg) = Expression::mapFoldShallow(e1.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone())) {ck.clone()} else {Arc::new(NFClockKind::REAL_CLOCK { interval: e3.clone() })}
        },
        Deref @ EVENT_CLOCK { condition: e1, startInterval: e2 } => {
            (e3, arg) = Expression::mapFoldShallow(e1.clone(), func.clone(), arg.clone())?;
            (e4, arg) = Expression::mapFoldShallow(e2.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::EVENT_CLOCK { condition: e3.clone(), startInterval: e4.clone() })}
        },
        Deref @ SOLVER_CLOCK { c: e1, solverMethod: e2 } => {
            (e3, arg) = Expression::mapFoldShallow(e1.clone(), func.clone(), arg.clone())?;
            (e4, arg) = Expression::mapFoldShallow(e2.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&e3.clone()) && referenceEq(&e2.clone(),&e4.clone())) {ck.clone()} else {Arc::new(NFClockKind::SOLVER_CLOCK { c: e3.clone(), solverMethod: e4.clone() })}
        },
        _ => ck.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCk, arg))
}

pub fn toAbsyn(mut clk: Arc<NFClockKind>) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    args = (::match_deref::match_deref! { match &(clk.clone()) {
        Deref @ INFERRED_CLOCK { .. } => metamodelica::nil(),
        Deref @ RATIONAL_CLOCK { .. } => list![Expression::toAbsyn(var_field!((*clk).intervalCounter, NFClockKind::RATIONAL_CLOCK).clone())?, Expression::toAbsyn(var_field!((*clk).resolution, NFClockKind::RATIONAL_CLOCK).clone())?],
        Deref @ REAL_CLOCK { .. } => list![Expression::toAbsyn(var_field!((*clk).interval, NFClockKind::REAL_CLOCK).clone())?],
        Deref @ EVENT_CLOCK { .. } => list![Expression::toAbsyn(var_field!((*clk).condition, NFClockKind::EVENT_CLOCK).clone())?, Expression::toAbsyn(var_field!((*clk).startInterval, NFClockKind::EVENT_CLOCK).clone())?],
        Deref @ SOLVER_CLOCK { .. } => list![Expression::toAbsyn(var_field!((*clk).c, NFClockKind::SOLVER_CLOCK).clone())?, Expression::toAbsyn(var_field!((*clk).solverMethod, NFClockKind::SOLVER_CLOCK).clone())?],
        _ => bail!("match: no arm matched"),
    } });
    exp = AbsynUtil::makeCall(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("Clock")).clone(), subscripts: metamodelica::nil() }), args.clone(), metamodelica::nil());
    Ok(exp)
}

pub fn toDAE(mut ick: Arc<NFClockKind>) -> Result<Arc<DAE::ClockKind>> {
    let mut ock: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    ock = (::match_deref::match_deref! { match &(ick.clone()) {
        Deref @ INFERRED_CLOCK { .. } => {
            Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK)
        },
        Deref @ RATIONAL_CLOCK { intervalCounter: i, resolution: r } => {
            Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Expression::toDAE(i.clone(), false)?, resolution: Expression::toDAE(r.clone(), false)? })
        },
        Deref @ REAL_CLOCK { interval: i } => {
            Arc::new(DAE::ClockKind::REAL_CLOCK { interval: Expression::toDAE(i.clone(), false)? })
        },
        Deref @ EVENT_CLOCK { condition: c, startInterval: si } => {
            Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: Expression::toDAE(c.clone(), false)?, startInterval: Expression::toDAE(si.clone(), false)? })
        },
        Deref @ SOLVER_CLOCK { c, solverMethod: sm } => {
            Arc::new(DAE::ClockKind::SOLVER_CLOCK { c: Expression::toDAE(c.clone(), false)?, solverMethod: Expression::toDAE(sm.clone(), false)? })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ock)
}

pub fn toDebugString(mut ick: Arc<NFClockKind>) -> Result<ArcStr> {
    let mut ock: ArcStr = arcstr::literal!("");
    ock = ((::match_deref::match_deref! { match &(ick.clone()) {
        Deref @ INFERRED_CLOCK { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("INFERRED_CLOCK(")); __mm_s.push_str(&*intString(var_field!((*ick).idx, NFClockKind::INFERRED_CLOCK).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ RATIONAL_CLOCK { intervalCounter: i, resolution: r } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("RATIONAL_CLOCK(")); __mm_s.push_str(&*Expression::toString(i.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(r.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ REAL_CLOCK { interval: i } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("REAL_CLOCK(")); __mm_s.push_str(&*Expression::toString(i.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ EVENT_CLOCK { condition: c, startInterval: si } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EVENT_CLOCK(")); __mm_s.push_str(&*Expression::toString(c.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(si.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ SOLVER_CLOCK { c, solverMethod: sm } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SOLVER_CLOCK(")); __mm_s.push_str(&*Expression::toString(c.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(sm.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(ock)
}

pub fn toString(mut ck: Arc<NFClockKind>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ INFERRED_CLOCK { .. } => {
            literal!("")
        },
        Deref @ RATIONAL_CLOCK { intervalCounter: e1, resolution: e2 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(e2.clone())?); ArcStr::from(__mm_s) }
        },
        Deref @ REAL_CLOCK { interval: e1 } => {
            Expression::toString(e1.clone())?
        },
        Deref @ EVENT_CLOCK { condition: e1, startInterval: e2 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(e2.clone())?); ArcStr::from(__mm_s) }
        },
        Deref @ SOLVER_CLOCK { c: e1, solverMethod: e2 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(e2.clone())?); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Clock(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn toFlatString(mut ck: Arc<NFClockKind>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(ck.clone()) {
        Deref @ INFERRED_CLOCK { .. } => {
            literal!("")
        },
        Deref @ RATIONAL_CLOCK { intervalCounter: e1, resolution: e2 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toFlatString(e1.clone(), format.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toFlatString(e2.clone(), format.clone())?); ArcStr::from(__mm_s) }
        },
        Deref @ REAL_CLOCK { interval: e1 } => {
            Expression::toFlatString(e1.clone(), format.clone())?
        },
        Deref @ EVENT_CLOCK { condition: e1, startInterval: e2 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toFlatString(e1.clone(), format.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toFlatString(e2.clone(), format.clone())?); ArcStr::from(__mm_s) }
        },
        Deref @ SOLVER_CLOCK { c: e1, solverMethod: e2 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toFlatString(e1.clone(), format.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toFlatString(e2.clone(), format.clone())?); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Clock(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn toJSON(mut clk: Arc<NFClockKind>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::emptyListObject();
    json = JSON::addPair((literal!("kind")).clone(), JSON::makeString((literal!("clock")).clone()), json.clone())?;
    let () = (::match_deref::match_deref! { match &(clk.clone()) {
        Deref @ INFERRED_CLOCK { .. } => {
            json = JSON::addPair((literal!("type")).clone(), JSON::makeString((literal!("inferred")).clone()), json.clone())?;
            ()
        },
        Deref @ RATIONAL_CLOCK { .. } => {
            json = JSON::addPair((literal!("type")).clone(), JSON::makeString((literal!("rational")).clone()), json.clone())?;
            json = JSON::addPair((literal!("intervalCounter")).clone(), Expression::toJSON(var_field!((*clk).intervalCounter, NFClockKind::RATIONAL_CLOCK).clone())?, json.clone())?;
            json = JSON::addPair((literal!("resolution")).clone(), Expression::toJSON(var_field!((*clk).resolution, NFClockKind::RATIONAL_CLOCK).clone())?, json.clone())?;
            ()
        },
        Deref @ REAL_CLOCK { .. } => {
            json = JSON::addPair((literal!("type")).clone(), JSON::makeString((literal!("real")).clone()), json.clone())?;
            json = JSON::addPair((literal!("interval")).clone(), Expression::toJSON(var_field!((*clk).interval, NFClockKind::REAL_CLOCK).clone())?, json.clone())?;
            ()
        },
        Deref @ EVENT_CLOCK { .. } => {
            json = JSON::addPair((literal!("type")).clone(), JSON::makeString((literal!("event")).clone()), json.clone())?;
            json = JSON::addPair((literal!("condition")).clone(), Expression::toJSON(var_field!((*clk).condition, NFClockKind::EVENT_CLOCK).clone())?, json.clone())?;
            json = JSON::addPair((literal!("startInterval")).clone(), Expression::toJSON(var_field!((*clk).startInterval, NFClockKind::EVENT_CLOCK).clone())?, json.clone())?;
            ()
        },
        Deref @ SOLVER_CLOCK { .. } => {
            json = JSON::addPair((literal!("type")).clone(), JSON::makeString((literal!("solver")).clone()), json.clone())?;
            json = JSON::addPair((literal!("c")).clone(), Expression::toJSON(var_field!((*clk).c, NFClockKind::SOLVER_CLOCK).clone())?, json.clone())?;
            json = JSON::addPair((literal!("solverMethod")).clone(), Expression::toJSON(var_field!((*clk).solverMethod, NFClockKind::SOLVER_CLOCK).clone())?, json.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(json)
}

pub fn hashContinue(mut clk: Arc<NFClockKind>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = stringHashDjb2Continue((literal!("Clock(")).clone(), hash.clone());
    hash = (::match_deref::match_deref! { match &(clk.clone()) {
        Deref @ INFERRED_CLOCK { .. } => hash.clone() + var_field!((*clk).idx, NFClockKind::INFERRED_CLOCK).clone(),
        Deref @ RATIONAL_CLOCK { .. } => {
            hash = Expression::hashContinue(var_field!((*clk).intervalCounter, NFClockKind::RATIONAL_CLOCK).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            hash = Expression::hashContinue(var_field!((*clk).resolution, NFClockKind::RATIONAL_CLOCK).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ REAL_CLOCK { .. } => Expression::hashContinue(var_field!((*clk).interval, NFClockKind::REAL_CLOCK).clone(), hash.clone())?,
        Deref @ EVENT_CLOCK { .. } => {
            hash = Expression::hashContinue(var_field!((*clk).condition, NFClockKind::EVENT_CLOCK).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            hash = Expression::hashContinue(var_field!((*clk).startInterval, NFClockKind::EVENT_CLOCK).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ SOLVER_CLOCK { .. } => {
            hash = Expression::hashContinue(var_field!((*clk).c, NFClockKind::SOLVER_CLOCK).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            hash = Expression::hashContinue(var_field!((*clk).solverMethod, NFClockKind::SOLVER_CLOCK).clone(), hash.clone())?;
            hash.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
    Ok(hash)
}


