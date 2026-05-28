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

use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBEquation;
use crate::NBReplacements as Replacements;
use crate::NBSolve as Solve;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:         NBResizable.mo
///  package:      NBResizable
///  description:  This file contains util functions for resizable parameters.
pub struct NBResizable;
pub const debug: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum EvalOrder {
    INDEPENDENT = 1,
    FORWARD = 2,
    BACKWARD = 3,
    FAILED = 4,
}
impl PartialOrd for EvalOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for EvalOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for EvalOrder {
    fn default() -> Self { Self::INDEPENDENT }
}

pub fn resize(mut equations: Arc<EquationPointers::EquationPointers>, mut varData: Arc<VarData::VarData>) -> Result<(Arc<EquationPointers::EquationPointers>, Arc<VarData::VarData>)> {
    type applyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>;

    let mut equations: Arc<EquationPointers::EquationPointers> = equations;
    let mut varData: Arc<VarData::VarData> = varData;
    let mut parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut min_parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut c2pi: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>;
    let mut c2pe: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>;
    let mut p2ci: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>;
    let mut p2ce: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>;
    let mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>;
    varData = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ BVariable::VarData::VAR_DATA_SIM { .. } if (BVariable::VariablePointers::size(var_field!((*varData).resizables, VarData::VarData::VAR_DATA_SIM).clone()) > 0) => {
            parameters = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            min_parameters = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            optimal_values = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            c2pi = UnorderedMap::new((std::sync::Arc::new(fnptr!(Expression::hash, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<i32> + 'static>), (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>), 1);
            c2pe = UnorderedMap::new((std::sync::Arc::new(fnptr!(Expression::hash, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<i32> + 'static>), (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>), 1);
            EquationPointers::map(equations.clone(), Arc::new({ let __pe_b1 = parameters.clone(); let __pe_b2 = min_parameters.clone(); let __pe_b3 = optimal_values.clone(); let __pe_b4 = c2pi.clone(); let __pe_b5 = c2pe.clone(); move |__pe_a0| findOptimalResizableValues(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?;
            UnorderedSet::apply(parameters.clone(), Arc::new({ let __pe_b1 = min_parameters.clone(); let __pe_b2 = optimal_values.clone(); move |__pe_a0| setInitialValues(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            if debug.clone() {
                println!("{}", (optimalValuesToString(optimal_values.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("[debug] Initial Resizable Parameter Values:")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?).clone());
                println!("{}", (StringUtil::headline_2((literal!("[debug] Final Inequality Constraints:")).clone())).clone());
                if UnorderedMap::isEmpty(c2pi.clone()) {
                    println!("{}", (literal!("  <No Constraints>\n\n")).clone());
                } else {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(UnorderedMap::keyList(c2pi.clone()), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("  0 >= ")).clone(), (literal!("\n  0 >= ")).clone(), (literal!("\n")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                println!("{}", (StringUtil::headline_2((literal!("[debug] Final Equality Constraints:")).clone())).clone());
                if UnorderedMap::isEmpty(c2pe.clone()) {
                    println!("{}", (literal!("  <No Constraints>\n\n")).clone());
                } else {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(UnorderedMap::keyList(c2pe.clone()), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("  0 = ")).clone(), (literal!("\n  0 = ")).clone(), (literal!("\n")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            p2ci = invertConstraintParameterMap(c2pi.clone(), parameters.clone())?;
            p2ce = invertConstraintParameterMap(c2pe.clone(), parameters.clone())?;
            computeOptimalValues(optimal_values.clone(), c2pi.clone(), p2ci.clone(), c2pe.clone(), p2ce.clone())?;
            func = Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static> = Arc::new({ let __pe_b1 = optimal_values.clone(); move |__pe_a0| updateDimension(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Type::applyToDims(__pe_a0, __pe_b1.clone()) });
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                variables = BVariable::VariablePointers::map(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static> = func.clone(); move |__pe_a0| Ok(Variable::applyToType(__pe_a0, __pe_b1.clone())) }))?,
                variables = BVariable::VariablePointers::mapPtr(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), Arc::new({ let __pe_b1 = optimal_values.clone(); move |__pe_a0| Ok(BVariable::updateResizableParameter(__pe_a0, __pe_b1.clone())) }))?,
                variables = BVariable::VariablePointers::mapPtr(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), Arc::new({ let __pe_b1 = Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static> = func.clone(); move |__pe_a0| Expression::applyToType(__pe_a0, __pe_b1.clone()) }); let __pe_b2 = (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| BVariable::mapExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?
            );
            EquationPointers::mapPtr(equations.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static> = func.clone(); move |__pe_a0| Ok(Equation::applyToType(__pe_a0, __pe_b1.clone())) }))?;
            equations = EquationPointers::mapExp(equations.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static> = func.clone(); move |__pe_a0| Expression::applyToType(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            EquationPointers::mapRes(equations.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static> = func.clone(); move |__pe_a0| Ok(BVariable::applyToType(__pe_a0, __pe_b1.clone())) }))?;
            if Flags::isSet(Flags::DUMP_RESIZABLE.clone())? || debug.clone() {
                println!("{}", (optimalValuesToString(optimal_values.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("[dumpResizable] Evaluated Optimal Resizable Parameter Values:")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?).clone());
            }
            varData.clone()
        },
        _ => {
            if Flags::isSet(Flags::DUMP_RESIZABLE.clone())? || debug.clone() {
                println!("{}", (StringUtil::headline_2((literal!("[dumpResizable] No resizable parameters were detected in the model.")).clone())).clone());
            }
            varData.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((equations, varData))
}

pub fn detect(mut eqn: Arc<Equation::Equation>, mut cref_to_solve: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, EvalOrder>>> {
    let mut order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, EvalOrder>>;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = BVariable::getVarPointer(cref_to_solve.clone(), metamodelica::sourceInfo!())?;
    let mut var_occurences: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut ite_occurences: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut occ_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut iterators: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    let mut subs_to_solve: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut local_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut sub_to_solve: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut iter: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    let mut opt_factor: Option<i32> = None;
    let mut factor: i32 = 0;
    let mut shift_value: i32 = 0;
    let mut v2: i32 = 0;
    let mut eval: EvalOrder = EvalOrder::INDEPENDENT;
    order = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { .. } => {
            for mut body in &*var_field!((*eqn).body, Equation::Equation::FOR_EQUATION).clone() {
                let mut body = body.clone();
                Equation::map(body.clone(), Arc::new({ let __pe_b1 = Arc::new({ let __pe_b1 = var_ptr.clone(); move |__pe_a0| Ok(BVariable::equalName(__pe_a0, __pe_b1.clone())) }); let __pe_b2 = var_occurences.clone(); move |__pe_a0| collectVars(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                occ_lst = UnorderedSet::toList(var_occurences.clone());
                (iterators, _, _) = Iterator::getFrames(Equation::getForIterator(eqn.clone()))?;
                order = UnorderedMap::fromLists(iterators.clone(), ({
        let mut __acc: Arc<metamodelica::List<EvalOrder>> = metamodelica::nil();
        for mut i in (iterators.clone()).into_iter().cloned() {
            let __x = EvalOrder::INDEPENDENT.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                if !(List::hasOneElement(occ_lst.clone())) {
                    subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
        for mut cref in (occ_lst.clone()).into_iter().cloned() {
            let __x = ComponentRef::subscriptsAllWithWholeFlat(cref.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    subs = List::transposeList(subs.clone())?;
                    subs_to_solve = ComponentRef::subscriptsAllWithWholeFlat(cref_to_solve.clone());
                    for mut dim in &*List::zip(subs.clone(), subs_to_solve.clone()) {
                        let mut dim = dim.clone();
                        (local_subs, sub_to_solve) = dim.clone();
                        ite_occurences = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
                        for mut sub in &*local_subs.clone() {
                            let mut sub = sub.clone();
                            Subscript::mapExp(sub.clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(BVariable::isIterator, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); let __pe_b2 = ite_occurences.clone(); move |__pe_a0| collectVars(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
                        }
                        iterators = UnorderedSet::toList(ite_occurences.clone());
                        let () = (::match_deref::match_deref! { match &(iterators.clone()) {
        Deref @ metamodelica::List::Cons { head: iter, tail: Deref @ metamodelica::List::Nil } => {
            eval = UnorderedMap::getSafe(iter.clone(), order.clone(), metamodelica::sourceInfo!())?;
            if eval.clone() < EvalOrder::FAILED.clone() {
                args = Differentiate::DifferentiationArguments::simpleCref(iter.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1));
                opt_factor = None;
                for mut sub in &*local_subs.clone() {
                    let mut sub = sub.clone();
                    factor = getFactor(Subscript::toExp(sub.clone())?, args.clone(), opt_factor.clone())?;
                    opt_factor = Some(factor.clone());
                }
                let () = (match opt_factor.clone() {
        Some(mut factor) if (factor.clone() != 0) => {
            if '__try0: {
                let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(getShift(Subscript::toExp(sub_to_solve.clone())?, iter.clone()), '__try0)) {
                    Deref @ Expression::INTEGER { value: __pa1 } => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                shift_value = __pa1.clone();
                for mut sub in &*local_subs.clone() {
                    let mut sub = sub.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(getShift(Subscript::toExp(sub.clone())?, iter.clone()), '__try0)) {
                        Deref @ Expression::INTEGER { value: __pa2 } => __pa2.clone(),
                        _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    v2 = __pa2.clone();
                    eval = (match eval.clone() {
        EvalOrder::INDEPENDENT if (shift_value.clone() == v2.clone()) => EvalOrder::INDEPENDENT.clone(),
        EvalOrder::INDEPENDENT if (shift_value.clone() > v2.clone()) => EvalOrder::FORWARD.clone(),
        EvalOrder::INDEPENDENT if (shift_value.clone() < v2.clone()) => EvalOrder::BACKWARD.clone(),
        EvalOrder::FORWARD if (shift_value.clone() >= v2.clone()) => EvalOrder::FORWARD.clone(),
        EvalOrder::BACKWARD if (shift_value.clone() <= v2.clone()) => EvalOrder::BACKWARD.clone(),
        _ => EvalOrder::FAILED.clone(),
    });
                }
                if eval.clone() == EvalOrder::FAILED.clone() {
                    break;
                }
                Ok::<(), anyhow::Error>(())
            }.is_err() {
                eval = EvalOrder::FAILED.clone();
            }
            UnorderedMap::add(iter.clone(), eval.clone(), order.clone())?;
            ()
        },
        _ => (),
    });
            }
            ()
        },
        _ => {
            for mut it in &*iterators.clone() {
                let mut it = it.clone();
                UnorderedMap::add(it.clone(), EvalOrder::FAILED.clone(), order.clone())?;
            }
            break;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    }
                }
            }
            order.clone()
        },
        _ => {
            order = UnorderedMap::fromLists(list![Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY)], list![EvalOrder::FAILED.clone()], (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            order.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(order)
}

pub fn orderFailed(mut eo: EvalOrder) -> bool {
    let mut b: bool = eo.clone() == EvalOrder::FAILED.clone();
    b
}

pub fn orderString(mut eo: EvalOrder) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match eo.clone() {
        EvalOrder::INDEPENDENT => literal!("INDEPENDENT"),
        EvalOrder::FORWARD => literal!("FORWARD"),
        EvalOrder::BACKWARD => literal!("BACKWARD"),
        _ => literal!("FAILED"),
    })).clone();
    r#str
}

pub type ParameterList = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

pub type ConstraintList = Arc<metamodelica::List<Arc<Expression::NFExpression>>>;

pub type Occurences = Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>;

pub const fn END_TPL() -> (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>) { (openmodelica_nf_frontend::NFComponentRef::EMPTY, openmodelica_nf_frontend::NFExpression::END) }

fn findOptimalResizableValues(mut eqn: Arc<Equation::Equation>, mut parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut min_parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut c2pi: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut c2pe: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut resizables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut occs: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>>>;
    let mut constrained_vars: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut lhs_dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rhs_dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut r#const: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[debug] checking equation:\n")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    let () = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { .. } => {
            resizables = getResizableIterators(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            replacements = getVarReplacements(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            occs = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            for mut res in &*UnorderedMap::keyList(resizables.clone()) {
                let mut res = res.clone();
                UnorderedMap::add(res.clone(), UnorderedSet::new((std::sync::Arc::new(fnptr!(Expression::hash, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<i32> + 'static>), (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>), 13), occs.clone())?;
            }
            for mut body in &*var_field!((*eqn).body, Equation::Equation::FOR_EQUATION).clone() {
                let mut body = body.clone();
                Equation::map(body.clone(), Arc::new({ let __pe_b1 = occs.clone(); move |__pe_a0| Ok(collectOccurences(__pe_a0, __pe_b1.clone())) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                Equation::map(body.clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(BVariable::isArray, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); let __pe_b2 = constrained_vars.clone(); move |__pe_a0| collectVars(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            }
            findOptimalValue(eqn.clone(), occs.clone(), resizables.clone(), parameters.clone(), min_parameters.clone(), optimal_values.clone(), c2pi.clone())?;
            UnorderedSet::fold(constrained_vars.clone(), Arc::new({ let __pe_b1 = eqn.clone(); let __pe_b2 = Some(replacements.clone()); move |__pe_a0, __pe_a3| addVariableConstraint(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), c2pi.clone());
            ()
        },
        Deref @ Equation::ARRAY_EQUATION { .. } => {
            Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(BVariable::isResizable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); let __pe_b2 = constrained_vars.clone(); move |__pe_a0| collectVars(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            UnorderedSet::fold(constrained_vars.clone(), Arc::new({ let __pe_b1 = eqn.clone(); let __pe_b2 = None; move |__pe_a0, __pe_a3| addVariableConstraint(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), c2pi.clone());
            for mut tpl in &*List::zip(Type::arrayDims(Expression::typeOf(var_field!((*eqn).lhs, Equation::Equation::ARRAY_EQUATION).clone())), Type::arrayDims(Expression::typeOf(var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone()))) {
                let mut tpl = tpl.clone();
                (lhs_dim, rhs_dim) = tpl.clone();
                if Dimension::isResizable(lhs_dim.clone()) || Dimension::isResizable(rhs_dim.clone()) {
                    r#const = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Dimension::sizeExp(lhs_dim.clone())?], inv_arguments: list![Dimension::sizeExp(rhs_dim.clone())?], operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::INTEGER)) });
                    if '__try0: {
                        unwrap_break_err!(addConstraint(r#const.clone(), None, c2pe.clone(), (std::sync::Arc::new(fnptr!(Expression::isZero, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>), (literal!("array dimension")).clone(), (literal!("=")).clone()), '__try0);
                        unwrap_break_err!(Expression::map(r#const.clone(), Arc::new({ let __pe_b1 = parameters.clone(); move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone()) })), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_err() {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResizable.findOptimalResizableValues")); __mm_s.push_str(&*literal!(" failed.\nViolation of implicit constraint `")); __mm_s.push_str(&*Dimension::toString(lhs_dim.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Dimension::toString(rhs_dim.clone())?); __mm_s.push_str(&*literal!("` for LHS and RHS type dimensions in equation:\n")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                        bail!("fail");
                    }
                }
            }
            ()
        },
        _ => {
            Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(BVariable::isResizable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); let __pe_b2 = constrained_vars.clone(); move |__pe_a0| collectVars(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            UnorderedSet::fold(constrained_vars.clone(), Arc::new({ let __pe_b1 = eqn.clone(); let __pe_b2 = None; move |__pe_a0, __pe_a3| addVariableConstraint(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), c2pi.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if debug.clone() {
        println!("{}", (literal!("\n")).clone());
    }
    Ok(eqn)
}

fn getResizableIterators(mut iter: Arc<Iterator::Iterator>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>> {
    let mut resizables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    (names, ranges, _) = Iterator::getFrames(iter.clone())?;
    for mut tpl in &*List::zip(names.clone(), ranges.clone()) {
        let mut tpl = tpl.clone();
        if iteratorIsResizable(Util::tuple22(tpl.clone())) {
            UnorderedMap::add(Util::tuple21(tpl.clone()), Util::tuple22(tpl.clone()), resizables.clone())?;
        }
    }
    Ok(resizables)
}

fn getVarReplacements(mut iter: Arc<Iterator::Iterator>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>> {
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut max_call: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (names, ranges, _) = Iterator::getFrames(iter.clone())?;
    for mut tpl in &*List::zip(names.clone(), ranges.clone()) {
        let mut tpl = tpl.clone();
        (name, range) = tpl.clone();
        let () = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { .. } => {
            if Util::isSome(var_field!((*range).step, Expression::NFExpression::RANGE).clone()) && Expression::isNegative(Util::getOption(var_field!((*range).step, Expression::NFExpression::RANGE).clone())?) {
                UnorderedMap::add(name.clone(), var_field!((*range).start, Expression::NFExpression::RANGE).clone(), replacements.clone())?;
            } else if Util::isNone(var_field!((*range).step, Expression::NFExpression::RANGE).clone()) || Expression::isPositive(Util::getOption(var_field!((*range).step, Expression::NFExpression::RANGE).clone())?) {
                UnorderedMap::add(name.clone(), var_field!((*range).stop, Expression::NFExpression::RANGE).clone(), replacements.clone())?;
            } else {
                max_call = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_INT().clone(), list![var_field!((*range).start, Expression::NFExpression::RANGE).clone(), var_field!((*range).stop, Expression::NFExpression::RANGE).clone()], Expression::variability(var_field!((*range).start, Expression::NFExpression::RANGE).clone())?, NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::MAX_INT().returnType.clone()) });
                UnorderedMap::add(name.clone(), max_call.clone(), replacements.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(replacements)
}

fn iteratorIsResizable(mut range: Arc<Expression::NFExpression>) -> bool {
    let mut b: bool = Expression::fold(range.clone(), (std::sync::Arc::new(fnptr!(expContainsResizable, Arc<Expression::NFExpression>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<bool> + 'static>), false).unwrap();
    b
}

fn expContainsResizable(mut exp: Arc<Expression::NFExpression>, mut b: bool) -> bool {
    let mut b: bool = b;
    if !(b.clone()) {
        b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => BVariable::checkCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(fnptr!(BVariable::isResizableParameter, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    b
}

fn collectResizables(mut exp: Arc<Expression::NFExpression>, mut collector: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (BVariable::checkCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(fnptr!(BVariable::isResizableParameter, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!())) => {
            UnorderedSet::add(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), collector.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn collectOccurences(mut exp: Arc<Expression::NFExpression>, mut occs: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>>>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            ComponentRef::mapSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), Arc::new({ let __pe_b1 = occs.clone(); move |__pe_a0| collectOccurencesSubscript(__pe_a0, __pe_b1.clone()) }), false);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

fn collectOccurencesSubscript(mut sub: Arc<Subscript::NFSubscript>, mut occs: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>>>) -> Result<Arc<Subscript::NFSubscript>> {
    let mut sub: Arc<Subscript::NFSubscript> = sub;
    let mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut subExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    Subscript::mapExp(sub.clone(), Arc::new({ let __pe_b1 = occs.clone(); let __pe_b2 = acc.clone(); move |__pe_a0| collectOccurencesSubscriptExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    if !(UnorderedSet::isEmpty(acc.clone())) {
        subExp = Subscript::toExp(sub.clone())?;
        UnorderedSet::apply(acc.clone(), Arc::new({ let __pe_b1 = subExp.clone(); let __pe_b2 = occs.clone(); move |__pe_a0| addOccurence(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    }
    Ok(sub)
}

fn collectOccurencesSubscriptExp(mut exp: Arc<Expression::NFExpression>, mut occs: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>>>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (UnorderedMap::contains(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), occs.clone())) => {
            UnorderedSet::add(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), acc.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn addOccurence(mut iter: Arc<ComponentRef::NFComponentRef>, mut subExp: Arc<Expression::NFExpression>, mut occs: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut iter: Arc<ComponentRef::NFComponentRef> = iter;
    let mut local_occ: Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>> = UnorderedMap::getSafe(iter.clone(), occs.clone(), metamodelica::sourceInfo!())?;
    UnorderedSet::add(subExp.clone(), local_occ.clone())?;
    Ok(iter)
}

fn collectVars(mut exp: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut collector: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (func(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?)?) => {
            UnorderedSet::add(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), collector.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn findOptimalValue(mut eqn: Arc<Equation::Equation>, mut occs: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>>>, mut resizables: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut min_parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut c2pi: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<()> {
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut target: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut local_parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut args: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    for mut cref in &*UnorderedMap::keyList(occs.clone()) {
        let mut cref = cref.clone();
        range = UnorderedMap::getSafe(cref.clone(), resizables.clone(), metamodelica::sourceInfo!())?;
        Expression::map(range.clone(), Arc::new({ let __pe_b1 = parameters.clone(); move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone()) }))?;
        let () = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { .. } => {
            target = Arc::new(Expression::NFExpression::MULTARY { arguments: list![var_field!((*range).stop, Expression::NFExpression::RANGE).clone()], inv_arguments: list![var_field!((*range).start, Expression::NFExpression::RANGE).clone()], operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::INTEGER)) });
            target = SimplifyExp::simplify(target.clone(), false)?;
            local_parameters = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            Expression::map(range.clone(), Arc::new({ let __pe_b1 = local_parameters.clone(); move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone()) }))?;
            UnorderedSet::merge(parameters.clone(), local_parameters.clone())?;
            args = Differentiate::DifferentiationArguments::simpleCref(cref.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1));
            UnorderedSet::apply(local_parameters.clone(), Arc::new({ let __pe_b1 = target.clone(); let __pe_b2 = args.clone(); let __pe_b3 = min_parameters.clone(); let __pe_b4 = optimal_values.clone(); move |__pe_a0| getInitialValues(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
            getRangeConstraint(var_field!((*range).start, Expression::NFExpression::RANGE).clone(), var_field!((*range).step, Expression::NFExpression::RANGE).clone(), var_field!((*range).stop, Expression::NFExpression::RANGE).clone(), local_parameters.clone(), c2pi.clone(), (literal!("equation")).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

fn getRangeConstraint(mut start: Arc<Expression::NFExpression>, mut step_opt: Option<Arc<Expression::NFExpression>>, mut stop: Arc<Expression::NFExpression>, mut parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut c2pi: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut const_kind: ArcStr) -> Result<()> {
    let mut step: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut target: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut distance_const: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    step = Util::getOptionOrDefault(step_opt.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }));
    target = Arc::new(Expression::NFExpression::MULTARY { arguments: list![stop.clone()], inv_arguments: list![start.clone()], operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::INTEGER)) });
    distance_const = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::INTEGER { value: 2 })], inv_arguments: list![Arc::new(Expression::NFExpression::MULTARY { arguments: list![target.clone()], inv_arguments: list![step.clone()], operator: Operator::makeMul(Arc::new(openmodelica_nf_frontend::NFType::INTEGER)) })], operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::INTEGER)) });
    distance_const = SimplifyExp::simplify(distance_const.clone(), false)?;
    distance_const = SimplifyExp::combineBinaries(distance_const.clone())?;
    distance_const = SimplifyExp::simplify(distance_const.clone(), false)?;
    UnorderedMap::add(distance_const.clone(), UnorderedSet::toList(parameters.clone()), c2pi.clone())?;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[debug] adding ")); __mm_s.push_str(&*const_kind.clone()); __mm_s.push_str(&*literal!(" constraint: 0 >= ")); __mm_s.push_str(&*Expression::toString(distance_const.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn getFactor(mut exp: Arc<Expression::NFExpression>, mut args: Arc<DifferentiationArguments::DifferentiationArguments>, mut opt_factor: Option<i32>) -> Result<i32> {
    let mut factor: i32 = 0;
    let mut diff: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (diff, _) = Differentiate::differentiateExpression(exp.clone(), args.clone())?;
    diff = SimplifyExp::simplify(diff.clone(), false)?;
    factor = Expression::integerValueOrDefault(diff.clone(), 0);
    if Util::isSome(opt_factor.clone()) && factor.clone() != Util::getOption(opt_factor.clone())? {
        factor = 0;
    }
    Ok(factor)
}

fn getShift(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<Expression::NFExpression>> {
    let mut shift: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    shift = Replacements::single(exp.clone(), Arc::new(Expression::NFExpression::CREF { ty: Arc::new(openmodelica_nf_frontend::NFType::INTEGER), cref: cref.clone() }), Arc::new(Expression::NFExpression::INTEGER { value: 0 }))?;
    shift = SimplifyExp::simplify(shift.clone(), false)?;
    Ok(shift)
}

fn getDistance(mut cref: Arc<ComponentRef::NFComponentRef>, mut exp: Arc<Expression::NFExpression>, mut args: Arc<DifferentiationArguments::DifferentiationArguments>, mut opt_factor: Option<i32>, mut min_distance: i32, mut max_distance: i32) -> Result<(Option<i32>, i32, i32)> {
    let mut opt_factor: Option<i32> = opt_factor;
    let mut min_distance: i32 = min_distance;
    let mut max_distance: i32 = max_distance;
    let mut shift: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut factor: i32 = 0;
    let mut distance: i32 = 0;
    if Util::isNone(opt_factor.clone()) || Util::getOption(opt_factor.clone())? != 0 {
        factor = getFactor(exp.clone(), args.clone(), opt_factor.clone())?;
        if factor.clone() != 0 {
            shift = getShift(exp.clone(), cref.clone())?;
            match '__try0: {
                let __pa1 = ::match_deref::match_deref! { match &(shift.clone()) {
                    Deref @ Expression::INTEGER { value: __pa1 } => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                distance = __pa1.clone();
                if Util::isNone(opt_factor.clone()) {
                    min_distance = distance.clone();
                    max_distance = distance.clone();
                    opt_factor = Some(factor.clone());
                } else {
                    min_distance = intMin(distance.clone(), min_distance.clone());
                    max_distance = intMax(distance.clone(), max_distance.clone());
                }
                Ok::<_, anyhow::Error>((max_distance.clone(), min_distance.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    max_distance = __try0_o0;
                    min_distance = __try0_o1;
                }
                Err(_) => {
                    min_distance = 0;
                    max_distance = 0;
                    opt_factor = Some(0);
                }
            }
        } else {
            min_distance = 0;
            max_distance = 0;
            opt_factor = Some(0);
        }
    }
    Ok((opt_factor, min_distance, max_distance))
}

fn addVariableConstraint(mut cref: Arc<ComponentRef::NFComponentRef>, mut eqn: Arc<Equation::Equation>, mut replacements: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>>, mut c2pi: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>> {
    let mut c2pi: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = c2pi;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = Type::arrayDims(var.ty.clone());
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = ComponentRef::subscriptsAllWithWholeFlat(cref.clone());
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut sub_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#const: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator> = Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::INTEGER));
    for mut tpl in &*List::zip(dims.clone(), subs.clone()) {
        let mut tpl = tpl.clone();
        (dim, sub) = tpl.clone();
        sub_exp = Subscript::toExp(sub.clone())?;
        r#const = Arc::new(Expression::NFExpression::MULTARY { arguments: list![sub_exp.clone()], inv_arguments: list![Dimension::sizeExp(dim.clone())?], operator: op.clone() });
        if '__try0: {
            unwrap_break_err!(addConstraint(r#const.clone(), replacements.clone(), c2pi.clone(), (std::sync::Arc::new(fnptr!(Expression::isNonPositive, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" (variable)")); ArcStr::from(__mm_s) }).clone(), (literal!(">=")).clone()), '__try0);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResizable.addVariableConstraint")); __mm_s.push_str(&*literal!(" failed.\nViolation of implicit constraint `")); __mm_s.push_str(&*Dimension::toString(dim.clone())?); __mm_s.push_str(&*literal!(" >= ")); __mm_s.push_str(&*Subscript::toString(sub.clone())?); __mm_s.push_str(&*literal!("` for component reference `")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!("` of variable `")); __mm_s.push_str(&*Variable::toString(Pointer::access(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?), (literal!("")).clone(), false)?); __mm_s.push_str(&*literal!("`\nin equation:\n")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        r#const = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::INTEGER { value: 1 })], inv_arguments: list![Dimension::sizeExp(dim.clone())?], operator: op.clone() });
        if '__try1: {
            unwrap_break_err!(addConstraint(r#const.clone(), replacements.clone(), c2pi.clone(), (std::sync::Arc::new(fnptr!(Expression::isNonPositive, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" (variable)")); ArcStr::from(__mm_s) }).clone(), (literal!(">=")).clone()), '__try1);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResizable.addVariableConstraint")); __mm_s.push_str(&*literal!(" failed.\nViolation of implicit constraint `")); __mm_s.push_str(&*Dimension::toString(dim.clone())?); __mm_s.push_str(&*literal!(" >= 1")); __mm_s.push_str(&*literal!("` for component reference `")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!("` of variable `")); __mm_s.push_str(&*Variable::toString(Pointer::access(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?), (literal!("")).clone(), false)?); __mm_s.push_str(&*literal!("`\nin equation:\n")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
    }
    Ok(c2pi)
}

fn addConstraint(mut old_const: Arc<Expression::NFExpression>, mut replacements: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>>, mut c2p: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>, mut const_kind: ArcStr, mut eq_kind: ArcStr) -> Result<()> {
    pub type checkFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut r#const: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut diff: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut params: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut redundant: bool = false;
    let mut args: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    let mut zero_replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    if Util::isSome(replacements.clone()) {
        r#const = Expression::map(old_const.clone(), Arc::new({ let __pe_b1 = Util::getOption(replacements.clone())?; move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
    } else {
        r#const = old_const.clone();
    }
    r#const = Expression::map(r#const.clone(), Arc::new({ let __pe_b1 = replacements.clone(); let __pe_b2 = c2p.clone(); let __pe_b3: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static> = func.clone(); let __pe_b4 = (const_kind.clone()).clone(); let __pe_b5 = (eq_kind.clone()).clone(); move |__pe_a0| addRangeConstraints(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?;
    parameters = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    Expression::map(r#const.clone(), Arc::new({ let __pe_b1 = parameters.clone(); move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone()) }))?;
    params = UnorderedSet::toList(parameters.clone());
    redundant = true;
    for mut p in &*params.clone() {
        let mut p = p.clone();
        args = Differentiate::DifferentiationArguments::simpleCref(p.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1));
        (diff, _) = Differentiate::differentiateExpression(r#const.clone(), args.clone())?;
        diff = SimplifyExp::simplify(diff.clone(), false)?;
        if !(Expression::isZero(diff.clone())) {
            redundant = false;
            break;
        }
    }
    if !(redundant.clone()) {
        UnorderedMap::add(r#const.clone(), params.clone(), c2p.clone())?;
    } else {
        zero_replacements = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        for mut p in &*params.clone() {
            let mut p = p.clone();
            UnorderedMap::add(p.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), zero_replacements.clone())?;
        }
        r#const = Expression::map(r#const.clone(), Arc::new({ let __pe_b1 = zero_replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
        r#const = SimplifyExp::simplify(r#const.clone(), false)?;
        if !(func(r#const.clone())?) {
            bail!("fail");
        }
    }
    if debug.clone() {
        if redundant.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[debug] not adding redundant ")); __mm_s.push_str(&*const_kind.clone()); __mm_s.push_str(&*literal!(" constraint: 0 ")); __mm_s.push_str(&*eq_kind.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Expression::toString(old_const.clone())?); __mm_s.push_str(&*literal!(" simplified to: 0 ")); __mm_s.push_str(&*eq_kind.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Expression::toString(r#const.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        } else {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[debug] adding ")); __mm_s.push_str(&*const_kind.clone()); __mm_s.push_str(&*literal!(" constraint: 0 ")); __mm_s.push_str(&*eq_kind.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Expression::toString(old_const.clone())?); __mm_s.push_str(&*literal!(" simplified to: 0 ")); __mm_s.push_str(&*eq_kind.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Expression::toString(r#const.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok(())
}

fn addRangeConstraints(mut exp: Arc<Expression::NFExpression>, mut replacements: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>>, mut c2p: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>, mut const_kind: ArcStr, mut eq_kind: ArcStr) -> Result<Arc<Expression::NFExpression>> {
    pub type checkFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RANGE { .. } => {
            parameters = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            Expression::map(exp.clone(), Arc::new({ let __pe_b1 = parameters.clone(); move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone()) }))?;
            getRangeConstraint(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), var_field!((*exp).step, Expression::NFExpression::RANGE).clone(), var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), parameters.clone(), c2p.clone(), (literal!("variable")).clone())?;
            Expression::rangeSizeExp(exp.clone())
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn getInitialValues(mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<Expression::NFExpression>, mut args: Arc<DifferentiationArguments::DifferentiationArguments>, mut min_parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut diff: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut binding: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    assign_field!(args.diffCref = cref.clone());
    (diff, _) = Differentiate::differentiateExpression(target.clone(), args.clone())?;
    diff = SimplifyExp::simplify(diff.clone(), false)?;
    if Expression::isPositive(diff.clone()) {
        UnorderedSet::add(cref.clone(), min_parameters.clone())?;
    } else if Expression::isNegative(diff.clone()) {
    } else {
        var = Pointer::access(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?);
        binding = Binding::getExp(var.binding.clone())?;
        UnorderedMap::add(cref.clone(), binding.clone(), optimal_values.clone())?;
    }
    Ok(cref)
}

fn setInitialValues(mut cref: Arc<ComponentRef::NFComponentRef>, mut min_parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut attributes: Arc<VariableAttributes::VariableAttributes>;
    let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if !(UnorderedMap::contains(cref.clone(), optimal_values.clone())) {
        var = Pointer::access(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?);
        value = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { attributes: attributes @ Deref @ VariableAttributes::VAR_ATTR_INT { .. }, .. }, .. } => {
            if UnorderedSet::contains(cref.clone(), min_parameters.clone())? {
                if Util::isSome(var_field!((**attributes).min, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(var_field!((**attributes).min, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    value = __pa0.clone();
                } else {
                    value = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
                }
            } else if Util::isSome(var_field!((**attributes).max, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone()) {
                let __pa1 = ::match_deref::match_deref! { match &(var_field!((**attributes).max, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                value = __pa1.clone();
            } else {
                value = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
            }
            value.clone()
        },
        _ => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        UnorderedMap::add(cref.clone(), value.clone(), optimal_values.clone())?;
    }
    Ok(cref)
}

fn invertConstraintParameterMap(mut c2p: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>> {
    let mut p2c: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut r#const: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut params: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    for mut param in &*UnorderedSet::toList(parameters.clone()) {
        let mut param = param.clone();
        UnorderedMap::add(param.clone(), metamodelica::nil(), p2c.clone())?;
    }
    for mut tpl in &*UnorderedMap::toList(c2p.clone()) {
        let mut tpl = tpl.clone();
        (r#const, params) = tpl.clone();
        for mut param in &*params.clone() {
            let mut param = param.clone();
            UnorderedMap::add(param.clone(), cons(r#const.clone(), UnorderedMap::getSafe(param.clone(), p2c.clone(), metamodelica::sourceInfo!())?), p2c.clone())?;
        }
    }
    Ok(p2c)
}

fn computeOptimalValues(mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut c2pi: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut p2ci: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>, mut c2pe: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut p2ce: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>) -> Result<()> {
    let mut failed_parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    if debug.clone() {
        println!("{}", (literal!("FIXING CONSTRAINTS\n\n")).clone());
    }
    fixConstraints(optimal_values.clone(), c2pi.clone(), p2ci.clone(), failed_parameters.clone(), Arc::new(todo!("PARTEVALFUNCTION of intLe: named args do not match any formal: ["i2"]")))?;
    fixConstraints(optimal_values.clone(), c2pe.clone(), p2ce.clone(), failed_parameters.clone(), Arc::new(todo!("PARTEVALFUNCTION of intEq: named args do not match any formal: ["i2"]")))?;
    if debug.clone() {
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

fn fixConstraints(mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut c2p: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut p2c: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>, mut failed_parameters: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut func: Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>) -> Result<()> {
    pub type checkVal = std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>;

    let mut parsed_constraints: Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(Expression::hash, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<i32> + 'static>), (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>), 13);
    let mut constraint: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut old_optimal_value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut solved_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut status: Solve::Status = Solve::Status::UNPROCESSED;
    let mut value: i32 = 0;
    let mut failed: bool = false;
    for mut tpl in &*UnorderedMap::toList(c2p.clone()) {
        let mut tpl = tpl.clone();
        (constraint, crefs) = tpl.clone();
        let () = (match checkConstraint(constraint.clone(), optimal_values.clone())? {
        Some(mut value) if (func(value.clone())?) => {
            if debug.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(constraint.clone())?); __mm_s.push_str(&*literal!(" || is not violated ")); __mm_s.push_str(&*intString(value.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            ()
        },
        Some(mut value) => {
            if debug.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(constraint.clone())?); __mm_s.push_str(&*literal!(" || is violated by ")); __mm_s.push_str(&*intString(value.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            eqn = Equation::makeAssignmentEqn(constraint.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Arc::new(crate::NBEquation::Iterator::EMPTY), NBEquation::default(EquationKind::DISCRETE.clone(), false, None, None))?;
            for mut cref in &*crefs.clone() {
                let mut cref = cref.clone();
                failed = false;
                (solved_eqn, status, _) = Solve::solveBody(eqn.clone(), cref.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1))?;
                if status.clone() == Solve::Status::EXPLICIT.clone() {
                    let () = (match checkConstraint(Util::getOption(Equation::getRHS(solved_eqn.clone())?)?, optimal_values.clone())? {
        Some(mut value) => {
            old_optimal_value = UnorderedMap::getSafe(cref.clone(), optimal_values.clone(), metamodelica::sourceInfo!())?;
            UnorderedMap::add(cref.clone(), Arc::new(Expression::NFExpression::INTEGER { value: value.clone() }), optimal_values.clone())?;
            for mut cons in &*UnorderedMap::getSafe(cref.clone(), p2c.clone(), metamodelica::sourceInfo!())? {
                let mut cons = cons.clone();
                if UnorderedSet::contains(constraint.clone(), parsed_constraints.clone())? && !(func(Util::getOptionOrDefault(checkConstraint(cons.clone(), optimal_values.clone())?, 1))?) {
                    failed = true;
                    break;
                }
            }
            if failed.clone() {
                UnorderedMap::add(cref.clone(), old_optimal_value.clone(), optimal_values.clone())?;
            }
            ()
        },
        _ => (),
    });
                } else {
                    failed = true;
                }
                if !(failed.clone()) {
                    UnorderedSet::add(constraint.clone(), parsed_constraints.clone())?;
                    break;
                }
            }
            ()
        },
        _ => {
            for mut cref in &*crefs.clone() {
                let mut cref = cref.clone();
                UnorderedSet::add(cref.clone(), failed_parameters.clone())?;
            }
            ()
        },
    });
        if failed.clone() {
            for mut cref in &*crefs.clone() {
                let mut cref = cref.clone();
                UnorderedSet::add(cref.clone(), failed_parameters.clone())?;
            }
        }
    }
    Ok(())
}

fn checkConstraint(mut constraint: Arc<Expression::NFExpression>, mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Option<i32>> {
    let mut value: Option<i32> = None;
    let mut replaced: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    replaced = Expression::map(constraint.clone(), Arc::new({ let __pe_b1 = optimal_values.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
    replaced = SimplifyExp::simplify(replaced.clone(), false)?;
    value = (::match_deref::match_deref! { match &(replaced.clone()) {
        Deref @ Expression::INTEGER { .. } => Some(var_field!((*replaced).value, Expression::NFExpression::INTEGER).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

fn updateDimension(mut dim: Arc<Dimension::NFDimension>, mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = dim;
    dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::RESIZABLE { .. } => {
            assign_variant_field!(dim => Dimension::NFDimension::RESIZABLE; opt_size = checkConstraint(var_field!((*dim).exp, Dimension::NFDimension::RESIZABLE).clone(), optimal_values.clone())?);
            dim.clone()
        },
        _ => dim.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

fn optimalValuesToString(mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut param: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut old_vals: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut new_vals: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut old: ArcStr = arcstr::literal!("");
    let mut new: ArcStr = arcstr::literal!("");
    let mut names_len: i32 = 0;
    for mut tpl in &*UnorderedMap::toList(optimal_values.clone()) {
        let mut tpl = tpl.clone();
        (param, value) = tpl.clone();
        var = Pointer::access(BVariable::getVarPointer(param.clone(), metamodelica::sourceInfo!())?);
        names = cons((ComponentRef::toString(param.clone())?).clone(), names.clone());
        new_vals = cons((Expression::toString(value.clone())?).clone(), new_vals.clone());
        old_vals = cons((Binding::toString(var.binding.clone(), (literal!("")).clone())?).clone(), old_vals.clone());
    }
    names_len = ({
        let mut __acc: Option<i32> = None;
        for mut n in (names.clone()).into_iter().cloned() {
            let __x = ((n.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    });
    while !(names.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(names.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa0.clone();
        names = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(new_vals.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        new = __pa2.clone();
        new_vals = __pa3.clone();
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(old_vals.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        old = __pa4.clone();
        old_vals = __pa5.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), names_len.clone() + 5 - ((name.clone()).clone().len() as i32))); __mm_s.push_str(&*literal!(" OPTIMAL: ")); __mm_s.push_str(&*new.clone()); __mm_s.push_str(&*literal!(" (ORIGINAL: ")); __mm_s.push_str(&*old.clone()); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
    }
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn occurencesToString(mut occs: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    for mut tpl in &*UnorderedMap::toList(occs.clone()) {
        let mut tpl = tpl.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(Util::tuple21(tpl.clone()))?); __mm_s.push_str(&*literal!(": {")); __mm_s.push_str(&*UnorderedSet::toString(Util::tuple22(tpl.clone()), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

fn distancesToString(mut tpl: (Arc<ComponentRef::NFComponentRef>, i32)) -> ArcStr {
    let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(Util::tuple21(tpl.clone())).unwrap()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(Util::tuple22(tpl.clone()))); ArcStr::from(__mm_s) };
    r#str
}

fn parametersToString(mut parameters: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> ArcStr {
    let mut r#str: ArcStr = List::toString(parameters.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0).unwrap();
    r#str
}

fn constraintsToString(mut constraints: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> ArcStr {
    let mut r#str: ArcStr = List::toString(constraints.clone(), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0).unwrap();
    r#str
}


