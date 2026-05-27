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
use crate::NBAdjacency::Mode;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBMatching as Matching;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// NB imports
// NF imports
// Util imports
// ############################################################
//                Pseudo Bucket Structures
// ############################################################
pub mod Value {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Value {
        SINGLE_VAL {
            /// cref to solve for in this mode
            cref_to_solve: Arc<ComponentRef::NFComponentRef>,
            /// indices of all scalarized equations that have to be solved that way
            eqn_scal_indices: Arc<metamodelica::List<i32>>,
        },
        MULTI_VAL {
            /// crefs to solve for in this mode
            crefs_to_solve: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
            /// indices of all scalarized equations that have to be solved that way
            eqn_scal_indices: Arc<metamodelica::List<i32>>,
        },
    }
    impl Default for Value {
        fn default() -> Self {
            Self::SINGLE_VAL {
                cref_to_solve: Default::default(),
                eqn_scal_indices: Default::default(),
            }
        }
    }
    pub use self::Value::{SINGLE_VAL,MULTI_VAL};
    pub fn toString(mut val: Arc<Value>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(val.clone()) {
        Deref @ SINGLE_VAL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\tval: (")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*val).cref_to_solve, Value::SINGLE_VAL).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ MULTI_VAL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\tval: ")); __mm_s.push_str(&*List::toString(var_field!((*val).crefs_to_solve, Value::MULTI_VAL).clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(r#str)
    }

    pub fn filter(mut val: Arc<Value>, mut set: Arc<UnorderedSet::UnorderedSet<i32>>) -> Result<Arc<Value>> {
        let mut val: Arc<Value> = val;
        val = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ SINGLE_VAL { .. } => {
            assign_variant_field!(val => Value::SINGLE_VAL; eqn_scal_indices = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut idx in (var_field!((*val).eqn_scal_indices, Value::SINGLE_VAL).clone()).into_iter().cloned() {
            if !(!(UnorderedSet::contains(idx.clone(), set.clone())?)) { continue; }
            let __x = idx.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            val.clone()
        },
        Deref @ MULTI_VAL { .. } => {
            assign_variant_field!(val => Value::MULTI_VAL; eqn_scal_indices = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut idx in (var_field!((*val).eqn_scal_indices, Value::MULTI_VAL).clone()).into_iter().cloned() {
            if !(!(UnorderedSet::contains(idx.clone(), set.clone())?)) { continue; }
            let __x = idx.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            val.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(val)
    }

    pub fn getEquations(mut val: Arc<Value>) -> Result<Arc<metamodelica::List<i32>>> {
        let mut eqn_scal_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
        eqn_scal_indices = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ SINGLE_VAL { .. } => var_field!((*val).eqn_scal_indices, Value::SINGLE_VAL).clone(),
        Deref @ MULTI_VAL { .. } => var_field!((*val).eqn_scal_indices, Value::MULTI_VAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(eqn_scal_indices)
    }

    pub fn addEquation(mut val: Arc<Value>, mut eqn_idx: i32) -> Result<Arc<Value>> {
        let mut val: Arc<Value> = val;
        val = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ SINGLE_VAL { .. } => {
            assign_variant_field!(val => Value::SINGLE_VAL; eqn_scal_indices = cons(eqn_idx.clone(), var_field!((*val).eqn_scal_indices, Value::SINGLE_VAL).clone()));
            val.clone()
        },
        Deref @ MULTI_VAL { .. } => {
            assign_variant_field!(val => Value::MULTI_VAL; eqn_scal_indices = cons(eqn_idx.clone(), var_field!((*val).eqn_scal_indices, Value::MULTI_VAL).clone()));
            val.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(val)
    }

    pub fn addCref(mut val: Arc<Value>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<Value>> {
        let mut val: Arc<Value> = val;
        val = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ MULTI_VAL { .. } => {
            assign_variant_field!(val => Value::MULTI_VAL; crefs_to_solve = cons(cref.clone(), var_field!((*val).crefs_to_solve, Value::MULTI_VAL).clone()));
            val.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.Value.addCref")); __mm_s.push_str(&*literal!(" failed because trying to add a cref to a single value.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(val)
    }

}

pub mod PseudoBucket {
    use super::*;
    pub fn create(mut eqn_to_var: metamodelica::Array<i32>, mut eqns: Arc<EquationPointers::EquationPointers>, mut mapping: Arc<Adjacency::Mapping::Mapping>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<Mode::Mode>, Arc<Value::Value>>>> {
        let mut buckets: Arc<UnorderedMap::UnorderedMap<Arc<Mode::Mode>, Arc<Value::Value>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(Mode::hash, Arc<Mode::Mode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Mode::Mode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(Mode::isEqual, Arc<Mode::Mode>, Arc<Mode::Mode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Mode::Mode>, Arc<Mode::Mode>) -> Result<bool> + 'static>), 1);
        let mut mode_opt: Option<Arc<Mode::Mode>> = None;
        let mut mode: Arc<Mode::Mode> = Arc::new(<Mode::Mode as ::std::default::Default>::default());
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let __range0 = 1..=(eqn_to_var.clone().borrow().len() as i32);
        for mut eqn_scal_idx in __range0 {
            mode_opt = UnorderedMap::get((eqn_scal_idx.clone(), eqn_to_var.borrow()[(eqn_scal_idx.clone()-1) as usize].clone()), modes.clone());
            if Util::isSome(mode_opt.clone()) {
                mode = Util::getOption(mode_opt.clone())?;
                if BEquation::Equation::isRecordOrTupleEquation(BEquation::EquationPointers::getEqnAt(eqns.clone(), mapping.eqn_StA.borrow()[(eqn_scal_idx.clone()-1) as usize].clone())?) {
                    cref = listHead(mode.crefs.clone())?;
                    assign_field!(mode.crefs = metamodelica::nil());
                    addMulti(cref.clone(), eqn_scal_idx.clone(), mode.clone(), buckets.clone())?;
                } else {
                    add(eqn_scal_idx.clone(), mode.clone(), buckets.clone())?;
                }
            }
        }
        if Flags::isSet(Flags::DUMP_SORTING.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*UnorderedMap::toString(buckets.clone(), (std::sync::Arc::new(fnptr!(Mode::toString, Arc<Mode::Mode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Mode::Mode>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Value::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Value::Value>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok(buckets)
    }

    pub fn add(mut eqn_scal_idx: i32, mut mode: Arc<Mode::Mode>, mut buckets: Arc<UnorderedMap::UnorderedMap<Arc<Mode::Mode>, Arc<Value::Value>>>) -> Result<()> {
        let mut val_opt: Option<Arc<Value::Value>> = UnorderedMap::get(mode.clone(), buckets.clone());
        let mut val: Arc<Value::Value>;
        if Util::isSome(val_opt.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(val_opt.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            val = __pa0.clone();
            val = Value::addEquation(val.clone(), eqn_scal_idx.clone())?;
            UnorderedMap::add(mode.clone(), val.clone(), buckets.clone())?;
        } else {
            val = Arc::new(Value::Value::SINGLE_VAL { cref_to_solve: listHead(mode.crefs.clone())?, eqn_scal_indices: list![eqn_scal_idx.clone()] });
            UnorderedMap::addNew(mode.clone(), val.clone(), buckets.clone())?;
        }
        Ok(())
    }

    pub fn addMulti(mut cref: Arc<ComponentRef::NFComponentRef>, mut eqn_scal_idx: i32, mut mode: Arc<Mode::Mode>, mut buckets: Arc<UnorderedMap::UnorderedMap<Arc<Mode::Mode>, Arc<Value::Value>>>) -> Result<()> {
        let mut val_opt: Option<Arc<Value::Value>> = UnorderedMap::get(mode.clone(), buckets.clone());
        let mut val: Arc<Value::Value>;
        if Util::isSome(val_opt.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(val_opt.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            val = __pa0.clone();
            val = Value::addCref(val.clone(), cref.clone())?;
            val = Value::addEquation(val.clone(), eqn_scal_idx.clone())?;
            UnorderedMap::add(mode.clone(), val.clone(), buckets.clone())?;
        } else {
            val = Arc::new(Value::Value::MULTI_VAL { crefs_to_solve: mode.crefs.clone(), eqn_scal_indices: list![eqn_scal_idx.clone()] });
            UnorderedMap::addNew(mode.clone(), val.clone(), buckets.clone())?;
        }
        Ok(())
    }

    pub fn filter(mut tpl: (Arc<Mode::Mode>, Arc<Value::Value>), mut set: Arc<UnorderedSet::UnorderedSet<i32>>) -> Result<(Arc<Mode::Mode>, Arc<Value::Value>)> {
        let mut tpl: (Arc<Mode::Mode>, Arc<Value::Value>) = tpl;
        let mut mode: Arc<Mode::Mode> = Arc::new(<Mode::Mode as ::std::default::Default>::default());
        let mut val: Arc<Value::Value>;
        (mode, val) = tpl.clone();
        val = Value::filter(val.clone(), set.clone())?;
        tpl = (mode.clone(), val.clone());
        Ok(tpl)
    }

    pub fn relevant(mut tpl: (Arc<Mode::Mode>, Arc<Value::Value>)) -> Result<bool> {
        let mut b: bool = false;
        let mut val: Arc<Value::Value>;
        (_, val) = tpl.clone();
        b = List::hasSeveralElements(Value::getEquations(val.clone())?);
        Ok(b)
    }

}

// ############################################################
//                      Main Functions
// ############################################################
pub fn tarjan(mut adj: Arc<Adjacency::Matrix::Matrix>, mut matching: Arc<Matching::NBMatching>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>) -> Result<Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>> {
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut mapping_opt: Option<Arc<Adjacency::Mapping::Mapping>> = None;
    let mut eqn_AtS: Option<metamodelica::Array<(i32, i32)>> = None;
    let mut var_AtS: Option<metamodelica::Array<(i32, i32)>> = None;
    match '__try0: {
        comps = (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ Adjacency::Matrix::FINAL { .. } => {
            let mut comps_indices: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut phase2_indices: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut phase2_adj: Arc<Adjacency::Matrix::Matrix>;
            let mut phase2_matching: Arc<Matching::NBMatching> = Arc::new(<Matching::NBMatching as ::std::default::Default>::default());
            let mut super_nodes: metamodelica::Array<Arc<SuperNode::SuperNode>>;
            let mut buckets: Arc<UnorderedMap::UnorderedMap<Arc<Mode::Mode>, Arc<Value::Value>>>;
            if unwrap_break_err!(Flags::isSet(Flags::DUMP_SORTING.clone()), '__try0) {
                println!("{}", (StringUtil::headline_1((literal!("Sorting")).clone())).clone());
            }
            buckets = unwrap_break_err!(PseudoBucket::create(matching.eqn_to_var.clone(), eqns.clone(), var_field!((*adj).mapping, Adjacency::Matrix::Matrix::FINAL).clone(), var_field!((*adj).modes, Adjacency::Matrix::Matrix::FINAL).clone()), '__try0);
            comps_indices = unwrap_break_err!(tarjanScalar(var_field!((*adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), matching.clone()), '__try0);
            (phase2_adj, phase2_matching, super_nodes) = unwrap_break_err!(SuperNode::create(adj.clone(), var_field!((*adj).mapping, Adjacency::Matrix::Matrix::FINAL).clone(), matching.clone(), eqns.map.clone(), comps_indices.clone(), buckets.clone()), '__try0);
            let () = (::match_deref::match_deref! { match &(phase2_adj.clone()) {
        Deref @ Adjacency::Matrix::FINAL { .. } => {
            phase2_indices = unwrap_break_err!(tarjanScalar(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), phase2_matching.clone()), '__try0);
            comps = {
        let mut __acc: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        for mut comp in (phase2_indices.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(SuperNode::collapse(comp.clone(), super_nodes.clone(), var_field!((*adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), var_field!((*adj).mapping, Adjacency::Matrix::Matrix::FINAL).clone(), matching.clone(), vars.clone(), eqns.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ()
        },
        _ => {
            unwrap_break_err!(Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.tarjan")); __mm_s.push_str(&*literal!(" failed because of unknown adjacency matrix or matching type.")); ArcStr::from(__mm_s) }).clone()]), '__try0);
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            comps.clone()
        },
        Deref @ Adjacency::Matrix::EMPTY { .. } => {
            metamodelica::nil()
        },
        _ => {
            unwrap_break_err!(Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.tarjan")); __mm_s.push_str(&*literal!(" failed because adjacency matrix has unknown type.")); ArcStr::from(__mm_s) }).clone()]), '__try0);
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok::<_, anyhow::Error>((comps.clone(),))
    } {
        Ok((__try0_o0,)) => {
            comps = __try0_o0;
        }
        Err(_) => {
            mapping_opt = Adjacency::Matrix::getMappingOpt(adj.clone());
            (eqn_AtS, var_AtS) = (::match_deref::match_deref! { match &(mapping_opt.clone()) {
        Some(mapping) => {
            (Some(mapping.eqn_AtS.clone()), Some(mapping.var_AtS.clone()))
        },
        _ => {
            (None, None)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.tarjan")); __mm_s.push_str(&*literal!(" failed to sort system:\n")); __mm_s.push_str(&*BVariable::VariablePointers::toString(vars.clone(), (literal!("System")).clone(), var_AtS.clone(), true)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BEquation::EquationPointers::toString(eqns.clone(), (literal!("System")).clone(), eqn_AtS.clone(), true, None)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Matching::toString(matching.clone(), (literal!("")).clone())); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
    }
    Ok(comps)
}

pub fn tarjanScalar(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut number: metamodelica::Array<i32>;
    let mut lowlink: metamodelica::Array<i32>;
    let mut onStack: metamodelica::Array<bool>;
    let mut N: i32 = (matching.var_to_eqn.clone().borrow().len() as i32);
    let mut M: i32 = (matching.eqn_to_var.clone().borrow().len() as i32);
    let mut eqn: i32 = 0;
    number = arrayCreate(M.clone(), -1);
    lowlink = arrayCreate(M.clone(), -1);
    onStack = arrayCreate(M.clone(), false);
    for mut var in 1..=N.clone() {
        eqn = matching.var_to_eqn[(var.clone()-1) as usize].clone();
        if eqn.clone() > 0 && number.borrow()[(eqn.clone()-1) as usize].clone() == -1 {
            (stack, index, comps) = strongConnect(m.clone(), matching.var_to_eqn.clone(), eqn.clone(), stack.clone(), index.clone(), number.clone(), lowlink.clone(), onStack.clone(), comps.clone())?;
        }
    }
    GCExt::free(number.clone());
    GCExt::free(lowlink.clone());
    GCExt::free(onStack.clone());
    comps = comps.clone().reverse();
    Ok(comps)
}

pub type SCC = Arc<metamodelica::List<i32>>;

pub mod LoopIdentifier {
    use super::*;
    /// used to identify algebraic loops that are structurally equal just differ in local indexing
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LoopIdentifier {
        pub eqns: Arc<UnorderedSet::UnorderedSet<i32>>,
        pub vars: Arc<UnorderedSet::UnorderedSet<i32>>,
    }

    impl Default for LoopIdentifier {
        fn default() -> Self {
            Self {
                eqns: Default::default(),
                vars: Default::default(),
            }
        }
    }

    pub type LOOP_IDENTIFIER = LoopIdentifier;

    pub fn hash(mut li: Arc<LoopIdentifier>) -> i32 {
        let mut i: i32 = stringHashDjb2((toString(li.clone())).clone());
        i
    }

    pub fn isEqual(mut li1: Arc<LoopIdentifier>, mut li2: Arc<LoopIdentifier>) -> bool {
        let mut b: bool = UnorderedSet::isEqual(li1.eqns.clone(), li2.eqns.clone()).unwrap() && UnorderedSet::isEqual(li1.vars.clone(), li2.vars.clone()).unwrap();
        b
    }

    pub fn toString(mut li: Arc<LoopIdentifier>) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" eqns: ")); __mm_s.push_str(&*UnorderedSet::toString(li.eqns.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n vars:")); __mm_s.push_str(&*UnorderedSet::toString(li.vars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str
    }

    pub fn fromSCC(mut scc: Arc<metamodelica::List<i32>>, mut mapping: Arc<Adjacency::Mapping::Mapping>, mut matching: Arc<Matching::NBMatching>) -> Result<Arc<LoopIdentifier>> {
        let mut li: Arc<LoopIdentifier> = Arc::new(<LoopIdentifier as ::std::default::Default>::default());
        li = Arc::new(LoopIdentifier { vars: UnorderedSet::fromList({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (scc.clone()).into_iter().cloned() {
            let __x = mapping.var_StA.borrow()[(matching.eqn_to_var[(i.clone()-1) as usize].clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, eqns: UnorderedSet::fromList({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (scc.clone()).into_iter().cloned() {
            let __x = mapping.eqn_StA.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))? });
        Ok(li)
    }

}

pub mod SuperNode {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SuperNode {
        /// does not belong to an algebraic loop or array
        SINGLE {
            index: i32,
        },
        /// is part of either an algebraic loop or array
        ELEMENT {
            index: i32,
            parent: i32,
        },
        /// an algebraic loop of equations
        ALGEBRAIC_LOOP {
            index: i32,
            eqn_indices: Arc<metamodelica::List<i32>>,
        },
        /// a bucket of array equations solved for the same cref
        ARRAY_BUCKET {
            index: i32,
            cref_to_solve: Arc<ComponentRef::NFComponentRef>,
            eqn_indices: Arc<metamodelica::List<i32>>,
            arr_idx: i32,
        },
    }
    pub use self::SuperNode::{SINGLE,ELEMENT,ALGEBRAIC_LOOP,ARRAY_BUCKET};
    pub fn toString(mut node: Arc<SuperNode>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(node.clone()) {
        Deref @ SINGLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(var_field!((*node).index, SuperNode::SINGLE).clone() + 1)); __mm_s.push_str(&*literal!("] single ")); ArcStr::from(__mm_s) },
        Deref @ ELEMENT { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(var_field!((*node).index, SuperNode::ELEMENT).clone() + 1)); __mm_s.push_str(&*literal!("] scalar element of (")); __mm_s.push_str(&*intString(var_field!((*node).parent, SuperNode::ELEMENT).clone() + 1)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ ALGEBRAIC_LOOP { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(var_field!((*node).index, SuperNode::ALGEBRAIC_LOOP).clone() + 1)); __mm_s.push_str(&*literal!("] algebraic loop ")); __mm_s.push_str(&*List::toString({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (var_field!((*node).eqn_indices, SuperNode::ALGEBRAIC_LOOP).clone()).into_iter().cloned() {
            let __x = i.clone() + 1;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        Deref @ ARRAY_BUCKET { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(var_field!((*node).index, SuperNode::ARRAY_BUCKET).clone() + 1)); __mm_s.push_str(&*literal!("] array bucket ")); __mm_s.push_str(&*List::toString({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (var_field!((*node).eqn_indices, SuperNode::ARRAY_BUCKET).clone()).into_iter().cloned() {
            let __x = i.clone() + 1;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        _ => literal!("ERROR"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn isArrayBucket(mut node: Arc<SuperNode>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ ARRAY_BUCKET { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn getEqnIndices(mut node: Arc<SuperNode>) -> Result<Arc<metamodelica::List<i32>>> {
        let mut eqn_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
        eqn_indices = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ SINGLE { .. } => list![var_field!((*node).index, SuperNode::SINGLE).clone()],
        Deref @ ALGEBRAIC_LOOP { .. } => var_field!((*node).eqn_indices, SuperNode::ALGEBRAIC_LOOP).clone(),
        Deref @ ARRAY_BUCKET { .. } => var_field!((*node).eqn_indices, SuperNode::ARRAY_BUCKET).clone(),
        Deref @ ELEMENT { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.SuperNode.getEqnIndices")); __mm_s.push_str(&*literal!(" failed because elements should not be accessed, only their parents: ")); __mm_s.push_str(&*toString(node.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.SuperNode.getEqnIndices")); __mm_s.push_str(&*literal!(" failed because of incorrect super node type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqn_indices)
    }

    pub fn create(mut adj: Arc<Adjacency::Matrix::Matrix>, mut mapping: Arc<Adjacency::Mapping::Mapping>, mut matching: Arc<Matching::NBMatching>, mut eqn_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut scc_phase1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut buck: Arc<UnorderedMap::UnorderedMap<Arc<Mode::Mode>, Arc<Value::Value>>>) -> Result<(Arc<Adjacency::Matrix::Matrix>, Arc<Matching::NBMatching>, metamodelica::Array<Arc<SuperNode>>)> {
        let mut phase2_adj: Arc<Adjacency::Matrix::Matrix> = adj.clone();
        let mut phase2_matching: Arc<Matching::NBMatching> = matching.clone();
        let mut super_nodes: metamodelica::Array<Arc<SuperNode>>;
        let mut li: Arc<LoopIdentifier::LoopIdentifier> = Arc::new(<LoopIdentifier::LoopIdentifier as ::std::default::Default>::default());
        let mut loop_map: Arc<UnorderedMap::UnorderedMap<Arc<LoopIdentifier::LoopIdentifier>, Arc<metamodelica::List<i32>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(LoopIdentifier::hash, Arc<LoopIdentifier::LoopIdentifier>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<LoopIdentifier::LoopIdentifier>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(LoopIdentifier::isEqual, Arc<LoopIdentifier::LoopIdentifier>, Arc<LoopIdentifier::LoopIdentifier>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<LoopIdentifier::LoopIdentifier>, Arc<LoopIdentifier::LoopIdentifier>) -> Result<bool> + 'static>), 1);
        let mut algebraic_loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut scc in (scc_phase1.clone()).into_iter().cloned() {
            if !(List::hasSeveralElements(scc.clone())) { continue; }
            let __x = scc.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        let mut buckets: Arc<metamodelica::List<(Arc<Mode::Mode>, Arc<Value::Value>)>> = UnorderedMap::toList(buck.clone());
        let mut mode: Arc<Mode::Mode> = Arc::new(<Mode::Mode as ::std::default::Default>::default());
        let mut val: Arc<Value::Value>;
        let mut index: i32 = 0;
        let mut shift: i32 = 0;
        let mut var_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut eqn_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut alg_loop_set: Arc<UnorderedSet::UnorderedSet<i32>> = UnorderedSet::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 13);
        phase2_adj = (::match_deref::match_deref! { match &(phase2_adj.clone()) {
        Deref @ Adjacency::Matrix::FINAL { .. } => {
            for mut scc in &*algebraic_loops.clone() {
                let mut scc = scc.clone();
                li = LoopIdentifier::fromSCC(scc.clone(), mapping.clone(), matching.clone())?;
                UnorderedMap::add(li.clone(), listAppend(scc.clone(), UnorderedMap::getOrDefault(li.clone(), loop_map.clone(), metamodelica::nil())), loop_map.clone())?;
            }
            algebraic_loops = UnorderedMap::valueList(loop_map.clone());
            for mut scc in &*algebraic_loops.clone() {
                let mut scc = scc.clone();
                for mut idx in &*scc.clone() {
                    let mut idx = idx.clone();
                    UnorderedSet::add(idx.clone(), alg_loop_set.clone())?;
                }
            }
            buckets = {
        let mut __acc: Arc<metamodelica::List<(Arc<Mode::Mode>, Arc<Value::Value>)>> = metamodelica::nil();
        for mut bucket_tpl in (buckets.clone()).into_iter().cloned() {
            let __x = PseudoBucket::filter(bucket_tpl.clone(), alg_loop_set.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            buckets = {
        let mut __acc: Arc<metamodelica::List<(Arc<Mode::Mode>, Arc<Value::Value>)>> = metamodelica::nil();
        for mut bucket_tpl in (buckets.clone()).into_iter().cloned() {
            if !(PseudoBucket::relevant(bucket_tpl.clone())?) { continue; }
            let __x = bucket_tpl.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            shift = (algebraic_loops.clone().len() as i32) + (buckets.clone().len() as i32);
            super_nodes = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<SuperNode>>> = metamodelica::nil();
        for mut i in (1..=(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL).clone().borrow().len() as i32) + shift.clone()).into_iter() {
            let __x = Arc::new(SuperNode::SINGLE { index: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
            index = (phase2_matching.eqn_to_var.clone().borrow().len() as i32);
            assign_field!(phase2_matching.eqn_to_var = Array::expandToSize((phase2_matching.eqn_to_var.clone().borrow().len() as i32) + shift.clone(), phase2_matching.eqn_to_var.clone(), -1)?);
            for mut i in index.clone() + 1..=index.clone() + shift.clone() {
                phase2_matching.eqn_to_var.clone()[(i.clone()-1) as usize] = i.clone(); // TODO: indexed assign on non-Array base
            }
            index = (phase2_matching.var_to_eqn.clone().borrow().len() as i32);
            assign_field!(phase2_matching.var_to_eqn = Array::expandToSize((phase2_matching.var_to_eqn.clone().borrow().len() as i32) + shift.clone(), phase2_matching.var_to_eqn.clone(), -1)?);
            for mut i in index.clone() + 1..=index.clone() + shift.clone() {
                phase2_matching.var_to_eqn.clone()[(i.clone()-1) as usize] = i.clone(); // TODO: indexed assign on non-Array base
            }
            index = (var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL).clone().borrow().len() as i32) + 1;
            assign_variant_field!(phase2_adj => Adjacency::Matrix::Matrix::FINAL; mT = Adjacency::Matrix::expandMatrix(var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL).clone(), shift.clone())?);
            for mut scc in &*algebraic_loops.clone() {
                let mut scc = scc.clone();
                var_lst = {
        let mut __acc: Arc<metamodelica::List<Arc<Matching::NBMatching>>> = metamodelica::nil();
        for mut idx in (scc.clone()).into_iter().cloned() {
            let __x = phase2_matching.eqn_to_var[(idx.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                mergeLoopNodes(super_nodes.clone(), var_lst.clone(), index.clone(), false)?;
                index = mergeRows(var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL).clone(), phase2_matching.var_to_eqn.clone(), super_nodes.clone(), var_lst.clone(), index.clone())?;
            }
            for mut bucket in &*buckets.clone() {
                let mut bucket = bucket.clone();
                (mode, val) = bucket.clone();
                var_lst = {
        let mut __acc: Arc<metamodelica::List<Arc<Matching::NBMatching>>> = metamodelica::nil();
        for mut idx in (Value::getEquations(val.clone())?).into_iter().cloned() {
            let __x = phase2_matching.eqn_to_var[(idx.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                let _ = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Value::SINGLE_VAL { .. } => {
            mergeArrayNodes(super_nodes.clone(), var_field!((*val).cref_to_solve, Value::Value::SINGLE_VAL).clone(), var_lst.clone(), index.clone(), UnorderedMap::getSafe(mode.eqn_name.clone(), eqn_map.clone(), metamodelica::sourceInfo!())?, false)?;
            ()
        },
        Deref @ Value::MULTI_VAL { .. } => {
            mergeLoopNodes(super_nodes.clone(), var_lst.clone(), index.clone(), false)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
                index = mergeRows(var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL).clone(), phase2_matching.var_to_eqn.clone(), super_nodes.clone(), var_lst.clone(), index.clone())?;
            }
            index = (var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL).clone().borrow().len() as i32) + 1;
            assign_variant_field!(phase2_adj => Adjacency::Matrix::Matrix::FINAL; m = Adjacency::Matrix::transposeScalar(var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL).clone(), (var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL).clone().borrow().len() as i32) + shift.clone())?);
            for mut scc in &*algebraic_loops.clone() {
                let mut scc = scc.clone();
                mergeLoopNodes(super_nodes.clone(), scc.clone(), index.clone(), true)?;
                index = mergeRows(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), phase2_matching.eqn_to_var.clone(), super_nodes.clone(), scc.clone(), index.clone())?;
            }
            for mut bucket in &*buckets.clone() {
                let mut bucket = bucket.clone();
                (mode, val) = bucket.clone();
                eqn_lst = Value::getEquations(val.clone())?;
                let _ = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Value::SINGLE_VAL { .. } => {
            mergeArrayNodes(super_nodes.clone(), var_field!((*val).cref_to_solve, Value::Value::SINGLE_VAL).clone(), eqn_lst.clone(), index.clone(), UnorderedMap::getSafe(mode.eqn_name.clone(), eqn_map.clone(), metamodelica::sourceInfo!())?, true)?;
            ()
        },
        Deref @ Value::MULTI_VAL { .. } => {
            mergeLoopNodes(super_nodes.clone(), eqn_lst.clone(), index.clone(), true)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
                index = mergeRows(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), phase2_matching.eqn_to_var.clone(), super_nodes.clone(), eqn_lst.clone(), index.clone())?;
            }
            assign_variant_field!(phase2_adj => Adjacency::Matrix::Matrix::FINAL; mT = Adjacency::Matrix::transposeScalar(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), (var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL).clone().borrow().len() as i32))?);
            phase2_adj.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.SuperNode.create")); __mm_s.push_str(&*literal!(" failed because of unknown adjacency matrix type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((phase2_adj, phase2_matching, super_nodes))
    }

    pub fn collapse(mut comp_indices: Arc<metamodelica::List<i32>>, mut super_nodes: metamodelica::Array<Arc<SuperNode>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapping: Arc<Adjacency::Mapping::Mapping>, mut matching: Arc<Matching::NBMatching>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>) -> Result<Arc<StrongComponent::NBStrongComponent>> {
        let mut comp: Arc<StrongComponent::NBStrongComponent>;
        let mut node_comp: Arc<metamodelica::List<Arc<SuperNode>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<SuperNode>>> = metamodelica::nil();
        for mut i in (comp_indices.clone()).into_iter().cloned() {
            let __x = super_nodes.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        let mut sorted_body_components: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        let mut sorted_body_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
        comp = ({
        let mut indep: bool = true;
        (::match_deref::match_deref! { match &(node_comp.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ SINGLE { .. }, tail: Deref @ metamodelica::List::Nil } => {
            StrongComponent::createPseudoScalar(comp_indices.clone(), matching.eqn_to_var.clone(), mapping.clone(), vars.clone(), eqns.clone())?
        },
        Deref @ metamodelica::List::Cons { head: node @ Deref @ ALGEBRAIC_LOOP { .. }, tail: Deref @ metamodelica::List::Nil } => {
            StrongComponent::createPseudoScalar(var_field!((**node).eqn_indices, SuperNode::ALGEBRAIC_LOOP).clone(), matching.eqn_to_var.clone(), mapping.clone(), vars.clone(), eqns.clone())?
        },
        Deref @ metamodelica::List::Cons { head: node @ Deref @ ARRAY_BUCKET { .. }, tail: Deref @ metamodelica::List::Nil } => {
            let mut m_local: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut matching_local: Arc<Matching::NBMatching> = Arc::new(<Matching::NBMatching as ::std::default::Default>::default());
            let mut map_back: metamodelica::Array<i32>;
            let mut eqn_arr_idx: i32 = 0;
            let mut var_arr_idx: i32 = 0;
            (m_local, matching_local, map_back) = BackendUtil::getLocalSystem(m.clone(), matching.clone(), var_field!((**node).eqn_indices, SuperNode::ARRAY_BUCKET).clone())?;
            sorted_body_components = tarjanScalar(m_local.clone(), matching_local.clone())?;
            sorted_body_indices = List::flatten(sorted_body_components.clone());
            sorted_body_indices = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (sorted_body_indices.clone()).into_iter().cloned() {
            let __x = map_back.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if List::compareLength(sorted_body_components.clone(), sorted_body_indices.clone())? != 0 {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.SuperNode.collapse")); __mm_s.push_str(&*literal!(" crucially failed for the following Phase II strong component")); __mm_s.push_str(&*literal!(" because the body turned out to still have strong components:\n")); __mm_s.push_str(&*List::toString(node_comp.clone(), (std::sync::Arc::new(SuperNode::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SuperNode>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone()])?;
            }
            indep = Array::all(m_local.clone(), std::sync::Arc::new(fnptr!(List::hasOneElement, _)));
            eqn_arr_idx = mapping.eqn_StA.borrow()[(listHead(var_field!((**node).eqn_indices, SuperNode::ARRAY_BUCKET).clone())?-1) as usize].clone();
            var_arr_idx = mapping.var_StA.borrow()[(matching.eqn_to_var[(listHead(var_field!((**node).eqn_indices, SuperNode::ARRAY_BUCKET).clone())?-1) as usize].clone()-1) as usize].clone();
            StrongComponent::createPseudoSlice(var_arr_idx.clone(), eqn_arr_idx.clone(), var_field!((**node).cref_to_solve, SuperNode::ARRAY_BUCKET).clone(), sorted_body_indices.clone(), matching.eqn_to_var.clone(), eqns.clone(), mapping.clone(), indep.clone())?
        },
        _ if (List::all(node_comp.clone(), (std::sync::Arc::new(fnptr!(isArrayBucket, Arc<SuperNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SuperNode>) -> Result<bool> + 'static>))) => {
            let mut m_local: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut matching_local: Arc<Matching::NBMatching> = Arc::new(<Matching::NBMatching as ::std::default::Default>::default());
            let mut map_back: metamodelica::Array<i32>;
            (m_local, matching_local, map_back) = BackendUtil::getLocalSystem(m.clone(), matching.clone(), List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut n in (node_comp.clone()).into_iter().cloned() {
            let __x = getEqnIndices(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            sorted_body_components = tarjanScalar(m_local.clone(), matching_local.clone())?;
            sorted_body_indices = List::flatten(sorted_body_components.clone());
            sorted_body_indices = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (sorted_body_indices.clone()).into_iter().cloned() {
            let __x = map_back.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if List::compareLength(sorted_body_components.clone(), sorted_body_indices.clone())? == 0 {
                comp = StrongComponent::createPseudoEntwined(sorted_body_indices.clone(), matching.eqn_to_var.clone(), mapping.clone(), vars.clone(), eqns.clone(), node_comp.clone())?;
            } else {
                comp = StrongComponent::createPseudoScalar(sorted_body_indices.clone(), matching.eqn_to_var.clone(), mapping.clone(), vars.clone(), eqns.clone())?;
            }
            comp.clone()
        },
        _ => {
            sorted_body_indices = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut n in (node_comp.clone()).into_iter().cloned() {
            let __x = getEqnIndices(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            StrongComponent::createPseudoScalar(sorted_body_indices.clone(), matching.eqn_to_var.clone(), mapping.clone(), vars.clone(), eqns.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        Ok(comp)
    }

    fn mergeRows(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: metamodelica::Array<i32>, mut super_nodes: metamodelica::Array<Arc<SuperNode>>, mut rows_to_merge: Arc<metamodelica::List<i32>>, mut new_idx: i32) -> Result<i32> {
        let mut new_idx: i32 = new_idx;
        {let _arr = m.clone(); let _val = UnorderedSet::unique_list(List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut idx in (rows_to_merge.clone()).into_iter().cloned() {
            let __x = m.borrow()[(idx.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)); _arr.borrow_mut()[(new_idx.clone()-1) as usize] = _val; _arr};
        for mut idx in &*rows_to_merge.clone() {
            let mut idx = idx.clone();
            {let _arr = m.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = metamodelica::nil(); _arr};
            {let _arr = matching.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = -1; _arr};
        }
        new_idx = new_idx.clone() + 1;
        Ok(new_idx)
    }

    fn mergeArrayNodes(mut super_nodes: metamodelica::Array<Arc<SuperNode>>, mut cref_to_solve: Arc<ComponentRef::NFComponentRef>, mut rows_to_merge: Arc<metamodelica::List<i32>>, mut new_idx: i32, mut arr_idx: i32, mut update_scalar: bool) -> Result<i32> {
        let mut new_idx: i32 = new_idx;
        {let _arr = super_nodes.clone(); _arr.borrow_mut()[(new_idx.clone()-1) as usize] = Arc::new(SuperNode::ARRAY_BUCKET { index: new_idx.clone(), cref_to_solve: cref_to_solve.clone(), eqn_indices: rows_to_merge.clone(), arr_idx: arr_idx.clone() }); _arr};
        if update_scalar.clone() {
            for mut i in &*rows_to_merge.clone() {
                let mut i = i.clone();
                {let _arr = super_nodes.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = Arc::new(SuperNode::ELEMENT { index: i.clone(), parent: new_idx.clone() }); _arr};
            }
        }
        Ok(new_idx)
    }

    fn mergeLoopNodes(mut super_nodes: metamodelica::Array<Arc<SuperNode>>, mut rows_to_merge: Arc<metamodelica::List<i32>>, mut new_idx: i32, mut update_scalar: bool) -> Result<i32> {
        let mut new_idx: i32 = new_idx;
        {let _arr = super_nodes.clone(); _arr.borrow_mut()[(new_idx.clone()-1) as usize] = Arc::new(SuperNode::ALGEBRAIC_LOOP { index: new_idx.clone(), eqn_indices: rows_to_merge.clone() }); _arr};
        if update_scalar.clone() {
            for mut i in &*rows_to_merge.clone() {
                let mut i = i.clone();
                {let _arr = super_nodes.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = Arc::new(SuperNode::ELEMENT { index: i.clone(), parent: new_idx.clone() }); _arr};
            }
        }
        Ok(new_idx)
    }

}

// ############################################################
//                Protected Functions and Types
// ############################################################
fn strongConnect(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut var_to_eqn: metamodelica::Array<i32>, mut eqn: i32, mut stack: Arc<metamodelica::List<i32>>, mut index: i32, mut number: metamodelica::Array<i32>, mut lowlink: metamodelica::Array<i32>, mut onStack: metamodelica::Array<bool>, mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut stack: Arc<metamodelica::List<i32>> = stack;
    let mut index: i32 = index;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = comps;
    let mut SCC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqn2: i32 = 0;
    {let _arr = number.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = index.clone(); _arr};
    {let _arr = lowlink.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = index.clone(); _arr};
    {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = true; _arr};
    index = index.clone() + 1;
    stack = cons(eqn.clone(), stack.clone());
    for mut eqn2 in &*predecessors(eqn.clone(), m.clone(), var_to_eqn.clone()) {
        let mut eqn2 = eqn2.clone();
        if number.borrow()[(eqn2.clone()-1) as usize].clone() == -1 {
            (stack, index, comps) = strongConnect(m.clone(), var_to_eqn.clone(), eqn2.clone(), stack.clone(), index.clone(), number.clone(), lowlink.clone(), onStack.clone(), comps.clone())?;
            {let _arr = lowlink.clone(); let _val = intMin(lowlink.borrow()[(eqn.clone()-1) as usize].clone(), lowlink.borrow()[(eqn2.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn.clone()-1) as usize] = _val; _arr};
        } else if onStack.borrow()[(eqn2.clone()-1) as usize].clone() {
            {let _arr = lowlink.clone(); let _val = intMin(lowlink.borrow()[(eqn.clone()-1) as usize].clone(), number.borrow()[(eqn2.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn.clone()-1) as usize] = _val; _arr};
        }
    }
    if lowlink.borrow()[(eqn.clone()-1) as usize].clone() == number.borrow()[(eqn.clone()-1) as usize].clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(stack.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        eqn2 = __pa0.clone();
        stack = __pa1.clone();
        {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn2.clone()-1) as usize] = false; _arr};
        SCC = list![eqn2.clone()];
        while eqn.clone() != eqn2.clone() {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(stack.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eqn2 = __pa2.clone();
            stack = __pa3.clone();
            {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn2.clone()-1) as usize] = false; _arr};
            SCC = cons(eqn2.clone(), SCC.clone());
        }
        comps = cons(metamodelica::Dangerous::listReverseInPlace(SCC.clone()), comps.clone());
    }
    Ok((stack, index, comps))
}

fn predecessors(mut idx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapping: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut pre_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    pre_lst = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut cand in (m.borrow()[(idx.clone()-1) as usize].clone()).into_iter().cloned() {
            if !(cand.clone() > 0 && mapping.borrow()[(cand.clone()-1) as usize].clone() != idx.clone() && mapping.borrow()[(cand.clone()-1) as usize].clone() > 0) { continue; }
            let __x = mapping.borrow()[(cand.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    pre_lst
}

