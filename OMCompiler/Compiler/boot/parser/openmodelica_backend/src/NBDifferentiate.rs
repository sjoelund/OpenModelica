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
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointer;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_nf_frontend::BaseModelica;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs as BuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFClassTree::ClassTree;
use openmodelica_nf_frontend::NFComponent as Component;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFFunction::Slot;
use openmodelica_nf_frontend::NFFunction;
use openmodelica_nf_frontend::NFFunctionDerivative as FunctionDerivative;
use openmodelica_nf_frontend::NFInstContext as InstContext;
use openmodelica_nf_frontend::NFInstNode::CachedData;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFSections as Sections;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// Util imports
// ================================
//        TYPES AND UNIONTYPES
// ================================
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum DifferentiationType {
    TIME = 1,
    SIMPLE = 2,
    FUNCTION = 3,
    JACOBIAN = 4,
}
impl PartialOrd for DifferentiationType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for DifferentiationType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for DifferentiationType {
    fn default() -> Self { Self::TIME }
}

pub mod DifferentiationArguments {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DifferentiationArguments {
        /// The input will be differentiated w.r.t. this cref (only SIMPLE).
        pub diffCref: Arc<ComponentRef::NFComponentRef>,
        /// contains all new variables that need to be added to the system
        pub new_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>,
        /// seed and temporary cref map x --> $SEED.MATRIX.x, y --> $pDer.MATRIX.y. Can be used for any differentiation rules
        pub diff_map: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>>,
        /// Differentiation use case (time, simple, function, jacobian)
        pub diffType: DifferentiationType,
        /// Function tree containing all functions and their known derivatives
        pub funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>,
        /// true if the variables are scalarized
        pub scalarized: bool,
        /// map for accumulating adjoint gradients for component refs
        pub adjoint_map: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>>,
        /// current gradient expression, used in reverse mode
        pub current_grad: Arc<Expression::NFExpression>,
        /// If false, skip writing into adjoint_map (used for LHS traversal in reverse/Jacobian).
        pub collectAdjoints: bool,
    }

    impl Default for DifferentiationArguments {
        fn default() -> Self {
            Self {
                diffCref: Default::default(),
                new_vars: Default::default(),
                diff_map: Default::default(),
                diffType: Default::default(),
                funcMap: Default::default(),
                scalarized: Default::default(),
                adjoint_map: Default::default(),
                current_grad: Default::default(),
                collectAdjoints: Default::default(),
            }
        }
    }

    pub type DIFFERENTIATION_ARGUMENTS = DifferentiationArguments;

    pub fn default(mut ty: DifferentiationType, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Arc<DifferentiationArguments> {
        let mut diffArgs: Arc<DifferentiationArguments> = Arc::new(DifferentiationArguments { collectAdjoints: false, current_grad: Arc::new(Expression::NFExpression::EMPTY { ty: Arc::new(openmodelica_nf_frontend::NFType::REAL) }), adjoint_map: None, scalarized: false, funcMap: funcMap.clone(), diffType: ty.clone(), diff_map: None, new_vars: metamodelica::nil(), diffCref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY) });
        diffArgs
    }

    pub fn simpleCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Arc<DifferentiationArguments> {
        let mut diffArgs: Arc<DifferentiationArguments> = Arc::new(DifferentiationArguments { collectAdjoints: false, current_grad: Arc::new(Expression::NFExpression::EMPTY { ty: Arc::new(openmodelica_nf_frontend::NFType::REAL) }), adjoint_map: None, scalarized: false, funcMap: funcMap.clone(), diffType: DifferentiationType::SIMPLE.clone(), diff_map: None, new_vars: metamodelica::nil(), diffCref: cref.clone() });
        diffArgs
    }

    pub fn toString(mut diffArgs: Arc<DifferentiationArguments>) -> Result<ArcStr> {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*diffTypeStr(diffArgs.diffType.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
        if diffArgs.diffType.clone() == DifferentiationType::SIMPLE.clone() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ComponentRef::toString(diffArgs.diffCref.clone())?); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn diffTypeStr(mut diffType: DifferentiationType) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((match diffType.clone() {
        DifferentiationType::TIME => literal!("TIME"),
        DifferentiationType::SIMPLE => literal!("SIMPLE"),
        DifferentiationType::FUNCTION { .. } => literal!("FUNCTION"),
        DifferentiationType::JACOBIAN => literal!("JACOBIAN"),
        _ => literal!("FAIL"),
    })).clone();
        r#str
    }

}

// ================================
//             FUNCTIONS
// ================================
pub fn differentiateStrongComponentList(mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>, mut idx: Pointer::Pointer<i32>, mut context: ArcStr, mut name: ArcStr) -> (Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>, Arc<DifferentiationArguments::DifferentiationArguments>) {
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = comps;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>> = Pointer::create(diffArguments.clone());
    comps = List::map(comps.clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = idx.clone(); let __pe_b3 = (context.clone()).clone(); let __pe_b4 = (name.clone()).clone(); move |__pe_a0| differentiateStrongComponent(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }));
    diffArguments = Pointer::access(diffArguments_ptr.clone());
    (comps, diffArguments)
}

pub fn differentiateStrongComponentListAdjoint(mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>, mut idx: Pointer::Pointer<i32>, mut context: ArcStr, mut name: ArcStr) -> Result<(Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = comps;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>> = Pointer::create(diffArguments.clone());
    let mut newComps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;
    let mut lhsCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut gradCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut compVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut da: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    diff_map = Util::getOption(diffArguments.diff_map.clone())?;
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Component: ")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone())?;
        compVars = StrongComponent::getVariables(comp.clone())?;
        for mut var in &*compVars.clone() {
            let mut var = var.clone();
            lhsCref = BVariable::getVarName(var.clone());
            if !(ComponentRef::isEmpty(lhsCref.clone())) {
                gradCref = UnorderedMap::getOrFail(lhsCref.clone(), diff_map.clone());
                gradCref = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::RESIZABLE_COMPONENT { .. } => ComponentRef::copySubscripts(StrongComponent::getVarCref(comp.clone())?, gradCref.clone())?,
        Deref @ StrongComponent::SLICED_COMPONENT { .. } => ComponentRef::copySubscripts(StrongComponent::getVarCref(comp.clone())?, gradCref.clone())?,
        _ => gradCref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                da = Pointer::access(diffArguments_ptr.clone());
                assign_field!(da.current_grad = Expression::fromCref(gradCref.clone(), false)?);
                Pointer::update(diffArguments_ptr.clone(), da.clone());
            } else {
                dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  No seed mapping for: ")); __mm_s.push_str(&*ComponentRef::toString(lhsCref.clone())?); ArcStr::from(__mm_s) }).clone())?;
            }
            dbg((literal!("  Differentiating component...")).clone())?;
            comp = differentiateStrongComponent(comp.clone(), diffArguments_ptr.clone(), idx.clone(), (context.clone()).clone(), (name.clone()).clone())?;
            newComps = cons(comp.clone(), newComps.clone());
            dbg((literal!("  Done differentiating component.")).clone())?;
        }
    }
    comps = newComps.clone().reverse();
    diffArguments = Pointer::access(diffArguments_ptr.clone());
    Ok((comps, diffArguments))
}

pub fn differentiateStrongComponent(mut comp: Arc<StrongComponent::NBStrongComponent>, mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>>, mut idx: Pointer::Pointer<i32>, mut context: ArcStr, mut name: ArcStr) -> Result<Arc<StrongComponent::NBStrongComponent>> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => {
            let mut new_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
            new_var = differentiateVariablePointer(var_field!((*comp).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone(), diffArguments_ptr.clone())?;
            new_eqn = differentiateEquationPointer(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone(), diffArguments_ptr.clone(), (name.clone()).clone())?;
            Equation::createName(new_eqn.clone(), idx.clone(), (context.clone()).clone())?;
            Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { var: new_var.clone(), eqn: new_eqn.clone(), status: var_field!((*comp).status, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone() })
        },
        Deref @ StrongComponent::MULTI_COMPONENT { .. } => {
            let mut new_var_slices: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
            let mut new_eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            new_var_slices = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut var in (var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone()).into_iter().cloned() {
            let __x = Slice::apply(var.clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            new_eqn_slice = Slice::apply(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = (name.clone()).clone(); move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            Equation::createName(Slice::getT(new_eqn_slice.clone()), idx.clone(), (context.clone()).clone())?;
            Arc::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT { vars: new_var_slices.clone(), eqn: new_eqn_slice.clone(), status: var_field!((*comp).status, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone() })
        },
        Deref @ StrongComponent::SLICED_COMPONENT { .. } => {
            let mut new_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut new_var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut new_eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateComponentRefNoCollect(Expression::fromCref(var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), false)?, Pointer::access(diffArguments_ptr.clone()))?) {
                (Deref @ Expression::CREF { cref: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            new_cref = __pa0.clone();
            diffArguments = __pa1.clone();
            Pointer::update(diffArguments_ptr.clone(), diffArguments.clone());
            new_var_slice = Slice::apply(var_field!((*comp).var, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone()) }));
            new_eqn_slice = Slice::apply(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = (name.clone()).clone(); move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            Slice::applyMutable(new_eqn_slice.clone(), Arc::new({ let __pe_b1 = idx.clone(); let __pe_b2 = (context.clone()).clone(); move |__pe_a0| Equation::createName(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            Arc::new(StrongComponent::NBStrongComponent::SLICED_COMPONENT { var_cref: new_cref.clone(), var: new_var_slice.clone(), eqn: new_eqn_slice.clone(), status: var_field!((*comp).status, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone() })
        },
        Deref @ StrongComponent::RESIZABLE_COMPONENT { .. } => {
            let mut new_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut new_var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut new_eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateComponentRef(Expression::fromCref(var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone(), false)?, Pointer::access(diffArguments_ptr.clone()))?) {
                (Deref @ Expression::CREF { cref: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            new_cref = __pa0.clone();
            diffArguments = __pa1.clone();
            Pointer::update(diffArguments_ptr.clone(), diffArguments.clone());
            new_var_slice = Slice::apply(var_field!((*comp).var, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone()) }));
            new_eqn_slice = Slice::apply(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = (name.clone()).clone(); move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            Slice::applyMutable(new_eqn_slice.clone(), Arc::new({ let __pe_b1 = idx.clone(); let __pe_b2 = (context.clone()).clone(); move |__pe_a0| Equation::createName(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            Arc::new(StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT { var_cref: new_cref.clone(), var: new_var_slice.clone(), eqn: new_eqn_slice.clone(), order: var_field!((*comp).order, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone(), status: var_field!((*comp).status, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone() })
        },
        Deref @ StrongComponent::GENERIC_COMPONENT { .. } => {
            let mut new_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut new_var_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut new_eqn_slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
            let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateComponentRef(Expression::fromCref(var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::GENERIC_COMPONENT).clone(), false)?, Pointer::access(diffArguments_ptr.clone()))?) {
                (Deref @ Expression::CREF { cref: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            new_cref = __pa0.clone();
            diffArguments = __pa1.clone();
            Pointer::update(diffArguments_ptr.clone(), diffArguments.clone());
            new_var_slice = Slice::apply(var_field!((*comp).var, StrongComponent::NBStrongComponent::GENERIC_COMPONENT).clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone()) }));
            new_eqn_slice = Slice::apply(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::GENERIC_COMPONENT).clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = (name.clone()).clone(); move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            Slice::applyMutable(new_eqn_slice.clone(), Arc::new({ let __pe_b1 = idx.clone(); let __pe_b2 = (context.clone()).clone(); move |__pe_a0| Equation::createName(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            Arc::new(StrongComponent::NBStrongComponent::GENERIC_COMPONENT { var_cref: new_cref.clone(), var: new_var_slice.clone(), eqn: new_eqn_slice.clone() })
        },
        Deref @ StrongComponent::ALGEBRAIC_LOOP { .. } => {
            let mut strict: Arc<Tearing::NBTearing> = Arc::new(<Tearing::NBTearing as ::std::default::Default>::default());
            let mut casual: Option<Arc<Tearing::NBTearing>> = None;
            let mut linear: bool = false;
            strict = differentiateTearing(var_field!((*comp).strict, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone(), diffArguments_ptr.clone(), idx.clone(), (context.clone()).clone(), (name.clone()).clone())?;
            casual = Util::applyOption(var_field!((*comp).casual, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = idx.clone(); let __pe_b3 = (context.clone()).clone(); let __pe_b4 = (name.clone()).clone(); move |__pe_a0| differentiateTearing(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }));
            linear = (::match_deref::match_deref! { match &(Pointer::access(diffArguments_ptr.clone())) {
        Deref @ DifferentiationArguments::DIFFERENTIATION_ARGUMENTS { diffType: DifferentiationType::JACOBIAN, .. } => true,
        _ => var_field!((*comp).linear, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { idx: -1, strict: strict.clone(), casual: casual.clone(), linear: linear.clone(), mixed: false, homotopy: var_field!((*comp).homotopy, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone(), status: var_field!((*comp).status, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone() })
        },
        Deref @ StrongComponent::ENTWINED_COMPONENT { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateStrongComponent")); __mm_s.push_str(&*literal!(" not implemented for entwined equation:\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        Deref @ StrongComponent::ALIAS { .. } => {
            differentiateStrongComponent(var_field!((*comp).original, StrongComponent::NBStrongComponent::ALIAS).clone(), diffArguments_ptr.clone(), idx.clone(), (context.clone()).clone(), (name.clone()).clone())?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateStrongComponent")); __mm_s.push_str(&*literal!(" not implemented for unknown strong component:\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub fn differentiateTearing(mut tearing: Arc<Tearing::NBTearing>, mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>>, mut idx: Pointer::Pointer<i32>, mut context: ArcStr, mut name: ArcStr) -> Result<Arc<Tearing::NBTearing>> {
    let mut diff_tearing: Arc<Tearing::NBTearing> = Arc::new(<Tearing::NBTearing as ::std::default::Default>::default());
    let mut ite_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut res_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut inner_eqns: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>;
    ite_vars = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut var in (tearing.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::apply(var.clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res_eqns = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut eqn in (tearing.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::apply(eqn.clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = (name.clone()).clone(); move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    inner_eqns = metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        for mut ie in (tearing.innerEquations.clone()).borrow().iter() {
            let __x = differentiateStrongComponent(ie.clone(), diffArguments_ptr.clone(), idx.clone(), (context.clone()).clone(), (name.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
    diff_tearing = Arc::new(Tearing::NBTearing { iteration_vars: ite_vars.clone(), residual_eqns: res_eqns.clone(), innerEquations: inner_eqns.clone(), jac: None });
    Ok(diff_tearing)
}

pub fn differentiateEquationPointerList(mut equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>, mut idx: Pointer::Pointer<i32>, mut context: ArcStr, mut name: ArcStr) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = equations;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>> = Pointer::create(diffArguments.clone());
    equations = List::map(equations.clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = (name.clone()).clone(); move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    for mut eqn in &*equations.clone() {
        let mut eqn = eqn.clone();
        Equation::createName(eqn.clone(), idx.clone(), (context.clone()).clone())?;
    }
    diffArguments = Pointer::access(diffArguments_ptr.clone());
    Ok((equations, diffArguments))
}

pub fn differentiateEquationPointer(mut eq_ptr: Pointer::Pointer<Arc<Equation::Equation>>, mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>>, mut name: ArcStr) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
    let mut derivative_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut eq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut diffedEq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut old_diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    let mut new_diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    eq = Pointer::access(eq_ptr.clone());
    old_diffArguments = Pointer::access(diffArguments_ptr.clone());
    derivative_ptr = (::match_deref::match_deref! { match &(Equation::getAttributes(eq.clone())) {
        Deref @ EquationAttributes::EQUATION_ATTRIBUTES { derivative: Some(derivative_ptr), .. } if (old_diffArguments.diffType.clone() == DifferentiationType::TIME.clone()) => derivative_ptr.clone(),
        _ => {
            (diffedEq, new_diffArguments) = differentiateEquation(eq.clone(), old_diffArguments.clone(), (name.clone()).clone())?;
            derivative_ptr = Pointer::create(diffedEq.clone());
            if new_diffArguments.diffType.clone() == DifferentiationType::TIME.clone() {
                Pointer::update(eq_ptr.clone(), Equation::setDerivative(eq.clone(), derivative_ptr.clone())?);
            }
            if !(referenceEq(&new_diffArguments.clone(),&old_diffArguments.clone())) {
                Pointer::update(diffArguments_ptr.clone(), new_diffArguments.clone());
            }
            derivative_ptr.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(derivative_ptr)
}

pub fn differentiateEquation(mut eq: Arc<Equation::Equation>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>, mut name: ArcStr) -> Result<(Arc<Equation::Equation>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut eq: Arc<Equation::Equation> = eq;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? && !(stringEqual((name.clone()).clone(), (literal!("")).clone())) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### debugDifferentiation | ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ###\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[BEFORE] ")); __mm_s.push_str(&*Equation::toString(eq.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (eq, diffArguments) = ({
        let mut forBody: Arc<metamodelica::List<Arc<Equation::Equation>>> = metamodelica::nil();
        let mut lhs_base: Arc<ComponentRef::NFComponentRef> = Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY);
        let mut n: i32 = 0;
        (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::SCALAR_EQUATION { .. } => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
            (lhs, diffArguments) = differentiateExpressionNoCollect(var_field!((*eq).lhs, Equation::Equation::SCALAR_EQUATION).clone(), diffArguments.clone())?;
            (rhs, diffArguments) = differentiateExpression(var_field!((*eq).rhs, Equation::Equation::SCALAR_EQUATION).clone(), diffArguments.clone())?;
            attr = differentiateEquationAttributes(var_field!((*eq).attr, Equation::Equation::SCALAR_EQUATION).clone(), diffArguments.clone())?;
            (Arc::new(Equation::Equation::SCALAR_EQUATION { ty: var_field!((*eq).ty, Equation::Equation::SCALAR_EQUATION).clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: var_field!((*eq).source, Equation::Equation::SCALAR_EQUATION).clone(), attr: attr.clone() }), diffArguments.clone())
        },
        Deref @ Equation::ARRAY_EQUATION { .. } => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
            let mut dm: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;
            let mut seed_base: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut iel: i32 = 0;
            let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            let mut grad_save: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs_i: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut grad_i: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut collect_save: bool = false;
            (lhs, diffArguments) = differentiateExpressionNoCollect(var_field!((*eq).lhs, Equation::Equation::ARRAY_EQUATION).clone(), diffArguments.clone())?;
            if Util::isSome(diffArguments.adjoint_map.clone()) && diffArguments.diffType.clone() == DifferentiationType::JACOBIAN.clone() && Expression::isArray(var_field!((*eq).rhs, Equation::Equation::ARRAY_EQUATION).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(diffArguments.diff_map.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                dm = __pa0.clone();
                lhs_base = Expression::toCref(var_field!((*eq).lhs, Equation::Equation::ARRAY_EQUATION).clone())?;
                if Type::isArray(var_field!((*eq).ty, Equation::Equation::ARRAY_EQUATION).clone()) {
                    dims = Type::arrayDims(var_field!((*eq).ty, Equation::Equation::ARRAY_EQUATION).clone());
                    if !(dims.clone().is_empty()) {
                        n = Dimension::size(listHead(dims.clone())?, false)?;
                    }
                }
                if !(ComponentRef::isEmpty(lhs_base.clone())) && UnorderedMap::contains(lhs_base.clone(), dm.clone()) && n.clone() > 0 {
                    seed_base = UnorderedMap::getOrFail(lhs_base.clone(), dm.clone());
                    grad_save = diffArguments.current_grad.clone();
                    collect_save = diffArguments.collectAdjoints.clone();
                    for mut iel in 1..=n.clone() {
                        grad_i = Expression::applySubscripts(list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: iel.clone() }) })], Expression::fromCref(seed_base.clone(), false)?, true)?;
                        rhs_i = Expression::applySubscripts(list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: iel.clone() }) })], var_field!((*eq).rhs, Equation::Equation::ARRAY_EQUATION).clone(), true)?;
                        assign_field!(
                            diffArguments.current_grad = grad_i.clone(),
                            diffArguments.collectAdjoints = true
                        );
                        (_, diffArguments) = differentiateExpression(rhs_i.clone(), diffArguments.clone())?;
                    }
                    assign_field!(
                        diffArguments.current_grad = grad_save.clone(),
                        diffArguments.collectAdjoints = collect_save.clone()
                    );
                    (rhs, diffArguments) = differentiateExpressionNoCollect(var_field!((*eq).rhs, Equation::Equation::ARRAY_EQUATION).clone(), diffArguments.clone())?;
                } else {
                    (rhs, diffArguments) = differentiateExpression(var_field!((*eq).rhs, Equation::Equation::ARRAY_EQUATION).clone(), diffArguments.clone())?;
                }
            } else {
                (rhs, diffArguments) = differentiateExpression(var_field!((*eq).rhs, Equation::Equation::ARRAY_EQUATION).clone(), diffArguments.clone())?;
            }
            attr = differentiateEquationAttributes(var_field!((*eq).attr, Equation::Equation::ARRAY_EQUATION).clone(), diffArguments.clone())?;
            (Arc::new(Equation::Equation::ARRAY_EQUATION { ty: var_field!((*eq).ty, Equation::Equation::ARRAY_EQUATION).clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: var_field!((*eq).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: attr.clone(), recordSize: var_field!((*eq).recordSize, Equation::Equation::ARRAY_EQUATION).clone() }), diffArguments.clone())
        },
        Deref @ Equation::RECORD_EQUATION { .. } => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
            (lhs, diffArguments) = differentiateExpressionNoCollect(var_field!((*eq).lhs, Equation::Equation::RECORD_EQUATION).clone(), diffArguments.clone())?;
            (rhs, diffArguments) = differentiateExpression(var_field!((*eq).rhs, Equation::Equation::RECORD_EQUATION).clone(), diffArguments.clone())?;
            attr = differentiateEquationAttributes(var_field!((*eq).attr, Equation::Equation::RECORD_EQUATION).clone(), diffArguments.clone())?;
            (Arc::new(Equation::Equation::RECORD_EQUATION { ty: var_field!((*eq).ty, Equation::Equation::RECORD_EQUATION).clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: var_field!((*eq).source, Equation::Equation::RECORD_EQUATION).clone(), attr: attr.clone(), recordSize: var_field!((*eq).recordSize, Equation::Equation::RECORD_EQUATION).clone() }), diffArguments.clone())
        },
        Deref @ Equation::IF_EQUATION { .. } => {
            let mut ifBody: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
            let mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>>;
            let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
            (ifBody, diffArguments_ptr) = differentiateIfEquationBody(var_field!((*eq).body, Equation::Equation::IF_EQUATION).clone(), Pointer::create(diffArguments.clone()))?;
            attr = differentiateEquationAttributes(var_field!((*eq).attr, Equation::Equation::IF_EQUATION).clone(), diffArguments.clone())?;
            (Arc::new(Equation::Equation::IF_EQUATION { size: var_field!((*eq).size, Equation::Equation::IF_EQUATION).clone(), body: ifBody.clone(), source: var_field!((*eq).source, Equation::Equation::IF_EQUATION).clone(), attr: attr.clone() }), Pointer::access(diffArguments_ptr.clone()))
        },
        Deref @ Equation::FOR_EQUATION { .. } => {
            let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
            for mut body_eqn in &*var_field!((*eq).body, Equation::Equation::FOR_EQUATION).clone() {
                let mut body_eqn = body_eqn.clone();
                (body_eqn, diffArguments) = differentiateEquation(body_eqn.clone(), diffArguments.clone(), (literal!("")).clone())?;
                forBody = cons(body_eqn.clone(), forBody.clone());
            }
            attr = differentiateEquationAttributes(var_field!((*eq).attr, Equation::Equation::FOR_EQUATION).clone(), diffArguments.clone())?;
            (Arc::new(Equation::Equation::FOR_EQUATION { size: var_field!((*eq).size, Equation::Equation::FOR_EQUATION).clone(), iter: var_field!((*eq).iter, Equation::Equation::FOR_EQUATION).clone(), body: forBody.clone().reverse(), source: var_field!((*eq).source, Equation::Equation::FOR_EQUATION).clone(), attr: attr.clone() }), diffArguments.clone())
        },
        Deref @ Equation::WHEN_EQUATION { .. } => {
            let mut whenBody: Arc<WhenEquationBody::WhenEquationBody> = Arc::new(<WhenEquationBody::WhenEquationBody as ::std::default::Default>::default());
            let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
            (whenBody, diffArguments) = differentiateWhenEquationBody(var_field!((*eq).body, Equation::Equation::WHEN_EQUATION).clone(), diffArguments.clone())?;
            attr = differentiateEquationAttributes(var_field!((*eq).attr, Equation::Equation::WHEN_EQUATION).clone(), diffArguments.clone())?;
            (Arc::new(Equation::Equation::WHEN_EQUATION { size: var_field!((*eq).size, Equation::Equation::WHEN_EQUATION).clone(), body: whenBody.clone(), source: var_field!((*eq).source, Equation::Equation::WHEN_EQUATION).clone(), attr: attr.clone() }), diffArguments.clone())
        },
        Deref @ Equation::ALGORITHM { .. } => {
            let mut alg: Arc<Algorithm::NFAlgorithm> = Arc::new(<Algorithm::NFAlgorithm as ::std::default::Default>::default());
            (alg, diffArguments) = differentiateAlgorithm(var_field!((*eq).alg, Equation::Equation::ALGORITHM).clone(), diffArguments.clone())?;
            (Arc::new(Equation::Equation::ALGORITHM { size: var_field!((*eq).size, Equation::Equation::ALGORITHM).clone(), alg: alg.clone(), source: var_field!((*eq).source, Equation::Equation::ALGORITHM).clone(), expand: var_field!((*eq).expand, Equation::Equation::ALGORITHM).clone(), attr: var_field!((*eq).attr, Equation::Equation::ALGORITHM).clone() }), diffArguments.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateEquation")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Equation::toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? && !(stringEqual((name.clone()).clone(), (literal!("")).clone())) {
        eq = Equation::simplify(eq.clone(), (name.clone()).clone(), (literal!("\t")).clone(), Pointer::create(metamodelica::nil()), Pointer::create(metamodelica::nil()), Arc::new({ let __pe_b1 = true; let __pe_b2 = (name.clone()).clone(); let __pe_b3 = (literal!("\t")).clone(); move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[AFTER ] ")); __mm_s.push_str(&*Equation::toString(eq.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    } else {
        eq = Equation::simplify(eq.clone(), (name.clone()).clone(), (literal!("")).clone(), Pointer::create(metamodelica::nil()), Pointer::create(metamodelica::nil()), Arc::new({ let __pe_b1 = true; let __pe_b2 = (name.clone()).clone(); let __pe_b3 = (literal!("")).clone(); move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    }
    Ok((eq, diffArguments))
}

pub fn differentiateIfEquationBody(mut body: Arc<IfEquationBody::IfEquationBody>, mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>>) -> Result<(Arc<IfEquationBody::IfEquationBody>, Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>>)> {
    let mut body: Arc<IfEquationBody::IfEquationBody> = body;
    let mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>> = diffArguments_ptr;
    let mut then_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut else_if: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
    then_eqns = List::map(body.then_eqns.clone(), Arc::new({ let __pe_b1 = diffArguments_ptr.clone(); let __pe_b2 = (literal!("")).clone(); move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    if isSome(body.else_if.clone()) {
        (else_if, diffArguments_ptr) = differentiateIfEquationBody(Util::getOption(body.else_if.clone())?, diffArguments_ptr.clone())?;
        body = Arc::new(IfEquationBody::IfEquationBody { condition: body.condition.clone(), then_eqns: then_eqns.clone(), else_if: Some(else_if.clone()) });
    } else {
        body = Arc::new(IfEquationBody::IfEquationBody { condition: body.condition.clone(), then_eqns: then_eqns.clone(), else_if: None });
    }
    Ok((body, diffArguments_ptr))
}

pub fn differentiateWhenEquationBody(mut body: Arc<WhenEquationBody::WhenEquationBody>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<WhenEquationBody::WhenEquationBody>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut body: Arc<WhenEquationBody::WhenEquationBody> = body;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut when_stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>> = metamodelica::nil();
    let mut else_when: Arc<WhenEquationBody::WhenEquationBody> = Arc::new(<WhenEquationBody::WhenEquationBody as ::std::default::Default>::default());
    (when_stmts, diffArguments) = List::mapFold(body.when_stmts.clone(), Arc::new(move |__pe_a0, __pe_a1| differentiateWhenStatement(__pe_a0, __pe_a1)), diffArguments.clone());
    if isSome(body.else_when.clone()) {
        (else_when, diffArguments) = differentiateWhenEquationBody(Util::getOption(body.else_when.clone())?, diffArguments.clone())?;
        body = Arc::new(WhenEquationBody::WhenEquationBody { condition: body.condition.clone(), when_stmts: when_stmts.clone(), else_when: Some(else_when.clone()) });
    } else {
        body = Arc::new(WhenEquationBody::WhenEquationBody { condition: body.condition.clone(), when_stmts: when_stmts.clone(), else_when: None });
    }
    Ok((body, diffArguments))
}

pub fn differentiateWhenStatement(mut stmt: Arc<WhenStatement::WhenStatement>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<WhenStatement::WhenStatement>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut stmt: Arc<WhenStatement::WhenStatement> = stmt;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    (stmt, diffArguments) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ WhenStatement::ASSIGN { .. } => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (lhs, diffArguments) = differentiateExpression(var_field!((*stmt).lhs, WhenStatement::WhenStatement::ASSIGN).clone(), diffArguments.clone())?;
            (rhs, diffArguments) = differentiateExpression(var_field!((*stmt).rhs, WhenStatement::WhenStatement::ASSIGN).clone(), diffArguments.clone())?;
            (Arc::new(WhenStatement::WhenStatement::ASSIGN { lhs: lhs.clone(), rhs: rhs.clone(), source: var_field!((*stmt).source, WhenStatement::WhenStatement::ASSIGN).clone() }), diffArguments.clone())
        },
        _ => {
            (stmt.clone(), diffArguments.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((stmt, diffArguments))
}

pub fn differentiateExpressionDump(mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>, mut name: ArcStr, mut indent: ArcStr) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("### debugDifferentiation | ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ###\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("[BEFORE] ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        (exp, diffArguments) = differentiateExpression(exp.clone(), diffArguments.clone())?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("[AFTER ] ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    } else {
        (exp, diffArguments) = differentiateExpression(exp.clone(), diffArguments.clone())?;
    }
    Ok((exp, diffArguments))
}

pub fn differentiateExpression(mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    (exp, diffArguments) = ({
        let mut new_elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut new_matrix_elements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Expression::NFExpression>>>>> = metamodelica::nil();
        let mut isReverse: bool = Util::isSome(diffArguments.adjoint_map.clone());
        (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            (Arc::new(Expression::NFExpression::INTEGER { value: 0 }), diffArguments.clone())
        },
        Deref @ Expression::REAL { .. } => {
            (Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }), diffArguments.clone())
        },
        Deref @ Expression::STRING { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::BOOLEAN { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::CREF { .. } => {
            differentiateComponentRef(exp.clone(), diffArguments.clone())?
        },
        Deref @ Expression::ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            (arr, diffArguments) = Array::mapFold(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new(differentiateExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> + 'static>), diffArguments.clone());
            assign_variant_field!(exp => Expression::NFExpression::ARRAY; elements = arr.clone());
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::MATRIX { .. } => {
            for mut element_lst in &*var_field!((*exp).elements, Expression::NFExpression::MATRIX).clone() {
                let mut element_lst = element_lst.clone();
                new_elements = metamodelica::nil();
                for mut element in &*element_lst.clone() {
                    let mut element = element.clone();
                    (element, diffArguments) = differentiateExpression(element.clone(), diffArguments.clone())?;
                    new_elements = cons(element.clone(), new_elements.clone());
                }
                new_matrix_elements = cons(new_elements.clone().reverse(), new_matrix_elements.clone());
            }
            (Arc::new(Expression::NFExpression::MATRIX { elements: new_matrix_elements.clone().reverse() }), diffArguments.clone())
        },
        Deref @ Expression::TUPLE { .. } => {
            for mut element in &*var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut element = element.clone();
                (element, diffArguments) = differentiateExpression(element.clone(), diffArguments.clone())?;
                new_elements = cons(element.clone(), new_elements.clone());
            }
            (Arc::new(Expression::NFExpression::TUPLE { ty: var_field!((*exp).ty, Expression::NFExpression::TUPLE).clone(), elements: new_elements.clone().reverse() }), diffArguments.clone())
        },
        Deref @ Expression::RECORD { .. } => {
            for mut element in &*var_field!((*exp).elements, Expression::NFExpression::RECORD).clone() {
                let mut element = element.clone();
                (element, diffArguments) = differentiateExpression(element.clone(), diffArguments.clone())?;
                new_elements = cons(element.clone(), new_elements.clone());
            }
            (Arc::new(Expression::NFExpression::RECORD { path: var_field!((*exp).path, Expression::NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, Expression::NFExpression::RECORD).clone(), elements: new_elements.clone().reverse() }), diffArguments.clone())
        },
        Deref @ Expression::CALL { .. } => {
            differentiateCall(exp.clone(), diffArguments.clone())?
        },
        Deref @ Expression::IF { .. } => {
            let mut elem1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut elem2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut gradTrue: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut gradFalse: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                gradTrue = Arc::new(Expression::NFExpression::IF { ty: Expression::typeOf(current_grad.clone()), condition: var_field!((*exp).condition, Expression::NFExpression::IF).clone(), trueBranch: current_grad.clone(), falseBranch: Expression::makeZero(Expression::typeOf(current_grad.clone()))? });
                gradFalse = Arc::new(Expression::NFExpression::IF { ty: Expression::typeOf(current_grad.clone()), condition: var_field!((*exp).condition, Expression::NFExpression::IF).clone(), trueBranch: Expression::makeZero(Expression::typeOf(current_grad.clone()))?, falseBranch: current_grad.clone() });
                assign_field!(diffArguments.current_grad = gradTrue.clone());
                (elem1, diffArguments) = differentiateExpression(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = gradFalse.clone());
                (elem2, diffArguments) = differentiateExpression(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = current_grad.clone());
            } else {
                (elem1, diffArguments) = differentiateExpression(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), diffArguments.clone())?;
                (elem2, diffArguments) = differentiateExpression(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), diffArguments.clone())?;
            }
            (Arc::new(Expression::NFExpression::IF { ty: var_field!((*exp).ty, Expression::NFExpression::IF).clone(), condition: var_field!((*exp).condition, Expression::NFExpression::IF).clone(), trueBranch: elem1.clone(), falseBranch: elem2.clone() }), diffArguments.clone())
        },
        Deref @ Expression::BINARY { .. } => {
            differentiateBinary(exp.clone(), diffArguments.clone())?
        },
        Deref @ Expression::MULTARY { .. } => {
            differentiateMultary(exp.clone(), diffArguments.clone())?
        },
        Deref @ Expression::UNARY { .. } => {
            let mut elem1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::UNARY { operator: var_field!((*exp).operator, Expression::NFExpression::UNARY).clone(), exp: current_grad.clone() }));
                (elem1, diffArguments) = differentiateExpression(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = current_grad.clone());
            } else {
                (elem1, diffArguments) = differentiateExpression(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), diffArguments.clone())?;
            }
            (Arc::new(Expression::NFExpression::UNARY { operator: var_field!((*exp).operator, Expression::NFExpression::UNARY).clone(), exp: elem1.clone() }), diffArguments.clone())
        },
        Deref @ Expression::CAST { .. } => {
            let mut elem1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (elem1, diffArguments) = differentiateExpression(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), diffArguments.clone())?;
            (Arc::new(Expression::NFExpression::CAST { ty: var_field!((*exp).ty, Expression::NFExpression::CAST).clone(), exp: elem1.clone() }), diffArguments.clone())
        },
        Deref @ Expression::BOX { .. } => {
            let mut elem1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (elem1, diffArguments) = differentiateExpression(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), diffArguments.clone())?;
            (Arc::new(Expression::NFExpression::BOX { exp: elem1.clone() }), diffArguments.clone())
        },
        Deref @ Expression::UNBOX { .. } => {
            let mut elem1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (elem1, diffArguments) = differentiateExpression(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), diffArguments.clone())?;
            (Arc::new(Expression::NFExpression::UNBOX { exp: elem1.clone(), ty: var_field!((*exp).ty, Expression::NFExpression::UNBOX).clone() }), diffArguments.clone())
        },
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            let mut elem1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (elem1, diffArguments) = differentiateExpression(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), diffArguments.clone())?;
            (Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: elem1.clone(), subscripts: var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), ty: var_field!((*exp).ty, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, Expression::NFExpression::SUBSCRIPTED_EXP).clone() }), diffArguments.clone())
        },
        Deref @ Expression::TUPLE_ELEMENT { .. } => {
            let mut elem1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (elem1, diffArguments) = differentiateExpression(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone(), diffArguments.clone())?;
            (Arc::new(Expression::NFExpression::TUPLE_ELEMENT { tupleExp: elem1.clone(), index: var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, Expression::NFExpression::TUPLE_ELEMENT).clone() }), diffArguments.clone())
        },
        Deref @ Expression::RECORD_ELEMENT { .. } => {
            let mut elem1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if diffArguments.diffType.clone() == DifferentiationType::SIMPLE.clone() && !(Expression::containsCref(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone(), diffArguments.diffCref.clone())?) {
                elem1 = Expression::makeZero(Expression::typeOf(exp.clone()))?;
            } else {
                (elem1, diffArguments) = differentiateExpression(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone(), diffArguments.clone())?;
                elem1 = Arc::new(Expression::NFExpression::RECORD_ELEMENT { recordExp: elem1.clone(), index: var_field!((*exp).index, Expression::NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, Expression::NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, Expression::NFExpression::RECORD_ELEMENT).clone() });
            }
            (elem1.clone(), diffArguments.clone())
        },
        Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut d_fn: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            d_fn = BVariable::makeFDerVar(var_field!((*exp).r#fn, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?;
            for mut element in &*var_field!((*exp).args, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone() {
                let mut element = element.clone();
                (element, diffArguments) = differentiateExpression(element.clone(), diffArguments.clone())?;
                new_elements = cons(element.clone(), new_elements.clone());
            }
            (Arc::new(Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION { r#fn: d_fn.clone(), args: listAppend(var_field!((*exp).args, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), new_elements.clone().reverse()), argNames: listAppend(var_field!((*exp).argNames, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut name in (var_field!((*exp).argNames, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = BackendUtil::makeFDerString((name.clone()).clone(), None);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), ty: var_field!((*exp).ty, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone() }), diffArguments.clone())
        },
        Deref @ Expression::LBINARY { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::LUNARY { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::RELATION { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::SIZE { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::RANGE { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::END => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::EMPTY { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::ENUM_LITERAL { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        Deref @ Expression::TYPENAME { .. } => {
            (exp.clone(), diffArguments.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateExpression")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((exp, diffArguments))
}

pub fn differentiateExpressionNoCollect(mut expr: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut expr: Arc<Expression::NFExpression> = expr;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut oldCollect: bool = false;
    if Util::isSome(diffArguments.adjoint_map.clone()) {
        oldCollect = diffArguments.collectAdjoints.clone();
        assign_field!(diffArguments.collectAdjoints = false);
        (expr, diffArguments) = differentiateExpression(expr.clone(), diffArguments.clone())?;
        assign_field!(diffArguments.collectAdjoints = oldCollect.clone());
    } else {
        (expr, diffArguments) = differentiateExpression(expr.clone(), diffArguments.clone())?;
    }
    Ok((expr, diffArguments))
}

pub fn differentiateComponentRef(mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut der_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut derCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut strippedCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    var_ptr = (::match_deref::match_deref! { match &(exp.clone()) {
        _ if (diffArguments.diffType.clone() == DifferentiationType::FUNCTION.clone()) => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::EMPTY, .. } => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. } => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { .. } => BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateComponentRef")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF] exp=")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(" | diffType=")); __mm_s.push_str(&*DifferentiationArguments::diffTypeStr(diffArguments.diffType.clone())); __mm_s.push_str(&*literal!(" | scalarized=")); __mm_s.push_str(&*boolString(diffArguments.scalarized.clone())); __mm_s.push_str(&*literal!(" | collectAdjoints=")); __mm_s.push_str(&*boolString(diffArguments.collectAdjoints.clone())); ArcStr::from(__mm_s) }).clone())?;
    if Util::isSome(diffArguments.adjoint_map.clone()) {
        dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF] current_grad=")); __mm_s.push_str(&*Expression::toString(diffArguments.current_grad.clone())?); ArcStr::from(__mm_s) }).clone())?;
    }
    (exp, diffArguments) = (::match_deref::match_deref! { match &((exp.clone(), diffArguments.diffType.clone(), diffArguments.diff_map.clone())) {
        (Deref @ Expression::CREF { cref: Deref @ ComponentRef::EMPTY, .. }, _, _) => {
            (exp.clone(), diffArguments.clone())
        },
        (Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. }, _, _) => {
            (exp.clone(), diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::FUNCTION { .. }, Some(diff_map)) => {
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            strippedCref = ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone());
            if UnorderedMap::contains(strippedCref.clone(), diff_map.clone()) {
                derCref = UnorderedMap::getOrFail(strippedCref.clone(), diff_map.clone());
                derCref = ComponentRef::copySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), derCref.clone())?;
                res = Expression::fromCref(derCref.clone(), false)?;
            } else {
                res = Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?;
            }
            (res.clone(), diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (ComponentRef::isTime(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) => {
            (Expression::makeOne(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, _, _) if (ComponentRef::isTime(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, _, _) if (BVariable::isStart(var_ptr.clone())) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::SIMPLE, _) if (ComponentRef::isEqual(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), diffArguments.diffCref.clone())?) => {
            (Expression::makeOne(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::SIMPLE, _) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, _, _) if (BVariable::isParamOrConst(var_ptr.clone()) && !(ComponentRef::isTopLevel(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()) && BVariable::isInput(var_ptr.clone())) && !(BVariable::isOptimizable(var_ptr.clone()))) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (BVariable::isDiscrete(var_ptr.clone()) || BVariable::isDiscreteState(var_ptr.clone())) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, Some(diff_map)) if (UnorderedMap::contains(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()), diff_map.clone())) => {
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            derCref = UnorderedMap::getOrFail(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()), diff_map.clone());
            derCref = ComponentRef::copySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), derCref.clone())?;
            res = Expression::fromCref(derCref.clone(), false)?;
            (res.clone(), diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (BVariable::isDummyState(var_ptr.clone())) => {
            (Expression::fromCref(BVariable::getPartnerCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(fnptr!(BVariable::getVarDummyDer, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>), false)?, false)?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (BVariable::isState(var_ptr.clone())) => {
            (Expression::fromCref(BVariable::getPartnerCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(fnptr!(BVariable::getVarDer, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>), false)?, false)?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (BVariable::isContinuous(var_ptr.clone(), false)?) => {
            (derCref, der_ptr) = BVariable::makeDerVar(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), false)?;
            assign_field!(diffArguments.new_vars = cons(der_ptr.clone(), diffArguments.new_vars.clone()));
            BVariable::setStateDerivativeVar(var_ptr.clone(), der_ptr.clone());
            (Expression::fromCref(derCref.clone(), false)?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::JACOBIAN, Some(diff_map)) if (diffArguments.scalarized.clone()) => {
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if UnorderedMap::contains(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), diff_map.clone()) {
                res = Expression::fromCref(UnorderedMap::getOrFail(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), diff_map.clone()), false)?;
                if diffArguments.collectAdjoints.clone() {
                    UnorderedMap::tryAddUpdate(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), Arc::new({ let __pe_b1 = diffArguments.current_grad.clone(); move |__pe_a0| Ok(updateAdjointList(__pe_a0, __pe_b1.clone())) }), Util::getOption(diffArguments.adjoint_map.clone())?)?;
                }
            } else {
                res = Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?;
            }
            (res.clone(), diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::JACOBIAN, Some(diff_map)) if (!(diffArguments.scalarized.clone())) => {
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut adjExpr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut expCrefSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            strippedCref = ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone());
            expCrefSubscripts = ComponentRef::subscriptsAllFlat(var_field!((*exp).cref, Expression::NFExpression::CREF).clone());
            dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] cref=")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?); __mm_s.push_str(&*literal!(" | stripped=")); __mm_s.push_str(&*ComponentRef::toString(strippedCref.clone())?); __mm_s.push_str(&*literal!(" | subs=")); __mm_s.push_str(&*Subscript::toStringList(expCrefSubscripts.clone())?); ArcStr::from(__mm_s) }).clone())?;
            if UnorderedMap::contains(strippedCref.clone(), diff_map.clone()) {
                derCref = UnorderedMap::getOrFail(strippedCref.clone(), diff_map.clone());
                dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] mapped -> ")); __mm_s.push_str(&*ComponentRef::toString(derCref.clone())?); ArcStr::from(__mm_s) }).clone())?;
                res = Expression::fromCref(ComponentRef::copySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), derCref.clone())?, false)?;
                dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] get variable for derivative cref: ")); __mm_s.push_str(&*BVariable::pointerToString(BVariable::getVarPointer(derCref.clone(), metamodelica::sourceInfo!())?)); ArcStr::from(__mm_s) }).clone())?;
                if diffArguments.collectAdjoints.clone() {
                    adjExpr = (::match_deref::match_deref! { match &(expCrefSubscripts.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Subscript::INDEX { index: Deref @ Expression::INTEGER { value: iidx } }, tail: Deref @ metamodelica::List::Nil } => {
            let mut onehotOpt: Option<Arc<Expression::NFExpression>> = None;
            dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] adjoint via INDEX[")); __mm_s.push_str(&*intString(iidx.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone())?;
            onehotOpt = buildOneHotVectorAdjoint(derCref.clone(), iidx.clone(), diffArguments.current_grad.clone())?;
            if (Util::isSome(onehotOpt.clone())) {Util::getOption(onehotOpt.clone())?} else {diffArguments.current_grad.clone()}
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Subscript::SLICE { .. }, tail: Deref @ metamodelica::List::Nil } => {
            let mut multiOpt: Option<Arc<Expression::NFExpression>> = None;
            dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] adjoint via SLICE ")); __mm_s.push_str(&*Subscript::toString(listHead(expCrefSubscripts.clone())?)?); ArcStr::from(__mm_s) }).clone())?;
            multiOpt = buildMultiHotVectorAdjoint(derCref.clone(), listHead(expCrefSubscripts.clone())?, diffArguments.current_grad.clone())?;
            if (Util::isSome(multiOpt.clone())) {Util::getOption(multiOpt.clone())?} else {diffArguments.current_grad.clone()}
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Subscript::WHOLE, tail: Deref @ metamodelica::List::Nil } => {
            diffArguments.current_grad.clone()
        },
        _ => {
            diffArguments.current_grad.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    dbg(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] append adjoint key=")); __mm_s.push_str(&*ComponentRef::toString(derCref.clone())?); __mm_s.push_str(&*literal!(" expr=")); __mm_s.push_str(&*Expression::toString(adjExpr.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    UnorderedMap::tryAddUpdate(derCref.clone(), Arc::new({ let __pe_b1 = adjExpr.clone(); move |__pe_a0| Ok(updateAdjointList(__pe_a0, __pe_b1.clone())) }), Util::getOption(diffArguments.adjoint_map.clone())?)?;
                } else {
                    dbg((literal!("[dCREF:JAC] collectAdjoints=false, skip append")).clone())?;
                }
            } else {
                res = Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?;
            }
            (res.clone(), diffArguments.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateComponentRef")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, diffArguments))
}

pub fn differentiateComponentRefNoCollect(mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut oldCollect: bool = false;
    if Util::isSome(diffArguments.adjoint_map.clone()) {
        oldCollect = diffArguments.collectAdjoints.clone();
        assign_field!(diffArguments.collectAdjoints = false);
        (exp, diffArguments) = differentiateComponentRef(exp.clone(), diffArguments.clone())?;
        assign_field!(diffArguments.collectAdjoints = oldCollect.clone());
    } else {
        (exp, diffArguments) = differentiateComponentRef(exp.clone(), diffArguments.clone())?;
    }
    Ok((exp, diffArguments))
}

pub fn differentiateVariablePointer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut diffArguments_ptr: Pointer::Pointer<Arc<DifferentiationArguments::DifferentiationArguments>>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut diff_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = Pointer::access(diffArguments_ptr.clone());
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut crefExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (crefExp, diffArguments) = differentiateComponentRefNoCollect(Expression::fromCref(var.name.clone(), false)?, diffArguments.clone())?;
    diff_ptr = (::match_deref::match_deref! { match &(crefExp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::EMPTY, .. } => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. } => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { .. } => BVariable::getVarPointer(var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateVariablePointer")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*Variable::toString(var.clone(), (literal!("")).clone(), false)?); __mm_s.push_str(&*literal!(" because the result is expected to be a variable but turned out to be ")); __mm_s.push_str(&*Expression::toString(crefExp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(diffArguments_ptr.clone(), diffArguments.clone());
    Ok(diff_ptr)
}

pub fn differentiateCall(mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let debug: bool = false;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate Exp-Call: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (exp, diffArguments) = ({
        let mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(exp.clone()) {
        ret @ Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret = (*ret).clone();
            let mut call = (*call).clone();
            (arg, diffArguments) = differentiateExpression(var_field!((*call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), diffArguments.clone())?;
            assign_variant_field!(call => Call::NFCall::TYPED_ARRAY_CONSTRUCTOR; exp = arg.clone());
            assign_variant_field!(ret => Expression::NFExpression::CALL; call = call.clone());
            (ret.clone(), diffArguments.clone())
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (ret, diffArguments) = differentiateReduction((AbsynUtil::pathString(NFFunction::Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_REDUCTION).clone())?, (literal!(".")).clone(), true, false)?).clone(), exp.clone(), diffArguments.clone())?;
            (ret.clone(), diffArguments.clone())
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (NFFunction::Function::isBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())) => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (ret, diffArguments) = differentiateBuiltinCall((AbsynUtil::pathString(NFFunction::Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?).clone(), exp.clone(), diffArguments.clone())?;
            (ret.clone(), diffArguments.clone())
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut func_opt: Option<Arc<Function::Function>> = None;
            let mut der_func_opt: Option<Arc<Function::Function>> = None;
            let mut func: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut der_func: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut arguments_inputs: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>> = metamodelica::nil();
            let mut inp: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut isCont: bool = false;
            let mut isReal: bool = false;
            let mut isFunc: bool = false;
            let mut isSkipped: bool = false;
            let mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>;
            func_opt = UnorderedMap::get(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).path.clone(), diffArguments.funcMap.clone());
            if Util::isSome(func_opt.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(func_opt.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                func = __pa0.clone();
                interface_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
                arguments_inputs = List::zip(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone(), func.inputs.clone());
                for mut tpl in &*arguments_inputs.clone() {
                    let mut tpl = tpl.clone();
                    (arg, inp) = tpl.clone();
                    isCont = diffArguments.diffType.clone() == DifferentiationType::FUNCTION.clone() || BackendUtil::isContinuous(arg.clone(), false)?;
                    isReal = Type::isReal(Type::arrayElementType(Expression::typeOf(arg.clone())));
                    isFunc = InstNode::isFunction(inp.clone())?;
                    isSkipped = Util::applyOptionOrDefault(func.interfaceDiffInfo.clone(), Arc::new({ let __pe_b0 = inp.clone(); move |__pe_a1| UnorderedSet::contains(__pe_b0.clone(), __pe_a1) }), false);
                    if isSkipped.clone() || !(isFunc.clone() || isCont.clone() && isReal.clone()) {
                        UnorderedMap::add((InstNode::name(inp.clone())?).clone(), !(isFunc.clone() || isReal.clone()), interface_map.clone())?;
                    }
                }
                der_func_opt = NFFunction::Function::getDerivative(func.clone(), interface_map.clone())?;
                if Util::isSome(der_func_opt.clone()) {
                    let __pa1 = ::match_deref::match_deref! { match &(der_func_opt.clone()) {
                        Some(__pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    der_func = __pa1.clone();
                    (der_func, _) = addDiffInfo(func.clone(), der_func.clone(), diffArguments.clone())?;
                } else {
                    (der_func, diffArguments) = differentiateFunction(func.clone(), interface_map.clone(), diffArguments.clone())?;
                }
                for mut tpl in &*arguments_inputs.clone().reverse() {
                    let mut tpl = tpl.clone();
                    (arg, inp) = tpl.clone();
                    isSkipped = Util::applyOptionOrDefault(func.interfaceDiffInfo.clone(), Arc::new({ let __pe_b0 = inp.clone(); move |__pe_a1| UnorderedSet::contains(__pe_b0.clone(), __pe_a1) }), false);
                    if !(isSkipped.clone() || UnorderedMap::getOrDefault((InstNode::name(inp.clone())?).clone(), interface_map.clone(), false)) {
                        arguments = cons(arg.clone(), arguments.clone());
                    }
                }
                (arguments, diffArguments) = List::mapFold(arguments.clone(), (std::sync::Arc::new(differentiateExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> + 'static>), diffArguments.clone());
                arguments = listAppend(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone(), arguments.clone());
                ret = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(der_func.clone(), arguments.clone(), var_field!((**call).var, Call::NFCall::TYPED_CALL).clone(), var_field!((**call).purity, Call::NFCall::TYPED_CALL).clone(), der_func.returnType.clone()) });
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateCall")); __mm_s.push_str(&*literal!(" failed because the function is not a builtin function and could not be found in the function tree: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            (ret.clone(), diffArguments.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate-ExpCall-result: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((exp, diffArguments))
}

pub fn differentiateReduction(mut name: ArcStr, mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } if (name.clone() == literal!("sum")) => {
            let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut call = (*call).clone();
            (arg, diffArguments) = differentiateExpression(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), diffArguments.clone())?;
            assign_variant_field!(call => Call::NFCall::TYPED_REDUCTION; exp = arg.clone());
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateReduction")); __mm_s.push_str(&*literal!(" failed because of non-call expression: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, diffArguments))
}

pub fn differentiateBuiltinCall(mut name: ArcStr, mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR.clone();
    let mut addOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
    let mut mulOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
    exp = ({
        let mut isReverse: bool = Util::isSome(diffArguments.adjoint_map.clone());
        (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("delay")) => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffType: DifferentiationType = DifferentiationType::TIME;
            (arg1, arg2, arg3) = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Cons { head: arg3, tail: Deref @ metamodelica::List::Nil } } } => (arg1.clone(), arg2.clone(), arg3.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ret1 = Arc::new(Expression::NFExpression::REAL { value: if (diffArguments.diffType.clone() == DifferentiationType::TIME.clone()) {metamodelica::OrderedFloat(1.0_f64)} else {metamodelica::OrderedFloat(0.0_f64)} });
            (ret2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments.clone())?;
            ret2 = SimplifyExp::simplifyDump(Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret1.clone()], inv_arguments: list![ret2.clone()], operator: addOp.clone() }), true, (literal!("NBDifferentiate.differentiateBuiltinCall")).clone(), (literal!("")).clone())?;
            if Expression::isZero(ret2.clone()) {
                ret = Expression::makeZero(Expression::typeOf(arg1.clone()))?;
            } else {
                diffType = diffArguments.diffType.clone();
                assign_field!(diffArguments.diffType = DifferentiationType::TIME.clone());
                (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.diffType = diffType.clone());
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1.clone(), arg2.clone(), arg3.clone()])?);
                ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret2.clone(), exp.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() });
            }
            ret.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("smooth")) => {
            let mut i: i32 = 0;
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1 @ Deref @ Expression::INTEGER { value: i }, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } if (i.clone() > 0) => {
            (ret2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![Arc::new(Expression::NFExpression::INTEGER { value: i.clone() - 1 }), ret2.clone()])?);
            exp.clone()
        },
        Deref @ metamodelica::List::Cons { head: arg1 @ Deref @ Expression::INTEGER { value: i }, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => {
            (ret2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments.clone())?;
            exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::NO_EVENT().clone(), list![ret2.clone()], Expression::variability(ret2.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::NO_EVENT().returnType.clone()) });
            exp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ret.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("sum")) => {
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            arg1 = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => arg1.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::BINARY { exp1: Expression::makeOne(Expression::typeOf(arg1.clone()))?, operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::ARRAY_SCALAR.clone()), Expression::typeOf(arg1.clone()))?, exp2: current_grad.clone() }));
            }
            (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            if isReverse.clone() {
                assign_field!(diffArguments.current_grad = current_grad.clone());
            }
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1.clone()])?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("symmetric")) => {
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut addM: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut subM: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut sumG: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut triuG: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut nExp: i32 = 0;
            let mut eyeNN: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut mulEW: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut diagG: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            arg1 = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => arg1.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                ty = Expression::typeOf(current_grad.clone());
                elTy = if (Type::isArray(ty.clone())) {Type::arrayElementType(ty.clone())} else {ty.clone()};
                nExp = Dimension::size(listHead(Type::arrayDims(Expression::typeOf(arg1.clone())))?, false)?;
                addM = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), ty.clone())?;
                subM = Operator::fromClassification((Operator::MathClassification::SUBTRACTION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), ty.clone())?;
                mulEW = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), ty.clone())?;
                sumG = Arc::new(Expression::NFExpression::BINARY { exp1: current_grad.clone(), operator: addM.clone(), exp2: typeTransposeCall(current_grad.clone())? });
                triuG = Arc::new(Expression::NFExpression::BINARY { exp1: sumG.clone(), operator: mulEW.clone(), exp2: Expression::makeTriuMask(nExp.clone(), elTy.clone())? });
                eyeNN = Expression::makeIdentityMatrix(nExp.clone(), elTy.clone())?;
                diagG = Arc::new(Expression::NFExpression::BINARY { exp1: current_grad.clone(), operator: mulEW.clone(), exp2: eyeNN.clone() });
                assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::BINARY { exp1: triuG.clone(), operator: subM.clone(), exp2: diagG.clone() }));
            }
            (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            if isReverse.clone() {
                assign_field!(diffArguments.current_grad = current_grad.clone());
            }
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1.clone()])?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("diagonal")) => {
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut nExp: i32 = 0;
            arg1 = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => arg1.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                nExp = Dimension::size(listHead(Type::arrayDims(Expression::typeOf(arg1.clone())))?, false)?;
                assign_field!(diffArguments.current_grad = extractDiagonalVector(current_grad.clone(), nExp.clone(), Expression::typeOf(arg1.clone()))?);
            }
            (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            if isReverse.clone() {
                assign_field!(diffArguments.current_grad = current_grad.clone());
            }
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1.clone()])?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("matrix")) => {
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut grad_x: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut rX: i32 = 0;
            arg1 = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => arg1.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                ty = Expression::typeOf(arg1.clone());
                rX = if (Type::isArray(ty.clone())) {Type::dimensionCount(ty.clone())} else {0};
                grad_x = current_grad.clone();
                if rX.clone() < 2 {
                    for mut i in 1..=2 - rX.clone() {
                        grad_x = dropLastDimIndex1(grad_x.clone())?;
                    }
                } else if rX.clone() > 2 {
                    grad_x = typePromoteCall(grad_x.clone(), rX.clone());
                }
                assign_field!(diffArguments.current_grad = grad_x.clone());
                (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = current_grad.clone());
            } else {
                (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            }
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1.clone()])?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (List::contains(list![(literal!("pre")).clone(), (literal!("noEvent")).clone(), (literal!("scalar")).clone(), (literal!("vector")).clone(), (literal!("transpose")).clone(), (literal!("skew")).clone()], (name.clone()).clone(), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) => {
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            arg1 = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => arg1.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1.clone()])?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (List::contains(list![(literal!("homotopy")).clone(), (literal!("$OMC$inStreamDiv")).clone()], (name.clone()).clone(), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) => {
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (arg1, arg2) = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => (arg1.clone(), arg2.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            (ret2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1.clone(), ret2.clone()])?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("cat")) => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut diffRest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            if isReverse.clone() {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\nReverse Mode not implemented for `cat()`.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                bail!("fail");
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg1 = __pa0.clone();
            rest = __pa1.clone();
            diffRest = metamodelica::nil();
            for mut arg in &*rest.clone().reverse() {
                let mut arg = arg.clone();
                (ret, diffArguments) = differentiateExpression(arg.clone(), diffArguments.clone())?;
                diffRest = cons(ret.clone(), diffRest.clone());
            }
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), cons(arg1.clone(), diffRest.clone()))?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("promote")) => {
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut old_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rY: i32 = 0;
            let mut rX: i32 = 0;
            (arg1, arg2) = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => (arg1.clone(), arg2.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if isReverse.clone() {
                rY = if (Type::isArray(Expression::typeOf(exp.clone()))) {Type::dimensionCount(Expression::typeOf(exp.clone()))} else {0};
                rX = if (Type::isArray(Expression::typeOf(arg1.clone()))) {Type::dimensionCount(Expression::typeOf(arg1.clone()))} else {0};
                current_grad = diffArguments.current_grad.clone();
                old_grad = current_grad.clone();
                for mut i in 1..=std::cmp::max(0, rY.clone() - rX.clone()) {
                    current_grad = dropLastDimIndex1(current_grad.clone())?;
                }
                assign_field!(diffArguments.current_grad = current_grad.clone());
                (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = old_grad.clone());
            } else {
                (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            }
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1.clone(), arg2.clone()])?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("identity")) => {
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            arg1 = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => arg1.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::FILL_FUNC().clone(), list![Arc::new(Expression::NFExpression::INTEGER { value: 0 }), arg1.clone(), arg1.clone()], Variability::CONSTANT.clone(), Prefixes::Purity::PURE.clone(), BuiltinFuncs::FILL_FUNC().returnType.clone()) })
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("fill")) => {
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut old_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut rY: i32 = 0;
            let mut rX: i32 = 0;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg1 = __pa0.clone();
            rest = __pa1.clone();
            if isReverse.clone() {
                rY = if (Type::isArray(Expression::typeOf(exp.clone()))) {Type::dimensionCount(Expression::typeOf(exp.clone()))} else {0};
                rX = if (Type::isArray(Expression::typeOf(arg1.clone()))) {Type::dimensionCount(Expression::typeOf(arg1.clone()))} else {0};
                current_grad = diffArguments.current_grad.clone();
                old_grad = current_grad.clone();
                for mut i in 1..=std::cmp::max(0, rY.clone() - rX.clone()) {
                    current_grad = typeSumCall(current_grad.clone());
                }
                assign_field!(diffArguments.current_grad = current_grad.clone());
                (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = old_grad.clone());
            } else {
                (ret1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            }
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), cons(ret1.clone(), rest.clone()))?);
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("semiLinear")) => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffArg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffArg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffArg3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut grad_x: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            (arg1, arg2, arg3) = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Cons { head: arg3, tail: Deref @ metamodelica::List::Nil } } } => (arg1.clone(), arg2.clone(), arg3.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            current_grad = diffArguments.current_grad.clone();
            if isReverse.clone() {
                cond = Arc::new(Expression::NFExpression::RELATION { exp1: arg1.clone(), operator: Operator::makeGreaterEq(Expression::typeOf(arg1.clone())), exp2: Expression::makeZero(Expression::typeOf(arg1.clone()))?, index: -1 });
                grad_x = Arc::new(Expression::NFExpression::IF { ty: Expression::typeOf(arg1.clone()), condition: cond.clone(), trueBranch: Arc::new(Expression::NFExpression::MULTARY { arguments: list![arg2.clone(), current_grad.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() }), falseBranch: Arc::new(Expression::NFExpression::MULTARY { arguments: list![arg3.clone(), current_grad.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() }) });
                assign_field!(diffArguments.current_grad = grad_x.clone());
            }
            (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            assign_field!(diffArguments.current_grad = current_grad.clone());
            (diffArg2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments.clone())?;
            (diffArg3, diffArguments) = differentiateExpression(arg3.clone(), diffArguments.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![arg1.clone(), diffArg2.clone(), diffArg3.clone()])?);
            ret = exp.clone();
            if !(Expression::isZero(diffArg1.clone())) {
                ty = Expression::typeOf(diffArg1.clone());
                ret1 = Arc::new(Expression::NFExpression::RELATION { exp1: arg1.clone(), operator: Operator::makeGreaterEq(ty.clone()), exp2: Expression::makeZero(ty.clone())?, index: -1 });
                ret1 = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: ret1.clone(), trueBranch: arg2.clone(), falseBranch: arg3.clone() });
                ret2 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![diffArg1.clone(), ret1.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() });
                ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret.clone(), ret2.clone()], inv_arguments: metamodelica::nil(), operator: addOp.clone() });
            }
            ret.clone()
        },
        Deref @ Expression::CALL { .. } if (name.clone() == literal!("min") || name.clone() == literal!("max")) => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffArg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffArg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut cond1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut cond2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut zero1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut zero2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut grad_x: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut grad_y: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut old_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            ret = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => {
            (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            ty = Expression::typeOf(diffArg1.clone());
            if Expression::isZero(diffArg1.clone()) {
                ret = Expression::makeZero(Type::arrayElementType(ty.clone()))?;
            } else {
                ret1 = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(if (name.clone() == literal!("min")) {BuiltinFuncs::ARG_MIN_ARR_REAL().clone()} else {BuiltinFuncs::ARG_MAX_ARR_REAL().clone()}, list![arg1.clone()], Expression::variability(arg1.clone())?, Prefixes::Purity::PURE.clone(), r#fn.returnType.clone()) });
                ret = Expression::applySubscripts(list![Arc::new(Subscript::NFSubscript::INDEX { index: ret1.clone() })], diffArg1.clone(), true)?;
            }
            ret.clone()
        },
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => {
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                cond1 = Arc::new(Expression::NFExpression::RELATION { exp1: arg1.clone(), operator: if (name.clone() == literal!("min")) {Operator::makeLess(Expression::typeOf(arg1.clone()))} else {Operator::makeGreater(Expression::typeOf(arg1.clone()))}, exp2: arg2.clone(), index: -1 });
                cond2 = Arc::new(Expression::NFExpression::RELATION { exp1: arg2.clone(), operator: if (name.clone() == literal!("min")) {Operator::makeLess(Expression::typeOf(arg2.clone()))} else {Operator::makeGreater(Expression::typeOf(arg2.clone()))}, exp2: arg1.clone(), index: -1 });
                zero1 = Expression::makeZero(Expression::typeOf(arg1.clone()))?;
                zero2 = Expression::makeZero(Expression::typeOf(arg2.clone()))?;
                grad_x = Arc::new(Expression::NFExpression::IF { ty: Expression::typeOf(arg1.clone()), condition: cond1.clone(), trueBranch: current_grad.clone(), falseBranch: zero1.clone() });
                grad_y = Arc::new(Expression::NFExpression::IF { ty: Expression::typeOf(arg2.clone()), condition: cond2.clone(), trueBranch: current_grad.clone(), falseBranch: zero2.clone() });
                old_grad = diffArguments.current_grad.clone();
                assign_field!(diffArguments.current_grad = grad_x.clone());
                (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = grad_y.clone());
                (diffArg2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = old_grad.clone());
            } else {
                (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
                (diffArg2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments.clone())?;
            }
            ty = Expression::typeOf(diffArg1.clone());
            if Expression::isZero(diffArg1.clone()) && Expression::isZero(diffArg2.clone()) {
                ret = Expression::makeZero(ty.clone())?;
            } else {
                ret1 = Arc::new(Expression::NFExpression::RELATION { exp1: arg1.clone(), operator: if (name.clone() == literal!("min")) {Operator::makeLess(ty.clone())} else {Operator::makeGreater(ty.clone())}, exp2: arg2.clone(), index: -1 });
                ret = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: ret1.clone(), trueBranch: diffArg1.clone(), falseBranch: diffArg2.clone() });
            }
            ret.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ret.clone()
        },
        Deref @ Expression::CALL { .. } if (List::hasOneElement(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?)) => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffArg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            arg1 = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => arg1.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ret = differentiateBuiltinCall1Arg((name.clone()).clone(), arg1.clone())?;
            if !(Expression::isZero(ret.clone())) {
                current_grad = diffArguments.current_grad.clone();
                assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::MULTARY { arguments: list![current_grad.clone(), ret.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() }));
                (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = current_grad.clone());
                ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret.clone(), diffArg1.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() });
            }
            ret.clone()
        },
        Deref @ Expression::CALL { .. } if ((Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?.len() as i32) == 2) => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffArg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffArg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (arg1, arg2) = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => (arg1.clone(), arg2.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (ret1, ret2) = differentiateBuiltinCall2Arg((name.clone()).clone(), arg1.clone(), arg2.clone())?;
            current_grad = diffArguments.current_grad.clone();
            assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::MULTARY { arguments: list![current_grad.clone(), ret1.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() }));
            (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments.clone())?;
            assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::MULTARY { arguments: list![current_grad.clone(), ret2.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() }));
            (diffArg2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments.clone())?;
            assign_field!(diffArguments.current_grad = current_grad.clone());
            ret1 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret1.clone(), diffArg1.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() });
            ret2 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret2.clone(), diffArg2.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret1.clone(), ret2.clone()], inv_arguments: metamodelica::nil(), operator: addOp.clone() });
            ret.clone()
        },
        Deref @ Expression::CALL { .. } => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = (::match_deref::match_deref! { match &(Call::functionNameLast(var_field!((*exp).call, Expression::NFExpression::CALL).clone())) {
        Deref @ "sample" => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ret.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed because of non-call expression: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((exp, diffArguments))
}

pub fn differentiateBuiltinCall1Arg(mut name: ArcStr, mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut derFuncCall: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR.clone();
    let mut powOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::POWER.clone(), sizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
    let mut addOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
    let mut mulOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
    derFuncCall = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "sign" => {
            Arc::new(Expression::NFExpression::INTEGER { value: 0 })
        },
        Deref @ "ceil" => {
            Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ "floor" => {
            Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ "integer" => {
            Arc::new(Expression::NFExpression::INTEGER { value: 0 })
        },
        Deref @ "abs" => {
            Arc::new(Expression::NFExpression::CAST { ty: Expression::typeOf(arg.clone()), exp: Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::SIGN().clone(), list![arg.clone()], Expression::variability(arg.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::SIGN().returnType.clone()) }) })
        },
        Deref @ "sqrt" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: arg.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) })], inv_arguments: list![ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        Deref @ "sin" => {
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::COS_REAL().clone(), list![arg.clone()], Expression::variability(arg.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::COS_REAL().returnType.clone()) })
        },
        Deref @ "cos" => {
            Expression::negate(Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::SIN_REAL().clone(), list![arg.clone()], Expression::variability(arg.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::SIN_REAL().returnType.clone()) }))
        },
        Deref @ "tan" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::COS_REAL().clone(), list![arg.clone()], Expression::variability(arg.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::COS_REAL().returnType.clone()) });
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: ret.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        Deref @ "asin" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: arg.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: addOp.clone() });
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: ret.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        Deref @ "acos" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: arg.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: addOp.clone() });
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: ret.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(-1.0_f64) })], inv_arguments: list![ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        Deref @ "atan" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: arg.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) }), ret.clone()], inv_arguments: metamodelica::nil(), operator: addOp.clone() });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        Deref @ "sinh" => {
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::COSH_REAL().clone(), list![arg.clone()], Expression::variability(arg.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::COSH_REAL().returnType.clone()) })
        },
        Deref @ "cosh" => {
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::SINH_REAL().clone(), list![arg.clone()], Expression::variability(arg.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::SINH_REAL().returnType.clone()) })
        },
        Deref @ "tanh" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::TANH_REAL().clone(), list![arg.clone()], Expression::variability(arg.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::TANH_REAL().returnType.clone()) });
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: ret.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: addOp.clone() });
            ret.clone()
        },
        Deref @ "acosh" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: arg.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret.clone()], inv_arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], operator: addOp.clone() });
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: ret.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        Deref @ "asinh" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: arg.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![ret.clone(), Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: metamodelica::nil(), operator: addOp.clone() });
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: ret.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        Deref @ "atanh" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::BINARY { exp1: arg.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: addOp.clone() });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        Deref @ "exp" => {
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::EXP_REAL().clone(), list![arg.clone()], Expression::variability(arg.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::EXP_REAL().returnType.clone()) })
        },
        Deref @ "log" => {
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![arg.clone()], operator: mulOp.clone() })
        },
        Deref @ "log10" => {
            let mut ret: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ret = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::LOG_REAL().clone(), list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(10.0_f64) })], Variability::CONSTANT.clone(), Prefixes::Purity::PURE.clone(), BuiltinFuncs::LOG_REAL().returnType.clone()) });
            ret = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![arg.clone(), ret.clone()], operator: mulOp.clone() });
            ret.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall1Arg")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(derFuncCall)
}

pub fn differentiateBuiltinCall2Arg(mut name: ArcStr, mut arg1: Arc<Expression::NFExpression>, mut arg2: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)> {
    let mut derFuncCall1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut derFuncCall2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR.clone();
    let mut powOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::POWER.clone(), sizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
    let mut addOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
    let mut mulOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
    (derFuncCall1, derFuncCall2) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "div" => {
            (Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Arc::new(Expression::NFExpression::INTEGER { value: 0 }))
        },
        Deref @ "mod" => {
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp2 = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::FLOOR().clone(), list![Arc::new(Expression::NFExpression::MULTARY { arguments: list![arg1.clone()], inv_arguments: list![arg2.clone()], operator: mulOp.clone() })], Prefixes::variabilityMax(Expression::variability(arg1.clone())?, Expression::variability(arg2.clone())?), Prefixes::Purity::PURE.clone(), BuiltinFuncs::FLOOR().returnType.clone()) });
            ret2 = Expression::negate(exp2.clone());
            (Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((1) as f64) }), ret2.clone())
        },
        Deref @ "rem" => {
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp2 = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::DIV_REAL().clone(), list![arg1.clone(), arg2.clone()], Prefixes::variabilityMax(Expression::variability(arg1.clone())?, Expression::variability(arg2.clone())?), Prefixes::Purity::PURE.clone(), BuiltinFuncs::DIV_REAL().returnType.clone()) });
            ret2 = Expression::negate(exp2.clone());
            (Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((1) as f64) }), ret2.clone())
        },
        Deref @ "atan2" => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ret2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = Arc::new(Expression::NFExpression::BINARY { exp1: arg1.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            exp2 = Arc::new(Expression::NFExpression::BINARY { exp1: arg2.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            exp1 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![exp1.clone(), exp2.clone()], inv_arguments: metamodelica::nil(), operator: addOp.clone() });
            ret1 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Expression::negate(arg2.clone())], inv_arguments: list![exp1.clone()], operator: mulOp.clone() });
            ret2 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![arg1.clone()], inv_arguments: list![exp1.clone()], operator: mulOp.clone() });
            (ret1.clone(), ret2.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall2Arg")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((derFuncCall1, derFuncCall2))
}

pub fn addDiffInfo(mut func: Arc<Function::Function>, mut der_func: Arc<Function::Function>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Function::Function>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut der_func: Arc<Function::Function> = der_func;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diffInfo: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>;
    diffInfo = (::match_deref::match_deref! { match &(func.interfaceDiffInfo.clone()) {
        Some(diffInfo) => UnorderedSet::copy(diffInfo.clone()),
        _ => UnorderedSet::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::nameEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 13),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut node in &*func.inputs.clone() {
        let mut node = node.clone();
        UnorderedSet::add(node.clone(), diffInfo.clone())?;
    }
    assign_field!(der_func.interfaceDiffInfo = Some(diffInfo.clone()));
    UnorderedMap::add(der_func.path.clone(), der_func.clone(), diffArguments.funcMap.clone())?;
    Ok((der_func, diffArguments))
}

pub fn differentiateFunction(mut func: Arc<Function::Function>, mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Function::Function>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut der_func: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    der_func = ({
        let mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        (::match_deref::match_deref! { match &(func.clone()) {
        der_func @ Deref @ NFFunction::Function::FUNCTION { node: node @ Deref @ InstNode::CLASS_NODE { cls, .. }, .. } => {
            let mut new_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            let mut funcDiffArgs: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
            let mut diffInfo: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>;
            let mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
            let mut funcDer: Arc<FunctionDerivative::NFFunctionDerivative> = Arc::new(<FunctionDerivative::NFFunctionDerivative as ::std::default::Default>::default());
            let mut dummy_func: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut cachedData: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
            let mut der_func_name: ArcStr = arcstr::literal!("");
            let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
            let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
            let mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
            let mut local_outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
            let mut slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
            let mut der_func = (*der_func).clone();
            let mut node = (*node).clone();
            new_cls = (::match_deref::match_deref! { match &(Pointer::access(cls.clone())) {
        new_cls @ Deref @ Class::INSTANCED_CLASS { .. } => {
            let mut new_cls = (*new_cls).clone();
            local_outputs = ({
        let mut __acc: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        for mut lout in (der_func.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::setComponentDirection(Prefixes::Direction::NONE.clone(), lout.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            local_outputs = ({
        let mut __acc: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        for mut lout in (local_outputs.clone()).into_iter().cloned() {
            let __x = InstNode::protect(lout.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            funcDiffArgs = DifferentiationArguments::default(DifferentiationType::TIME.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Path>, Arc<Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Path>) -> Result<bool> + 'static>), 1));
            assign_field!(
                funcDiffArgs.diffType = DifferentiationType::FUNCTION.clone(),
                funcDiffArgs.funcMap = diffArguments.funcMap.clone()
            );
            diffInfo = (::match_deref::match_deref! { match &(der_func.interfaceDiffInfo.clone()) {
        Some(diffInfo) => UnorderedSet::copy(diffInfo.clone()),
        _ => UnorderedSet::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::nameEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 13),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            createInterfaceDerivatives(der_func.inputs.clone(), interface_map.clone(), diff_map.clone())?;
            createInterfaceDerivatives(der_func.locals.clone(), interface_map.clone(), diff_map.clone())?;
            createInterfaceDerivatives(der_func.outputs.clone(), interface_map.clone(), diff_map.clone())?;
            assign_field!(funcDiffArgs.diff_map = Some(diff_map.clone()));
            (inputs, funcDiffArgs) = differentiateFunctionInterfaceNodes(der_func.inputs.clone(), interface_map.clone(), diff_map.clone(), funcDiffArgs.clone(), diffInfo.clone(), true)?;
            (locals, funcDiffArgs) = differentiateFunctionInterfaceNodes(der_func.locals.clone(), interface_map.clone(), diff_map.clone(), funcDiffArgs.clone(), diffInfo.clone(), false)?;
            (outputs, funcDiffArgs) = differentiateFunctionInterfaceNodes(der_func.outputs.clone(), interface_map.clone(), diff_map.clone(), funcDiffArgs.clone(), diffInfo.clone(), false)?;
            assign_field!(
                der_func.inputs = inputs.clone(),
                der_func.locals = List::flatten(list![der_func.locals.clone(), locals.clone(), local_outputs.clone()]),
                der_func.outputs = outputs.clone()
            );
            assign_variant_field!(new_cls => Class::NFClass::INSTANCED_CLASS; elements = ClassTree::appendComponentsToFlatTree(locals.clone(), var_field!((*new_cls).elements, Class::NFClass::INSTANCED_CLASS).clone())?);
            (slots, funcDiffArgs) = createSlotDerivatives(der_func.slots.clone(), interface_map.clone(), diff_map.clone(), funcDiffArgs.clone())?;
            assign_field!(der_func.slots = listAppend(der_func.slots.clone(), slots.clone()));
            dummy_func = func.clone();
            assign_variant_field!(node => InstNode::InstNode::CLASS_NODE; cls = Pointer::create(new_cls.clone()));
            der_func_name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BVariable::FUNCTION_DERIVATIVE_STR)); __mm_s.push_str(&*intString((func.derivatives.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
            assign_variant_field!(node => InstNode::InstNode::CLASS_NODE;
                name = { let mut __mm_s = String::new(); __mm_s.push_str(&*der_func_name.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*node).name, InstNode::InstNode::CLASS_NODE).clone()); ArcStr::from(__mm_s) },
                definition = SCodeUtil::setElementName(var_field!((*node).definition, InstNode::InstNode::CLASS_NODE).clone(), (var_field!((*node).name, InstNode::InstNode::CLASS_NODE).clone()).clone())
            );
            assign_field!(
                der_func.path = AbsynUtil::prefixPath((der_func_name.clone()).clone(), der_func.path.clone()),
                der_func.derivatives = metamodelica::nil(),
                der_func.derivedInputs = metamodelica::nil(),
                der_func.interfaceDiffInfo = Some(diffInfo.clone())
            );
            cachedData = Arc::new(CachedData::CachedData::FUNCTION { funcs: list![der_func.clone()], typed: true, specialBuiltin: false });
            assign_field!(der_func.node = InstNode::newFuncCache(node.clone(), cachedData.clone())?);
            funcDer = Arc::new(FunctionDerivative::NFFunctionDerivative { lowerOrderDerivatives: metamodelica::nil(), conditions: FunctionDerivative::conditionsFromMap(interface_map.clone()), order: Arc::new(Expression::NFExpression::INTEGER { value: 1 }), derivedFn: dummy_func.node.clone(), derivativeFn: der_func.node.clone() });
            assign_field!(dummy_func.derivatives = cons(funcDer.clone(), dummy_func.derivatives.clone()));
            UnorderedMap::add(dummy_func.path.clone(), dummy_func.clone(), funcDiffArgs.funcMap.clone())?;
            funcDiffArgs = (::match_deref::match_deref! { match &(var_field!((*new_cls).sections, Class::NFClass::INSTANCED_CLASS).clone()) {
        sections @ Deref @ Sections::SECTIONS { .. } => {
            let mut sections = (*sections).clone();
            (algorithms, funcDiffArgs) = List::mapFold(var_field!((*sections).algorithms, Sections::NFSections::SECTIONS).clone(), (std::sync::Arc::new(differentiateAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Algorithm::NFAlgorithm>, Arc<DifferentiationArguments::DifferentiationArguments>)> + 'static>), funcDiffArgs.clone());
            assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = algorithms.clone());
            assign_variant_field!(new_cls => Class::NFClass::INSTANCED_CLASS; sections = sections.clone());
            funcDiffArgs.clone()
        },
        _ => {
            funcDiffArgs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            assign_variant_field!(node => InstNode::InstNode::CLASS_NODE; cls = Pointer::create(new_cls.clone()));
            assign_field!(
                der_func.derivatives = metamodelica::nil(),
                der_func.derivedInputs = metamodelica::nil(),
                der_func.interfaceDiffInfo = Some(diffInfo.clone())
            );
            cachedData = Arc::new(CachedData::CachedData::FUNCTION { funcs: list![der_func.clone()], typed: true, specialBuiltin: false });
            assign_field!(der_func.node = InstNode::newFuncCache(node.clone(), cachedData.clone())?);
            assign_field!(diffArguments.funcMap = funcDiffArgs.funcMap.clone());
            new_cls.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateFunction")); __mm_s.push_str(&*literal!(" failed for class ")); __mm_s.push_str(&*Class::toFlatString(Pointer::access(cls.clone()), func.node.clone(), BaseModelica::defaultFormat.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            UnorderedMap::add(der_func.path.clone(), der_func.clone(), diffArguments.funcMap.clone())?;
            funcDer = Arc::new(FunctionDerivative::NFFunctionDerivative { lowerOrderDerivatives: metamodelica::nil(), conditions: FunctionDerivative::conditionsFromMap(interface_map.clone()), order: Arc::new(Expression::NFExpression::INTEGER { value: 1 }), derivedFn: func.node.clone(), derivativeFn: der_func.node.clone() });
            assign_field!(func.derivatives = List::appendElt(funcDer.clone(), func.derivatives.clone()));
            UnorderedMap::add(func.path.clone(), func.clone(), diffArguments.funcMap.clone())?;
            der_func.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateFunction")); __mm_s.push_str(&*literal!(" failed for uninstantiated function ")); __mm_s.push_str(&*NFFunction::Function::signatureString(func.clone(), true)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[BEFORE] ")); __mm_s.push_str(&*NFFunction::Function::toFlatString(func.clone(), BaseModelica::defaultFormat.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[AFTER ] ")); __mm_s.push_str(&*NFFunction::Function::toFlatString(der_func.clone(), BaseModelica::defaultFormat.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((der_func, diffArguments))
}

pub fn differentiateFunctionInterfaceNodes(mut interface_nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments>, mut diffInfo: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>, mut keepOld: bool) -> Result<(Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut interface_nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = interface_nodes;
    let mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments> = diffArgs;
    let mut new_nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut d_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    new_nodes = if (keepOld.clone()) {interface_nodes.clone().reverse()} else {metamodelica::nil()};
    for mut node in &*interface_nodes.clone() {
        let mut node = node.clone();
        if !(UnorderedMap::contains((InstNode::name(node.clone())?).clone(), interface_map.clone())) {
            if !(UnorderedSet::contains(node.clone(), diffInfo.clone())?) {
                (d_node, diffArgs) = differentiateFunctionInterfaceNode(node.clone(), diff_map.clone(), diffArgs.clone())?;
                new_nodes = cons(d_node.clone(), new_nodes.clone());
                UnorderedSet::add(node.clone(), diffInfo.clone())?;
            }
        }
    }
    interface_nodes = new_nodes.clone().reverse();
    Ok((interface_nodes, diffArgs))
}

pub fn differentiateFunctionInterfaceNode(mut node: Arc<InstNode::InstNode>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<InstNode::InstNode>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut d_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments> = diffArgs;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut diff_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut func: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut d_func: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    cref = ComponentRef::fromNode(node.clone(), InstNode::getType(node.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone());
    diff_cref = UnorderedMap::getSafe(cref.clone(), diff_map.clone(), metamodelica::sourceInfo!())?;
    diff_cref = (::match_deref::match_deref! { match &(diff_cref.clone()) {
        Deref @ ComponentRef::CREF { node: d_node @ Deref @ InstNode::COMPONENT_NODE { .. }, .. } => {
            let mut d_node = (*d_node).clone();
            comp = Pointer::access(var_field!((*d_node).component, InstNode::InstNode::COMPONENT_NODE).clone());
            comp = (::match_deref::match_deref! { match &(comp.clone()) {
        comp @ Deref @ Component::COMPONENT { .. } => {
            let mut comp = (*comp).clone();
            (binding, diffArgs) = differentiateBinding(var_field!((*comp).binding, Component::NFComponent::COMPONENT).clone(), diffArgs.clone())?;
            assign_variant_field!(comp => Component::NFComponent::COMPONENT; binding = binding.clone());
            comp.clone()
        },
        _ => comp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            assign_variant_field!(d_node => InstNode::InstNode::COMPONENT_NODE; component = Pointer::create(comp.clone()));
            assign_variant_field!(diff_cref => ComponentRef::NFComponentRef::CREF; node = d_node.clone());
            diff_cref.clone()
        },
        _ => diff_cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if InstNode::isFunction(node.clone())? {
        func = listHead(NFFunction::Function::getCachedFuncs(node.clone())?)?;
        (d_func, diffArgs) = differentiateFunction(func.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1), diffArgs.clone())?;
    }
    d_node = ComponentRef::node(diff_cref.clone())?;
    Ok((d_node, diffArgs))
}

pub fn createInterfaceDerivatives(mut interface_nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    fn addCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        let mut diff_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut children: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        diff_cref = BVariable::makeFDerVar(cref.clone())?;
        UnorderedMap::add(cref.clone(), diff_cref.clone(), diff_map.clone())?;
        children = ComponentRef::getRecordChildren(cref.clone())?;
        for mut child in &*children.clone() {
            let mut child = child.clone();
            addCref(child.clone(), diff_map.clone())?;
        }
        Ok(())
    }

    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    for mut node in &*interface_nodes.clone() {
        let mut node = node.clone();
        if !(UnorderedMap::contains((InstNode::name(node.clone())?).clone(), interface_map.clone())) {
            cref = ComponentRef::fromNode(node.clone(), InstNode::getType(node.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone());
            addCref(cref.clone(), diff_map.clone())?;
        }
    }
    Ok(())
}

pub fn createSlotDerivatives(mut slots: Arc<metamodelica::List<Arc<Slot::Slot>>>, mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<metamodelica::List<Arc<Slot::Slot>>>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut new_slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
    let mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments> = diffArgs;
    let mut d_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut local_index: i32 = (slots.clone().len() as i32) + 1;
    for mut slot in &*slots.clone() {
        let mut slot = slot.clone();
        if !(UnorderedMap::contains((InstNode::name(slot.node.clone())?).clone(), interface_map.clone())) {
            (d_node, diffArgs) = differentiateFunctionInterfaceNode(slot.node.clone(), diff_map.clone(), diffArgs.clone())?;
            assign_field!(
                slot.node = d_node.clone(),
                slot.index = local_index.clone()
            );
            new_slots = cons(slot.clone(), new_slots.clone());
            local_index = local_index.clone() + 1;
        }
    }
    new_slots = new_slots.clone().reverse();
    Ok((new_slots, diffArgs))
}

pub fn resolvePartialDerivatives(mut func: Arc<Function::Function>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<Arc<Function::Function>> {
    let mut func: Arc<Function::Function> = func;
    let mut der_func: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Pointer::Pointer<Arc<Class::NFClass>>;
    let mut tmp_cls: Pointer::Pointer<Arc<Class::NFClass>>;
    let mut new_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut wrap_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>;
    let mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments> = DifferentiationArguments::default(DifferentiationType::TIME.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Path>, Arc<Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Path>) -> Result<bool> + 'static>), 1));
    let mut diffInfo: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>;
    let mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let mut cachedData: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
    let mut diffCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut local_outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut changed: bool = false;
    func = (::match_deref::match_deref! { match &(func.clone()) {
        der_func @ Deref @ NFFunction::Function::FUNCTION { node: Deref @ InstNode::CLASS_NODE { cls, .. }, .. } => {
            let mut der_func = (*der_func).clone();
            wrap_cls = Pointer::access(cls.clone());
            new_cls = (::match_deref::match_deref! { match &(wrap_cls.clone()) {
        wrap_cls @ Deref @ Class::TYPED_DERIVED { baseClass: node @ Deref @ InstNode::CLASS_NODE { cls: tmp_cls, .. }, .. } => {
            let mut node = (*node).clone();
            new_cls = (::match_deref::match_deref! { match &(Pointer::access(tmp_cls.clone())) {
        new_cls @ Deref @ Class::INSTANCED_CLASS { sections: sections @ Deref @ Sections::SECTIONS { algorithms, .. }, .. } => {
            let mut new_cls = (*new_cls).clone();
            let mut sections = (*sections).clone();
            let mut algorithms = (*algorithms).clone();
            assign_field!(
                diffArgs.diffType = DifferentiationType::FUNCTION.clone(),
                diffArgs.funcMap = funcMap.clone()
            );
            diffInfo = (::match_deref::match_deref! { match &(der_func.interfaceDiffInfo.clone()) {
        Some(diffInfo) => UnorderedSet::copy(diffInfo.clone()),
        _ => UnorderedSet::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::nameEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 13),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            interface_map = UnorderedMap::fromLists(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut var in (der_func.inputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(var.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), List::fill(false, (der_func.inputs.clone().len() as i32)), (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
            for mut var in &*List::getAtIndexLst(der_func.inputs.clone(), der_func.derivedInputs.clone(), false) {
                let mut var = var.clone();
                UnorderedMap::remove((InstNode::name(var.clone())?).clone(), interface_map.clone())?;
                local_outputs = ({
        let mut __acc: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        for mut node in (der_func.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::setComponentDirection(Prefixes::Direction::NONE.clone(), node.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                local_outputs = ({
        let mut __acc: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        for mut node in (local_outputs.clone()).into_iter().cloned() {
            let __x = InstNode::protect(node.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                createInterfaceDerivatives(list![var.clone()], interface_map.clone(), diff_map.clone())?;
                createInterfaceDerivatives(der_func.locals.clone(), interface_map.clone(), diff_map.clone())?;
                createInterfaceDerivatives(der_func.outputs.clone(), interface_map.clone(), diff_map.clone())?;
                assign_field!(diffArgs.diff_map = Some(diff_map.clone()));
                (locals, diffArgs) = differentiateFunctionInterfaceNodes(der_func.locals.clone(), interface_map.clone(), diff_map.clone(), diffArgs.clone(), diffInfo.clone(), true)?;
                (outputs, diffArgs) = differentiateFunctionInterfaceNodes(der_func.outputs.clone(), interface_map.clone(), diff_map.clone(), diffArgs.clone(), diffInfo.clone(), false)?;
                diffCref = UnorderedMap::getSafe(ComponentRef::fromNode(var.clone(), InstNode::getType(var.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()), diff_map.clone(), metamodelica::sourceInfo!())?;
                assign_field!(
                    der_func.locals = listAppend(locals.clone(), local_outputs.clone()),
                    der_func.outputs = outputs.clone(),
                    der_func.interfaceDiffInfo = Some(diffInfo.clone())
                );
                (algorithms, diffArgs) = List::mapFold(algorithms.clone(), (std::sync::Arc::new(differentiateAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Algorithm::NFAlgorithm>, Arc<DifferentiationArguments::DifferentiationArguments>)> + 'static>), diffArgs.clone());
                algorithms = Algorithm::mapExpList(algorithms.clone(), Arc::new({ let __pe_b1 = Expression::fromCref(diffCref.clone(), false)?; let __pe_b2 = Expression::makeOne(ComponentRef::getSubscriptedType(diffCref.clone(), false)?)?; move |__pe_a0| Replacements::single(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                UnorderedMap::add((InstNode::name(var.clone())?).clone(), false, interface_map.clone())?;
            }
            assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = algorithms.clone());
            assign_variant_field!(new_cls => Class::NFClass::INSTANCED_CLASS;
                sections = sections.clone(),
                ty = var_field!((**wrap_cls).ty, Class::NFClass::TYPED_DERIVED).clone(),
                restriction = var_field!((**wrap_cls).restriction, Class::NFClass::TYPED_DERIVED).clone()
            );
            assign_variant_field!(node => InstNode::InstNode::CLASS_NODE; cls = Pointer::create(new_cls.clone()));
            assign_field!(
                der_func.derivatives = metamodelica::nil(),
                der_func.derivedInputs = metamodelica::nil(),
                der_func.interfaceDiffInfo = Some(diffInfo.clone())
            );
            cachedData = Arc::new(CachedData::CachedData::FUNCTION { funcs: list![der_func.clone()], typed: true, specialBuiltin: false });
            assign_field!(der_func.node = InstNode::newFuncCache(node.clone(), cachedData.clone())?);
            changed = true;
            new_cls.clone()
        },
        _ => wrap_cls.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            new_cls.clone()
        },
        _ => wrap_cls.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if changed.clone() {
                if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[BEFORE] ")); __mm_s.push_str(&*NFFunction::Function::toFlatString(func.clone(), BaseModelica::defaultFormat.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[AFTER ] ")); __mm_s.push_str(&*NFFunction::Function::toFlatString(der_func.clone(), BaseModelica::defaultFormat.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                }
                UnorderedMap::add(der_func.path.clone(), der_func.clone(), funcMap.clone())?;
            }
            der_func.clone()
        },
        _ => func.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub fn differentiateAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Algorithm::NFAlgorithm>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut statements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
    let mut statements_flat: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut inputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut outputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut diffInfo: Arc<UnorderedSet::UnorderedSet<Arc<Statement::NFStatement>>>;
    diffInfo = (::match_deref::match_deref! { match &(alg.stmtDiffInfo.clone()) {
        Some(diffInfo) => UnorderedSet::copy(diffInfo.clone()),
        _ => UnorderedSet::new((std::sync::Arc::new(fnptr!(Statement::hash, Arc<Statement::NFStatement>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<i32> + 'static>), (std::sync::Arc::new(Statement::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<Statement::NFStatement>) -> Result<bool> + 'static>), 13),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (statements, diffArguments) = List::mapFold(alg.statements.clone(), Arc::new({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments.clone());
    for mut stmt in &*alg.statements.clone() {
        let mut stmt = stmt.clone();
        UnorderedSet::add(stmt.clone(), diffInfo.clone())?;
    }
    statements_flat = List::flatten(statements.clone());
    (inputs, outputs) = Algorithm::getInputsOutputs(statements_flat.clone())?;
    alg = Arc::new(Algorithm::NFAlgorithm { statements: statements_flat.clone(), inputs: inputs.clone(), outputs: outputs.clone(), stmtDiffInfo: Some(diffInfo.clone()), scope: alg.scope.clone(), source: alg.source.clone() });
    Ok((alg, diffArguments))
}

pub fn differentiateStatement(mut stmt: Arc<Statement::NFStatement>, mut diffInfo: Arc<UnorderedSet::UnorderedSet<Arc<Statement::NFStatement>>>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<metamodelica::List<Arc<Statement::NFStatement>>>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut diff_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    diff_stmts = ({
        let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(stmt.clone()) {
        _ if (UnorderedSet::contains(stmt.clone(), diffInfo.clone())?) => {
            list![stmt.clone()]
        },
        diff_stmt @ Deref @ Statement::ASSIGNMENT { .. } if (Type::isReal(Type::arrayElementType(Expression::typeOf(var_field!((**diff_stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone())))) => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diff_stmt = (*diff_stmt).clone();
            (lhs, diffArguments) = differentiateExpression(var_field!((*diff_stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), diffArguments.clone())?;
            (rhs, diffArguments) = differentiateExpression(var_field!((*diff_stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), diffArguments.clone())?;
            assign_variant_field!(diff_stmt => Statement::NFStatement::ASSIGNMENT;
                lhs = lhs.clone(),
                rhs = SimplifyExp::simplifyDump(rhs.clone(), true, (literal!("NBDifferentiate.differentiateStatement")).clone(), (literal!("")).clone())?
            );
            list![diff_stmt.clone(), stmt.clone()]
        },
        diff_stmt @ Deref @ Statement::FOR { .. } => {
            let mut branch_stmts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
            let mut diff_stmt = (*diff_stmt).clone();
            (branch_stmts, diffArguments) = List::mapFold(var_field!((*diff_stmt).body, Statement::NFStatement::FOR).clone(), Arc::new({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments.clone());
            assign_variant_field!(diff_stmt => Statement::NFStatement::FOR; body = List::flatten(branch_stmts.clone()));
            list![diff_stmt.clone()]
        },
        diff_stmt @ Deref @ Statement::WHILE { .. } => {
            let mut branch_stmts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
            let mut diff_stmt = (*diff_stmt).clone();
            (branch_stmts, diffArguments) = List::mapFold(var_field!((*diff_stmt).body, Statement::NFStatement::WHILE).clone(), Arc::new({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments.clone());
            assign_variant_field!(diff_stmt => Statement::NFStatement::WHILE; body = List::flatten(branch_stmts.clone()));
            list![diff_stmt.clone()]
        },
        diff_stmt @ Deref @ Statement::FAILURE { .. } => {
            let mut branch_stmts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
            let mut diff_stmt = (*diff_stmt).clone();
            (branch_stmts, diffArguments) = List::mapFold(var_field!((*diff_stmt).body, Statement::NFStatement::FAILURE).clone(), Arc::new({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments.clone());
            assign_variant_field!(diff_stmt => Statement::NFStatement::FAILURE; body = List::flatten(branch_stmts.clone()));
            list![diff_stmt.clone()]
        },
        diff_stmt @ Deref @ Statement::IF { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut branch_stmts_flat: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut branch_stmts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
            let mut diff_stmt = (*diff_stmt).clone();
            for mut branch in &*var_field!((*diff_stmt).branches, Statement::NFStatement::IF).clone() {
                let mut branch = branch.clone();
                (exp, branch_stmts_flat) = branch.clone();
                (branch_stmts, diffArguments) = List::mapFold(branch_stmts_flat.clone(), Arc::new({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments.clone());
                branches = cons((exp.clone(), List::flatten(branch_stmts.clone())), branches.clone());
            }
            assign_variant_field!(diff_stmt => Statement::NFStatement::IF; branches = branches.clone().reverse());
            list![diff_stmt.clone()]
        },
        diff_stmt @ Deref @ Statement::WHEN { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut branch_stmts_flat: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut branch_stmts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
            let mut diff_stmt = (*diff_stmt).clone();
            for mut branch in &*var_field!((*diff_stmt).branches, Statement::NFStatement::WHEN).clone() {
                let mut branch = branch.clone();
                (exp, branch_stmts_flat) = branch.clone();
                (branch_stmts, diffArguments) = List::mapFold(branch_stmts_flat.clone(), Arc::new({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments.clone());
                branches = cons((exp.clone(), List::flatten(branch_stmts.clone())), branches.clone());
            }
            assign_variant_field!(diff_stmt => Statement::NFStatement::WHEN; branches = branches.clone().reverse());
            list![diff_stmt.clone()]
        },
        Deref @ Statement::ASSIGNMENT { .. } => {
            list![stmt.clone()]
        },
        Deref @ Statement::FUNCTION_ARRAY_INIT { .. } => {
            list![stmt.clone()]
        },
        Deref @ Statement::ASSERT { .. } => {
            list![stmt.clone()]
        },
        Deref @ Statement::TERMINATE { .. } => {
            list![stmt.clone()]
        },
        Deref @ Statement::NORETCALL { .. } => {
            list![stmt.clone()]
        },
        Deref @ Statement::RETURN { .. } => {
            list![stmt.clone()]
        },
        Deref @ Statement::BREAK { .. } => {
            list![stmt.clone()]
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateStatement")); __mm_s.push_str(&*literal!(" failed for:")); __mm_s.push_str(&*Statement::toString(stmt.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((diff_stmts, diffArguments))
}

pub fn differentiateBinary(mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("differentiateBinary: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (exp, diffArguments) = ({
        let mut isReverse: bool = Util::isSome(diffArguments.adjoint_map.clone());
        (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2, operator, exp1 } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::ADDITION.clone()) => {
            let mut diffExp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffExp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments.clone())?;
            (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments.clone())?;
            (Arc::new(Expression::NFExpression::MULTARY { arguments: list![diffExp1.clone(), diffExp2.clone()], inv_arguments: metamodelica::nil(), operator: operator.clone() }), diffArguments.clone())
        },
        Deref @ Expression::BINARY { exp2, operator, exp1 } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::SUBTRACTION.clone()) => {
            let mut diffExp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffExp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            current_grad = diffArguments.current_grad.clone();
            (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments.clone())?;
            assign_field!(diffArguments.current_grad = Expression::negate(current_grad.clone()));
            (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments.clone())?;
            assign_field!(diffArguments.current_grad = current_grad.clone());
            (_, sizeClass) = Operator::classify(operator.clone())?;
            addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), operator.ty.clone())?;
            (Arc::new(Expression::NFExpression::MULTARY { arguments: list![diffExp1.clone()], inv_arguments: list![diffExp2.clone()], operator: addOp.clone() }), diffArguments.clone())
        },
        Deref @ Expression::BINARY { exp2, operator, exp1 } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::MULTIPLICATION.clone()) => {
            let mut diffExp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffExp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut grad_exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut grad_exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut isVec1: bool = false;
            let mut isVec2: bool = false;
            let mut isMat1: bool = false;
            let mut isMat2: bool = false;
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut r1: i32 = 0;
            let mut r2: i32 = 0;
            let mut dim1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut dim2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                ty1 = Expression::typeOf(exp1.clone());
                ty2 = Expression::typeOf(exp2.clone());
                r1 = if (Type::isArray(ty1.clone())) {Type::dimensionCount(ty1.clone())} else {0};
                r2 = if (Type::isArray(ty2.clone())) {Type::dimensionCount(ty2.clone())} else {0};
                dim1 = if (r1.clone() > 0) {Dimension::sizes(Type::arrayDims(ty1.clone()), false)} else {metamodelica::nil()};
                dim2 = if (r2.clone() > 0) {Dimension::sizes(Type::arrayDims(ty2.clone()), false)} else {metamodelica::nil()};
                isVec1 = r1.clone() == 1;
                isVec2 = r2.clone() == 1;
                isMat1 = r1.clone() == 2;
                isMat2 = r2.clone() == 2;
                (_, sizeClass) = Operator::classify(operator.clone())?;
                if isVec1.clone() && isVec2.clone() && sizeClass.clone() == Operator::SizeClassification::SCALAR.clone() {
                    grad_exp1 = Arc::new(Expression::NFExpression::BINARY { exp1: current_grad.clone(), operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::SCALAR_ARRAY.clone()), operator.ty.clone())?, exp2: exp2.clone() });
                    grad_exp2 = Arc::new(Expression::NFExpression::BINARY { exp1: current_grad.clone(), operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::SCALAR_ARRAY.clone()), operator.ty.clone())?, exp2: exp1.clone() });
                } else if isMat1.clone() && isMat2.clone() && sizeClass.clone() == Operator::SizeClassification::MATRIX.clone() && (dim1.clone()).get(1)? > 1 && (dim1.clone()).get(2)? == 1 && (dim2.clone()).get(1)? == 1 && (dim2.clone()).get(2)? > 1 {
                    grad_exp1 = Arc::new(Expression::NFExpression::BINARY { exp1: current_grad.clone(), operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::MATRIX.clone()), operator.ty.clone())?, exp2: exp2.clone() });
                    grad_exp2 = Arc::new(Expression::NFExpression::BINARY { exp1: typeTransposeCall(current_grad.clone())?, operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::MATRIX.clone()), operator.ty.clone())?, exp2: exp1.clone() });
                } else if isMat1.clone() && isVec2.clone() {
                    grad_exp1 = Arc::new(Expression::NFExpression::BINARY { exp1: current_grad.clone(), operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::MATRIX.clone()), operator.ty.clone())?, exp2: typeTransposeCall(exp2.clone())? });
                    grad_exp2 = Arc::new(Expression::NFExpression::BINARY { exp1: typeTransposeCall(exp1.clone())?, operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::MATRIX_VECTOR.clone()), operator.ty.clone())?, exp2: current_grad.clone() });
                } else if isVec1.clone() && isMat2.clone() {
                    grad_exp1 = Arc::new(Expression::NFExpression::BINARY { exp1: exp2.clone(), operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::MATRIX_VECTOR.clone()), operator.ty.clone())?, exp2: typeTransposeCall(current_grad.clone())? });
                    grad_exp2 = Arc::new(Expression::NFExpression::BINARY { exp1: typeTransposeCall(exp1.clone())?, operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::MATRIX.clone()), operator.ty.clone())?, exp2: current_grad.clone() });
                } else if isMat1.clone() && isMat2.clone() {
                    grad_exp1 = Arc::new(Expression::NFExpression::BINARY { exp1: current_grad.clone(), operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::MATRIX.clone()), operator.ty.clone())?, exp2: typeTransposeCall(exp2.clone())? });
                    grad_exp2 = Arc::new(Expression::NFExpression::BINARY { exp1: typeTransposeCall(exp1.clone())?, operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::MATRIX.clone()), operator.ty.clone())?, exp2: current_grad.clone() });
                } else {
                    grad_exp1 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![current_grad.clone(), exp2.clone()], inv_arguments: metamodelica::nil(), operator: makeMulFromOperator(operator.clone())? });
                    grad_exp2 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![current_grad.clone(), exp1.clone()], inv_arguments: metamodelica::nil(), operator: makeMulFromOperator(operator.clone())? });
                }
                assign_field!(diffArguments.current_grad = grad_exp1.clone());
                (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = grad_exp2.clone());
                (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments.clone())?;
                assign_field!(diffArguments.current_grad = current_grad.clone());
            } else {
                (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments.clone())?;
                (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments.clone())?;
            }
            sizeClass = Operator::classifyAddition(operator.clone());
            addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), operator.ty.clone())?;
            (Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::BINARY { exp1: diffExp1.clone(), operator: operator.clone(), exp2: exp2.clone() }), Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: operator.clone(), exp2: diffExp2.clone() })], inv_arguments: metamodelica::nil(), operator: addOp.clone() }), diffArguments.clone())
        },
        Deref @ Expression::BINARY { exp2, operator, exp1 } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::DIVISION.clone()) => {
            let mut diffExp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffExp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut mulOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut powOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut divOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut powSizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut denom2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut numUF: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            powSizeClass = Operator::SizeClassification::SCALAR.clone();
            powOp = Operator::fromClassification((Operator::MathClassification::POWER.clone(), powSizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
            if isReverse.clone() {
                current_grad = diffArguments.current_grad.clone();
                assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::MULTARY { arguments: list![current_grad.clone()], inv_arguments: list![exp2.clone()], operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), if (Type::isArray(Expression::typeOf(current_grad.clone()))) {Operator::SizeClassification::ARRAY_SCALAR.clone()} else {Operator::SizeClassification::SCALAR.clone()}), operator.ty.clone())? }));
            }
            (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments.clone())?;
            if isReverse.clone() {
                denom2 = Arc::new(Expression::NFExpression::BINARY { exp1: exp2.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
                numUF = Arc::new(Expression::NFExpression::BINARY { exp1: current_grad.clone(), operator: if (Type::isArray(Expression::typeOf(exp1.clone()))) {Operator::makeScalarProduct(operator.ty.clone())} else {Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::SCALAR.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?}, exp2: exp1.clone() });
                divOp = Operator::fromClassification((Operator::MathClassification::DIVISION.clone(), Operator::SizeClassification::SCALAR.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
                assign_field!(diffArguments.current_grad = Expression::negate(Arc::new(Expression::NFExpression::BINARY { exp1: numUF.clone(), operator: divOp.clone(), exp2: denom2.clone() })));
            }
            (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments.clone())?;
            if isReverse.clone() {
                assign_field!(diffArguments.current_grad = current_grad.clone());
            }
            (_, sizeClass) = Operator::classify(operator.clone())?;
            addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), operator.ty.clone())?;
            mulOp = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sizeClass.clone()), operator.ty.clone())?;
            (Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::BINARY { exp1: diffExp1.clone(), operator: mulOp.clone(), exp2: exp2.clone() })], inv_arguments: list![Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: mulOp.clone(), exp2: diffExp2.clone() })], operator: addOp.clone() })], inv_arguments: list![Arc::new(Expression::NFExpression::BINARY { exp1: exp2.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) })], operator: mulOp.clone() }), diffArguments.clone())
        },
        Deref @ Expression::BINARY { operator, exp1, .. } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::POWER.clone() && Expression::isZero(exp1.clone())) => {
            (Expression::makeZero(operator.ty.clone())?, diffArguments.clone())
        },
        Deref @ Expression::BINARY { exp2, operator, exp1 } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::POWER.clone()) => {
            let mut diffExp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diffExp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut mulOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (_, sizeClass) = Operator::classify(operator.clone())?;
            addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), operator.ty.clone())?;
            current_grad = diffArguments.current_grad.clone();
            assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::MULTARY { arguments: list![current_grad.clone(), exp2.clone(), Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: operator.clone(), exp2: minusOne(exp2.clone(), addOp.clone())? })], inv_arguments: metamodelica::nil(), operator: makeMulFromOperator(operator.clone())? }));
            (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments.clone())?;
            assign_field!(diffArguments.current_grad = Arc::new(Expression::NFExpression::MULTARY { arguments: list![current_grad.clone(), exp.clone(), expLog(exp1.clone())?], inv_arguments: metamodelica::nil(), operator: makeMulFromOperator(operator.clone())? }));
            (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments.clone())?;
            assign_field!(diffArguments.current_grad = current_grad.clone());
            diffExp1 = SimplifyExp::simplifyDump(diffExp1.clone(), true, (literal!("NBDifferentiate.differentiateBinary")).clone(), (literal!("")).clone())?;
            diffExp2 = SimplifyExp::simplifyDump(diffExp2.clone(), true, (literal!("NBDifferentiate.differentiateBinary")).clone(), (literal!("")).clone())?;
            mulOp = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sizeClass.clone()), operator.ty.clone())?;
            res = (match (Expression::isZero(diffExp1.clone()), Expression::isZero(diffExp2.clone())) {
        (true, true) => Expression::makeZero(operator.ty.clone())?,
        (false, true) => Arc::new(Expression::NFExpression::MULTARY { arguments: list![exp2.clone(), Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: operator.clone(), exp2: minusOne(exp2.clone(), addOp.clone())? }), diffExp1.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() }),
        (true, false) => Arc::new(Expression::NFExpression::MULTARY { arguments: list![exp.clone(), expLog(exp1.clone())?, diffExp2.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() }),
        _ => {
            e1 = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: operator.clone(), exp2: minusOne(exp2.clone(), addOp.clone())? });
            e2 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![exp1.clone(), expLog(exp1.clone())?, diffExp2.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() });
            e3 = Arc::new(Expression::NFExpression::MULTARY { arguments: list![exp2.clone(), diffExp1.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() });
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![e1.clone(), Arc::new(Expression::NFExpression::MULTARY { arguments: list![e2.clone(), e3.clone()], inv_arguments: metamodelica::nil(), operator: addOp.clone() })], inv_arguments: metamodelica::nil(), operator: mulOp.clone() })
        },
    });
            (res.clone(), diffArguments.clone())
        },
        Deref @ Expression::BINARY { operator, .. } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::LOGICAL.clone() || Operator::getMathClassification(operator.clone())? == Operator::MathClassification::RELATION.clone()) => {
            (exp.clone(), diffArguments.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBinary")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((exp, diffArguments))
}

pub fn differentiateMultary(mut exp: Arc<Expression::NFExpression>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Expression::NFExpression>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut isReverse: bool = Util::isSome(diffArguments.adjoint_map.clone());
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("differentiateMultary: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    exp = ({
        let mut new_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut new_inv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::MULTARY { operator, inv_arguments, arguments } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::ADDITION.clone()) => {
            let mut diff_arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut hasArray: bool = false;
            let mut local_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if isReverse.clone() {
                hasArray = List::any(arguments.clone(), (std::sync::Arc::new(fnptr!(Expression::hasArrayType, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>)) || List::any(inv_arguments.clone(), (std::sync::Arc::new(fnptr!(Expression::hasArrayType, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>));
            }
            for mut arg in &*arguments.clone().reverse() {
                let mut arg = arg.clone();
                if isReverse.clone() {
                    current_grad = diffArguments.current_grad.clone();
                    if Expression::isScalar(arg.clone()) && hasArray.clone() {
                        assign_field!(diffArguments.current_grad = typeSumCall(current_grad.clone()));
                    } else {
                        assign_field!(diffArguments.current_grad = current_grad.clone());
                    }
                }
                (diff_arg, diffArguments) = differentiateExpression(arg.clone(), diffArguments.clone())?;
                if isReverse.clone() {
                    assign_field!(diffArguments.current_grad = current_grad.clone());
                } else {
                    new_arguments = cons(diff_arg.clone(), new_arguments.clone());
                }
            }
            for mut arg in &*inv_arguments.clone().reverse() {
                let mut arg = arg.clone();
                if isReverse.clone() {
                    current_grad = diffArguments.current_grad.clone();
                    local_grad = Expression::negate(current_grad.clone());
                    if Expression::isScalar(arg.clone()) && hasArray.clone() {
                        local_grad = typeSumCall(local_grad.clone());
                    }
                    assign_field!(diffArguments.current_grad = local_grad.clone());
                }
                (diff_arg, diffArguments) = differentiateExpression(arg.clone(), diffArguments.clone())?;
                if isReverse.clone() {
                    assign_field!(diffArguments.current_grad = current_grad.clone());
                } else {
                    new_inv_arguments = cons(diff_arg.clone(), new_inv_arguments.clone());
                }
            }
            Arc::new(Expression::NFExpression::MULTARY { arguments: new_arguments.clone(), inv_arguments: new_inv_arguments.clone(), operator: operator.clone() })
        },
        Deref @ Expression::MULTARY { operator, inv_arguments: Deref @ metamodelica::List::Nil, arguments } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::MULTIPLICATION.clone()) => {
            let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            sizeClass = Operator::classifyAddition(operator.clone());
            addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), operator.ty.clone())?;
            (new_arguments, diffArguments) = differentiateMultaryMultiplicationArgs(arguments.clone(), diffArguments.clone(), operator.clone())?;
            Arc::new(Expression::NFExpression::MULTARY { arguments: new_arguments.clone(), inv_arguments: metamodelica::nil(), operator: addOp.clone() })
        },
        Deref @ Expression::MULTARY { operator, inv_arguments, arguments } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::MULTIPLICATION.clone() && !(inv_arguments.clone().is_empty()) && isReverse.clone()) => {
            let mut diff_arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut mulEWOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut powSizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut upstream: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e_over_f: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e_over_g: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut numProd: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut denomProd: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg_rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut hasArrayNum: bool = false;
            let mut localUpF: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut localUpG: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut i: i32 = 0;
            (_, sizeClass) = Operator::classify(operator.clone())?;
            Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), operator.ty.clone())?;
            makeMulFromOperator(operator.clone())?;
            mulEWOp = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), operator.ty.clone())?;
            Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), operator.ty.clone())?;
            hasArrayNum = List::any(arguments.clone(), (std::sync::Arc::new(fnptr!(Expression::hasArrayType, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>));
            numProd = Arc::new(Expression::NFExpression::MULTARY { arguments: arguments.clone(), inv_arguments: metamodelica::nil(), operator: operator.clone() });
            denomProd = Arc::new(Expression::NFExpression::MULTARY { arguments: inv_arguments.clone(), inv_arguments: metamodelica::nil(), operator: operator.clone() });
            upstream = diffArguments.current_grad.clone();
            i = 1;
            for mut f in &*arguments.clone() {
                let mut f = f.clone();
                arg_rest = listDelete(arguments.clone(), i.clone())?;
                e_over_f = Arc::new(Expression::NFExpression::MULTARY { arguments: arg_rest.clone(), inv_arguments: list![denomProd.clone()], operator: operator.clone() });
                localUpF = Arc::new(Expression::NFExpression::MULTARY { arguments: list![upstream.clone(), e_over_f.clone()], inv_arguments: metamodelica::nil(), operator: mulEWOp.clone() });
                if Expression::isScalar(f.clone()) && hasArrayNum.clone() {
                    localUpF = typeSumCall(localUpF.clone());
                }
                assign_field!(diffArguments.current_grad = localUpF.clone());
                (diff_arg, diffArguments) = differentiateExpression(f.clone(), diffArguments.clone())?;
                i = i.clone() + 1;
            }
            i = 1;
            powSizeClass = if (Expression::hasArrayType(listHead(inv_arguments.clone())?)) {Operator::SizeClassification::ARRAY_SCALAR.clone()} else {Operator::SizeClassification::SCALAR.clone()};
            Operator::fromClassification((Operator::MathClassification::POWER.clone(), powSizeClass.clone()), Arc::new(openmodelica_nf_frontend::NFType::REAL))?;
            for mut g in &*inv_arguments.clone() {
                let mut g = g.clone();
                listDelete(inv_arguments.clone(), i.clone())?;
                e_over_g = Arc::new(Expression::NFExpression::MULTARY { arguments: list![numProd.clone()], inv_arguments: cons(g.clone(), inv_arguments.clone()), operator: operator.clone() });
                localUpG = Expression::negate(Arc::new(Expression::NFExpression::MULTARY { arguments: list![upstream.clone(), e_over_g.clone()], inv_arguments: metamodelica::nil(), operator: mulEWOp.clone() }));
                if hasArrayNum.clone() {
                    localUpG = typeSumCall(localUpG.clone());
                }
                assign_field!(diffArguments.current_grad = localUpG.clone());
                (diff_arg, diffArguments) = differentiateExpression(g.clone(), diffArguments.clone())?;
                Expression::negate(Arc::new(Expression::NFExpression::MULTARY { arguments: list![diff_arg.clone(), e_over_g.clone()], inv_arguments: metamodelica::nil(), operator: mulEWOp.clone() }));
                i = i.clone() + 1;
            }
            assign_field!(diffArguments.current_grad = upstream.clone());
            Arc::new(openmodelica_nf_frontend::NFExpression::END)
        },
        Deref @ Expression::MULTARY { operator, inv_arguments, arguments } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::MULTIPLICATION.clone() && !(inv_arguments.clone().is_empty())) => {
            let mut divisor: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diff_enumerator: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diff_divisor: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut diff_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut diff_inv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut powOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut powSizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            let mut powTy: Arc<Type::NFType> = Arc::new(Type::ANY);
            if !(inv_arguments.clone().is_empty()) && Type::isArray(Expression::typeOf(listHead(inv_arguments.clone())?)) {
                powSizeClass = Operator::SizeClassification::ARRAY_SCALAR.clone();
                powTy = operator.ty.clone();
            } else {
                powSizeClass = Operator::SizeClassification::SCALAR.clone();
                powTy = Arc::new(openmodelica_nf_frontend::NFType::REAL);
            }
            if !(arguments.clone().is_empty()) && Type::isArray(Expression::typeOf(listHead(arguments.clone())?)) {
                sizeClass = Operator::SizeClassification::ELEMENT_WISE.clone();
            } else {
                (_, sizeClass) = Operator::classify(operator.clone())?;
            }
            addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), operator.ty.clone())?;
            powOp = Operator::fromClassification((Operator::MathClassification::POWER.clone(), powSizeClass.clone()), powTy.clone())?;
            (diff_arguments, diffArguments) = differentiateMultaryMultiplicationArgs(arguments.clone(), diffArguments.clone(), operator.clone())?;
            diff_enumerator = Arc::new(Expression::NFExpression::MULTARY { arguments: diff_arguments.clone(), inv_arguments: metamodelica::nil(), operator: addOp.clone() });
            (diff_inv_arguments, diffArguments) = differentiateMultaryMultiplicationArgs(inv_arguments.clone(), diffArguments.clone(), operator.clone())?;
            diff_divisor = Arc::new(Expression::NFExpression::MULTARY { arguments: diff_inv_arguments.clone(), inv_arguments: metamodelica::nil(), operator: addOp.clone() });
            divisor = Arc::new(Expression::NFExpression::MULTARY { arguments: inv_arguments.clone(), inv_arguments: metamodelica::nil(), operator: operator.clone() });
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::MULTARY { arguments: cons(diff_enumerator.clone(), inv_arguments.clone()), inv_arguments: metamodelica::nil(), operator: operator.clone() })], inv_arguments: list![Arc::new(Expression::NFExpression::MULTARY { arguments: cons(diff_divisor.clone(), arguments.clone()), inv_arguments: metamodelica::nil(), operator: operator.clone() })], operator: addOp.clone() })], inv_arguments: list![Arc::new(Expression::NFExpression::BINARY { exp1: divisor.clone(), operator: powOp.clone(), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) })], operator: operator.clone() })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateMultary")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((exp, diffArguments))
}

pub fn differentiateMultaryMultiplicationArgs(mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>, mut operator: Arc<Operator::NFOperator>) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut new_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diff_arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut current_grad: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut localUp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut restProd: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut diff_lists: metamodelica::Array<Arc<metamodelica::List<Arc<Expression::NFExpression>>>>;
    let mut arg_products: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut restArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut idx: i32 = 1;
    let mut isReverse: bool = Util::isSome(diffArguments.adjoint_map.clone());
    let mut mulEWOp: Arc<Operator::NFOperator> = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), operator.ty.clone())?;
    if isReverse.clone() {
        arg_products = Expression::productOfListExceptSelf(arguments.clone(), makeMulFromOperator(operator.clone())?)?;
    } else {
        diff_lists = arrayCreate((arguments.clone().len() as i32), metamodelica::nil());
    }
    for mut arg in &*arguments.clone() {
        let mut arg = arg.clone();
        if isReverse.clone() {
            current_grad = diffArguments.current_grad.clone();
            restProd = (arg_products.clone()).get(idx.clone())?;
            restArgs = (::match_deref::match_deref! { match &(restProd.clone()) {
        Deref @ Expression::MULTARY { arguments: rA, operator: mOp, .. } if (Operator::getMathClassification(mOp.clone())? == Operator::MathClassification::MULTIPLICATION.clone()) => {
            rA.clone()
        },
        _ => {
            list![restProd.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            localUp = Arc::new(Expression::NFExpression::MULTARY { arguments: listAppend(list![current_grad.clone()], restArgs.clone()), inv_arguments: metamodelica::nil(), operator: mulEWOp.clone() });
            if Expression::isScalar(arg.clone()) && Expression::hasArrayType(restProd.clone()) {
                localUp = typeSumCall(localUp.clone());
            }
            assign_field!(diffArguments.current_grad = localUp.clone());
        }
        (diff_arg, diffArguments) = differentiateExpression(arg.clone(), diffArguments.clone())?;
        if isReverse.clone() {
            assign_field!(diffArguments.current_grad = current_grad.clone());
        } else {
            let __range0 = 1..=(diff_lists.clone().borrow().len() as i32);
            for mut i in __range0 {
                {
                    let __cell1 = if (i.clone() == idx.clone()) {cons(diff_arg.clone(), diff_lists.borrow()[(i.clone()-1) as usize].clone())} else {cons(arg.clone(), diff_lists.borrow()[(i.clone()-1) as usize].clone())};
                    diff_lists.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                }
            }
        }
        idx = idx.clone() + 1;
    }
    if !(isReverse.clone()) {
        let __range2 = (1..=(diff_lists.clone().borrow().len() as i32)).rev();
        for mut i in __range2 {
            new_arguments = cons(Arc::new(Expression::NFExpression::MULTARY { arguments: diff_lists.borrow()[(i.clone()-1) as usize].clone().reverse(), inv_arguments: metamodelica::nil(), operator: operator.clone() }), new_arguments.clone());
        }
    }
    Ok((new_arguments, diffArguments))
}

pub fn differentiateEquationAttributes(mut attr: Arc<EquationAttributes::EquationAttributes>, mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<Arc<EquationAttributes::EquationAttributes>> {
    let mut attr: Arc<EquationAttributes::EquationAttributes> = attr;
    attr = (::match_deref::match_deref! { match &((attr.clone(), diffArguments.clone())) {
        (Deref @ EquationAttributes::EQUATION_ATTRIBUTES { residualVar: Some(residualVar), .. }, Deref @ DifferentiationArguments::DIFFERENTIATION_ARGUMENTS { diffType: DifferentiationType::JACOBIAN, diff_map: Some(diff_map), .. }) if (UnorderedMap::contains(BVariable::getVarName(residualVar.clone()), diff_map.clone())) => {
            let mut diffedResidualVar: Pointer::Pointer<Arc<Variable::NFVariable>>;
            diffedResidualVar = BVariable::getVarPointer(UnorderedMap::getOrFail(BVariable::getVarName(residualVar.clone()), diff_map.clone()), metamodelica::sourceInfo!())?;
            assign_field!(attr.residualVar = Some(diffedResidualVar.clone()));
            attr.clone()
        },
        _ => {
            attr.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attr)
}

pub fn differentiateBinding(mut binding: Arc<Binding::NFBinding>, mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments>) -> Result<(Arc<Binding::NFBinding>, Arc<DifferentiationArguments::DifferentiationArguments>)> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let mut diffArgs: Arc<DifferentiationArguments::DifferentiationArguments> = diffArgs;
    let mut opt_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    opt_exp = Binding::getExpOpt(binding.clone());
    if Util::isSome(opt_exp.clone()) {
        (exp, diffArgs) = differentiateExpression(Util::getOption(opt_exp.clone())?, diffArgs.clone())?;
        binding = Binding::setExp(exp.clone(), binding.clone())?;
    }
    Ok((binding, diffArgs))
}

fn minusOne(mut exp: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { value: r } => {
            Arc::new(Expression::NFExpression::REAL { value: r.clone() - metamodelica::OrderedFloat(1.0_f64) })
        },
        Deref @ Expression::INTEGER { value: i } => {
            Arc::new(Expression::NFExpression::INTEGER { value: i.clone() - 1 })
        },
        _ => {
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![exp.clone()], inv_arguments: list![Expression::makeOne(op.ty.clone())?], operator: op.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn expLog(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { value: r } => {
            Arc::new(Expression::NFExpression::REAL { value: (r.clone()).ln() })
        },
        Deref @ Expression::INTEGER { value: i } => {
            Arc::new(Expression::NFExpression::REAL { value: (i.clone()).ln() })
        },
        _ => {
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::LOG_REAL().clone(), list![exp.clone()], Expression::variability(exp.clone())?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::LOG_REAL().returnType.clone()) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn makeMulFromOperator(mut operator: Arc<Operator::NFOperator>) -> Result<Arc<Operator::NFOperator>> {
    let mut mulOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    mulOp = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::getSizeClassification(operator.clone())?), operator.ty.clone())?;
    Ok(mulOp)
}

fn typeTransposeCall(mut mat: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut tr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut inTy: Arc<Type::NFType> = Expression::typeOf(mat.clone());
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut resTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut call: Arc<Call::NFCall>;
    let mut var: Variability = Expression::variability(mat.clone())?;
    let mut pur: Prefixes::Purity = Expression::purity(mat.clone())?;
    if !(Type::isArray(inTy.clone())) {
        tr = mat.clone();
        return Ok(tr.clone());
    }
    elTy = Type::arrayElementType(inTy.clone());
    dims = Type::arrayDims(inTy.clone());
    if (dims.clone().len() as i32) < 2 {
        tr = mat.clone();
        return Ok(tr.clone());
    }
    resTy = Arc::new(Type::NFType::ARRAY { elementType: elTy.clone(), dimensions: listAppend(list![(dims.clone()).get(2)?, (dims.clone()).get(1)?], listRest(listRest(dims.clone())?)?) });
    call = Call::makeTypedCall(BuiltinFuncs::TRANSPOSE().clone(), list![mat.clone()], var.clone(), pur.clone(), resTy.clone());
    tr = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    Ok(tr)
}

// Helper: build a typed builtin promote(A, n) call that appends (n - ndims(A)) singleton dims.
fn typePromoteCall(mut arr: Arc<Expression::NFExpression>, mut n: i32) -> Arc<Expression::NFExpression> {
    let mut promoted: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut inTy: Arc<Type::NFType> = Expression::typeOf(arr.clone());
    let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut inDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut m: i32 = 0;
    let mut k: i32 = 0;
    let mut ones: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut resDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut resTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut call: Arc<Call::NFCall>;
    let mut var: Variability = Expression::variability(arr.clone()).unwrap();
    let mut pur: Prefixes::Purity = Expression::purity(arr.clone()).unwrap();
    elTy = if (Type::isArray(inTy.clone())) {Type::arrayElementType(inTy.clone())} else {inTy.clone()};
    inDims = if (Type::isArray(inTy.clone())) {Type::arrayDims(inTy.clone())} else {metamodelica::nil()};
    m = (inDims.clone().len() as i32);
    for mut k in 1..=std::cmp::max(0, n.clone() - m.clone()) {
        ones = cons(Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone()), ones.clone());
    }
    resDims = List::append_reverse(ones.clone(), inDims.clone());
    resTy = if (n.clone() > 0) {Arc::new(Type::NFType::ARRAY { elementType: elTy.clone(), dimensions: resDims.clone() })} else {elTy.clone()};
    call = Call::makeTypedCall(BuiltinFuncs::PROMOTE().clone(), list![arr.clone(), Arc::new(Expression::NFExpression::INTEGER { value: n.clone() })], var.clone(), pur.clone(), resTy.clone());
    promoted = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    promoted
}

fn typeSumCall(mut arr: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut s: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut inTy: Arc<Type::NFType> = Expression::typeOf(arr.clone());
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut resTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut call: Arc<Call::NFCall>;
    let mut var: Variability = Expression::variability(arr.clone()).unwrap();
    let mut pur: Prefixes::Purity = Expression::purity(arr.clone()).unwrap();
    if !(Type::isArray(inTy.clone())) {
        s = arr.clone();
        return s.clone();
    }
    elTy = Type::arrayElementType(inTy.clone());
    dims = Type::arrayDims(inTy.clone());
    resTy = elTy.clone();
    call = Call::makeTypedCall(BuiltinFuncs::SUM().clone(), list![arr.clone()], var.clone(), pur.clone(), resTy.clone());
    s = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    s
}

// Helper: build matrix * vector (or matrix * matrix) MULTARY with a proper mul operator
fn makeMul(mut a: Arc<Expression::NFExpression>, mut b: Arc<Expression::NFExpression>, mut sc: Operator::SizeClassification, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    res = Arc::new(Expression::NFExpression::BINARY { exp1: a.clone(), operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sc.clone()), ty.clone())?, exp2: b.clone() });
    Ok(res)
}

// Drop the last array dimension by indexing it with 1:
// arr[..., 1]. If arr is not an array, return it unchanged.
fn dropLastDimIndex1(mut arr: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Expression::typeOf(arr.clone());
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut m: i32 = 0;
    let mut i: i32 = 0;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    if !(Type::isArray(ty.clone())) {
        res = arr.clone();
        return Ok(res.clone());
    }
    dims = Type::arrayDims(ty.clone());
    m = (dims.clone().len() as i32);
    if m.clone() <= 0 {
        res = arr.clone();
        return Ok(res.clone());
    }
    for mut i in 1..=m.clone() - 1 {
        subs = cons(Arc::new(openmodelica_nf_frontend::NFSubscript::WHOLE), subs.clone());
    }
    subs = cons(Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) }), subs.clone());
    subs = subs.clone().reverse();
    res = Expression::applySubscripts(subs.clone(), arr.clone(), true)?;
    Ok(res)
}

// Build vector[n] with elements A[i,i], i=1..n (literal array).
fn extractDiagonalVector(mut A: Arc<Expression::NFExpression>, mut n: i32, mut vecTy: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut v: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut i: i32 = 0;
    for mut i in 1..=n.clone() {
        elems = cons(Expression::applySubscripts(list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) }), Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], A.clone(), true)?, elems.clone());
    }
    v = Arc::new(Expression::NFExpression::ARRAY { ty: vecTy.clone(), elements: metamodelica::arrayFromVec(elems.clone().reverse().into_iter().cloned().collect()), literal: false });
    Ok(v)
}

fn dbg(mut s: ArcStr) -> Result<()> {
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn updateAdjointList(mut oldOpt: Option<Arc<metamodelica::List<Arc<Expression::NFExpression>>>>, mut current_grad: Arc<Expression::NFExpression>) -> Arc<metamodelica::List<Arc<Expression::NFExpression>>> {
    let mut newList: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut oldList: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    newList = (::match_deref::match_deref! { match &(oldOpt.clone()) {
        Some(oldList) => cons(current_grad.clone(), oldList.clone()),
        _ => list![current_grad.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    newList
}

// Build a 1D one-hot array of the same type as derBaseCref:
// zeros(n) with value placed at index idx.
fn buildOneHotVectorAdjoint(mut derBaseCref: Arc<ComponentRef::NFComponentRef>, mut idx: i32, mut value: Arc<Expression::NFExpression>) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut onehot: Option<Arc<Expression::NFExpression>> = None;
    let mut arrTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    arrTy = ComponentRef::getSubscriptedType(derBaseCref.clone(), false)?;
    if !(Type::isArray(arrTy.clone())) {
        onehot = None;
        return Ok(onehot.clone());
    }
    dims = Type::arrayDims(arrTy.clone());
    if !(List::hasOneElement(dims.clone())) {
        onehot = None;
        return Ok(onehot.clone());
    }
    sizes = Dimension::sizes(dims.clone(), false);
    if sizes.clone().is_empty() {
        onehot = None;
        return Ok(onehot.clone());
    }
    n = listHead(sizes.clone())?;
    elTy = Type::arrayElementType(arrTy.clone());
    for mut i in 1..=n.clone() {
        elems = cons(if (i.clone() == idx.clone()) {value.clone()} else {Expression::makeZero(elTy.clone())?}, elems.clone());
    }
    onehot = Some(Arc::new(Expression::NFExpression::ARRAY { ty: arrTy.clone(), elements: metamodelica::arrayFromVec(elems.clone().reverse().into_iter().cloned().collect()), literal: false }));
    Ok(onehot)
}

// Build a multi-hot scatter vector for a SLICE subscript:
// result = sum_t [onehot(idx_t) * seed_elem_t]
// Handles:
//   - WHOLE()                     -> returns seed
//   - SLICE {i1,i2,...}           -> sum of one-hots; indices must be literal integers
//   - SLICE range lo[:st]:hi      -> sum over lo, lo+st, ..., hi; lo,st,hi must be literal integers
fn buildMultiHotVectorAdjoint(mut derBaseCref: Arc<ComponentRef::NFComponentRef>, mut sub: Arc<Subscript::NFSubscript>, mut seed: Arc<Expression::NFExpression>) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut scatter: Option<Arc<Expression::NFExpression>> = None;
    let mut arrTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    let mut seedIsArray: bool = false;
    let mut m: i32 = 0;
    let mut j: i32 = 0;
    let mut loI: i32 = 0;
    let mut hiI: i32 = 0;
    let mut accOpt: Option<Arc<Expression::NFExpression>> = None;
    let mut ohOpt: Option<Arc<Expression::NFExpression>> = None;
    let mut acc: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut term: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    arrTy = ComponentRef::getSubscriptedType(derBaseCref.clone(), false)?;
    elTy = Type::arrayElementType(arrTy.clone());
    addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), elTy.clone())?;
    seedIsArray = Type::isArray(Expression::typeOf(seed.clone()));
    scatter = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::SLICE { slice: Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: hiI }, step: None, start: Deref @ Expression::INTEGER { value: loI }, .. } } => {
            if hiI.clone() < loI.clone() {
                scatter = Some(Expression::makeZero(arrTy.clone())?);
                return Ok(scatter.clone());
            }
            accOpt = None;
            m = hiI.clone() - loI.clone() + 1;
            for mut j in 0..=m.clone() - 1 {
                ohOpt = buildOneHotVectorAdjoint(derBaseCref.clone(), loI.clone() + j.clone(), if (seedIsArray.clone()) {Expression::applySubscripts(list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: j.clone() + 1 }) })], seed.clone(), true)?} else {seed.clone()})?;
                if Util::isSome(ohOpt.clone()) {
                    if Util::isSome(accOpt.clone()) {
                        acc = Util::getOption(accOpt.clone())?;
                        term = Util::getOption(ohOpt.clone())?;
                        accOpt = Some(Arc::new(Expression::NFExpression::MULTARY { arguments: list![acc.clone(), term.clone()], inv_arguments: metamodelica::nil(), operator: addOp.clone() }));
                    } else {
                        accOpt = ohOpt.clone();
                    }
                } else {
                    scatter = None;
                    return Ok(scatter.clone());
                }
            }
            scatter = if (Util::isSome(accOpt.clone())) {accOpt.clone()} else {Some(Expression::makeZero(arrTy.clone())?)};
            scatter.clone()
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(scatter)
}

