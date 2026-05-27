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

use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBModule as Module;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:         NBInline.mo
///  package:      NBInline
///  description:  This file contains functions for inlining operations.
pub struct NBInline<T>(std::marker::PhantomData<T>);
pub fn inlineForEquation(mut eqn: Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: new_eqn, tail: Deref @ metamodelica::List::Nil }, .. } if (BEquation::Iterator::size(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), false) == 1 && !(BEquation::Iterator::isResizable(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?)) => {
            let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut start: i32 = 0;
            let mut new_eqn = (*new_eqn).clone();
            replacements = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            (names, ranges, _) = BEquation::Iterator::getFrames(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            for mut tpl in &*List::zip(names.clone(), ranges.clone()) {
                let mut tpl = tpl.clone();
                (name, range) = tpl.clone();
                (start, _, _) = Expression::getIntegerRange(range.clone(), true)?;
                UnorderedMap::add(name.clone(), Arc::new(Expression::NFExpression::INTEGER { value: start.clone() }), replacements.clone())?;
            }
            new_eqn = BEquation::Equation::map(new_eqn.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*literal!("NBInline.inlineForEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::toString(new_eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            new_eqn.clone()
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

pub fn functionInlineable(mut r#fn: Arc<Function::Function>) -> bool {
    let mut b: bool = false;
    if Function::hasSingleOrEmptyBody(r#fn.clone()) {
        b = (::match_deref::match_deref! { match &(Function::getBody(r#fn.clone())) {
        Deref @ metamodelica::List::Cons { head: Deref @ Statement::ASSIGNMENT { .. }, tail: Deref @ metamodelica::List::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    b
}

pub fn inlineRecordSliceEquation(mut slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>> {
    let mut slices: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut record_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    inlineRecordTupleArrayEquation(Pointer::access(Slice::getT(slice.clone())), Arc::new(crate::NBEquation::Iterator::EMPTY), variables.clone(), record_eqns.clone(), set.clone(), index.clone(), inlineSimple.clone())?;
    slices = {
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice>>> = metamodelica::nil();
        for mut eqn in (Pointer::access(record_eqns.clone())).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: eqn.clone(), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if slices.clone().is_empty() {
        slices = list![slice.clone()];
    }
    Ok(slices)
}

pub fn inlineArrayConstructorSingle(mut eqn: Arc<Equation::Equation>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>) -> Result<(Arc<Equation::Equation>, bool)> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut changed: bool = false;
    match '__try0: {
        (eqn, changed) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, lhs: lhs @ Deref @ Expression::CREF { .. }, .. } => {
            (unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0), true)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: rhs @ Deref @ Expression::CREF { .. }, lhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, .. } => {
            (unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0), true)
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut new_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            (new_eqn, changed) = unwrap_break_err!(inlineArrayConstructorSingle(body.clone(), var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), variables.clone(), set.clone(), index.clone(), new_eqns.clone()), '__try0);
            new_eqn = if (changed.clone()) {new_eqn.clone()} else {eqn.clone()};
            (new_eqn.clone(), changed.clone())
        },
        _ => {
            (eqn.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        eqn = if (BEquation::Equation::isDummy(eqn.clone())) {Pointer::access(listHead(Pointer::access(new_eqns.clone()))?)} else {eqn.clone()};
        Ok::<_, anyhow::Error>((changed.clone(), eqn.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            changed = __try0_o0;
            eqn = __try0_o1;
        }
        Err(_) => {
            changed = false;
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to inline following equation:\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone())?;
            }
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((eqn, changed))
}

fn inlineRecordsTuplesArrays(mut eqData: Arc<EqData::EqData>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut init: bool) -> Result<Arc<EqData::EqData>> {
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut index: Pointer::Pointer<i32> = BEquation::EqData::getUniqueIndex(eqData.clone())?;
    let mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    if init.clone() {
        eqData = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ BEquation::EqData::EQ_DATA_SIM { .. } => {
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM;
                initials = BEquation::EquationPointers::map(var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone(), Arc::new({ let __pe_b1 = Arc::new(crate::NBEquation::Iterator::EMPTY); let __pe_b2 = variables.clone(); let __pe_b3 = new_eqns.clone(); let __pe_b4 = set.clone(); let __pe_b5 = index.clone(); let __pe_b6 = false; move |__pe_a0| inlineRecordTupleArrayEquation(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }))?,
                initials = BEquation::EquationPointers::addList(Pointer::access(new_eqns.clone()), var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone()),
                initials = BEquation::EquationPointers::compress(var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone())?
            );
            eqData.clone()
        },
        _ => eqData.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        eqData = BEquation::EqData::map(eqData.clone(), Arc::new({ let __pe_b1 = Arc::new(crate::NBEquation::Iterator::EMPTY); let __pe_b2 = variables.clone(); let __pe_b3 = new_eqns.clone(); let __pe_b4 = set.clone(); let __pe_b5 = index.clone(); let __pe_b6 = false; move |__pe_a0| inlineRecordTupleArrayEquation(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }))?;
        eqData = BEquation::EqData::addUntypedList(eqData.clone(), Pointer::access(new_eqns.clone()), false)?;
        eqData = BEquation::EqData::compress(eqData.clone())?;
    }
    Ok(eqData)
}

pub fn inlineRecordTupleArrayEquation(mut eqn: Arc<Equation::Equation>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    match '__try0: {
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::RECORD_EQUATION { rhs: Deref @ Expression::CREF { .. }, lhs: Deref @ Expression::CREF { .. }, .. } if (!(inlineSimple.clone())) => {
            eqn.clone()
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: Deref @ Expression::CREF { .. }, lhs: Deref @ Expression::CREF { .. }, .. } if (!(inlineSimple.clone())) => {
            eqn.clone()
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { ty: Deref @ Type::COMPLEX { .. }, .. } => {
            unwrap_break_err!(inlineRecordEquation(eqn.clone(), var_field!((*eqn).lhs, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::RECORD_EQUATION).clone(), iter.clone(), var_field!((*eqn).attr, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).recordSize, Equation::Equation::RECORD_EQUATION).clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), inlineSimple.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { recordSize: Some(size), .. } => {
            unwrap_break_err!(inlineRecordEquation(eqn.clone(), var_field!((*eqn).lhs, Equation::Equation::ARRAY_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), size.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), inlineSimple.clone()), '__try0)
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { .. } => {
            unwrap_break_err!(inlineTupleEquation(eqn.clone(), var_field!((*eqn).lhs, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).attr, Equation::Equation::RECORD_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: rhs @ Deref @ Expression::ARRAY { .. }, lhs: lhs @ Deref @ Expression::ARRAY { .. }, .. } => {
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone(), var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: rhs @ Deref @ Expression::ARRAY { .. }, lhs: lhs @ Deref @ Expression::CREF { .. }, .. } => {
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            let mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            dim = unwrap_break_err!(listHead(Type::arrayDims(var_field!((**lhs).ty, Expression::NFExpression::CREF).clone())), '__try0);
            elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (1..=(var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)).into_iter() {
            let __x = unwrap_break_err!(Expression::applySubscripts(list![Subscript::nth(dim.clone(), i.clone())?], lhs.clone(), true), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), metamodelica::arrayFromVec(elements.clone().into_iter().cloned().collect()), var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: rhs @ Deref @ Expression::CREF { .. }, lhs: lhs @ Deref @ Expression::ARRAY { .. }, .. } => {
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            let mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            dim = unwrap_break_err!(listHead(Type::arrayDims(var_field!((**rhs).ty, Expression::NFExpression::CREF).clone())), '__try0);
            elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (1..=(var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)).into_iter() {
            let __x = unwrap_break_err!(Expression::applySubscripts(list![Subscript::nth(dim.clone(), i.clone())?], rhs.clone(), true), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone(), metamodelica::arrayFromVec(elements.clone().into_iter().cloned().collect()), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, lhs: lhs @ Deref @ Expression::CREF { .. }, .. } => {
            unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: rhs @ Deref @ Expression::CREF { .. }, lhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, .. } => {
            unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: Deref @ Expression::CALL { call }, lhs: lhs @ Deref @ Expression::CREF { .. }, .. } if (unwrap_break_err!(AbsynUtil::pathString(Function::nameConsiderBuiltin(Call::typedFunction(call.clone())?)?, (literal!(".")).clone(), true, false), '__try0) == literal!("cat")) => {
            unwrap_break_err!(inlineCatCall(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), Call::arguments(call.clone())?, var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: rhs @ Deref @ Expression::CREF { .. }, lhs: Deref @ Expression::CALL { call }, .. } if (unwrap_break_err!(AbsynUtil::pathString(Function::nameConsiderBuiltin(Call::typedFunction(call.clone())?)?, (literal!(".")).clone(), true, false), '__try0) == literal!("cat")) => {
            unwrap_break_err!(inlineCatCall(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), Call::arguments(call.clone())?, var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: Deref @ Expression::CALL { call }, lhs: lhs @ Deref @ Expression::CREF { .. }, .. } if (unwrap_break_err!(AbsynUtil::pathString(Function::nameConsiderBuiltin(Call::typedFunction(call.clone())?)?, (literal!(".")).clone(), true, false), '__try0) == literal!("promote")) => {
            unwrap_break_err!(inlinePromoteCall(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), Call::arguments(call.clone())?, var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: rhs @ Deref @ Expression::CREF { .. }, lhs: Deref @ Expression::CALL { call }, .. } if (unwrap_break_err!(AbsynUtil::pathString(Function::nameConsiderBuiltin(Call::typedFunction(call.clone())?)?, (literal!(".")).clone(), true, false), '__try0) == literal!("promote")) => {
            unwrap_break_err!(inlinePromoteCall(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), Call::arguments(call.clone())?, var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut new_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            new_eqn = unwrap_break_err!(inlineRecordTupleArrayEquation(body.clone(), var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), true), '__try0);
            new_eqn = if (BEquation::Equation::isDummy(new_eqn.clone())) {new_eqn.clone()} else {eqn.clone()};
            new_eqn.clone()
        },
        Deref @ BEquation::Equation::IF_EQUATION { .. } if (BEquation::IfEquationBody::isRecordOrTupleEquation(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone())) => {
            let mut new_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            new_eqn = unwrap_break_err!(inlineRecordTupleArrayIfEquation(eqn.clone(), var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), inlineSimple.clone()), '__try0);
            new_eqn = if (BEquation::Equation::isDummy(new_eqn.clone())) {new_eqn.clone()} else {eqn.clone()};
            new_eqn.clone()
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok::<_, anyhow::Error>((eqn.clone(),))
    } {
        Ok((__try0_o0,)) => {
            eqn = __try0_o0;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to inline following equation:\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone())?;
            }
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(eqn)
}

fn inlineRecordTupleArrayIfEquation(mut eqn: Arc<Equation::Equation>, mut body: Arc<IfEquationBody::IfEquationBody>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut new_body: Arc<IfEquationBody::IfEquationBody>;
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    eqns = Pointer::access(new_eqns.clone());
    new_body = inlineRecordTupleArrayIfBody(body.clone(), iter.clone(), variables.clone(), set.clone(), index.clone(), inlineSimple.clone())?;
    for mut b in &*BEquation::IfEquationBody::split(new_body.clone())? {
        let mut b = b.clone();
        new_eqn = BEquation::IfEquationBody::makeIfEquation(b.clone(), index.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), iter.clone(), BEquation::Equation::getSource(eqn.clone()), BEquation::Equation::getAttributes(eqn.clone()))?;
        eqns = cons(new_eqn.clone(), eqns.clone());
    }
    Pointer::update(new_eqns.clone(), eqns.clone());
    eqn = Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION);
    Ok(eqn)
}

fn inlineRecordTupleArrayIfBody(mut body: Arc<IfEquationBody::IfEquationBody>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<IfEquationBody::IfEquationBody>> {
    let mut body: Arc<IfEquationBody::IfEquationBody> = body;
    let mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    assign_field!(
        body.then_eqns = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut e in (body.then_eqns.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(inlineRecordTupleArrayEquation(Pointer::access(e.clone()), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), inlineSimple.clone())?) {
        Deref @ BEquation::Equation::DUMMY_EQUATION => Pointer::access(new_eqns.clone()),
        _ => list![e.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        body.else_if = Util::applyOption(body.else_if.clone(), Arc::new({ let __pe_b1 = iter.clone(); let __pe_b2 = variables.clone(); let __pe_b3 = set.clone(); let __pe_b4 = index.clone(); let __pe_b5 = inlineSimple.clone(); move |__pe_a0| inlineRecordTupleArrayIfBody(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))
    );
    Ok(body)
}

fn inlineRecordEquation(mut eqn: Arc<Equation::Equation>, mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut iter: Arc<Iterator::Iterator>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut recordSize: i32, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut new_lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineRecordEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqns = Pointer::access(new_eqns.clone());
    for mut i in 1..=recordSize.clone() {
        new_lhs = inlineRecordConstructorExp(lhs.clone(), i.clone(), variables.clone())?;
        new_rhs = inlineRecordConstructorExp(rhs.clone(), i.clone(), variables.clone())?;
        eqns = createInlinedEquation(eqns.clone(), new_lhs.clone(), new_rhs.clone(), attr.clone(), iter.clone(), variables.clone(), set.clone(), index.clone())?;
    }
    Pointer::update(new_eqns.clone(), eqns.clone());
    eqn = Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION);
    Ok(eqn)
}

fn inlineTupleEquation(mut eqn: Arc<Equation::Equation>, mut LHS: Arc<Expression::NFExpression>, mut RHS: Arc<Expression::NFExpression>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut lhs_elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut rhs_elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tmp_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    lhs_elems = getElementList(LHS.clone())?;
    rhs_elems = getElementList(RHS.clone())?;
    if !(lhs_elems.clone().is_empty()) && List::compareLength(lhs_elems.clone(), rhs_elems.clone())? == 0 {
        if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineTupleEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
            if !(BEquation::Iterator::isEmpty(iter.clone())) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
            }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        eqns = Pointer::access(new_eqns.clone());
        for mut tpl in &*List::zip(lhs_elems.clone(), rhs_elems.clone()) {
            let mut tpl = tpl.clone();
            (lhs, rhs) = tpl.clone();
            if !(Expression::isWildCref(lhs.clone()) || Expression::isWildCref(rhs.clone())) {
                eqns = createInlinedEquation(eqns.clone(), lhs.clone(), rhs.clone(), attr.clone(), iter.clone(), variables.clone(), set.clone(), index.clone())?;
            }
        }
        Pointer::update(new_eqns.clone(), eqns.clone());
        eqn = Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION);
    }
    Ok(eqn)
}

fn inlineArrayEquation(mut eqn: Arc<Equation::Equation>, mut lhs_elements: metamodelica::Array<Arc<Expression::NFExpression>>, mut rhs_elements: metamodelica::Array<Arc<Expression::NFExpression>>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut tmp_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>;
    let mut inlined: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineArrayEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqns = Pointer::access(new_eqns.clone());
    let __range0 = 1..=(lhs_elements.clone().borrow().len() as i32);
    for mut i in __range0 {
        eqns = createInlinedEquation(eqns.clone(), lhs_elements.borrow()[(i.clone()-1) as usize].clone(), rhs_elements.borrow()[(i.clone()-1) as usize].clone(), attr.clone(), iter.clone(), variables.clone(), set.clone(), index.clone())?;
    }
    Pointer::update(new_eqns.clone(), eqns.clone());
    eqn = Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION);
    Ok(eqn)
}

fn inlineArrayConstructor(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>, mut rhs: Arc<Expression::NFExpression>, mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut cref_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut local_set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
    let mut local_it: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineArrayConstructor")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqns = Pointer::access(new_eqns.clone());
    frames = {
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
        for mut iter in (iters.clone()).into_iter().cloned() {
            let __x = BEquation::Iterator::createFrame(iter.clone(), local_set.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    let _ = UnorderedSet::merge(set.clone(), local_set.clone())?;
    subs = BEquation::Iterator::normalizedSubscripts(BEquation::Iterator::fromFrames(frames.clone()), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?;
    cref_exp = Expression::fromCref(ComponentRef::mergeSubscripts(subs.clone(), cref.clone(), true, false, false)?, false)?;
    local_it = BVariable::VariablePointers::fromList(UnorderedSet::toList(local_set.clone()), false);
    cref_exp = Expression::map(cref_exp.clone(), Arc::new({ let __pe_b1 = local_it.clone(); let __pe_b2 = false; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    new_rhs = Expression::map(rhs.clone(), Arc::new({ let __pe_b1 = local_it.clone(); let __pe_b2 = false; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    eqns = createInlinedEquation(eqns.clone(), cref_exp.clone(), new_rhs.clone(), attr.clone(), BEquation::Iterator::addFrames(iter.clone(), frames.clone())?, variables.clone(), set.clone(), index.clone())?;
    Pointer::update(new_eqns.clone(), eqns.clone());
    eqn = Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION);
    Ok(eqn)
}

fn inlinePromoteCall(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: i32 = 0;
    let mut dim_count: i32 = 0;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlinePromoteCall")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __pa1 }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    arg = __pa0.clone();
    n = __pa1.clone();
    eqn = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { .. } => {
            dim_count = Type::dimensionCount(ComponentRef::getSubscriptedType(var_field!((*arg).cref, Expression::NFExpression::CREF).clone(), false)?);
            if n.clone() == dim_count.clone() {
                lhs = Expression::fromCref(cref.clone(), false)?;
            } else {
                subs = Subscript::fillWithWholeLeft(List::fill(Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) }), n.clone() - dim_count.clone()), n.clone());
                lhs = Expression::fromCref(ComponentRef::mergeSubscripts(subs.clone(), cref.clone(), false, false, false)?, false)?;
            }
            new_eqn = BEquation::Equation::makeAssignment(lhs.clone(), arg.clone(), index.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), iter.clone(), attr.clone())?;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            Pointer::access(new_eqn.clone())
        },
        _ => eqn.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

fn inlineCatCall(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut n: i32 = 0;
    let mut sz: i32 = 0;
    let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut iterator_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut iterator_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut update_vars: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut subscript_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lhs_sub: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs_sub: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut shift: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_size: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut local_iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut failed: bool = false;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineCatCall")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqns = Pointer::access(new_eqns.clone());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __pa0 }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    rest = __pa1.clone();
    iterator_name = ComponentRef::makeIterator(InstNode::newUniqueIterator(Absyn::dummyInfo.clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER)), Arc::new(openmodelica_nf_frontend::NFType::INTEGER));
    iterator_var = BackendDAE::lowerIterator(iterator_name.clone());
    iterator_name = BVariable::getVarName(iterator_var.clone());
    update_vars = BVariable::VariablePointers::fromList(list![iterator_var.clone()], false);
    UnorderedSet::add(iterator_var.clone(), set.clone())?;
    subscript_exp = Expression::fromCref(iterator_name.clone(), false)?;
    shift = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    for mut arg in &*rest.clone() {
        let mut arg = arg.clone();
        failed = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { cref: rhs, .. } if (!(failed.clone())) => {
            let mut rhs = (*rhs).clone();
            ty = Expression::typeOf(arg.clone());
            if Type::isArray(ty.clone()) {
                dim = Type::nthDimension(ty.clone(), n.clone())?;
                sz = Dimension::size(dim.clone(), false)?;
                if sz.clone() != 1 || Dimension::isResizable(dim.clone()) {
                    new_size = Dimension::sizeExp(dim.clone())?;
                    range = Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), None, new_size.clone())?;
                    local_iter = BEquation::Iterator::addFrames(iter.clone(), list![(iterator_name.clone(), range.clone(), None)])?;
                    lhs_sub = if (Expression::isZero(shift.clone())) {subscript_exp.clone()} else {Arc::new(Expression::NFExpression::MULTARY { arguments: list![shift.clone(), subscript_exp.clone()], inv_arguments: metamodelica::nil(), operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::INTEGER)) })};
                    lhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: lhs_sub.clone() })], n.clone()), cref.clone(), false, false, false)?;
                    rhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: subscript_exp.clone() })], n.clone()), rhs.clone(), false, false, false)?;
                    lhs_exp = Expression::map(Expression::fromCref(lhs.clone(), false)?, Arc::new({ let __pe_b1 = update_vars.clone(); let __pe_b2 = false; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
                    rhs_exp = Expression::map(Expression::fromCref(rhs.clone(), false)?, Arc::new({ let __pe_b1 = update_vars.clone(); let __pe_b2 = false; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
                } else {
                    new_size = Arc::new(Expression::NFExpression::INTEGER { value: 1 });
                    lhs_sub = bumpShift(shift.clone(), new_size.clone())?;
                    lhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: lhs_sub.clone() })], n.clone()), cref.clone(), false, false, false)?;
                    lhs_exp = Expression::fromCref(lhs.clone(), false)?;
                    rhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) })], n.clone()), rhs.clone(), false, false, false)?;
                    rhs_exp = Expression::fromCref(rhs.clone(), false)?;
                    local_iter = iter.clone();
                }
            } else {
                new_size = Arc::new(Expression::NFExpression::INTEGER { value: 1 });
                lhs_sub = bumpShift(shift.clone(), new_size.clone())?;
                lhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: lhs_sub.clone() })], n.clone()), cref.clone(), false, false, false)?;
                lhs_exp = Expression::fromCref(lhs.clone(), false)?;
                rhs_exp = Expression::fromCref(rhs.clone(), false)?;
                local_iter = iter.clone();
            }
            new_eqn = BEquation::Equation::makeAssignment(lhs_exp.clone(), rhs_exp.clone(), index.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), local_iter.clone(), attr.clone())?;
            shift = bumpShift(shift.clone(), new_size.clone())?;
            eqns = cons(new_eqn.clone(), eqns.clone());
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            false
        },
        Deref @ Expression::ARRAY { .. } if (!(failed.clone()) && Expression::isLiteral(arg.clone())) => {
            (eqns, shift) = inlineCatCallLiterals(arg.clone(), cref.clone(), iter.clone(), attr.clone(), n.clone(), index.clone(), eqns.clone(), shift.clone(), metamodelica::nil())?;
            false
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if !(failed.clone()) {
        Pointer::update(new_eqns.clone(), eqns.clone());
        eqn = Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION);
    }
    Ok(eqn)
}

fn inlineCatCallLiterals(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut iter: Arc<Iterator::Iterator>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut n: i32, mut index: Pointer::Pointer<i32>, mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut shift: Arc<Expression::NFExpression>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<Expression::NFExpression>)> {
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = eqns;
    let mut shift: Arc<Expression::NFExpression> = shift;
    let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { .. } => {
            let mut sub_idx: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut is_cat_dim: bool = false;
            let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
            is_cat_dim = n.clone() == (subs.clone().len() as i32) + 1;
            sub_idx = if (is_cat_dim.clone()) {bumpShift(shift.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }))?} else {Arc::new(Expression::NFExpression::INTEGER { value: 1 })};
            let __range0 = var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut elem in __range0 {
                sub = Arc::new(Subscript::NFSubscript::INDEX { index: sub_idx.clone() });
                (eqns, shift) = inlineCatCallLiterals(elem.clone(), cref.clone(), iter.clone(), attr.clone(), n.clone(), index.clone(), eqns.clone(), shift.clone(), cons(sub.clone(), subs.clone()))?;
                sub_idx = bumpShift(sub_idx.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }))?;
            }
            if is_cat_dim.clone() {
                shift = bumpShift(shift.clone(), Arc::new(Expression::NFExpression::INTEGER { value: (var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32) }))?;
            }
            ()
        },
        _ => {
            let mut lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut lhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
            lhs = ComponentRef::mergeSubscripts(subs.clone().reverse(), cref.clone(), false, false, false)?;
            lhs_exp = Expression::fromCref(lhs.clone(), false)?;
            new_eqn = BEquation::Equation::makeAssignment(lhs_exp.clone(), exp.clone(), index.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), iter.clone(), attr.clone())?;
            eqns = cons(new_eqn.clone(), eqns.clone());
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqns, shift))
}

fn bumpShift(mut shift: Arc<Expression::NFExpression>, mut new_size: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut shift: Arc<Expression::NFExpression> = shift;
    shift = (::match_deref::match_deref! { match &((shift.clone(), new_size.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*shift).value, Expression::NFExpression::INTEGER).clone() + var_field!((*new_size).value, Expression::NFExpression::INTEGER).clone() })
        },
        (Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value }, tail: args }, .. }, Deref @ Expression::INTEGER { .. }) if (Operator::getMathClassification(var_field!((*shift).operator, Expression::NFExpression::MULTARY).clone())? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(shift => Expression::NFExpression::MULTARY; arguments = cons(Arc::new(Expression::NFExpression::INTEGER { value: value.clone() + var_field!((*new_size).value, Expression::NFExpression::INTEGER).clone() }), args.clone()));
            shift.clone()
        },
        (Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: args }, .. }, _) if (Operator::getMathClassification(var_field!((*shift).operator, Expression::NFExpression::MULTARY).clone())? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(shift => Expression::NFExpression::MULTARY; arguments = cons(arg.clone(), cons(new_size.clone(), args.clone())));
            shift.clone()
        },
        _ => {
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![shift.clone(), new_size.clone()], inv_arguments: metamodelica::nil(), operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::INTEGER)) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(shift)
}

fn createInlinedEquation(mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = eqns;
    let mut tmp_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    let mut inlined: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    new_eqn = BEquation::Equation::makeAssignment(lhs.clone(), rhs.clone(), index.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), iter.clone(), attr.clone())?;
    inlined = inlineRecordTupleArrayEquation(Pointer::access(new_eqn.clone()), iter.clone(), variables.clone(), tmp_eqns.clone(), set.clone(), index.clone(), false)?;
    eqns = (::match_deref::match_deref! { match &(inlined.clone()) {
        Deref @ BEquation::Equation::DUMMY_EQUATION => listAppend(eqns.clone(), Pointer::access(tmp_eqns.clone())),
        _ => {
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::toString(inlined.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            cons(new_eqn.clone(), eqns.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqns)
}

fn inlineRecordConstructorExp(mut exp: Arc<Expression::NFExpression>, mut index: i32, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::nthRecordElement(index.clone(), exp.clone())?;
    exp = Expression::map(exp.clone(), (std::sync::Arc::new(fnptr!(inlineRecordConstructorElements, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    exp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = true; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    Ok(exp)
}

fn inlineRecordConstructorElements(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD_ELEMENT { recordExp: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { r#fn, .. } }, .. } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if Function::isDefaultRecordConstructor(r#fn.clone()) {
                new_exp = (var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()).get(var_field!((*exp).index, Expression::NFExpression::RECORD_ELEMENT).clone()).unwrap();
            } else if Function::isNonDefaultRecordConstructor(r#fn.clone()) {
                new_exp = (var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()).get(var_field!((*exp).index, Expression::NFExpression::RECORD_ELEMENT).clone()).unwrap();
            } else {
                new_exp = exp.clone();
            }
            new_exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

fn getElementList(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    elements = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::TUPLE { .. } => {
            var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone()
        },
        Deref @ Expression::TUPLE_ELEMENT { tupleExp: sub_exp @ Deref @ Expression::TUPLE { .. }, .. } => {
            let mut elem: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone() > (var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE).clone().len() as i32) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInline.getElementList")); __mm_s.push_str(&*literal!(" failed to get subscripted tuple element: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            } else {
                elem = (var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE).clone()).get(var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone())?;
            }
            list![elem.clone()]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

fn checkInline(mut func: Arc<Function::Function>, mut inline_types: Arc<metamodelica::List<DAE::InlineType>>, mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating::InlineRating>>>) -> Result<bool> {
    let mut b: bool = false;
    let mut it: DAE::InlineType = Function::inlineBuiltin(func.clone());
    b = List::contains(inline_types.clone(), it.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::inlineTypeEqual, DAE::InlineType, DAE::InlineType)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::InlineType, DAE::InlineType) -> Result<bool> + 'static>)) && functionInlineable(func.clone());
    if b.clone() && DAEUtil::inlineTypeEqual(it.clone(), openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE) {
        b = defaultHeuristic(func.clone(), func_map.clone())?;
    }
    Ok(b)
}

pub const HEURISTIC_THRESHOLD: i32 = 10;

fn defaultHeuristic(mut r#fn: Arc<Function::Function>, mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating::InlineRating>>>) -> Result<bool> {
    let mut b: bool = false;
    b = InlineRating::resolve(InlineRating::fromFunction(r#fn.clone(), func_map.clone())?) < metamodelica::OrderedFloat((HEURISTIC_THRESHOLD.clone()) as f64);
    Ok(b)
}

pub mod InlineRating {
    use super::*;
    /// used to rate a function by how much it grows when inlining.
    ///    collects data about how often the inputs will occur and how much constant bloating inlining would cause.
    /// factors for each input with an additional constant overhead.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct InlineRating {
        pub input_rating: metamodelica::Array<i32>,
        pub constant_rating: i32,
    }

    impl Default for InlineRating {
        fn default() -> Self {
            Self {
                input_rating: Default::default(),
                constant_rating: Default::default(),
            }
        }
    }

    pub type INLINE_RATING = InlineRating;

    pub fn toString(mut ir: Arc<InlineRating>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{resolved: ")); __mm_s.push_str(&*realString(resolve(ir.clone()))); __mm_s.push_str(&*literal!(" | input: ")); __mm_s.push_str(&*Array::toString(ir.input_rating.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), true, 0)?); __mm_s.push_str(&*literal!(" | constant: ")); __mm_s.push_str(&*intString(ir.constant_rating.clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn resolve(mut ir: Arc<InlineRating>) -> metamodelica::Real {
        let mut r: metamodelica::Real = metamodelica::OrderedFloat(({
        let mut __acc: i32 = 0;
        for mut v in (ir.input_rating.clone()).borrow().iter() {
            let __x = v.clone();
            __acc += __x;
        }
        __acc
    } / (ir.input_rating.clone().borrow().len() as i32)) as f64) + intReal(ir.constant_rating.clone());
        r
    }

    pub fn add(mut dst: Arc<InlineRating>, mut src: Arc<InlineRating>) -> Result<Arc<InlineRating>> {
        let mut dst: Arc<InlineRating> = dst;
        if (dst.input_rating.clone().borrow().len() as i32) == (src.input_rating.clone().borrow().len() as i32) {
            let __range0 = 1..=(dst.input_rating.clone().borrow().len() as i32);
            for mut i in __range0 {
                {
                    let __cell1 = dst.input_rating.borrow()[(i.clone()-1) as usize].clone() + src.input_rating.borrow()[(i.clone()-1) as usize].clone();
                    dst.input_rating.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                }
            }
            assign_field!(dst.constant_rating = dst.constant_rating.clone() + src.constant_rating.clone());
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInline.InlineRating.add")); __mm_s.push_str(&*literal!(" failed because dst and src input arrays are of different length.\n")); __mm_s.push_str(&*literal!("dst: ")); __mm_s.push_str(&*toString(dst.clone())?); __mm_s.push_str(&*literal!("\nsrc: ")); __mm_s.push_str(&*toString(src.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(dst)
    }

    pub fn multiply(mut ir: Arc<InlineRating>, mut i: i32) -> Arc<InlineRating> {
        let mut ir: Arc<InlineRating> = ir;
        let __range0 = 1..=(ir.input_rating.clone().borrow().len() as i32);
        for mut i in __range0 {
            {
                let __cell1 = i.clone() * ir.input_rating.borrow()[(i.clone()-1) as usize].clone();
                ir.input_rating.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
            }
        }
        assign_field!(ir.constant_rating = i.clone() * ir.constant_rating.clone());
        ir
    }

    pub fn addConst(mut ir: Arc<InlineRating>) -> Arc<InlineRating> {
        let mut ir: Arc<InlineRating> = ir;
        assign_field!(ir.constant_rating = ir.constant_rating.clone() + 1);
        ir
    }

    pub fn addMapped(mut dst: Arc<InlineRating>, mut src: Arc<InlineRating>, mut args: metamodelica::Array<Arc<Expression::NFExpression>>, mut local_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<InlineRating>>>) -> Result<Arc<InlineRating>> {
        let mut dst: Arc<InlineRating> = dst;
        let mut irp: Pointer::Pointer<Arc<InlineRating>> = Pointer::create(Arc::new(InlineRating { input_rating: arrayCreate((dst.input_rating.clone().borrow().len() as i32), 0), constant_rating: src.constant_rating.clone() }));
        if (src.input_rating.clone().borrow().len() as i32) == (args.clone().borrow().len() as i32) {
            let __range0 = 1..=(src.input_rating.clone().borrow().len() as i32);
            for mut i in __range0 {
                if src.input_rating.borrow()[(i.clone()-1) as usize].clone() != 0 {
                    Expression::map(args.borrow()[(i.clone()-1) as usize].clone(), Arc::new({ let __pe_b1 = src.input_rating.borrow()[(i.clone()-1) as usize].clone(); let __pe_b2 = irp.clone(); let __pe_b3 = local_map.clone(); move |__pe_a0| addMappedExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
                }
            }
            dst = add(dst.clone(), Pointer::access(irp.clone()))?;
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInline.InlineRating.addMapped")); __mm_s.push_str(&*literal!(" failed because src input array and arguments are of different length.\n")); __mm_s.push_str(&*literal!("src: ")); __mm_s.push_str(&*toString(src.clone())?); __mm_s.push_str(&*literal!("\nargs: ")); __mm_s.push_str(&*Array::toString(args.clone(), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(dst)
    }

    pub fn addMappedExp(mut exp: Arc<Expression::NFExpression>, mut i: i32, mut irp: Pointer::Pointer<Arc<InlineRating>>, mut local_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<InlineRating>>>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            let mut iro: Option<Arc<InlineRating>> = None;
            iro = UnorderedMap::get(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), local_map.clone());
            if Util::isSome(iro.clone()) {
                Pointer::update(irp.clone(), add(Pointer::access(irp.clone()), multiply(Util::getOption(iro.clone())?, i.clone()))?);
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    pub fn fromFunction(mut r#fn: Arc<Function::Function>, mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating>>>) -> Result<Arc<InlineRating>> {
        let mut ir: Arc<InlineRating> = Arc::new(<InlineRating as ::std::default::Default>::default());
        let mut irp: Pointer::Pointer<Arc<InlineRating>>;
        let mut lir: Arc<InlineRating> = Arc::new(<InlineRating as ::std::default::Default>::default());
        let mut idx: i32 = 1;
        let mut num_inp: i32 = (r#fn.inputs.clone().len() as i32);
        let mut tmp: Arc<InlineRating> = Arc::new(<InlineRating as ::std::default::Default>::default());
        let mut local_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<InlineRating>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        for mut inp in &*r#fn.inputs.clone() {
            let mut inp = inp.clone();
            tmp = Arc::new(InlineRating { input_rating: arrayCreate(num_inp.clone(), 0), constant_rating: 0 });
            {
                let __cell0 = 1;
                tmp.input_rating.clone().borrow_mut()[(idx.clone()-1) as usize] = __cell0;
            }
            idx = idx.clone() + 1;
            UnorderedMap::add(ComponentRef::fromNode(inp.clone(), InstNode::getType(inp.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()), tmp.clone(), local_map.clone())?;
        }
        for mut loc in &*r#fn.locals.clone() {
            let mut loc = loc.clone();
            irp = Pointer::create(Arc::new(InlineRating { input_rating: arrayCreate(num_inp.clone(), 0), constant_rating: 0 }));
            lir = (::match_deref::match_deref! { match &(InstNode::getBindingExpOpt(loc.clone())) {
        Some(bind) => {
            Expression::fakeMap(bind.clone(), Arc::new({ let __pe_b1 = func_map.clone(); let __pe_b2 = local_map.clone(); let __pe_b3 = irp.clone(); move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }));
            Pointer::access(irp.clone())
        },
        _ => {
            Pointer::access(irp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            UnorderedMap::add(ComponentRef::fromNode(loc.clone(), InstNode::getType(loc.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()), lir.clone(), local_map.clone())?;
        }
        irp = Pointer::create(Arc::new(InlineRating { input_rating: arrayCreate(num_inp.clone(), 0), constant_rating: 0 }));
        Expression::fakeMap(Function::getSingleBodyExp(r#fn.clone())?, Arc::new({ let __pe_b1 = func_map.clone(); let __pe_b2 = local_map.clone(); let __pe_b3 = irp.clone(); move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }));
        ir = Pointer::access(irp.clone());
        UnorderedMap::add(r#fn.clone(), ir.clone(), func_map.clone())?;
        Ok(ir)
    }

    pub fn rateExpression(mut exp: Arc<Expression::NFExpression>, mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating>>>, mut local_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<InlineRating>>>, mut irp: Pointer::Pointer<Arc<InlineRating>>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut cont: bool = false;
        if Expression::isLiteral(exp.clone()) {
            Pointer::update(irp.clone(), addConst(Pointer::access(irp.clone())));
        } else {
            cont = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } if (functionInlineable(Call::typedFunction(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?)) => {
            let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut lir: Option<Arc<InlineRating>> = None;
            r#fn = Call::typedFunction(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?;
            lir = UnorderedMap::get(r#fn.clone(), func_map.clone());
            if Util::isSome(lir.clone()) {
                Pointer::update(irp.clone(), addMapped(Pointer::access(irp.clone()), Util::getOption(lir.clone())?, metamodelica::arrayFromVec(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?.into_iter().cloned().collect()), local_map.clone())?);
                cont = false;
            } else if DAEUtil::inlineTypeEqual(Function::inlineBuiltin(r#fn.clone()), openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE) {
                Pointer::update(irp.clone(), addMapped(Pointer::access(irp.clone()), fromFunction(r#fn.clone(), func_map.clone())?, metamodelica::arrayFromVec(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?.into_iter().cloned().collect()), local_map.clone())?);
                cont = false;
            } else {
                cont = true;
            }
            cont.clone()
        },
        Deref @ Expression::CREF { .. } => {
            let mut lir: Option<Arc<InlineRating>> = None;
            (::match_deref::match_deref! { match &(UnorderedMap::get(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()), local_map.clone())) {
        lir @ Some(_) => {
            Pointer::update(irp.clone(), add(Pointer::access(irp.clone()), Util::getOption(lir.clone())?)?);
            false
        },
        _ => {
            Pointer::update(irp.clone(), addConst(Pointer::access(irp.clone())));
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if cont.clone() {
                exp = Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = func_map.clone(); let __pe_b2 = local_map.clone(); let __pe_b3 = irp.clone(); move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            }
        }
        Ok(exp)
    }

}


