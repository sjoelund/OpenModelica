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

use crate::BackendDAE as OldBackendDAE;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NSimCode::Identifier;
use crate::SimCode as OldSimCode;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NSimGenericCall.mo
/// package:     NSimGenericCall
/// description: This file contains the data types and functions for generic for loop calls.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSimGenericCall {
    SINGLE_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<Arc<SimIterator::SimIterator>>>,
        lhs: Arc<Expression::NFExpression>,
        rhs: Arc<Expression::NFExpression>,
        resizable: bool,
    },
    IF_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<Arc<SimIterator::SimIterator>>>,
        branches: Arc<metamodelica::List<Arc<SimBranch::SimBranch>>>,
        resizable: bool,
    },
    WHEN_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<Arc<SimIterator::SimIterator>>>,
        branches: Arc<metamodelica::List<Arc<SimBranch::SimBranch>>>,
        resizable: bool,
    },
}
impl Default for NSimGenericCall {
    fn default() -> Self {
        Self::IF_GENERIC_CALL {
            index: Default::default(),
            iters: Default::default(),
            branches: Default::default(),
            resizable: Default::default(),
        }
    }
}
pub use self::NSimGenericCall::{SINGLE_GENERIC_CALL,IF_GENERIC_CALL,WHEN_GENERIC_CALL};
pub fn mapShallow(mut call: Arc<NSimGenericCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NSimGenericCall> {
    pub type mapExp = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut call: Arc<NSimGenericCall> = call;
    call = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ SINGLE_GENERIC_CALL { .. } => {
            assign_variant_field!(call => NSimGenericCall::SINGLE_GENERIC_CALL;
                lhs = func(var_field!((*call).lhs, NSimGenericCall::SINGLE_GENERIC_CALL).clone()).unwrap(),
                rhs = func(var_field!((*call).rhs, NSimGenericCall::SINGLE_GENERIC_CALL).clone()).unwrap()
            );
            call.clone()
        },
        Deref @ IF_GENERIC_CALL { .. } => {
            assign_variant_field!(call => NSimGenericCall::IF_GENERIC_CALL; branches = {
        let mut __acc: Arc<metamodelica::List<Arc<SimBranch::SimBranch>>> = metamodelica::nil();
        for mut branch in (var_field!((*call).branches, NSimGenericCall::IF_GENERIC_CALL).clone()).into_iter().cloned() {
            let __x = SimBranch::mapShallow(branch.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            call.clone()
        },
        Deref @ WHEN_GENERIC_CALL { .. } => {
            assign_variant_field!(call => NSimGenericCall::WHEN_GENERIC_CALL; branches = {
        let mut __acc: Arc<metamodelica::List<Arc<SimBranch::SimBranch>>> = metamodelica::nil();
        for mut branch in (var_field!((*call).branches, NSimGenericCall::WHEN_GENERIC_CALL).clone()).into_iter().cloned() {
            let __x = SimBranch::mapShallow(branch.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            call.clone()
        },
        _ => call.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    call
}

pub fn toString(mut call: Arc<NSimGenericCall>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(call.clone()) {
        Deref @ SINGLE_GENERIC_CALL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*call).index, NSimGenericCall::SINGLE_GENERIC_CALL).clone())); __mm_s.push_str(&*literal!(") [SNGL]: ")); __mm_s.push_str(&*List::toString(var_field!((*call).iters, NSimGenericCall::SINGLE_GENERIC_CALL).clone(), (std::sync::Arc::new(SimIterator::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimIterator::SimIterator>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*Expression::toString(var_field!((*call).lhs, NSimGenericCall::SINGLE_GENERIC_CALL).clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(var_field!((*call).rhs, NSimGenericCall::SINGLE_GENERIC_CALL).clone())?); ArcStr::from(__mm_s) },
        Deref @ IF_GENERIC_CALL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*call).index, NSimGenericCall::IF_GENERIC_CALL).clone())); __mm_s.push_str(&*literal!(") [-IF-]: ")); __mm_s.push_str(&*List::toString(var_field!((*call).iters, NSimGenericCall::IF_GENERIC_CALL).clone(), (std::sync::Arc::new(SimIterator::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimIterator::SimIterator>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*List::toString(var_field!((*call).branches, NSimGenericCall::IF_GENERIC_CALL).clone(), (std::sync::Arc::new(SimBranch::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimBranch::SimBranch>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!("\telse")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        Deref @ WHEN_GENERIC_CALL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*call).index, NSimGenericCall::WHEN_GENERIC_CALL).clone())); __mm_s.push_str(&*literal!(") [WHEN]: ")); __mm_s.push_str(&*List::toString(var_field!((*call).iters, NSimGenericCall::WHEN_GENERIC_CALL).clone(), (std::sync::Arc::new(SimIterator::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimIterator::SimIterator>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*List::toString(var_field!((*call).branches, NSimGenericCall::WHEN_GENERIC_CALL).clone(), (std::sync::Arc::new(SimBranch::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimBranch::SimBranch>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!("\telse")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        _ => literal!("CALL_NOT_SUPPORTED"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn fromIdentifier(mut ident_tpl: (Arc<Identifier::Identifier>, i32)) -> Result<Arc<NSimGenericCall>> {
    let mut call: Arc<NSimGenericCall>;
    let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut index: i32 = 0;
    let mut resizable: bool = false;
    let mut body: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ident_tpl.clone()) {
        (Deref @ Identifier::IDENTIFIER { resizable: __pa0, eqn: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    resizable = __pa0.clone();
    eqn_ptr = __pa1.clone();
    index = __pa2.clone();
    eqn = Pointer::access(eqn_ptr.clone());
    call = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body @ Deref @ Equation::IF_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut iters: Arc<metamodelica::List<Arc<SimIterator::SimIterator>>> = metamodelica::nil();
            iters = SimIterator::fromIterator(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            Arc::new(NSimGenericCall::IF_GENERIC_CALL { resizable: resizable.clone(), branches: SimBranch::fromIfBody(var_field!((**body).body, Equation::Equation::IF_EQUATION).clone())?, iters: iters.clone(), index: index.clone() })
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body @ Deref @ Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut iters: Arc<metamodelica::List<Arc<SimIterator::SimIterator>>> = metamodelica::nil();
            iters = SimIterator::fromIterator(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            Arc::new(NSimGenericCall::WHEN_GENERIC_CALL { resizable: resizable.clone(), branches: SimBranch::fromWhenBody(var_field!((**body).body, Equation::Equation::WHEN_EQUATION).clone())?, iters: iters.clone(), index: index.clone() })
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut iters: Arc<metamodelica::List<Arc<SimIterator::SimIterator>>> = metamodelica::nil();
            iters = SimIterator::fromIterator(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            Arc::new(NSimGenericCall::SINGLE_GENERIC_CALL { resizable: resizable.clone(), rhs: Util::getOption(Equation::getRHS(body.clone())?)?, lhs: Util::getOption(Equation::getLHS(body.clone())?)?, iters: iters.clone(), index: index.clone() })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimGenericCall.fromIdentifier")); __mm_s.push_str(&*literal!(" failed for incorrect equation: ")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(call)
}

pub fn convert(mut call: Arc<NSimGenericCall>) -> Result<OldSimCode::SimGenericCall> {
    let mut old_call: OldSimCode::SimGenericCall;
    old_call = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ SINGLE_GENERIC_CALL { .. } => OldSimCode::SimGenericCall::SINGLE_GENERIC_CALL { resizable: var_field!((*call).resizable, NSimGenericCall::SINGLE_GENERIC_CALL).clone(), rhs: Expression::toDAE(var_field!((*call).rhs, NSimGenericCall::SINGLE_GENERIC_CALL).clone(), false)?, lhs: Expression::toDAE(var_field!((*call).lhs, NSimGenericCall::SINGLE_GENERIC_CALL).clone(), false)?, iters: {
        let mut __acc: Arc<metamodelica::List<OldBackendDAE::SimIterator>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NSimGenericCall::SINGLE_GENERIC_CALL).clone()).into_iter().cloned() {
            let __x = SimIterator::convert(iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, index: var_field!((*call).index, NSimGenericCall::SINGLE_GENERIC_CALL).clone() },
        Deref @ IF_GENERIC_CALL { .. } => OldSimCode::SimGenericCall::IF_GENERIC_CALL { resizable: var_field!((*call).resizable, NSimGenericCall::IF_GENERIC_CALL).clone(), branches: {
        let mut __acc: Arc<metamodelica::List<OldSimCode::SimBranch>> = metamodelica::nil();
        for mut branch in (var_field!((*call).branches, NSimGenericCall::IF_GENERIC_CALL).clone()).into_iter().cloned() {
            let __x = SimBranch::convert(branch.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, iters: {
        let mut __acc: Arc<metamodelica::List<OldBackendDAE::SimIterator>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NSimGenericCall::IF_GENERIC_CALL).clone()).into_iter().cloned() {
            let __x = SimIterator::convert(iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, index: var_field!((*call).index, NSimGenericCall::IF_GENERIC_CALL).clone() },
        Deref @ WHEN_GENERIC_CALL { .. } => OldSimCode::SimGenericCall::WHEN_GENERIC_CALL { resizable: var_field!((*call).resizable, NSimGenericCall::WHEN_GENERIC_CALL).clone(), branches: {
        let mut __acc: Arc<metamodelica::List<OldSimCode::SimBranch>> = metamodelica::nil();
        for mut branch in (var_field!((*call).branches, NSimGenericCall::WHEN_GENERIC_CALL).clone()).into_iter().cloned() {
            let __x = SimBranch::convert(branch.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, iters: {
        let mut __acc: Arc<metamodelica::List<OldBackendDAE::SimIterator>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NSimGenericCall::WHEN_GENERIC_CALL).clone()).into_iter().cloned() {
            let __x = SimIterator::convert(iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, index: var_field!((*call).index, NSimGenericCall::WHEN_GENERIC_CALL).clone() },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimGenericCall.convert")); __mm_s.push_str(&*literal!(" failed for incorrect call: ")); __mm_s.push_str(&*toString(call.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(old_call)
}

pub mod SimIterator {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SimIterator {
        SIM_ITERATOR_RANGE {
            name: Arc<ComponentRef::NFComponentRef>,
            start: Arc<Expression::NFExpression>,
            step: Arc<Expression::NFExpression>,
            stop: Arc<Expression::NFExpression>,
            size: Arc<Expression::NFExpression>,
            sub_iter: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, metamodelica::Array<Arc<Expression::NFExpression>>)>>,
        },
        SIM_ITERATOR_LIST {
            name: Arc<ComponentRef::NFComponentRef>,
            lst: Arc<metamodelica::List<i32>>,
            size: i32,
            sub_iter: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, metamodelica::Array<Arc<Expression::NFExpression>>)>>,
        },
    }
    impl Default for SimIterator {
        fn default() -> Self {
            Self::SIM_ITERATOR_LIST {
                name: Default::default(),
                lst: Default::default(),
                size: Default::default(),
                sub_iter: Default::default(),
            }
        }
    }
    pub use self::SimIterator::{SIM_ITERATOR_RANGE,SIM_ITERATOR_LIST};
    pub fn toString(mut iter: Arc<SimIterator>) -> Result<ArcStr> {
        pub fn subIterString(mut sub_iter: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, metamodelica::Array<Arc<Expression::NFExpression>>)>>) -> ArcStr {
            let mut r#str: ArcStr = List::toString({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut tpl in (sub_iter.clone()).into_iter().cloned() {
            let __x = Util::tuple21(tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0).unwrap();
            r#str
        }

        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SIM_ITERATOR_RANGE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*iter).name, SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(" | start:")); __mm_s.push_str(&*Expression::toString(var_field!((*iter).start, SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(", step:")); __mm_s.push_str(&*Expression::toString(var_field!((*iter).step, SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(", stop:")); __mm_s.push_str(&*Expression::toString(var_field!((*iter).stop, SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(", size: ")); __mm_s.push_str(&*Expression::toString(var_field!((*iter).size, SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*subIterString(var_field!((*iter).sub_iter, SimIterator::SIM_ITERATOR_RANGE).clone())); ArcStr::from(__mm_s) },
        Deref @ SIM_ITERATOR_LIST { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*iter).name, SimIterator::SIM_ITERATOR_LIST).clone())?); __mm_s.push_str(&*literal!(" | list: ")); __mm_s.push_str(&*List::toString(var_field!((*iter).lst, SimIterator::SIM_ITERATOR_LIST).clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 10)?); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*subIterString(var_field!((*iter).sub_iter, SimIterator::SIM_ITERATOR_LIST).clone())); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(r#str)
    }

    pub fn fromIterator(mut iter: Arc<Iterator::Iterator>) -> Result<Arc<metamodelica::List<Arc<SimIterator>>>> {
        let mut sim_iter: Arc<metamodelica::List<Arc<SimIterator>>> = metamodelica::nil();
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
        let mut mulOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
        let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut step: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut size: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut map: Option<Arc<Iterator::Iterator>> = None;
        let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut sub_iter: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, metamodelica::Array<Arc<Expression::NFExpression>>)>> = metamodelica::nil();
        (names, ranges, maps) = Iterator::getFrames(iter.clone())?;
        for mut tpl in &*List::zip3(names.clone(), ranges.clone(), maps.clone()) {
            let mut tpl = tpl.clone();
            (name, range, map) = tpl.clone();
            sim_iter = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { .. } => {
            step = Util::getOptionOrDefault(var_field!((*range).step, Expression::NFExpression::RANGE).clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }));
            addOp = Operator::makeAdd(Expression::typeOf(var_field!((*range).start, Expression::NFExpression::RANGE).clone()));
            mulOp = Operator::makeMul(Expression::typeOf(var_field!((*range).start, Expression::NFExpression::RANGE).clone()));
            size = Arc::new(Expression::NFExpression::MULTARY { arguments: list![var_field!((*range).stop, Expression::NFExpression::RANGE).clone()], inv_arguments: list![var_field!((*range).start, Expression::NFExpression::RANGE).clone()], operator: addOp.clone() });
            size = Arc::new(Expression::NFExpression::MULTARY { arguments: list![size.clone()], inv_arguments: list![step.clone()], operator: mulOp.clone() });
            size = Arc::new(Expression::NFExpression::MULTARY { arguments: list![size.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 })], inv_arguments: metamodelica::nil(), operator: addOp.clone() });
            size = SimplifyExp::simplify(size.clone(), false)?;
            sub_iter = if (isSome(map.clone())) {subIterators(Util::getOption(map.clone())?)?} else {metamodelica::nil()};
            cons(Arc::new(SimIterator::SIM_ITERATOR_RANGE { name: name.clone(), start: var_field!((*range).start, Expression::NFExpression::RANGE).clone(), step: step.clone(), stop: var_field!((*range).stop, Expression::NFExpression::RANGE).clone(), size: size.clone(), sub_iter: sub_iter.clone() }), sim_iter.clone())
        },
        Deref @ Expression::ARRAY { .. } if (var_field!((*range).literal, Expression::NFExpression::ARRAY).clone()) => {
            lst = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (var_field!((*range).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::integerValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            sub_iter = if (isSome(map.clone())) {subIterators(Util::getOption(map.clone())?)?} else {metamodelica::nil()};
            cons(Arc::new(SimIterator::SIM_ITERATOR_LIST { name: name.clone(), lst: lst.clone(), size: (lst.clone().len() as i32), sub_iter: sub_iter.clone() }), sim_iter.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimGenericCall.SimIterator.fromIterator")); __mm_s.push_str(&*literal!(" failed for incorrect iterator domain: ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(sim_iter)
    }

    pub fn subIterators(mut iter: Arc<Iterator::Iterator>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, metamodelica::Array<Arc<Expression::NFExpression>>)>>> {
        let mut sub_iter: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, metamodelica::Array<Arc<Expression::NFExpression>>)>> = metamodelica::nil();
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        (names, ranges, _) = Iterator::getFrames(iter.clone())?;
        for mut tpl in &*List::zip(names.clone(), ranges.clone()).reverse() {
            let mut tpl = tpl.clone();
            (name, range) = tpl.clone();
            sub_iter = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::ARRAY { .. } => cons((name.clone(), var_field!((*range).elements, Expression::NFExpression::ARRAY).clone()), sub_iter.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimGenericCall.SimIterator.subIterators")); __mm_s.push_str(&*literal!(" failed for incorrect iterator domain: ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(sub_iter)
    }

    pub fn convert(mut iter: Arc<SimIterator>) -> Result<OldBackendDAE::SimIterator> {
        pub fn convertSubIterator(mut sub_iter: (Arc<ComponentRef::NFComponentRef>, metamodelica::Array<Arc<Expression::NFExpression>>)) -> (Arc<DAE::ComponentRef>, metamodelica::Array<Arc<DAE::Exp>>) {
            let mut old_sub_iter: (Arc<DAE::ComponentRef>, metamodelica::Array<Arc<DAE::Exp>>) = (ComponentRef::toDAE(Util::tuple21(sub_iter.clone())).unwrap(), metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (Util::tuple22(sub_iter.clone())).borrow().iter() {
            let __x = Expression::toDAE(e.clone(), false).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()));
            old_sub_iter
        }

        let mut old_iter: OldBackendDAE::SimIterator;
        old_iter = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SIM_ITERATOR_RANGE { .. } => OldBackendDAE::SimIterator::SIM_ITERATOR_RANGE { name: ComponentRef::toDAE(var_field!((*iter).name, SimIterator::SIM_ITERATOR_RANGE).clone())?, start: Expression::toDAE(var_field!((*iter).start, SimIterator::SIM_ITERATOR_RANGE).clone(), false)?, step: Expression::toDAE(var_field!((*iter).step, SimIterator::SIM_ITERATOR_RANGE).clone(), false)?, stop: Expression::toDAE(var_field!((*iter).stop, SimIterator::SIM_ITERATOR_RANGE).clone(), false)?, size: Expression::toDAE(var_field!((*iter).size, SimIterator::SIM_ITERATOR_RANGE).clone(), false)?, non_resizable_size: Expression::getInteger(var_field!((*iter).size, SimIterator::SIM_ITERATOR_RANGE).clone(), false)?, sub_iter: {
        let mut __acc: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, metamodelica::Array<Arc<DAE::Exp>>)>> = metamodelica::nil();
        for mut si in (var_field!((*iter).sub_iter, SimIterator::SIM_ITERATOR_RANGE).clone()).into_iter().cloned() {
            let __x = convertSubIterator(si.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } },
        Deref @ SIM_ITERATOR_LIST { .. } => OldBackendDAE::SimIterator::SIM_ITERATOR_LIST { name: ComponentRef::toDAE(var_field!((*iter).name, SimIterator::SIM_ITERATOR_LIST).clone())?, lst: var_field!((*iter).lst, SimIterator::SIM_ITERATOR_LIST).clone(), size: var_field!((*iter).size, SimIterator::SIM_ITERATOR_LIST).clone(), sub_iter: {
        let mut __acc: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, metamodelica::Array<Arc<DAE::Exp>>)>> = metamodelica::nil();
        for mut si in (var_field!((*iter).sub_iter, SimIterator::SIM_ITERATOR_LIST).clone()).into_iter().cloned() {
            let __x = convertSubIterator(si.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } },
        _ => bail!("match: no arm matched"),
    } });
        Ok(old_iter)
    }

}

/// represents a dependent sub iterator
pub type DependentIterator = (Arc<ComponentRef::NFComponentRef>, metamodelica::Array<Arc<Expression::NFExpression>>);

pub mod SimBranch {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SimBranch {
        SIM_BRANCH {
            condition: Arc<Expression::NFExpression>,
            body: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)>>,
        },
        SIM_BRANCH_STMT {
            condition: Arc<Expression::NFExpression>,
            body: Arc<metamodelica::List<Arc<Statement::NFStatement>>>,
        },
    }
    impl Default for SimBranch {
        fn default() -> Self {
            Self::SIM_BRANCH {
                condition: Default::default(),
                body: Default::default(),
            }
        }
    }
    pub use self::SimBranch::{SIM_BRANCH,SIM_BRANCH_STMT};
    pub fn mapShallow(mut branch: Arc<SimBranch>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<SimBranch> {
        type mapExp = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

        let mut branch: Arc<SimBranch> = branch;
        branch = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ SIM_BRANCH { .. } => {
            assign_variant_field!(branch => SimBranch::SIM_BRANCH; body = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        for mut tpl in (var_field!((*branch).body, SimBranch::SIM_BRANCH).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(tpl.clone())).unwrap(), func(Util::tuple22(tpl.clone())).unwrap());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            branch.clone()
        },
        Deref @ SIM_BRANCH_STMT { .. } => {
            assign_variant_field!(branch => SimBranch::SIM_BRANCH_STMT; body = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut stmt in (var_field!((*branch).body, SimBranch::SIM_BRANCH_STMT).clone()).into_iter().cloned() {
            let __x = Statement::mapExp(stmt.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            branch.clone()
        },
        _ => branch.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        branch
    }

    pub fn toString(mut branch: Arc<SimBranch>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        r#str = ((::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ SIM_BRANCH { .. } => {
            r#str = (if (Expression::isEnd(var_field!((*branch).condition, SimBranch::SIM_BRANCH).clone())) {literal!("\n")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*Expression::toString(var_field!((*branch).condition, SimBranch::SIM_BRANCH).clone())?); __mm_s.push_str(&*literal!(" then\n")); ArcStr::from(__mm_s) }}).clone();
            for mut tpl in &*var_field!((*branch).body, SimBranch::SIM_BRANCH).clone() {
                let mut tpl = tpl.clone();
                (lhs, rhs) = tpl.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\t  ")); __mm_s.push_str(&*Expression::toString(lhs.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(rhs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        Deref @ SIM_BRANCH_STMT { .. } => {
            r#str = (if (Expression::isEnd(var_field!((*branch).condition, SimBranch::SIM_BRANCH_STMT).clone())) {literal!("\n")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("when ")); __mm_s.push_str(&*Expression::toString(var_field!((*branch).condition, SimBranch::SIM_BRANCH_STMT).clone())?); __mm_s.push_str(&*literal!(" then\n")); ArcStr::from(__mm_s) }}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(var_field!((*branch).body, SimBranch::SIM_BRANCH_STMT).clone(), Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Statement::toString(__pe_a0, __pe_b1.clone()) }), (literal!("\t  ")).clone(), (literal!("\t  ")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => literal!("SIM BRANCH NOT KNOWN"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn fromIfBody(mut if_body: Arc<IfEquationBody::IfEquationBody>) -> Result<Arc<metamodelica::List<Arc<SimBranch>>>> {
        let mut branches: Arc<metamodelica::List<Arc<SimBranch>>> = metamodelica::nil();
        let mut body: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        let mut branch: Arc<SimBranch>;
        for mut eqn in &*if_body.then_eqns.clone().reverse() {
            let mut eqn = eqn.clone();
            body = cons((Util::getOption(Equation::getLHS(Pointer::access(eqn.clone()))?)?, Util::getOption(Equation::getRHS(Pointer::access(eqn.clone()))?)?), body.clone());
        }
        branch = Arc::new(SimBranch::SIM_BRANCH { condition: if_body.condition.clone(), body: body.clone() });
        if isSome(if_body.else_if.clone()) {
            branches = cons(branch.clone(), fromIfBody(Util::getOption(if_body.else_if.clone())?)?);
        } else {
            branches = list![branch.clone()];
        }
        Ok(branches)
    }

    pub fn fromWhenBody(mut when_body: Arc<WhenEquationBody::WhenEquationBody>) -> Result<Arc<metamodelica::List<Arc<SimBranch>>>> {
        let mut branches: Arc<metamodelica::List<Arc<SimBranch>>> = metamodelica::nil();
        let mut branch: Arc<SimBranch>;
        branch = Arc::new(SimBranch::SIM_BRANCH_STMT { condition: when_body.condition.clone(), body: {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut stmt in (when_body.when_stmts.clone()).into_iter().cloned() {
            let __x = WhenStatement::toStatement(stmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
        if isSome(when_body.else_when.clone()) {
            branches = cons(branch.clone(), fromWhenBody(Util::getOption(when_body.else_when.clone())?)?);
        } else {
            branches = list![branch.clone()];
        }
        Ok(branches)
    }

    pub fn convert(mut branch: Arc<SimBranch>) -> Result<OldSimCode::SimBranch> {
        let mut old_branch: OldSimCode::SimBranch;
        let mut old_condition: Option<Arc<DAE::Exp>> = None;
        let mut old_body: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::Exp>)>> = metamodelica::nil();
        let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        old_branch = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ SIM_BRANCH { .. } => {
            old_condition = (::match_deref::match_deref! { match &(var_field!((*branch).condition, SimBranch::SIM_BRANCH).clone()) {
        Deref @ Expression::END => None,
        _ => Some(Expression::toDAE(var_field!((*branch).condition, SimBranch::SIM_BRANCH).clone(), false)?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            for mut tpl in &*var_field!((*branch).body, SimBranch::SIM_BRANCH).clone().reverse() {
                let mut tpl = tpl.clone();
                (lhs, rhs) = tpl.clone();
                old_body = cons((Expression::toDAE(lhs.clone(), false)?, Expression::toDAE(rhs.clone(), false)?), old_body.clone());
            }
            OldSimCode::SimBranch::SIM_BRANCH { body: old_body.clone(), condition: old_condition.clone() }
        },
        Deref @ SIM_BRANCH_STMT { .. } => {
            old_condition = (::match_deref::match_deref! { match &(var_field!((*branch).condition, SimBranch::SIM_BRANCH_STMT).clone()) {
        Deref @ Expression::END => None,
        _ => Some(Expression::toDAE(var_field!((*branch).condition, SimBranch::SIM_BRANCH_STMT).clone(), false)?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            OldSimCode::SimBranch::SIM_BRANCH_STMT { body: ConvertDAE::convertStatements(var_field!((*branch).body, SimBranch::SIM_BRANCH_STMT).clone())?, condition: old_condition.clone() }
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(old_branch)
    }

}


