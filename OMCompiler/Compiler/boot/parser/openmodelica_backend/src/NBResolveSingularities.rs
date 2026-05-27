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

use crate::NBAdjacency as Adjacency;
use crate::NBBackendUtil as BackendUtil;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBEquation::SlicingStatus;
use crate::NBFunctionAlias::Call_Aux;
use crate::NBInitialization as Initialization;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBSlice as Slice;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// NB imports
// util imports
fn getMSSS(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>, mut excluded_eqns: metamodelica::Array<bool>, mut mapping: Arc<Adjacency::Mapping::Mapping>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut msss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqn_candidates: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut color_clustering: metamodelica::Array<i32>;
    let mut eqn_coloring: metamodelica::Array<i32> = arrayCreate((m.clone().borrow().len() as i32), -1);
    let mut var_coloring: metamodelica::Array<i32> = arrayCreate((mT.clone().borrow().len() as i32), -1);
    let mut color: i32 = 0;
    let __range0 = 1..=(matching.eqn_to_var.clone().borrow().len() as i32);
    for mut eqn in __range0 {
        if matching.eqn_to_var[(eqn.clone()-1) as usize].clone() == -1 {
            eqn_candidates = cons(eqn.clone(), eqn_candidates.clone());
        }
    }
    color_clustering = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(eqn_candidates.clone().len() as i32)).into_iter() {
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
    for mut eqn in &*eqn_candidates.clone() {
        let mut eqn = eqn.clone();
        if eqn_coloring.borrow()[(eqn.clone()-1) as usize].clone() == -1 {
            color = color.clone() + 1;
            fillColorEqn(eqn.clone(), color.clone(), eqn_coloring.clone(), var_coloring.clone(), color_clustering.clone(), m.clone(), mT.clone(), matching.clone(), mapping.clone())?;
        }
    }
    resolveClustering(color_clustering.clone())?;
    msss = arrayCreate(color.clone(), metamodelica::nil());
    let __range1 = 1..=(eqn_coloring.clone().borrow().len() as i32);
    for mut eqn in __range1 {
        if eqn_coloring.borrow()[(eqn.clone()-1) as usize].clone() != -1 && !(excluded_eqns.borrow()[(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize].clone()) {
            color = color_clustering.borrow()[(eqn_coloring.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize].clone();
            {
                let __cell2 = cons(eqn.clone(), msss.borrow()[(color.clone()-1) as usize].clone());
                msss.clone().borrow_mut()[(color.clone()-1) as usize] = __cell2;
            }
        }
    }
    msss = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut ms in (Arc::new(msss.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())).into_iter().cloned() {
            if !(!(ms.clone().is_empty())) { continue; }
            let __x = ms.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
    Ok(msss)
}

fn fillColorEqn(mut eqn: i32, mut color: i32, mut eqn_coloring: metamodelica::Array<i32>, mut var_coloring: metamodelica::Array<i32>, mut color_clustering: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>, mut mapping: Arc<Adjacency::Mapping::Mapping>) -> Result<()> {
    {let _arr = eqn_coloring.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = color.clone(); _arr};
    let __range0 = &*m.borrow()[(eqn.clone()-1) as usize].clone();
    for mut var in __range0 {
        let mut var = var.clone();
        fillColorVar(var.clone(), color.clone(), eqn_coloring.clone(), var_coloring.clone(), color_clustering.clone(), m.clone(), mT.clone(), matching.clone(), mapping.clone())?;
    }
    Ok(())
}

fn fillColorVar(mut var: i32, mut color: i32, mut eqn_coloring: metamodelica::Array<i32>, mut var_coloring: metamodelica::Array<i32>, mut color_clustering: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>, mut mapping: Arc<Adjacency::Mapping::Mapping>) -> Result<()> {
    let mut eqn: i32 = matching.var_to_eqn[(var.clone()-1) as usize].clone();
    if var_coloring.borrow()[(var.clone()-1) as usize].clone() == -1 {
        {let _arr = var_coloring.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = color.clone(); _arr};
        if eqn.clone() != -1 {
            if eqn_coloring.borrow()[(eqn.clone()-1) as usize].clone() == -1 {
                fillColorEqn(eqn.clone(), color.clone(), eqn_coloring.clone(), var_coloring.clone(), color_clustering.clone(), m.clone(), mT.clone(), matching.clone(), mapping.clone())?;
            }
        }
    } else {
        colorClustering(var_coloring.borrow()[(var.clone()-1) as usize].clone(), color.clone(), color_clustering.clone())?;
    }
    Ok(())
}

fn colorClustering(mut old_color: i32, mut new_color: i32, mut color_clustering: metamodelica::Array<i32>) -> Result<()> {
    if color_clustering.borrow()[(old_color.clone()-1) as usize].clone() != old_color.clone() {
        colorClustering(color_clustering.borrow()[(old_color.clone()-1) as usize].clone(), new_color.clone(), color_clustering.clone())?;
    }
    {let _arr = color_clustering.clone(); _arr.borrow_mut()[(old_color.clone()-1) as usize] = new_color.clone(); _arr};
    Ok(())
}

fn resolveClustering(mut color_clustering: metamodelica::Array<i32>) -> Result<()> {
    let mut color: i32 = 0;
    let mut color2: i32 = 0;
    let __range0 = 1..=(color_clustering.clone().borrow().len() as i32);
    for mut i in __range0 {
        color = i.clone();
        while color_clustering.borrow()[(color.clone()-1) as usize].clone() != color.clone() {
            color = color_clustering.borrow()[(color.clone()-1) as usize].clone();
        }
        {let _arr = color_clustering.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = color.clone(); _arr};
    }
    Ok(())
}

fn getConstraintsAndCandidates(mut equations: Arc<EquationPointers::EquationPointers>, mut marked_eqns: Arc<metamodelica::List<i32>>, mut mapping: Arc<Adjacency::Mapping::Mapping>) -> Result<(Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>)> {
    let mut constr: Arc<EquationPointers::EquationPointers> = EquationPointers::empty(BaseHashTable::bigBucketSize.clone());
    let mut states: Arc<VariablePointers::VariablePointers> = BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false);
    let mut sliced_constr: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut eqn_indices: Arc<UnorderedSet::UnorderedSet<i32>> = UnorderedSet::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 13);
    let mut eqn_slices: metamodelica::Array<Arc<metamodelica::List<i32>>> = arrayCreate(EquationPointers::size(equations.clone()), metamodelica::nil());
    let mut state_candidates: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    for mut eqn in &*marked_eqns.clone() {
        let mut eqn = eqn.clone();
        UnorderedSet::add(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone(), eqn_indices.clone())?;
        {
            let __cell0 = cons(eqn.clone(), eqn_slices.borrow()[(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize].clone());
            eqn_slices.clone().borrow_mut()[(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize] = __cell0;
        }
    }
    for mut eqn in &*UnorderedSet::toList(eqn_indices.clone()) {
        let mut eqn = eqn.clone();
        eqn_ptr = EquationPointers::getEqnAt(equations.clone(), eqn.clone())?;
        constr = EquationPointers::add(eqn_ptr.clone(), constr.clone())?;
        sliced_constr = cons(Arc::new(Slice::NBSlice { t: eqn_ptr.clone(), indices: eqn_slices.borrow()[(eqn.clone()-1) as usize].clone() }), sliced_constr.clone());
        for mut candidate in &*Equation::collectCrefs(Pointer::access(eqn_ptr.clone()), (std::sync::Arc::new(getStateCandidate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> + 'static>), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))? {
            let mut candidate = candidate.clone();
            UnorderedSet::add(candidate.clone(), state_candidates.clone())?;
        }
    }
    for mut candidate in &*UnorderedSet::toList(state_candidates.clone()) {
        let mut candidate = candidate.clone();
        var_ptr = BVariable::getVarPointer(candidate.clone(), metamodelica::sourceInfo!())?;
        states = BVariable::VariablePointers::add(var_ptr.clone(), states.clone())?;
    }
    Ok((constr, states, sliced_constr))
}

fn getStateCandidate(mut cref: Arc<ComponentRef::NFComponentRef>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    fn getStateCandidateVar(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        if BVariable::isContinuous(var.clone(), false)? && !(BVariable::isTime(var.clone()) || BVariable::isDummyVariable(var.clone()) || BVariable::isDummyState(var.clone()) || BVariable::isForcedState(var.clone())) {
            UnorderedSet::add(BVariable::getVarName(var.clone()), acc.clone())?;
        }
        Ok(())
    }

    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    var = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
    if BVariable::isRecord(var.clone()) {
        for mut child in &*BVariable::getRecordChildren(var.clone()) {
            let mut child = child.clone();
            getStateCandidateVar(child.clone(), acc.clone())?;
        }
    } else {
        getStateCandidateVar(var.clone(), acc.clone())?;
    }
    Ok(cref)
}

fn candidatePriority(mut candidate: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> {
    let mut prio: i32 = 0;
    prio = (::match_deref::match_deref! { match &(Pointer::access(candidate.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { attributes, .. }, .. } => {
            (match VariableAttributes::getStateSelect(attributes.clone()) {
        StateSelect::NEVER => -200,
        StateSelect::AVOID => -100,
        StateSelect::DEFAULT => 0,
        StateSelect::PREFER => 100,
        StateSelect::ALWAYS => 200,
        _ => 0,
    })
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(prio)
}

fn sortCandidates(mut candidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut candidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = candidates;
    let mut priorities: Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Variable::NFVariable>>)>> = metamodelica::nil();
    for mut candidate in &*candidates.clone() {
        let mut candidate = candidate.clone();
        priorities = cons((candidatePriority(candidate.clone())?, candidate.clone()), priorities.clone());
    }
    priorities = List::sort(priorities.clone(), std::sync::Arc::new(fnptr!(BackendUtil::indexTplGt, _, _)))?;
    candidates = List::unzipSecond(priorities.clone());
    Ok(candidates)
}

fn resolveSlicedUnmatched(mut old_unmatched: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut slice_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<i32>>>>) -> Result<Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>> {
    pub fn resolveSlicedUnmatchedSingle(mut eq: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, mut acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut slice_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<i32>>>>) -> Result<Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>> {
        let mut acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = acc;
        let mut relevant_indices: Arc<UnorderedSet::UnorderedSet<i32>>;
        relevant_indices = UnorderedMap::getSafe(Equation::getEqnName(Slice::getT(eq.clone()))?, slice_map.clone(), metamodelica::sourceInfo!())?;
        if UnorderedSet::isEmpty(relevant_indices.clone()) {
            acc = cons(eq.clone(), acc.clone());
        } else {
            assign_field!(eq.indices = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut ind in (eq.indices.clone()).into_iter().cloned() {
            if !(UnorderedSet::contains(ind.clone(), relevant_indices.clone())?) { continue; }
            let __x = ind.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if !(eq.indices.clone().is_empty()) {
                acc = cons(eq.clone(), acc.clone());
            }
        }
        Ok(acc)
    }

    let mut filtered_unmatched: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    for mut eq in &*old_unmatched.clone() {
        let mut eq = eq.clone();
        filtered_unmatched = resolveSlicedUnmatchedSingle(eq.clone(), filtered_unmatched.clone(), slice_map.clone())?;
    }
    Ok(filtered_unmatched)
}

fn removeSlicedDerivatives(mut derivative: Pointer::Pointer<Arc<Equation::Equation>>, mut slice_set: Arc<UnorderedSet::UnorderedSet<i32>>, mut dummy_slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut aux_index: Pointer::Pointer<i32>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
    let mut derivative: Pointer::Pointer<Arc<Equation::Equation>> = derivative;
    let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    if !(UnorderedSet::isEmpty(slice_set.clone())) {
        eqn = removeSlicedDerivateEqn(Pointer::access(derivative.clone()), Arc::new(crate::NBEquation::Iterator::EMPTY), dummy_slice_set.clone(), aux_index.clone())?;
        Pointer::update(derivative.clone(), eqn.clone());
    }
    Ok(derivative)
}

fn removeSlicedDerivateEqn(mut eqn: Arc<Equation::Equation>, mut iter: Arc<Iterator::Iterator>, mut dummy_slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut aux_index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    fn replaceTupleLiterals(mut exp: Arc<Expression::NFExpression>, mut iter: Arc<Iterator::Iterator>, mut dummy_slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut aux_index: Pointer::Pointer<i32>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut aux: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        if Expression::isLiteral(exp.clone()) {
            aux = Call_Aux::createName(Expression::typeOf(exp.clone()), iter.clone(), aux_index.clone(), (arcstr::literal!(BVariable::DERIVATIVE_STR)).clone(), false)?;
            exp = Arc::new(Expression::NFExpression::CREF { ty: ComponentRef::getSubscriptedType(aux.clone(), false)?, cref: aux.clone() });
            UnorderedSet::add(aux.clone(), dummy_slice_set.clone())?;
        }
        Ok(exp)
    }

    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::Equation>>> = metamodelica::nil();
        for mut b in (var_field!((*eqn).body, Equation::Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = removeSlicedDerivateEqn(b.clone(), var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), dummy_slice_set.clone(), aux_index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eqn.clone()
        },
        Deref @ Equation::RECORD_EQUATION { lhs: lhs @ Deref @ Expression::TUPLE { .. }, .. } => {
            let mut lhs = (*lhs).clone();
            assign_variant_field!(lhs => Expression::NFExpression::TUPLE; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*lhs).elements, Expression::NFExpression::TUPLE).clone()).borrow().iter() {
            let __x = replaceTupleLiterals(e.clone(), iter.clone(), dummy_slice_set.clone(), aux_index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(eqn => Equation::Equation::RECORD_EQUATION; lhs = lhs.clone());
            eqn.clone()
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

fn toStringCandidatesConstraints(mut state_candidates: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut constraint_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1((literal!("Index Reduction")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((state_candidates.clone().len() as i32))); __mm_s.push_str(&*literal!(") Sorted State Candidates")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(state_candidates.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((constraint_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(") Constraint Equations")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(constraint_eqns.clone(), { let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }, 10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn toStringDynamicSelect(mut dummy_states: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut unmatched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("\t  DYNAMIC STATE SELECTION\n\t(some unmatched equations)")).clone())); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((dummy_states.clone().len() as i32))); __mm_s.push_str(&*literal!(") Remaining State Candidates")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(dummy_states.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((unmatched_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(") Remaining Equations")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(unmatched_eqns.clone(), { let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }, 10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn toStringUnmatched(mut unmatched_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut unmatched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    let mut s4: ArcStr = arcstr::literal!("");
    if unmatched_vars.clone().is_empty() {
        s1 = (StringUtil::headline_4((literal!("Not underdetermined.")).clone())).clone();
        s3 = (literal!("")).clone();
    } else {
        s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stage ")); __mm_s.push_str(&*intString((unmatched_vars.clone().len() as i32))); __mm_s.push_str(&*literal!(" underdetermined.\n")); ArcStr::from(__mm_s) }).clone();
        s3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((unmatched_vars.clone().len() as i32))); __mm_s.push_str(&*literal!(") Unmatched variables:")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(unmatched_vars.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    if unmatched_eqns.clone().is_empty() {
        s2 = (StringUtil::headline_4((literal!("Not overdetermined.")).clone())).clone();
        s4 = (literal!("")).clone();
    } else {
        s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stage ")); __mm_s.push_str(&*intString((unmatched_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(" overdetermined.\n")); ArcStr::from(__mm_s) }).clone();
        s4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((unmatched_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(") Unmatched equations:")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(unmatched_eqns.clone(), { let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }, 10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str
}

