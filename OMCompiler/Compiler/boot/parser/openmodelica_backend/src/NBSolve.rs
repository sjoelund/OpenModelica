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

use crate::NBAdjacency;
use crate::NBBackendUtil as BackendUtil;
use crate::NBCausalize as Causalize;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::SlicingStatus;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBEquation;
use crate::NBInline as Inline;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// backend imports
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Status {
    UNPROCESSED = 1,
    EXPLICIT = 2,
    IMPLICIT = 3,
    UNSOLVABLE = 4,
}
impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

// TRUE -> relation must be inverted, FALSE -> relation must not be inverted, UNKNOWN -> TODO: make relation depend on derivative of the expr
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum RelationInversion {
    TRUE = 1,
    FALSE = 2,
    UNKNOWN = 3,
}
impl PartialOrd for RelationInversion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for RelationInversion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn statusString(mut status: Status) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match status.clone() {
        Status::UNPROCESSED => literal!("Solve.UNPROCESSED"),
        Status::EXPLICIT => literal!("Solve.EXPLICIT"),
        Status::IMPLICIT => literal!("Solve.IMPLICIT"),
        Status::UNSOLVABLE => literal!("Solve.UNSOLVABLE"),
        _ => literal!("Solve.FAILED"),
    })).clone();
    r#str
}

pub fn solvePartition(mut partition: Arc<Partition::Partition>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut implicit_index_ptr: Pointer::Pointer<i32>, mut duplicate_map: Arc<UnorderedMap::UnorderedMap<Arc<StrongComponent::NBStrongComponent>, Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>>>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<Arc<Partition::Partition>> {
    let mut partition: Arc<Partition::Partition> = partition;
    let mut kind: BPartition::Kind = BPartition::Partition::getKind(partition.clone());
    let mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut solved_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut implicit_index: i32 = Pointer::access(implicit_index_ptr.clone());
    let mut new_comps: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>;
    let mut sliced_idx: Pointer::Pointer<i32>;
    let mut comp_idx: Pointer::Pointer<i32> = Pointer::create(1);
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut sliced_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    if Util::isSome(partition.strongComponents.clone()) {
        let __range0 = Util::getOption(partition.strongComponents.clone())?.borrow().iter().cloned().collect::<Vec<_>>();
        for mut comp in __range0 {
            solved_comps = (::match_deref::match_deref! { match &(UnorderedMap::get(comp.clone(), duplicate_map.clone())) {
        Some(alias_comps) => {
            listAppend(alias_comps.clone(), solved_comps.clone())
        },
        _ => {
            let mut alias_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
            (alias_comps, implicit_index) = solveStrongComponent(comp.clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone())?;
            UnorderedMap::add(comp.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        for mut c in (alias_comps.clone()).into_iter().cloned() {
            let __x = StrongComponent::createAlias(kind.clone(), partition.index.clone(), comp_idx.clone(), c.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, duplicate_map.clone())?;
            listAppend(alias_comps.clone(), solved_comps.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        assign_field!(partition.strongComponents = Some(metamodelica::arrayFromVec(solved_comps.clone().reverse().into_iter().cloned().collect())));
        for mut tpl in &*UnorderedMap::toList(slicing_map.clone()) {
            let mut tpl = tpl.clone();
            (name, sliced_eqns) = tpl.clone();
            if !(sliced_eqns.clone().is_empty()) {
                sliced_idx = Pointer::create(1);
                for mut eqn_ptr in &*sliced_eqns.clone() {
                    let mut eqn_ptr = eqn_ptr.clone();
                    Equation::subIdxName(eqn_ptr.clone(), sliced_idx.clone())?;
                }
            }
        }
        Pointer::update(implicit_index_ptr.clone(), implicit_index.clone());
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solvePartition")); __mm_s.push_str(&*literal!(" cannot solve partition without strong components: ")); __mm_s.push_str(&*BPartition::Partition::toString(partition.clone(), 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(partition)
}

pub fn solveStrongComponent(mut comp: Arc<StrongComponent::NBStrongComponent>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: BPartition::Kind, mut implicit_index: i32, mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>, i32)> {
    let mut solved_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut implicit_index: i32 = implicit_index;
    let mut solve_status: Status = Status::UNPROCESSED;
    let mut implicit_comp: Arc<StrongComponent::NBStrongComponent>;
    match '__try0: {
        (solved_comps, solve_status) = ({
        let mut entwined_slices: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        let mut inner_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        let mut failed_inner: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => {
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            (eqn, solve_status, implicit_index) = unwrap_break_err!(solveSingleStrongComponent(Pointer::access(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), Pointer::access(var_field!((*comp).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
            (list![Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { var: var_field!((*comp).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone(), eqn: Pointer::create(eqn.clone()), status: solve_status.clone() })], solve_status.clone())
        },
        Deref @ StrongComponent::MULTI_COMPONENT { vars: Deref @ metamodelica::List::Cons { head: var_slice, tail: Deref @ metamodelica::List::Nil }, .. } if (!(Equation::isCompound(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone())))) => {
            (solved_comps, implicit_index) = unwrap_break_err!(solveStrongComponent(StrongComponent::createSliceOrSingle(BVariable::getVarName(Slice::getT(var_slice.clone())), var_slice.clone(), var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone()), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
            (solved_comps.clone(), Status::UNPROCESSED.clone())
        },
        Deref @ StrongComponent::MULTI_COMPONENT { .. } => {
            let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            let mut solved_comp: Arc<StrongComponent::NBStrongComponent>;
            let mut strict: Arc<Tearing::NBTearing> = Arc::new(<Tearing::NBTearing as ::std::default::Default>::default());
            let mut alg: Arc<Algorithm::NFAlgorithm> = Arc::new(<Algorithm::NFAlgorithm as ::std::default::Default>::default());
            let mut solved_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut inputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut outputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut output_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut input_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut solved_inputs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut tmp_crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>> = metamodelica::nil();
            let mut tmp_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut tmp_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
            let mut idx: Pointer::Pointer<i32>;
            let mut cref_repl: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;
            let mut exp_repl: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
            eqn_ptr = Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone());
            eqn = Pointer::access(eqn_ptr.clone());
            (solved_comp, solve_status) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::ALGORITHM { alg, .. } => {
            let mut alg = (*alg).clone();
            solved_crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone()).into_iter().cloned() {
            let __x = BVariable::getVarName(Slice::getT(var.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            inputs = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut i in (alg.inputs.clone()).into_iter().cloned() {
            let __x = BVariable::getRecordChildrenCrefOrSelf(i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            input_crefs = unwrap_break_err!(UnorderedSet::fromList(inputs.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>)), '__try0);
            solved_inputs = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            for mut solved_cref in &*solved_crefs.clone() {
                let mut solved_cref = solved_cref.clone();
                if unwrap_break_err!(UnorderedSet::contains(solved_cref.clone(), input_crefs.clone()), '__try0) {
                    unwrap_break_err!(UnorderedSet::add(solved_cref.clone(), solved_inputs.clone()), '__try0);
                }
            }
            outputs = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut o in (alg.outputs.clone()).into_iter().cloned() {
            let __x = BVariable::getRecordChildrenCrefOrSelf(o.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            output_crefs = unwrap_break_err!(UnorderedSet::fromList(outputs.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>)), '__try0);
            for mut solved_cref in &*solved_crefs.clone() {
                let mut solved_cref = solved_cref.clone();
                unwrap_break_err!(UnorderedSet::remove(solved_cref.clone(), output_crefs.clone()), '__try0);
            }
            if UnorderedSet::isEmpty(output_crefs.clone()) {
                (eqn_slice, solve_status, implicit_index) = unwrap_break_err!(solveMultiStrongComponent(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), varData.clone(), eqData.clone()), '__try0);
                solved_comp = Arc::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT { vars: var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), eqn: eqn_slice.clone(), status: solve_status.clone() });
            } else {
                solve_status = Status::IMPLICIT.clone();
                idx = Pointer::create(0);
                tmp_crefs = {
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>> = metamodelica::nil();
        for mut cref in (UnorderedSet::toList(output_crefs.clone())).into_iter().cloned() {
            let __x = (cref.clone(), unwrap_break_err!(BVariable::makeTmpVar(cref.clone()), '__try0));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                tmp_vars = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut tpl in (tmp_crefs.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(BVariable::getVarPointer(Util::tuple22(tpl.clone()), metamodelica::sourceInfo!()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                tmp_eqns = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut tpl in (tmp_crefs.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(Equation::makeAssignment(Expression::fromCref(Util::tuple21(tpl.clone()), false)?, Expression::fromCref(Util::tuple22(tpl.clone()), false)?, idx.clone(), (literal!("TMP")).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), NBEquation::default(NBEquation::EquationKind::CONTINUOUS.clone(), false, None, None)), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                tmp_eqns = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut e in (tmp_eqns.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(Equation::createResidual(e.clone(), None, false, false), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                cref_repl = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                exp_repl = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                for mut tpl in &*tmp_crefs.clone() {
                    let mut tpl = tpl.clone();
                    unwrap_break_err!(UnorderedMap::add(Util::tuple21(tpl.clone()), Util::tuple22(tpl.clone()), cref_repl.clone()), '__try0);
                    unwrap_break_err!(UnorderedMap::add(Util::tuple21(tpl.clone()), Expression::fromCref(Util::tuple22(tpl.clone()), false)?, exp_repl.clone()), '__try0);
                }
                assign_field!(alg.outputs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut c in (var_field!((*eqn).alg, Equation::Equation::ALGORITHM).outputs.clone()).into_iter().cloned() {
            let __x = UnorderedMap::getOrDefault(c.clone(), cref_repl.clone(), c.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::MULTI_COMPONENT;
                    vars = {
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice>>> = metamodelica::nil();
        for mut c in (alg.outputs.clone()).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: unwrap_break_err!(BVariable::getVarPointer(c.clone(), metamodelica::sourceInfo!()), '__try0), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                    status = Status::EXPLICIT.clone()
                );
                assign_variant_field!(eqn => Equation::Equation::ALGORITHM; alg = alg.clone());
                Pointer::update(eqn_ptr.clone(), Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = exp_repl.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?);
                strict = Arc::new(Tearing::NBTearing { iteration_vars: {
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice>>> = metamodelica::nil();
        for mut c in (UnorderedSet::toList(solved_inputs.clone())).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: unwrap_break_err!(BVariable::getVarPointer(c.clone(), metamodelica::sourceInfo!()), '__try0), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, residual_eqns: {
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice>>> = metamodelica::nil();
        for mut e in (tmp_eqns.clone()).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: e.clone(), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, innerEquations: metamodelica::arrayFromVec(list![comp.clone()].into_iter().cloned().collect()), jac: None });
                solved_comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { idx: implicit_index.clone(), strict: strict.clone(), casual: None, linear: false, mixed: false, homotopy: false, status: solve_status.clone() });
                unwrap_break_err!(EqData::addTypedList(eqData.clone(), tmp_eqns.clone(), EqData::EqType::CONTINUOUS.clone(), true), '__try0);
                unwrap_break_err!(BVariable::VarData::addTypedList(varData.clone(), tmp_vars.clone(), BVariable::VarData::VarType::ALGEBRAIC.clone()), '__try0);
            }
            (solved_comp.clone(), solve_status.clone())
        },
        _ => {
            (eqn_slice, solve_status, implicit_index) = unwrap_break_err!(solveMultiStrongComponent(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), varData.clone(), eqData.clone()), '__try0);
            (Arc::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT { vars: var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), eqn: eqn_slice.clone(), status: solve_status.clone() }), solve_status.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (list![solved_comp.clone()], solve_status.clone())
        },
        Deref @ StrongComponent::ALGEBRAIC_LOOP { strict, .. } => {
            let mut tmp: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
            let mut err_str: ArcStr = arcstr::literal!("");
            let mut strict = (*strict).clone();
            let __range0 = (1..=(strict.innerEquations.clone().borrow().len() as i32)).rev();
            for mut index in __range0 {
                (tmp, implicit_index) = unwrap_break_err!(solveStrongComponent(strict.innerEquations.borrow()[(index.clone()-1) as usize].clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
                inner_comps = listAppend(tmp.clone(), inner_comps.clone());
                for mut elem in &*tmp.clone() {
                    let mut elem = elem.clone();
                    if unwrap_break_err!(StrongComponent::getSolveStatus(elem.clone()), '__try0) != Status::EXPLICIT.clone() {
                        failed_inner = cons(elem.clone(), failed_inner.clone());
                    }
                }
            }
            if !(failed_inner.clone().is_empty()) {
                if unwrap_break_err!(Flags::isSet(Flags::TEARING_DUMP.clone()), '__try0) {
                    err_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" Following inner equations could not be solved explicitely:\n")); __mm_s.push_str(&*unwrap_break_err!(List::toString(failed_inner.clone(), Arc::new({ let __pe_b1 = -1; move |__pe_a0| StrongComponent::toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0), '__try0)); ArcStr::from(__mm_s) }).clone();
                } else {
                    err_str = (literal!(" Use -d=tearingdump for more information.")).clone();
                }
                unwrap_break_err!(Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveStrongComponent")); __mm_s.push_str(&*literal!(" failed. ")); __mm_s.push_str(&*err_str.clone()); ArcStr::from(__mm_s) }).clone()]), '__try0);
                break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
            }
            assign_field!(strict.innerEquations = metamodelica::arrayFromVec(inner_comps.clone().into_iter().cloned().collect()));
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP;
                strict = strict.clone(),
                status = Status::IMPLICIT.clone()
            );
            (list![comp.clone()], Status::IMPLICIT.clone())
        },
        Deref @ StrongComponent::SLICED_COMPONENT { eqn: eqn_slice, .. } if (Equation::isForEquation(Slice::getT(eqn_slice.clone()))) => {
            let mut generic_comp: Arc<StrongComponent::NBStrongComponent>;
            (generic_comp, solve_status, implicit_index) = unwrap_break_err!(solveGenericEquation(comp.clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
            (list![generic_comp.clone()], solve_status.clone())
        },
        Deref @ StrongComponent::SLICED_COMPONENT { eqn: eqn_slice, var: var_slice, .. } if (Equation::isArrayEquation(Slice::getT(eqn_slice.clone()))) => {
            let mut eqn_slice = (*eqn_slice).clone();
            (eqn_slice, implicit_index, solve_status) = unwrap_break_err!(solveForVarSlice(eqn_slice.clone(), var_slice.clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT;
                eqn = eqn_slice.clone(),
                status = solve_status.clone()
            );
            (list![comp.clone()], solve_status.clone())
        },
        Deref @ StrongComponent::SLICED_COMPONENT { .. } => {
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            (eqn, solve_status, implicit_index) = unwrap_break_err!(solveSingleStrongComponent(Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone())), Variable::fromCref(var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone())?, funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
            if solve_status.clone() < Status::UNSOLVABLE.clone() {
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT; eqn = Arc::new(Slice::NBSlice { t: Pointer::create(eqn.clone()), indices: metamodelica::nil() }));
            } else {
                (eqn_slice, implicit_index, solve_status) = unwrap_break_err!(solveForVarSlice(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), var_field!((*comp).var, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT; eqn = eqn_slice.clone());
            }
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT; status = solve_status.clone());
            (list![comp.clone()], solve_status.clone())
        },
        Deref @ StrongComponent::RESIZABLE_COMPONENT { .. } => {
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            (eqn, solve_status, implicit_index, _) = unwrap_break_err!(solveEquation(Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone())), var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
            eqn = unwrap_break_err!(Equation::applyForOrder(eqn.clone(), var_field!((*comp).order, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone()), '__try0);
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT;
                eqn = Arc::new(Slice::NBSlice { t: Pointer::create(eqn.clone()), indices: var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).indices.clone() }),
                status = solve_status.clone()
            );
            (list![comp.clone()], solve_status.clone())
        },
        Deref @ StrongComponent::ENTWINED_COMPONENT { .. } => {
            let mut generic_comp: Arc<StrongComponent::NBStrongComponent>;
            for mut slice in &*var_field!((*comp).entwined_slices, StrongComponent::NBStrongComponent::ENTWINED_COMPONENT).clone() {
                let mut slice = slice.clone();
                (generic_comp, solve_status, implicit_index) = unwrap_break_err!(solveGenericEquation(slice.clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone()), '__try0);
                entwined_slices = cons(generic_comp.clone(), entwined_slices.clone());
            }
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ENTWINED_COMPONENT; entwined_slices = entwined_slices.clone().reverse());
            (list![comp.clone()], Status::EXPLICIT.clone())
        },
        _ => {
            (list![comp.clone()], Status::UNSOLVABLE.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        Ok::<_, anyhow::Error>((solve_status.clone(), solved_comps.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            solve_status = __try0_o0;
            solved_comps = __try0_o1;
        }
        Err(_) => {
            (solved_comps, solve_status) = (list![comp.clone()], Status::UNSOLVABLE.clone());
        }
    }
    if solve_status.clone() == Status::IMPLICIT.clone() && List::hasOneElement(solved_comps.clone()) {
        (implicit_comp, implicit_index) = Tearing::implicit(listHead(solved_comps.clone())?, funcMap.clone(), implicit_index.clone(), kind.clone())?;
        solved_comps = list![implicit_comp.clone()];
    } else if solve_status.clone() > Status::EXPLICIT.clone() {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveStrongComponent")); __mm_s.push_str(&*literal!(" failed with status = ")); __mm_s.push_str(&*statusString(solve_status.clone())); __mm_s.push_str(&*literal!(" while trying to solve following strong component:\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok((solved_comps, implicit_index))
}

pub fn solveGenericEquation(mut comp: Arc<StrongComponent::NBStrongComponent>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: BPartition::Kind, mut implicit_index: i32, mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<StrongComponent::NBStrongComponent>, Status, i32)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut solve_status: Status = Status::UNPROCESSED;
    let mut implicit_index: i32 = implicit_index;
    (comp, solve_status) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SLICED_COMPONENT { eqn: eqn_slice, var: var_slice, .. } if (Equation::isForEquation(Slice::getT(eqn_slice.clone()))) => {
            (comp, solve_status, implicit_index) = solveGenericEquationSlice(var_slice.clone(), eqn_slice.clone(), var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone())?;
            (comp.clone(), solve_status.clone())
        },
        Deref @ StrongComponent::RESIZABLE_COMPONENT { eqn: eqn_slice, var: var_slice, .. } if (Equation::isForEquation(Slice::getT(eqn_slice.clone()))) => {
            let mut eqn_slice = (*eqn_slice).clone();
            eqn_slice = Slice::apply(eqn_slice.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static> = Arc::new({ let __pe_b1 = var_field!((*comp).order, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone(); move |__pe_a0| Equation::applyForOrder(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Ok(Pointer::apply(__pe_a0, __pe_b1.clone())) }));
            (comp, solve_status, implicit_index) = solveGenericEquationSlice(var_slice.clone(), eqn_slice.clone(), var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone())?;
            (comp.clone(), solve_status.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveGenericEquation")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, solve_status, implicit_index))
}

pub fn solveGenericEquationSlice(mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, mut cref: Arc<ComponentRef::NFComponentRef>, mut functions: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: BPartition::Kind, mut implicit_index: i32, mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<StrongComponent::NBStrongComponent>, Status, i32)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent>;
    let mut solve_status: Status = Status::UNPROCESSED;
    let mut implicit_index: i32 = implicit_index;
    let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>> = Slice::getT(eqn_slice.clone());
    let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut solved_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    if List::hasOneElement(eqn_slice.indices.clone()) {
        replacements = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        (eqn, solve_status) = Equation::singleSlice(eqn_ptr.clone(), listHead(eqn_slice.indices.clone())?, Equation::sizes(eqn_ptr.clone(), false)?, cref.clone(), replacements.clone(), functions.clone())?;
    } else {
        (eqn, solve_status, implicit_index, _) = solveEquation(Pointer::access(eqn_ptr.clone()), cref.clone(), functions.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone())?;
    }
    if solve_status.clone() < Status::UNSOLVABLE.clone() {
        solved_slice = Arc::new(Slice::NBSlice { t: Pointer::create(eqn.clone()), indices: eqn_slice.indices.clone() });
    } else {
        (solved_slice, implicit_index, solve_status) = solveForVarSlice(eqn_slice.clone(), var_slice.clone(), functions.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone())?;
    }
    if Equation::isForEquation(Slice::getT(solved_slice.clone())) {
        comp = Arc::new(StrongComponent::NBStrongComponent::GENERIC_COMPONENT { var_cref: cref.clone(), var: var_slice.clone(), eqn: solved_slice.clone() });
    } else {
        comp = Arc::new(StrongComponent::NBStrongComponent::SLICED_COMPONENT { var_cref: cref.clone(), var: var_slice.clone(), eqn: solved_slice.clone(), status: solve_status.clone() });
    }
    Ok((comp, solve_status, implicit_index))
}

pub fn solveSingleStrongComponent(mut eqn: Arc<Equation::Equation>, mut var: Arc<Variable::NFVariable>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: BPartition::Kind, mut implicit_index: i32, mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<Equation::Equation>, Status, i32)> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut status: Status = Status::UNPROCESSED;
    let mut implicit_index: i32 = implicit_index;
    let mut var_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    if ComponentRef::isEmpty(var.name.clone()) {
        (eqn, status) = (eqn.clone(), Status::EXPLICIT.clone());
    } else {
        (var_cref, status) = getVarSlice(var.name.clone(), eqn.clone())?;
        var_cref = if (status.clone() < Status::UNSOLVABLE.clone()) {var_cref.clone()} else {var.name.clone()};
        (eqn, status, implicit_index, _) = solveEquation(eqn.clone(), var_cref.clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone())?;
    }
    Ok((eqn, status, implicit_index))
}

pub fn solveMultiStrongComponent(mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, mut var_slices: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: BPartition::Kind, mut implicit_index: i32, mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut iter: Arc<Iterator::Iterator>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, Status, i32)> {
    let mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>> = eqn_slice;
    let mut status: Status = Status::UNPROCESSED;
    let mut implicit_index: i32 = implicit_index;
    let mut eqn: Arc<Equation::Equation> = Pointer::access(Slice::getT(eqn_slice.clone()));
    (eqn_slice, status) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::IF_EQUATION { .. } => {
            let mut if_body: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
            (if_body, status, implicit_index) = solveIfBody(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), BVariable::VariablePointers::fromList({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut v in (var_slices.clone()).into_iter().cloned() {
            let __x = Slice::getT(v.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, false), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), iter.clone(), varData.clone(), eqData.clone())?;
            assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; body = if_body.clone());
            (Arc::new(Slice::NBSlice { t: Pointer::create(eqn.clone()), indices: eqn_slice.indices.clone() }), status.clone())
        },
        Deref @ Equation::ALGORITHM { .. } => {
            (Arc::new(Slice::NBSlice { t: Pointer::clone(Slice::getT(eqn_slice.clone())), indices: eqn_slice.indices.clone() }), Status::EXPLICIT.clone())
        },
        Deref @ Equation::WHEN_EQUATION { .. } => {
            (Arc::new(Slice::NBSlice { t: Pointer::clone(Slice::getT(eqn_slice.clone())), indices: eqn_slice.indices.clone() }), Status::EXPLICIT.clone())
        },
        Deref @ Equation::RECORD_EQUATION { .. } => {
            let mut solved_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            (solved_eqn, status) = solveMultiRecordStrongComponent(eqn.clone(), var_slices.clone(), funcMap.clone())?;
            (Arc::new(Slice::NBSlice { t: Pointer::create(solved_eqn.clone()), indices: eqn_slice.indices.clone() }), status.clone())
        },
        Deref @ Equation::ARRAY_EQUATION { .. } => {
            let mut solved_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            (solved_eqn, status) = solveMultiRecordStrongComponent(eqn.clone(), var_slices.clone(), funcMap.clone())?;
            (Arc::new(Slice::NBSlice { t: Pointer::create(solved_eqn.clone()), indices: eqn_slice.indices.clone() }), status.clone())
        },
        Deref @ Equation::DUMMY_EQUATION => {
            (eqn_slice.clone(), Status::EXPLICIT.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveMultiStrongComponent")); __mm_s.push_str(&*literal!(" failed for equation:\n")); __mm_s.push_str(&*Slice::toString(eqn_slice.clone(), Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn_slice, status, implicit_index))
}

pub fn solveMultiRecordStrongComponent(mut eqn: Arc<Equation::Equation>, mut var_slices: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Equation::Equation>, Status)> {
    let mut solved_eqn: Arc<Equation::Equation> = eqn.clone();
    let mut status: Status = Status::UNPROCESSED;
    let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut v in (var_slices.clone()).into_iter().cloned() {
            let __x = Slice::getT(v.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    let mut lhs: Arc<Expression::NFExpression> = Util::getOption(Equation::getLHS(eqn.clone())?)?;
    let mut rhs: Arc<Expression::NFExpression> = Util::getOption(Equation::getRHS(eqn.clone())?)?;
    let mut record_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut var_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    (solved_eqn, status) = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (exp @ Deref @ Expression::TUPLE { .. }, _) if (tupleSolvable(var_field!((**exp).elements, Expression::NFExpression::TUPLE).clone(), vars.clone())?) => {
            (solved_eqn.clone(), Status::EXPLICIT.clone())
        },
        (_, exp @ Deref @ Expression::TUPLE { .. }) if (tupleSolvable(var_field!((**exp).elements, Expression::NFExpression::TUPLE).clone(), vars.clone())?) => {
            solved_eqn = Equation::setRHS(solved_eqn.clone(), lhs.clone())?;
            solved_eqn = Equation::setLHS(solved_eqn.clone(), rhs.clone())?;
            (solved_eqn.clone(), Status::EXPLICIT.clone())
        },
        _ => {
            record_crefs = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            for mut var_slice in &*var_slices.clone() {
                let mut var_slice = var_slice.clone();
                (var_cref, status) = getVarSlice(BVariable::getVarName(Slice::getT(var_slice.clone())), eqn.clone())?;
                UnorderedSet::add(var_cref.clone(), record_crefs.clone())?;
                if status.clone() == Status::UNSOLVABLE.clone() {
                    break;
                }
            }
            solved_eqn = (::match_deref::match_deref! { match &((UnorderedSet::toList(record_crefs.clone()), status.clone())) {
        (Deref @ metamodelica::List::Cons { head: var_cref, tail: Deref @ metamodelica::List::Nil }, Status::UNPROCESSED) => {
            (solved_eqn, status, _) = solveBody(eqn.clone(), var_cref.clone(), funcMap.clone())?;
            solved_eqn.clone()
        },
        _ => eqn.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (solved_eqn.clone(), status.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((solved_eqn, status))
}

pub fn solveEquation(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: BPartition::Kind, mut implicit_index: i32, mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<Equation::Equation>, Status, i32, RelationInversion)> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut status: Status = Status::UNPROCESSED;
    let mut implicit_index: i32 = implicit_index;
    let mut invertRelation: RelationInversion = RelationInversion::TRUE;
    (eqn, status, invertRelation) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body @ Deref @ Equation::IF_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut body_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            let mut indexed_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut dummy: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
            (indexed_var, _) = BVariable::makeVarPtrCyclic(BVariable::getVar(cref.clone(), metamodelica::sourceInfo!())?, cref.clone())?;
            dummy = Iterator::dummy(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            (body_slice, status, implicit_index) = solveMultiStrongComponent(Arc::new(Slice::NBSlice { t: Pointer::create(body.clone()), indices: metamodelica::nil() }), list![Arc::new(Slice::NBSlice { t: indexed_var.clone(), indices: metamodelica::nil() })], funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), dummy.clone(), varData.clone(), eqData.clone())?;
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![Pointer::access(Slice::getT(body_slice.clone()))]);
            (eqn.clone(), status.clone(), RelationInversion::FALSE.clone())
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut body = (*body).clone();
            (body, status, invertRelation) = solveBody(body.clone(), cref.clone(), funcMap.clone())?;
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![body.clone()]);
            (eqn.clone(), status.clone(), invertRelation.clone())
        },
        Deref @ Equation::FOR_EQUATION { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveEquation")); __mm_s.push_str(&*literal!(" failed to solve a for-equation with multiple body eqns for a single cref. Please iterate over body elements individually.\n")); __mm_s.push_str(&*literal!("cref: ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" in equation:\n")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        Deref @ Equation::DUMMY_EQUATION => {
            (eqn.clone(), Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        _ => {
            solveBody(eqn.clone(), cref.clone(), funcMap.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn, status, implicit_index, invertRelation))
}

pub fn solveBody(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Equation::Equation>, Status, RelationInversion)> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut status: Status = Status::UNPROCESSED;
    let mut invertRelation: RelationInversion = RelationInversion::TRUE;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut fixed_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut residual: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut derivative: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut diffArgs: Arc<Differentiate::DifferentiationArguments::DifferentiationArguments> = Arc::new(<Differentiate::DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    let mut divOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    let mut uminOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    fixed_cref = ComponentRef::stripSubscriptsAll(cref.clone());
    ty = ComponentRef::getSubscriptedType(fixed_cref.clone(), true)?;
    if Type::isArray(ty.clone()) && Type::sizeOf(ty.clone(), false)? == 1 {
        (fixed_cref, _) = getVarSlice(fixed_cref.clone(), eqn.clone())?;
    } else {
        fixed_cref = cref.clone();
    }
    if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
        solvePrintInput(eqn.clone(), fixed_cref.clone())?;
    }
    (eqn, status, invertRelation) = solveSimple(eqn.clone(), fixed_cref.clone())?;
    if status.clone() == Status::UNPROCESSED.clone() {
        residual = Equation::getResidualExp(eqn.clone(), true)?;
        (eqn, status) = solveUnique(eqn.clone(), residual.clone(), fixed_cref.clone())?;
        if status.clone() == Status::EXPLICIT.clone() {
            invertRelation = RelationInversion::UNKNOWN.clone();
        } else {
            diffArgs = Differentiate::DifferentiationArguments::simpleCref(fixed_cref.clone(), funcMap.clone());
            (derivative, diffArgs) = Differentiate::differentiateExpressionDump(residual.clone(), diffArgs.clone(), (literal!("NBSolve.solveBody")).clone(), (literal!("")).clone())?;
            derivative = SimplifyExp::simplifyDump(derivative.clone(), true, (literal!("NBSolve.solveBody")).clone(), (literal!("")).clone())?;
            if Expression::isZero(derivative.clone()) {
                invertRelation = RelationInversion::FALSE.clone();
                status = Status::UNSOLVABLE.clone();
            } else if !(Expression::containsCref(derivative.clone(), fixed_cref.clone())?) {
                eqn = solveLinear(eqn.clone(), residual.clone(), derivative.clone(), diffArgs.clone(), fixed_cref.clone())?;
                invertRelation = if (Expression::isPositive(derivative.clone())) {RelationInversion::FALSE.clone()} else {if (Expression::isNegative(derivative.clone())) {RelationInversion::TRUE.clone()} else {RelationInversion::UNKNOWN.clone()}};
                status = Status::EXPLICIT.clone();
            } else {
                invertRelation = RelationInversion::FALSE.clone();
                if Flags::isSet(Flags::FAILTRACE.clone())? && status.clone() != Status::EXPLICIT.clone() {
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveBody")); __mm_s.push_str(&*literal!(" cref: ")); __mm_s.push_str(&*ComponentRef::toString(fixed_cref.clone())?); __mm_s.push_str(&*literal!(" has to be solved implicitely in equation:\n")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone())?;
                }
            }
        }
    }
    eqn = Equation::simplify(eqn.clone(), (literal!("NBSolve.solveBody")).clone(), (literal!("")).clone(), Pointer::create(metamodelica::nil()), Pointer::create(metamodelica::nil()), Arc::new({ let __pe_b1 = true; let __pe_b2 = (literal!("NBSolve.solveBody")).clone(); let __pe_b3 = (literal!("")).clone(); move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
        solvePrintOutput(eqn.clone(), status.clone())?;
    }
    Ok((eqn, status, invertRelation))
}

pub fn solveIfBody(mut body: Arc<IfEquationBody::IfEquationBody>, mut vars: Arc<VariablePointers::VariablePointers>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: BPartition::Kind, mut implicit_index: i32, mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut iter: Arc<Iterator::Iterator>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<IfEquationBody::IfEquationBody>, Status, i32)> {
    let mut body: Arc<IfEquationBody::IfEquationBody> = body;
    let mut status: Status = Status::UNPROCESSED;
    let mut implicit_index: i32 = implicit_index;
    let mut else_if: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut solved_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut new_then_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    (_, comps) = Causalize::simple(vars.clone(), EquationPointers::fromList(body.then_eqns.clone()), kind.clone(), NBAdjacency::MatrixStrictness::MATCHING.clone(), iter.clone())?;
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        (solved_comps, implicit_index) = solveStrongComponent(comp.clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone())?;
        for mut solved_comp in &*solved_comps.clone() {
            let mut solved_comp = solved_comp.clone();
            new_then_eqns = cons(StrongComponent::toSolvedEquation(solved_comp.clone())?, new_then_eqns.clone());
        }
    }
    assign_field!(body.then_eqns = new_then_eqns.clone().reverse());
    if Util::isSome(body.else_if.clone()) {
        (else_if, status, implicit_index) = solveIfBody(Util::getOption(body.else_if.clone())?, vars.clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), iter.clone(), varData.clone(), eqData.clone())?;
        assign_field!(body.else_if = Some(else_if.clone()));
    } else {
        status = Status::EXPLICIT.clone();
    }
    Ok((body, status, implicit_index))
}

pub fn solveSimple(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<Equation::Equation>, Status, RelationInversion)> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut status: Status = Status::UNPROCESSED;
    let mut invertRelation: RelationInversion = RelationInversion::TRUE;
    (eqn, status, invertRelation) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::SCALAR_EQUATION { .. } => {
            solveSimpleLhsRhs(var_field!((*eqn).lhs, Equation::Equation::SCALAR_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::SCALAR_EQUATION).clone(), cref.clone(), eqn.clone())?
        },
        Deref @ Equation::ARRAY_EQUATION { .. } => {
            solveSimpleLhsRhs(var_field!((*eqn).lhs, Equation::Equation::ARRAY_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), cref.clone(), eqn.clone())?
        },
        Deref @ Equation::RECORD_EQUATION { .. } => {
            solveSimpleLhsRhs(var_field!((*eqn).lhs, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::RECORD_EQUATION).clone(), cref.clone(), eqn.clone())?
        },
        Deref @ Equation::WHEN_EQUATION { .. } => {
            solveSimpleWhen(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone(), cref.clone(), eqn.clone())?
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut body = (*body).clone();
            (body, status, invertRelation) = solveSimple(body.clone(), cref.clone())?;
            if status.clone() == Status::EXPLICIT.clone() {
                assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![body.clone()]);
            } else {
                status = Status::UNPROCESSED.clone();
            }
            (eqn.clone(), status.clone(), invertRelation.clone())
        },
        Deref @ Equation::IF_EQUATION { .. } => {
            let mut if_body: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
            (if_body, status, invertRelation) = solveSimpleIf(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), cref.clone())?;
            if status.clone() == Status::EXPLICIT.clone() {
                assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; body = if_body.clone());
            } else {
                status = Status::UNPROCESSED.clone();
            }
            (eqn.clone(), status.clone(), invertRelation.clone())
        },
        _ => {
            (eqn.clone(), Status::UNPROCESSED.clone(), RelationInversion::FALSE.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn, status, invertRelation))
}

fn solveSimpleLhsRhs(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut eqn: Arc<Equation::Equation>) -> Result<(Arc<Equation::Equation>, Status, RelationInversion)> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut status: Status = Status::UNPROCESSED;
    let mut invertRelation: RelationInversion = RelationInversion::TRUE;
    (eqn, status, invertRelation) = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ Expression::CREF { cref: checkCref, .. }, exp) if (ComponentRef::isEqual(cref.clone(), checkCref.clone())? && !(Expression::containsCref(exp.clone(), cref.clone())?)) => {
            (eqn.clone(), Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (exp, Deref @ Expression::CREF { cref: checkCref, .. }) if (ComponentRef::isEqual(cref.clone(), checkCref.clone())? && !(Expression::containsCref(exp.clone(), cref.clone())?)) => {
            (Equation::swapLHSandRHS(eqn.clone())?, Status::EXPLICIT.clone(), RelationInversion::TRUE.clone())
        },
        (Deref @ Expression::UNARY { exp: Deref @ Expression::CREF { cref: checkCref, .. }, .. }, exp) if (ComponentRef::isEqual(cref.clone(), checkCref.clone())? && !(Expression::containsCref(exp.clone(), cref.clone())?)) => {
            (Equation::updateLHSandRHS(eqn.clone(), Expression::negate(lhs.clone()), Expression::negate(rhs.clone()))?, Status::EXPLICIT.clone(), RelationInversion::TRUE.clone())
        },
        (Deref @ Expression::LUNARY { exp: Deref @ Expression::CREF { cref: checkCref, .. }, .. }, exp) if (ComponentRef::isEqual(cref.clone(), checkCref.clone())? && !(Expression::containsCref(exp.clone(), cref.clone())?)) => {
            (Equation::updateLHSandRHS(eqn.clone(), Expression::logicNegate(lhs.clone()), Expression::logicNegate(rhs.clone()))?, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (exp, Deref @ Expression::UNARY { exp: Deref @ Expression::CREF { cref: checkCref, .. }, .. }) if (ComponentRef::isEqual(cref.clone(), checkCref.clone())? && !(Expression::containsCref(exp.clone(), cref.clone())?)) => {
            (Equation::updateLHSandRHS(eqn.clone(), Expression::negate(rhs.clone()), Expression::negate(lhs.clone()))?, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (exp, Deref @ Expression::LUNARY { exp: Deref @ Expression::CREF { cref: checkCref, .. }, .. }) if (ComponentRef::isEqual(cref.clone(), checkCref.clone())? && !(Expression::containsCref(exp.clone(), cref.clone())?)) => {
            (Equation::updateLHSandRHS(eqn.clone(), Expression::logicNegate(rhs.clone()), Expression::logicNegate(lhs.clone()))?, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (exp @ Deref @ Expression::TUPLE { .. }, _) if (tupleSolvable(var_field!((**exp).elements, Expression::NFExpression::TUPLE).clone(), list![BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?])?) => {
            (eqn.clone(), Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (_, exp @ Deref @ Expression::TUPLE { .. }) if (tupleSolvable(var_field!((**exp).elements, Expression::NFExpression::TUPLE).clone(), list![BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?])?) => {
            (Equation::swapLHSandRHS(eqn.clone())?, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        _ => {
            (eqn.clone(), Status::UNPROCESSED.clone(), RelationInversion::FALSE.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn, status, invertRelation))
}

fn solveSimpleWhen(mut body: Arc<WhenEquationBody::WhenEquationBody>, mut cref: Arc<ComponentRef::NFComponentRef>, mut eqn: Arc<Equation::Equation>) -> Result<(Arc<Equation::Equation>, Status, RelationInversion)> {
    let mut eqnOut: Arc<Equation::Equation> = eqn.clone();
    let mut status: Status = Status::UNPROCESSED;
    let mut invertRelation: RelationInversion = RelationInversion::FALSE.clone();
    for mut stmt in &*body.when_stmts.clone() {
        let mut stmt = stmt.clone();
        status = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref: checkCref, .. }, .. } if (ComponentRef::isEqual(cref.clone(), checkCref.clone())? && !(Expression::containsCref(var_field!((*stmt).rhs, WhenStatement::WhenStatement::ASSIGN).clone(), cref.clone())?)) => {
            Status::EXPLICIT.clone()
        },
        _ => {
            Status::UNSOLVABLE.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if status.clone() == Status::EXPLICIT.clone() {
            break;
        }
    }
    Ok((eqnOut, status, invertRelation))
}

fn solveSimpleIf(mut body: Arc<IfEquationBody::IfEquationBody>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<IfEquationBody::IfEquationBody>, Status, RelationInversion)> {
    let mut body: Arc<IfEquationBody::IfEquationBody> = body;
    let mut status: Status = Status::EXPLICIT.clone();
    let mut invertRelation: RelationInversion = RelationInversion::FALSE.clone();
    let mut else_if: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
    let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    if Util::isSome(body.else_if.clone()) {
        (else_if, status, _) = solveSimpleIf(Util::getOption(body.else_if.clone())?, cref.clone())?;
        if status.clone() == Status::EXPLICIT.clone() {
            assign_field!(body.else_if = Some(else_if.clone()));
        }
    }
    if status.clone() == Status::EXPLICIT.clone() && List::hasOneElement(body.then_eqns.clone()) {
        eqn = Pointer::access(listHead(body.then_eqns.clone())?);
        (eqn, status, _) = solveSimple(eqn.clone(), cref.clone())?;
        if status.clone() == Status::EXPLICIT.clone() {
            Pointer::update(listHead(body.then_eqns.clone())?, eqn.clone());
        }
    } else {
        status = Status::UNPROCESSED.clone();
    }
    Ok((body, status, invertRelation))
}

fn solveLinear(mut eqn: Arc<Equation::Equation>, mut residual: Arc<Expression::NFExpression>, mut derivative: Arc<Expression::NFExpression>, mut diffArgs: Arc<Differentiate::DifferentiationArguments::DifferentiationArguments>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut crefExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut numerator: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mulOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    let mut uminOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    crefExp = Expression::fromCref(cref.clone(), false)?;
    ty = ComponentRef::getSubscriptedType(cref.clone(), true)?;
    numerator = Replacements::single(residual.clone(), crefExp.clone(), Expression::makeZero(ty.clone())?)?;
    mulOp = Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::MUL.clone() });
    uminOp = Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::UMINUS.clone() });
    eqn = Equation::setLHS(eqn.clone(), crefExp.clone())?;
    eqn = Equation::setRHS(eqn.clone(), Arc::new(Expression::NFExpression::UNARY { operator: uminOp.clone(), exp: Arc::new(Expression::NFExpression::MULTARY { arguments: list![numerator.clone()], inv_arguments: list![derivative.clone()], operator: mulOp.clone() }) }))?;
    Ok(eqn)
}

fn solveUnique(mut eqn: Arc<Equation::Equation>, mut residual: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<Equation::Equation>, Status)> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefExp: Arc<Expression::NFExpression> = Expression::fromCref(cref.clone(), false)?;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut solvedRHS: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut crefFound: bool = false;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = ComponentRef::getSubscriptedType(cref.clone(), true)?;
    (crefFound, inverseInstructions, status) = solveUniqueFindInstructions(residual.clone(), cref.clone(), false, inverseInstructions.clone())?;
    if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
        solveUniquePrintInstructions(inverseInstructions.clone(), status.clone())?;
    }
    eqn = (match status.clone() {
        Status::IMPLICIT => eqn.clone(),
        _ => {
            status = Status::EXPLICIT.clone();
            solvedRHS = Expression::makeZero(ty.clone())?;
            for mut instruction in &*inverseInstructions.clone() {
                let mut instruction = instruction.clone();
                solvedRHS = applyInstruction(solvedRHS.clone(), instruction.clone())?;
            }
            eqn = Equation::setLHS(eqn.clone(), crefExp.clone())?;
            eqn = Equation::setRHS(eqn.clone(), solvedRHS.clone())?;
            eqn.clone()
        },
    });
    Ok((eqn, status))
}

fn solveUniqueFindInstructions(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefFound: bool, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Status)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    let mut status: Status = Status::EXPLICIT.clone();
    let mut substExp: Arc<Expression::NFExpression> = BVariable::toExpression(Pointer::create(BVariable::SUBST_VARIABLE().clone()));
    let mut ty: Arc<Type::NFType> = ComponentRef::getSubscriptedType(cref.clone(), true)?;
    let mut crefFoundInRecursion: bool = false;
    let mut name: ArcStr = arcstr::literal!("");
    let mut call: Arc<Call::NFCall>;
    if crefFound.clone() {
        if Expression::containsCref(exp.clone(), cref.clone())? {
            status = Status::IMPLICIT.clone();
        } else {
            crefFound = false;
        }
        return Ok((crefFound, inverseInstructions, status));
    }
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { .. } => (),
        Deref @ Expression::INTEGER { .. } => (),
        Deref @ Expression::CREF { .. } => {
            if ComponentRef::isEqual(cref.clone(), var_field!((*exp).cref, Expression::NFExpression::CREF).clone())? {
                crefFound = true;
            }
            ()
        },
        Deref @ Expression::CAST { .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsCast(substExp.clone(), exp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            ()
        },
        Deref @ Expression::MULTARY { .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsMultary(substExp.clone(), exp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            ()
        },
        Deref @ Expression::BINARY { .. } => {
            let () = (::match_deref::match_deref! { match &(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone()) {
        Deref @ Operator::OPERATOR { op: Operator::Op::POW, .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsBinaryPow(ty.clone(), substExp.clone(), exp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            ()
        },
        Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsBinaryComOp(substExp.clone(), exp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            ()
        },
        Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsBinaryComOp(substExp.clone(), exp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            ()
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Expression::UNARY { .. } => {
            let () = (::match_deref::match_deref! { match &(var_field!((*exp).operator, Expression::NFExpression::UNARY).clone()) {
        Deref @ Operator::OPERATOR { op: Operator::Op::UMINUS, .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsUnaryUminus(ty.clone(), substExp.clone(), exp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            ()
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (List::none(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?, Arc::new({ let __pe_b1 = cref.clone(); move |__pe_a0| Ok(solveUniqueExpressionNoCref(__pe_a0, __pe_b1.clone())) }))) => (),
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } if (List::none(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?, Arc::new({ let __pe_b1 = cref.clone(); move |__pe_a0| Ok(solveUniqueExpressionNoCref(__pe_a0, __pe_b1.clone())) }))) => (),
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } if (List::none(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?, Arc::new({ let __pe_b1 = cref.clone(); move |__pe_a0| Ok(solveUniqueExpressionNoCref(__pe_a0, __pe_b1.clone())) }))) => (),
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (List::hasOneElement(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?)) => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsCallOneArg(ty.clone(), substExp.clone(), exp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            ()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if ((Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?.len() as i32) == 2) => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsCallTwoArgs(ty.clone(), substExp.clone(), exp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            ()
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsMultary(mut substExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefFound: bool, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Status)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut argList: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut invargList: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut crefFoundInRecursion: bool = false;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::MULTARY { .. } => {
            for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(arg.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
                if status.clone() == Status::IMPLICIT.clone() {
                    return Ok((crefFound, inverseInstructions, status));
                }
                if !(crefFoundInRecursion.clone()) {
                    argList = cons(arg.clone(), argList.clone());
                } else {
                    crefFound = true;
                }
            }
            if crefFound.clone() {
                if List::any(var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone(), Arc::new({ let __pe_b1 = cref.clone(); move |__pe_a0| Expression::containsCref(__pe_a0, __pe_b1.clone()) })) {
                    status = Status::IMPLICIT.clone();
                    return Ok((crefFound, inverseInstructions, status));
                } else {
                    inverseInstructions = cons(Arc::new(Expression::NFExpression::MULTARY { arguments: cons(substExp.clone(), var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone()), inv_arguments: argList.clone(), operator: var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone() }), inverseInstructions.clone());
                }
            } else {
                for mut invarg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                    let mut invarg = invarg.clone();
                    (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(invarg.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
                    if status.clone() == Status::IMPLICIT.clone() {
                        return Ok((crefFound, inverseInstructions, status));
                    }
                    if !(crefFoundInRecursion.clone()) {
                        invargList = cons(invarg.clone(), invargList.clone());
                    } else {
                        crefFound = true;
                    }
                }
                if crefFound.clone() {
                    inverseInstructions = cons(Arc::new(Expression::NFExpression::MULTARY { arguments: argList.clone(), inv_arguments: cons(substExp.clone(), invargList.clone()), operator: var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone() }), inverseInstructions.clone());
                }
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsMultary")); __mm_s.push_str(&*literal!(" can only be called for Expression.MULTARY.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsBinaryPow(mut ty: Arc<Type::NFType>, mut substExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefFound: bool, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Status)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool = false;
    let mut local_exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut local_exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { .. } => {
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            if status.clone() == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion.clone() {
                crefFound = true;
                if Expression::containsCref(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), cref.clone())? {
                    status = Status::IMPLICIT.clone();
                } else {
                    inverseInstructions = cons(Arc::new(Expression::NFExpression::BINARY { exp1: substExp.clone(), operator: var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), exp2: Arc::new(Expression::NFExpression::MULTARY { arguments: metamodelica::nil(), inv_arguments: list![var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone()], operator: Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::MUL.clone() }) }) }), inverseInstructions.clone());
                }
            } else {
                (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
                if status.clone() == Status::IMPLICIT.clone() {
                    return Ok((crefFound, inverseInstructions, status));
                }
                if crefFoundInRecursion.clone() {
                    crefFound = true;
                    local_exp1 = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::LOG_REAL().clone(), list![substExp.clone()], Expression::variability(substExp.clone())?, NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::LOG_REAL().returnType.clone()) });
                    local_exp2 = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::LOG_REAL().clone(), list![var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone()], Expression::variability(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone())?, NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::LOG_REAL().returnType.clone()) });
                    inverseInstructions = cons(local_exp1.clone(), cons(Arc::new(Expression::NFExpression::MULTARY { arguments: list![substExp.clone()], inv_arguments: list![local_exp2.clone()], operator: Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::MUL.clone() }) }), inverseInstructions.clone()));
                }
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsBinaryPow")); __mm_s.push_str(&*literal!(" can only be called for Expression.BINARY with operator POW.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsBinaryComOp(mut substExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefFound: bool, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Status)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool = false;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { .. } => {
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructionsMultary(substExp.clone(), Arc::new(Expression::NFExpression::MULTARY { arguments: list![var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone()], inv_arguments: metamodelica::nil(), operator: var_field!((*exp).operator, Expression::NFExpression::BINARY).clone() }), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            if status.clone() == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion.clone() {
                crefFound = true;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsBinaryComOp")); __mm_s.push_str(&*literal!(" can only be called for Expression.BINARY with commutative operator.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsCast(mut substExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefFound: bool, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Status)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool = false;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CAST { .. } => {
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            if status.clone() == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion.clone() {
                crefFound = true;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsCast")); __mm_s.push_str(&*literal!(" can only be called for Expression.CAST.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsUnaryUminus(mut ty: Arc<Type::NFType>, mut substExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefFound: bool, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Status)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool = false;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::UNARY { .. } => {
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            if status.clone() == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion.clone() {
                crefFound = true;
                inverseInstructions = cons(Arc::new(Expression::NFExpression::UNARY { operator: Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::UMINUS.clone() }), exp: substExp.clone() }), inverseInstructions.clone());
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsUnaryUminus")); __mm_s.push_str(&*literal!(" can only be called for Expression.BINARY with commutative operator.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsCallOneArg(mut ty: Arc<Type::NFType>, mut substExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefFound: bool, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Status)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool = false;
    let mut argExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut call: Arc<Call::NFCall>;
    let mut name: ArcStr = arcstr::literal!("");
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (List::hasOneElement(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?)) => {
            name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?).clone();
            argExp = listHead(Call::arguments(call.clone())?)?;
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(argExp.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            if status.clone() == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion.clone() {
                crefFound = true;
                inverseInstructions = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "sqrt" => cons(Arc::new(Expression::NFExpression::BINARY { exp1: substExp.clone(), operator: Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::POW.clone() }), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((2) as f64) }) }), inverseInstructions.clone()),
        Deref @ "cos" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ACOS_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "sin" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ASIN_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "tan" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ATAN_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "acos" => solveUniqueCreateSubstCall(NFBuiltinFuncs::COS_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "asin" => solveUniqueCreateSubstCall(NFBuiltinFuncs::SIN_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "atan" => solveUniqueCreateSubstCall(NFBuiltinFuncs::TAN_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "cosh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ACOSH_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "sinh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ASINH_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "tanh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ATANH_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "acosh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::COSH_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "asinh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::SINH_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "atanh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::TANH_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "exp" => solveUniqueCreateSubstCall(NFBuiltinFuncs::LOG_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "log" => solveUniqueCreateSubstCall(NFBuiltinFuncs::EXP_REAL().clone(), substExp.clone(), inverseInstructions.clone())?,
        Deref @ "log10" => cons(Arc::new(Expression::NFExpression::BINARY { exp1: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((10) as f64) }), operator: Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::POW.clone() }), exp2: substExp.clone() }), inverseInstructions.clone()),
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            inverseInstructions.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsCallOneArg")); __mm_s.push_str(&*literal!(" can only be called for Expression.CALL with one argument.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsCallTwoArgs(mut ty: Arc<Type::NFType>, mut substExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefFound: bool, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Status)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool = false;
    let mut argExp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut argExp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut call: Arc<Call::NFCall>;
    let mut name: ArcStr = arcstr::literal!("");
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if ((Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?.len() as i32) == 2) => {
            name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Call::arguments(call.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            argExp1 = __pa0.clone();
            argExp2 = __pa1.clone();
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(argExp1.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
            if status.clone() == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion.clone() {
                crefFound = true;
                if Expression::containsCref(argExp2.clone(), cref.clone())? {
                    status = Status::IMPLICIT.clone();
                } else {
                    inverseInstructions = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "atan2" => {
            inverseInstructions = cons(Arc::new(Expression::NFExpression::MULTARY { arguments: list![substExp.clone(), argExp2.clone()], inv_arguments: metamodelica::nil(), operator: Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::MUL.clone() }) }), inverseInstructions.clone());
            solveUniqueCreateSubstCall(NFBuiltinFuncs::TAN_REAL().clone(), substExp.clone(), inverseInstructions.clone())?
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            inverseInstructions.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                }
            } else {
                (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(argExp2.clone(), cref.clone(), crefFound.clone(), inverseInstructions.clone())?;
                if status.clone() == Status::IMPLICIT.clone() {
                    return Ok((crefFound, inverseInstructions, status));
                }
                if crefFoundInRecursion.clone() {
                    crefFound = true;
                    inverseInstructions = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "atan2" => {
            inverseInstructions = cons(Arc::new(Expression::NFExpression::MULTARY { arguments: list![argExp1.clone()], inv_arguments: list![substExp.clone()], operator: Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::MUL.clone() }) }), inverseInstructions.clone());
            solveUniqueCreateSubstCall(NFBuiltinFuncs::TAN_REAL().clone(), substExp.clone(), inverseInstructions.clone())?
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            inverseInstructions.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                }
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsCallTwoArgs")); __mm_s.push_str(&*literal!(" can only be called for Expression.CALL with two arguments.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueCreateSubstCall(mut r#fn: Arc<Function::Function>, mut exp: Arc<Expression::NFExpression>, mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = inverseInstructions;
    inverseInstructions = cons(Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![exp.clone()], Expression::variability(exp.clone())?, NFPrefixes::Purity::PURE.clone(), r#fn.returnType.clone()) }), inverseInstructions.clone());
    Ok(inverseInstructions)
}

fn solveUniqueExpressionNoCref(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>) -> bool {
    fn solveUniqueExpressionNoCrefTraverse(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut res: Pointer::Pointer<bool>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        if !(Pointer::access(res.clone())) {
            exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            Pointer::update(res.clone(), ComponentRef::isEqual(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), cref.clone())?);
            exp.clone()
        },
        _ => Expression::mapShallow(exp.clone(), Arc::new(todo!("PARTEVALFUNCTION of solveUniqueExpressionNoCrefTraverse: function signature not resolved")))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(exp)
    }

    let mut b: bool = false;
    let mut res: Pointer::Pointer<bool> = Pointer::create(false);
    Expression::fakeMap(exp.clone(), Arc::new({ let __pe_b1 = cref.clone(); let __pe_b2 = res.clone(); move |__pe_a0| solveUniqueExpressionNoCrefTraverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    b = Pointer::access(res.clone());
    b
}

fn solvePrintInput(mut eqn: Arc<Equation::Equation>, mut crefExp: Arc<ComponentRef::NFComponentRef>) -> Result<()> {
    println!("{}", (literal!("\n##########################################\nSTART - Solve\n\n")).clone());
    println!("{}", (literal!("Solve Input:\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Variable:\n\t")); __mm_s.push_str(&*ComponentRef::toString(crefExp.clone())?); __mm_s.push_str(&*literal!("\n### Equation:\n\t")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn solvePrintOutput(mut eqn: Arc<Equation::Equation>, mut status: Status) -> Result<()> {
    println!("{}", (literal!("Solve Output:\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Status:\n\t")); __mm_s.push_str(&*statusString(status.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Equation:\n\t")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (literal!("\nEND - Solve\n##########################################\n\n")).clone());
    Ok(())
}

fn solveUniquePrintInstructions(mut inverseInstructions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut status: Status) -> Result<()> {
    println!("{}", (literal!("SolveUnique Instructions (substitute from top to bottom):\n")).clone());
    println!("{}", (literal!("\t0 (is initial)\n")).clone());
    for mut instruction in &*inverseInstructions.clone() {
        let mut instruction = instruction.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*Expression::toString(instruction.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Status:\n\t")); __mm_s.push_str(&*statusString(status.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn solveUniquePrintImplicitFallback(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    println!("{}", (literal!("Setting Status.Implicit (fallback) due to:\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Expression:\n\t")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn applyInstruction(mut insertExp: Arc<Expression::NFExpression>, mut instruction: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut insertExp: Arc<Expression::NFExpression> = insertExp;
    insertExp = ({
        let mut argList: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut invargList: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(instruction.clone()) {
        Deref @ Expression::MULTARY { .. } => {
            for mut arg in &*var_field!((*instruction).arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if !(Expression::isSubstitute(arg.clone())) {
                    argList = cons(arg.clone(), argList.clone());
                } else {
                    argList = cons(insertExp.clone(), argList.clone());
                }
            }
            for mut invarg in &*var_field!((*instruction).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                let mut invarg = invarg.clone();
                if !(Expression::isSubstitute(invarg.clone())) {
                    invargList = cons(invarg.clone(), invargList.clone());
                } else {
                    invargList = cons(insertExp.clone(), invargList.clone());
                }
            }
            Arc::new(Expression::NFExpression::MULTARY { arguments: argList.clone(), inv_arguments: invargList.clone(), operator: var_field!((*instruction).operator, Expression::NFExpression::MULTARY).clone() })
        },
        Deref @ Expression::BINARY { .. } => {
            if Expression::isSubstitute(var_field!((*instruction).exp1, Expression::NFExpression::BINARY).clone()) {
                assign_variant_field!(instruction => Expression::NFExpression::BINARY; exp1 = insertExp.clone());
            }
            if Expression::isSubstitute(var_field!((*instruction).exp2, Expression::NFExpression::BINARY).clone()) {
                assign_variant_field!(instruction => Expression::NFExpression::BINARY; exp2 = insertExp.clone());
            }
            instruction.clone()
        },
        Deref @ Expression::UNARY { .. } => {
            if Expression::isSubstitute(var_field!((*instruction).exp, Expression::NFExpression::UNARY).clone()) {
                assign_variant_field!(instruction => Expression::NFExpression::UNARY; exp = insertExp.clone());
            }
            instruction.clone()
        },
        exp @ Deref @ Expression::CALL { .. } => {
            let mut exp = (*exp).clone();
            let () = (::match_deref::match_deref! { match &(var_field!((*instruction).call, Expression::NFExpression::CALL).clone()) {
        local_call @ Deref @ Call::TYPED_CALL { .. } => {
            let mut local_call = (*local_call).clone();
            for mut arg in &*var_field!((*local_call).arguments, Call::NFCall::TYPED_CALL).clone() {
                let mut arg = arg.clone();
                if !(Expression::isSubstitute(arg.clone())) {
                    argList = cons(arg.clone(), argList.clone());
                } else {
                    argList = cons(insertExp.clone(), argList.clone());
                }
            }
            assign_variant_field!(local_call => Call::NFCall::TYPED_CALL; arguments = argList.clone().reverse());
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = local_call.clone());
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.applyInstruction")); __mm_s.push_str(&*literal!(" can only handle TYPED_CALL.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            exp.clone()
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok(insertExp)
}

fn tupleSolvable(mut tuple_exps: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<bool> {
    let mut b: bool = false;
    let mut filtered_exps: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (tuple_exps.clone()).into_iter().cloned() {
            if !(!(Expression::isWildCref(e.clone()))) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, bool>>;
    if List::compareLength(filtered_exps.clone(), vars.clone())? == 0 {
        map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        for mut var in &*vars.clone() {
            let mut var = var.clone();
            UnorderedMap::add(BVariable::getVarName(var.clone()), false, map.clone())?;
        }
        for mut exp in &*filtered_exps.clone() {
            let mut exp = exp.clone();
            let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (UnorderedMap::contains(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), map.clone())) => {
            UnorderedMap::add(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), true, map.clone())?;
            ()
        },
        _ => {
            return Ok(b);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        b = List::all(UnorderedMap::valueList(map.clone()), std::sync::Arc::new(fnptr!(Util::id, _)));
    }
    Ok(b)
}

fn getVarSlice(mut var_cref: Arc<ComponentRef::NFComponentRef>, mut eqn: Arc<Equation::Equation>) -> Result<(Arc<ComponentRef::NFComponentRef>, Status)> {
    let mut var_cref: Arc<ComponentRef::NFComponentRef> = var_cref;
    let mut solve_status: Status = Status::UNPROCESSED;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = BVariable::getVarPointer(var_cref.clone(), metamodelica::sourceInfo!())?;
    let mut slices_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut record_parent: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    slices_lst = Equation::collectCrefs(eqn.clone(), Arc::new({ let __pe_b2 = var_cref.clone(); move |__pe_a0, __pe_a1| Slice::getSliceCandidates(__pe_a0, __pe_a1, __pe_b2.clone()) }), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    if List::hasOneElement(slices_lst.clone()) {
        var_cref = listHead(slices_lst.clone())?;
        solve_status = Status::UNPROCESSED.clone();
    } else {
        record_parent = BVariable::getParent(BVariable::getVarPointer(var_cref.clone(), metamodelica::sourceInfo!())?);
        if Util::isSome(record_parent.clone()) {
            (var_cref, solve_status) = getVarSlice(BVariable::getVarName(Util::getOption(record_parent.clone())?), eqn.clone())?;
        } else {
            solve_status = Status::UNSOLVABLE.clone();
        }
    }
    Ok((var_cref, solve_status))
}

fn solveForVarSlice(mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: BPartition::Kind, mut implicit_index: i32, mut slicing_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, i32, Status)> {
    let mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>> = eqn_slice;
    let mut implicit_index: i32 = implicit_index;
    let mut solve_status: Status = Status::UNPROCESSED;
    let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut var_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    eqn = Pointer::access(Slice::getT(eqn_slice.clone()));
    (var_cref, solve_status) = getVarSlice(BVariable::getVarName(Slice::getT(var_slice.clone())), eqn.clone())?;
    if solve_status.clone() < Status::UNSOLVABLE.clone() {
        (eqn, solve_status, implicit_index, _) = solveEquation(eqn.clone(), var_cref.clone(), funcMap.clone(), kind.clone(), implicit_index.clone(), slicing_map.clone(), varData.clone(), eqData.clone())?;
        eqn_slice = Arc::new(Slice::NBSlice { t: Pointer::create(eqn.clone()), indices: metamodelica::nil() });
    }
    Ok((eqn_slice, implicit_index, solve_status))
}

