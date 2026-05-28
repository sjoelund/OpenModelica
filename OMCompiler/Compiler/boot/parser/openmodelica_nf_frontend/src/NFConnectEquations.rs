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

use crate::NFBinding as Binding;
use crate::NFBuiltin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCardinalityTable as CardinalityTable;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnector as Connector;
use crate::NFConnector::Face;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatten as Flatten;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

thread_local! { static __EQ_ASSERT_STR_TLS: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::STRING { value: (literal!("Connected constants/parameters must be equal")).clone() }); }
pub fn EQ_ASSERT_STR() -> Arc<Expression::NFExpression> { __EQ_ASSERT_STR_TLS.with(|__t| __t.clone()) }

pub fn generateEquations(mut sets: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>>)> {
    type potFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Connector::NFConnector>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>)> + 'static>;

    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut connectedLocalIOs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut unhandledStreamSets: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>> = metamodelica::nil();
    let mut set_eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut potfunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Connector::NFConnector>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>)> + 'static>;
    let mut flowThreshold: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cty: i32 = 0;
    let mut flow_alias_elim: bool = Flags::isSet(Flags::FLOW_ALIAS_ELIMINATION.clone())?;
    openmodelica_util::Globals::isInStream.with(|__root| *__root.borrow_mut() = None);
    connectedLocalIOs = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    potfunc = (std::sync::Arc::new(generatePotentialEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Connector::NFConnector>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>)> + 'static>);
    flowThreshold = Arc::new(Expression::NFExpression::REAL { value: Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())? });
    let __range0 = sets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut set in __range0 {
        cty = getSetType(set.clone())?;
        if Prefixes::ConnectorType::isPotential(cty.clone()) {
            (set_eql, connectedLocalIOs) = potfunc(set.clone(), connectedLocalIOs.clone())?;
        } else if Prefixes::ConnectorType::isFlow(cty.clone()) {
            set_eql = generateFlowEquations(set.clone())?;
        } else if Prefixes::ConnectorType::isStream(cty.clone()) {
            if flow_alias_elim.clone() {
                unhandledStreamSets = cons(set.clone(), unhandledStreamSets.clone());
                set_eql = metamodelica::nil();
            } else {
                set_eql = generateStreamEquations(set.clone(), flowThreshold.clone(), variables.clone())?;
            }
        } else {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConnectEquations.generateEquations")); __mm_s.push_str(&*literal!(" got connection set with invalid type '")); __mm_s.push_str(&*Prefixes::ConnectorType::toDebugString(cty.clone())); __mm_s.push_str(&*literal!("': ")); __mm_s.push_str(&*List::toString(set.clone(), (std::sync::Arc::new(fnptr!(Connector::toString, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
        equations = listAppend(set_eql.clone(), equations.clone());
    }
    unhandledStreamSets = unhandledStreamSets.clone().reverse();
    Ok((equations, connectedLocalIOs, unhandledStreamSets))
}

pub fn evaluateOperators(mut exp: Arc<Expression::NFExpression>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<Expression::NFExpression>> {
    use crate::NFOperator::Op;
    let mut evalExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    evalExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call } => {
            (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_CALL { .. } => (::match_deref::match_deref! { match &(Function::name(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" } => evaluateInStream(Expression::toCref(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?)?, sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?,
        Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" } => {
            (evalExp, _) = evaluateActualStream(Expression::toCref(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?)?, sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
            evalExp.clone()
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "cardinality" } => CardinalityTable::evaluateCardinality(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?, ctable.clone())?,
        _ => Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = sets.clone(); let __pe_b2 = setsArray.clone(); let __pe_b3 = variables.clone(); let __pe_b4 = ctable.clone(); move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ Call::TYPED_REDUCTION { .. } if (Expression::contains(var_field!((**call).exp, Call::NFCall::TYPED_REDUCTION).clone(), (std::sync::Arc::new(isStreamCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) => evaluateOperatorReductionExp(exp.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?,
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } if (Expression::contains(var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), (std::sync::Arc::new(isStreamCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) => evaluateOperatorArrayConstructorExp(exp.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?,
        _ => Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = sets.clone(); let __pe_b2 = setsArray.clone(); let __pe_b3 = variables.clone(); let __pe_b4 = ctable.clone(); move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ Expression::BINARY { exp2: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp1: Deref @ Expression::CREF { .. } } if (AbsynUtil::isNamedPathIdent(Function::name(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone()), (literal!("actualStream")).clone())) => {
            evaluateActualStreamMul(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?, var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?
        },
        Deref @ Expression::BINARY { exp2: Deref @ Expression::CREF { .. }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp1: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } } if (AbsynUtil::isNamedPathIdent(Function::name(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone()), (literal!("actualStream")).clone())) => {
            evaluateActualStreamMul(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?, var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?
        },
        _ => {
            Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = sets.clone(); let __pe_b2 = setsArray.clone(); let __pe_b3 = variables.clone(); let __pe_b4 = ctable.clone(); move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(evalExp)
}

fn getSetType(mut set: Arc<metamodelica::List<Arc<Connector::NFConnector>>>) -> Result<i32> {
    let mut cty: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(set.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { cty: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cty = __pa0.clone();
    Ok(cty)
}

fn generatePotentialEquations(mut elements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut connectedLocalIOs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>)> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut connectedLocalIOs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = connectedLocalIOs;
    let mut c1: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    c1 = listHead(elements.clone())?;
    if Connector::variability(c1.clone()) > Variability::PARAMETER.clone() {
        equations = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut c2 in (listRest(elements.clone())?).into_iter().cloned() {
            let __x = makeEqualityEquation(c1.name.clone(), c1.source.clone(), c2.name.clone(), c2.source.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        if Flags::getConfigInt(Flags::EXPOSE_LOCAL_IOS.clone())? > 0 {
            for mut c in &*elements.clone() {
                let mut c = c.clone();
                if Connector::isInside(c.clone()) && (ComponentRef::isInput(c.name.clone()) || ComponentRef::isOutput(c.name.clone())) {
                    UnorderedSet::add((ComponentRef::stripSubscripts(c.name.clone())).0, connectedLocalIOs.clone())?;
                }
            }
        }
    } else {
        if Type::isEmptyArray(c1.ty.clone()) {
            equations = metamodelica::nil();
        } else {
            equations = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut c2 in (listRest(elements.clone())?).into_iter().cloned() {
            let __x = makeEqualityAssert(c1.name.clone(), c1.source.clone(), c2.name.clone(), c2.source.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        }
    }
    Ok((equations, connectedLocalIOs))
}

//function generatePotentialEquationsOrdered
//  "Like generatePotentialEquations, but orders the connectors with
//   shouldFlipPotentialEquation."
//  input list<Connector> elements;
//  output list<Equation> equations = {};
//protected
//  partial function eqFunc
//    input ComponentRef lhsCref;
//    input DAE.ElementSource lhsSource;
//    input ComponentRef rhsCref;
//    input DAE.ElementSource rhsSource;
//    output Equation eq;
//  end eqFunc;
//
//  Connector c1;
//  ComponentRef cr1, cr2;
//  DAE.ElementSource source;
//  eqFunc eqfunc;
//algorithm
//  if listEmpty(elements) then
//    return;
//  end if;
//
//  c1 := listHead(elements);
//  eqfunc := if Connector.variability(c1) > Variability.PARAMETER then
//    makeEqualityEquation else makeEqualityAssert;
//
//  cr1 := c1.name;
//
//  for c2 in listRest(elements) loop
//    cr2 := c2.name;
//    (cr1, cr2) := Util.swap(shouldFlipPotentialEquation(cr1, c1.source), cr1, cr2);
//    equations := eqfunc(cr1, c2.source, cr2, c2.source) :: equations;
//    c1 := c2;
//    cr1 := cr2;
//  end for;
//end generatePotentialEquationsOrdered;
fn makeEqualityEquation(mut lhsCref: Arc<ComponentRef::NFComponentRef>, mut lhsSource: Arc<DAE::ElementSource>, mut rhsCref: Arc<ComponentRef::NFComponentRef>, mut rhsSource: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut equalityEq: Arc<Equation::NFEquation>;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    source = ElementSource::mergeSources(lhsSource.clone(), rhsSource.clone())?;
    equalityEq = Equation::makeCrefEquality(lhsCref.clone(), rhsCref.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), source.clone())?;
    Ok(equalityEq)
}

fn makeEqualityAssert(mut lhsCref: Arc<ComponentRef::NFComponentRef>, mut lhsSource: Arc<DAE::ElementSource>, mut rhsCref: Arc<ComponentRef::NFComponentRef>, mut rhsSource: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut equalityAssert: Arc<Equation::NFEquation>;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut lhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut elem_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut iterators: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    source = ElementSource::mergeSources(lhsSource.clone(), rhsSource.clone())?;
    ty = ComponentRef::getSubscriptedType(lhsCref.clone(), false)?;
    if Type::isArray(ty.clone()) {
        (iterators, ranges, subs) = Flatten::makeIterators(lhsCref.clone(), Type::arrayDims(ty.clone()))?;
        subs = subs.clone().reverse();
        lhs_exp = Expression::fromCref(ComponentRef::mergeSubscripts(subs.clone(), lhsCref.clone(), false, false, false)?, false)?;
        rhs_exp = Expression::fromCref(ComponentRef::mergeSubscripts(subs.clone(), rhsCref.clone(), false, false, false)?, false)?;
    } else {
        lhs_exp = Expression::fromCref(lhsCref.clone(), false)?;
        rhs_exp = Expression::fromCref(rhsCref.clone(), false)?;
    }
    elem_ty = Type::arrayElementType(ty.clone());
    if Type::isReal(elem_ty.clone()) {
        exp = Arc::new(Expression::NFExpression::BINARY { exp1: lhs_exp.clone(), operator: Operator::makeSub(elem_ty.clone()), exp2: rhs_exp.clone() });
        exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::ABS_REAL().clone(), list![exp.clone()], Expression::variability(exp.clone())?, Purity::PURE.clone(), NFBuiltinFuncs::ABS_REAL().returnType.clone()) });
        exp = Arc::new(Expression::NFExpression::RELATION { exp1: exp.clone(), operator: Operator::makeLessEq(elem_ty.clone()), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }), index: -1 });
    } else {
        exp = Arc::new(Expression::NFExpression::RELATION { exp1: lhs_exp.clone(), operator: Operator::makeEqual(elem_ty.clone()), exp2: rhs_exp.clone(), index: -1 });
    }
    equalityAssert = Arc::new(Equation::NFEquation::ASSERT { condition: exp.clone(), message: EQ_ASSERT_STR().clone(), level: NFBuiltin::ASSERTIONLEVEL_ERROR().clone(), scope: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), source: source.clone() });
    while !(iterators.clone().is_empty()) {
        equalityAssert = Arc::new(Equation::NFEquation::FOR { iterator: listHead(iterators.clone())?, range: Some(listHead(ranges.clone())?), body: list![equalityAssert.clone()], scope: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), source: source.clone() });
        iterators = listRest(iterators.clone())?;
        ranges = listRest(ranges.clone())?;
    }
    Ok(equalityAssert)
}

//protected function shouldFlipPotentialEquation
//  "If the flag +orderConnections=false is used, then we should keep the order of
//   the connector elements as they occur in the connection (if possible). In that
//   case we check if the cref of the first argument to the first connection
//   stored in the element source is a prefix of the connector element cref. If
//   it isn't, indicate that we should flip the generated equation."
//  input DAE.ComponentRef lhsCref;
//  input DAE.ElementSource lhsSource;
//  output Boolean shouldFlip;
//algorithm
//  shouldFlip := match lhsSource
//    local
//      DAE.ComponentRef lhs;
//
//    case DAE.SOURCE(connectEquationOptLst = (lhs, _) :: _)
//      then not ComponentReferenceBasics.crefPrefixOf(lhs, lhsCref);
//
//    else false;
//  end match;
//end shouldFlipPotentialEquation;
fn generateFlowEquations(mut elements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut c: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut c_rest: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut sum: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iterators: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut eq: Arc<Equation::NFEquation>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elements.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    c = __pa0.clone();
    c_rest = __pa1.clone();
    src = c.source.clone();
    if Connector::isArray(c.clone()) {
        (iterators, ranges, subs) = Flatten::makeIterators(c.name.clone(), Type::arrayDims(c.ty.clone()))?;
        subs = subs.clone().reverse();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &({
        let mut __acc: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
        for mut e in (elements.clone()).into_iter().cloned() {
            let __x = Connector::addSubscripts(subs.clone(), e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        c = __pa2.clone();
        c_rest = __pa3.clone();
    }
    if c_rest.clone().is_empty() {
        sum = Expression::fromCref(c.name.clone(), false)?;
    } else {
        sum = makeFlowExp(c.clone())?;
        for mut e in &*c_rest.clone() {
            let mut e = e.clone();
            sum = Arc::new(Expression::NFExpression::BINARY { exp1: sum.clone(), operator: Operator::makeAdd(Arc::new(crate::NFType::REAL)), exp2: makeFlowExp(e.clone())? });
            src = ElementSource::mergeSources(src.clone(), e.source.clone())?;
        }
    }
    equations = list![Equation::makeEquality(sum.clone(), Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }), Type::arrayElementType(c.ty.clone()), src.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone())];
    while !(iterators.clone().is_empty()) {
        equations = list![Arc::new(Equation::NFEquation::FOR { iterator: listHead(iterators.clone())?, range: Some(listHead(ranges.clone())?), body: equations.clone(), scope: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), source: src.clone() })];
        iterators = listRest(iterators.clone())?;
        ranges = listRest(ranges.clone())?;
    }
    Ok(equations)
}

fn makeFlowExp(mut element: Arc<Connector::NFConnector>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut face: Face = Face::INSIDE;
    exp = Expression::fromCref(element.name.clone(), false)?;
    face = element.face.clone();
    if face.clone() == Face::OUTSIDE.clone() {
        exp = Arc::new(Expression::NFExpression::UNARY { operator: Operator::makeUMinus(Arc::new(crate::NFType::REAL)), exp: exp.clone() });
    }
    Ok(exp)
}

fn generateStreamEquations(mut elements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut flowThreshold: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut cr1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cr2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut src1: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut src2: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut cref1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut inside: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut outside: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut var1: Variability = Variability::CONSTANT;
    let mut var2: Variability = Variability::CONSTANT;
    (outside, inside) = List::splitOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(Connector::isOutside, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>) -> Result<bool> + 'static>));
    inside = {
        let mut __acc: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
        for mut s in (inside.clone()).into_iter().cloned() {
            if !(!(isNoFlowInside(s.clone(), variables.clone())?)) { continue; }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    equations = (::match_deref::match_deref! { match &((inside.clone(), outside.clone())) {
        (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil) => metamodelica::nil(),
        (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil) => metamodelica::nil(),
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { source: src1, name: cr1, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { source: src2, name: cr2, .. }, tail: Deref @ metamodelica::List::Nil } }) => {
            cref1 = Expression::fromCref(cr1.clone(), false)?;
            cref2 = Expression::fromCref(cr2.clone(), false)?;
            e1 = makeInStreamCall(cref2.clone())?;
            e2 = makeInStreamCall(cref1.clone())?;
            src = ElementSource::mergeSources(src1.clone(), src2.clone())?;
            list![Equation::makeEquality(cref1.clone(), e1.clone(), Arc::new(crate::NFType::REAL), src.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone()), Equation::makeEquality(cref2.clone(), e2.clone(), Arc::new(crate::NFType::REAL), src.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone())]
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { source: src1, name: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { source: src2, name: cr2, .. }, tail: Deref @ metamodelica::List::Nil }) => {
            src = ElementSource::mergeSources(src1.clone(), src2.clone())?;
            list![Equation::makeCrefEquality(cr1.clone(), cr2.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), src.clone())?]
        },
        _ => streamEquationGeneral(outside.clone(), inside.clone(), flowThreshold.clone(), variables.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

fn streamEquationGeneral(mut outsideElements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut insideElements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut flowThreshold: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut reduced_outside: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut outside: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut cref_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    reduced_outside = {
        let mut __acc: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
        for mut s in (outsideElements.clone()).into_iter().cloned() {
            if !(!(isNoFlowOutside(s.clone(), variables.clone())?)) { continue; }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    for mut e in &*outsideElements.clone() {
        let mut e = e.clone();
        cref_exp = Expression::fromCref(e.name.clone(), false)?;
        outside = removeStreamSetElement(e.name.clone(), reduced_outside.clone())?;
        res = streamSumEquationExp(outside.clone(), insideElements.clone(), flowThreshold.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), variables.clone())?;
        src = ElementSource::addAdditionalComment(e.source.clone(), (literal!(" equation generated from stream connection")).clone())?;
        equations = cons(Equation::makeEquality(cref_exp.clone(), res.clone(), Arc::new(crate::NFType::REAL), src.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone()), equations.clone());
    }
    Ok(equations)
}

fn streamSumEquationExp(mut outsideElements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut insideElements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut flowThreshold: Arc<Expression::NFExpression>, mut fallback: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut sumExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outside_sum1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outside_sum2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut inside_sum1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut inside_sum2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    sumExp = (match (outsideElements.clone().is_empty(), insideElements.clone().is_empty()) {
        (true, true) => fallback.clone(),
        (true, false) => {
            inside_sum1 = sumMap(insideElements.clone(), (std::sync::Arc::new(sumInside1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>), flowThreshold.clone(), variables.clone())?;
            inside_sum2 = sumMap(insideElements.clone(), (std::sync::Arc::new(sumInside2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>), flowThreshold.clone(), variables.clone())?;
            sumExp = Arc::new(Expression::NFExpression::BINARY { exp1: inside_sum1.clone(), operator: Operator::makeDiv(Arc::new(crate::NFType::REAL)), exp2: inside_sum2.clone() });
            makeInStreamDivCall(sumExp.clone(), fallback.clone())?
        },
        (false, true) => {
            outside_sum1 = sumMap(outsideElements.clone(), (std::sync::Arc::new(sumOutside1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>), flowThreshold.clone(), variables.clone())?;
            outside_sum2 = sumMap(outsideElements.clone(), (std::sync::Arc::new(sumOutside2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>), flowThreshold.clone(), variables.clone())?;
            sumExp = Arc::new(Expression::NFExpression::BINARY { exp1: outside_sum1.clone(), operator: Operator::makeDiv(Arc::new(crate::NFType::REAL)), exp2: outside_sum2.clone() });
            makeInStreamDivCall(sumExp.clone(), fallback.clone())?
        },
        (false, false) => {
            outside_sum1 = sumMap(outsideElements.clone(), (std::sync::Arc::new(sumOutside1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>), flowThreshold.clone(), variables.clone())?;
            outside_sum2 = sumMap(outsideElements.clone(), (std::sync::Arc::new(sumOutside2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>), flowThreshold.clone(), variables.clone())?;
            inside_sum1 = sumMap(insideElements.clone(), (std::sync::Arc::new(sumInside1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>), flowThreshold.clone(), variables.clone())?;
            inside_sum2 = sumMap(insideElements.clone(), (std::sync::Arc::new(sumInside2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>), flowThreshold.clone(), variables.clone())?;
            sumExp = Arc::new(Expression::NFExpression::BINARY { exp1: Arc::new(Expression::NFExpression::BINARY { exp1: outside_sum1.clone(), operator: Operator::makeAdd(Arc::new(crate::NFType::REAL)), exp2: inside_sum1.clone() }), operator: Operator::makeDiv(Arc::new(crate::NFType::REAL)), exp2: Arc::new(Expression::NFExpression::BINARY { exp1: outside_sum2.clone(), operator: Operator::makeAdd(Arc::new(crate::NFType::REAL)), exp2: inside_sum2.clone() }) });
            makeInStreamDivCall(sumExp.clone(), fallback.clone())?
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(sumExp)
}

fn sumMap(mut elements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut flowThreshold: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = func(listHead(elements.clone())?, flowThreshold.clone(), variables.clone())?;
    for mut e in &*listRest(elements.clone())? {
        let mut e = e.clone();
        exp = Arc::new(Expression::NFExpression::BINARY { exp1: func(e.clone(), flowThreshold.clone(), variables.clone())?, operator: Operator::makeAdd(Arc::new(crate::NFType::REAL)), exp2: exp.clone() });
    }
    Ok(exp)
}

fn streamFlowExp(mut element: Arc<Connector::NFConnector>) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)> {
    let mut streamExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flowExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stream_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    stream_cr = Connector::name(element.clone());
    streamExp = Expression::fromCref(stream_cr.clone(), false)?;
    flowExp = Expression::fromCref(associatedFlowCref(stream_cr.clone())?, false)?;
    Ok((streamExp, flowExp))
}

fn flowExp(mut element: Arc<Connector::NFConnector>) -> Result<Arc<Expression::NFExpression>> {
    let mut flowExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    flow_cr = associatedFlowCref(Connector::name(element.clone()))?;
    flowExp = Expression::fromCref(flow_cr.clone(), false)?;
    Ok(flowExp)
}

fn sumOutside1(mut element: Arc<Connector::NFConnector>, mut flowThreshold: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stream_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (stream_exp, flow_exp) = streamFlowExp(element.clone())?;
    exp = Arc::new(Expression::NFExpression::BINARY { exp1: makePositiveMaxCall(flow_exp.clone(), stream_exp.clone(), element.clone(), flowThreshold.clone(), variables.clone())?, operator: Operator::makeMul(Arc::new(crate::NFType::REAL)), exp2: makeInStreamCall(stream_exp.clone())? });
    Ok(exp)
}

fn sumInside1(mut element: Arc<Connector::NFConnector>, mut flowThreshold: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stream_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_threshold: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (stream_exp, flow_exp) = streamFlowExp(element.clone())?;
    flow_exp = Arc::new(Expression::NFExpression::UNARY { operator: Operator::makeUMinus(Arc::new(crate::NFType::REAL)), exp: flow_exp.clone() });
    exp = Arc::new(Expression::NFExpression::BINARY { exp1: makePositiveMaxCall(flow_exp.clone(), stream_exp.clone(), element.clone(), flowThreshold.clone(), variables.clone())?, operator: Operator::makeMul(Arc::new(crate::NFType::REAL)), exp2: stream_exp.clone() });
    Ok(exp)
}

fn sumOutside2(mut element: Arc<Connector::NFConnector>, mut flowThreshold: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stream_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (stream_exp, flow_exp) = streamFlowExp(element.clone())?;
    exp = makePositiveMaxCall(flow_exp.clone(), stream_exp.clone(), element.clone(), flowThreshold.clone(), variables.clone())?;
    Ok(exp)
}

fn sumInside2(mut element: Arc<Connector::NFConnector>, mut flowThreshold: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stream_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (stream_exp, flow_exp) = streamFlowExp(element.clone())?;
    flow_exp = Arc::new(Expression::NFExpression::UNARY { operator: Operator::makeUMinus(Arc::new(crate::NFType::REAL)), exp: flow_exp.clone() });
    exp = makePositiveMaxCall(flow_exp.clone(), stream_exp.clone(), element.clone(), flowThreshold.clone(), variables.clone())?;
    Ok(exp)
}

fn makeInStreamCall(mut streamExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut inStreamCall: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    inStreamCall = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::IN_STREAM().clone(), list![streamExp.clone()], Expression::variability(streamExp.clone())?, Purity::PURE.clone(), NFBuiltinFuncs::IN_STREAM().returnType.clone()) });
    Ok(inStreamCall)
}

fn makePositiveMaxCall(mut flowExp: Arc<Expression::NFExpression>, mut streamExp: Arc<Expression::NFExpression>, mut element: Arc<Connector::NFConnector>, mut flowThreshold: Arc<Expression::NFExpression>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut positiveMaxCall: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut nominal_oexp: Option<Arc<Expression::NFExpression>> = None;
    let mut nominal_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_threshold: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fn_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    flow_name = associatedFlowCref(Connector::name(element.clone()))?;
    nominal_oexp = lookupVarAttr(flow_name.clone(), (literal!("nominal")).clone(), variables.clone())?;
    if isSome(nominal_oexp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(nominal_oexp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        nominal_exp = __pa0.clone();
        flow_threshold = Arc::new(Expression::NFExpression::BINARY { exp1: flowThreshold.clone(), operator: Operator::makeMul(Arc::new(crate::NFType::REAL)), exp2: nominal_exp.clone() });
    } else {
        flow_threshold = flowThreshold.clone();
    }
    if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
        (fn_node, _) = Class::lookupElement((literal!("$OMC$PositiveMax")).clone(), InstNode::getClass(InstNode::topScope(ComponentRef::node(flow_name.clone())?))?)?;
        fn_node = Function::instFunctionNode(fn_node.clone(), NFInstContext::NO_CONTEXT.clone(), Absyn::dummyInfo.clone())?;
        let __pa1 = ::match_deref::match_deref! { match &(Function::typeNodeCache(fn_node.clone(), NFInstContext::FUNCTION.clone())?) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        r#fn = __pa1.clone();
        positiveMaxCall = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![flowExp.clone(), flow_threshold.clone()], Connector::variability(element.clone()), Purity::PURE.clone(), r#fn.returnType.clone()) });
    } else {
        positiveMaxCall = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::POSITIVE_MAX_REAL().clone(), list![flowExp.clone(), flow_threshold.clone()], Connector::variability(element.clone()), Purity::PURE.clone(), NFBuiltinFuncs::POSITIVE_MAX_REAL().returnType.clone()) });
    }
    openmodelica_util::Globals::isInStream.with(|__root| *__root.borrow_mut() = Some(true));
    Ok(positiveMaxCall)
}

fn makeInStreamDivCall(mut sum_exp: Arc<Expression::NFExpression>, mut fallback: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut inStreamDivCall: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
        inStreamDivCall = sum_exp.clone();
    } else {
        inStreamDivCall = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INSTREAM_DIV_REAL().clone(), list![sum_exp.clone(), fallback.clone()], Expression::variability(fallback.clone())?, Purity::PURE.clone(), NFBuiltinFuncs::INSTREAM_DIV_REAL().returnType.clone()) });
    }
    Ok(inStreamDivCall)
}

fn isStreamCall(mut exp: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut streamCall: bool = false;
    streamCall = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } => {
            (::match_deref::match_deref! { match &(Function::name(Call::typedFunction(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?)) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" } => true,
        Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(streamCall)
}

fn evaluateOperatorReductionExp(mut exp: Arc<Expression::NFExpression>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<Expression::NFExpression>> {
    let mut evalExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut call: Arc<Call::NFCall>;
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iter_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut iter_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    evalExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
            ty = Expression::typeOf(var_field!((**call).exp, Call::NFCall::TYPED_REDUCTION).clone());
            for mut iter in &*var_field!((**call).iters, Call::NFCall::TYPED_REDUCTION).clone() {
                let mut iter = iter.clone();
                (iter_node, iter_exp) = iter.clone();
                if Component::variability(InstNode::component(iter_node.clone())?) > Variability::PARAMETER.clone() {
                    println!("{}", (literal!("Iteration range in reduction containing connector operator calls must be a parameter expression.")).clone());
                    bail!("fail");
                }
                iter_exp = Ceval::evalExp(iter_exp.clone(), Ceval::noTarget().clone())?;
                ty = Type::liftArrayLeftList(ty.clone(), Type::arrayDims(Expression::typeOf(iter_exp.clone())));
                iters = cons((iter_node.clone(), iter_exp.clone()), iters.clone());
            }
            iters = iters.clone().reverse();
            (arg, _) = ExpandExp::expandArrayConstructor(var_field!((**call).exp, Call::NFCall::TYPED_REDUCTION).clone(), ty.clone(), iters.clone())?;
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(var_field!((**call).r#fn, Call::NFCall::TYPED_REDUCTION).clone(), list![arg.clone()], var_field!((**call).var, Call::NFCall::TYPED_REDUCTION).clone(), Purity::PURE.clone(), var_field!((**call).ty, Call::NFCall::TYPED_REDUCTION).clone()) })
        },
        _ => bail!("match: no arm matched"),
    } });
    evalExp = evaluateOperators(evalExp.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
    Ok(evalExp)
}

fn evaluateOperatorArrayConstructorExp(mut exp: Arc<Expression::NFExpression>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<Expression::NFExpression>> {
    let mut evalExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    (evalExp, expanded) = ExpandExp::expand(exp.clone(), false, false)?;
    if !(expanded.clone()) {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConnectEquations.evaluateOperatorArrayConstructorExp")); __mm_s.push_str(&*literal!(" failed to expand call containing stream operator: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    evalExp = evaluateOperators(evalExp.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
    Ok(evalExp)
}

fn evaluateInStream(mut cref: Arc<ComponentRef::NFComponentRef>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut sl: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut set: i32 = 0;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    cr = ComponentRef::evaluateSubscripts(cref.clone())?;
    c = Arc::new(Connector::NFConnector { name: cr.clone(), ty: Arc::new(crate::NFType::UNKNOWN), face: Face::INSIDE.clone(), cty: ConnectorType::STREAM.clone(), source: DAE::emptyElementSource().clone() });
    match '__try0: {
        set = ConnectionSets::findSetArrayIndex(c.clone(), sets.clone());
        sl = setsArray.clone().borrow()[(set.clone()-1) as usize].clone();
        Ok::<_, anyhow::Error>((sl.clone(),))
    } {
        Ok((__try0_o0,)) => {
            sl = __try0_o0;
        }
        Err(_) => {
            sl = list![c.clone()];
        }
    }
    exp = generateInStreamExp(cr.clone(), sl.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone(), Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())?)?;
    Ok(exp)
}

fn generateInStreamExp(mut streamCref: Arc<ComponentRef::NFComponentRef>, mut streams: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>, mut flowThreshold: metamodelica::Real) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut reducedStreams: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut inside: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut outside: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut f1: Face = Face::INSIDE;
    let mut f2: Face = Face::INSIDE;
    reducedStreams = {
        let mut __acc: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
        for mut s in (streams.clone()).into_iter().cloned() {
            if !(!(isNoFlowMinMax(s.clone(), streamCref.clone(), variables.clone())?)) { continue; }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    exp = (::match_deref::match_deref! { match &(reducedStreams.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { face: Connector::Face::INSIDE, .. }, tail: Deref @ metamodelica::List::Nil } => Expression::fromCref(streamCref.clone(), false)?,
        Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { face: Connector::Face::INSIDE, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { face: Connector::Face::INSIDE, .. }, tail: Deref @ metamodelica::List::Nil } } => {
            let __pa0 = ::match_deref::match_deref! { match &(removeStreamSetElement(streamCref.clone(), reducedStreams.clone())?) {
                Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { name: __pa0, .. }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            Expression::fromCref(cr.clone(), false)?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { face: f1, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { face: f2, .. }, tail: Deref @ metamodelica::List::Nil } } if (f1.clone() != f2.clone()) => {
            let __pa0 = ::match_deref::match_deref! { match &(removeStreamSetElement(streamCref.clone(), reducedStreams.clone())?) {
                Deref @ metamodelica::List::Cons { head: Deref @ Connector::CONNECTOR { name: __pa0, .. }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            evaluateInStream(cr.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?
        },
        _ => {
            (outside, inside) = List::splitOnTrue(reducedStreams.clone(), (std::sync::Arc::new(fnptr!(Connector::isOutside, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>) -> Result<bool> + 'static>));
            inside = removeStreamSetElement(streamCref.clone(), inside.clone())?;
            exp = streamSumEquationExp(outside.clone(), inside.clone(), Arc::new(Expression::NFExpression::REAL { value: flowThreshold.clone() }), Expression::fromCref(streamCref.clone(), false)?, variables.clone())?;
            exp = evaluateOperators(exp.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn isNoFlowMinMax(mut conn: Arc<Connector::NFConnector>, mut streamCref: Arc<ComponentRef::NFComponentRef>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<bool> {
    let mut noFlow: bool = false;
    if ComponentRef::isEqual(streamCref.clone(), conn.name.clone())? {
        noFlow = false;
    } else if Connector::isOutside(conn.clone()) {
        noFlow = isNoFlowOutside(conn.clone(), variables.clone())?;
    } else {
        noFlow = isNoFlowInside(conn.clone(), variables.clone())?;
    }
    Ok(noFlow)
}

fn isNoFlowOutside(mut conn: Arc<Connector::NFConnector>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<bool> {
    let mut noFlow: bool = false;
    noFlow = isNoFlow(conn.clone(), (literal!("max")).clone(), (std::sync::Arc::new(fnptr!(Expression::isNonPositive, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>), variables.clone())?;
    Ok(noFlow)
}

fn isNoFlowInside(mut conn: Arc<Connector::NFConnector>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<bool> {
    let mut noFlow: bool = false;
    noFlow = isNoFlow(conn.clone(), (literal!("min")).clone(), (std::sync::Arc::new(fnptr!(Expression::isNonNegative, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>), variables.clone())?;
    Ok(noFlow)
}

fn isNoFlow(mut element: Arc<Connector::NFConnector>, mut attr: ArcStr, mut pred: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<bool> {
    pub type FlowPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut noFlow: bool = false;
    let mut flow_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut attr_oexp: Option<Arc<Expression::NFExpression>> = None;
    let mut attr_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Variability = Variability::CONSTANT;
    flow_name = Expression::toCref(flowExp(element.clone())?)?;
    attr_oexp = lookupVarAttr(flow_name.clone(), (attr.clone()).clone(), variables.clone())?;
    if isSome(attr_oexp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(attr_oexp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        attr_exp = __pa0.clone();
        var = Expression::variability(attr_exp.clone())?;
        if var.clone() == Variability::PARAMETER.clone() && !(Structural::isExpressionNotFixed(attr_exp.clone(), false, 4)?) {
            Structural::markExp(attr_exp.clone())?;
            var = Variability::STRUCTURAL_PARAMETER.clone();
        }
        if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
            attr_exp = Ceval::evalExp(attr_exp.clone(), Ceval::noTarget().clone())?;
        }
        noFlow = pred(attr_exp.clone())?;
    } else {
        noFlow = false;
    }
    Ok(noFlow)
}

fn evaluateActualStream(mut streamCref: Arc<ComponentRef::NFComponentRef>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<(Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flowCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut stream_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut flow_dir: i32 = 0;
    let mut flow_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stream_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut instream_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    stream_cref = ComponentRef::evaluateSubscripts(streamCref.clone())?;
    flowCref = associatedFlowCref(stream_cref.clone())?;
    flow_dir = evaluateFlowDirection(flowCref.clone(), variables.clone())?;
    if flow_dir.clone() == 1 {
        exp = evaluateInStream(stream_cref.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
    } else if flow_dir.clone() == -1 {
        exp = Expression::fromCref(stream_cref.clone(), false)?;
    } else {
        flow_exp = Expression::fromCref(flowCref.clone(), false)?;
        stream_exp = Expression::fromCref(stream_cref.clone(), false)?;
        instream_exp = evaluateInStream(stream_cref.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
        op = Operator::makeGreater(ComponentRef::nodeType(flowCref.clone())?);
        exp = Arc::new(Expression::NFExpression::IF { ty: Arc::new(crate::NFType::REAL), condition: Arc::new(Expression::NFExpression::RELATION { exp1: flow_exp.clone(), operator: op.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }), index: -1 }), trueBranch: instream_exp.clone(), falseBranch: stream_exp.clone() });
    }
    Ok((exp, flowCref))
}

fn evaluateActualStreamMul(mut crefExp: Arc<Expression::NFExpression>, mut actualStreamArg: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut sets: ConnectionSets::Sets, mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>, mut ctable: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut flow_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(evaluateOperators(crefExp.clone(), sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?) {
        __pa1 @ Deref @ Expression::CREF { cref: __pa0, .. } => (__pa1.clone(), __pa0.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cr = __pa0.clone();
    e1 = __pa1.clone();
    (e2, flow_cr) = evaluateActualStream(Expression::toCref(actualStreamArg.clone())?, sets.clone(), setsArray.clone(), variables.clone(), ctable.clone())?;
    outExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
    outExp = (::match_deref::match_deref! { match &(e2.clone()) {
        Deref @ Expression::IF { .. } if (ComponentRef::isEqual(cr.clone(), flow_cr.clone())?) => makeSmoothCall(outExp.clone(), 0)?,
        _ => outExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn evaluateFlowDirection(mut flowCref: Arc<ComponentRef::NFComponentRef>, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<i32> {
    let mut direction: i32 = 0;
    let mut omin: Option<Arc<Expression::NFExpression>> = None;
    let mut omax: Option<Arc<Expression::NFExpression>> = None;
    let mut min_val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut max_val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    omin = lookupVarAttr(flowCref.clone(), (literal!("min")).clone(), variables.clone())?;
    omin = Util::applyOption(omin.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone()) }));
    omax = lookupVarAttr(flowCref.clone(), (literal!("max")).clone(), variables.clone())?;
    omax = Util::applyOption(omax.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone()) }));
    direction = (::match_deref::match_deref! { match &((omin.clone(), omax.clone())) {
        (None, None) => 0,
        (Some(Deref @ Expression::REAL { value: min_val }), None) => if (min_val.clone() >= metamodelica::OrderedFloat((0) as f64)) {1} else {0},
        (None, Some(Deref @ Expression::REAL { value: max_val })) => if (max_val.clone() <= metamodelica::OrderedFloat((0) as f64)) {-1} else {0},
        (Some(Deref @ Expression::REAL { value: min_val }), Some(Deref @ Expression::REAL { value: max_val })) => if (min_val.clone() >= metamodelica::OrderedFloat((0) as f64) && max_val.clone() >= min_val.clone()) {1} else if (max_val.clone() <= metamodelica::OrderedFloat((0) as f64) && min_val.clone() <= max_val.clone()) {-1} else {0},
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(direction)
}

fn makeSmoothCall(mut arg: Arc<Expression::NFExpression>, mut order: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::SMOOTH().clone(), list![Arc::new(Expression::NFExpression::INTEGER { value: order.clone() }), arg.clone()], Expression::variability(arg.clone())?, Purity::PURE.clone(), NFBuiltinFuncs::SMOOTH().returnType.clone()) });
    Ok(callExp)
}

fn removeStreamSetElement(mut cref: Arc<ComponentRef::NFComponentRef>, mut elements: Arc<metamodelica::List<Arc<Connector::NFConnector>>>) -> Result<Arc<metamodelica::List<Arc<Connector::NFConnector>>>> {
    let mut elements: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = elements;
    (elements, _) = List::deleteMemberOnTrue(cref.clone(), elements.clone(), (std::sync::Arc::new(compareCrefStreamSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<Connector::NFConnector>) -> Result<bool> + 'static>))?;
    Ok(elements)
}

fn compareCrefStreamSet(mut cref: Arc<ComponentRef::NFComponentRef>, mut element: Arc<Connector::NFConnector>) -> Result<bool> {
    let mut matches: bool = false;
    matches = ComponentRef::isEqual(cref.clone(), element.name.clone())?;
    Ok(matches)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn associatedFlowCref(mut streamCref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut flowCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut rest_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut flow_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(streamCref.clone()) {
        Deref @ ComponentRef::CREF { restCref: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    rest_cr = __pa0.clone();
    ty = __pa1.clone();
    flowCref = (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::CONNECTOR { flows: Deref @ metamodelica::List::Cons { head: flow_node, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => ComponentRef::prefixCref(flow_node.clone(), InstNode::getType(flow_node.clone())?, metamodelica::nil(), streamCref.clone()),
        _ => associatedFlowCref(rest_cr.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(flowCref)
}

fn lookupVarAttr(mut varName: Arc<ComponentRef::NFComponentRef>, mut attrName: ArcStr, mut variables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Variable::NFVariable>>>) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut attrValue: Option<Arc<Expression::NFExpression>> = None;
    let mut ovar: Option<Arc<Variable::NFVariable>> = None;
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    ovar = UnorderedMap::get(varName.clone(), variables.clone());
    if isNone(ovar.clone()) {
        ovar = UnorderedMap::get(ComponentRef::stripSubscriptsAll(varName.clone()), variables.clone());
    }
    if isNone(ovar.clone()) {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConnectEquations.lookupVarAttr")); __mm_s.push_str(&*literal!(" could not find the variable ")); __mm_s.push_str(&*ComponentRef::toString(varName.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    let __pa0 = ::match_deref::match_deref! { match &(ovar.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    binding = Variable::lookupTypeAttribute((attrName.clone()).clone(), var.clone());
    attrValue = Binding::typedExp(binding.clone());
    Ok(attrValue)
}

