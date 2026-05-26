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
use crate::NBAdjacency::Solvability;
use crate::NBBackendUtil as BackendUtil;
use crate::NBCausalize as Causalize;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::EqnSlice;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBInitialization as Initialization;
use crate::NBInline as Inline;
use crate::NBJacobian as BJacobian;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VarSlice;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as Jacobian;
use openmodelica_ast::Absyn::Path;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBTearing.mo
/// package:     NBTearing
/// description: This file contains the data-types used for tearing. It is a
///              uniontype and therefore also contains some structures for tearing.
#[derive(Clone, Debug, PartialEq)]
pub struct NBTearing {
    /// the variables used for iteration
    pub iteration_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>,
    /// implicitely solved residual equations
    pub residual_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>,
    /// array of matched equations and variables
    pub innerEquations: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>,
    /// optional jacobian
    pub jac: Option<Arc<Jacobian::NBackendDAE>>,
}

pub type TEARING_SET = NBTearing;

pub fn hash(mut set: Arc<NBTearing>) -> i32 {
    let mut h: i32 = {
        let mut __acc: i32 = 0;
        for mut var in (set.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::hash(var.clone(), fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>));
            __acc += __x;
        }
        __acc
    };
    h
}

pub fn isEqual(mut set1: Arc<NBTearing>, mut set2: Arc<NBTearing>) -> Result<bool> {
    let mut b: bool = false;
    b = UnorderedSet::equal_list(set1.residual_eqns.clone(), set2.residual_eqns.clone(), { let __pe_b1 = fnptr!(Equation::hash, Pointer::Pointer<Arc<Equation::Equation>>); move |__pe_a0| Ok(Slice::hash(__pe_a0, __pe_b1.clone())) }, { let __pe_b2 = fnptr!(Equation::isEqualPtr, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>); move |__pe_a0, __pe_a1| Ok(Slice::isEqual(__pe_a0, __pe_a1, __pe_b2.clone())) })?;
    b = if (b.clone()) {Array::isEqualOnTrue(set1.innerEquations.clone(), set2.innerEquations.clone(), Arc::new(StrongComponent::isEqual))} else {b.clone()};
    b = if (b.clone()) {UnorderedSet::equal_list(set1.iteration_vars.clone(), set2.iteration_vars.clone(), { let __pe_b1 = fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>); move |__pe_a0| Ok(Slice::hash(__pe_a0, __pe_b1.clone())) }, { let __pe_b2 = fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>); move |__pe_a0, __pe_a1| Ok(Slice::isEqual(__pe_a0, __pe_a1, __pe_b2.clone())) })?} else {b.clone()};
    Ok(b)
}

pub fn size(mut set: Arc<NBTearing>, mut resize: bool) -> Result<i32> {
    let mut s: i32 = 0;
    s = {
        let mut __acc: i32 = 0;
        for mut eq in (set.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::size(eq.clone(), { let __pe_b1 = resize.clone(); move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone()) });
            __acc += __x;
        }
        __acc
    };
    s = s.clone() + {
        let mut __acc: i32 = 0;
        for mut eq in (set.innerEquations.clone()).borrow().iter() {
            let __x = StrongComponent::size(eq.clone(), resize.clone())?;
            __acc += __x;
        }
        __acc
    };
    Ok(s)
}

pub fn toString(mut set: Arc<NBTearing>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = (StringUtil::headline_4((r#str.clone()).clone())).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Iteration Variables:\n")); __mm_s.push_str(&*Slice::lstToString(set.iteration_vars.clone(), fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>), 10)); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n### Residual Equations:\n")); __mm_s.push_str(&*Slice::lstToString(set.residual_eqns.clone(), { let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }, 10)); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n### Inner Equations:\n")); __mm_s.push_str(&*Array::toString(set.innerEquations.clone(), Arc::new({ let __pe_b1 = -1; move |__pe_a0| StrongComponent::toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
    if Util::isSome(set.jac.clone()) {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BJacobian::toString(Util::getOption(set.jac.clone())?, (literal!("NLS")).clone())?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn implicit(mut comp: Arc<StrongComponent::NBStrongComponent>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut index: i32, mut kind: Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, i32)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut index: i32 = index;
    let mut dummy: Arc<Adjacency::Matrix::Matrix> = Arc::new(Adjacency::Matrix::Matrix::EMPTY { st: Adjacency::MatrixStrictness::FULL.clone() });
    let mut new_comp: Arc<StrongComponent::NBStrongComponent>;
    let mut homotopy: Pointer::Pointer<bool> = Pointer::create(false);
    (comp, dummy, index) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => {
            BEquation::Equation::map(Pointer::access(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), { let __pe_b1 = homotopy.clone(); move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone()) }, None, Expression::map)?;
            new_comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { status: Solve::Status::IMPLICIT.clone(), homotopy: Pointer::access(homotopy.clone()), mixed: false, linear: false, casual: None, strict: singleImplicit(var_field!((*comp).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone(), var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), idx: index.clone() });
            finalize(new_comp.clone(), dummy.clone(), funcMap.clone(), index.clone(), BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false), BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), Pointer::create(0), kind.clone())?
        },
        Deref @ StrongComponent::MULTI_COMPONENT { .. } => {
            BEquation::Equation::map(Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone())), { let __pe_b1 = homotopy.clone(); move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone()) }, None, Expression::map)?;
            new_comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { status: Solve::Status::IMPLICIT.clone(), homotopy: Pointer::access(homotopy.clone()), mixed: false, linear: false, casual: None, strict: singleImplicit(Slice::getT(listHead(var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone())?), Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone())), idx: index.clone() });
            finalize(new_comp.clone(), dummy.clone(), funcMap.clone(), index.clone(), BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false), BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), Pointer::create(0), kind.clone())?
        },
        Deref @ StrongComponent::RESIZABLE_COMPONENT { .. } => {
            BEquation::Equation::map(Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone())), { let __pe_b1 = homotopy.clone(); move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone()) }, None, Expression::map)?;
            new_comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { status: Solve::Status::IMPLICIT.clone(), homotopy: Pointer::access(homotopy.clone()), mixed: false, linear: false, casual: None, strict: singleImplicit(Slice::getT(var_field!((*comp).var, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone()), Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone())), idx: index.clone() });
            finalize(new_comp.clone(), dummy.clone(), funcMap.clone(), index.clone(), BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false), BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), Pointer::create(0), kind.clone())?
        },
        _ => (comp.clone(), dummy.clone(), index.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, index))
}

pub fn singleImplicit(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut eqn: Pointer::Pointer<Arc<Equation::Equation>>) -> Arc<NBTearing> {
    let mut tearingSet: Arc<NBTearing> = Arc::new(NBTearing { jac: None, innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), residual_eqns: list![Arc::new(Slice::NBSlice { t: eqn.clone(), indices: metamodelica::nil() })], iteration_vars: list![Arc::new(Slice::NBSlice { t: var.clone(), indices: metamodelica::nil() })] });
    tearingSet
}

pub fn getModule() -> Result<Arc<metamodelica::List<Module::tearingInterface>>> {
    let mut funcs: Arc<metamodelica::List<Module::tearingInterface>> = metamodelica::nil();
    let mut flag: ArcStr = Flags::getConfigString(Flags::TEARING_METHOD.clone())?;
    funcs = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "minimalTearing" => list![todo!("PARTEVALFUNCTION of initialize: function signature not resolved"), minimal.clone(), finalize.clone()],
        Deref @ "cellier" => list![todo!("PARTEVALFUNCTION of initialize: function signature not resolved"), minimal.clone(), finalize.clone()],
        Deref @ "omcTearing" => list![todo!("PARTEVALFUNCTION of initialize: function signature not resolved"), minimal.clone(), finalize.clone()],
        Deref @ "guruTearing" => list![todo!("PARTEVALFUNCTION of initialize: function signature not resolved"), guru.clone(), finalize.clone()],
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub fn getVariables(mut tearing: Arc<NBTearing>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut variables: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    variables = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (cons({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (tearing.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut comp in (tearing.innerEquations.clone()).borrow().iter() {
            let __x = StrongComponent::getVariables(comp.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })).into_iter().cloned() {
            let __x = var.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    };
    Ok(variables)
}

pub fn getResidualVars(mut tearing: Arc<NBTearing>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut residuals: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut eqn in (tearing.residual_eqns.clone()).into_iter().cloned() {
            let __x = BEquation::Equation::getResidualVar(Slice::getT(eqn.clone())).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    residuals
}

pub fn getIterationVars(mut tearing: Arc<NBTearing>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut iterationVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (tearing.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    iterationVars
}

pub fn getResidualEqns(mut tearing: Arc<NBTearing>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> {
    let mut residuals: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut eqn in (tearing.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    residuals
}

pub fn setResidualEqns(mut tearing: Arc<NBTearing>, mut residuals: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>) -> Arc<NBTearing> {
    let mut tearing: Arc<NBTearing> = tearing;
    assign_field!(tearing.residual_eqns = residuals.clone());
    tearing
}

fn tearingTraverser(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut funcs: Arc<metamodelica::List<Module::tearingInterface>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut eq_index: Pointer::Pointer<i32>, mut kind: Partition::Kind) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> {
    let mut new_partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
    let mut strongComponents: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>;
    let mut tmp: Arc<StrongComponent::NBStrongComponent>;
    let mut idx: i32 = 0;
    let mut full: Arc<Adjacency::Matrix::Matrix>;
    for mut part in &*partitions.clone() {
        let mut part = part.clone();
        if isSome(part.strongComponents.clone()) && isSome(part.adjacencyMatrix.clone()) {
            let Some(__pa0) = (part.strongComponents.clone()) else { bail!("pattern mismatch") };
            strongComponents = __pa0.clone();
            let __pa1 = ::match_deref::match_deref! { match &(part.adjacencyMatrix.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            full = __pa1.clone();
            let __range2 = 1..=(strongComponents.clone().borrow().len() as i32);
            for mut i in __range2 {
                tmp = strongComponents.borrow()[(i.clone()-1) as usize].clone();
                for mut func in &*funcs.clone() {
                    let mut func = func.clone();
                    (tmp, full, idx) = func(tmp.clone(), full.clone(), funcMap.clone(), idx.clone(), part.unknowns.clone(), part.equations.clone(), eq_index.clone(), kind.clone())?;
                }
                if !(referenceEq(&tmp.clone(),&strongComponents.borrow()[(i.clone()-1) as usize].clone())) {
                    {let _arr = strongComponents.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = tmp.clone(); _arr};
                }
            }
            assign_field!(
                part.strongComponents = Some(strongComponents.clone()),
                part.adjacencyMatrix = Some(full.clone())
            );
        }
        new_partitions = cons(part.clone(), new_partitions.clone());
    }
    new_partitions = new_partitions.clone().reverse();
    Ok(new_partitions)
}

fn checkLinearity(mut full: Arc<Adjacency::Matrix::Matrix>, mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut e: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<bool> {
    fn varIsLinear(mut var: Arc<ComponentRef::NFComponentRef>, mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut sol: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>) -> bool {
        let mut b: bool = !(UnorderedMap::contains(var.clone(), v.clone()) && Adjacency::Solvability::isNonlinearOrImplicit(UnorderedMap::getSafe(var.clone(), sol.clone(), metamodelica::sourceInfo!()).unwrap()));
        b
    }

    fn eqnIsLinear(mut i: i32, mut occ: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>, mut sol: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>>, mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> bool {
        let mut b: bool = UnorderedSet::all(occ.borrow()[(i.clone()-1) as usize].clone(), Arc::new(todo!("PARTEVALFUNCTION of varIsLinear: function signature not resolved")));
        b
    }

    let mut linear: bool = false;
    linear = (::match_deref::match_deref! { match &(full.clone()) {
        Deref @ Adjacency::Matrix::FULL { .. } => UnorderedMap::all(e.clone(), Arc::new({ let __pe_b1 = var_field!((*full).occurrences, Adjacency::Matrix::Matrix::FULL).clone(); let __pe_b2 = var_field!((*full).solvabilities, Adjacency::Matrix::Matrix::FULL).clone(); let __pe_b3 = v.clone(); move |__pe_a0| Ok(eqnIsLinear(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())) })),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.checkLinearity")); __mm_s.push_str(&*literal!(" expected type full, got type ")); __mm_s.push_str(&*Adjacency::Matrix::strictnessString(Adjacency::Matrix::getStrictness(full.clone())?)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(linear)
}

fn filterDiscreteVariables(mut vars_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut init: bool) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>)> {
    fn addDiscreteRecord(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut discrete_records: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        let _ = (match BVariable::getParent(var.clone()) {
        Some(mut parent) => {
            UnorderedSet::add(BVariable::getVarName(parent.clone()), discrete_records.clone())?;
            addDiscreteRecord(parent.clone(), discrete_records.clone())?;
            ()
        },
        _ => {
            ()
        },
    });
        Ok(())
    }

    fn checkDiscreteRecord(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut discrete_records: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut is_parent: bool) -> Result<bool> {
        let mut b: bool = false;
        b = (match BVariable::getParent(var.clone()) {
        Some(mut parent) => {
            checkDiscreteRecord(parent.clone(), discrete_records.clone(), true)?
        },
        _ => {
            is_parent.clone() && UnorderedSet::contains(BVariable::getVarName(var.clone()), discrete_records.clone())?
        },
    });
        Ok(b)
    }

    let mut cont_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut disc_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut discrete_records: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 13);
    let mut rec_disc_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    (cont_vars, disc_vars) = List::splitOnTrue(vars_lst.clone(), Arc::new({ let __pe_b1 = init.clone(); move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone()) }));
    for mut var in &*disc_vars.clone() {
        let mut var = var.clone();
        addDiscreteRecord(var.clone(), discrete_records.clone())?;
    }
    (rec_disc_vars, cont_vars) = List::splitOnTrue(cont_vars.clone(), Arc::new({ let __pe_b1 = discrete_records.clone(); let __pe_b2 = false; move |__pe_a0| checkDiscreteRecord(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    disc_vars = listAppend(rec_disc_vars.clone(), disc_vars.clone()).reverse();
    Ok((cont_vars, disc_vars))
}

fn getImpliedInnerVars(mut eqn: Pointer::Pointer<Arc<Equation::Equation>>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    vars = (::match_deref::match_deref! { match &(Pointer::access(eqn.clone())) {
        Deref @ BEquation::Equation::ALGORITHM { alg, .. } => {
            {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut out_cr in (alg.outputs.clone()).into_iter().cloned() {
            let __x = BVariable::getVarPointer(out_cr.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: tpl @ Deref @ Expression::TUPLE { .. }, .. } => {
            {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut tpl_cr in (UnorderedSet::toList(Expression::extractCrefs(tpl.clone()))).into_iter().cloned() {
            let __x = BVariable::getVarPointer(tpl_cr.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { .. }, .. } => {
            metamodelica::nil()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vars)
}


