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

use crate::NBBackendUtil as BackendUtil;
use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBDifferentiate::DifferentiationType;
use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBPartition as Partition;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn::Path;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System as BuiltinSystem;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

// self import
// OF imports
// NF imports
// NB imports
// Util import
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum MatrixStrictness {
    LINEAR = 1,
    MATCHING = 2,
    SORTING = 3,
    FULL = 4,
}
impl PartialOrd for MatrixStrictness {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for MatrixStrictness {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for MatrixStrictness {
    fn default() -> Self { Self::LINEAR }
}

pub fn strictnessString(mut s: MatrixStrictness) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match s.clone() {
        MatrixStrictness::LINEAR { .. } => literal!("linear"),
        MatrixStrictness::MATCHING { .. } => literal!("matching"),
        MatrixStrictness::SORTING => literal!("sorting"),
        MatrixStrictness::FULL { .. } => literal!("full"),
        _ => literal!("unknown"),
    })).clone();
    r#str
}

pub mod Mapping {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Mapping {
        /// eqn: scal_idx -> arr_idx
        pub eqn_StA: metamodelica::Array<i32>,
        /// var: scal_idx -> arr_idx
        pub var_StA: metamodelica::Array<i32>,
        /// eqn: arr_idx -> start_idx/length
        pub eqn_AtS: metamodelica::Array<(i32, i32)>,
        /// var: arr_idx -> start_idx/length
        pub var_AtS: metamodelica::Array<(i32, i32)>,
    }

    pub type MAPPING = Mapping;

    pub fn toString(mut mapping: Arc<Mapping>) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut start: i32 = 0;
        let mut size: i32 = 0;
        r#str = (StringUtil::headline_4((literal!("Equation Index Mapping (ARR) -> START | SIZE")).clone())).clone();
        let __range0 = 1..=(mapping.eqn_AtS.clone().borrow().len() as i32);
        for mut i in __range0 {
            (start, size) = mapping.eqn_AtS.borrow()[(i.clone()-1) as usize].clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\t")); __mm_s.push_str(&*intString(start.clone())); __mm_s.push_str(&*literal!(" | ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("Variable Index Mapping (ARR) -> START | SIZE")).clone())); ArcStr::from(__mm_s) }).clone();
        let __range1 = 1..=(mapping.var_AtS.clone().borrow().len() as i32);
        for mut i in __range1 {
            (start, size) = mapping.var_AtS.borrow()[(i.clone()-1) as usize].clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\t")); __mm_s.push_str(&*intString(start.clone())); __mm_s.push_str(&*literal!(" | ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        r#str
    }

    pub fn empty() -> Arc<Mapping> {
        let mut mapping: Arc<Mapping> = Arc::new(Mapping { eqn_StA: arrayCreate(0, 0), var_StA: arrayCreate(0, 0), eqn_AtS: arrayCreate(0, (0, 0)), var_AtS: arrayCreate(0, (0, 0)) });
        mapping
    }

    pub fn create(mut eqns: Arc<EquationPointers::EquationPointers>, mut vars: Arc<VariablePointers::VariablePointers>) -> Result<Arc<Mapping>> {
        let mut mapping: Arc<Mapping>;
        let mut eqn_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = BEquation::EquationPointers::toList(eqns.clone())?;
        let mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = BVariable::VariablePointers::toList(vars.clone())?;
        let mut eqn_StA: metamodelica::Array<i32>;
        let mut var_StA: metamodelica::Array<i32>;
        let mut eqn_AtS: metamodelica::Array<(i32, i32)>;
        let mut var_AtS: metamodelica::Array<(i32, i32)>;
        let mut eqn_scalar_size: i32 = 0;
        let mut var_scalar_size: i32 = 0;
        let mut eqn_idx_scal: i32 = 1;
        let mut eqn_idx_arr: i32 = 1;
        let mut var_idx_scal: i32 = 1;
        let mut var_idx_arr: i32 = 1;
        eqn_scalar_size = {
        let mut __acc: i32 = 0;
        for mut eqn in (eqn_lst.clone()).into_iter().cloned() {
            let __x = BEquation::Equation::size(eqn.clone(), true)?;
            __acc += __x;
        }
        __acc
    };
        var_scalar_size = {
        let mut __acc: i32 = 0;
        for mut var in (var_lst.clone()).into_iter().cloned() {
            let __x = BVariable::size(var.clone(), true);
            __acc += __x;
        }
        __acc
    };
        eqn_StA = arrayCreate(eqn_scalar_size.clone(), -1);
        var_StA = arrayCreate(var_scalar_size.clone(), -1);
        eqn_AtS = arrayCreate(BEquation::EquationPointers::size(eqns.clone()), (-1, -1));
        var_AtS = arrayCreate(BVariable::VariablePointers::size(vars.clone()), (-1, -1));
        (eqn_StA, var_StA, eqn_AtS, var_AtS) = fill_(eqn_StA.clone(), var_StA.clone(), eqn_AtS.clone(), var_AtS.clone(), eqn_lst.clone(), var_lst.clone(), eqn_idx_scal.clone(), eqn_idx_arr.clone(), var_idx_scal.clone(), var_idx_arr.clone())?;
        mapping = Arc::new(Mapping { eqn_StA: eqn_StA.clone(), var_StA: var_StA.clone(), eqn_AtS: eqn_AtS.clone(), var_AtS: var_AtS.clone() });
        Ok(mapping)
    }

    pub fn expand(mut mapping: Arc<Mapping>, mut eqn_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<Arc<Mapping>> {
        let mut mapping: Arc<Mapping> = mapping;
        let mut eqn_StA: metamodelica::Array<i32>;
        let mut var_StA: metamodelica::Array<i32>;
        let mut eqn_AtS: metamodelica::Array<(i32, i32)>;
        let mut var_AtS: metamodelica::Array<(i32, i32)>;
        let mut eqn_scalar_size: i32 = 0;
        let mut var_scalar_size: i32 = 0;
        let mut neqn_scal: i32 = {
        let mut __acc: i32 = 0;
        for mut eqn in (eqn_lst.clone()).into_iter().cloned() {
            let __x = BEquation::Equation::size(eqn.clone(), true)?;
            __acc += __x;
        }
        __acc
    };
        let mut nvar_scal: i32 = {
        let mut __acc: i32 = 0;
        for mut var in (var_lst.clone()).into_iter().cloned() {
            let __x = BVariable::size(var.clone(), true);
            __acc += __x;
        }
        __acc
    };
        let mut neqn_arr: i32 = (eqn_lst.clone().len() as i32);
        let mut nvar_arr: i32 = (var_lst.clone().len() as i32);
        let mut eqn_idx_scal: i32 = (mapping.eqn_StA.clone().borrow().len() as i32) + 1;
        let mut eqn_idx_arr: i32 = (mapping.eqn_AtS.clone().borrow().len() as i32) + 1;
        let mut var_idx_scal: i32 = (mapping.var_StA.clone().borrow().len() as i32) + 1;
        let mut var_idx_arr: i32 = (mapping.var_AtS.clone().borrow().len() as i32) + 1;
        eqn_StA = Array::expandToSize(eqn_idx_scal.clone() - 1 + neqn_scal.clone(), mapping.eqn_StA.clone(), -1)?;
        var_StA = Array::expandToSize(var_idx_scal.clone() - 1 + nvar_scal.clone(), mapping.var_StA.clone(), -1)?;
        eqn_AtS = Array::expandToSize(eqn_idx_arr.clone() - 1 + neqn_arr.clone(), mapping.eqn_AtS.clone(), (-1, -1))?;
        var_AtS = Array::expandToSize(var_idx_arr.clone() - 1 + nvar_arr.clone(), mapping.var_AtS.clone(), (-1, -1))?;
        (eqn_StA, var_StA, eqn_AtS, var_AtS) = fill_(eqn_StA.clone(), var_StA.clone(), eqn_AtS.clone(), var_AtS.clone(), eqn_lst.clone(), var_lst.clone(), eqn_idx_scal.clone(), eqn_idx_arr.clone(), var_idx_scal.clone(), var_idx_arr.clone())?;
        mapping = Arc::new(Mapping { eqn_StA: eqn_StA.clone(), var_StA: var_StA.clone(), eqn_AtS: eqn_AtS.clone(), var_AtS: var_AtS.clone() });
        Ok(mapping)
    }

    pub fn getEqnScalIndices(mut arr_idx: i32, mut mapping: Arc<Mapping>, mut reverse: bool) -> Arc<metamodelica::List<i32>> {
        let mut scal_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut start: i32 = 0;
        let mut length: i32 = 0;
        (start, length) = mapping.eqn_AtS.borrow()[(arr_idx.clone()-1) as usize].clone();
        scal_indices = if (reverse.clone()) {List::intRange2(start.clone() + length.clone() - 1, start.clone())} else {List::intRange2(start.clone(), start.clone() + length.clone() - 1)};
        scal_indices
    }

    pub fn getVarScalIndices(mut arr_idx: i32, mut mapping: Arc<Mapping>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut reverse: bool) -> Result<Arc<metamodelica::List<i32>>> {
        fn subscriptedIndices(mut start: i32, mut length: i32, mut slice: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
            let mut scal_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
            scal_indices = List::intRange2(start.clone(), start.clone() + length.clone() - 1);
            if !(slice.clone().is_empty()) {
                scal_indices = List::keepPositions(scal_indices.clone(), slice.clone(), false)?;
            }
            Ok(scal_indices)
        }

        let mut scal_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut start: i32 = 0;
        let mut length: i32 = 0;
        (start, length) = mapping.var_AtS.borrow()[(arr_idx.clone()-1) as usize].clone();
        scal_indices = ({
        let mut slice: Arc<metamodelica::List<i32>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(subs.clone()) {
        Deref @ metamodelica::List::Nil => {
            subscriptedIndices(start.clone(), length.clone(), metamodelica::nil())?
        },
        _ if (List::all(subs.clone(), (std::sync::Arc::new(fnptr!(Subscript::isWhole, Arc<Subscript::NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<bool> + 'static>))) => {
            subscriptedIndices(start.clone(), length.clone(), metamodelica::nil())?
        },
        Deref @ metamodelica::List::Cons { head: sub, tail: Deref @ metamodelica::List::Nil } => {
            slice = Subscript::toIndexList(sub.clone(), length.clone())?;
            subscriptedIndices(start.clone(), length.clone(), slice.clone())?
        },
        _ => {
            let mut subs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
            let mut dim_sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut values: Arc<metamodelica::List<i32>> = metamodelica::nil();
            subs_lst = Subscript::scalarizeList(subs.clone(), dims.clone(), true)?;
            subs_lst = List::combination(subs_lst.clone());
            dim_sizes = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dim in (dims.clone()).into_iter().cloned() {
            let __x = Dimension::size(dim.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            for mut sub_lst in &*subs_lst.clone().reverse() {
                let mut sub_lst = sub_lst.clone();
                values = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut s in (sub_lst.clone()).into_iter().cloned() {
            let __x = Subscript::toInteger(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                slice = cons(Slice::locationToIndex(dim_sizes.clone(), values.clone(), start.clone())?, slice.clone());
            }
            slice.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        if reverse.clone() {
            scal_indices = scal_indices.clone().reverse();
        }
        Ok(scal_indices)
    }

    fn fill_(mut eqn_StA: metamodelica::Array<i32>, mut var_StA: metamodelica::Array<i32>, mut eqn_AtS: metamodelica::Array<(i32, i32)>, mut var_AtS: metamodelica::Array<(i32, i32)>, mut eqn_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut eqn_idx_scal_start: i32, mut eqn_idx_arr_start: i32, mut var_idx_scal_start: i32, mut var_idx_arr_start: i32) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<(i32, i32)>, metamodelica::Array<(i32, i32)>)> {
        let mut eqn_StA: metamodelica::Array<i32> = eqn_StA;
        let mut var_StA: metamodelica::Array<i32> = var_StA;
        let mut eqn_AtS: metamodelica::Array<(i32, i32)> = eqn_AtS;
        let mut var_AtS: metamodelica::Array<(i32, i32)> = var_AtS;
        let mut size: i32 = 0;
        let mut eqn_idx_scal: i32 = eqn_idx_scal_start.clone();
        let mut eqn_idx_arr: i32 = eqn_idx_arr_start.clone();
        let mut var_idx_scal: i32 = var_idx_scal_start.clone();
        let mut var_idx_arr: i32 = var_idx_arr_start.clone();
        for mut eqn_ptr in &*eqn_lst.clone() {
            let mut eqn_ptr = eqn_ptr.clone();
            size = BEquation::Equation::size(eqn_ptr.clone(), true)?;
            {
                let __cell0 = (eqn_idx_scal.clone(), size.clone());
                eqn_AtS.clone().borrow_mut()[(eqn_idx_arr.clone()-1) as usize] = __cell0;
            }
            for mut i in eqn_idx_scal.clone()..=eqn_idx_scal.clone() + size.clone() - 1 {
                {
                    let __cell1 = eqn_idx_arr.clone();
                    eqn_StA.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                }
            }
            eqn_idx_scal = eqn_idx_scal.clone() + size.clone();
            eqn_idx_arr = eqn_idx_arr.clone() + 1;
        }
        for mut var_ptr in &*var_lst.clone() {
            let mut var_ptr = var_ptr.clone();
            size = BVariable::size(var_ptr.clone(), true);
            {
                let __cell2 = (var_idx_scal.clone(), size.clone());
                var_AtS.clone().borrow_mut()[(var_idx_arr.clone()-1) as usize] = __cell2;
            }
            for mut i in var_idx_scal.clone()..=var_idx_scal.clone() + size.clone() - 1 {
                {
                    let __cell3 = var_idx_arr.clone();
                    var_StA.clone().borrow_mut()[(i.clone()-1) as usize] = __cell3;
                }
            }
            var_idx_scal = var_idx_scal.clone() + size.clone();
            var_idx_arr = var_idx_arr.clone() + 1;
        }
        Ok((eqn_StA, var_StA, eqn_AtS, var_AtS))
    }

}

pub mod Mode {
    use super::*;
    /// most of the time this will only have one cref. if there are multiple crefs
    ///      representing the same variable its a multi mode and the equation needs to
    ///      be split when solved for it
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Mode {
        /// the equation name
        pub eqn_name: Arc<ComponentRef::NFComponentRef>,
        /// the cref(s) to solve for
        pub crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
        /// true if the equation needs to be scalarized to find the cref to solve for
        pub scalarize: bool,
    }

    impl Default for Mode {
        fn default() -> Self {
            Self {
                eqn_name: Default::default(),
                crefs: Default::default(),
                scalarize: Default::default(),
            }
        }
    }

    pub type MODE = Mode;

    pub fn toString(mut mode: Arc<Mode>) -> ArcStr {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[eqn: ")); __mm_s.push_str(&*ComponentRef::toString(mode.eqn_name.clone()).unwrap()); __mm_s.push_str(&*literal!(", crefs: ")); __mm_s.push_str(&*List::toString(mode.crefs.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0).unwrap()); __mm_s.push_str(&*literal!(", scal: ")); __mm_s.push_str(&*boolString(mode.scalarize.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
        r#str
    }

    pub fn hash(mut mode: Arc<Mode>) -> i32 {
        let mut hash: i32 = ComponentRef::hash(mode.eqn_name.clone());
        hash
    }

    pub fn isEqual(mut mode1: Arc<Mode>, mut mode2: Arc<Mode>) -> bool {
        let mut b: bool = ComponentRef::isEqual(mode1.eqn_name.clone(), mode2.eqn_name.clone()).unwrap() && mode1.scalarize.clone() == mode2.scalarize.clone() && List::isEqualOnTrue(mode1.crefs.clone(), mode2.crefs.clone(), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>));
        b
    }

    pub fn create(mut eqn_name: Arc<ComponentRef::NFComponentRef>, mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut scalarize: bool) -> Arc<Mode> {
        let mut mode: Arc<Mode> = Arc::new(Mode { eqn_name: eqn_name.clone(), crefs: {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut cref in (crefs.clone()).into_iter().cloned() {
            let __x = ComponentRef::simplifySubscripts(cref.clone(), false).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, scalarize: scalarize.clone() });
        mode
    }

    pub fn merge(mut mode1: Arc<Mode>, mut mode2: Arc<Mode>) -> Arc<Mode> {
        let mut oMode: Arc<Mode> = Arc::new(Mode { eqn_name: mode1.eqn_name.clone(), crefs: listAppend(mode1.crefs.clone(), mode2.crefs.clone()), scalarize: mode1.scalarize.clone() || mode2.scalarize.clone() });
        oMode
    }

    pub fn mergeCreate(mut omode: Option<Arc<Mode>>, mut mode: Arc<Mode>) -> Arc<Mode> {
        let mut mode: Arc<Mode> = mode;
        mode = Util::applyOptionOrDefault(omode.clone(), Arc::new({ let __pe_b0 = mode.clone(); move |__pe_a1| Ok(merge(__pe_b0.clone(), __pe_a1)) }), mode.clone());
        mode
    }

    pub type Key = (i32, i32);

    pub fn keyString(mut key: Key) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut e: i32 = 0;
        let mut v: i32 = 0;
        (e, v) = key.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(v.clone())); ArcStr::from(__mm_s) }).clone();
        r#str
    }

    pub fn keyHash(mut key: Key) -> i32 {
        let mut hash: i32 = 0;
        let mut e: i32 = 0;
        let mut v: i32 = 0;
        (e, v) = key.clone();
        hash = e.clone() * 31 + v.clone();
        hash
    }

    pub fn keyEqual(mut key1: Key, mut key2: Key) -> bool {
        let mut b: bool = false;
        let mut e1: i32 = 0;
        let mut e2: i32 = 0;
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        (e1, v1) = key1.clone();
        (e2, v2) = key2.clone();
        b = e1.clone() == e2.clone() && v1.clone() == v2.clone();
        b
    }

}

pub mod Matrix {
    use super::*;
    /// used to store adjacency information for the bipartite graph representing the system of equations and variables
    ///    you have to create it in this specific order: EMPTY->FULL->FINAL(LINEAR)->FINAL->(MATCHING)->FINAL(SORTING)
    ///    and store the FULL for further use.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Matrix {
        /// placeholder for empty matrices, just stores intended strictness
        EMPTY {
            st: MatrixStrictness,
        },
        /// contains all information needed. create specific final matrices from this
        FULL {
            equation_names: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>,
            occurrences: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>,
            dependencies: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>>,
            solvabilities: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>>,
            repetitions: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>,
            mapping: Arc<Mapping::Mapping>,
        },
        /// specific final matrix, defined by its strictness
        FINAL {
            /// eqn -> list<var>
            m: metamodelica::Array<Arc<metamodelica::List<i32>>>,
            /// var -> list<eqn>
            mT: metamodelica::Array<Arc<metamodelica::List<i32>>>,
            /// index mapping scalar <-> array
            mapping: Arc<Mapping::Mapping>,
            /// array reconstruction information
            modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>,
            /// strictness with which it was created
            st: MatrixStrictness,
        },
        SPARSITY {
            equation_names: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>,
            dependencies: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>>,
            repetitions: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>,
            solved_variables: metamodelica::Array<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>,
        },
    }
    impl Default for Matrix {
        fn default() -> Self {
            Self::EMPTY {
                st: Default::default(),
            }
        }
    }
    pub use self::Matrix::{EMPTY,FULL,FINAL,SPARSITY};
    pub fn createFull(mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>, mut kind: Partition::Kind) -> Result<Arc<Matrix>> {
        let mut adj: Arc<Matrix>;
        let mut index: i32 = 0;
        let mut size: i32 = BEquation::EquationPointers::size(eqns.clone());
        let mut equation_names: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut occurrences: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>;
        let mut dependencies: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>>;
        let mut solvabilities: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>>;
        let mut repetitions: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>;
        let mut occ_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
        let mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
        let mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>;
        let mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>;
        let mut mapping: Arc<Mapping::Mapping>;
        if ExpandableArray::getNumberOfElements(vars.varArr.clone()) > 0 || ExpandableArray::getNumberOfElements(eqns.eqArr.clone()) > 0 {
            equation_names = arrayCreate(size.clone(), Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY));
            occurrences = arrayCreate(size.clone(), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13));
            dependencies = arrayCreate(size.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1));
            solvabilities = arrayCreate(size.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1));
            repetitions = arrayCreate(size.clone(), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13));
            for mut eqn_ptr in &*BEquation::EquationPointers::toList(eqns.clone())? {
                let mut eqn_ptr = eqn_ptr.clone();
                index = UnorderedMap::getSafe(BEquation::Equation::getEqnName(eqn_ptr.clone())?, eqns.map.clone(), metamodelica::sourceInfo!())?;
                dep_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                sol_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                rep_set = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
                occ_set = collectDependenciesEquation(Pointer::access(eqn_ptr.clone()), kind.clone(), vars.map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                addInitialStartOccurrences(occ_set.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone(), kind.clone())?;
                {
                    let __cell0 = BEquation::Equation::getEqnName(eqn_ptr.clone())?;
                    equation_names.clone().borrow_mut()[(index.clone()-1) as usize] = __cell0;
                }
                {
                    let __cell1 = occ_set.clone();
                    occurrences.clone().borrow_mut()[(index.clone()-1) as usize] = __cell1;
                }
                {
                    let __cell2 = dep_map.clone();
                    dependencies.clone().borrow_mut()[(index.clone()-1) as usize] = __cell2;
                }
                {
                    let __cell3 = sol_map.clone();
                    solvabilities.clone().borrow_mut()[(index.clone()-1) as usize] = __cell3;
                }
                {
                    let __cell4 = rep_set.clone();
                    repetitions.clone().borrow_mut()[(index.clone()-1) as usize] = __cell4;
                }
            }
            mapping = Mapping::create(eqns.clone(), vars.clone())?;
            adj = Arc::new(Matrix::FULL { equation_names: equation_names.clone(), occurrences: occurrences.clone(), dependencies: dependencies.clone(), solvabilities: solvabilities.clone(), repetitions: repetitions.clone(), mapping: mapping.clone() });
        } else {
            adj = Arc::new(Matrix::EMPTY { st: MatrixStrictness::FULL.clone() });
        }
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1((literal!("Creating Adjacency Matrices")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::EquationPointers::toString(eqns.clone(), (literal!("")).clone(), None, true, None)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BVariable::VariablePointers::toString(vars.clone(), (literal!("")).clone(), None, true)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(adj.clone(), (literal!("Full")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*solvabilityString(adj.clone(), (literal!("Full")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dependencyString(adj.clone(), (literal!("Full")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok(adj)
    }

    pub fn fullToFinal(mut full: Arc<Matrix>, mut vars_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut eqns_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut eqns: Arc<EquationPointers::EquationPointers>, mut st: MatrixStrictness, mut iter: Arc<Iterator::Iterator>) -> Arc<Matrix> {
        let mut adj: Arc<Matrix> = upgrade(Arc::new(Matrix::EMPTY { st: MatrixStrictness::FULL.clone() }), full.clone(), vars_map.clone(), eqns_map.clone(), eqns.clone(), st.clone(), iter.clone()).unwrap();
        adj
    }

    pub fn fullToSparsity(mut full: Arc<Matrix>, mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>) -> Result<Arc<Matrix>> {
        let mut sparse: Arc<Matrix>;
        sparse = ({
        let mut index_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut inner_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut eqn_names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut deps: Arc<metamodelica::List<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>>> = metamodelica::nil();
        let mut reps: Arc<metamodelica::List<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        let mut solved_vars: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(full.clone()) {
        Deref @ FULL { .. } => {
            let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut eqn_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut dep_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut eqn_index: i32 = 0;
            let mut dep: Arc<Dependency::Dependency> = Arc::new(<Dependency::Dependency as ::std::default::Default>::default());
            let mut local_deps: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<Dependency::Dependency>, bool)>> = metamodelica::nil();
            let mut repeated: bool = false;
            let mut inner_deps: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut changed: bool = false;
            let mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>;
            let mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let __range0 = 1..=(var_field!((*full).equation_names, Matrix::FULL).clone().borrow().len() as i32);
            for mut i in __range0 {
                UnorderedMap::add(var_field!((*full).equation_names, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(), i.clone(), index_map.clone())?;
            }
            for mut comp in &*comps.clone() {
                let mut comp = comp.clone();
                eqns = StrongComponent::getEquations(comp.clone())?;
                vars = StrongComponent::getVariables(comp.clone())?;
                for mut eqn in &*eqns.clone() {
                    let mut eqn = eqn.clone();
                    eqn_name = BEquation::Equation::getEqnName(eqn.clone())?;
                    eqn_index = UnorderedMap::getSafe(eqn_name.clone(), index_map.clone(), metamodelica::sourceInfo!())?;
                    local_deps = metamodelica::nil();
                    changed = false;
                    let __range1 = &*UnorderedMap::toList(var_field!((*full).dependencies, Matrix::FULL).borrow()[(eqn_index.clone()-1) as usize].clone());
                    for mut tpl in __range1 {
                        let mut tpl = tpl.clone();
                        (dep_cref, dep) = tpl.clone();
                        repeated = UnorderedSet::contains(dep_cref.clone(), var_field!((*full).repetitions, Matrix::FULL).borrow()[(eqn_index.clone()-1) as usize].clone())?;
                        (inner_deps, changed) = (::match_deref::match_deref! { match &(UnorderedMap::get(dep_cref.clone(), inner_map.clone())) {
        Some(inner_deps) => (inner_deps.clone(), true),
        _ => (list![dep_cref.clone()], changed.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                        local_deps = cons((inner_deps.clone(), dep.clone(), repeated.clone()), local_deps.clone());
                    }
                    if List::any(vars.clone(), (std::sync::Arc::new(fnptr!(BVariable::isJacobianResultVar, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>)) {
                        if changed.clone() {
                            dep_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                            rep_set = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
                            for mut tpl in &*local_deps.clone() {
                                let mut tpl = tpl.clone();
                                (inner_deps, dep, repeated) = tpl.clone();
                                for mut dep_cref in &*inner_deps.clone() {
                                    let mut dep_cref = dep_cref.clone();
                                    UnorderedMap::add(dep_cref.clone(), dep.clone(), dep_map.clone())?;
                                    if repeated.clone() {
                                        UnorderedSet::add(dep_cref.clone(), rep_set.clone())?;
                                    }
                                }
                            }
                        } else {
                            dep_map = var_field!((*full).dependencies, Matrix::FULL).borrow()[(eqn_index.clone()-1) as usize].clone();
                            rep_set = var_field!((*full).repetitions, Matrix::FULL).borrow()[(eqn_index.clone()-1) as usize].clone();
                        }
                        eqn_names = cons(eqn_name.clone(), eqn_names.clone());
                        deps = cons(dep_map.clone(), deps.clone());
                        reps = cons(rep_set.clone(), reps.clone());
                        solved_vars = cons(vars.clone(), solved_vars.clone());
                    } else {
                        if changed.clone() {
                            inner_deps = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut tpl in (local_deps.clone()).into_iter().cloned() {
            let __x = Util::tuple31(tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                            inner_deps = UnorderedSet::unique_list(inner_deps.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>));
                        } else {
                            inner_deps = UnorderedMap::keyList(var_field!((*full).dependencies, Matrix::FULL).borrow()[(eqn_index.clone()-1) as usize].clone());
                        }
                        for mut var in &*vars.clone() {
                            let mut var = var.clone();
                            UnorderedMap::add(BVariable::getVarName(var.clone()), inner_deps.clone(), inner_map.clone())?;
                        }
                    }
                }
            }
            Arc::new(Matrix::SPARSITY { equation_names: metamodelica::arrayFromVec(eqn_names.clone().reverse().into_iter().cloned().collect()), dependencies: metamodelica::arrayFromVec(deps.clone().reverse().into_iter().cloned().collect()), repetitions: metamodelica::arrayFromVec(reps.clone().reverse().into_iter().cloned().collect()), solved_variables: metamodelica::arrayFromVec(solved_vars.clone().reverse().into_iter().cloned().collect()) })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.fullToSparsity")); __mm_s.push_str(&*literal!(" failed because of wrong matrix type.\n            Expected: full, Got :")); __mm_s.push_str(&*strictnessString(getStrictness(full.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        Ok(sparse)
    }

    pub fn upgrade(mut adj: Arc<Matrix>, mut full: Arc<Matrix>, mut vars_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut eqns_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut eqns: Arc<EquationPointers::EquationPointers>, mut st: MatrixStrictness, mut iter: Arc<Iterator::Iterator>) -> Result<Arc<Matrix>> {
        let mut adj: Arc<Matrix> = adj;
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Upgrading from [")); __mm_s.push_str(&*strictnessString(getStrictness(adj.clone())?)); __mm_s.push_str(&*literal!("] to [")); __mm_s.push_str(&*strictnessString(st.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        adj = (::match_deref::match_deref! { match &(full.clone()) {
        Deref @ EMPTY { .. } => {
            Arc::new(Matrix::EMPTY { st: st.clone() })
        },
        Deref @ FULL { .. } => {
            let mut min: i32 = 0;
            let mut max: i32 = 0;
            if isEmpty(adj.clone()) {
                min = 0;
                adj = initialize(var_field!((*full).mapping, Matrix::FULL).clone(), st.clone())?;
            } else {
                min = Solvability::rank(Solvability::fromStrictness(getStrictness(adj.clone())?))?;
            }
            max = Solvability::rank(Solvability::fromStrictness(st.clone()))?;
            adj = (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ FINAL { .. } => {
            let mut result: Arc<Matrix>;
            let mut filtered: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut occ: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>;
            let mut dep: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>>;
            let mut sol: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>>;
            let mut rep: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>;
            if max.clone() == min.clone() {
                result = adj.clone();
            } else if max.clone() > min.clone() {
                (occ, dep, sol, rep) = (var_field!((*full).occurrences, Matrix::FULL).clone(), var_field!((*full).dependencies, Matrix::FULL).clone(), var_field!((*full).solvabilities, Matrix::FULL).clone(), var_field!((*full).repetitions, Matrix::FULL).clone());
                for mut index in &*UnorderedMap::valueList(eqns_map.clone()) {
                    let mut index = index.clone();
                    filtered = Solvability::filter(UnorderedSet::toList(occ.borrow()[(index.clone()-1) as usize].clone()), sol.borrow()[(index.clone()-1) as usize].clone(), vars_map.clone(), min.clone(), max.clone())?;
                    upgradeRow(BEquation::EquationPointers::getEqnAt(eqns.clone(), index.clone())?, index.clone(), filtered.clone(), dep.borrow()[(index.clone()-1) as usize].clone(), rep.borrow()[(index.clone()-1) as usize].clone(), vars_map.clone(), vars_map.clone(), var_field!((*adj).m, Matrix::FINAL).clone(), var_field!((*adj).mapping, Matrix::FINAL).clone(), var_field!((*adj).modes, Matrix::FINAL).clone(), iter.clone())?;
                }
                assign_variant_field!(adj => Matrix::FINAL; mT = transposeScalar(var_field!((*adj).m, Matrix::FINAL).clone(), (var_field!((*adj).mapping, Matrix::FINAL).var_StA.clone().borrow().len() as i32))?);
                result = adj.clone();
            } else {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Invalid matrix upgrade request. Cannot upgrade matrix of type ")); __mm_s.push_str(&*Solvability::toString(Solvability::fromStrictness(getStrictness(adj.clone())?))?); __mm_s.push_str(&*literal!(" to type ")); __mm_s.push_str(&*Solvability::toString(Solvability::fromStrictness(st.clone()))?); __mm_s.push_str(&*literal!(". The new matrix will be\n                    created from using only the full adjacency matrix.")); ArcStr::from(__mm_s) }).clone())?;
                }
                result = fullToFinal(full.clone(), vars_map.clone(), eqns_map.clone(), eqns.clone(), st.clone(), iter.clone());
            }
            result.clone()
        },
        Deref @ EMPTY { .. } => {
            adj.clone()
        },
        Deref @ FULL { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.upgrade")); __mm_s.push_str(&*literal!(" failed because of wrong matrix type for the 1st input.\n                Expected: final or empty, Got :")); __mm_s.push_str(&*strictnessString(getStrictness(adj.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
            adj.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.upgrade")); __mm_s.push_str(&*literal!(" failed because of wrong matrix type for the 2nd input.\n            Expected: full, Got :")); __mm_s.push_str(&*strictnessString(getStrictness(full.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(adj.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok(adj)
    }

    pub fn expand(mut adj: Arc<Matrix>, mut full: Arc<Matrix>, mut vo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut vn: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut eo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut en: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>, mut kind: Partition::Kind) -> Result<(Arc<Matrix>, Arc<Matrix>)> {
        let mut adj: Arc<Matrix> = adj;
        let mut full: Arc<Matrix> = full;
        let mut size_vo: i32 = 0;
        let mut size_vn: i32 = 0;
        let mut size_eo: i32 = 0;
        let mut size_en: i32 = 0;
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            size_vo = {
        let mut __acc: i32 = 0;
        for mut var in (UnorderedMap::keyList(vo.clone())).into_iter().cloned() {
            let __x = ComponentRef::size(var.clone(), true, false);
            __acc += __x;
        }
        __acc
    };
            size_vn = {
        let mut __acc: i32 = 0;
        for mut var in (UnorderedMap::keyList(vn.clone())).into_iter().cloned() {
            let __x = ComponentRef::size(var.clone(), true, false);
            __acc += __x;
        }
        __acc
    } + size_vo.clone();
            size_eo = {
        let mut __acc: i32 = 0;
        for mut eqn in (UnorderedMap::keyList(eo.clone())).into_iter().cloned() {
            let __x = ComponentRef::size(eqn.clone(), true, false);
            __acc += __x;
        }
        __acc
    };
            size_en = {
        let mut __acc: i32 = 0;
        for mut eqn in (UnorderedMap::keyList(en.clone())).into_iter().cloned() {
            let __x = ComponentRef::size(eqn.clone(), true, false);
            __acc += __x;
        }
        __acc
    } + size_eo.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expanding from size [vars: ")); __mm_s.push_str(&*intString(size_vo.clone())); __mm_s.push_str(&*literal!("| eqns: ")); __mm_s.push_str(&*intString(size_eo.clone())); __mm_s.push_str(&*literal!("] to [vars: ")); __mm_s.push_str(&*intString(size_vn.clone())); __mm_s.push_str(&*literal!("| eqns: ")); __mm_s.push_str(&*intString(size_en.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        full = (::match_deref::match_deref! { match &(full.clone()) {
        Deref @ FULL { .. } if (BEquation::EquationPointers::size(eqns.clone()) > (var_field!((*full).equation_names, Matrix::FULL).clone().borrow().len() as i32)) => expandFull(full.clone(), vo.clone(), vn.clone(), eo.clone(), en.clone(), vars.clone(), eqns.clone(), kind.clone())?,
        _ => full.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        adj = ({
        let mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = vo.clone();
        (::match_deref::match_deref! { match &((adj.clone(), full.clone())) {
        (Deref @ EMPTY { .. }, Deref @ FULL { .. }) => {
            let mut new: Arc<Matrix>;
            new = initialize(var_field!((*full).mapping, Matrix::FULL).clone(), var_field!((*adj).st, Matrix::EMPTY).clone())?;
            if !(isEmpty(new.clone())) {
                (new, _) = expand(new.clone(), full.clone(), vo.clone(), vn.clone(), eo.clone(), en.clone(), vars.clone(), eqns.clone(), kind.clone())?;
            }
            new.clone()
        },
        (Deref @ FINAL { .. }, Deref @ FULL { .. }) => {
            let mut rank: i32 = 0;
            let mut filtered: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            assign_variant_field!(adj => Matrix::FINAL;
                m = expandMatrix(var_field!((*adj).m, Matrix::FINAL).clone(), BEquation::EquationPointers::scalarSize(eqns.clone(), true)? - (var_field!((*adj).m, Matrix::FINAL).clone().borrow().len() as i32))?,
                mapping = var_field!((*full).mapping, Matrix::FULL).clone()
            );
            rank = Solvability::rank(Solvability::fromStrictness(getStrictness(adj.clone())?))?;
            if !(UnorderedMap::isEmpty(vn.clone())) && !(UnorderedMap::isEmpty(en.clone())) {
                v = UnorderedMap::merge(v.clone(), vn.clone(), metamodelica::sourceInfo!())?;
            }
            if !(UnorderedMap::isEmpty(vn.clone())) {
                for mut e in &*UnorderedMap::valueList(eo.clone()) {
                    let mut e = e.clone();
                    filtered = Solvability::filter(UnorderedSet::toList(var_field!((*full).occurrences, Matrix::FULL).borrow()[(e.clone()-1) as usize].clone()), var_field!((*full).solvabilities, Matrix::FULL).borrow()[(e.clone()-1) as usize].clone(), vn.clone(), 0, rank.clone())?;
                    upgradeRow(BEquation::EquationPointers::getEqnAt(eqns.clone(), e.clone())?, e.clone(), filtered.clone(), var_field!((*full).dependencies, Matrix::FULL).borrow()[(e.clone()-1) as usize].clone(), var_field!((*full).repetitions, Matrix::FULL).borrow()[(e.clone()-1) as usize].clone(), vn.clone(), vars.map.clone(), var_field!((*adj).m, Matrix::FINAL).clone(), var_field!((*adj).mapping, Matrix::FINAL).clone(), var_field!((*adj).modes, Matrix::FINAL).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY))?;
                }
            }
            if !(UnorderedMap::isEmpty(en.clone())) {
                for mut e in &*UnorderedMap::valueList(en.clone()) {
                    let mut e = e.clone();
                    filtered = Solvability::filter(UnorderedSet::toList(var_field!((*full).occurrences, Matrix::FULL).borrow()[(e.clone()-1) as usize].clone()), var_field!((*full).solvabilities, Matrix::FULL).borrow()[(e.clone()-1) as usize].clone(), v.clone(), 0, rank.clone())?;
                    upgradeRow(BEquation::EquationPointers::getEqnAt(eqns.clone(), e.clone())?, e.clone(), filtered.clone(), var_field!((*full).dependencies, Matrix::FULL).borrow()[(e.clone()-1) as usize].clone(), var_field!((*full).repetitions, Matrix::FULL).borrow()[(e.clone()-1) as usize].clone(), v.clone(), vars.map.clone(), var_field!((*adj).m, Matrix::FINAL).clone(), var_field!((*adj).mapping, Matrix::FINAL).clone(), var_field!((*adj).modes, Matrix::FINAL).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY))?;
                }
            }
            if UnorderedMap::isEmpty(vo.clone()) && UnorderedMap::isEmpty(vn.clone()) {
            } else {
                let _ = intMax({
        let mut __acc: Option<i32> = None;
        for mut i in (UnorderedMap::valueList(vo.clone())).into_iter().cloned() {
            let __x = i.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    }, {
        let mut __acc: Option<i32> = None;
        for mut i in (UnorderedMap::valueList(vn.clone())).into_iter().cloned() {
            let __x = i.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    });
            }
            assign_variant_field!(adj => Matrix::FINAL; mT = transposeScalar(var_field!((*adj).m, Matrix::FINAL).clone(), BVariable::VariablePointers::scalarSize(vars.clone(), true))?);
            adj.clone()
        },
        (Deref @ FINAL { .. }, _) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.expand")); __mm_s.push_str(&*literal!(" failed because the full matrix expected to contain all information is instead of type ")); __mm_s.push_str(&*strictnessString(getStrictness(full.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        (_, Deref @ FULL { .. }) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.expand")); __mm_s.push_str(&*literal!(" failed because the matrix to be expanded of type ")); __mm_s.push_str(&*strictnessString(getStrictness(adj.clone())?)); __mm_s.push_str(&*literal!(" should be of type final.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.expand")); __mm_s.push_str(&*literal!(" expected types final and full, got types ")); __mm_s.push_str(&*strictnessString(getStrictness(adj.clone())?)); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*strictnessString(getStrictness(full.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(adj.clone(), (literal!("Expanded ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok((adj, full))
    }

    pub fn expandFull(mut full: Arc<Matrix>, mut vo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut vn: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut eo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut en: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>, mut kind: Partition::Kind) -> Result<Arc<Matrix>> {
        let mut full: Arc<Matrix> = full;
        full = ({
        let mut new_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut idx in (UnorderedMap::valueList(vn.clone())).into_iter().cloned() {
            let __x = BVariable::VariablePointers::getVarAt(vars.clone(), idx.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        let mut new_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut idx in (UnorderedMap::valueList(en.clone())).into_iter().cloned() {
            let __x = BEquation::EquationPointers::getEqnAt(eqns.clone(), idx.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        let mut size: i32 = BEquation::EquationPointers::size(eqns.clone());
        (::match_deref::match_deref! { match &(full.clone()) {
        Deref @ FULL { .. } => {
            let mut index: i32 = 0;
            let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut occ_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            full = Arc::new(Matrix::FULL { equation_names: Array::expandToSize(size.clone(), var_field!((*full).equation_names, Matrix::FULL).clone(), Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY))?, occurrences: Array::expandToSize(size.clone(), var_field!((*full).occurrences, Matrix::FULL).clone(), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13))?, dependencies: Array::expandToSize(size.clone(), var_field!((*full).dependencies, Matrix::FULL).clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?, solvabilities: Array::expandToSize(size.clone(), var_field!((*full).solvabilities, Matrix::FULL).clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?, repetitions: Array::expandToSize(size.clone(), var_field!((*full).repetitions, Matrix::FULL).clone(), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13))?, mapping: Mapping::expand(var_field!((*full).mapping, Matrix::FULL).clone(), new_eqns.clone(), new_vars.clone())? });
            if !(UnorderedMap::isEmpty(vn.clone())) {
                for mut e in &*UnorderedMap::valueList(eo.clone()) {
                    let mut e = e.clone();
                    eqn_ptr = BEquation::EquationPointers::getEqnAt(eqns.clone(), e.clone())?;
                    index = UnorderedMap::getSafe(BEquation::Equation::getEqnName(eqn_ptr.clone())?, eqns.map.clone(), metamodelica::sourceInfo!())?;
                    occ_set = collectDependenciesEquation(Pointer::access(eqn_ptr.clone()), kind.clone(), vn.clone(), var_field!((*full).dependencies, Matrix::FULL).borrow()[(index.clone()-1) as usize].clone(), var_field!((*full).solvabilities, Matrix::FULL).borrow()[(index.clone()-1) as usize].clone(), var_field!((*full).repetitions, Matrix::FULL).borrow()[(index.clone()-1) as usize].clone())?;
                    {
                        let __cell0 = UnorderedSet::union(var_field!((*full).occurrences, Matrix::FULL).borrow()[(index.clone()-1) as usize].clone(), occ_set.clone())?;
                        var_field!((*full).occurrences, Matrix::FULL).clone().borrow_mut()[(index.clone()-1) as usize] = __cell0;
                    }
                }
            }
            if !(UnorderedMap::isEmpty(en.clone())) {
                for mut e in &*UnorderedMap::valueList(en.clone()) {
                    let mut e = e.clone();
                    eqn_ptr = BEquation::EquationPointers::getEqnAt(eqns.clone(), e.clone())?;
                    index = UnorderedMap::getSafe(BEquation::Equation::getEqnName(eqn_ptr.clone())?, eqns.map.clone(), metamodelica::sourceInfo!())?;
                    occ_set = collectDependenciesEquation(Pointer::access(eqn_ptr.clone()), kind.clone(), vars.map.clone(), var_field!((*full).dependencies, Matrix::FULL).borrow()[(index.clone()-1) as usize].clone(), var_field!((*full).solvabilities, Matrix::FULL).borrow()[(index.clone()-1) as usize].clone(), var_field!((*full).repetitions, Matrix::FULL).borrow()[(index.clone()-1) as usize].clone())?;
                    {
                        let __cell1 = BEquation::Equation::getEqnName(eqn_ptr.clone())?;
                        var_field!((*full).equation_names, Matrix::FULL).clone().borrow_mut()[(index.clone()-1) as usize] = __cell1;
                    }
                    {
                        let __cell2 = occ_set.clone();
                        var_field!((*full).occurrences, Matrix::FULL).clone().borrow_mut()[(index.clone()-1) as usize] = __cell2;
                    }
                }
            }
            full.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.expandFull")); __mm_s.push_str(&*literal!(" expected type full, got type ")); __mm_s.push_str(&*strictnessString(getStrictness(full.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(full.clone(), (literal!("Expanded Full")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok(full)
    }

    pub fn refine(mut full: Arc<Matrix>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut e: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>, mut vars_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut init: bool) -> Result<Arc<Matrix>> {
        let mut full: Arc<Matrix> = full;
        full = ({
        let mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments> = Differentiate::DifferentiationArguments::default(Differentiate::DifferentiationType::SIMPLE.clone(), funcMap.clone());
        (::match_deref::match_deref! { match &(full.clone()) {
        Deref @ FULL { .. } => {
            let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut residual: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut status: Solve::Status = Solve::Status::UNPROCESSED;
            let mut sol: Arc<Solvability::Solvability> = Arc::new(Solvability::IMPLICIT);
            let mut linear_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut param_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut var_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut eqnIsDiscrete: bool = false;
            let mut eqnIsIf: bool = false;
            let __range0 = UnorderedMap::valueArray(e.clone()).borrow().iter().cloned().collect::<Vec<_>>();
            for mut eqn_idx in __range0 {
                eqn_ptr = BEquation::EquationPointers::getEqnAt(eqns.clone(), eqn_idx.clone())?;
                if BEquation::Equation::isAlgorithm(eqn_ptr.clone()) {
                    continue;
                }
                eqnIsDiscrete = BEquation::Equation::isDiscrete(eqn_ptr.clone()) || BEquation::Equation::isWhenEquation(eqn_ptr.clone());
                eqnIsIf = BEquation::Equation::isIfEquation(eqn_ptr.clone());
                if !(eqnIsDiscrete.clone() || eqnIsIf.clone()) {
                    residual = BEquation::Equation::getResidualExp(Pointer::access(eqn_ptr.clone()), true)?;
                }
                let __range1 = UnorderedSet::toArray(var_field!((*full).occurrences, Matrix::FULL).borrow()[(eqn_idx.clone()-1) as usize].clone()).borrow().iter().cloned().collect::<Vec<_>>();
                for mut var in __range1 {
                    if UnorderedMap::contains(var.clone(), v.clone()) {
                        sol = UnorderedMap::getSafe(var.clone(), var_field!((*full).solvabilities, Matrix::FULL).borrow()[(eqn_idx.clone()-1) as usize].clone(), metamodelica::sourceInfo!())?;
                        if Solvability::rank(sol.clone())? < Solvability::rank(Arc::new(crate::NBAdjacency::Solvability::IMPLICIT))? {
                            if eqnIsDiscrete.clone() || !(BVariable::checkCref(var.clone(), Arc::new({ let __pe_b1 = init.clone(); move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone()) }), metamodelica::sourceInfo!())) {
                                (_, status, _) = Solve::solveSimple(Pointer::access(eqn_ptr.clone()), var.clone())?;
                                sol = if (status.clone() == Solve::Status::EXPLICIT.clone()) {Arc::new(Solvability::Solvability::EXPLICIT_LINEAR { pars: None, vars: None })} else {Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE)};
                            } else if eqnIsIf.clone() {
                                sol = Arc::new(crate::NBAdjacency::Solvability::IMPLICIT);
                            } else {
                                assign_field!(diffArgs.diffCref = var.clone());
                                (exp, diffArgs) = Differentiate::differentiateExpressionDump(residual.clone(), diffArgs.clone(), (literal!("NBAdjacency.Matrix.refine")).clone(), (literal!("")).clone())?;
                                exp = SimplifyExp::simplifyDump(exp.clone(), true, (literal!("NBAdjacency.Matrix.refine")).clone(), (literal!("")).clone())?;
                                if Expression::isZero(exp.clone()) {
                                    sol = Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE);
                                } else if Expression::containsCrefSet(exp.clone(), vars_set.clone())? {
                                    sol = Arc::new(Solvability::Solvability::EXPLICIT_NONLINEAR { unique: Expression::containsCref(exp.clone(), var.clone())? });
                                } else {
                                    linear_set = Expression::extractCrefs(exp.clone());
                                    linear_set = UnorderedSet::filterOnFalse(linear_set.clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(BVariable::isConst, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); let __pe_b2 = metamodelica::sourceInfo!(); move |__pe_a0| Ok(BVariable::checkCref(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }))?;
                                    (param_set, var_set) = UnorderedSet::splitOnTrue(linear_set.clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(BVariable::isParamOrConst, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); let __pe_b2 = metamodelica::sourceInfo!(); move |__pe_a0| Ok(BVariable::checkCref(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }))?;
                                    sol = Arc::new(Solvability::Solvability::EXPLICIT_LINEAR { vars: if (UnorderedSet::isEmpty(var_set.clone())) {None} else {Some(var_set.clone())}, pars: if (UnorderedSet::isEmpty(param_set.clone())) {None} else {Some(param_set.clone())} });
                                }
                            }
                            UnorderedMap::add(var.clone(), sol.clone(), var_field!((*full).solvabilities, Matrix::FULL).borrow()[(eqn_idx.clone()-1) as usize].clone())?;
                        }
                    }
                }
            }
            full.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.refine")); __mm_s.push_str(&*literal!(" expected type full, got type ")); __mm_s.push_str(&*strictnessString(getStrictness(full.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(full.clone(), (literal!("Refined Full")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok(full)
    }

    pub fn compress(mut adj: Arc<Matrix>, mut full: Arc<Matrix>, mut eqns: Arc<EquationPointers::EquationPointers>, mut vars: Arc<VariablePointers::VariablePointers>, mut old_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<(Arc<Matrix>, Arc<Matrix>)> {
        let mut adj: Arc<Matrix> = adj;
        let mut full: Arc<Matrix> = full;
        let mut index_old: i32 = 0;
        let mut index_new: i32 = 0;
        let mut size: i32 = BEquation::EquationPointers::size(eqns.clone());
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut equation_names: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut occurrences: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>;
        let mut dependencies: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>>;
        let mut solvabilities: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>>;
        let mut repetitions: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>;
        let mut mapping: Arc<Mapping::Mapping>;
        let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut old_start: i32 = 0;
        let mut old_size: i32 = 0;
        let mut new_start: i32 = 0;
        let mut new_size: i32 = 0;
        (adj, full) = (::match_deref::match_deref! { match &((adj.clone(), full.clone())) {
        (Deref @ FINAL { .. }, Deref @ FULL { .. }) => {
            let mut new_adj: Arc<Matrix>;
            let mut new_full: Arc<Matrix>;
            mapping = Mapping::create(eqns.clone(), vars.clone())?;
            m = arrayCreate((mapping.eqn_StA.clone().borrow().len() as i32), metamodelica::nil());
            equation_names = arrayCreate(size.clone(), Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY));
            occurrences = arrayCreate(size.clone(), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13));
            dependencies = arrayCreate(size.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1));
            solvabilities = arrayCreate(size.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1));
            repetitions = arrayCreate(size.clone(), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13));
            for mut eqn_ptr in &*BEquation::EquationPointers::toList(eqns.clone())? {
                let mut eqn_ptr = eqn_ptr.clone();
                name = BEquation::Equation::getEqnName(eqn_ptr.clone())?;
                index_new = UnorderedMap::getSafe(name.clone(), eqns.map.clone(), metamodelica::sourceInfo!())?;
                index_old = UnorderedMap::getSafe(name.clone(), old_map.clone(), metamodelica::sourceInfo!())?;
                (old_start, old_size) = var_field!((*adj).mapping, Matrix::FINAL).eqn_AtS[(index_old.clone()-1) as usize].clone();
                (new_start, new_size) = mapping.eqn_AtS.borrow()[(index_new.clone()-1) as usize].clone();
                if old_size.clone() == new_size.clone() {
                    for mut i in 0..=old_size.clone() - 1 {
                        {
                            let __cell0 = var_field!((*adj).m, Matrix::FINAL).borrow()[(old_start.clone() + i.clone()-1) as usize].clone();
                            m.clone().borrow_mut()[(new_start.clone() + i.clone()-1) as usize] = __cell0;
                        }
                    }
                } else {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.compress")); __mm_s.push_str(&*literal!(" sizes (old: ")); __mm_s.push_str(&*intString(old_size.clone())); __mm_s.push_str(&*literal!(", new: ")); __mm_s.push_str(&*intString(new_size.clone())); __mm_s.push_str(&*literal!(" do not mach for equation:\n")); __mm_s.push_str(&*BEquation::Equation::pointerToString(eqn_ptr.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                }
                {
                    let __cell1 = name.clone();
                    equation_names.clone().borrow_mut()[(index_new.clone()-1) as usize] = __cell1;
                }
                {
                    let __cell2 = var_field!((*full).occurrences, Matrix::FULL).borrow()[(index_old.clone()-1) as usize].clone();
                    occurrences.clone().borrow_mut()[(index_new.clone()-1) as usize] = __cell2;
                }
                {
                    let __cell3 = var_field!((*full).dependencies, Matrix::FULL).borrow()[(index_old.clone()-1) as usize].clone();
                    dependencies.clone().borrow_mut()[(index_new.clone()-1) as usize] = __cell3;
                }
                {
                    let __cell4 = var_field!((*full).solvabilities, Matrix::FULL).borrow()[(index_old.clone()-1) as usize].clone();
                    solvabilities.clone().borrow_mut()[(index_new.clone()-1) as usize] = __cell4;
                }
                {
                    let __cell5 = var_field!((*full).repetitions, Matrix::FULL).borrow()[(index_old.clone()-1) as usize].clone();
                    repetitions.clone().borrow_mut()[(index_new.clone()-1) as usize] = __cell5;
                }
            }
            new_adj = Arc::new(Matrix::FINAL { m: m.clone(), mT: transposeScalar(m.clone(), BVariable::VariablePointers::scalarSize(vars.clone(), true))?, mapping: mapping.clone(), modes: var_field!((*adj).modes, Matrix::FINAL).clone(), st: var_field!((*adj).st, Matrix::FINAL).clone() });
            new_full = Arc::new(Matrix::FULL { equation_names: equation_names.clone(), occurrences: occurrences.clone(), dependencies: dependencies.clone(), solvabilities: solvabilities.clone(), repetitions: repetitions.clone(), mapping: mapping.clone() });
            (new_adj.clone(), new_full.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.compress")); __mm_s.push_str(&*literal!(" expected types final and full, got types ")); __mm_s.push_str(&*strictnessString(getStrictness(adj.clone())?)); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*strictnessString(getStrictness(full.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(adj.clone(), (literal!("Compressed Final")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(full.clone(), (literal!("Compressed Full")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok((adj, full))
    }

    pub fn toString(mut adj: Arc<Matrix>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = ((::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ FULL { .. } => {
            let mut types: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
            let mut names: metamodelica::Array<ArcStr>;
            let mut types_str: metamodelica::Array<ArcStr>;
            let mut complex_sizes: metamodelica::Array<ArcStr>;
            let mut length0: i32 = 0;
            let mut length1: i32 = 0;
            let mut length2: i32 = 0;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("FULL Adjacency Matrix")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            types = {
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut name in (var_field!((*adj).equation_names, Matrix::FULL).clone()).borrow().iter() {
            let __x = ComponentRef::getSubscriptedType(name.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            complex_sizes = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut ty in (types.clone()).into_iter().cloned() {
            let __x = Util::applyOptionOrDefault(Type::complexSize(ty.clone(), true)?, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("0")).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            types_str = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut ty in (types.clone()).into_iter().cloned() {
            let __x = dimsString(Type::arrayDims(ty.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            names = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut name in (var_field!((*adj).equation_names, Matrix::FULL).clone()).borrow().iter() {
            let __x = ComponentRef::toString(name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            length0 = {
        let mut __acc: Option<i32> = None;
        for mut sz in (complex_sizes.clone()).borrow().iter() {
            let __x = ((sz.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length1 = {
        let mut __acc: Option<i32> = None;
        for mut ty in (types_str.clone()).borrow().iter() {
            let __x = ((ty.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 1;
            length2 = {
        let mut __acc: Option<i32> = None;
        for mut name in (names.clone()).borrow().iter() {
            let __x = ((name.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 3;
            let __range0 = 1..=(names.clone().borrow().len() as i32);
            for mut i in __range0 {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*complex_sizes.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(" ")).clone(), length0.clone() - ((complex_sizes.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*literal!(" | ")); __mm_s.push_str(&*types_str.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length1.clone() - ((types_str.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*names.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length2.clone() - ((names.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(UnorderedSet::toList(var_field!((*adj).occurrences, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone()), Arc::new({ let __pe_b1 = var_field!((*adj).dependencies, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); let __pe_b2 = var_field!((*adj).solvabilities, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); let __pe_b3 = var_field!((*adj).repetitions, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); move |__pe_a0| fullString(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        Deref @ FINAL { .. } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("FINAL Adjacency Matrix")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            if (var_field!((*adj).m, Matrix::FINAL).clone().borrow().len() as i32) > 0 {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("Normal Adjacency Matrix (row = equation)")).clone())); ArcStr::from(__mm_s) }).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toStringSingle(var_field!((*adj).m, Matrix::FINAL).clone())?); ArcStr::from(__mm_s) }).clone();
            }
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            if (var_field!((*adj).mT, Matrix::FINAL).clone().borrow().len() as i32) > 0 {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("Transposed Adjacency Matrix (row = variable)")).clone())); ArcStr::from(__mm_s) }).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toStringSingle(var_field!((*adj).mT, Matrix::FINAL).clone())?); ArcStr::from(__mm_s) }).clone();
            }
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Mapping::toString(var_field!((*adj).mapping, Matrix::FINAL).clone())); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ SPARSITY { .. } => {
            let mut types: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
            let mut vars: metamodelica::Array<ArcStr>;
            let mut names: metamodelica::Array<ArcStr>;
            let mut types_str: metamodelica::Array<ArcStr>;
            let mut complex_sizes: metamodelica::Array<ArcStr>;
            let mut length0: i32 = 0;
            let mut length1: i32 = 0;
            let mut length2: i32 = 0;
            let mut length3: i32 = 0;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("SPARSITY Adjacency Matrix")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            types = {
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut name in (var_field!((*adj).equation_names, Matrix::SPARSITY).clone()).borrow().iter() {
            let __x = ComponentRef::getSubscriptedType(name.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            complex_sizes = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut ty in (types.clone()).into_iter().cloned() {
            let __x = Util::applyOptionOrDefault(Type::complexSize(ty.clone(), true)?, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("0")).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            types_str = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut ty in (types.clone()).into_iter().cloned() {
            let __x = dimsString(Type::arrayDims(ty.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            names = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut name in (var_field!((*adj).equation_names, Matrix::SPARSITY).clone()).borrow().iter() {
            let __x = ComponentRef::toString(name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            vars = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut var_list in (var_field!((*adj).solved_variables, Matrix::SPARSITY).clone()).borrow().iter() {
            let __x = List::toString(var_list.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            length0 = {
        let mut __acc: Option<i32> = None;
        for mut sz in (complex_sizes.clone()).borrow().iter() {
            let __x = ((sz.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length1 = {
        let mut __acc: Option<i32> = None;
        for mut ty in (types_str.clone()).borrow().iter() {
            let __x = ((ty.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 1;
            length2 = {
        let mut __acc: Option<i32> = None;
        for mut name in (names.clone()).borrow().iter() {
            let __x = ((name.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 3;
            length3 = {
        let mut __acc: Option<i32> = None;
        for mut var in (vars.clone()).borrow().iter() {
            let __x = ((var.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 3;
            let __range0 = 1..=(names.clone().borrow().len() as i32);
            for mut i in __range0 {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*complex_sizes.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(" ")).clone(), length0.clone() - ((complex_sizes.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*literal!(" | ")); __mm_s.push_str(&*types_str.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length1.clone() - ((types_str.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*names.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length2.clone() - ((names.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*vars.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length3.clone() - ((vars.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(UnorderedMap::keyList(var_field!((*adj).dependencies, Matrix::SPARSITY).borrow()[(i.clone()-1) as usize].clone()), Arc::new({ let __pe_b1 = var_field!((*adj).dependencies, Matrix::SPARSITY).borrow()[(i.clone()-1) as usize].clone(); let __pe_b2 = var_field!((*adj).repetitions, Matrix::SPARSITY).borrow()[(i.clone()-1) as usize].clone(); move |__pe_a0| sparseString(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        Deref @ EMPTY { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("EMPTY Adjacency Matrix")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.toString")); __mm_s.push_str(&*literal!(" failed because of unknown adjacency matrix type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn solvabilityString(mut adj: Arc<Matrix>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = (({
        let mut xx: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut ii: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut nm: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut np: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut lv: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut lp: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut lc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut qq: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ FULL { .. } => {
            let mut XX: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut II: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut NM: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut NP: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut LV: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut LP: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut LC: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut QQ: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut names: metamodelica::Array<ArcStr>;
            let mut types: metamodelica::Array<ArcStr>;
            let mut XX_: metamodelica::Array<ArcStr>;
            let mut II_: metamodelica::Array<ArcStr>;
            let mut NM_: metamodelica::Array<ArcStr>;
            let mut NP_: metamodelica::Array<ArcStr>;
            let mut LV_: metamodelica::Array<ArcStr>;
            let mut LP_: metamodelica::Array<ArcStr>;
            let mut LC_: metamodelica::Array<ArcStr>;
            let mut QQ_: metamodelica::Array<ArcStr>;
            let mut length1: i32 = 0;
            let mut length2: i32 = 0;
            let mut length_xx: i32 = 0;
            let mut length_ii: i32 = 0;
            let mut length_nm: i32 = 0;
            let mut length_np: i32 = 0;
            let mut length_lv: i32 = 0;
            let mut length_lp: i32 = 0;
            let mut length_lc: i32 = 0;
            let mut length_qq: i32 = 0;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" Solvability Adjacency Matrix")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            types = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut name in (var_field!((*adj).equation_names, Matrix::FULL).clone()).borrow().iter() {
            let __x = intString(Type::sizeOf(ComponentRef::getSubscriptedType(name.clone(), false)?, true)?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            names = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut name in (var_field!((*adj).equation_names, Matrix::FULL).clone()).borrow().iter() {
            let __x = ComponentRef::toString(name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            let __range0 = (1..=(names.clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                (XX, II, NM, NP, LV, LP, LC, QQ) = Solvability::categorize(UnorderedSet::toList(var_field!((*adj).occurrences, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone()), var_field!((*adj).solvabilities, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone())?;
                xx = cons((List::toString(XX.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("XX ")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), xx.clone());
                ii = cons((List::toString(II.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("II ")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), ii.clone());
                nm = cons((List::toString(NM.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("N- ")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), nm.clone());
                np = cons((List::toString(NP.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("N+ ")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), np.clone());
                lv = cons((List::toString(LV.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("LV ")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), lv.clone());
                lp = cons((List::toString(LP.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("LP ")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), lp.clone());
                lc = cons((List::toString(LC.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("LC ")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), lc.clone());
                qq = cons((List::toString(QQ.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("|| ")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), qq.clone());
            }
            XX_ = metamodelica::arrayFromVec(xx.clone().into_iter().cloned().collect());
            II_ = metamodelica::arrayFromVec(ii.clone().into_iter().cloned().collect());
            NM_ = metamodelica::arrayFromVec(nm.clone().into_iter().cloned().collect());
            NP_ = metamodelica::arrayFromVec(np.clone().into_iter().cloned().collect());
            LV_ = metamodelica::arrayFromVec(lv.clone().into_iter().cloned().collect());
            LP_ = metamodelica::arrayFromVec(lp.clone().into_iter().cloned().collect());
            LC_ = metamodelica::arrayFromVec(lc.clone().into_iter().cloned().collect());
            QQ_ = metamodelica::arrayFromVec(qq.clone().into_iter().cloned().collect());
            length1 = {
        let mut __acc: Option<i32> = None;
        for mut ty in (types.clone()).borrow().iter() {
            let __x = ((ty.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 1;
            length2 = {
        let mut __acc: Option<i32> = None;
        for mut name in (names.clone()).borrow().iter() {
            let __x = ((name.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 3;
            length_xx = {
        let mut __acc: Option<i32> = None;
        for mut s in (XX_.clone()).borrow().iter() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length_ii = {
        let mut __acc: Option<i32> = None;
        for mut s in (II_.clone()).borrow().iter() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length_nm = {
        let mut __acc: Option<i32> = None;
        for mut s in (NM_.clone()).borrow().iter() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length_np = {
        let mut __acc: Option<i32> = None;
        for mut s in (NP_.clone()).borrow().iter() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length_lv = {
        let mut __acc: Option<i32> = None;
        for mut s in (LV_.clone()).borrow().iter() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length_lp = {
        let mut __acc: Option<i32> = None;
        for mut s in (LP_.clone()).borrow().iter() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length_lc = {
        let mut __acc: Option<i32> = None;
        for mut s in (LC_.clone()).borrow().iter() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            length_qq = {
        let mut __acc: Option<i32> = None;
        for mut s in (QQ_.clone()).borrow().iter() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            let __range1 = 1..=(names.clone().borrow().len() as i32);
            for mut i in __range1 {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*types.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length1.clone() - ((types.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*names.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length2.clone() - ((names.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*LC_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length_lc.clone() - ((LC_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*LP_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length_lp.clone() - ((LP_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*LV_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length_lv.clone() - ((LV_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*NP_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length_np.clone() - ((NP_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*NM_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length_nm.clone() - ((NM_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*II_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length_ii.clone() - ((II_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*XX_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length_xx.clone() - ((XX_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*QQ_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length_qq.clone() - ((QQ_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        _ => {
            toString(adj.clone(), (r#str.clone()).clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    })).clone();
        Ok(r#str)
    }

    pub fn dependencyString(mut adj: Arc<Matrix>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = (({
        let mut f: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut e: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut a: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut s: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut k: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ FULL { .. } => {
            let mut F: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut R: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut E: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut S: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut K: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut names: metamodelica::Array<ArcStr>;
            let mut types: metamodelica::Array<ArcStr>;
            let mut F_: metamodelica::Array<ArcStr>;
            let mut R_: metamodelica::Array<ArcStr>;
            let mut E_: metamodelica::Array<ArcStr>;
            let mut A_: metamodelica::Array<ArcStr>;
            let mut S_: metamodelica::Array<ArcStr>;
            let mut K_: metamodelica::Array<ArcStr>;
            let mut length1: i32 = 0;
            let mut length2: i32 = 0;
            let mut lengthf: i32 = 0;
            let mut lengthr: i32 = 0;
            let mut lengthe: i32 = 0;
            let mut lengtha: i32 = 0;
            let mut lengths: i32 = 0;
            let mut lengthk: i32 = 0;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" Dependency Adjacency Matrix")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            types = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut name in (var_field!((*adj).equation_names, Matrix::FULL).clone()).borrow().iter() {
            let __x = intString(Type::sizeOf(ComponentRef::getSubscriptedType(name.clone(), false)?, true)?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            names = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut name in (var_field!((*adj).equation_names, Matrix::FULL).clone()).borrow().iter() {
            let __x = ComponentRef::toString(name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            let __range0 = (1..=(names.clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                (F, R, E, A, S, K) = Dependency::categorize(UnorderedSet::toList(var_field!((*adj).occurrences, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone()), var_field!((*adj).dependencies, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(), var_field!((*adj).repetitions, Matrix::FULL).borrow()[(i.clone()-1) as usize].clone())?;
                f = cons((List::toString(F.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("[!]")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), f.clone());
                r = cons((List::toString(R.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("[-]")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), r.clone());
                e = cons((List::toString(E.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("[+]")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), e.clone());
                a = cons((List::toString(A.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("[:]")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), a.clone());
                s = cons((List::toString(S.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("[.]")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), s.clone());
                k = cons((List::toString(K.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("[o]")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), false, 0)?).clone(), k.clone());
            }
            F_ = metamodelica::arrayFromVec(f.clone().into_iter().cloned().collect());
            R_ = metamodelica::arrayFromVec(r.clone().into_iter().cloned().collect());
            E_ = metamodelica::arrayFromVec(e.clone().into_iter().cloned().collect());
            A_ = metamodelica::arrayFromVec(a.clone().into_iter().cloned().collect());
            S_ = metamodelica::arrayFromVec(s.clone().into_iter().cloned().collect());
            K_ = metamodelica::arrayFromVec(k.clone().into_iter().cloned().collect());
            length1 = {
        let mut __acc: Option<i32> = None;
        for mut ty in (types.clone()).borrow().iter() {
            let __x = ((ty.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 1;
            length2 = {
        let mut __acc: Option<i32> = None;
        for mut name in (names.clone()).borrow().iter() {
            let __x = ((name.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 3;
            lengthf = {
        let mut __acc: Option<i32> = None;
        for mut st in (F_.clone()).borrow().iter() {
            let __x = ((st.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            lengthr = {
        let mut __acc: Option<i32> = None;
        for mut st in (R_.clone()).borrow().iter() {
            let __x = ((st.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            lengthe = {
        let mut __acc: Option<i32> = None;
        for mut st in (E_.clone()).borrow().iter() {
            let __x = ((st.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            lengtha = {
        let mut __acc: Option<i32> = None;
        for mut st in (A_.clone()).borrow().iter() {
            let __x = ((st.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            lengths = {
        let mut __acc: Option<i32> = None;
        for mut st in (S_.clone()).borrow().iter() {
            let __x = ((st.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            lengthk = {
        let mut __acc: Option<i32> = None;
        for mut st in (K_.clone()).borrow().iter() {
            let __x = ((st.clone()).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
            let __range1 = 1..=(names.clone().borrow().len() as i32);
            for mut i in __range1 {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*types.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length1.clone() - ((types.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*names.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), length2.clone() - ((names.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*K_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), lengthk.clone() - ((K_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*S_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), lengths.clone() - ((S_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*A_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), lengtha.clone() - ((A_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*E_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), lengthe.clone() - ((E_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*R_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), lengthr.clone() - ((R_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*F_.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), lengthf.clone() - ((F_.clone().borrow()[(i.clone()-1) as usize].clone()).clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        _ => {
            toString(adj.clone(), (r#str.clone()).clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    })).clone();
        Ok(r#str)
    }

    pub fn getStrictness(mut adj: Arc<Matrix>) -> Result<MatrixStrictness> {
        let mut st: MatrixStrictness = MatrixStrictness::LINEAR;
        st = (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ FULL { .. } => MatrixStrictness::FULL.clone(),
        Deref @ FINAL { .. } => var_field!((*adj).st, Matrix::FINAL).clone(),
        Deref @ EMPTY { .. } => var_field!((*adj).st, Matrix::EMPTY).clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(st)
    }

    pub fn isEmpty(mut adj: Arc<Matrix>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn getMappingOpt(mut adj: Arc<Matrix>) -> Option<Arc<Mapping::Mapping>> {
        let mut mapping: Option<Arc<Mapping::Mapping>> = None;
        mapping = (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ FULL { .. } => Some(var_field!((*adj).mapping, Matrix::FULL).clone()),
        Deref @ FINAL { .. } => Some(var_field!((*adj).mapping, Matrix::FINAL).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        mapping
    }

    pub fn nonZeroCount(mut adj: Arc<Matrix>) -> Result<i32> {
        let mut count: i32 = 0;
        count = (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ FINAL { .. } => BackendUtil::countElem(var_field!((*adj).m, Matrix::FINAL).clone()),
        Deref @ EMPTY { .. } => 0,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.nonZeroCount")); __mm_s.push_str(&*literal!(" failed because of unknown matrix type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(count)
    }

    pub fn expandMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut shift: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
        let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = m;
        if shift.clone() > 0 {
            m = Array::expandToSize((m.clone().borrow().len() as i32) + shift.clone(), m.clone(), metamodelica::nil())?;
        }
        Ok(m)
    }

    pub fn transposeScalar(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut size: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
        let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        mT = arrayCreate(size.clone(), metamodelica::nil());
        let __range0 = 1..=(m.clone().borrow().len() as i32);
        for mut row in __range0 {
            let __range1 = &*m.borrow()[(row.clone()-1) as usize].clone();
            for mut idx in __range1 {
                let mut idx = idx.clone();
                if '__try2: {
                    if idx.clone() > 0 {
                        {
                            let __cell3 = cons(row.clone(), mT.borrow()[(idx.clone()-1) as usize].clone());
                            mT.clone().borrow_mut()[(idx.clone()-1) as usize] = __cell3;
                        }
                    } else {
                        {
                            let __cell4 = cons(-(row.clone()), mT.borrow()[(intAbs(idx.clone())-1) as usize].clone());
                            mT.clone().borrow_mut()[(intAbs(idx.clone())-1) as usize] = __cell4;
                        }
                    }
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.transposeScalar")); __mm_s.push_str(&*literal!(" failed for variable index ")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(".\n              The variables have to be dense (without empty spaces) for this to work!")); ArcStr::from(__mm_s) }).clone()])?;
                }
            }
        }
        let __range5 = 1..=(mT.clone().borrow().len() as i32);
        for mut row in __range5 {
            {
                let __cell6 = List::sort(mT.borrow()[(row.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                mT.clone().borrow_mut()[(row.clone()-1) as usize] = __cell6;
            }
        }
        Ok(mT)
    }

    pub fn toStringSingle(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut skip: i32 = ((intString((m.clone().borrow().len() as i32))).clone().len() as i32) + 1;
        let mut tmp: ArcStr = arcstr::literal!("");
        let __range0 = 1..=(m.clone().borrow().len() as i32);
        for mut row in __range0 {
            tmp = (intString(row.clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\t(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*StringUtil::repeat((literal!(" ")).clone(), skip.clone() - ((tmp.clone()).clone().len() as i32))); __mm_s.push_str(&*List::toString(m.borrow()[(row.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    fn fullString(mut cref: Arc<ComponentRef::NFComponentRef>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!("[")); ArcStr::from(__mm_s) };
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Solvability::toString(UnorderedMap::getSafe(cref.clone(), sol_map.clone(), metamodelica::sourceInfo!())?)?); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*Dependency::toString(UnorderedMap::getSafe(cref.clone(), dep_map.clone(), metamodelica::sourceInfo!())?)?); ArcStr::from(__mm_s) }).clone();
        if UnorderedSet::contains(cref.clone(), rep_set.clone())? {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("+")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    fn sparseString(mut cref: Arc<ComponentRef::NFComponentRef>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!("[")); ArcStr::from(__mm_s) };
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Dependency::toString(UnorderedMap::getSafe(cref.clone(), dep_map.clone(), metamodelica::sourceInfo!())?)?); ArcStr::from(__mm_s) }).clone();
        if UnorderedSet::contains(cref.clone(), rep_set.clone())? {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("+")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    fn dimsString(mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(dims.clone()) {
        Deref @ metamodelica::List::Nil => literal!("{1}"),
        _ => List::toString({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = Dimension::size(d.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    fn initialize(mut mapping: Arc<Mapping::Mapping>, mut st: MatrixStrictness) -> Result<Arc<Matrix>> {
        let mut adj: Arc<Matrix>;
        let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut eqn_scalar_size: i32 = 0;
        let mut var_scalar_size: i32 = 0;
        eqn_scalar_size = (mapping.eqn_StA.clone().borrow().len() as i32);
        var_scalar_size = (mapping.var_StA.clone().borrow().len() as i32);
        if eqn_scalar_size.clone() > 0 || var_scalar_size.clone() > 0 {
            m = arrayCreate(eqn_scalar_size.clone(), metamodelica::nil());
            mT = transposeScalar(m.clone(), var_scalar_size.clone())?;
            adj = Arc::new(Matrix::FINAL { m: m.clone(), mT: mT.clone(), mapping: mapping.clone(), modes: UnorderedMap::new((std::sync::Arc::new(fnptr!(Mode::keyHash, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(Mode::keyEqual, (i32, i32), (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), (i32, i32)) -> Result<bool> + 'static>), 1), st: st.clone() });
        } else {
            adj = Arc::new(Matrix::EMPTY { st: st.clone() });
        }
        Ok(adj)
    }

    fn upgradeRow(mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>, mut eqn_arr_idx: i32, mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut dep: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut rep: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut fullmap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapping: Arc<Mapping::Mapping>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>, mut iter_: Arc<Iterator::Iterator>) -> Result<()> {
        let mut eqn_scal_idx: i32 = 0;
        let mut eqn_size: i32 = 0;
        let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut eqn: Arc<Equation::Equation> = Pointer::access(eqn_ptr.clone());
        let mut iter: Arc<Iterator::Iterator> = BEquation::Equation::getForIterator(eqn.clone());
        let mut ty: Arc<Type::NFType> = BEquation::Equation::getType(eqn.clone(), true)?;
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
        if '__try0: {
            if BEquation::Equation::isAlgorithm(eqn_ptr.clone()) || BEquation::Equation::isIfEquation(eqn_ptr.clone()) {
                (eqn_scal_idx, eqn_size) = mapping.eqn_AtS.borrow()[(eqn_arr_idx.clone()-1) as usize].clone();
                row = unwrap_break_err!(Slice::upgradeRowFull(dependencies.clone(), map.clone(), mapping.clone()), '__try0);
                for mut i in 0..=eqn_size.clone() - 1 {
                    unwrap_break_err!(updateIntegerRow(m.clone(), eqn_scal_idx.clone() + i.clone(), row.clone()), '__try0);
                }
            } else {
                if !(BEquation::Iterator::isEmpty(iter_.clone())) {
                    (names, ranges, maps) = unwrap_break_err!(BEquation::Iterator::getFrames(iter_.clone()), '__try0);
                    iter = unwrap_break_err!(BEquation::Iterator::addFrames(iter.clone(), List::zip3(names.clone(), ranges.clone(), maps.clone())), '__try0);
                }
                unwrap_break_err!(Slice::upgradeRow(BEquation::Equation::getEqnName(eqn_ptr.clone())?, eqn_arr_idx.clone(), iter.clone(), ty.clone(), dependencies.clone(), dep.clone(), rep.clone(), map.clone(), fullmap.clone(), m.clone(), mapping.clone(), modes.clone()), '__try0);
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.upgradeRow")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*BEquation::Equation::pointerToString(eqn_ptr.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(())
    }

    fn updateIntegerRow(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut idx: i32, mut row: Arc<metamodelica::List<i32>>) -> Result<()> {
        {let _arr = m.clone(); let _val = listAppend(row.clone(), m.borrow()[(idx.clone()-1) as usize].clone()); _arr.borrow_mut()[(idx.clone()-1) as usize] = _val; _arr};
        Ok(())
    }

}

pub mod Dependency {
    use super::*;
    /// the dependency kind to show how a component reference occurs in an equation.
    ///    for each dimension there has to be one dependency kind.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Dependency {
        pub skips: metamodelica::Array<Arc<metamodelica::List<i32>>>,
        pub kinds: Arc<metamodelica::List<Kind>>,
    }

    impl Default for Dependency {
        fn default() -> Self {
            Self {
                skips: Default::default(),
                kinds: Default::default(),
            }
        }
    }

    pub type DEPENDENCY = Dependency;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    #[repr(i32)]
    pub enum Kind {
        REGULAR = 1,
        REDUCTION = 2,
    }
    impl PartialOrd for Kind {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl Ord for Kind {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
    }
    impl Default for Kind {
        fn default() -> Self { Self::REGULAR }
    }

    pub fn toString(mut dep: Arc<Dependency>) -> Result<ArcStr> {
        fn kindString(mut kind: Kind) -> ArcStr {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ((match kind.clone() {
        Kind::REGULAR => literal!(":"),
        _ => literal!("-"),
    })).clone();
            r#str
        }

        let mut r#str: ArcStr = arcstr::literal!("");
        let mut str1: ArcStr = arcstr::literal!("");
        let mut str2: ArcStr = arcstr::literal!("");
        str1 = (Array::toString(dep.skips.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> = (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>); let __pe_b2 = (literal!("")).clone(); let __pe_b3 = (literal!("{")).clone(); let __pe_b4 = (literal!(", ")).clone(); let __pe_b5 = (literal!("}")).clone(); let __pe_b6 = false; let __pe_b7 = 0; move |__pe_a0| List::toString(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), true, 0)?).clone();
        str2 = (List::toString(dep.kinds.clone(), (std::sync::Arc::new(fnptr!(kindString, Kind)) as std::sync::Arc<dyn ::std::ops::Fn(Kind) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), true, 0)?).clone();
        r#str = (if (str1.clone() == literal!("") || str2.clone() == literal!("")) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*str2.clone()); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*str2.clone()); ArcStr::from(__mm_s) }}).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn toBoolean(mut dep: Arc<Dependency>) -> Arc<metamodelica::List<bool>> {
        let mut b: Arc<metamodelica::List<bool>> = {
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for mut k in (dep.kinds.clone()).into_iter().cloned() {
            let __x = !(isReductionKind(k.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        b
    }

    pub fn create(mut sub_ty: Arc<Type::NFType>, mut depth: i32) -> Result<Arc<Dependency>> {
        let mut dep: Arc<Dependency> = Arc::new(<Dependency as ::std::default::Default>::default());
        dep = Arc::new(Dependency { skips: arrayCreate(depth.clone(), metamodelica::nil()), kinds: {
        let mut __acc: Arc<metamodelica::List<Kind>> = metamodelica::nil();
        for mut dim in (Type::arrayDims(sub_ty.clone())).into_iter().cloned() {
            if !(!(Dimension::isOne(dim.clone())?)) { continue; }
            let __x = Kind::REGULAR.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
        Ok(dep)
    }

    pub fn update(mut cref: Arc<ComponentRef::NFComponentRef>, mut num: i32, mut reverse: bool, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency>>>) -> Result<()> {
        fn makeNewKinds(mut kinds: Arc<metamodelica::List<Kind>>, mut num: i32) -> (Arc<metamodelica::List<Kind>>, i32) {
            let mut kinds: Arc<metamodelica::List<Kind>> = kinds;
            let mut num: i32 = num;
            (kinds, num) = (::match_deref::match_deref! { match &((kinds.clone(), num.clone())) {
        (_, 0) => {
            (kinds.clone(), num.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            (kinds, num) = makeNewKinds(rest.clone(), num.clone() - 1);
            (cons(Kind::REDUCTION.clone(), kinds.clone()), num.clone())
        },
        _ => {
            (kinds.clone(), num.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (kinds, num)
        }

        let mut opt_dep: Option<Arc<Dependency>> = UnorderedMap::get(cref.clone(), map.clone());
        let mut dep: Arc<Dependency> = Arc::new(<Dependency as ::std::default::Default>::default());
        let mut kinds: Arc<metamodelica::List<Kind>> = metamodelica::nil();
        let mut res: i32 = 0;
        if Util::isSome(opt_dep.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(opt_dep.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            dep = __pa0.clone();
            if reverse.clone() {
                (kinds, res) = makeNewKinds(dep.kinds.clone().reverse(), num.clone());
                assign_field!(dep.kinds = kinds.clone().reverse());
            } else {
                (kinds, res) = makeNewKinds(dep.kinds.clone(), num.clone());
                assign_field!(dep.kinds = kinds.clone());
            }
            if res.clone() > 0 {
                removeSkips(cref.clone(), map.clone(), res.clone(), reverse.clone())?;
            }
            UnorderedMap::add(cref.clone(), dep.clone(), map.clone())?;
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Dependency.update")); __mm_s.push_str(&*literal!(" failed because cref ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" was not found in the map.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(())
    }

    pub fn skip(mut cref: Arc<ComponentRef::NFComponentRef>, mut depth: i32, mut sk: i32, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency>>>) -> Result<()> {
        let mut opt_dep: Option<Arc<Dependency>> = UnorderedMap::get(cref.clone(), map.clone());
        let mut dep: Arc<Dependency> = Arc::new(<Dependency as ::std::default::Default>::default());
        if Util::isSome(opt_dep.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(opt_dep.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            dep = __pa0.clone();
            if (dep.skips.clone().borrow().len() as i32) >= depth.clone() {
                {let _arr = dep.skips.clone(); let _val = UnorderedSet::unique_list(cons(sk.clone(), dep.skips.borrow()[(depth.clone()-1) as usize].clone()), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)); _arr.borrow_mut()[(depth.clone()-1) as usize] = _val; _arr};
            } else {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Dependency.skip")); __mm_s.push_str(&*literal!(": Cref ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" was saved with depth ")); __mm_s.push_str(&*intString((dep.skips.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(" but depth ")); __mm_s.push_str(&*intString(depth.clone())); __mm_s.push_str(&*literal!(" was requested.")); ArcStr::from(__mm_s) }).clone())?;
                }
            }
            UnorderedMap::add(cref.clone(), dep.clone(), map.clone())?;
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Dependency.skip")); __mm_s.push_str(&*literal!(" failed because cref ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" was not found in the map.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(())
    }

    pub fn removeSkips(mut cref: Arc<ComponentRef::NFComponentRef>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency>>>, mut num: i32, mut reverse: bool) -> Result<()> {
        let mut opt_dep: Option<Arc<Dependency>> = UnorderedMap::get(cref.clone(), map.clone());
        let mut dep: Arc<Dependency> = Arc::new(<Dependency as ::std::default::Default>::default());
        let mut rest: i32 = num.clone();
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        if Util::isSome(opt_dep.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(opt_dep.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            dep = __pa0.clone();
            if num.clone() < 0 {
                let __range1 = 1..=(dep.skips.clone().borrow().len() as i32);
                for mut i in __range1 {
                    {let _arr = dep.skips.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
                }
            } else {
                i = if (reverse.clone()) {(dep.skips.clone().borrow().len() as i32)} else {1};
                while rest.clone() > 0 && i.clone() > 0 && i.clone() < (dep.skips.clone().borrow().len() as i32) + 1 {
                    len = (dep.skips.borrow()[(i.clone()-1) as usize].clone().len() as i32);
                    if len.clone() <= rest.clone() {
                        {let _arr = dep.skips.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
                    } else if len.clone() > 0 {
                        if reverse.clone() {
                            {let _arr = dep.skips.clone(); let _val = List::firstN(dep.skips.borrow()[(i.clone()-1) as usize].clone(), len.clone() - rest.clone())?; _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
                        } else {
                            {let _arr = dep.skips.clone(); let _val = List::lastN(dep.skips.borrow()[(i.clone()-1) as usize].clone(), len.clone() - rest.clone())?; _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
                        }
                    }
                    rest = rest.clone() - len.clone();
                    i = if (reverse.clone()) {i.clone() - 1} else {i.clone() + 1};
                }
            }
            UnorderedMap::add(cref.clone(), dep.clone(), map.clone())?;
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Dependency.removeSkips")); __mm_s.push_str(&*literal!(" failed because cref ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" was not found in the map.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(())
    }

    pub fn updateList(mut lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut num: i32, mut reverse: bool, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency>>>) -> Result<()> {
        for mut cref in &*lst.clone() {
            let mut cref = cref.clone();
            update(cref.clone(), num.clone(), reverse.clone(), map.clone())?;
        }
        Ok(())
    }

    pub fn skipList(mut lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut depth: i32, mut sk: i32, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency>>>) -> Result<()> {
        for mut cref in &*lst.clone() {
            let mut cref = cref.clone();
            skip(cref.clone(), depth.clone(), sk.clone(), map.clone())?;
        }
        Ok(())
    }

    pub fn removeSkipsList(mut lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency>>>) -> Result<()> {
        for mut cref in &*lst.clone() {
            let mut cref = cref.clone();
            removeSkips(cref.clone(), map.clone(), -1, false)?;
        }
        Ok(())
    }

    pub fn addListFull(mut lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut depth: i32, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency>>>, mut rep: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        let mut dep: Arc<Dependency> = Arc::new(<Dependency as ::std::default::Default>::default());
        for mut cref in &*lst.clone() {
            let mut cref = cref.clone();
            UnorderedMap::add(cref.clone(), create(ComponentRef::getSubscriptedType(cref.clone(), false)?, depth.clone())?, map.clone())?;
            UnorderedSet::add(cref.clone(), rep.clone())?;
        }
        updateList(lst.clone(), -1, false, map.clone())?;
        Ok(())
    }

    pub fn isReductionKind(mut kind: Kind) -> bool {
        let mut b: bool = kind.clone() == Kind::REDUCTION.clone();
        b
    }

    pub fn categorize(mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)> {
        let mut F: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut R: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut E: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut A: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut S: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut K: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut repeats: bool = false;
        for mut cref in &*crefs.clone() {
            let mut cref = cref.clone();
            repeats = UnorderedSet::contains(cref.clone(), rep_set.clone())?;
            let _ = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!())?) {
        Deref @ Dependency { skips, .. } if (!(Array::all(skips.clone(), std::sync::Arc::new(fnptr!(listEmpty, _))))) => {
            K = cons(cref.clone(), K.clone());
            ()
        },
        Deref @ Dependency { kinds: Deref @ metamodelica::List::Nil, .. } if (repeats.clone()) => {
            E = cons(cref.clone(), E.clone());
            ()
        },
        Deref @ Dependency { kinds: Deref @ metamodelica::List::Nil, .. } => {
            S = cons(cref.clone(), S.clone());
            ()
        },
        Deref @ Dependency { kinds, .. } => {
            if List::any(kinds.clone(), (std::sync::Arc::new(fnptr!(isReductionKind, Kind)) as std::sync::Arc<dyn ::std::ops::Fn(Kind) -> Result<bool> + 'static>)) {
                if repeats.clone() {
                    F = cons(cref.clone(), F.clone());
                } else {
                    R = cons(cref.clone(), R.clone());
                }
            } else {
                A = cons(cref.clone(), A.clone());
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok((F, R, E, A, S, K))
    }

}

pub mod Solvability {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Solvability {
        UNKNOWN,
        UNSOLVABLE,
        IMPLICIT,
        EXPLICIT_NONLINEAR {
            /// true if it has a unique solution when solved
            unique: bool,
        },
        EXPLICIT_LINEAR {
            /// parameters we need to divide by to solve
            pars: Option<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>,
            /// variables we need to divide by to solve
            vars: Option<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>,
        },
    }
    impl Default for Solvability {
        fn default() -> Self { Self::UNKNOWN }
    }
    pub use self::Solvability::{UNKNOWN,UNSOLVABLE,IMPLICIT,EXPLICIT_NONLINEAR,EXPLICIT_LINEAR};
    pub fn toString(mut sol: Arc<Solvability>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(sol.clone()) {
        Deref @ UNSOLVABLE { .. } => literal!("XX"),
        Deref @ IMPLICIT { .. } => literal!("II"),
        Deref @ EXPLICIT_NONLINEAR { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("N")); __mm_s.push_str(&*if (var_field!((*sol).unique, Solvability::EXPLICIT_NONLINEAR).clone()) {literal!("+")} else {literal!("-")}); ArcStr::from(__mm_s) },
        Deref @ EXPLICIT_LINEAR { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("L")); __mm_s.push_str(&*if (Util::isSome(var_field!((*sol).vars, Solvability::EXPLICIT_LINEAR).clone())) {literal!("V")} else if (Util::isSome(var_field!((*sol).pars, Solvability::EXPLICIT_LINEAR).clone())) {literal!("P")} else {literal!("C")}); ArcStr::from(__mm_s) },
        Deref @ UNKNOWN { .. } => literal!("||"),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Solvability.toString")); __mm_s.push_str(&*literal!(" failed because of unknown solvability kind.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn rank(mut sol: Arc<Solvability>) -> Result<i32> {
        let mut r: i32 = 0;
        r = (::match_deref::match_deref! { match &(sol.clone()) {
        Deref @ UNSOLVABLE { .. } => 7,
        Deref @ IMPLICIT { .. } => 6,
        Deref @ EXPLICIT_NONLINEAR { unique: false } => 5,
        Deref @ EXPLICIT_NONLINEAR { .. } => 4,
        Deref @ EXPLICIT_LINEAR { vars: Some(_), .. } => 3,
        Deref @ EXPLICIT_LINEAR { pars: Some(_), .. } => 2,
        Deref @ EXPLICIT_LINEAR { .. } => 1,
        Deref @ UNKNOWN { .. } => 0,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Solvability.rank")); __mm_s.push_str(&*literal!(" failed because of unknown solvability kind.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(r)
    }

    pub fn update(mut cref: Arc<ComponentRef::NFComponentRef>, mut sol: Arc<Solvability>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability>>>) -> Result<()> {
        if rank(sol.clone())? > rank(Util::getOptionOrDefault(UnorderedMap::get(cref.clone(), map.clone()), Arc::new(crate::NBAdjacency::Solvability::UNKNOWN)))? {
            UnorderedMap::add(cref.clone(), sol.clone(), map.clone())?;
        }
        Ok(())
    }

    pub fn updateList(mut lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut sol: Arc<Solvability>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability>>>) -> Result<()> {
        for mut cref in &*lst.clone() {
            let mut cref = cref.clone();
            update(cref.clone(), sol.clone(), map.clone())?;
        }
        Ok(())
    }

    pub fn categorize(mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability>>>) -> Result<(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)> {
        let mut XX: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut II: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut NM: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut NP: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut LV: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut LP: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut LC: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut QQ: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut cref in &*crefs.clone() {
            let mut cref = cref.clone();
            let _ = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!())?) {
        Deref @ UNSOLVABLE { .. } => {
            XX = cons(cref.clone(), XX.clone());
            ()
        },
        Deref @ IMPLICIT { .. } => {
            II = cons(cref.clone(), II.clone());
            ()
        },
        Deref @ EXPLICIT_NONLINEAR { unique: false } => {
            NM = cons(cref.clone(), NM.clone());
            ()
        },
        Deref @ EXPLICIT_NONLINEAR { .. } => {
            NP = cons(cref.clone(), NP.clone());
            ()
        },
        Deref @ EXPLICIT_LINEAR { vars: Some(_), .. } => {
            LV = cons(cref.clone(), LV.clone());
            ()
        },
        Deref @ EXPLICIT_LINEAR { pars: Some(_), .. } => {
            LP = cons(cref.clone(), LP.clone());
            ()
        },
        Deref @ EXPLICIT_LINEAR { .. } => {
            LC = cons(cref.clone(), LC.clone());
            ()
        },
        _ => {
            QQ = cons(cref.clone(), QQ.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok((XX, II, NM, NP, LV, LP, LC, QQ))
    }

    pub fn filter(mut all_occ: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability>>>, mut rel: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut min: i32, mut max: i32) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
        let mut occ: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut r: i32 = 0;
        for mut cref in &*all_occ.clone() {
            let mut cref = cref.clone();
            if UnorderedMap::contains(cref.clone(), rel.clone()) {
                r = rank(UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!())?)?;
                if r.clone() >= min.clone() && r.clone() <= max.clone() {
                    occ = cons(cref.clone(), occ.clone());
                }
            }
        }
        Ok(occ)
    }

    pub fn fromStrictness(mut st: MatrixStrictness) -> Arc<Solvability> {
        let mut sol: Arc<Solvability> = Arc::new(Solvability::IMPLICIT);
        sol = (match st.clone() {
        MatrixStrictness::LINEAR { .. } => Arc::new(Solvability::EXPLICIT_LINEAR { pars: None, vars: Some(UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)) }),
        MatrixStrictness::MATCHING { .. } => Arc::new(crate::NBAdjacency::Solvability::IMPLICIT),
        MatrixStrictness::SORTING => Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE),
        _ => Arc::new(crate::NBAdjacency::Solvability::UNKNOWN),
    });
        sol
    }

    pub fn isNonlinearOrImplicit(mut sol: Arc<Solvability>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(sol.clone()) {
        Deref @ EXPLICIT_NONLINEAR { .. } => true,
        Deref @ IMPLICIT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

}

pub fn collectDependenciesEquation(mut eqn: Arc<Equation::Equation>, mut kind: Partition::Kind, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut occurrences: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut inputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut outputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    occurrences = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { .. } => {
            let mut occ1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut occ2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            occ1 = collectDependencies(var_field!((*eqn).lhs, Equation::Equation::SCALAR_EQUATION).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            occ2 = collectDependencies(var_field!((*eqn).rhs, Equation::Equation::SCALAR_EQUATION).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            UnorderedSet::union(occ1.clone(), occ2.clone())?
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { .. } => {
            let mut occ1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut occ2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            occ1 = collectDependencies(var_field!((*eqn).lhs, Equation::Equation::ARRAY_EQUATION).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            occ2 = collectDependencies(var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            UnorderedSet::union(occ1.clone(), occ2.clone())?
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { .. } => {
            let mut occ1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut occ2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            occ1 = collectDependencies(var_field!((*eqn).lhs, Equation::Equation::RECORD_EQUATION).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            occ2 = collectDependencies(var_field!((*eqn).rhs, Equation::Equation::RECORD_EQUATION).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            UnorderedSet::union(occ1.clone(), occ2.clone())?
        },
        Deref @ BEquation::Equation::ALGORITHM { .. } => {
            inputs = collectDependenciesAlgorithmInputs(var_field!((*eqn).alg, Equation::Equation::ALGORITHM).statements.clone(), var_field!((*eqn).alg, Equation::Equation::ALGORITHM).inputs.clone())?;
            inputs = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut c in (inputs.clone()).into_iter().cloned() {
            let __x = collectDependenciesCref(c.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outputs = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut c in (var_field!((*eqn).alg, Equation::Equation::ALGORITHM).outputs.clone()).into_iter().cloned() {
            let __x = collectDependenciesCref(c.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Dependency::addListFull(inputs.clone(), 0, dep_map.clone(), rep_set.clone())?;
            Dependency::addListFull(outputs.clone(), 0, dep_map.clone(), rep_set.clone())?;
            Solvability::updateList(inputs.clone(), Arc::new(crate::NBAdjacency::Solvability::IMPLICIT), sol_map.clone())?;
            Solvability::updateList(outputs.clone(), Arc::new(Solvability::Solvability::EXPLICIT_LINEAR { pars: None, vars: None }), sol_map.clone())?;
            UnorderedSet::fromList(listAppend(inputs.clone(), outputs.clone()), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut occ1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut occ2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut filter: Slice::filterCref;
            occ1 = collectDependenciesEquation(body.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            occ2 = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            filter = Arc::new({ let __pe_b2 = map.clone(); let __pe_b3 = true; move |__pe_a0, __pe_a1| Slice::getDependentCref(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) });
            let _ = BEquation::Iterator::map(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(filter, Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> + 'static>); let __pe_b2 = occ2.clone(); move |__pe_a0| Slice::Slice::filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), Some(todo!("PARTEVALFUNCTION of filter: named args do not match any formal: ["acc"]")), (std::sync::Arc::new(Expression::mapShallow) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            Solvability::updateList(UnorderedSet::toList(occ2.clone()), Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE), sol_map.clone())?;
            UnorderedSet::union(occ1.clone(), occ2.clone())?
        },
        Deref @ BEquation::Equation::IF_EQUATION { .. } => {
            collectDependenciesIf(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            collectDependenciesWhen(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        _ => {
            UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(occurrences)
}

pub fn collectDependencies(mut exp: Arc<Expression::NFExpression>, mut depth: i32, mut kind: Partition::Kind, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    set = ({
        let mut sets: Arc<metamodelica::List<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            UnorderedSet::fromList(collectDependenciesCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), depth.clone(), map.clone(), dep_map.clone(), sol_map.clone())?, (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?
        },
        Deref @ Expression::ARRAY { literal: false, .. } => {
            let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let __range0 = 1..=(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32);
            for mut i in __range0 {
                set1 = collectDependencies(var_field!((*exp).elements, Expression::NFExpression::ARRAY).borrow()[(i.clone()-1) as usize].clone(), depth.clone() + 1, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                Dependency::skipList(UnorderedSet::toList(set1.clone()), depth.clone() + 1, i.clone(), dep_map.clone())?;
                sets = cons(set1.clone(), sets.clone());
            }
            UnorderedSet::union_list(sets.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?
        },
        Deref @ Expression::TUPLE { .. } => {
            let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut ind: i32 = 0;
            ind = 1;
            for mut elem in &*var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut elem = elem.clone();
                set1 = collectDependencies(elem.clone(), depth.clone() + 1, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                Dependency::skipList(UnorderedSet::toList(set1.clone()), depth.clone() + 1, ind.clone(), dep_map.clone())?;
                sets = cons(set1.clone(), sets.clone());
                ind = ind.clone() + 1;
            }
            set = UnorderedSet::union_list(sets.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            set.clone()
        },
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            set = collectDependencies(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            Dependency::updateList(UnorderedSet::toList(set.clone()), (var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone().len() as i32), true, dep_map.clone())?;
            Dependency::removeSkipsList(UnorderedSet::toList(set.clone()), dep_map.clone())?;
            set.clone()
        },
        Deref @ Expression::TUPLE_ELEMENT { .. } => {
            collectDependencies(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::RECORD_ELEMENT { .. } => {
            collectDependencies(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::BINARY { .. } => {
            let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut repeatLeft: bool = false;
            let mut repeatRight: bool = false;
            let mut reduce: bool = false;
            set2 = collectDependencies(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            set1 = collectDependencies(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            (repeatLeft, repeatRight) = Operator::repetition(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone());
            if repeatLeft.clone() {
                addRepetitions(set1.clone(), rep_set.clone())?;
            }
            if repeatRight.clone() {
                addRepetitions(set2.clone(), rep_set.clone())?;
            }
            reduce = Operator::reduction(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone());
            if reduce.clone() {
                Dependency::updateList(UnorderedSet::toList(set1.clone()), 1, true, dep_map.clone())?;
                Dependency::updateList(UnorderedSet::toList(set2.clone()), 1, false, dep_map.clone())?;
            }
            UnorderedSet::union(set1.clone(), set2.clone())?
        },
        Deref @ Expression::MULTARY { .. } => {
            let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut repeatLeft: bool = false;
            let mut repeatRight: bool = false;
            (repeatLeft, repeatRight) = Operator::repetition(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone());
            repeatLeft = repeatLeft.clone() || repeatRight.clone();
            for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                set1 = collectDependencies(arg.clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                addRepetitionsCond(set1.clone(), arg.clone(), repeatLeft.clone(), rep_set.clone())?;
                sets = cons(set1.clone(), sets.clone());
            }
            for mut arg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                set2 = collectDependencies(arg.clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                addRepetitionsCond(set2.clone(), arg.clone(), repeatLeft.clone(), rep_set.clone())?;
                sets = cons(set2.clone(), sets.clone());
            }
            set = UnorderedSet::union_list(sets.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            set.clone()
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            set1 = collectDependencies(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            set2 = collectDependencies(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            set = UnorderedSet::union(set1.clone(), set2.clone())?;
            Solvability::updateList(UnorderedSet::toList(set.clone()), Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE), sol_map.clone())?;
            set.clone()
        },
        Deref @ Expression::RELATION { .. } => {
            let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            set1 = collectDependencies(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            set2 = collectDependencies(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            set = UnorderedSet::union(set1.clone(), set2.clone())?;
            Solvability::updateList(UnorderedSet::toList(set.clone()), Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE), sol_map.clone())?;
            set.clone()
        },
        Deref @ Expression::CAST { .. } => {
            collectDependencies(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::BOX { .. } => {
            collectDependencies(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::UNBOX { .. } => {
            collectDependencies(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::UNARY { .. } => {
            collectDependencies(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::LUNARY { .. } => {
            collectDependencies(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::MUTABLE { .. } => {
            collectDependencies(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone()), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::SIZE { .. } => {
            let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            set = collectDependencies(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            if Util::isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                set2 = collectDependencies(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?, depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                set = UnorderedSet::union(set.clone(), set2.clone())?;
            }
            Solvability::updateList(UnorderedSet::toList(set.clone()), Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE), sol_map.clone())?;
            set.clone()
        },
        Deref @ Expression::IF { .. } => {
            let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut diff: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            if isInitialException(var_field!((*exp).condition, Expression::NFExpression::IF).clone()) {
                set = collectDependencies(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            } else {
                set1 = collectDependencies(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                set2 = collectDependencies(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                diff = UnorderedSet::sym_difference(set1.clone(), set2.clone())?;
                Solvability::updateList(UnorderedSet::toList(diff.clone()), Arc::new(crate::NBAdjacency::Solvability::IMPLICIT), sol_map.clone())?;
                set = collectDependencies(var_field!((*exp).condition, Expression::NFExpression::IF).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                addRepetitions(set.clone(), rep_set.clone())?;
                updateConditionCrefs(UnorderedSet::toList(set.clone()), dep_map.clone(), sol_map.clone())?;
                set = UnorderedSet::union_list(list![set.clone(), set1.clone(), set2.clone()], (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            }
            set.clone()
        },
        Deref @ Expression::CALL { .. } if (!(Partition::kindIsInitial(kind.clone())) && Call::isNamed(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), (literal!("homotopy")).clone())?) => {
            collectDependencies(listHead(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?)?, depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: call_exp, .. } } => {
            let mut call_exp = (*call_exp).clone();
            for mut iter in &*var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone() {
                let mut iter = iter.clone();
                call_exp = Expression::replaceIterator(call_exp.clone(), Util::tuple21(iter.clone()), Util::tuple22(iter.clone()))?;
            }
            set = collectDependencies(call_exp.clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            Solvability::updateList(UnorderedSet::toList(set.clone()), Arc::new(crate::NBAdjacency::Solvability::IMPLICIT), sol_map.clone())?;
            set.clone()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { exp: call_exp, .. } } => {
            let mut call_exp = (*call_exp).clone();
            for mut iter in &*var_field!((**call).iters, Call::NFCall::TYPED_REDUCTION).clone() {
                let mut iter = iter.clone();
                call_exp = Expression::replaceIterator(call_exp.clone(), Util::tuple21(iter.clone()), Util::tuple22(iter.clone()))?;
            }
            set = collectDependencies(call_exp.clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            Dependency::updateList(UnorderedSet::toList(set.clone()), -1, false, dep_map.clone())?;
            set.clone()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut new_depth: i32 = 0;
            let mut isTuple: bool = false;
            isTuple = Type::isTuple(var_field!((**call).ty, Call::NFCall::TYPED_CALL).clone());
            new_depth = if (isTuple.clone()) {depth.clone() + 1} else {depth.clone()};
            for mut arg in &*var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone() {
                let mut arg = arg.clone();
                sets = cons(collectDependencies(arg.clone(), new_depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets.clone());
            }
            set = UnorderedSet::union_list(sets.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            Dependency::updateList(UnorderedSet::toList(set.clone()), -1, false, dep_map.clone())?;
            Solvability::updateList(UnorderedSet::toList(set.clone()), Arc::new(crate::NBAdjacency::Solvability::IMPLICIT), sol_map.clone())?;
            addRepetitions(set.clone(), rep_set.clone())?;
            if isTuple.clone() {
                Dependency::skipList(UnorderedSet::toList(set.clone()), depth.clone() + 1, 0, dep_map.clone())?;
            }
            set.clone()
        },
        Deref @ Expression::RECORD { .. } => {
            for mut arg in &*var_field!((*exp).elements, Expression::NFExpression::RECORD).clone() {
                let mut arg = arg.clone();
                sets = cons(collectDependencies(arg.clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets.clone());
            }
            set = UnorderedSet::union_list(sets.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            Dependency::updateList(UnorderedSet::toList(set.clone()), -1, false, dep_map.clone())?;
            Solvability::updateList(UnorderedSet::toList(set.clone()), Arc::new(crate::NBAdjacency::Solvability::IMPLICIT), sol_map.clone())?;
            addRepetitions(set.clone(), rep_set.clone())?;
            set.clone()
        },
        Deref @ Expression::RANGE { .. } => {
            sets = cons(collectDependencies(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets.clone());
            if Util::isSome(var_field!((*exp).step, Expression::NFExpression::RANGE).clone()) {
                sets = cons(collectDependencies(Util::getOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone())?, depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets.clone());
            }
            sets = cons(collectDependencies(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), depth.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets.clone());
            set = UnorderedSet::union_list(sets.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            Solvability::updateList(UnorderedSet::toList(set.clone()), Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE), sol_map.clone())?;
            set.clone()
        },
        _ => {
            UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(set)
}

pub fn collectDependenciesCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut depth: i32, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut sk: i32 = 1;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    if UnorderedMap::contains(cref.clone(), map.clone()) {
        if !(UnorderedMap::contains(cref.clone(), dep_map.clone())) {
            UnorderedMap::add(cref.clone(), Dependency::create(ComponentRef::getSubscriptedType(cref.clone(), false)?, depth.clone())?, dep_map.clone())?;
        }
        Solvability::update(cref.clone(), Arc::new(Solvability::Solvability::EXPLICIT_LINEAR { pars: None, vars: None }), sol_map.clone())?;
        crefs = list![cref.clone()];
    } else {
        var = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
        if BVariable::isRecord(var.clone()) {
            subs = ComponentRef::subscriptsAllFlat(cref.clone());
            crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut child in (BVariable::getRecordChildren(var.clone())).into_iter().cloned() {
            let __x = BVariable::getVarName(child.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut child in (crefs.clone()).into_iter().cloned() {
            let __x = ComponentRef::mergeSubscripts(subs.clone(), child.clone(), false, false, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            crefs = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut child in (crefs.clone()).into_iter().cloned() {
            let __x = collectDependenciesCref(child.clone(), depth.clone() + 1, map.clone(), dep_map.clone(), sol_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut cref in &*crefs.clone() {
                let mut cref = cref.clone();
                Dependency::skip(cref.clone(), depth.clone() + 1, sk.clone(), dep_map.clone())?;
                sk = sk.clone() + 1;
            }
        } else {
            crefs = metamodelica::nil();
        }
    }
    Ok(crefs)
}

pub fn addRepetitionsCond(mut occ: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut exp: Arc<Expression::NFExpression>, mut isRep: bool, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    if isRep.clone() && Type::sizeOf(Expression::typeOf(exp.clone()), false)? == 1 {
        addRepetitions(occ.clone(), rep_set.clone())?;
    }
    Ok(())
}

pub fn addRepetitions(mut occ: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    for mut cref in &*UnorderedSet::toList(occ.clone()) {
        let mut cref = cref.clone();
        UnorderedSet::add(cref.clone(), rep_set.clone())?;
    }
    Ok(())
}

pub fn collectDependenciesIf(mut body: Arc<IfEquationBody::IfEquationBody>, mut kind: Partition::Kind, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut sets1: Arc<metamodelica::List<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut diff: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    if isInitialException(body.condition.clone()) {
        if Util::isSome(body.else_if.clone()) {
            set = collectDependenciesIf(Util::getOption(body.else_if.clone())?, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
        }
    } else {
        set = collectDependencies(body.condition.clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
        addRepetitions(set.clone(), rep_set.clone())?;
        updateConditionCrefs(UnorderedSet::toList(set.clone()), dep_map.clone(), sol_map.clone())?;
        for mut eqn in &*body.then_eqns.clone() {
            let mut eqn = eqn.clone();
            sets1 = cons(collectDependenciesEquation(Pointer::access(eqn.clone()), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets1.clone());
        }
        if Util::isSome(body.else_if.clone()) {
            set1 = UnorderedSet::union_list(sets1.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            set2 = collectDependenciesIf(Util::getOption(body.else_if.clone())?, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            diff = UnorderedSet::sym_difference(set1.clone(), set2.clone())?;
            Solvability::updateList(UnorderedSet::toList(diff.clone()), Arc::new(crate::NBAdjacency::Solvability::IMPLICIT), sol_map.clone())?;
            set = UnorderedSet::union_list(list![set.clone(), set1.clone(), set2.clone()], (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
        } else {
            set = UnorderedSet::union_list(cons(set.clone(), sets1.clone()), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
        }
    }
    Ok(set)
}

pub fn collectDependenciesWhen(mut body: Arc<WhenEquationBody::WhenEquationBody>, mut kind: Partition::Kind, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut diff: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut lst: Arc<metamodelica::List<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut lst1: Arc<metamodelica::List<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut lst2: Arc<metamodelica::List<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut tpl_lst: Arc<metamodelica::List<(Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    if isInitialException(body.condition.clone()) {
        if Util::isSome(body.else_when.clone()) {
            lst = cons(collectDependenciesWhen(Util::getOption(body.else_when.clone())?, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, lst.clone());
        }
        set = UnorderedSet::union_list(lst.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    } else {
        set = collectDependencies(body.condition.clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
        updateConditionCrefs(UnorderedSet::toList(set.clone()), dep_map.clone(), sol_map.clone())?;
        if {
        let mut __acc: i32 = 0;
        for mut stmt in (body.when_stmts.clone()).into_iter().cloned() {
            let __x = BEquation::WhenStatement::size(stmt.clone(), true)?;
            __acc += __x;
        }
        __acc
    } > 1 {
            addRepetitions(set.clone(), rep_set.clone())?;
        }
        for mut stmt in &*body.when_stmts.clone() {
            let mut stmt = stmt.clone();
            tpl_lst = cons(collectDependenciesStmt(stmt.clone(), kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, tpl_lst.clone());
        }
        (lst1, lst2) = List::unzip(tpl_lst.clone());
        set1 = UnorderedSet::union_list(lst1.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
        set2 = UnorderedSet::union_list(lst2.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
        diff = UnorderedSet::difference(set2.clone(), set1.clone())?;
        Solvability::updateList(UnorderedSet::toList(diff.clone()), Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE), sol_map.clone())?;
        if Util::isSome(body.else_when.clone()) {
            lst = cons(collectDependenciesWhen(Util::getOption(body.else_when.clone())?, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, lst.clone());
        }
        set = UnorderedSet::union_list(cons(set.clone(), cons(set1.clone(), cons(set2.clone(), lst.clone()))), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    }
    Ok(set)
}

pub fn collectDependenciesStmt(mut stmt: Arc<WhenStatement::WhenStatement>, mut kind: Partition::Kind, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<(Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>)> {
    let mut set_tpl: (Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>);
    let mut set1: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut set2: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    set_tpl = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ BEquation::WhenStatement::ASSIGN { .. } => {
            set1 = collectDependencies(var_field!((*stmt).lhs, WhenStatement::WhenStatement::ASSIGN).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            set2 = collectDependencies(var_field!((*stmt).rhs, WhenStatement::WhenStatement::ASSIGN).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            (set1.clone(), set2.clone())
        },
        Deref @ BEquation::WhenStatement::REINIT { .. } => {
            set1 = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            set2 = collectDependencies(var_field!((*stmt).value, WhenStatement::WhenStatement::REINIT).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            (set1.clone(), set2.clone())
        },
        Deref @ BEquation::WhenStatement::ASSERT { .. } => {
            set1 = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            set2 = collectDependencies(var_field!((*stmt).condition, WhenStatement::WhenStatement::ASSERT).clone(), 0, kind.clone(), map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            updateConditionCrefs(UnorderedSet::toList(set2.clone()), dep_map.clone(), sol_map.clone())?;
            (set1.clone(), set2.clone())
        },
        _ => {
            set1 = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            set2 = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            (set1.clone(), set2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(set_tpl)
}

pub fn collectDependenciesAlgorithmInputs(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut inputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut inputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = inputs;
    let mut candidates: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::fromList(inputs.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    let mut result: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::fromList(inputs.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    for mut stmt in &*stmts.clone() {
        let mut stmt = stmt.clone();
        collectDependenciesAlgorithmStatement(stmt.clone(), candidates.clone(), result.clone())?;
    }
    inputs = UnorderedSet::toList(result.clone());
    Ok(inputs)
}

pub fn collectDependenciesAlgorithmStatement(mut stmt: Arc<Statement::NFStatement>, mut candidates: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut result: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            Slice::filterExp(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), Arc::new(todo!("PARTEVALFUNCTION of Equation.collectFromSet: function signature not resolved")), result.clone())?;
            Slice::filterExp(var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), Arc::new(todo!("PARTEVALFUNCTION of Equation.collectFromSet: function signature not resolved")), result.clone())?;
            ()
        },
        Deref @ Statement::FOR { .. } => {
            for mut s in &*var_field!((*stmt).body, Statement::NFStatement::FOR).clone() {
                let mut s = s.clone();
                collectDependenciesAlgorithmStatement(s.clone(), candidates.clone(), result.clone())?;
            }
            ()
        },
        Deref @ Statement::WHILE { .. } => {
            for mut s in &*var_field!((*stmt).body, Statement::NFStatement::WHILE).clone() {
                let mut s = s.clone();
                collectDependenciesAlgorithmStatement(s.clone(), candidates.clone(), result.clone())?;
            }
            ()
        },
        Deref @ Statement::IF { .. } => {
            for mut branch in &*var_field!((*stmt).branches, Statement::NFStatement::IF).clone() {
                let mut branch = branch.clone();
                if !(isInitialException(Util::tuple21(branch.clone()))) {
                    for mut s in &*Util::tuple22(branch.clone()) {
                        let mut s = s.clone();
                        collectDependenciesAlgorithmStatement(s.clone(), candidates.clone(), result.clone())?;
                    }
                }
            }
            ()
        },
        Deref @ Statement::WHEN { .. } => {
            for mut branch in &*var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone() {
                let mut branch = branch.clone();
                if !(isInitialException(Util::tuple21(branch.clone()))) {
                    for mut s in &*Util::tuple22(branch.clone()) {
                        let mut s = s.clone();
                        collectDependenciesAlgorithmStatement(s.clone(), candidates.clone(), result.clone())?;
                    }
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn updateConditionCrefs(mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>) -> Result<()> {
    Dependency::removeSkipsList(crefs.clone(), dep_map.clone())?;
    Dependency::updateList(crefs.clone(), -1, false, dep_map.clone())?;
    Solvability::updateList(crefs.clone(), Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE), sol_map.clone())?;
    Ok(())
}

pub fn isInitialException(mut exp: Arc<Expression::NFExpression>) -> bool {
    let mut b: bool = Expression::isCallNamed(exp.clone(), (literal!("initial")).clone()).unwrap() || Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("initialSimplified")).clone()).unwrap() && Expression::isCallNamed(exp.clone(), (literal!("initialSimplified")).clone()).unwrap();
    b
}

pub fn addInitialStartOccurrences(mut occs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut dep_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut sol_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>, mut rep_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut kind: Partition::Kind) -> Result<()> {
    if Partition::kindIsInitial(kind.clone()) {
        for mut cref in &*UnorderedSet::toList(occs.clone()) {
            let mut cref = cref.clone();
            let _ = (match BVariable::getVarStart(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?) {
        Some(mut start) if (BVariable::isStart(start.clone())) => {
            let mut start_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            start_cref = BVariable::getVarName(start.clone());
            UnorderedSet::add(start_cref.clone(), occs.clone())?;
            UnorderedMap::add(start_cref.clone(), UnorderedMap::getSafe(cref.clone(), dep_map.clone(), metamodelica::sourceInfo!())?, dep_map.clone())?;
            UnorderedMap::add(start_cref.clone(), Arc::new(crate::NBAdjacency::Solvability::UNSOLVABLE), sol_map.clone())?;
            if UnorderedSet::contains(cref.clone(), rep_set.clone())? {
                UnorderedSet::add(start_cref.clone(), rep_set.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
    });
        }
    }
    Ok(())
}

