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

use crate::BackendDAEEXT;
use crate::NBAdjacency as Adjacency;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBResolveSingularities as ResolveSingularities;
use crate::NBSlice as Slice;
use crate::NBSlice::IntLst;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn::Path;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBMatching.mo
/// package:     NBMatching
/// description: This file contains the functions which perform the matching process;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NBMatching {
    /// eqn := var_to_eqn[var]
    pub var_to_eqn: metamodelica::Array<i32>,
    /// var := eqn_to_var[eqn]
    pub eqn_to_var: metamodelica::Array<i32>,
}

impl Default for NBMatching {
    fn default() -> Self {
        Self {
            var_to_eqn: Default::default(),
            eqn_to_var: Default::default(),
        }
    }
}

pub type MATCHING = NBMatching;

thread_local! { static __EMPTY_MATCHING_TLS: Arc<NBMatching> = Arc::new(NBMatching { var_to_eqn: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), eqn_to_var: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()) }); }
pub fn EMPTY_MATCHING() -> Arc<NBMatching> { __EMPTY_MATCHING_TLS.with(|__t| __t.clone()) }

pub fn toString(mut matching: Arc<NBMatching>, mut r#str: ArcStr) -> ArcStr {
    let mut r#str: ArcStr = r#str;
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("Scalar Matching")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toStringSingle(matching.var_to_eqn.clone(), false)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toStringSingle(matching.eqn_to_var.clone(), true)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str
}

pub fn trivial(mut n: i32) -> Arc<NBMatching> {
    let mut matching: Arc<NBMatching> = Arc::new(<NBMatching as ::std::default::Default>::default());
    let mut arr: metamodelica::Array<i32> = Array::createIntRange(n.clone());
    matching = Arc::new(NBMatching { var_to_eqn: arr.clone(), eqn_to_var: arr.clone() });
    matching
}

pub fn regular(mut matching: Arc<NBMatching>, mut adj: Arc<Adjacency::Matrix::Matrix>, mut transposed: bool, mut partially: bool, mut clear: bool) -> Result<Arc<NBMatching>> {
    let mut matching: Arc<NBMatching> = matching;
    let mut marked_eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (matching, marked_eqns, _, _) = continue_(matching.clone(), adj.clone(), transposed.clone(), clear.clone())?;
    if !(partially.clone()) && !(List::flatten(marked_eqns.clone()).is_empty()) {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBMatching.regular")); __mm_s.push_str(&*literal!(" failed because the partition is structurally singular.")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(matching)
}

pub fn singular(mut matching: Arc<NBMatching>, mut adj: Arc<Adjacency::Matrix::Matrix>, mut full: Arc<Adjacency::Matrix::Matrix>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>, mut kind: Partition::Kind, mut transposed: bool, mut clear: bool) -> Result<(Arc<NBMatching>, Arc<Adjacency::Matrix::Matrix>, Arc<Adjacency::Matrix::Matrix>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VarData::VarData>, Arc<EqData::EqData>)> {
    let mut matching: Arc<NBMatching> = matching;
    let mut adj: Arc<Adjacency::Matrix::Matrix> = adj;
    let mut full: Arc<Adjacency::Matrix::Matrix> = full;
    let mut vars: Arc<VariablePointers::VariablePointers> = vars;
    let mut eqns: Arc<EquationPointers::EquationPointers> = eqns;
    let mut varData: Arc<VarData::VarData> = varData;
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut marked_eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut mapping: Option<Arc<Adjacency::Mapping::Mapping>> = None;
    let mut matrixStrictness: Adjacency::MatrixStrictness = Adjacency::MatrixStrictness::LINEAR;
    let mut changed: bool = false;
    if let Ok((__pa0, __pa1, __pa2, __pa3)) = continue_(matching.clone(), adj.clone(), transposed.clone(), clear.clone()) {
        matching = __pa0.clone();
        marked_eqns = __pa1.clone();
        mapping = __pa2.clone();
        matrixStrictness = __pa3.clone();
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBMatching.singular")); __mm_s.push_str(&*literal!(" failed to match partition:\n")); __mm_s.push_str(&*BVariable::VariablePointers::toString(vars.clone(), (literal!("partition vars")).clone(), None, true)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*EquationPointers::toString(eqns.clone(), (literal!("partition eqns")).clone(), None, true, None)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Adjacency::Matrix::toString(adj.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    if Partition::kindIsInitial(kind.clone()) {
        (adj, full, vars, eqns, varData, eqData, changed) = ResolveSingularities::balanceInitialization(adj.clone(), full.clone(), vars.clone(), eqns.clone(), varData.clone(), eqData.clone(), kind.clone(), funcMap.clone(), matching.clone(), mapping.clone())?;
    } else {
        (adj, full, vars, eqns, varData, eqData, changed) = ResolveSingularities::indexReduction(adj.clone(), full.clone(), vars.clone(), eqns.clone(), varData.clone(), eqData.clone(), kind.clone(), funcMap.clone(), matching.clone(), mapping.clone())?;
    }
    if changed.clone() {
        full = Adjacency::Matrix::createFull(vars.clone(), eqns.clone(), kind.clone())?;
        adj = Adjacency::Matrix::fullToFinal(full.clone(), vars.map.clone(), eqns.map.clone(), eqns.clone(), matrixStrictness.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY));
        if Partition::kindIsInitial(kind.clone()) {
            matching = regular(EMPTY_MATCHING().clone(), adj.clone(), false, false, true)?;
        } else {
            (matching, adj, full, vars, eqns, varData, eqData) = singular(EMPTY_MATCHING().clone(), adj.clone(), full.clone(), vars.clone(), eqns.clone(), funcMap.clone(), varData.clone(), eqData.clone(), kind.clone(), transposed.clone(), true)?;
        }
    }
    Ok((matching, adj, full, vars, eqns, varData, eqData))
}

pub fn continue_(mut matching: Arc<NBMatching>, mut adj: Arc<Adjacency::Matrix::Matrix>, mut transposed: bool, mut clear: bool) -> Result<(Arc<NBMatching>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Option<Arc<Adjacency::Mapping::Mapping>>, Adjacency::MatrixStrictness)> {
    let mut matching: Arc<NBMatching> = matching;
    let mut marked_eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut mapping: Option<Arc<Adjacency::Mapping::Mapping>> = None;
    let mut matrixStrictness: Adjacency::MatrixStrictness = Adjacency::MatrixStrictness::LINEAR;
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    (matching, marked_eqns, mapping, matrixStrictness) = (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ Adjacency::Matrix::FINAL { .. } => {
            (var_to_eqn, eqn_to_var) = getAssignments(matching.clone(), var_field!((*adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), var_field!((*adj).mT, Adjacency::Matrix::Matrix::FINAL).clone())?;
            (var_to_eqn, eqn_to_var, marked_eqns) = PFPlusExternal(var_field!((*adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), var_to_eqn.clone(), eqn_to_var.clone(), clear.clone())?;
            matching = Arc::new(NBMatching { var_to_eqn: var_to_eqn.clone(), eqn_to_var: eqn_to_var.clone() });
            (matching.clone(), marked_eqns.clone(), Some(var_field!((*adj).mapping, Adjacency::Matrix::Matrix::FINAL).clone()), var_field!((*adj).st, Adjacency::Matrix::Matrix::FINAL).clone())
        },
        Deref @ Adjacency::Matrix::EMPTY { .. } => (EMPTY_MATCHING().clone(), metamodelica::nil(), None, Adjacency::MatrixStrictness::FULL.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBMatching.continue_")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((matching, marked_eqns, mapping, matrixStrictness))
}

pub fn isEmpty(mut matching: Arc<NBMatching>) -> bool {
    let mut b: bool = matching.eqn_to_var.clone().borrow().is_empty() && matching.var_to_eqn.clone().borrow().is_empty();
    b
}

pub fn isPerfect(mut matching: Arc<NBMatching>) -> bool {
    let mut b: bool = false;
    if (matching.var_to_eqn.clone().borrow().len() as i32) > (matching.eqn_to_var.clone().borrow().len() as i32) {
        b = Array::all(matching.eqn_to_var.clone(), Arc::new(todo!("PARTEVALFUNCTION of intGt: named args do not match any formal: ["i2"]")));
    } else {
        b = Array::all(matching.var_to_eqn.clone(), Arc::new(todo!("PARTEVALFUNCTION of intGt: named args do not match any formal: ["i2"]")));
    }
    b
}

pub fn getAssignments(mut matching: Arc<NBMatching>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    let mut nVars: i32 = (mT.clone().borrow().len() as i32);
    let mut nEqns: i32 = (m.clone().borrow().len() as i32);
    var_to_eqn = Array::expandToSize(nVars.clone(), matching.var_to_eqn.clone(), -1)?;
    eqn_to_var = Array::expandToSize(nEqns.clone(), matching.eqn_to_var.clone(), -1)?;
    Ok((var_to_eqn, eqn_to_var))
}

pub fn getMatches(mut matching: Arc<NBMatching>, mut mapping_opt: Option<Arc<Adjacency::Mapping::Mapping>>, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>) -> Result<(Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>, Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>)> {
    let mut matched_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut unmatched_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut matched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut unmatched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut mapping: Arc<Adjacency::Mapping::Mapping> = Arc::new(<Adjacency::Mapping::Mapping as ::std::default::Default>::default());
    let mut var_map_matched: Arc<UnorderedMap::UnorderedMap<Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<metamodelica::List<i32>>>>;
    let mut var_map_unmatched: Arc<UnorderedMap::UnorderedMap<Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<metamodelica::List<i32>>>>;
    let mut eqn_map_matched: Arc<UnorderedMap::UnorderedMap<Pointer::Pointer<Arc<Equation::Equation>>, Arc<metamodelica::List<i32>>>>;
    let mut eqn_map_unmatched: Arc<UnorderedMap::UnorderedMap<Pointer::Pointer<Arc<Equation::Equation>>, Arc<metamodelica::List<i32>>>>;
    let mut arr_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut arr_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut start_idx: i32 = 0;
    if isSome(mapping_opt.clone()) {
        mapping = Util::getOption(mapping_opt.clone())?;
        var_map_matched = UnorderedMap::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 1);
        var_map_unmatched = UnorderedMap::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 1);
        eqn_map_matched = UnorderedMap::new((std::sync::Arc::new(fnptr!(Equation::hash, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(Equation::equalName, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>), 1);
        eqn_map_unmatched = UnorderedMap::new((std::sync::Arc::new(fnptr!(Equation::hash, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(Equation::equalName, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>), 1);
        let __range0 = 1..=(matching.var_to_eqn.clone().borrow().len() as i32);
        for mut var in __range0 {
            arr_var = ExpandableArray::get(mapping.var_StA.borrow()[(var.clone()-1) as usize].clone(), variables.varArr.clone())?;
            (start_idx, _) = mapping.var_AtS.borrow()[(mapping.var_StA.borrow()[(var.clone()-1) as usize].clone()-1) as usize].clone();
            if matching.var_to_eqn[(var.clone()-1) as usize].clone() > 0 {
                Slice::addToSliceMap(arr_var.clone(), var.clone() - start_idx.clone(), var_map_matched.clone())?;
            } else {
                Slice::addToSliceMap(arr_var.clone(), var.clone() - start_idx.clone(), var_map_unmatched.clone())?;
            }
        }
        let __range1 = 1..=(matching.eqn_to_var.clone().borrow().len() as i32);
        for mut eqn in __range1 {
            arr_eqn = ExpandableArray::get(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone(), equations.eqArr.clone())?;
            (start_idx, _) = mapping.eqn_AtS.borrow()[(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize].clone();
            if matching.eqn_to_var[(eqn.clone()-1) as usize].clone() > 0 {
                Slice::addToSliceMap(arr_eqn.clone(), eqn.clone() - start_idx.clone(), eqn_map_matched.clone())?;
            } else {
                Slice::addToSliceMap(arr_eqn.clone(), eqn.clone() - start_idx.clone(), eqn_map_unmatched.clone())?;
            }
        }
        matched_vars = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut slice in (Slice::fromMap(var_map_matched.clone())).into_iter().cloned() {
            let __x = Slice::simplify(slice.clone(), Arc::new({ let __pe_b1 = true; move |__pe_a0| Ok(BVariable::size(__pe_a0, __pe_b1.clone())) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        unmatched_vars = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut slice in (Slice::fromMap(var_map_unmatched.clone())).into_iter().cloned() {
            let __x = Slice::simplify(slice.clone(), Arc::new({ let __pe_b1 = true; move |__pe_a0| Ok(BVariable::size(__pe_a0, __pe_b1.clone())) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        matched_eqns = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut slice in (Slice::fromMap(eqn_map_matched.clone())).into_iter().cloned() {
            let __x = Slice::simplify(slice.clone(), Arc::new({ let __pe_b1 = true; move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone()) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        unmatched_eqns = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut slice in (Slice::fromMap(eqn_map_unmatched.clone())).into_iter().cloned() {
            let __x = Slice::simplify(slice.clone(), Arc::new({ let __pe_b1 = true; move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone()) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    } else {
        let __range2 = 1..=(matching.var_to_eqn.clone().borrow().len() as i32);
        for mut var in __range2 {
            if matching.var_to_eqn[(var.clone()-1) as usize].clone() > 0 {
                matched_vars = cons(Arc::new(Slice::NBSlice { t: ExpandableArray::get(var.clone(), variables.varArr.clone())?, indices: metamodelica::nil() }), matched_vars.clone());
            } else {
                unmatched_vars = cons(Arc::new(Slice::NBSlice { t: ExpandableArray::get(var.clone(), variables.varArr.clone())?, indices: metamodelica::nil() }), unmatched_vars.clone());
            }
        }
        let __range3 = 1..=(matching.eqn_to_var.clone().borrow().len() as i32);
        for mut eqn in __range3 {
            if matching.eqn_to_var[(eqn.clone()-1) as usize].clone() > 0 {
                matched_eqns = cons(Arc::new(Slice::NBSlice { t: ExpandableArray::get(eqn.clone(), equations.eqArr.clone())?, indices: metamodelica::nil() }), matched_eqns.clone());
            } else {
                unmatched_eqns = cons(Arc::new(Slice::NBSlice { t: ExpandableArray::get(eqn.clone(), equations.eqArr.clone())?, indices: metamodelica::nil() }), unmatched_eqns.clone());
            }
        }
    }
    Ok((matched_vars, unmatched_vars, matched_eqns, unmatched_eqns))
}

fn toStringSingle(mut mapping: metamodelica::Array<i32>, mut inverse: bool) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut head: ArcStr = if (inverse.clone()) {literal!("equation to variable")} else {literal!("variable to equation")};
    let mut from: ArcStr = if (inverse.clone()) {literal!("eqn")} else {literal!("var")};
    let mut to: ArcStr = if (inverse.clone()) {literal!("var")} else {literal!("eqn")};
    r#str = (StringUtil::headline_4((head.clone()).clone())).clone();
    let __range0 = 1..=(mapping.clone().borrow().len() as i32);
    for mut i in __range0 {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*from.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(" --> ")); __mm_s.push_str(&*to.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(mapping.borrow()[(i.clone()-1) as usize].clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    r#str
}

fn scalarMatching(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut transposed: bool, mut partially: bool) -> Result<(Arc<NBMatching>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut matching: Arc<NBMatching> = Arc::new(<NBMatching as ::std::default::Default>::default());
    let mut marked_eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut nVars: i32 = (mT.clone().borrow().len() as i32);
    let mut nEqns: i32 = (m.clone().borrow().len() as i32);
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    let mut var_marks: metamodelica::Array<bool>;
    let mut eqn_marks: metamodelica::Array<bool>;
    let mut pathFound: bool = false;
    var_to_eqn = arrayCreate(nVars.clone(), -1);
    for mut eqn in 1..=nEqns.clone() {
        var_marks = arrayCreate(nVars.clone(), false);
        eqn_marks = arrayCreate(nEqns.clone(), false);
        (var_to_eqn, var_marks, eqn_marks, pathFound) = augmentPath(eqn.clone(), m.clone(), mT.clone(), var_to_eqn.clone(), var_marks.clone(), eqn_marks.clone());
        if !(pathFound.clone()) {
            if !(partially.clone()) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBMatching.scalarMatching")); __mm_s.push_str(&*literal!(" failed because the partition is structurally singular. Index Reduction is not yet supported")); ArcStr::from(__mm_s) }).clone()])?;
            } else if transposed.clone() {
                marked_eqns = cons(BackendUtil::findTrueIndices(var_marks.clone()), marked_eqns.clone());
            } else {
                marked_eqns = cons(BackendUtil::findTrueIndices(eqn_marks.clone()), marked_eqns.clone());
            }
        }
    }
    eqn_to_var = arrayCreate(nEqns.clone(), -1);
    for mut var in 1..=nVars.clone() {
        if var_to_eqn.borrow()[(var.clone()-1) as usize].clone() > 0 {
            {
                let __cell0 = var.clone();
                eqn_to_var.clone().borrow_mut()[(var_to_eqn.borrow()[(var.clone()-1) as usize].clone()-1) as usize] = __cell0;
            }
        }
    }
    if nEqns.clone() > 0 {
        GCExt::free(var_marks.clone());
        GCExt::free(eqn_marks.clone());
    }
    matching = if (transposed.clone()) {Arc::new(NBMatching { var_to_eqn: eqn_to_var.clone(), eqn_to_var: var_to_eqn.clone() })} else {Arc::new(NBMatching { var_to_eqn: var_to_eqn.clone(), eqn_to_var: eqn_to_var.clone() })};
    Ok((matching, marked_eqns))
}

fn augmentPath(mut eqn: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut var_to_eqn: metamodelica::Array<i32>, mut var_marks: metamodelica::Array<bool>, mut eqn_marks: metamodelica::Array<bool>) -> (metamodelica::Array<i32>, metamodelica::Array<bool>, metamodelica::Array<bool>, bool) {
    let mut var_to_eqn: metamodelica::Array<i32> = var_to_eqn;
    let mut var_marks: metamodelica::Array<bool> = var_marks;
    let mut eqn_marks: metamodelica::Array<bool> = eqn_marks;
    let mut pathFound: bool = false;
    {
        let __cell0 = true;
        eqn_marks.clone().borrow_mut()[(eqn.clone()-1) as usize] = __cell0;
    }
    let __range1 = &*m.borrow()[(eqn.clone()-1) as usize].clone();
    for mut var in __range1 {
        let mut var = var.clone();
        if var_to_eqn.borrow()[(var.clone()-1) as usize].clone() <= 0 {
            pathFound = true;
            {
                let __cell2 = eqn.clone();
                var_to_eqn.clone().borrow_mut()[(var.clone()-1) as usize] = __cell2;
            }
            return (var_to_eqn.clone(), var_marks.clone(), eqn_marks.clone(), pathFound.clone());
        }
    }
    let __range3 = &*m.borrow()[(eqn.clone()-1) as usize].clone();
    for mut var in __range3 {
        let mut var = var.clone();
        if !(var_marks.borrow()[(var.clone()-1) as usize].clone()) {
            {
                let __cell4 = true;
                var_marks.clone().borrow_mut()[(var.clone()-1) as usize] = __cell4;
            }
            (var_to_eqn, var_marks, eqn_marks, pathFound) = augmentPath(var_to_eqn.borrow()[(var.clone()-1) as usize].clone(), m.clone(), mT.clone(), var_to_eqn.clone(), var_marks.clone(), eqn_marks.clone());
            if pathFound.clone() {
                {
                    let __cell5 = eqn.clone();
                    var_to_eqn.clone().borrow_mut()[(var.clone()-1) as usize] = __cell5;
                }
                return (var_to_eqn.clone(), var_marks.clone(), eqn_marks.clone(), pathFound.clone());
            }
        }
    }
    (var_to_eqn, var_marks, eqn_marks, pathFound)
}

fn PFPlusExternal(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut clear: bool) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut ass1: metamodelica::Array<i32> = ass1;
    let mut ass2: metamodelica::Array<i32> = ass2;
    let mut marked_eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut n1: i32 = (ass1.clone().borrow().len() as i32);
    let mut n2: i32 = (ass2.clone().borrow().len() as i32);
    let mut nonZero: i32 = BackendUtil::countElem(m.clone());
    let mut cheap: i32 = 0;
    let mut algIndx: i32 = 5;
    BackendDAEEXT::setAssignment(n2.clone(), n1.clone(), ass2.clone(), ass1.clone());
    BackendDAEEXT::setAdjacencyMatrix(n1.clone(), n2.clone(), nonZero.clone(), m.clone());
    BackendDAEEXT::matching(n1.clone(), n2.clone(), algIndx.clone(), cheap.clone(), metamodelica::OrderedFloat(1.0_f64), if (clear.clone()) {1} else {0});
    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
    Ok((ass1, ass2, marked_eqns))
}


