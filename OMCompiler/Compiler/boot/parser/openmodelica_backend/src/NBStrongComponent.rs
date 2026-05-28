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
use crate::NBAdjacency::Mapping;
use crate::NBCausalize as Causalize;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBInitialization as Initialization;
use crate::NBInline as Inline;
use crate::NBJacobian::JacobianType;
use crate::NBMatching as Matching;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBResizable as Resizable;
use crate::NBResizable::EvalOrder;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBSorting as Sorting;
use crate::NBSorting::SuperNode;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBStrongComponent.mo
/// package:     NBStrongComponent
/// description: This file contains the data-types used save the strong Component
///              data after causalization.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NBStrongComponent {
    /// component for all equations that solve for a single (possibly multidimensional) variable
    ///    SCALAR_EQUATION, ARRAY_EQUATION, RECORD_EQUATION.
    SINGLE_COMPONENT {
        var: Pointer::Pointer<Arc<Variable::NFVariable>>,
        eqn: Pointer::Pointer<Arc<Equation::Equation>>,
        status: Solve::Status,
    },
    /// component for all equations that can solve for more than one variable instance
    ///    ALGORITHM, WHEN_EQUATION, IF_EQUATION
    MULTI_COMPONENT {
        vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>,
        eqn: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>,
        status: Solve::Status,
    },
    /// component for all equations AND/OR variables that need to be sliced (zero based indices)
    SLICED_COMPONENT {
        /// cref to solve for
        var_cref: Arc<ComponentRef::NFComponentRef>,
        /// sliced variable
        var: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>,
        /// sliced equation
        eqn: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>,
        status: Solve::Status,
    },
    /// component for for-equations with trivial evaluation order
    RESIZABLE_COMPONENT {
        /// cref to solve for
        var_cref: Arc<ComponentRef::NFComponentRef>,
        /// sliced variable
        var: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>,
        /// sliced equation
        eqn: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>,
        /// independent, forward, backward
        order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, EvalOrder>>,
        status: Solve::Status,
    },
    /// component for all equations that need to be sliced but where no for-loop could be recovered
    ///    has no status since this is generated by the Solve module and is always status=EXPLICIT.
    GENERIC_COMPONENT {
        /// cref to solve for
        var_cref: Arc<ComponentRef::NFComponentRef>,
        /// sliced variable
        var: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>,
        /// sliced equation
        eqn: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>,
    },
    /// component for entwined SLICED_COMPONENT or GENERIC_COMPONENT
    ///    the body equations have to be called in a specific pattern but do not form an algebraic loop
    ENTWINED_COMPONENT {
        /// has to be SLICED_COMPONENT()
        entwined_slices: Arc<metamodelica::List<Arc<NBStrongComponent>>>,
        /// equation with scalar idx (0 based) - fallback scalarization
        entwined_tpl_lst: Arc<metamodelica::List<(Pointer::Pointer<Arc<Equation::Equation>>, i32)>>,
    },
    /// component for equations that have to be solved as a system.
    ALGEBRAIC_LOOP {
        idx: i32,
        strict: Arc<Tearing::NBTearing>,
        casual: Option<Arc<Tearing::NBTearing>>,
        /// true if the loop is linear
        linear: bool,
        /// true for systems that have discrete variables
        mixed: bool,
        /// true if contains homotopy()
        homotopy: bool,
        status: Solve::Status,
    },
    /// Component representing equal strong components in ODE<->INIT<->DAE
    ///    has no status since this is generated by the Solve module and is always status=EXPLICIT.
    ALIAS {
        /// The strong component array and index it refers to
        aliasInfo: Arc<AliasInfo::AliasInfo>,
        /// The original strong component for analysis
        original: Arc<NBStrongComponent>,
    },
}
impl Default for NBStrongComponent {
    fn default() -> Self {
        Self::ENTWINED_COMPONENT {
            entwined_slices: Default::default(),
            entwined_tpl_lst: Default::default(),
        }
    }
}
pub use self::NBStrongComponent::{SINGLE_COMPONENT,MULTI_COMPONENT,SLICED_COMPONENT,RESIZABLE_COMPONENT,GENERIC_COMPONENT,ENTWINED_COMPONENT,ALGEBRAIC_LOOP,ALIAS};
pub mod AliasInfo {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AliasInfo {
        /// The partition kind
        pub kind: BPartition::Kind,
        /// the partition index
        pub partitionIndex: i32,
        /// The index in that strong component array
        pub componentIndex: i32,
    }

    impl Default for AliasInfo {
        fn default() -> Self {
            Self {
                kind: Default::default(),
                partitionIndex: Default::default(),
                componentIndex: Default::default(),
            }
        }
    }

    pub type ALIAS_INFO = AliasInfo;

    pub fn toString(mut info: Arc<AliasInfo>) -> ArcStr {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*BPartition::Partition::kindToString(info.kind.clone()).unwrap()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(info.partitionIndex.clone())); __mm_s.push_str(&*literal!(" | ")); __mm_s.push_str(&*intString(info.componentIndex.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
        r#str
    }

    pub fn hash(mut info: Arc<AliasInfo>) -> i32 {
        let mut i: i32 = stringHashDjb2((toString(info.clone())).clone());
        i
    }

    pub fn isEqual(mut info1: Arc<AliasInfo>, mut info2: Arc<AliasInfo>) -> bool {
        let mut b: bool = info1.componentIndex.clone() == info2.componentIndex.clone() && info1.partitionIndex.clone() == info2.partitionIndex.clone() && info1.kind.clone() == info2.kind.clone();
        b
    }

}

pub fn toString(mut comp: Arc<NBStrongComponent>, mut index: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: i32 = size(comp.clone(), true)?;
    let mut indexStr: ArcStr = if (index.clone() > 0) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(index.clone())); ArcStr::from(__mm_s) }} else {literal!("")};
    r#str = ((::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => {
            r#str = (StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLOCK")); __mm_s.push_str(&*indexStr.clone()); __mm_s.push_str(&*literal!(": Single Strong Component (status = ")); __mm_s.push_str(&*Solve::statusString(var_field!((*comp).status, NBStrongComponent::SINGLE_COMPONENT).clone())); __mm_s.push_str(&*literal!(", size = ")); __mm_s.push_str(&*intString(s.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Variable:\n")); __mm_s.push_str(&*Variable::toString(Pointer::access(var_field!((*comp).var, NBStrongComponent::SINGLE_COMPONENT).clone()), (literal!("\t")).clone(), false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Equation:\n")); __mm_s.push_str(&*Equation::toString(Pointer::access(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone()), (literal!("\t")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ MULTI_COMPONENT { .. } => {
            r#str = (StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLOCK")); __mm_s.push_str(&*indexStr.clone()); __mm_s.push_str(&*literal!(": Multi Strong Component (status = ")); __mm_s.push_str(&*Solve::statusString(var_field!((*comp).status, NBStrongComponent::MULTI_COMPONENT).clone())); __mm_s.push_str(&*literal!(", size = ")); __mm_s.push_str(&*intString(s.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Variables:\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(var_field!((*comp).vars, NBStrongComponent::MULTI_COMPONENT).clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>); let __pe_b2 = 10; move |__pe_a0| Slice::toString(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n### Equation:\n")); __mm_s.push_str(&*Slice::toString(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ SLICED_COMPONENT { .. } => {
            r#str = (if (index.clone() == -2) {literal!("")} else {StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLOCK")); __mm_s.push_str(&*indexStr.clone()); __mm_s.push_str(&*literal!(": Sliced Component (status = ")); __mm_s.push_str(&*Solve::statusString(var_field!((*comp).status, NBStrongComponent::SLICED_COMPONENT).clone())); __mm_s.push_str(&*literal!(", size = ")); __mm_s.push_str(&*intString(s.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Variable:\n\t")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Equation:\n")); __mm_s.push_str(&*Slice::toString(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ RESIZABLE_COMPONENT { .. } => {
            r#str = (StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLOCK")); __mm_s.push_str(&*indexStr.clone()); __mm_s.push_str(&*literal!(": Resizable Component (status = ")); __mm_s.push_str(&*Solve::statusString(var_field!((*comp).status, NBStrongComponent::RESIZABLE_COMPONENT).clone())); __mm_s.push_str(&*literal!(", size = ")); __mm_s.push_str(&*intString(s.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Variable:\n\t")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*comp).var_cref, NBStrongComponent::RESIZABLE_COMPONENT).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Equation:\n\t")); __mm_s.push_str(&*Equation::pointerToString(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone()), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ ENTWINED_COMPONENT { .. } => {
            r#str = (StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLOCK")); __mm_s.push_str(&*indexStr.clone()); __mm_s.push_str(&*literal!(": Entwined Component (status = Solve.EXPLICIT, size = ")); __mm_s.push_str(&*intString(s.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("call order: ")); __mm_s.push_str(&*List::toString({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut e in (var_field!((*comp).entwined_tpl_lst, NBStrongComponent::ENTWINED_COMPONENT).clone()).into_iter().cloned() {
            let __x = Equation::getEqnName(Util::tuple21(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 10)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(var_field!((*comp).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone(), Arc::new({ let __pe_b1 = -2; move |__pe_a0| toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ GENERIC_COMPONENT { .. } => {
            r#str = (StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLOCK")); __mm_s.push_str(&*indexStr.clone()); __mm_s.push_str(&*literal!(": Generic Component (status = Solve.EXPLICIT, size = ")); __mm_s.push_str(&*intString(s.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Variable:\n\t")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*comp).var_cref, NBStrongComponent::GENERIC_COMPONENT).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("### Equation:\n")); __mm_s.push_str(&*Slice::toString(var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ ALGEBRAIC_LOOP { .. } => {
            r#str = (StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLOCK")); __mm_s.push_str(&*indexStr.clone()); __mm_s.push_str(&*literal!(": Algebraic Loop (Linear = ")); __mm_s.push_str(&*boolString(var_field!((*comp).linear, NBStrongComponent::ALGEBRAIC_LOOP).clone())); __mm_s.push_str(&*literal!(", Mixed = ")); __mm_s.push_str(&*boolString(var_field!((*comp).mixed, NBStrongComponent::ALGEBRAIC_LOOP).clone())); __mm_s.push_str(&*literal!(", Homotopy = ")); __mm_s.push_str(&*boolString(var_field!((*comp).homotopy, NBStrongComponent::ALGEBRAIC_LOOP).clone())); __mm_s.push_str(&*literal!(", size = ")); __mm_s.push_str(&*intString(s.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Tearing::toString(var_field!((*comp).strict, NBStrongComponent::ALGEBRAIC_LOOP).clone(), (literal!("Strict Tearing Set")).clone())?); ArcStr::from(__mm_s) }).clone();
            if isSome(var_field!((*comp).casual, NBStrongComponent::ALGEBRAIC_LOOP).clone()) {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Tearing::toString(Util::getOption(var_field!((*comp).casual, NBStrongComponent::ALGEBRAIC_LOOP).clone())?, (literal!("Casual Tearing Set")).clone())?); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        Deref @ ALIAS { .. } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--- Alias of ")); __mm_s.push_str(&*AliasInfo::toString(var_field!((*comp).aliasInfo, NBStrongComponent::ALIAS).clone())); __mm_s.push_str(&*literal!(" ---\n")); __mm_s.push_str(&*toString(var_field!((*comp).original, NBStrongComponent::ALIAS).clone(), index.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.toString")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountCollector {
    pub single_scalar: i32,
    pub single_array: i32,
    pub single_record: i32,
    pub multi_algorithm: i32,
    pub multi_when: i32,
    pub multi_if: i32,
    pub multi_tpl: i32,
    pub resizable_for: i32,
    pub generic_for: i32,
    pub entwined_for: i32,
    pub loop_lin: i32,
    pub loop_nlin: i32,
}

pub type COUNT_COLLECTOR = CountCollector;


pub fn strongComponentInfo(mut comp: Arc<NBStrongComponent>, mut collector_ptr: Pointer::Pointer<CountCollector>) -> Result<Arc<NBStrongComponent>> {
    let mut comp: Arc<NBStrongComponent> = comp;
    let mut collector: CountCollector = Pointer::access(collector_ptr.clone());
    let _ = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => {
            let _ = (::match_deref::match_deref! { match &(Pointer::access(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone())) {
        Deref @ Equation::SCALAR_EQUATION { .. } => {
            assign_field!(collector.single_scalar = collector.single_scalar.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ Equation::ARRAY_EQUATION { .. } => {
            assign_field!(collector.single_array = collector.single_array.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ Equation::RECORD_EQUATION { .. } => {
            assign_field!(collector.single_record = collector.single_record.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        _ => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot classify strong component:\n")); __mm_s.push_str(&*toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ MULTI_COMPONENT { .. } => {
            let _ = (::match_deref::match_deref! { match &(Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone()))) {
        Deref @ Equation::ALGORITHM { .. } => {
            assign_field!(collector.multi_algorithm = collector.multi_algorithm.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ Equation::WHEN_EQUATION { .. } => {
            assign_field!(collector.multi_when = collector.multi_when.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ Equation::IF_EQUATION { .. } => {
            assign_field!(collector.multi_if = collector.multi_if.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ Equation::RECORD_EQUATION { .. } => {
            assign_field!(collector.multi_tpl = collector.multi_tpl.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        _ => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot classify strong component:\n")); __mm_s.push_str(&*toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ SLICED_COMPONENT { .. } => {
            let _ = (::match_deref::match_deref! { match &(Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone()))) {
        Deref @ Equation::SCALAR_EQUATION { .. } => {
            assign_field!(collector.single_scalar = collector.single_scalar.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ Equation::ARRAY_EQUATION { .. } => {
            assign_field!(collector.single_array = collector.single_array.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ Equation::RECORD_EQUATION { .. } => {
            assign_field!(collector.single_record = collector.single_record.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        _ => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot classify strong component:\n")); __mm_s.push_str(&*toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ RESIZABLE_COMPONENT { .. } => {
            assign_field!(collector.resizable_for = collector.resizable_for.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ GENERIC_COMPONENT { .. } => {
            assign_field!(collector.generic_for = collector.generic_for.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ ENTWINED_COMPONENT { .. } => {
            assign_field!(collector.entwined_for = collector.entwined_for.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ ALGEBRAIC_LOOP { .. } if (var_field!((*comp).linear, NBStrongComponent::ALGEBRAIC_LOOP).clone()) => {
            assign_field!(collector.loop_lin = collector.loop_lin.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ ALGEBRAIC_LOOP { .. } => {
            assign_field!(collector.loop_nlin = collector.loop_nlin.clone() + 1);
            Pointer::update(collector_ptr.clone(), collector.clone());
            ()
        },
        Deref @ ALIAS { .. } => {
            strongComponentInfo(var_field!((*comp).original, NBStrongComponent::ALIAS).clone(), collector_ptr.clone())?;
            ()
        },
        _ => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot classify strong component:\n")); __mm_s.push_str(&*toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub fn hash(mut comp: Arc<NBStrongComponent>) -> Result<i32> {
    let mut i: i32 = 0;
    i = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => BVariable::hash(var_field!((*comp).var, NBStrongComponent::SINGLE_COMPONENT).clone()) + Equation::hash(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone()),
        Deref @ MULTI_COMPONENT { .. } => Equation::hash(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone())),
        Deref @ SLICED_COMPONENT { .. } => ComponentRef::hash(var_field!((*comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone()) + Equation::hash(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone())),
        Deref @ RESIZABLE_COMPONENT { .. } => ComponentRef::hash(var_field!((*comp).var_cref, NBStrongComponent::RESIZABLE_COMPONENT).clone()) + Equation::hash(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone())),
        Deref @ GENERIC_COMPONENT { .. } => Equation::hash(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone())),
        Deref @ ENTWINED_COMPONENT { .. } => {
        let mut __acc: i32 = 0;
        for mut sub_comp in (var_field!((*comp).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone()).into_iter().cloned() {
            let __x = hash(sub_comp.clone())?;
            __acc += __x;
        }
        __acc
    },
        Deref @ ALGEBRAIC_LOOP { .. } => Tearing::hash(var_field!((*comp).strict, NBStrongComponent::ALGEBRAIC_LOOP).clone()),
        Deref @ ALIAS { .. } => AliasInfo::hash(var_field!((*comp).aliasInfo, NBStrongComponent::ALIAS).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(i)
}

pub fn isEqual(mut comp1: Arc<NBStrongComponent>, mut comp2: Arc<NBStrongComponent>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((comp1.clone(), comp2.clone())) {
        (Deref @ SINGLE_COMPONENT { .. }, Deref @ SINGLE_COMPONENT { .. }) => BVariable::equalName(var_field!((*comp1).var, NBStrongComponent::SINGLE_COMPONENT).clone(), var_field!((*comp2).var, NBStrongComponent::SINGLE_COMPONENT).clone()) && Equation::isEqualPtr(var_field!((*comp1).eqn, NBStrongComponent::SINGLE_COMPONENT).clone(), var_field!((*comp2).eqn, NBStrongComponent::SINGLE_COMPONENT).clone()),
        (Deref @ MULTI_COMPONENT { .. }, Deref @ MULTI_COMPONENT { .. }) => Equation::isEqualPtr(Slice::getT(var_field!((*comp1).eqn, NBStrongComponent::MULTI_COMPONENT).clone()), Slice::getT(var_field!((*comp2).eqn, NBStrongComponent::MULTI_COMPONENT).clone())),
        (Deref @ SLICED_COMPONENT { .. }, Deref @ SLICED_COMPONENT { .. }) => ComponentRef::isEqual(var_field!((*comp1).var_cref, NBStrongComponent::SLICED_COMPONENT).clone(), var_field!((*comp2).var_cref, NBStrongComponent::SLICED_COMPONENT).clone())? && Slice::isEqual(var_field!((*comp1).eqn, NBStrongComponent::SLICED_COMPONENT).clone(), var_field!((*comp2).eqn, NBStrongComponent::SLICED_COMPONENT).clone(), (std::sync::Arc::new(fnptr!(Equation::isEqualPtr, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>)),
        (Deref @ RESIZABLE_COMPONENT { .. }, Deref @ RESIZABLE_COMPONENT { .. }) => ComponentRef::isEqual(var_field!((*comp1).var_cref, NBStrongComponent::RESIZABLE_COMPONENT).clone(), var_field!((*comp2).var_cref, NBStrongComponent::RESIZABLE_COMPONENT).clone())? && Slice::isEqual(var_field!((*comp1).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone(), var_field!((*comp2).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone(), (std::sync::Arc::new(fnptr!(Equation::isEqualPtr, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>)),
        (Deref @ GENERIC_COMPONENT { .. }, Deref @ GENERIC_COMPONENT { .. }) => Slice::isEqual(var_field!((*comp1).eqn, NBStrongComponent::GENERIC_COMPONENT).clone(), var_field!((*comp2).eqn, NBStrongComponent::GENERIC_COMPONENT).clone(), (std::sync::Arc::new(fnptr!(Equation::isEqualPtr, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>)),
        (Deref @ ENTWINED_COMPONENT { .. }, Deref @ ENTWINED_COMPONENT { .. }) => List::isEqualOnTrue(var_field!((*comp1).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone(), var_field!((*comp2).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone(), (std::sync::Arc::new(isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NBStrongComponent>, Arc<NBStrongComponent>) -> Result<bool> + 'static>)),
        (Deref @ ALGEBRAIC_LOOP { .. }, Deref @ ALGEBRAIC_LOOP { .. }) => Tearing::isEqual(var_field!((*comp1).strict, NBStrongComponent::ALGEBRAIC_LOOP).clone(), var_field!((*comp2).strict, NBStrongComponent::ALGEBRAIC_LOOP).clone())?,
        (Deref @ ALIAS { .. }, Deref @ ALIAS { .. }) => AliasInfo::isEqual(var_field!((*comp1).aliasInfo, NBStrongComponent::ALIAS).clone(), var_field!((*comp2).aliasInfo, NBStrongComponent::ALIAS).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn size(mut comp: Arc<NBStrongComponent>, mut resize: bool) -> Result<i32> {
    let mut s: i32 = 0;
    s = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => Equation::size(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone(), resize.clone())?,
        Deref @ MULTI_COMPONENT { .. } => Slice::size(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone(), Arc::new({ let __pe_b1 = resize.clone(); move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone()) })),
        Deref @ SLICED_COMPONENT { .. } => Slice::size(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone(), Arc::new({ let __pe_b1 = resize.clone(); move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone()) })),
        Deref @ RESIZABLE_COMPONENT { .. } => Slice::size(var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone(), Arc::new({ let __pe_b1 = resize.clone(); move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone()) })),
        Deref @ GENERIC_COMPONENT { .. } => Slice::size(var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone(), Arc::new({ let __pe_b1 = resize.clone(); move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone()) })),
        Deref @ ENTWINED_COMPONENT { .. } => {
        let mut __acc: i32 = 0;
        for mut c in (var_field!((*comp).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone()).into_iter().cloned() {
            let __x = size(c.clone(), resize.clone())?;
            __acc += __x;
        }
        __acc
    },
        Deref @ ALGEBRAIC_LOOP { .. } => Tearing::size(var_field!((*comp).strict, NBStrongComponent::ALGEBRAIC_LOOP).clone(), resize.clone())?,
        Deref @ ALIAS { .. } => size(var_field!((*comp).original, NBStrongComponent::ALIAS).clone(), resize.clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.size")); __mm_s.push_str(&*literal!(" failed. Cannot determine size of strong component:\n")); __mm_s.push_str(&*toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn removeAlias(mut comp: Arc<NBStrongComponent>) -> Arc<NBStrongComponent> {
    let mut comp: Arc<NBStrongComponent> = comp;
    comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ ALIAS { .. } => var_field!((*comp).original, NBStrongComponent::ALIAS).clone(),
        _ => comp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    comp
}

pub fn createPseudoSlice(mut var_arr_idx: i32, mut eqn_arr_idx: i32, mut cref_to_solve: Arc<ComponentRef::NFComponentRef>, mut eqn_scal_indices: Arc<metamodelica::List<i32>>, mut eqn_to_var: metamodelica::Array<i32>, mut eqns: Arc<EquationPointers::EquationPointers>, mut mapping: Arc<Mapping::Mapping>, mut independent: bool) -> Result<Arc<NBStrongComponent>> {
    let mut comp: Arc<NBStrongComponent>;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut first_var: i32 = 0;
    let mut var_size: i32 = 0;
    let mut first_eqn: i32 = 0;
    let mut eqn_size: i32 = 0;
    let mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
    let mut var_scal_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, EvalOrder>>;
    var_ptr = BVariable::getVarPointer(cref_to_solve.clone(), metamodelica::sourceInfo!())?;
    eqn_ptr = EquationPointers::getEqnAt(eqns.clone(), eqn_arr_idx.clone())?;
    (first_var, var_size) = mapping.var_AtS.borrow()[(var_arr_idx.clone()-1) as usize].clone();
    (first_eqn, eqn_size) = mapping.eqn_AtS.borrow()[(eqn_arr_idx.clone()-1) as usize].clone();
    var_scal_indices = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (eqn_scal_indices.clone()).into_iter().cloned() {
            let __x = eqn_to_var.borrow()[(e.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if independent.clone() && Equation::isArrayEquation(eqn_ptr.clone()) && (eqn_scal_indices.clone().len() as i32) == eqn_size.clone() && (var_scal_indices.clone().len() as i32) == var_size.clone() {
        var_slice = Arc::new(Slice::NBSlice { t: var_ptr.clone(), indices: metamodelica::nil() });
        eqn_slice = Arc::new(Slice::NBSlice { t: eqn_ptr.clone(), indices: metamodelica::nil() });
    } else {
        var_slice = Arc::new(Slice::NBSlice { t: var_ptr.clone(), indices: {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut idx in (var_scal_indices.clone()).into_iter().cloned() {
            let __x = idx.clone() - first_var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
        eqn_slice = Arc::new(Slice::NBSlice { t: eqn_ptr.clone(), indices: {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut idx in (eqn_scal_indices.clone()).into_iter().cloned() {
            let __x = idx.clone() - first_eqn.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
    }
    order = Resizable::detect(Pointer::access(eqn_ptr.clone()), cref_to_solve.clone())?;
    if !(List::any(UnorderedMap::valueList(order.clone()), (std::sync::Arc::new(fnptr!(Resizable::orderFailed, EvalOrder)) as std::sync::Arc<dyn ::std::ops::Fn(EvalOrder) -> Result<bool> + 'static>))) && (eqn_scal_indices.clone().len() as i32) == eqn_size.clone() {
        comp = Arc::new(NBStrongComponent::RESIZABLE_COMPONENT { status: Solve::Status::UNPROCESSED.clone(), order: order.clone(), eqn: eqn_slice.clone(), var: var_slice.clone(), var_cref: cref_to_solve.clone() });
    } else {
        comp = createSliceOrSingle(cref_to_solve.clone(), var_slice.clone(), eqn_slice.clone());
    }
    Ok(comp)
}

pub fn createPseudoEntwined(mut eqn_indices: Arc<metamodelica::List<i32>>, mut eqn_to_var: metamodelica::Array<i32>, mut mapping: Arc<Mapping::Mapping>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>, mut nodes: Arc<metamodelica::List<Arc<SuperNode::SuperNode>>>) -> Result<Arc<NBStrongComponent>> {
    let mut entwined: Arc<NBStrongComponent>;
    let mut elem_map: Arc<UnorderedMap::UnorderedMap<i32, Arc<metamodelica::List<i32>>>> = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 1);
    let mut cref_map: Arc<UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 1);
    let mut flat_map: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut eqn_arr_idx: i32 = 0;
    let mut var_arr_idx: i32 = 0;
    let mut scal_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut entwined_slices: Arc<metamodelica::List<Arc<NBStrongComponent>>> = metamodelica::nil();
    let mut entwined_tpl_lst: Arc<metamodelica::List<(Pointer::Pointer<Arc<Equation::Equation>>, i32)>> = metamodelica::nil();
    for mut idx in &*eqn_indices.clone() {
        let mut idx = idx.clone();
        UnorderedMap::add(mapping.eqn_StA.borrow()[(idx.clone()-1) as usize].clone(), cons(idx.clone(), UnorderedMap::getOrDefault(mapping.eqn_StA.borrow()[(idx.clone()-1) as usize].clone(), elem_map.clone(), metamodelica::nil())), elem_map.clone())?;
    }
    for mut node in &*nodes.clone() {
        let mut node = node.clone();
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ Sorting::SuperNode::ARRAY_BUCKET { .. } => {
            UnorderedMap::add(var_field!((*node).arr_idx, SuperNode::SuperNode::ARRAY_BUCKET).clone(), var_field!((*node).cref_to_solve, SuperNode::SuperNode::ARRAY_BUCKET).clone(), cref_map.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    for mut tpl in &*UnorderedMap::toList(elem_map.clone()) {
        let mut tpl = tpl.clone();
        (eqn_arr_idx, scal_indices) = tpl.clone();
        var_arr_idx = mapping.var_StA.borrow()[(eqn_to_var.borrow()[(Util::tuple21(mapping.eqn_AtS.borrow()[(eqn_arr_idx.clone()-1) as usize].clone())-1) as usize].clone()-1) as usize].clone();
        entwined_slices = cons(createPseudoSlice(var_arr_idx.clone(), eqn_arr_idx.clone(), UnorderedMap::getSafe(eqn_arr_idx.clone(), cref_map.clone(), metamodelica::sourceInfo!())?, scal_indices.clone(), eqn_to_var.clone(), eqns.clone(), mapping.clone(), false)?, entwined_slices.clone());
    }
    entwined_tpl_lst = {
        let mut __acc: Arc<metamodelica::List<(Pointer::Pointer<Arc<Equation::Equation>>, i32)>> = metamodelica::nil();
        for mut idx in (eqn_indices.clone()).into_iter().cloned() {
            let __x = (EquationPointers::getEqnAt(eqns.clone(), mapping.eqn_StA.borrow()[(idx.clone()-1) as usize].clone())?, idx.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    entwined = Arc::new(NBStrongComponent::ENTWINED_COMPONENT { entwined_slices: entwined_slices.clone(), entwined_tpl_lst: entwined_tpl_lst.clone() });
    Ok(entwined)
}

pub fn createAlias(mut kind: BPartition::Kind, mut partitionIndex: i32, mut index_ptr: Pointer::Pointer<i32>, mut orig_comp: Arc<NBStrongComponent>) -> Arc<NBStrongComponent> {
    let mut alias_comp: Arc<NBStrongComponent>;
    alias_comp = Arc::new(NBStrongComponent::ALIAS { aliasInfo: Arc::new(AliasInfo::AliasInfo { kind: kind.clone(), partitionIndex: partitionIndex.clone(), componentIndex: Pointer::access(index_ptr.clone()) }), original: orig_comp.clone() });
    Pointer::update(index_ptr.clone(), Pointer::access(index_ptr.clone()) + 1);
    alias_comp
}

pub fn createPseudoEntwinedIndices(mut entwined_indices: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqns: Arc<EquationPointers::EquationPointers>, mut mapping: Arc<Mapping::Mapping>) -> Result<Arc<metamodelica::List<(Pointer::Pointer<Arc<Equation::Equation>>, i32)>>> {
    let mut flat_tpl_indices: Arc<metamodelica::List<(Pointer::Pointer<Arc<Equation::Equation>>, i32)>> = metamodelica::nil();
    let mut arr_idx: i32 = 0;
    let mut first_idx: i32 = 0;
    let mut eqn_StA: metamodelica::Array<i32>;
    let __range0 = entwined_indices.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut tmp in __range0 {
        for mut scal_idx in &*tmp.clone() {
            let mut scal_idx = scal_idx.clone();
            eqn_StA = mapping.eqn_StA.clone();
            arr_idx = eqn_StA.borrow()[(scal_idx.clone()-1) as usize].clone();
            (first_idx, _) = mapping.eqn_AtS.borrow()[(arr_idx.clone()-1) as usize].clone();
            flat_tpl_indices = cons((EquationPointers::getEqnAt(eqns.clone(), arr_idx.clone())?, scal_idx.clone() - first_idx.clone()), flat_tpl_indices.clone());
        }
    }
    flat_tpl_indices = flat_tpl_indices.clone().reverse();
    Ok(flat_tpl_indices)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum DAEType {
    UNPROCESSED = 1,
    REMOVED = 2,
    INNER = 3,
    RESIDUAL = 4,
}
impl PartialOrd for DAEType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for DAEType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn sortDAEModeComponents(mut comps: Option<metamodelica::Array<Arc<NBStrongComponent>>>, mut variables: Arc<VariablePointers::VariablePointers>, mut uniqueIndex: Pointer::Pointer<i32>) -> Result<Option<metamodelica::Array<Arc<NBStrongComponent>>>> {
    let mut comps: Option<metamodelica::Array<Arc<NBStrongComponent>>> = comps;
    let mut residuals: Arc<metamodelica::List<Arc<NBStrongComponent>>> = metamodelica::nil();
    let mut inners: Arc<metamodelica::List<Arc<NBStrongComponent>>> = metamodelica::nil();
    let mut slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    comps = (match comps.clone() {
        Some(mut original) => {
            let __range0 = original.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                (residuals, inners) = sortDAEModeComponent(comp.clone(), residuals.clone(), inners.clone(), variables.clone(), uniqueIndex.clone(), slice_set.clone())?;
            }
            comps = Some(metamodelica::arrayFromVec(listAppend(inners.clone().reverse(), residuals.clone()).into_iter().cloned().collect()));
            comps.clone()
        },
        _ => {
            comps.clone()
        },
    });
    Ok(comps)
}

pub fn sortDAEModeComponent(mut comp: Arc<NBStrongComponent>, mut residuals: Arc<metamodelica::List<Arc<NBStrongComponent>>>, mut inners: Arc<metamodelica::List<Arc<NBStrongComponent>>>, mut variables: Arc<VariablePointers::VariablePointers>, mut uniqueIndex: Pointer::Pointer<i32>, mut slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<NBStrongComponent>>>, Arc<metamodelica::List<Arc<NBStrongComponent>>>)> {
    let mut residuals: Arc<metamodelica::List<Arc<NBStrongComponent>>> = residuals;
    let mut inners: Arc<metamodelica::List<Arc<NBStrongComponent>>> = inners;
    let mut new_residuals: Arc<metamodelica::List<Arc<NBStrongComponent>>> = metamodelica::nil();
    let mut dae_type: DAEType = DAEType::UNPROCESSED;
    (new_residuals, dae_type) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => {
            (new_residuals, dae_type) = singleDAEModeComponent(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone(), variables.clone(), uniqueIndex.clone())?;
            (new_residuals.clone(), dae_type.clone())
        },
        Deref @ MULTI_COMPONENT { .. } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(var_field!((*comp).vars, NBStrongComponent::MULTI_COMPONENT).clone(), list![var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone()], variables.clone(), uniqueIndex.clone(), slice_set.clone())?;
            (new_residuals.clone(), dae_type.clone())
        },
        Deref @ SLICED_COMPONENT { .. } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(list![var_field!((*comp).var, NBStrongComponent::SLICED_COMPONENT).clone()], list![var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone()], variables.clone(), uniqueIndex.clone(), slice_set.clone())?;
            (new_residuals.clone(), dae_type.clone())
        },
        Deref @ RESIZABLE_COMPONENT { .. } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(list![var_field!((*comp).var, NBStrongComponent::RESIZABLE_COMPONENT).clone()], list![var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone()], variables.clone(), uniqueIndex.clone(), slice_set.clone())?;
            (new_residuals.clone(), dae_type.clone())
        },
        Deref @ GENERIC_COMPONENT { .. } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(list![var_field!((*comp).var, NBStrongComponent::GENERIC_COMPONENT).clone()], list![var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone()], variables.clone(), uniqueIndex.clone(), slice_set.clone())?;
            (new_residuals.clone(), dae_type.clone())
        },
        Deref @ ALGEBRAIC_LOOP { .. } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(var_field!((*comp).strict, NBStrongComponent::ALGEBRAIC_LOOP).iteration_vars.clone(), var_field!((*comp).strict, NBStrongComponent::ALGEBRAIC_LOOP).residual_eqns.clone(), variables.clone(), uniqueIndex.clone(), slice_set.clone())?;
            (new_residuals.clone(), dae_type.clone())
        },
        _ => (metamodelica::nil(), if (isDiscrete(comp.clone())?) {DAEType::REMOVED.clone()} else {DAEType::INNER.clone()}),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if dae_type.clone() == DAEType::RESIDUAL.clone() {
        residuals = listAppend(new_residuals.clone(), residuals.clone());
    } else if dae_type.clone() == DAEType::INNER.clone() {
        inners = cons(comp.clone(), inners.clone());
    }
    Ok((residuals, inners))
}

pub fn slicedDAEModeComponent(mut var_slices: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut eqn_slices: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut variables: Arc<VariablePointers::VariablePointers>, mut uniqueIndex: Pointer::Pointer<i32>, mut slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<NBStrongComponent>>>, DAEType)> {
    let mut new_residuals: Arc<metamodelica::List<Arc<NBStrongComponent>>> = metamodelica::nil();
    let mut dae_type: DAEType = DAEType::UNPROCESSED;
    let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut eqn_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut acc_new_residuals: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NBStrongComponent>>>>> = metamodelica::nil();
    if List::all({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut v in (var_slices.clone()).into_iter().cloned() {
            let __x = v.indices.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, std::sync::Arc::new(fnptr!(listEmpty, _))) {
        for mut eqn_slice in &*eqn_slices.clone() {
            let mut eqn_slice = eqn_slice.clone();
            eqn = Slice::getT(eqn_slice.clone());
            eqn_name = Equation::getEqnName(eqn.clone())?;
            if eqn_slice.indices.clone().is_empty() && !(UnorderedSet::contains(eqn_name.clone(), slice_set.clone())?) {
                (new_residuals, dae_type) = singleDAEModeComponent(eqn.clone(), variables.clone(), uniqueIndex.clone())?;
                if dae_type.clone() == DAEType::RESIDUAL.clone() {
                    acc_new_residuals = cons(new_residuals.clone(), acc_new_residuals.clone());
                } else if dae_type.clone() == DAEType::INNER.clone() {
                    break;
                }
            } else {
                dae_type = DAEType::INNER.clone();
                break;
            }
        }
    } else {
        dae_type = DAEType::INNER.clone();
    }
    if dae_type.clone() == DAEType::INNER.clone() {
        for mut eqn_slice in &*eqn_slices.clone() {
            let mut eqn_slice = eqn_slice.clone();
            eqn = Slice::getT(eqn_slice.clone());
            eqn_name = Equation::getEqnName(eqn.clone())?;
            UnorderedSet::add(eqn_name.clone(), slice_set.clone())?;
        }
        new_residuals = metamodelica::nil();
    } else {
        new_residuals = List::flatten(acc_new_residuals.clone());
    }
    Ok((new_residuals, dae_type))
}

pub fn singleDAEModeComponent(mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>, mut variables: Arc<VariablePointers::VariablePointers>, mut uniqueIndex: Pointer::Pointer<i32>) -> Result<(Arc<metamodelica::List<Arc<NBStrongComponent>>>, DAEType)> {
    let mut new_residuals: Arc<metamodelica::List<Arc<NBStrongComponent>>> = metamodelica::nil();
    let mut dae_type: DAEType = DAEType::RESIDUAL.clone();
    let mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>;
    let mut dummy_set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    new_eqns = Pointer::create(metamodelica::nil());
    dummy_set = UnorderedSet::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
    eqn = Inline::inlineRecordTupleArrayEquation(Pointer::access(eqn_ptr.clone()), Arc::new(crate::NBEquation::Iterator::EMPTY), variables.clone(), new_eqns.clone(), dummy_set.clone(), uniqueIndex.clone(), true)?;
    eqns = Pointer::access(new_eqns.clone());
    eqns = if (eqns.clone().is_empty()) {list![Pointer::create(eqn.clone())]} else {eqns.clone()};
    (new_residuals, dae_type) = inlinedDAEModeComponent(eqns.clone())?;
    Ok((new_residuals, dae_type))
}

pub fn inlinedDAEModeComponent(mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Result<(Arc<metamodelica::List<Arc<NBStrongComponent>>>, DAEType)> {
    let mut comps: Arc<metamodelica::List<Arc<NBStrongComponent>>> = metamodelica::nil();
    let mut dae_type: DAEType = DAEType::UNPROCESSED.clone();
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut new_comp: Arc<NBStrongComponent>;
    for mut eqn in &*eqns.clone() {
        let mut eqn = eqn.clone();
        if Equation::isDiscrete(eqn.clone()) {
            if dae_type.clone() < DAEType::INNER.clone() {
                dae_type = DAEType::REMOVED.clone();
            }
        } else {
            new_eqn = Equation::createResidual(eqn.clone(), None, false, true)?;
            if Equation::isResidual(new_eqn.clone()) {
                new_comp = Arc::new(NBStrongComponent::SINGLE_COMPONENT { var: Equation::getResidualVar(new_eqn.clone())?, eqn: new_eqn.clone(), status: Solve::Status::UNPROCESSED.clone() });
                comps = cons(new_comp.clone(), comps.clone());
                dae_type = DAEType::RESIDUAL.clone();
            } else {
                dae_type = DAEType::INNER.clone();
                break;
            }
        }
    }
    Ok((comps, dae_type))
}

pub fn fromSolvedEquationSlice(mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Result<Arc<NBStrongComponent>> {
    let mut comp: Arc<NBStrongComponent>;
    let mut eqn: Pointer::Pointer<Arc<Equation::Equation>> = Slice::getT(eqn_slice.clone());
    comp = (::match_deref::match_deref! { match &(Pointer::access(eqn.clone())) {
        Deref @ Equation::SCALAR_EQUATION { .. } => Arc::new(NBStrongComponent::SINGLE_COMPONENT { var: BVariable::getVarPointer(Expression::toCref(Util::getOption(Equation::getLHS(Pointer::access(eqn.clone()))?)?)?, metamodelica::sourceInfo!())?, eqn: eqn.clone(), status: Solve::Status::EXPLICIT.clone() }),
        Deref @ Equation::ARRAY_EQUATION { .. } => Arc::new(NBStrongComponent::SINGLE_COMPONENT { var: BVariable::getVarPointer(Expression::toCref(Util::getOption(Equation::getLHS(Pointer::access(eqn.clone()))?)?)?, metamodelica::sourceInfo!())?, eqn: eqn.clone(), status: Solve::Status::EXPLICIT.clone() }),
        Deref @ Equation::RECORD_EQUATION { .. } => Arc::new(NBStrongComponent::SINGLE_COMPONENT { var: BVariable::getVarPointer(Expression::toCref(Util::getOption(Equation::getLHS(Pointer::access(eqn.clone()))?)?)?, metamodelica::sourceInfo!())?, eqn: eqn.clone(), status: Solve::Status::EXPLICIT.clone() }),
        Deref @ Equation::IF_EQUATION { .. } => Arc::new(NBStrongComponent::SINGLE_COMPONENT { var: BVariable::getVarPointer(Expression::toCref(Util::getOption(Equation::getLHS(Pointer::access(eqn.clone()))?)?)?, metamodelica::sourceInfo!())?, eqn: eqn.clone(), status: Solve::Status::EXPLICIT.clone() }),
        Deref @ Equation::FOR_EQUATION { .. } => Arc::new(NBStrongComponent::SLICED_COMPONENT { var_cref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY), var: Arc::new(Slice::NBSlice { t: Pointer::create(BVariable::DUMMY_VARIABLE().clone()), indices: metamodelica::nil() }), eqn: eqn_slice.clone(), status: Solve::Status::EXPLICIT.clone() }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.fromSolvedEquationSlice")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*Slice::toString(eqn_slice.clone(), Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub fn toSolvedEquation(mut comp: Arc<NBStrongComponent>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
    let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    eqn = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { status: Solve::Status::EXPLICIT, .. } => var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone(),
        Deref @ MULTI_COMPONENT { status: Solve::Status::EXPLICIT, .. } => Slice::getT(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone()),
        Deref @ SLICED_COMPONENT { status: Solve::Status::EXPLICIT, .. } => Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone()),
        Deref @ GENERIC_COMPONENT { .. } => Slice::getT(var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.toSolvedEquation")); __mm_s.push_str(&*literal!(" failed because strong component could not be\n        solved explicitly:\n")); __mm_s.push_str(&*toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

pub fn collectCrefs(mut comp: Arc<NBStrongComponent>, mut var_rep: Arc<VariablePointers::VariablePointers>, mut eqn_rep: Arc<VariablePointers::VariablePointers>, mut var_rep_mapping: Arc<Mapping::Mapping>, mut eqn_rep_mapping: Arc<Mapping::Mapping>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut jacType: JacobianType) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } if (Equation::isArrayEquation(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone())) => {
            let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut scalarized_dependencies: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
            dependencies = Equation::collectCrefs(Pointer::access(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone()), Arc::new({ let __pe_b2 = set.clone(); move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone()) }), (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            scalarized_dependencies = Slice::getDependentCrefsPseudoArrayCausalized(BVariable::getVarName(var_field!((*comp).var, NBStrongComponent::SINGLE_COMPONENT).clone()), dependencies.clone(), metamodelica::nil())?;
            addScalarizedDependencies(scalarized_dependencies.clone(), map.clone(), jacType.clone())?;
            ()
        },
        Deref @ SINGLE_COMPONENT { .. } => {
            let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut deps_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            dependencies = Equation::collectCrefs(Pointer::access(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone()), Arc::new({ let __pe_b2 = set.clone(); move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone()) }), (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            dependencies = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            deps_set = prepareDependencies(UnorderedSet::fromList(dependencies.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?, map.clone(), jacType.clone())?;
            updateDependencyMap(BVariable::getVarName(var_field!((*comp).var, NBStrongComponent::SINGLE_COMPONENT).clone()), deps_set.clone(), map.clone())?;
            ()
        },
        Deref @ MULTI_COMPONENT { .. } => {
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut deps_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            dependencies = Equation::collectCrefs(Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone())), Arc::new({ let __pe_b2 = set.clone(); move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone()) }), (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            dependencies = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = ComponentRef::stripIteratorSubscripts(dep.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            dependencies = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            deps_set = prepareDependencies(UnorderedSet::fromList(dependencies.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?, map.clone(), jacType.clone())?;
            for mut var in &*var_field!((*comp).vars, NBStrongComponent::MULTI_COMPONENT).clone() {
                let mut var = var.clone();
                for mut cref in &*ComponentRef::scalarizeAll(BVariable::getVarName(Slice::getT(var.clone())), true)? {
                    let mut cref = cref.clone();
                    updateDependencyMap(cref.clone(), deps_set.clone(), map.clone())?;
                }
            }
            ()
        },
        Deref @ RESIZABLE_COMPONENT { .. } if (Equation::isForEquation(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone()))) => {
            addForLoopDependencies(Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone())), var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).indices.clone(), var_field!((*comp).var_cref, NBStrongComponent::RESIZABLE_COMPONENT).clone(), var_rep.clone(), eqn_rep.clone(), var_rep_mapping.clone(), eqn_rep_mapping.clone(), map.clone(), set.clone(), jacType.clone())?;
            ()
        },
        Deref @ SLICED_COMPONENT { .. } if (Equation::isForEquation(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone()))) => {
            addForLoopDependencies(Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone())), var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).indices.clone(), var_field!((*comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone(), var_rep.clone(), eqn_rep.clone(), var_rep_mapping.clone(), eqn_rep_mapping.clone(), map.clone(), set.clone(), jacType.clone())?;
            ()
        },
        Deref @ SLICED_COMPONENT { .. } if (Equation::isArrayEquation(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone()))) => {
            let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut scalarized_dependencies: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            eqn = Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone()));
            dependencies = Equation::collectCrefs(eqn.clone(), Arc::new({ let __pe_b2 = set.clone(); move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone()) }), (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            scalarized_dependencies = Slice::getDependentCrefsPseudoArrayCausalized(var_field!((*comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone(), dependencies.clone(), var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).indices.clone())?;
            addScalarizedDependencies(scalarized_dependencies.clone(), map.clone(), jacType.clone())?;
            ()
        },
        Deref @ SLICED_COMPONENT { .. } => {
            let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut deps_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            eqn = Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone()));
            dependencies = Equation::collectCrefs(eqn.clone(), Arc::new({ let __pe_b2 = set.clone(); move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone()) }), (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            dependencies = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            deps_set = prepareDependencies(UnorderedSet::fromList(dependencies.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?, map.clone(), jacType.clone())?;
            updateDependencyMap(var_field!((*comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone(), deps_set.clone(), map.clone())?;
            ()
        },
        Deref @ GENERIC_COMPONENT { .. } if (Equation::isForEquation(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone()))) => {
            addForLoopDependencies(Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone())), var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).indices.clone(), var_field!((*comp).var_cref, NBStrongComponent::GENERIC_COMPONENT).clone(), var_rep.clone(), eqn_rep.clone(), var_rep_mapping.clone(), eqn_rep_mapping.clone(), map.clone(), set.clone(), jacType.clone())?;
            ()
        },
        Deref @ ALGEBRAIC_LOOP { strict, .. } => {
            let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut loop_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut tmp: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut scalarized_dependencies: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
            let mut body: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
            let mut deps_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            deps_set = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            for mut slice in &*strict.residual_eqns.clone() {
                let mut slice = slice.clone();
                tmp = Equation::collectCrefs(Pointer::access(Slice::getT(slice.clone())), Arc::new({ let __pe_b2 = set.clone(); move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone()) }), (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                eqn_ptr = Slice::getT(slice.clone());
                if Equation::isForEquation(eqn_ptr.clone()) {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
                        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, iter: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    body = __pa0.clone();
                    iter = __pa1.clone();
                    cref = Equation::getEqnName(eqn_ptr.clone())?;
                    scalarized_dependencies = Slice::getDependentCrefsPseudoForCausalized(cref.clone(), tmp.clone(), var_rep.clone(), eqn_rep.clone(), var_rep_mapping.clone(), eqn_rep_mapping.clone(), iter.clone(), Equation::size(eqn_ptr.clone(), false)?, slice.indices.clone(), true)?;
                    tmp = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut tpl in (scalarized_dependencies.clone()).into_iter().cloned() {
            let __x = Util::tuple22(tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                }
                for mut dep in &*tmp.clone() {
                    let mut dep = dep.clone();
                    for mut scal in &*ComponentRef::scalarizeAll(dep.clone(), true)? {
                        let mut scal = scal.clone();
                        UnorderedSet::add(scal.clone(), deps_set.clone())?;
                    }
                }
            }
            deps_set = prepareDependencies(deps_set.clone(), map.clone(), jacType.clone())?;
            loop_vars = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
            let __x = BVariable::getVarName(Slice::getT(var.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            let __range3 = 1..=(strict.innerEquations.clone().borrow().len() as i32);
            for mut i in __range3 {
                collectCrefs(strict.innerEquations.borrow()[(i.clone()-1) as usize].clone(), var_rep.clone(), eqn_rep.clone(), var_rep_mapping.clone(), eqn_rep_mapping.clone(), map.clone(), set.clone(), jacType.clone())?;
                loop_vars = listAppend({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (getVariables(strict.innerEquations.borrow()[(i.clone()-1) as usize].clone())?).into_iter().cloned() {
            let __x = BVariable::getVarName(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, loop_vars.clone());
            }
            for mut cref in &*loop_vars.clone() {
                let mut cref = cref.clone();
                updateDependencyMap(cref.clone(), deps_set.clone(), map.clone())?;
            }
            ()
        },
        Deref @ ALIAS { .. } => {
            collectCrefs(var_field!((*comp).original, NBStrongComponent::ALIAS).clone(), var_rep.clone(), eqn_rep.clone(), var_rep_mapping.clone(), eqn_rep_mapping.clone(), map.clone(), set.clone(), jacType.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn addScalarizedDependencies(mut scalarized_dependencies: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut jacType: JacobianType) -> Result<()> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut deps_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    for mut tpl in &*scalarized_dependencies.clone().reverse() {
        let mut tpl = tpl.clone();
        (cref, dependencies) = tpl.clone();
        deps_set = prepareDependencies(UnorderedSet::fromList(dependencies.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?, map.clone(), jacType.clone())?;
        updateDependencyMap(cref.clone(), deps_set.clone(), map.clone())?;
    }
    Ok(())
}

pub fn addForLoopDependencies(mut eqn: Arc<Equation::Equation>, mut indices: Arc<metamodelica::List<i32>>, mut var_cref: Arc<ComponentRef::NFComponentRef>, mut var_rep: Arc<VariablePointers::VariablePointers>, mut eqn_rep: Arc<VariablePointers::VariablePointers>, mut var_rep_mapping: Arc<Mapping::Mapping>, mut eqn_rep_mapping: Arc<Mapping::Mapping>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut jacType: JacobianType) -> Result<()> {
    let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut body: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut scalarized_dependencies: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(eqn.clone()) {
            Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }, iter: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        body = __pa1.clone();
        iter = __pa2.clone();
        Ok::<_, anyhow::Error>((body.clone(), iter.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            body = __try0_o0;
            iter = __try0_o1;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.addForLoopDependencies")); __mm_s.push_str(&*literal!(" failed because the for-loop had more than one body equation:\n")); __mm_s.push_str(&*Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
    }
    dependencies = Equation::collectCrefs(eqn.clone(), Arc::new({ let __pe_b2 = set.clone(); move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone()) }), (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    if ComponentRef::isEmpty(var_cref.clone()) {
        let __pa4 = ::match_deref::match_deref! { match &(Equation::getLHS(body.clone())?) {
            Some(Deref @ Expression::CREF { cref: __pa4, .. }) => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        cref = __pa4.clone();
    } else {
        cref = var_cref.clone();
    }
    scalarized_dependencies = Slice::getDependentCrefsPseudoForCausalized(cref.clone(), dependencies.clone(), var_rep.clone(), eqn_rep.clone(), var_rep_mapping.clone(), eqn_rep_mapping.clone(), iter.clone(), Equation::size(Pointer::create(eqn.clone()), false)?, indices.clone(), false)?;
    addScalarizedDependencies(scalarized_dependencies.clone(), map.clone(), jacType.clone())?;
    Ok(())
}

pub fn addLoopJacobian(mut comp: Arc<NBStrongComponent>, mut jac: Option<Arc<BackendDAE::NBackendDAE>>) -> Result<Arc<NBStrongComponent>> {
    let mut comp: Arc<NBStrongComponent> = comp;
    comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ ALGEBRAIC_LOOP { strict, .. } => {
            let mut strict = (*strict).clone();
            assign_field!(strict.jac = jac.clone());
            assign_variant_field!(comp => NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            comp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.addLoopJacobian")); __mm_s.push_str(&*literal!(" failed because of wrong component: ")); __mm_s.push_str(&*toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub fn getLoopResiduals(mut comp: Arc<NBStrongComponent>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut residuals: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    residuals = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ ALGEBRAIC_LOOP { .. } => Tearing::getResidualVars(var_field!((*comp).strict, NBStrongComponent::ALGEBRAIC_LOOP).clone()),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    residuals
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getVariables(mut comp: Arc<NBStrongComponent>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    vars = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => list![var_field!((*comp).var, NBStrongComponent::SINGLE_COMPONENT).clone()],
        Deref @ MULTI_COMPONENT { .. } => {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut v in (var_field!((*comp).vars, NBStrongComponent::MULTI_COMPONENT).clone()).into_iter().cloned() {
            let __x = Slice::getT(v.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        Deref @ SLICED_COMPONENT { .. } => list![Slice::getT(var_field!((*comp).var, NBStrongComponent::SLICED_COMPONENT).clone())],
        Deref @ RESIZABLE_COMPONENT { .. } => list![Slice::getT(var_field!((*comp).var, NBStrongComponent::RESIZABLE_COMPONENT).clone())],
        Deref @ GENERIC_COMPONENT { .. } => list![Slice::getT(var_field!((*comp).var, NBStrongComponent::GENERIC_COMPONENT).clone())],
        Deref @ ENTWINED_COMPONENT { .. } => List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut slice in (var_field!((*comp).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone()).into_iter().cloned() {
            let __x = getVariables(slice.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ ALGEBRAIC_LOOP { .. } => Tearing::getVariables(var_field!((*comp).strict, NBStrongComponent::ALGEBRAIC_LOOP).clone())?,
        Deref @ ALIAS { .. } => getVariables(var_field!((*comp).original, NBStrongComponent::ALIAS).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.getVariables")); __mm_s.push_str(&*literal!(" failed because of wrong component: ")); __mm_s.push_str(&*toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vars)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getVarCref(mut comp: Arc<NBStrongComponent>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut var_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    var_cref = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SLICED_COMPONENT { .. } => var_field!((*comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone(),
        Deref @ RESIZABLE_COMPONENT { .. } => var_field!((*comp).var_cref, NBStrongComponent::RESIZABLE_COMPONENT).clone(),
        Deref @ GENERIC_COMPONENT { .. } => var_field!((*comp).var_cref, NBStrongComponent::GENERIC_COMPONENT).clone(),
        Deref @ ALIAS { .. } => getVarCref(var_field!((*comp).original, NBStrongComponent::ALIAS).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.getVarCref")); __mm_s.push_str(&*literal!(" failed because of wrong component: ")); __mm_s.push_str(&*toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var_cref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getEquations(mut comp: Arc<NBStrongComponent>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    eqns = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => list![var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone()],
        Deref @ MULTI_COMPONENT { .. } => list![Slice::getT(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone())],
        Deref @ SLICED_COMPONENT { .. } => list![Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone())],
        Deref @ RESIZABLE_COMPONENT { .. } => list![Slice::getT(var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone())],
        Deref @ GENERIC_COMPONENT { .. } => list![Slice::getT(var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone())],
        Deref @ ENTWINED_COMPONENT { .. } => List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut slice in (var_field!((*comp).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone()).into_iter().cloned() {
            let __x = getEquations(slice.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ ALGEBRAIC_LOOP { .. } => Tearing::getResidualEqns(var_field!((*comp).strict, NBStrongComponent::ALGEBRAIC_LOOP).clone()),
        Deref @ ALIAS { .. } => getEquations(var_field!((*comp).original, NBStrongComponent::ALIAS).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.getEquations")); __mm_s.push_str(&*literal!(" failed because of wrong component: ")); __mm_s.push_str(&*toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqns)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getSolveStatus(mut comp: Arc<NBStrongComponent>) -> Result<Solve::Status> {
    let mut status: Solve::Status = Solve::Status::UNPROCESSED;
    status = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => var_field!((*comp).status, NBStrongComponent::SINGLE_COMPONENT).clone(),
        Deref @ MULTI_COMPONENT { .. } => var_field!((*comp).status, NBStrongComponent::MULTI_COMPONENT).clone(),
        Deref @ SLICED_COMPONENT { .. } => var_field!((*comp).status, NBStrongComponent::SLICED_COMPONENT).clone(),
        Deref @ RESIZABLE_COMPONENT { .. } => var_field!((*comp).status, NBStrongComponent::RESIZABLE_COMPONENT).clone(),
        Deref @ GENERIC_COMPONENT { .. } => Solve::Status::EXPLICIT.clone(),
        Deref @ ENTWINED_COMPONENT { .. } => Solve::Status::EXPLICIT.clone(),
        Deref @ ALGEBRAIC_LOOP { .. } => var_field!((*comp).status, NBStrongComponent::ALGEBRAIC_LOOP).clone(),
        Deref @ ALIAS { .. } => getSolveStatus(var_field!((*comp).original, NBStrongComponent::ALIAS).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.getSolveStatus")); __mm_s.push_str(&*literal!(" failed because of wrong component: ")); __mm_s.push_str(&*toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(status)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isDiscrete(mut comp: Arc<NBStrongComponent>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => Equation::isDiscrete(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone()),
        Deref @ MULTI_COMPONENT { .. } => Equation::isDiscrete(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone())),
        Deref @ SLICED_COMPONENT { .. } => Equation::isDiscrete(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone())),
        Deref @ RESIZABLE_COMPONENT { .. } => Equation::isDiscrete(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone())),
        Deref @ ENTWINED_COMPONENT { .. } => List::all(var_field!((*comp).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone(), (std::sync::Arc::new(isDiscrete) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NBStrongComponent>) -> Result<bool> + 'static>)),
        Deref @ GENERIC_COMPONENT { .. } => Equation::isDiscrete(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone())),
        Deref @ ALGEBRAIC_LOOP { .. } => var_field!((*comp).mixed, NBStrongComponent::ALGEBRAIC_LOOP).clone(),
        Deref @ ALIAS { .. } => isDiscrete(var_field!((*comp).original, NBStrongComponent::ALIAS).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.isDiscrete")); __mm_s.push_str(&*literal!(" failed because of wrong component: ")); __mm_s.push_str(&*toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isDummy(mut comp: Arc<NBStrongComponent>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SINGLE_COMPONENT { .. } => Equation::isDummy(Pointer::access(var_field!((*comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone())),
        Deref @ MULTI_COMPONENT { .. } => Equation::isDummy(Pointer::access(Slice::getT(var_field!((*comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone()))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isAlias(mut comp: Arc<NBStrongComponent>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ ALIAS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isSingleComponent(mut comp: Arc<NBStrongComponent>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(removeAlias(comp.clone())) {
        Deref @ SINGLE_COMPONENT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isAlgebraicLoop(mut comp: Arc<NBStrongComponent>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(removeAlias(comp.clone())) {
        Deref @ ALGEBRAIC_LOOP { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn setHomotopy(mut comp: Arc<NBStrongComponent>, mut homotopy: bool) -> Arc<NBStrongComponent> {
    let mut comp: Arc<NBStrongComponent> = comp;
    comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ ALGEBRAIC_LOOP { .. } => {
            assign_variant_field!(comp => NBStrongComponent::ALGEBRAIC_LOOP; homotopy = homotopy.clone());
            comp.clone()
        },
        _ => comp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    comp
}

pub fn createPseudoScalar(mut comp_indices: Arc<metamodelica::List<i32>>, mut eqn_to_var: metamodelica::Array<i32>, mut mapping: Arc<Mapping::Mapping>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>) -> Result<Arc<NBStrongComponent>> {
    let mut comp: Arc<NBStrongComponent>;
    comp = ({
        let mut homotopy: Pointer::Pointer<bool> = Pointer::create(false);
        (::match_deref::match_deref! { match &(comp_indices.clone()) {
        Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil } => {
            let mut var_scal_idx: i32 = 0;
            let mut var_arr_idx: i32 = 0;
            let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            var_scal_idx = eqn_to_var.borrow()[(i.clone()-1) as usize].clone();
            var_arr_idx = mapping.var_StA.borrow()[(var_scal_idx.clone()-1) as usize].clone();
            var = BVariable::VariablePointers::getVarAt(vars.clone(), var_arr_idx.clone())?;
            eqn = EquationPointers::getEqnAt(eqns.clone(), mapping.eqn_StA.borrow()[(i.clone()-1) as usize].clone())?;
            if Equation::isForEquation(eqn.clone()) {
                if let Ok((metamodelica::List::Cons { head: __pa0, tail: __t2 }, metamodelica::List::Cons { head: __pa1, tail: __t3 })) = getLoopVarsAndEqns(comp_indices.clone(), eqn_to_var.clone(), mapping.clone(), vars.clone(), eqns.clone()) {
                    ::match_deref::match_deref! { match &(__t2.clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(__t3.clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    var_slice = __pa0.clone();
                    eqn_slice = __pa1.clone();
                } else {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.createPseudoScalar")); __mm_s.push_str(&*literal!(" failed because single indices did not turn out to be single components.")); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                }
                comp = Arc::new(NBStrongComponent::SLICED_COMPONENT { var_cref: BVariable::VariablePointers::varSlice(vars.clone(), var_scal_idx.clone(), mapping.var_StA.borrow()[(var_scal_idx.clone()-1) as usize].clone(), mapping.clone(), true)?, var: var_slice.clone(), eqn: eqn_slice.clone(), status: Solve::Status::UNPROCESSED.clone() });
            } else if Equation::isCompound(eqn.clone()) {
                comp = Arc::new(NBStrongComponent::MULTI_COMPONENT { vars: list![Arc::new(Slice::NBSlice { t: var.clone(), indices: metamodelica::nil() })], eqn: Arc::new(Slice::NBSlice { t: eqn.clone(), indices: metamodelica::nil() }), status: Solve::Status::UNPROCESSED.clone() });
            } else {
                if let Ok((metamodelica::List::Cons { head: __pa4, tail: __t6 }, metamodelica::List::Cons { head: __pa5, tail: __t7 })) = getLoopVarsAndEqns(comp_indices.clone(), eqn_to_var.clone(), mapping.clone(), vars.clone(), eqns.clone()) {
                    ::match_deref::match_deref! { match &(__t6.clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(__t7.clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    var_slice = __pa4.clone();
                    eqn_slice = __pa5.clone();
                } else {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.createPseudoScalar")); __mm_s.push_str(&*literal!(" failed because single indices did not turn out to be single components.")); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                }
                comp = createSliceOrSingle(BVariable::VariablePointers::varSlice(vars.clone(), var_scal_idx.clone(), mapping.var_StA.borrow()[(var_scal_idx.clone()-1) as usize].clone(), mapping.clone(), true)?, var_slice.clone(), eqn_slice.clone());
            }
            comp.clone()
        },
        _ => {
            let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut comp_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
            let mut comp_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
            let mut tearingSet: Arc<Tearing::NBTearing> = Arc::new(<Tearing::NBTearing as ::std::default::Default>::default());
            let mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            (comp_vars, comp_eqns) = getLoopVarsAndEqns(comp_indices.clone(), eqn_to_var.clone(), mapping.clone(), vars.clone(), eqns.clone())?;
            comp = (::match_deref::match_deref! { match &((comp_vars.clone(), comp_eqns.clone())) {
        (Deref @ metamodelica::List::Cons { head: var_slice, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: eqn_slice, tail: Deref @ metamodelica::List::Nil }) if (!(Equation::isForEquation(Slice::getT(eqn_slice.clone())) || Equation::isAlgorithm(Slice::getT(eqn_slice.clone())))) => createSliceOrSingle(BVariable::getVarName(Slice::getT(var_slice.clone())), var_slice.clone(), eqn_slice.clone()),
        (_, Deref @ metamodelica::List::Cons { head: eqn_slice, tail: Deref @ metamodelica::List::Nil }) if (!(Equation::isForEquation(Slice::getT(eqn_slice.clone())))) => Arc::new(NBStrongComponent::MULTI_COMPONENT { status: Solve::Status::UNPROCESSED.clone(), eqn: eqn_slice.clone(), vars: comp_vars.clone() }),
        _ => {
            tearingSet = Arc::new(Tearing::NBTearing { jac: None, innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), residual_eqns: comp_eqns.clone(), iteration_vars: comp_vars.clone() });
            for mut eqn in &*comp_eqns.clone() {
                let mut eqn = eqn.clone();
                Equation::map(Pointer::access(Slice::getT(eqn.clone())), Arc::new({ let __pe_b1 = homotopy.clone(); move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            }
            Arc::new(NBStrongComponent::ALGEBRAIC_LOOP { status: Solve::Status::IMPLICIT.clone(), homotopy: Pointer::access(homotopy.clone()), mixed: false, linear: false, casual: None, strict: tearingSet.clone(), idx: -1 })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            comp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.createPseudoScalar")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(comp)
}

pub fn createSliceOrSingle(mut cref: Arc<ComponentRef::NFComponentRef>, mut var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Arc<NBStrongComponent> {
    let mut comp: Arc<NBStrongComponent>;
    if Slice::isFull(var_slice.clone()) && Slice::isFull(eqn_slice.clone()) && !(ComponentRef::hasSubscripts(cref.clone())) {
        comp = Arc::new(NBStrongComponent::SINGLE_COMPONENT { status: Solve::Status::UNPROCESSED.clone(), eqn: Slice::getT(eqn_slice.clone()), var: Slice::getT(var_slice.clone()) });
    } else {
        comp = Arc::new(NBStrongComponent::SLICED_COMPONENT { status: Solve::Status::UNPROCESSED.clone(), eqn: eqn_slice.clone(), var: var_slice.clone(), var_cref: cref.clone() });
    }
    comp
}

fn getLoopVarsAndEqns(mut comp_indices: Arc<metamodelica::List<i32>>, mut eqn_to_var: metamodelica::Array<i32>, mut mapping: Arc<Mapping::Mapping>, mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>) -> Result<(Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>)> {
    let mut acc_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut acc_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut var_idx: i32 = 0;
    let mut var_arr_idx: i32 = 0;
    let mut var_scal_idx: i32 = 0;
    let mut eqn_arr_idx: i32 = 0;
    let mut eqn_scal_idx: i32 = 0;
    let mut idx_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut len_comps: i32 = (comp_indices.clone().len() as i32);
    let mut var_map: Arc<UnorderedMap::UnorderedMap<i32, Arc<metamodelica::List<i32>>>> = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), len_comps.clone());
    let mut eqn_map: Arc<UnorderedMap::UnorderedMap<i32, Arc<metamodelica::List<i32>>>> = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), len_comps.clone());
    for mut eqn_idx in &*comp_indices.clone() {
        let mut eqn_idx = eqn_idx.clone();
        var_idx = eqn_to_var.borrow()[(eqn_idx.clone()-1) as usize].clone();
        var_arr_idx = mapping.var_StA.borrow()[(var_idx.clone()-1) as usize].clone();
        eqn_arr_idx = mapping.eqn_StA.borrow()[(eqn_idx.clone()-1) as usize].clone();
        idx_lst = UnorderedMap::getOrDefault(var_arr_idx.clone(), var_map.clone(), metamodelica::nil());
        UnorderedMap::add(var_arr_idx.clone(), cons(var_idx.clone(), idx_lst.clone()), var_map.clone())?;
        idx_lst = UnorderedMap::getOrDefault(eqn_arr_idx.clone(), eqn_map.clone(), metamodelica::nil());
        UnorderedMap::add(eqn_arr_idx.clone(), cons(eqn_idx.clone(), idx_lst.clone()), eqn_map.clone())?;
    }
    for mut tpl in &*UnorderedMap::toList(var_map.clone()) {
        let mut tpl = tpl.clone();
        (var_arr_idx, idx_lst) = tpl.clone();
        (var_scal_idx, _) = mapping.var_AtS.borrow()[(var_arr_idx.clone()-1) as usize].clone();
        var = BVariable::VariablePointers::getVarAt(vars.clone(), var_arr_idx.clone())?;
        idx_lst = if ((idx_lst.clone().len() as i32) == BVariable::size(var.clone(), false)) {metamodelica::nil()} else {{
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (idx_lst.clone()).into_iter().cloned() {
            let __x = i.clone() - var_scal_idx.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }};
        acc_vars = cons(Arc::new(Slice::NBSlice { t: var.clone(), indices: List::sort(idx_lst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))? }), acc_vars.clone());
    }
    for mut tpl in &*UnorderedMap::toList(eqn_map.clone()) {
        let mut tpl = tpl.clone();
        (eqn_arr_idx, idx_lst) = tpl.clone();
        (eqn_scal_idx, _) = mapping.eqn_AtS.borrow()[(eqn_arr_idx.clone()-1) as usize].clone();
        eqn = EquationPointers::getEqnAt(eqns.clone(), eqn_arr_idx.clone())?;
        idx_lst = if ((idx_lst.clone().len() as i32) == Equation::size(eqn.clone(), false)?) {metamodelica::nil()} else {{
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (idx_lst.clone()).into_iter().cloned() {
            let __x = i.clone() - eqn_scal_idx.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }};
        acc_eqns = cons(Arc::new(Slice::NBSlice { t: eqn.clone(), indices: List::sort(idx_lst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))? }), acc_eqns.clone());
    }
    Ok((acc_vars, acc_eqns))
}

fn updateDependencyMap(mut cref: Arc<ComponentRef::NFComponentRef>, mut dependencies: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<()> {
    let mut removed: bool = false;
    removed = UnorderedSet::remove(cref.clone(), dependencies.clone())?;
    UnorderedMap::add(cref.clone(), UnorderedSet::toList(dependencies.clone()), map.clone())?;
    if removed.clone() {
        UnorderedSet::addNew(cref.clone(), dependencies.clone())?;
    }
    Ok(())
}

fn prepareDependencies(mut dependencies: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut jacType: JacobianType) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    fn addSubDependencies(mut dep: Arc<ComponentRef::NFComponentRef>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut checkFn: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
        let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = set;
        if BVariable::checkCref(dep.clone(), checkFn.clone(), metamodelica::sourceInfo!()) {
            UnorderedSet::add(dep.clone(), set.clone())?;
        } else {
            for mut tmp in &*UnorderedMap::getSafe(dep.clone(), map.clone(), metamodelica::sourceInfo!())? {
                let mut tmp = tmp.clone();
                UnorderedSet::add(tmp.clone(), set.clone())?;
            }
        }
        Ok(set)
    }

    let mut dependencies: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = dependencies;
    UnorderedSet::apply(dependencies.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(Expression::replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| ComponentRef::mapExp(__pe_a0, __pe_b1.clone()) }))?;
    UnorderedSet::apply(dependencies.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| ComponentRef::simplifySubscripts(__pe_a0, __pe_b1.clone()) }))?;
    dependencies = (match jacType.clone() {
        JacobianType::ODE => UnorderedSet::fold(dependencies.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = (std::sync::Arc::new(fnptr!(BVariable::isState, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a3| addSubDependencies(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)),
        JacobianType::OPT_LFG => UnorderedSet::fold(dependencies.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = (std::sync::Arc::new(fnptr!(BVariable::isStateOrOptimizable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a3| addSubDependencies(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)),
        JacobianType::OPT_MRF => UnorderedSet::fold(dependencies.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = (std::sync::Arc::new(fnptr!(BVariable::isStateOrOptimizable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a3| addSubDependencies(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)),
        JacobianType::OPT_R0 => UnorderedSet::fold(dependencies.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = (std::sync::Arc::new(fnptr!(BVariable::isStateOrOptimizable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a3| addSubDependencies(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)),
        _ => dependencies.clone(),
    });
    Ok(dependencies)
}


