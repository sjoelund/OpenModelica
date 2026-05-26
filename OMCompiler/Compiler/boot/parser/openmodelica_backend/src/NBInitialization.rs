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

use crate::NBCausalize as Causalize;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBInline as Inline;
use crate::NBJacobian as Jacobian;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBPartitioning as Partitioning;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten as Flatten;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// Backend imports
// Util imports
pub fn createStartEquations(mut states: Arc<VariablePointers::VariablePointers>, mut variables: Arc<VariablePointers::VariablePointers>, mut initialVars: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut initialEqs: Arc<EquationPointers::EquationPointers>, mut idx: Pointer::Pointer<i32>, mut algorithm_outputs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut r#str: ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<EquationPointers::EquationPointers>)> {
    let mut variables: Arc<VariablePointers::VariablePointers> = variables;
    let mut initialVars: Arc<VariablePointers::VariablePointers> = initialVars;
    let mut equations: Arc<EquationPointers::EquationPointers> = equations;
    let mut initialEqs: Arc<EquationPointers::EquationPointers> = initialEqs;
    let mut ptr_start_vars: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut ptr_start_vars_init: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut ptr_start_eqs: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    let mut start_eqs: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let _ = BVariable::VariablePointers::mapPtr(states.clone(), Arc::new({ let __pe_b1 = ptr_start_vars.clone(); let __pe_b2 = ptr_start_vars_init.clone(); let __pe_b3 = ptr_start_eqs.clone(); let __pe_b4 = idx.clone(); let __pe_b5 = algorithm_outputs.clone(); move |__pe_a0| createStartEquation(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?;
    start_eqs = Pointer::access(ptr_start_eqs.clone());
    variables = BVariable::VariablePointers::addList(Pointer::access(ptr_start_vars.clone()), variables.clone());
    initialVars = BVariable::VariablePointers::addList(Pointer::access(ptr_start_vars_init.clone()), initialVars.clone());
    equations = BEquation::EquationPointers::addList(start_eqs.clone(), equations.clone());
    initialEqs = BEquation::EquationPointers::addList(start_eqs.clone(), initialEqs.clone());
    if Flags::isSet(Flags::INITIALIZATION.clone())? && !(start_eqs.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(start_eqs.clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" Start Equations (")); __mm_s.push_str(&*intString((start_eqs.clone().len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) }).clone())).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), false, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((variables, initialVars, equations, initialEqs))
}

pub fn createStartEquation(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut ptr_start_vars: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut ptr_start_vars_init: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut ptr_start_eqs: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut idx: Pointer::Pointer<i32>, mut algorithm_outputs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    if !(UnorderedSet::contains(BVariable::getVarName(var.clone()), algorithm_outputs.clone())?) {
        let () = (::match_deref::match_deref! { match &(Pointer::access(var.clone())) {
        Deref @ Variable::VARIABLE { .. } if (BVariable::isArray(var.clone())) => {
            if BVariable::isFixed(var.clone()) {
                createStartEquationSlice(Arc::new(Slice::NBSlice { t: var.clone(), indices: metamodelica::nil() }), ptr_start_vars.clone(), ptr_start_eqs.clone(), idx.clone(), BVariable::isFixed(var.clone()))?;
            } else {
                createStartEquationSlice(Arc::new(Slice::NBSlice { t: var.clone(), indices: metamodelica::nil() }), ptr_start_vars_init.clone(), ptr_start_eqs.clone(), idx.clone(), BVariable::isFixed(var.clone()))?;
            }
            ()
        },
        Deref @ Variable::VARIABLE { .. } if (BVariable::isFixed(var.clone())) => {
            let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut start_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut start_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut start_eq: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut kind: EquationKind = EquationKind::CONTINUOUS;
            let mut start_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            name = BVariable::getVarName(var.clone());
            start_exp = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var.clone())) {
        Some(e) if (!(Expression::isLiteralXML(e.clone()))) => {
            e.clone()
        },
        _ => {
            (_, name, start_var, start_name) = createStartVar(var.clone(), name.clone(), metamodelica::nil())?;
            Pointer::update(ptr_start_vars.clone(), cons(start_var.clone(), Pointer::access(ptr_start_vars.clone())));
            Expression::fromCref(start_name.clone(), false)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            kind = if (BVariable::isContinuous(var.clone(), true)?) {EquationKind::CONTINUOUS.clone()} else {EquationKind::DISCRETE.clone()};
            start_eq = BEquation::Equation::makeAssignment(Expression::fromCref(name.clone(), false)?, start_exp.clone(), idx.clone(), (arcstr::literal!(BEquation::START_STR)).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), BEquation::default(kind.clone(), true, None, None))?;
            Pointer::update(ptr_start_eqs.clone(), cons(start_eq.clone(), Pointer::access(ptr_start_eqs.clone())));
            ()
        },
        Deref @ Variable::VARIABLE { .. } => {
            let mut start_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut start_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut start_eq: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut kind: EquationKind = EquationKind::CONTINUOUS;
            let _ = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var.clone())) {
        Some(e) if (!(Expression::isLiteralXML(e.clone()))) => {
            (_, _, start_var, start_name) = createStartVar(var.clone(), BVariable::getVarName(var.clone()), metamodelica::nil())?;
            kind = if (BVariable::isContinuous(var.clone(), true)?) {EquationKind::CONTINUOUS.clone()} else {EquationKind::DISCRETE.clone()};
            start_eq = BEquation::Equation::makeAssignment(Expression::fromCref(start_name.clone(), false)?, e.clone(), idx.clone(), (arcstr::literal!(BEquation::START_STR)).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), BEquation::default(kind.clone(), true, None, None))?;
            Pointer::update(ptr_start_eqs.clone(), cons(start_eq.clone(), Pointer::access(ptr_start_eqs.clone())));
            Pointer::update(ptr_start_vars_init.clone(), cons(start_var.clone(), Pointer::access(ptr_start_vars_init.clone())));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

pub fn createWhenReplacementEquations(mut cref_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Iterator::Iterator>>>, mut equations: Arc<EquationPointers::EquationPointers>, mut initialEqs: Arc<EquationPointers::EquationPointers>, mut idx: Pointer::Pointer<i32>) -> Result<(Arc<EquationPointers::EquationPointers>, Arc<EquationPointers::EquationPointers>)> {
    let mut equations: Arc<EquationPointers::EquationPointers> = equations;
    let mut initialEqs: Arc<EquationPointers::EquationPointers> = initialEqs;
    let mut ptr_start_eqs: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    let mut start_eqs: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    for mut tpl in &*UnorderedMap::toList(cref_map.clone()) {
        let mut tpl = tpl.clone();
        createWhenReplacementEquation(tpl.clone(), ptr_start_eqs.clone(), idx.clone())?;
    }
    start_eqs = Pointer::access(ptr_start_eqs.clone());
    equations = BEquation::EquationPointers::addList(start_eqs.clone(), equations.clone());
    initialEqs = BEquation::EquationPointers::addList(start_eqs.clone(), initialEqs.clone());
    if Flags::isSet(Flags::INITIALIZATION.clone())? && !(start_eqs.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(start_eqs.clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created When Replacement Equations (")); __mm_s.push_str(&*intString((start_eqs.clone().len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) }).clone())).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), false, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((equations, initialEqs))
}

pub fn createWhenReplacementEquation(mut tpl: (Arc<ComponentRef::NFComponentRef>, Arc<Iterator::Iterator>), mut ptr_start_eqs: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut idx: Pointer::Pointer<i32>) -> Result<()> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut var_pre: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut pre: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut subscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    let mut kind: EquationKind = EquationKind::CONTINUOUS;
    let mut eq: Pointer::Pointer<Arc<Equation::Equation>>;
    (cref, iter) = tpl.clone();
    var_ptr = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
    (var_pre, _) = BVariable::getVarPre(var_ptr.clone());
    if Util::isSome(var_pre.clone()) {
        pre = BVariable::getVarName(Util::getOption(var_pre.clone())?);
        pre = ComponentRef::copySubscripts(cref.clone(), pre.clone())?;
        kind = if (BVariable::isContinuous(var_ptr.clone(), true)?) {EquationKind::CONTINUOUS.clone()} else {EquationKind::DISCRETE.clone()};
        eq = BEquation::Equation::makeAssignment(Expression::fromCref(cref.clone(), true)?, Expression::fromCref(pre.clone(), true)?, idx.clone(), (arcstr::literal!(BEquation::START_STR)).clone(), iter.clone(), BEquation::default(kind.clone(), true, None, None))?;
        Pointer::update(ptr_start_eqs.clone(), cons(eq.clone(), Pointer::access(ptr_start_eqs.clone())));
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInitialization.createWhenReplacementEquation")); __mm_s.push_str(&*literal!(" could not replace when-replacement for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" because it has no pre-variable.")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(())
}

pub fn createStartVar(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut name: Arc<ComponentRef::NFComponentRef>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut name: Arc<ComponentRef::NFComponentRef> = name;
    let mut start_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut start_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let (mut var_pre, _): (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) = BVariable::getVarPre(var_ptr.clone());
    let mut disc_state_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut merged_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    if BVariable::isPrevious(var_ptr.clone()) && Util::isSome(var_pre.clone()) {
        merged_name = BVariable::getVarName(Util::getOption(var_pre.clone())?);
        merged_name = ComponentRef::mergeSubscripts(subscripts.clone(), merged_name.clone(), true, true, true)?;
    } else if Util::isSome(var_pre.clone()) {
        merged_name = ComponentRef::mergeSubscripts(subscripts.clone(), name.clone(), true, true, true)?;
        var_ptr = Util::getOption(var_pre.clone())?;
        name = BVariable::getVarName(var_ptr.clone());
        name = ComponentRef::mergeSubscripts(subscripts.clone(), name.clone(), true, true, true)?;
    } else {
        name = ComponentRef::mergeSubscripts(subscripts.clone(), name.clone(), true, true, true)?;
        merged_name = name.clone();
    }
    (start_name, start_var) = BVariable::makeStartVar(merged_name.clone())?;
    start_var = (match BVariable::getParent(var_ptr.clone()) {
        Some(mut parent) => {
            let mut start_parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
            start_parent = (match BVariable::getVarStart(parent.clone()) {
        Some(mut start_parent) => start_parent.clone(),
        _ => {
            (_, _, start_parent, _) = createStartVar(parent.clone(), BVariable::getVarName(parent.clone()), metamodelica::nil())?;
            start_parent.clone()
        },
    });
            BVariable::addRecordChild(start_parent.clone(), start_var.clone())?;
            start_var = BVariable::setParent(start_var.clone(), start_parent.clone());
            start_var.clone()
        },
        _ => {
            start_var.clone()
        },
    });
    Ok((var_ptr, name, start_var, start_name))
}

pub fn createParameterEquations(mut parameters: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut initialEqs: Arc<EquationPointers::EquationPointers>, mut initialVars: Arc<VariablePointers::VariablePointers>, mut new_iters: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut idx: Pointer::Pointer<i32>, mut r#str: ArcStr) -> Result<(Arc<EquationPointers::EquationPointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>)> {
    let mut equations: Arc<EquationPointers::EquationPointers> = equations;
    let mut initialEqs: Arc<EquationPointers::EquationPointers> = initialEqs;
    let mut initialVars: Arc<VariablePointers::VariablePointers> = initialVars;
    let mut parameter_eqs: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut initial_param_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut skip: bool = false;
    for mut var in &*BVariable::VariablePointers::toList(parameters.clone())? {
        let mut var = var.clone();
        (parameter_eqs, initial_param_vars) = createParameterEquation(var.clone(), new_iters.clone(), idx.clone(), parameter_eqs.clone(), initial_param_vars.clone())?;
    }
    equations = BEquation::EquationPointers::addList(parameter_eqs.clone(), equations.clone());
    initialEqs = BEquation::EquationPointers::addList(parameter_eqs.clone(), initialEqs.clone());
    initialVars = BVariable::VariablePointers::addList(initial_param_vars.clone(), initialVars.clone());
    if Flags::isSet(Flags::INITIALIZATION.clone())? && !(parameter_eqs.clone().is_empty()) || Flags::isSet(Flags::DUMP_BINDINGS.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(parameter_eqs.clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("Parameter Binding Equations (")); __mm_s.push_str(&*intString((parameter_eqs.clone().len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) }).clone())).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), false, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((equations, initialEqs, initialVars))
}

pub fn createParameterEquation(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut new_iters: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut idx: Pointer::Pointer<i32>, mut parameter_eqs: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut initial_param_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>)> {
    let mut parameter_eqs: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = parameter_eqs;
    let mut initial_param_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = initial_param_vars;
    let mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut skip: bool = false;
    if BVariable::isConst(var.clone()) {
        skip = true;
    } else {
        skip = (match BVariable::getParent(var.clone()) {
        Some(mut parent) => BVariable::isBound(parent.clone()) && BVariable::isKnownRecord(parent.clone()),
        _ => BVariable::isRecord(var.clone()) && !(BVariable::isBound(var.clone())),
    });
    }
    if skip.clone() {
        return Ok((parameter_eqs, initial_param_vars));
    }
    if BVariable::isKnownRecord(var.clone()) {
        if !(BVariable::hasEvaluableBinding(var.clone())?) && (BVariable::isBound(var.clone()) || BVariable::hasStartAttr(var.clone())) {
            initial_param_vars = listAppend(BVariable::getRecordChildren(var.clone()), initial_param_vars.clone());
            parameter_eqs = cons(BEquation::Equation::generateBindingEquation(var.clone(), idx.clone(), true, new_iters.clone())?, parameter_eqs.clone());
        } else {
            for mut c_var in &*BVariable::getRecordChildren(var.clone()) {
                let mut c_var = c_var.clone();
                if BVariable::isBound(c_var.clone()) {
                    BVariable::setBindingAsStart(c_var.clone(), true)?;
                }
                (parameter_eqs, initial_param_vars) = createParameterEquation(c_var.clone(), new_iters.clone(), idx.clone(), parameter_eqs.clone(), initial_param_vars.clone())?;
            }
        }
    } else if !(BVariable::isRecord(var.clone())) {
        if !(BVariable::hasEvaluableBinding(var.clone())?) {
            initial_param_vars = cons(var.clone(), initial_param_vars.clone());
            if BVariable::isFixed(var.clone()) {
                parameter_eqs = cons(BEquation::Equation::generateBindingEquation(var.clone(), idx.clone(), true, new_iters.clone())?, parameter_eqs.clone());
            }
        } else if BVariable::isBound(var.clone()) {
            BVariable::setBindingAsStart(var.clone(), true)?;
        }
    }
    Ok((parameter_eqs, initial_param_vars))
}

pub fn createStartEquationSlice(mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut ptr_start_vars: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut ptr_start_eqs: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut idx: Pointer::Pointer<i32>, mut fixed: bool) -> Result<()> {
    let mut start_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut start_var_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut start_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut start_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut start_eq: Option<Pointer::Pointer<Arc<Equation::Equation>>> = None;
    let mut kind: EquationKind = EquationKind::CONTINUOUS;
    let mut iterator: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut sliced_eqn: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    var_ptr = Slice::getT(var_slice.clone());
    name = BVariable::getVarName(var_ptr.clone());
    kind = if (BVariable::isContinuous(var_ptr.clone(), true)?) {EquationKind::CONTINUOUS.clone()} else {EquationKind::DISCRETE.clone()};
    if fixed.clone() {
        start_exp = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var_ptr.clone())) {
        Some(e) if (!(Expression::isLiteralXML(e.clone()))) => {
            (start_exp, var_ptr, name, _, _, iterator) = createStartExpressionSlice(e.clone(), var_slice.clone(), var_ptr.clone(), name.clone())?;
            start_exp.clone()
        },
        _ => {
            (start_var_exp, var_ptr, name, iterator) = createStartVariableSlice(var_slice.clone(), var_ptr.clone(), name.clone(), ptr_start_vars.clone())?;
            start_var_exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        start_eq = Some(BEquation::Equation::makeAssignment(Expression::fromCref(name.clone(), true)?, start_exp.clone(), idx.clone(), (arcstr::literal!(BEquation::START_STR)).clone(), iterator.clone(), BEquation::default(kind.clone(), true, None, None))?);
    } else {
        start_eq = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var_ptr.clone())) {
        Some(e) if (!(Expression::isLiteralXML(e.clone()))) => {
            (start_exp, var_ptr, _, start_var, name, iterator) = createStartExpressionSlice(e.clone(), var_slice.clone(), var_ptr.clone(), name.clone())?;
            start_eq = Some(BEquation::Equation::makeAssignment(Expression::fromCref(name.clone(), true)?, start_exp.clone(), idx.clone(), (arcstr::literal!(BEquation::START_STR)).clone(), iterator.clone(), BEquation::default(kind.clone(), true, None, None))?);
            Pointer::update(ptr_start_vars.clone(), cons(start_var.clone(), Pointer::access(ptr_start_vars.clone())));
            start_eq.clone()
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if Util::isSome(start_eq.clone()) {
        if !(var_slice.indices.clone().is_empty()) {
            (sliced_eqn, _) = BEquation::Equation::slice(Util::getOption(start_eq.clone())?, var_slice.indices.clone())?;
            Pointer::update(ptr_start_eqs.clone(), listAppend(Pointer::access(ptr_start_eqs.clone()), sliced_eqn.clone()));
        } else {
            Pointer::update(ptr_start_eqs.clone(), cons(Util::getOption(start_eq.clone())?, Pointer::access(ptr_start_eqs.clone())));
        }
    }
    Ok(())
}

pub fn createStartExpressionSlice(mut exp: Arc<Expression::NFExpression>, mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut name: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<Expression::NFExpression>, Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>, Arc<Iterator::Iterator>)> {
    let mut start_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut name: Arc<ComponentRef::NFComponentRef> = name;
    let mut start_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut start_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut iterator: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    (start_exp, iterator) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: array_constructor @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
            let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
            let mut old_iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut new_iter: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            (var_ptr, name, start_var, start_cref, _, frames, iterator) = createIteratedStartCref(var_ptr.clone(), name.clone(), (var_field!((**array_constructor).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone().len() as i32))?;
            replacements = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
            for mut tpl in &*List::zip(var_field!((**array_constructor).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), frames.clone()) {
                let mut tpl = tpl.clone();
                let ((__pa0, _), (__pa1, _, _)) = tpl.clone();
                old_iter = __pa0.clone();
                new_iter = __pa1.clone();
                UnorderedMap::add(ComponentRef::fromNode(old_iter.clone(), InstNode::getType(old_iter.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()), Expression::fromCref(new_iter.clone(), false)?, replacements.clone())?;
            }
            (Expression::map(var_field!((**array_constructor).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?, iterator.clone())
        },
        _ => {
            let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            if Slice::isFull(var_slice.clone()) {
                (var_ptr, name, start_var, start_cref) = createStartVar(var_ptr.clone(), name.clone(), metamodelica::nil())?;
                iterator = Arc::new(crate::NBEquation::Iterator::EMPTY);
                start_exp = exp.clone();
            } else {
                (var_ptr, name, start_var, start_cref, subscripts, _, iterator) = createIteratedStartCref(var_ptr.clone(), name.clone(), 0)?;
                start_exp = Expression::applySubscripts(subscripts.clone(), exp.clone(), true)?;
            }
            (start_exp.clone(), iterator.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start_exp, var_ptr, name, start_var, start_cref, iterator))
}

pub fn createStartVariableSlice(mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut name: Arc<ComponentRef::NFComponentRef>, mut ptr_start_vars: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>) -> Result<(Arc<Expression::NFExpression>, Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>, Arc<Iterator::Iterator>)> {
    let mut start_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut name: Arc<ComponentRef::NFComponentRef> = name;
    let mut iterator: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut start_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut start_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    if Slice::isFull(var_slice.clone()) {
        (var_ptr, name, start_var, start_name) = createStartVar(var_ptr.clone(), name.clone(), metamodelica::nil())?;
        iterator = Arc::new(crate::NBEquation::Iterator::EMPTY);
    } else {
        (var_ptr, name, start_var, start_name, subscripts, _, iterator) = createIteratedStartCref(var_ptr.clone(), name.clone(), 0)?;
    }
    Pointer::update(ptr_start_vars.clone(), cons(start_var.clone(), Pointer::access(ptr_start_vars.clone())));
    start_exp = Expression::fromCref(start_name.clone(), false)?;
    Ok((start_exp, var_ptr, name, iterator))
}

fn createIteratedStartCref(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut name: Arc<ComponentRef::NFComponentRef>, mut num_dim: i32) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, Arc<Iterator::Iterator>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut name: Arc<ComponentRef::NFComponentRef> = name;
    let mut start_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut start_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
    let mut iterator: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut iterators: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut iter_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    dims = Type::arrayDims(ComponentRef::getSubscriptedType(name.clone(), false)?);
    dims = if (num_dim.clone() == 0) {dims.clone()} else {List::firstN(dims.clone(), num_dim.clone())?};
    (iterators, ranges, subscripts) = Flatten::makeIterators(name.clone(), dims.clone())?;
    iter_crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut iter in (iterators.clone()).into_iter().cloned() {
            let __x = ComponentRef::makeIterator(iter.clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    iter_crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut iter in (iter_crefs.clone()).into_iter().cloned() {
            let __x = BackendDAE::lowerIteratorCref(iter.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut sub in (subscripts.clone()).into_iter().cloned() {
            let __x = Subscript::mapExp(sub.clone(), Arc::new(fnptr!(BackendDAE::lowerIteratorExp, Arc<Expression::NFExpression>)))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    frames = List::zip3(iter_crefs.clone(), ranges.clone(), List::fill(None, (iter_crefs.clone().len() as i32)));
    iterator = BEquation::Iterator::fromFrames(frames.clone());
    (var_ptr, name, start_var, start_cref) = createStartVar(var_ptr.clone(), name.clone(), subscripts.clone())?;
    Ok((var_ptr, name, start_var, start_cref, subscripts, frames, iterator))
}

pub fn createPreEquation(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut ptr_pre_eqs: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut idx: Pointer::Pointer<i32>) -> Result<()> {
    let mut pre: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut pre_eq: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut kind: EquationKind = EquationKind::CONTINUOUS;
    if !(BVariable::isPrevious(var_ptr.clone())) {
        (pre, _) = BVariable::getVarPre(var_ptr.clone());
        if Util::isSome(pre.clone()) {
            kind = if (BVariable::isContinuous(var_ptr.clone(), true)?) {EquationKind::CONTINUOUS.clone()} else {EquationKind::DISCRETE.clone()};
            pre_eq = BEquation::Equation::makeAssignment(Expression::fromCref(BVariable::getVarName(var_ptr.clone()), false)?, Expression::fromCref(BVariable::getVarName(Util::getOption(pre.clone())?), false)?, idx.clone(), (arcstr::literal!(BEquation::PRE_STR)).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), BEquation::default(kind.clone(), true, None, None))?;
            Pointer::update(ptr_pre_eqs.clone(), cons(pre_eq.clone(), Pointer::access(ptr_pre_eqs.clone())));
        }
    }
    Ok(())
}

pub fn createPreEquationSlice(mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut ptr_pre_eqs: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut idx: Pointer::Pointer<i32>) -> Result<()> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut pre: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut pre_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut iterators: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
    let mut pre_eq: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut kind: EquationKind = EquationKind::CONTINUOUS;
    let mut sliced_eqn: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    var_ptr = Slice::getT(var_slice.clone());
    if !(BVariable::isPrevious(var_ptr.clone())) {
        (pre, _) = BVariable::getVarPre(var_ptr.clone());
        if Util::isSome(pre.clone()) {
            name = BVariable::getVarName(var_ptr.clone());
            dims = Type::arrayDims(ComponentRef::getSubscriptedType(name.clone(), false)?);
            (iterators, ranges, subscripts) = Flatten::makeIterators(name.clone(), dims.clone())?;
            frames = List::zip3({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut iter in (iterators.clone()).into_iter().cloned() {
            let __x = ComponentRef::makeIterator(iter.clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, ranges.clone(), List::fill(None, (ranges.clone().len() as i32)));
            pre_name = BVariable::getVarName(Util::getOption(pre.clone())?);
            pre_name = ComponentRef::mergeSubscripts(subscripts.clone(), pre_name.clone(), true, true, false)?;
            name = ComponentRef::mergeSubscripts(subscripts.clone(), name.clone(), true, true, false)?;
            kind = if (BVariable::isContinuous(var_ptr.clone(), true)?) {EquationKind::CONTINUOUS.clone()} else {EquationKind::DISCRETE.clone()};
            pre_eq = BEquation::Equation::makeAssignment(Expression::fromCref(name.clone(), true)?, Expression::fromCref(pre_name.clone(), false)?, idx.clone(), (arcstr::literal!(BEquation::PRE_STR)).clone(), BEquation::Iterator::fromFrames(frames.clone()), BEquation::default(kind.clone(), true, None, None))?;
            if !(var_slice.indices.clone().is_empty()) {
                (sliced_eqn, _) = BEquation::Equation::slice(pre_eq.clone(), var_slice.indices.clone())?;
                Pointer::update(ptr_pre_eqs.clone(), listAppend(Pointer::access(ptr_pre_eqs.clone()), sliced_eqn.clone()));
            } else {
                Pointer::update(ptr_pre_eqs.clone(), cons(pre_eq.clone(), Pointer::access(ptr_pre_eqs.clone())));
            }
        }
    }
    Ok(())
}

pub fn cleanupInitialCall(mut eq: Arc<Equation::Equation>, mut kind: BPartition::Kind) -> Result<Arc<Equation::Equation>> {
    fn cleanupInitialCallExp(mut exp: Arc<Expression::NFExpression>, mut kind: BPartition::Kind, mut simplify: Pointer::Pointer<bool>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        if Expression::isCallNamed(exp.clone(), (literal!("initial")).clone())? {
            exp = Arc::new(Expression::NFExpression::BOOLEAN { value: kind.clone() == BPartition::Kind::INI.clone() || kind.clone() == BPartition::Kind::INI_0.clone() });
            Pointer::update(simplify.clone(), true);
        } else if Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("initialSimplified")).clone())? && Expression::isCallNamed(exp.clone(), (literal!("initialSimplified")).clone())? {
            exp = Arc::new(Expression::NFExpression::BOOLEAN { value: kind.clone() == BPartition::Kind::INI_0.clone() });
            Pointer::update(simplify.clone(), true);
        }
        Ok(exp)
    }

    let mut eq: Arc<Equation::Equation> = eq;
    let mut simplify: Pointer::Pointer<bool> = Pointer::create(false);
    eq = BEquation::Equation::map(eq.clone(), { let __pe_b1 = kind.clone(); let __pe_b2 = simplify.clone(); move |__pe_a0| cleanupInitialCallExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }, None, Expression::map)?;
    if Pointer::access(simplify.clone()) {
        eq = BEquation::Equation::simplify(eq.clone(), (literal!("")).clone(), (literal!("")).clone(), Pointer::create(metamodelica::nil()), Pointer::create(metamodelica::nil()), Arc::new({ let __pe_b1 = true; let __pe_b2 = (literal!("")).clone(); let __pe_b3 = (literal!("")).clone(); move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    }
    Ok(eq)
}

pub fn cleanupHomotopy(mut exp: Arc<Expression::NFExpression>, mut kind: BPartition::Kind) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } if (Call::isNamed(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), (literal!("homotopy")).clone())?) => (match kind.clone() {
        BPartition::Kind::INI_0 => (Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?).get(2)?,
        BPartition::Kind::INI => exp.clone(),
        _ => listHead(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?)?,
    }),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn containsHomotopyCall(mut exp: Arc<Expression::NFExpression>, mut b: Pointer::Pointer<bool>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    if !(Pointer::access(b.clone())) && Expression::isCallNamed(exp.clone(), (literal!("homotopy")).clone())? {
        Pointer::update(b.clone(), true);
    }
    Ok(exp)
}

pub fn containsLambda0(mut exp: Arc<Expression::NFExpression>, mut b: Pointer::Pointer<bool>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    if !(Pointer::access(b.clone())) && (Expression::isCallNamed(exp.clone(), (literal!("homotopy")).clone())? || Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("initialSimplified")).clone())? && Expression::isCallNamed(exp.clone(), (literal!("initialSimplified")).clone())?) {
        Pointer::update(b.clone(), true);
    }
    Ok(exp)
}

pub fn removeWhenEquation(mut eqn: Arc<Equation::Equation>, mut iter: Arc<Iterator::Iterator>, mut cref_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Iterator::Iterator>>>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::FOR_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::Equation>>> = metamodelica::nil();
        for mut b in (var_field!((*eqn).body, Equation::Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = removeWhenEquation(b.clone(), var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), cref_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if (List::all(var_field!((*eqn).body, Equation::Equation::FOR_EQUATION).clone(), Arc::new(fnptr!(Equation::isDummy, Arc<Equation::Equation>)))) {Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION)} else {eqn.clone()}
        },
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            let mut new_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut lhs_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            stmts = removeWhenEquationBody(Some(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone()))?;
            if !(stmts.clone().is_empty()) {
                new_eqn = Pointer::access(BEquation::Equation::makeAlgorithm(stmts.clone(), true)?);
                new_eqn = BEquation::Equation::setResidualVar(new_eqn.clone(), BEquation::Equation::getResidualVar(Pointer::create(eqn.clone()))?)?;
            } else {
                lhs_crefs = BEquation::WhenEquationBody::getAllAssigned(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone());
                for mut cref in &*lhs_crefs.clone() {
                    let mut cref = cref.clone();
                    UnorderedMap::add(cref.clone(), iter.clone(), cref_map.clone())?;
                }
                new_eqn = Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION);
            }
            new_eqn.clone()
        },
        Deref @ BEquation::Equation::IF_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::Equation::IF_EQUATION;
                body = removeWhenEquationIfBody(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), iter.clone(), cref_map.clone()),
                size = BEquation::IfEquationBody::size(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), false)
            );
            if (var_field!((*eqn).size, Equation::Equation::IF_EQUATION).clone() > 0) {eqn.clone()} else {Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION)}
        },
        Deref @ BEquation::Equation::ALGORITHM { .. } => {
            let mut new_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            stmts = removeWhenEquationAlgorithmBody(var_field!((*eqn).alg, Equation::Equation::ALGORITHM).statements.clone())?;
            if !(stmts.clone().is_empty()) {
                new_eqn = Pointer::access(BEquation::Equation::makeAlgorithm(stmts.clone(), true)?);
                new_eqn = BEquation::Equation::setResidualVar(new_eqn.clone(), BEquation::Equation::getResidualVar(Pointer::create(eqn.clone()))?)?;
            } else {
                new_eqn = Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION);
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

pub fn removeWhenEquationBody(mut body_opt: Option<Arc<WhenEquationBody::WhenEquationBody>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    stmts = (::match_deref::match_deref! { match &(body_opt.clone()) {
        Some(body) => {
            if isInitialCall(body.condition.clone())? {
                stmts = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut st in (body.when_stmts.clone()).into_iter().cloned() {
            let __x = BEquation::WhenStatement::toStatement(st.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            } else {
                stmts = removeWhenEquationBody(body.else_when.clone())?;
            }
            stmts.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmts)
}

pub fn removeWhenEquationIfBody(mut body: Arc<IfEquationBody::IfEquationBody>, mut iter: Arc<Iterator::Iterator>, mut cref_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Iterator::Iterator>>>) -> Arc<IfEquationBody::IfEquationBody> {
    let mut body: Arc<IfEquationBody::IfEquationBody> = body;
    assign_field!(
        body.then_eqns = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut e in (body.then_eqns.clone()).into_iter().cloned() {
            let __x = Pointer::apply(e.clone(), Arc::new({ let __pe_b1 = iter.clone(); let __pe_b2 = cref_map.clone(); move |__pe_a0| removeWhenEquation(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        body.else_if = Util::applyOption(body.else_if.clone(), Arc::new({ let __pe_b1 = iter.clone(); let __pe_b2 = cref_map.clone(); move |__pe_a0| Ok(removeWhenEquationIfBody(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }))
    );
    body
}

pub fn removeWhenEquationAlgorithmBody(mut in_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut out_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut condition_set: Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>> = UnorderedSet::new(fnptr!(Expression::hash, Arc<Expression::NFExpression>), Expression::isEqual, 13);
    let mut tail_stmts_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> = Pointer::create(metamodelica::nil());
    out_stmts = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
        for mut stmt in (in_stmts.clone()).into_iter().cloned() {
            let __x = removeWhenEquationStatement(stmt.clone(), condition_set.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    out_stmts = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
        for mut stmt in (out_stmts.clone()).into_iter().cloned() {
            let __x = removeConditionEquation(stmt.clone(), condition_set.clone(), tail_stmts_ptr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    out_stmts = listAppend(out_stmts.clone(), Pointer::access(tail_stmts_ptr.clone()));
    Ok(out_stmts)
}

pub fn removeWhenEquationStatement(mut stmt: Arc<Statement::NFStatement>, mut condition_set: Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut out_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    out_stmts = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::WHEN { .. } => {
            let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut stmts_acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
            for mut tpl in &*var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone() {
                let mut tpl = tpl.clone();
                (cond, stmts) = tpl.clone();
                if isInitialCall(cond.clone())? {
                    out_stmts = stmts.clone();
                }
                collectNonInitial(cond.clone(), condition_set.clone())?;
            }
            out_stmts.clone()
        },
        Deref @ Statement::FOR { .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut stmts_acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
            for mut body_stmt in &*var_field!((*stmt).body, Statement::NFStatement::FOR).clone().reverse() {
                let mut body_stmt = body_stmt.clone();
                stmts_acc = cons(removeWhenEquationStatement(body_stmt.clone(), condition_set.clone())?, stmts_acc.clone());
            }
            stmts = List::flatten(stmts_acc.clone());
            if !(stmts.clone().is_empty()) {
                assign_variant_field!(stmt => Statement::NFStatement::FOR; body = stmts.clone());
                out_stmts = list![stmt.clone()];
            } else {
                out_stmts = metamodelica::nil();
            }
            out_stmts.clone()
        },
        _ => {
            let mut stmts_acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
            list![stmt.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_stmts)
}

pub fn removeConditionEquation(mut stmt: Arc<Statement::NFStatement>, mut condition_set: Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>, mut tail_stmts_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut out_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    out_stmts = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } if (UnorderedSet::contains(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), condition_set.clone())?) => {
            let mut pre_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut post_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut tail_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            pre_set = UnorderedSet::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 13);
            Expression::map(var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), Arc::new({ let __pe_b1 = pre_set.clone(); move |__pe_a0| findPreVars(__pe_a0, __pe_b1.clone()) }))?;
            if UnorderedSet::isEmpty(pre_set.clone()) {
                out_stmts = list![stmt.clone()];
            } else {
                tail_stmts = cons(stmt.clone(), Pointer::access(tail_stmts_ptr.clone()));
                for mut pre_cref in &*UnorderedSet::toList(pre_set.clone()) {
                    let mut pre_cref = pre_cref.clone();
                    post_cref = BVariable::getPartnerCref(pre_cref.clone(), fnptr!(BVariable::getVarPre, Pointer::Pointer<Arc<Variable::NFVariable>>), false)?;
                    tail_stmts = cons(Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: Expression::fromCref(pre_cref.clone(), false)?, rhs: Expression::fromCref(post_cref.clone(), false)?, ty: ComponentRef::getSubscriptedType(pre_cref.clone(), false)?, source: DAE::emptyElementSource.clone() }), tail_stmts.clone());
                }
                Pointer::update(tail_stmts_ptr.clone(), tail_stmts.clone());
            }
            out_stmts.clone()
        },
        _ => {
            list![stmt.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_stmts)
}

pub fn findPreVars(mut exp: Arc<Expression::NFExpression>, mut pre_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (BVariable::isPrevious(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?)) => {
            UnorderedSet::add(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), pre_set.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn replaceClockedFunctionsEqn(mut eqn: Pointer::Pointer<Arc<Equation::Equation>>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
    let mut eqn: Pointer::Pointer<Arc<Equation::Equation>> = eqn;
    Pointer::update(eqn.clone(), BEquation::Equation::map(Pointer::access(eqn.clone()), replaceClockedFunctions, None, Expression::map)?);
    Ok(eqn)
}

pub fn replaceClockedFunctions(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)? == literal!("$getPart")) => {
            Expression::makeZero(Expression::typeOf(exp.clone()))?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn isInitialCall(mut condition: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(condition.clone()) {
        Deref @ Expression::CALL { .. } => Call::isNamed(var_field!((*condition).call, Expression::NFExpression::CALL).clone(), (literal!("initial")).clone())?,
        Deref @ Expression::LBINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::OR, .. }, .. } => isInitialCall(var_field!((*condition).exp1, Expression::NFExpression::LBINARY).clone())? || isInitialCall(var_field!((*condition).exp2, Expression::NFExpression::LBINARY).clone())?,
        Deref @ Expression::ARRAY { .. } => Array::any(var_field!((*condition).elements, Expression::NFExpression::ARRAY).clone(), Arc::new(isInitialCall)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn collectNonInitial(mut condition: Arc<Expression::NFExpression>, mut condition_set: Arc<UnorderedSet::UnorderedSet<Arc<Expression::NFExpression>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(condition.clone()) {
        Deref @ Expression::CREF { .. } => {
            UnorderedSet::add(condition.clone(), condition_set.clone())?;
            ()
        },
        Deref @ Expression::ARRAY { .. } => {
            let __range0 = var_field!((*condition).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut elem in __range0 {
                collectNonInitial(elem.clone(), condition_set.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn collectAlgorithmOutputs(mut eqn: Arc<Equation::Equation>, mut outputs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let _ = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::ALGORITHM { alg, .. } => {
            let mut out_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            out_crefs = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut o in (alg.outputs.clone()).into_iter().cloned() {
            let __x = BVariable::getRecordChildrenCrefOrSelf(o.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut cr in &*out_crefs.clone() {
                let mut cr = cr.clone();
                UnorderedSet::add(cr.clone(), outputs.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

