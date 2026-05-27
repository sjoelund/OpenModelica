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
use crate::NBCausalize as Causalize;
use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBDifferentiate::DifferentiationType;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBReplacements as Replacements;
use crate::NBSolve as Solve;
use crate::NBSolve::Status;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBackendExtension as BackendExtension;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::TearingSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFExpressionIterator as ExpressionIterator;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
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

// ToDo:
// 1. simple state rules (with derivative replacement)
//    - state = state
//    - state = alg
//    - state = time
//    - state = const
// 2. write rateVar() and decide if we want an auxiliary for each set
//    - rateVar() --> mergeAttributes()
// 3. post causalize alias elimination
//    - for the ODE
//    - for jacobians/hessians (once we got hessians)
//    - for strong components in general
// 4. simplify only replaced equations and remove simplify2 module
//    - probably not that trivial
//    - Equation mapExp function that returns true if something was replaced
//    - EquationArray map function that accumulates pointers if function returns true
//    - simplify all equations in pointer list
// 5. trivial solution a = b; a = -b; (or other cyclic sets)
//    - take an equation from the set, get both crefs in it (a,b)
//    - solve for a -> set a as known
//    - solve the rest of the set with causalize
//    - replacements a -> what it solves for in eq1 and apply on all eq in set
//    - find equation that solves b, and solve for b. add to replacements
//    - apply replacements on all eq
// OF imports
// NF imports
// Backend imports
// Util imports
// ==========================================================================
//               Single Variable constants and functions
// ==========================================================================
pub const NOMINAL_THRESHOLD: metamodelica::Real = metamodelica::OrderedFloat(1000.0_f64);

pub fn getModule() -> Result<Module::aliasInterface> {
    let mut func: Module::aliasInterface;
    let mut flag: ArcStr = literal!("default");
    func = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "default" => aliasDefault.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub mod AliasSet {
    use super::*;
    /// gets accumulated to find sets of alias equations and solve them
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AliasSet {
        /// list of all variables in this set
        pub simple_variables: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
        /// list of all equations in this set
        pub simple_equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>,
        /// optional constant binding of one variable
        pub const_opt: Option<Pointer::Pointer<Arc<Equation::Equation>>>,
    }

    impl Default for AliasSet {
        fn default() -> Self {
            Self {
                simple_variables: Default::default(),
                simple_equations: Default::default(),
                const_opt: Default::default(),
            }
        }
    }

    pub type ALIAS_SET = AliasSet;

    pub fn toString(mut set: Arc<AliasSet>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        if isSome(set.const_opt.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\tConstant/Parameter Binding: ")); __mm_s.push_str(&*BEquation::Equation::toString(Pointer::access(Util::getOption(set.const_opt.clone())?), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = (literal!("\t<No Constant/Parameter Binding>\n")).clone();
        }
        if set.simple_equations.clone().is_empty() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\t###<No Set Equations>\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\t### Set Equations:\n")); ArcStr::from(__mm_s) }).clone();
            for mut eq in &*set.simple_equations.clone() {
                let mut eq = eq.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*BEquation::Equation::toString(Pointer::access(eq.clone()), (literal!("\t")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
        }
        Ok(r#str)
    }

}

thread_local! { static __EMPTY_ALIAS_SET_TLS: Arc<AliasSet::AliasSet> = Arc::new(AliasSet::AliasSet { simple_variables: metamodelica::nil(), simple_equations: metamodelica::nil(), const_opt: None }); }
pub fn EMPTY_ALIAS_SET() -> Arc<AliasSet::AliasSet> { __EMPTY_ALIAS_SET_TLS.with(|__t| __t.clone()) }

// needed for unordered map
pub type SetPtr = Pointer::Pointer<Arc<AliasSet::AliasSet>>;

/// used for findCrefs()
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrefTpl {
    /// false if search already resulted in non simple structure
    pub cont: bool,
    /// variable count
    pub varCount: i32,
    /// parameter/constant count
    pub paramCount: i32,
    /// list of found variables for replacement
    pub cr_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
}

pub type CREF_TPL = CrefTpl;


thread_local! { static __EMPTY_CREF_TPL_TLS: CrefTpl = CrefTpl { cont: true, varCount: 0, paramCount: 0, cr_lst: metamodelica::nil() }; }
pub fn EMPTY_CREF_TPL() -> CrefTpl { __EMPTY_CREF_TPL_TLS.with(|__t| __t.clone()) }

thread_local! { static __FAILED_CREF_TPL_TLS: CrefTpl = CrefTpl { cont: false, varCount: 0, paramCount: 0, cr_lst: metamodelica::nil() }; }
pub fn FAILED_CREF_TPL() -> CrefTpl { __FAILED_CREF_TPL_TLS.with(|__t| __t.clone()) }

fn checkReplacements(mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut eqData: Arc<EqData::EqData>) -> Result<(Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> {
    let mut newReplacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut auxEquations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut exceptionMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, ExceptionKind>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eqPtr: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut attr: Arc<EquationAttributes::EquationAttributes>;
    BEquation::EqData::map(eqData.clone(), Arc::new({ let __pe_b1 = exceptionMap.clone(); move |__pe_a0| filterExceptionsEquation(__pe_a0, __pe_b1.clone()) }))?;
    for mut keyValueTpl in &*UnorderedMap::toList(replacements.clone()) {
        let mut keyValueTpl = keyValueTpl.clone();
        (cref, exp) = keyValueTpl.clone();
        if isValidReplacement(cref.clone(), exp.clone(), exceptionMap.clone()) {
            UnorderedMap::add(cref.clone(), exp.clone(), newReplacements.clone())?;
        } else {
            attr = BackendDAE::lowerEquationAttributes(ComponentRef::getSubscriptedType(cref.clone(), false)?, false);
            eqPtr = BEquation::Equation::makeAssignment(Expression::fromCref(cref.clone(), false)?, exp.clone(), BEquation::EqData::getUniqueIndex(eqData.clone())?, (literal!("SIM")).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), attr.clone())?;
            auxEquations = cons(eqPtr.clone(), auxEquations.clone());
        }
    }
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        dumpReplacements(newReplacements.clone(), auxEquations.clone())?;
    }
    Ok((newReplacements, auxEquations))
}

fn isValidReplacement(mut cref: Arc<ComponentRef::NFComponentRef>, mut exp: Arc<Expression::NFExpression>, mut exceptionMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, ExceptionKind>>) -> bool {
    let mut b: bool = true;
    b = (::match_deref::match_deref! { match &((UnorderedMap::get(cref.clone(), exceptionMap.clone()), exp.clone())) {
        (None, _) => true,
        (Some(ExceptionKind::CREF_ALIAS), Deref @ Expression::CREF { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// different kinds of exceptions
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ExceptionKind {
    NO_ALIAS = 1,
    CREF_ALIAS = 2,
}
impl PartialOrd for ExceptionKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ExceptionKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for ExceptionKind {
    fn default() -> Self { Self::NO_ALIAS }
}

fn filterExceptionsEquation(mut eqn: Arc<Equation::Equation>, mut acc: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, ExceptionKind>>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let _ = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::ALGORITHM { .. } => {
            for mut cref in &*var_field!((*eqn).alg, Equation::Equation::ALGORITHM).outputs.clone() {
                let mut cref = cref.clone();
                UnorderedMap::add(cref.clone(), ExceptionKind::NO_ALIAS.clone(), acc.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    BEquation::Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = acc.clone(); move |__pe_a0| filterExceptions(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(eqn)
}

fn filterExceptions(mut exp: Arc<Expression::NFExpression>, mut acc: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, ExceptionKind>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref, .. }, tail: Deref @ metamodelica::List::Nil }, .. } } if (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)? == literal!("pre")) => {
            UnorderedMap::add(cref.clone(), ExceptionKind::NO_ALIAS.clone(), acc.clone())?;
            ()
        },
        Deref @ Expression::CREF { .. } => {
            ()
        },
        Deref @ Expression::TUPLE { .. } => {
            for mut elem in &*var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut elem = elem.clone();
                let _ = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ Expression::CREF { .. } => {
            UnorderedMap::add(var_field!((*elem).cref, Expression::NFExpression::CREF).clone(), ExceptionKind::CREF_ALIAS.clone(), acc.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn dumpReplacements(mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut auxEquations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Replacements::simpleToString(replacements.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if !(auxEquations.clone().is_empty()) {
        println!("{}", (StringUtil::headline_4((literal!("[dumprepl] Found But Illegal Alias Replacements (added as equations):")).clone())).clone());
        for mut eqPtr in &*auxEquations.clone() {
            let mut eqPtr = eqPtr.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*BEquation::Equation::toString(Pointer::access(eqPtr.clone()), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

fn aliasCausalize(mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut kind: Partition::Kind, mut context: ArcStr) -> Result<(Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, Arc<EquationPointers::EquationPointers>)> {
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut newEquations: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
    let mut size: i32 = 0;
    let mut setIdx: i32 = 1;
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<AliasSet::AliasSet>>>>;
    let mut sets: Arc<metamodelica::List<Arc<AliasSet::AliasSet>>> = metamodelica::nil();
    size = BVariable::VariablePointers::size(variables.clone());
    map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), size.clone());
    (newEquations, map) = BEquation::EquationPointers::foldRemovePtr(equations.clone(), (std::sync::Arc::new(findSimpleEquation) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<AliasSet::AliasSet>>>>) -> Result<(Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<AliasSet::AliasSet>>>>, bool)> + 'static>), map.clone())?;
    sets = getSimpleSets(map.clone(), size.clone())?;
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dumprepl] ")); __mm_s.push_str(&*context.clone()); __mm_s.push_str(&*literal!(" Alias Sets:")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        if sets.clone().is_empty() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<No ")); __mm_s.push_str(&*context.clone()); __mm_s.push_str(&*literal!(" Alias Sets>\n\n")); ArcStr::from(__mm_s) }).clone());
        } else {
            for mut set in &*sets.clone() {
                let mut set = set.clone();
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Alias Set ")); __mm_s.push_str(&*intString(setIdx.clone())); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*AliasSet::toString(set.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                setIdx = setIdx.clone() + 1;
            }
        }
    }
    replacements = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), size.clone());
    for mut set in &*sets.clone() {
        let mut set = set.clone();
        replacements = createReplacementRules(set.clone(), replacements.clone(), kind.clone())?;
    }
    Ok((replacements, newEquations))
}

fn findSimpleEquation(mut eq_ptr: Pointer::Pointer<Arc<Equation::Equation>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<AliasSet::AliasSet>>>>) -> Result<(Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<AliasSet::AliasSet>>>>, bool)> {
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<AliasSet::AliasSet>>>> = map;
    let mut delete: bool = false;
    let mut eq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut crefTpl: CrefTpl = EMPTY_CREF_TPL().clone();
    eq = Pointer::access(eq_ptr.clone());
    crefTpl = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { .. } if (isSimpleExp(var_field!((*eq).lhs, Equation::Equation::SCALAR_EQUATION).clone(), true) && isSimpleExp(var_field!((*eq).rhs, Equation::Equation::SCALAR_EQUATION).clone(), true)) => {
            crefTpl = Expression::fold(var_field!((*eq).rhs, Equation::Equation::SCALAR_EQUATION).clone(), (std::sync::Arc::new(findCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl> + 'static>), crefTpl.clone())?;
            crefTpl = Expression::fold(var_field!((*eq).lhs, Equation::Equation::SCALAR_EQUATION).clone(), (std::sync::Arc::new(findCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl> + 'static>), crefTpl.clone())?;
            crefTpl.clone()
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { .. } if (isSimpleExp(var_field!((*eq).lhs, Equation::Equation::ARRAY_EQUATION).clone(), true) && isSimpleExp(var_field!((*eq).rhs, Equation::Equation::ARRAY_EQUATION).clone(), true)) => {
            crefTpl = Expression::fold(var_field!((*eq).rhs, Equation::Equation::ARRAY_EQUATION).clone(), (std::sync::Arc::new(findCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl> + 'static>), crefTpl.clone())?;
            crefTpl = Expression::fold(var_field!((*eq).lhs, Equation::Equation::ARRAY_EQUATION).clone(), (std::sync::Arc::new(findCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl> + 'static>), crefTpl.clone())?;
            crefTpl.clone()
        },
        _ => crefTpl.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (map, delete) = (::match_deref::match_deref! { match &(crefTpl.clone()) {
        CrefTpl { cr_lst: Deref @ metamodelica::List::Cons { head: cr1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut set_ptr: SetPtr;
            let mut set: Arc<AliasSet::AliasSet> = Arc::new(<AliasSet::AliasSet as ::std::default::Default>::default());
            if !(UnorderedMap::contains(cr1.clone(), map.clone())) {
                set = EMPTY_ALIAS_SET().clone();
                assign_field!(
                    set.simple_variables = list![cr1.clone()],
                    set.const_opt = Some(Pointer::create(eq.clone()))
                );
                UnorderedMap::add(cr1.clone(), Pointer::create(set.clone()), map.clone())?;
            } else {
                set_ptr = UnorderedMap::getOrFail(cr1.clone(), map.clone());
                set = Pointer::access(set_ptr.clone());
                if isSome(set.const_opt.clone()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.findSimpleEquation")); __mm_s.push_str(&*literal!(" failed to add Equation:\n")); __mm_s.push_str(&*BEquation::Equation::toString(eq.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n because the set already contains a constant binding.\n              Overdetermined Set!:")); __mm_s.push_str(&*AliasSet::toString(set.clone())?); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                } else {
                    assign_field!(set.const_opt = Some(Pointer::create(eq.clone())));
                    Pointer::update(set_ptr.clone(), set.clone());
                }
            }
            (map.clone(), true)
        },
        CrefTpl { cr_lst: Deref @ metamodelica::List::Cons { head: cr1, tail: Deref @ metamodelica::List::Cons { head: cr2, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            let mut set_ptr: SetPtr;
            let mut set1_ptr: SetPtr;
            let mut set2_ptr: SetPtr;
            let mut set: Arc<AliasSet::AliasSet> = Arc::new(<AliasSet::AliasSet as ::std::default::Default>::default());
            let mut set1: Arc<AliasSet::AliasSet> = Arc::new(<AliasSet::AliasSet as ::std::default::Default>::default());
            let mut set2: Arc<AliasSet::AliasSet> = Arc::new(<AliasSet::AliasSet as ::std::default::Default>::default());
            if UnorderedMap::contains(cr1.clone(), map.clone()) && UnorderedMap::contains(cr2.clone(), map.clone()) {
                set1_ptr = UnorderedMap::getOrFail(cr1.clone(), map.clone());
                set2_ptr = UnorderedMap::getOrFail(cr2.clone(), map.clone());
                set1 = Pointer::access(set1_ptr.clone());
                set2 = Pointer::access(set2_ptr.clone());
                set = EMPTY_ALIAS_SET().clone();
                if referenceEq(&set1_ptr.clone(),&set2_ptr.clone()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.findSimpleEquation")); __mm_s.push_str(&*literal!(" failed to merge following sets ")); __mm_s.push_str(&*literal!("because they would create a loop. This would create an underdetermined Set!:\n\n")); __mm_s.push_str(&*literal!("Trying to merge: ")); __mm_s.push_str(&*BEquation::Equation::toString(eq.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n\n")); __mm_s.push_str(&*AliasSet::toString(set1.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*AliasSet::toString(set2.clone())?); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                } else if isSome(set1.const_opt.clone()) && isSome(set2.const_opt.clone()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.findSimpleEquation")); __mm_s.push_str(&*literal!(" failed to merge following sets ")); __mm_s.push_str(&*literal!("because both have a constant binding. This would create an overdetermined Set!:\n\n")); __mm_s.push_str(&*AliasSet::toString(set1.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*AliasSet::toString(set2.clone())?); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                } else if isSome(set1.const_opt.clone()) {
                    assign_field!(set.const_opt = set1.const_opt.clone());
                } else if isSome(set2.const_opt.clone()) {
                    assign_field!(set.const_opt = set2.const_opt.clone());
                }
                if List::compareLength(set1.simple_equations.clone(), set2.simple_equations.clone())? > 0 {
                    assign_field!(set.simple_equations = cons(Pointer::create(eq.clone()), Dangerous::listAppendDestroy(set2.simple_equations.clone(), set1.simple_equations.clone())?));
                } else {
                    assign_field!(set.simple_equations = cons(Pointer::create(eq.clone()), Dangerous::listAppendDestroy(set1.simple_equations.clone(), set2.simple_equations.clone())?));
                }
                if List::compareLength(set1.simple_variables.clone(), set2.simple_variables.clone())? > 0 {
                    assign_field!(set.simple_variables = Dangerous::listAppendDestroy(set2.simple_variables.clone(), set1.simple_variables.clone())?);
                    Pointer::update(set1_ptr.clone(), set.clone());
                    for mut cr in &*set2.simple_variables.clone() {
                        let mut cr = cr.clone();
                        UnorderedMap::add(cr.clone(), set1_ptr.clone(), map.clone())?;
                    }
                } else {
                    assign_field!(set.simple_variables = Dangerous::listAppendDestroy(set2.simple_variables.clone(), set1.simple_variables.clone())?);
                    Pointer::update(set2_ptr.clone(), set.clone());
                    for mut cr in &*set1.simple_variables.clone() {
                        let mut cr = cr.clone();
                        UnorderedMap::add(cr.clone(), set2_ptr.clone(), map.clone())?;
                    }
                }
            } else if UnorderedMap::contains(cr1.clone(), map.clone()) {
                set_ptr = UnorderedMap::getOrFail(cr1.clone(), map.clone());
                set = Pointer::access(set_ptr.clone());
                assign_field!(
                    set.simple_variables = cons(cr2.clone(), set.simple_variables.clone()),
                    set.simple_equations = cons(Pointer::create(eq.clone()), set.simple_equations.clone())
                );
                Pointer::update(set_ptr.clone(), set.clone());
                UnorderedMap::add(cr2.clone(), set_ptr.clone(), map.clone())?;
            } else if UnorderedMap::contains(cr2.clone(), map.clone()) {
                set_ptr = UnorderedMap::getOrFail(cr2.clone(), map.clone());
                set = Pointer::access(set_ptr.clone());
                assign_field!(
                    set.simple_variables = cons(cr1.clone(), set.simple_variables.clone()),
                    set.simple_equations = cons(Pointer::create(eq.clone()), set.simple_equations.clone())
                );
                Pointer::update(set_ptr.clone(), set.clone());
                UnorderedMap::add(cr1.clone(), set_ptr.clone(), map.clone())?;
            } else {
                set = EMPTY_ALIAS_SET().clone();
                assign_field!(
                    set.simple_variables = list![cr1.clone(), cr2.clone()],
                    set.simple_equations = list![Pointer::create(eq.clone())]
                );
                set_ptr = Pointer::create(set.clone());
                UnorderedMap::add(cr1.clone(), set_ptr.clone(), map.clone())?;
                UnorderedMap::add(cr2.clone(), set_ptr.clone(), map.clone())?;
            }
            (map.clone(), true)
        },
        _ => {
            (map.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((map, delete))
}

fn findCrefs(mut exp: Arc<Expression::NFExpression>, mut tpl: CrefTpl) -> Result<CrefTpl> {
    let mut tpl: CrefTpl = tpl;
    tpl = (::match_deref::match_deref! { match &(exp.clone()) {
        _ if (!(tpl.cont.clone())) => FAILED_CREF_TPL().clone(),
        Deref @ Expression::CREF { .. } if (BVariable::isParamOrConst(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?) || ComponentRef::isTime(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) => tpl.clone(),
        Deref @ Expression::CREF { .. } if (Util::isSome(BVariable::getParent(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?))) => FAILED_CREF_TPL().clone(),
        Deref @ Expression::CREF { .. } if (tpl.varCount.clone() < 2 && !(ComponentRef::hasSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))) => {
            tpl.cr_lst = cons(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), tpl.cr_lst.clone());
            tpl.varCount = tpl.varCount.clone() + 1;
            tpl.clone()
        },
        _ if (findCrefsFail(exp.clone())) => FAILED_CREF_TPL().clone(),
        _ => tpl.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tpl)
}

fn findCrefsFail(mut exp: Arc<Expression::NFExpression>) -> bool {
    let mut cont: bool = false;
    cont = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => true,
        Deref @ Expression::RELATION { .. } => true,
        Deref @ Expression::IF { .. } => true,
        Deref @ Expression::CALL { .. } => true,
        Deref @ Expression::RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cont
}

fn isSimpleExp(mut exp: Arc<Expression::NFExpression>, mut simple: bool) -> (bool, i32) {
    let mut simple: bool = simple;
    let mut num_cref: i32 = 0;
    if !(simple.clone()) {
        return (simple, num_cref);
    }
    (simple, num_cref) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            (true, 0)
        },
        Deref @ Expression::REAL { .. } => {
            (true, 0)
        },
        Deref @ Expression::BOOLEAN { .. } => {
            (true, 0)
        },
        Deref @ Expression::STRING { .. } => {
            (true, 0)
        },
        Deref @ Expression::CREF { .. } => {
            (true, 1)
        },
        Deref @ Expression::CAST { .. } => {
            isSimpleExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), true)
        },
        Deref @ Expression::UNARY { .. } => {
            (simple, num_cref) = isSimpleExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), true);
            simple = if (simple.clone()) {checkOp(var_field!((*exp).operator, Expression::NFExpression::UNARY).clone(), num_cref.clone())} else {false};
            (simple.clone(), num_cref.clone())
        },
        Deref @ Expression::LUNARY { .. } => {
            (simple, num_cref) = isSimpleExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), true);
            simple = if (simple.clone()) {checkOp(var_field!((*exp).operator, Expression::NFExpression::LUNARY).clone(), num_cref.clone())} else {false};
            (simple.clone(), num_cref.clone())
        },
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op, .. }, .. } => {
            let mut num_cref_tmp: i32 = 0;
            (simple, num_cref) = isSimpleExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), true);
            if op.clone() == Operator::Op::DIV.clone() && num_cref.clone() != 0 {
                simple = false;
                return (simple, num_cref);
            }
            (simple, num_cref_tmp) = isSimpleExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), simple.clone());
            num_cref = num_cref.clone() + num_cref_tmp.clone();
            simple = if (simple.clone()) {checkOp(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), num_cref.clone())} else {false};
            (simple.clone(), num_cref.clone())
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut num_cref_tmp: i32 = 0;
            (simple, num_cref) = isSimpleExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), true);
            (simple, num_cref_tmp) = isSimpleExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), simple.clone());
            num_cref = num_cref.clone() + num_cref_tmp.clone();
            simple = if (simple.clone()) {checkOp(var_field!((*exp).operator, Expression::NFExpression::LBINARY).clone(), num_cref.clone())} else {false};
            (simple.clone(), num_cref.clone())
        },
        Deref @ Expression::MULTARY { operator: Deref @ Operator::OPERATOR { op, .. }, .. } => {
            let mut num_cref_tmp: i32 = 0;
            for mut arg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                (simple, num_cref_tmp) = isSimpleExp(arg.clone(), simple.clone());
                if !(simple.clone()) {
                    return (simple, num_cref);
                }
                num_cref = num_cref.clone() + num_cref_tmp.clone();
            }
            if op.clone() == Operator::Op::MUL.clone() && num_cref.clone() != 0 {
                simple = false;
                return (simple, num_cref);
            }
            for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                (simple, num_cref_tmp) = isSimpleExp(arg.clone(), simple.clone());
                if !(simple.clone()) {
                    return (simple, num_cref);
                }
                num_cref = num_cref.clone() + num_cref_tmp.clone();
            }
            simple = if (simple.clone()) {checkOp(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone(), num_cref.clone())} else {false};
            (simple.clone(), num_cref.clone())
        },
        _ => {
            (false, num_cref.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (simple, num_cref)
}

fn checkOp(mut op: Arc<Operator::NFOperator>, mut cref_num: i32) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(op.clone()) {
        Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. } => true,
        Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. } => true,
        Deref @ Operator::OPERATOR { op: Operator::Op::UMINUS, .. } => true,
        Deref @ Operator::OPERATOR { op: Operator::Op::NOT, .. } => true,
        Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. } => cref_num.clone() < 2,
        Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. } => cref_num.clone() < 2,
        _ => cref_num.clone() == 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn getSimpleSets(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<AliasSet::AliasSet>>>>, mut size: i32) -> Result<Arc<metamodelica::List<Arc<AliasSet::AliasSet>>>> {
    let mut sets: Arc<metamodelica::List<Arc<AliasSet::AliasSet>>> = metamodelica::nil();
    let mut cref_marks: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), size.clone());
    let mut entry_lst: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<AliasSet::AliasSet>>)>> = metamodelica::nil();
    let mut simple_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut set_ptr: SetPtr;
    let mut set: Arc<AliasSet::AliasSet> = Arc::new(<AliasSet::AliasSet as ::std::default::Default>::default());
    entry_lst = UnorderedMap::toList(map.clone());
    for mut entry in &*entry_lst.clone() {
        let mut entry = entry.clone();
        (simple_cref, set_ptr) = entry.clone();
        if !(UnorderedSet::contains(simple_cref.clone(), cref_marks.clone())?) {
            set = Pointer::access(set_ptr.clone());
            sets = cons(set.clone(), sets.clone());
            for mut cr in &*set.simple_variables.clone() {
                let mut cr = cr.clone();
                if '__try0: {
                    unwrap_break_err!(UnorderedSet::addUnique(cr.clone(), cref_marks.clone()), '__try0);
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.getSimpleSets")); __mm_s.push_str(&*literal!(" failed because the set for ")); __mm_s.push_str(&*ComponentRef::toString(cr.clone())?); __mm_s.push_str(&*literal!(" was already added.")); ArcStr::from(__mm_s) }).clone()])?;
                }
            }
        }
    }
    Ok(sets)
}

fn createReplacementRules(mut set: Arc<AliasSet::AliasSet>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut kind: Partition::Kind) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>> {
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = replacements;
    replacements = ({
        let mut var_to_keep: Pointer::Pointer<Pointer::Pointer<Arc<Variable::NFVariable>>> = Pointer::create(Pointer::create(BVariable::DUMMY_VARIABLE().clone()));
        (match set.const_opt.clone() {
        Some(mut const_eq) => {
            let mut vars: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
            let mut eqs: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
            let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
            vars = BVariable::VariablePointers::fromList({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut cr in (set.simple_variables.clone()).into_iter().cloned() {
            let __x = BVariable::getVarPointer(cr.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, true);
            eqs = BEquation::EquationPointers::fromList(cons(const_eq.clone(), set.simple_equations.clone()));
            (_, comps) = Causalize::simple(vars.clone(), eqs.clone(), kind.clone(), NBAdjacency::MatrixStrictness::MATCHING.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY))?;
            Replacements::simple(comps.clone(), replacements.clone())?;
            replacements.clone()
        },
        _ => {
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut solved_eq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut eq: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut alias_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut vars: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
            let mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut eqs: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
            let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
            let mut collector: Arc<AttributeCollector::AttributeCollector>;
            (alias_vars, collector) = chooseVariableToKeep({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut cr in (set.simple_variables.clone()).into_iter().cloned() {
            let __x = BVariable::getVarPointer(cr.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, var_to_keep.clone())?;
            vars = BVariable::VariablePointers::fromList(alias_vars.clone(), false);
            eqs = BEquation::EquationPointers::fromList(set.simple_equations.clone());
            (_, comps) = Causalize::simple(vars.clone(), eqs.clone(), kind.clone(), NBAdjacency::MatrixStrictness::MATCHING.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY))?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_3((literal!("Variable to keep (values of attributes before replacements):")).clone())); __mm_s.push_str(&*BVariable::pointerToString(Pointer::access(var_to_keep.clone()))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            Replacements::simple(comps.clone(), replacements.clone())?;
            var_lst = BVariable::VariablePointers::toList(vars.clone())?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_4((literal!("Attribute collector (before replacements): ")).clone())); __mm_s.push_str(&*AttributeCollector::toString(collector.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            for mut var in &*var_lst.clone() {
                let mut var = var.clone();
                rhs = UnorderedMap::getSafe(BVariable::getVarName(var.clone()), replacements.clone(), metamodelica::sourceInfo!())?;
                eq = BEquation::Equation::makeAssignment(BVariable::toExpression(var.clone()), rhs.clone(), Pointer::create(0), (arcstr::literal!(BEquation::TMP_STR)).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), BEquation::default(EquationKind::UNKNOWN.clone(), false, None, None))?;
                (solved_eq, _, _) = Solve::solveBody(Pointer::access(eq.clone()), BVariable::getVarName(Pointer::access(var_to_keep.clone())), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1))?;
                collector = AttributeCollector::fixValues(collector.clone(), BVariable::getVarName(var.clone()), solved_eq.clone())?;
            }
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_4((literal!("Attribute collector (after replacements): ")).clone())); __mm_s.push_str(&*AttributeCollector::toString(collector.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            diffTearingSelect(collector.tearingSelect_map.clone(), set.clone())?;
            stateSelectAlways(collector.stateSelect_map.clone(), set.clone())?;
            checkNominalThreshold(collector.nominal_map.clone(), set.clone())?;
            setNewAttributes(var_to_keep.clone(), collector.clone(), set.clone())?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_3((literal!("Variable to keep (values of attributes after replacements):")).clone())); __mm_s.push_str(&*BVariable::pointerToString(Pointer::access(var_to_keep.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            replacements.clone()
        },
    })
    });
    Ok(replacements)
}

fn setNewAttributes(mut var_to_keep_ptr: Pointer::Pointer<Pointer::Pointer<Arc<Variable::NFVariable>>>, mut attrcollector: Arc<AttributeCollector::AttributeCollector>, mut set: Arc<AliasSet::AliasSet>) -> Result<()> {
    let mut lst: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut new_cref: Option<Arc<ComponentRef::NFComponentRef>> = None;
    let mut new_min: Option<Arc<Expression::NFExpression>> = None;
    let mut new_max: Option<Arc<Expression::NFExpression>> = None;
    let mut new_start: Option<Arc<Expression::NFExpression>> = None;
    let mut new_stateSelect: Option<StateSelect> = None;
    let mut new_tearingSelect: Option<TearingSelect> = None;
    let mut fixed_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut var_to_keep: Pointer::Pointer<Arc<Variable::NFVariable>> = Pointer::access(var_to_keep_ptr.clone());
    let mut fixed_start_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    new_min = getMaximum(attrcollector.min_val_map.clone())?;
    if Util::isSome(new_min.clone()) {
        Pointer::update(var_to_keep.clone(), BVariable::setMin(Pointer::access(var_to_keep.clone()), new_min.clone(), true)?);
        UnorderedMap::add(BVariable::getVarName(var_to_keep.clone()), Util::getOption(new_min.clone())?, attrcollector.min_val_map.clone())?;
    }
    new_max = getMinimum(attrcollector.max_val_map.clone())?;
    if Util::isSome(new_max.clone()) {
        Pointer::update(var_to_keep.clone(), BVariable::setMax(Pointer::access(var_to_keep.clone()), new_max.clone(), true)?);
        UnorderedMap::add(BVariable::getVarName(var_to_keep.clone()), Util::getOption(new_max.clone())?, attrcollector.max_val_map.clone())?;
    }
    fixed_start_map = setStartFixed(attrcollector.start_map.clone(), attrcollector.fixed_map.clone(), set.clone())?;
    if UnorderedMap::size(fixed_start_map.clone()) == 1 {
        new_start = Some(listHead(UnorderedMap::valueList(fixed_start_map.clone()))?);
        fixed_var = BVariable::getVarPointer(UnorderedMap::firstKey(fixed_start_map.clone()), metamodelica::sourceInfo!())?;
        BVariable::setFixed(fixed_var.clone(), false, true)?;
        UnorderedMap::add(BVariable::getVarName(fixed_var.clone()), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), attrcollector.fixed_map.clone())?;
        BVariable::setFixed(var_to_keep.clone(), true, true)?;
        UnorderedMap::add(BVariable::getVarName(var_to_keep.clone()), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), attrcollector.fixed_map.clone())?;
        Pointer::update(var_to_keep.clone(), BVariable::setStartAttribute(Pointer::access(var_to_keep.clone()), Util::getOption(new_start.clone())?, true)?);
        UnorderedMap::add(BVariable::getVarName(var_to_keep.clone()), Util::getOption(new_start.clone())?, attrcollector.start_map.clone())?;
    }
    (new_cref, new_stateSelect) = chooseStateSelect(attrcollector.stateSelect_map.clone())?;
    if Util::isSome(new_stateSelect.clone()) && Util::isSome(UnorderedMap::get(BVariable::getVarName(var_to_keep.clone()), attrcollector.stateSelect_map.clone())) {
        Pointer::update(var_to_keep.clone(), BVariable::setStateSelect(Pointer::access(var_to_keep.clone()), Util::getOption(new_stateSelect.clone())?, true)?);
        UnorderedMap::add(BVariable::getVarName(var_to_keep.clone()), Util::getOption(new_stateSelect.clone())?, attrcollector.stateSelect_map.clone())?;
        if Util::getOption(new_stateSelect.clone())? == StateSelect::ALWAYS.clone() {
            new_start = Some(UnorderedMap::getSafe(Util::getOption(new_cref.clone())?, attrcollector.start_map.clone(), metamodelica::sourceInfo!())?);
            Pointer::update(var_to_keep.clone(), BVariable::setStartAttribute(Pointer::access(var_to_keep.clone()), Util::getOption(new_start.clone())?, true)?);
            UnorderedMap::add(BVariable::getVarName(var_to_keep.clone()), Util::getOption(new_start.clone())?, attrcollector.start_map.clone())?;
        }
    }
    new_tearingSelect = chooseTearingSelect(attrcollector.tearingSelect_map.clone())?;
    if Util::isSome(new_tearingSelect.clone()) && Util::isSome(UnorderedMap::get(BVariable::getVarName(var_to_keep.clone()), attrcollector.tearingSelect_map.clone())) {
        Pointer::update(var_to_keep.clone(), BVariable::setTearingSelect(Pointer::access(var_to_keep.clone()), Util::getOption(new_tearingSelect.clone())?, true));
        UnorderedMap::add(BVariable::getVarName(var_to_keep.clone()), Util::getOption(new_tearingSelect.clone())?, attrcollector.tearingSelect_map.clone())?;
    }
    Pointer::update(var_to_keep_ptr.clone(), var_to_keep.clone());
    Ok(())
}

fn chooseVariableToKeep(mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut var_to_keep: Pointer::Pointer<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<AttributeCollector::AttributeCollector>)> {
    let mut acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut attrcollector: Arc<AttributeCollector::AttributeCollector> = Arc::new(AttributeCollector::AttributeCollector { min_val_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), max_val_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), start_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), fixed_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), nominal_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), stateSelect_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), tearingSelect_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1) });
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cur_var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut rest: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut cur_rating: i32 = 0;
    let mut max_rating: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var_lst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    rest = __pa1.clone();
    Pointer::update(var_to_keep.clone(), var.clone());
    (max_rating, attrcollector) = rateVar(var.clone(), attrcollector.clone())?;
    for mut var in &*rest.clone() {
        let mut var = var.clone();
        (cur_rating, attrcollector) = rateVar(var.clone(), attrcollector.clone())?;
        if cur_rating.clone() > max_rating.clone() {
            max_rating = cur_rating.clone();
            acc = cons(Pointer::access(var_to_keep.clone()), acc.clone());
            Pointer::update(var_to_keep.clone(), var.clone());
        } else {
            acc = cons(var.clone(), acc.clone());
        }
    }
    Ok((acc, attrcollector))
}

fn getMaximum(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut max_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut constants: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut lst_values: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = UnorderedMap::valueList(map.clone());
    let mut max_exp_val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut max_val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (constants, rest) = List::splitOnTrue(lst_values.clone(), (std::sync::Arc::new(fnptr!(Expression::isConstNumber, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>));
    if !(constants.clone().is_empty()) {
        max_val = List::maxElement({
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut val in (constants.clone()).into_iter().cloned() {
            let __x = Expression::realValue(val.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(realLt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))?;
        rest = cons(Arc::new(Expression::NFExpression::REAL { value: max_val.clone() }), rest.clone());
    }
    if rest.clone().is_empty() {
        max_exp = None;
    } else if List::hasOneElement(rest.clone()) {
        max_exp = Some(listHead(rest.clone())?);
    } else {
        max_exp_val = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_REAL().clone(), rest.clone(), Variability::PARAMETER.clone(), NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::MAX_REAL().returnType.clone()) });
        max_exp = Some(max_exp_val.clone());
    }
    Ok(max_exp)
}

fn getMinimum(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut min_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut constants: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut lst_values: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = UnorderedMap::valueList(map.clone());
    let mut min_exp_val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut min_val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (constants, rest) = List::splitOnTrue(lst_values.clone(), (std::sync::Arc::new(fnptr!(Expression::isConstNumber, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>));
    if !(constants.clone().is_empty()) {
        min_val = List::minElement({
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut val in (constants.clone()).into_iter().cloned() {
            let __x = Expression::realValue(val.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(realLt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))?;
        rest = cons(Arc::new(Expression::NFExpression::REAL { value: min_val.clone() }), rest.clone());
    }
    if rest.clone().is_empty() {
        min_exp = None;
    } else if List::hasOneElement(rest.clone()) {
        min_exp = Some(listHead(rest.clone())?);
    } else {
        min_exp_val = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_REAL().clone(), rest.clone(), Variability::PARAMETER.clone(), NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::MAX_REAL().returnType.clone()) });
        min_exp = Some(min_exp_val.clone());
    }
    Ok(min_exp)
}

fn setStartFixed(mut start_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut fixed_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut set: Arc<AliasSet::AliasSet>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>> {
    let mut fixed_start_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut fixed_lst: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)>> = UnorderedMap::toList(fixed_map.clone());
    let mut start_lst: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = UnorderedMap::valueList(start_map.clone());
    let mut fixed_start_lst: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut count_fixed: i32 = 0;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut sval: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fval: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    for mut tpl in &*fixed_lst.clone() {
        let mut tpl = tpl.clone();
        (cref, fval) = tpl.clone();
        if Expression::isTrue(fval.clone()) {
            count_fixed = count_fixed.clone() + 1;
            sval = UnorderedMap::getSafe(cref.clone(), start_map.clone(), metamodelica::sourceInfo!())?;
            UnorderedMap::add(cref.clone(), sval.clone(), fixed_start_map.clone())?;
        }
    }
    if count_fixed.clone() == 0 {
        if !(List::allEqual(start_lst.clone(), (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) {
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.setStartFixed")); __mm_s.push_str(&*literal!(": Alias set with conflicting unfixed start values detected.\n")); __mm_s.push_str(&*AliasSet::toString(set.clone())?); __mm_s.push_str(&*literal!("\n\tStart map after replacements:\n\t")); __mm_s.push_str(&*UnorderedMap::toString(start_map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); ArcStr::from(__mm_s) }).clone())?;
            } else {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.setStartFixed")); __mm_s.push_str(&*literal!(": Alias set with conflicting unfixed start values detected. Use -d=dumprepl for more information.\n")); ArcStr::from(__mm_s) }).clone())?;
            }
        }
    } else if count_fixed.clone() > 1 {
        fixed_start_lst = UnorderedMap::valueList(fixed_start_map.clone());
        if !(List::allEqual(fixed_start_lst.clone(), (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) {
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.setStartFixed")); __mm_s.push_str(&*literal!(" failed because multiple variables are fixed with different start values!\n")); __mm_s.push_str(&*AliasSet::toString(set.clone())?); __mm_s.push_str(&*literal!("\n\tFixed start map after replacements:\n\t")); __mm_s.push_str(&*UnorderedMap::toString(fixed_start_map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.setStartFixed")); __mm_s.push_str(&*literal!(" failed because multiple variables are fixed with different start values! Use -d=dumprepl for more information.\n")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
        } else if List::allEqual(fixed_start_lst.clone(), (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>))? {
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.setStartFixed")); __mm_s.push_str(&*literal!(": Multiple variables are fixed and have identical start values.\n")); __mm_s.push_str(&*AliasSet::toString(set.clone())?); __mm_s.push_str(&*literal!("\n\tFixed start map after replacements:\n\t")); __mm_s.push_str(&*UnorderedMap::toString(fixed_start_map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); ArcStr::from(__mm_s) }).clone())?;
            } else {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.setStartFixed")); __mm_s.push_str(&*literal!(": Multiple variables are fixed and have identical start values. Use -d=dumprepl for more information.\n")); ArcStr::from(__mm_s) }).clone())?;
            }
        }
    }
    Ok(fixed_start_map)
}

fn checkNominalThreshold(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut set: Arc<AliasSet::AliasSet>) -> Result<()> {
    let mut current: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut lst_values: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = UnorderedMap::valueList(map.clone());
    let mut arr_iter: metamodelica::Array<Arc<ExpressionIterator::NFExpressionIterator>>;
    let mut iter: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index: i32 = 1;
    if lst_values.clone().is_empty() {
        return Ok(());
    }
    if !(List::allEqual({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (lst_values.clone()).into_iter().cloned() {
            let __x = Type::sizeOf(Expression::typeOf(e.clone()), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) {
        Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.checkNominalThreshold")); __mm_s.push_str(&*literal!(" failed because array nominal values have different size. Use -d=dumprepl for more information.\n")); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    arr_iter = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<ExpressionIterator::NFExpressionIterator>>> = metamodelica::nil();
        for mut e in (lst_values.clone()).into_iter().cloned() {
            let __x = ExpressionIterator::fromExp(e.clone(), false, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
    while ExpressionIterator::hasNext(arr_iter.borrow()[(1-1) as usize].clone())? {
        current = metamodelica::nil();
        let __range0 = 1..=(arr_iter.clone().borrow().len() as i32);
        for mut i in __range0 {
            (iter, exp) = ExpressionIterator::next(arr_iter.borrow()[(i.clone()-1) as usize].clone())?;
            {let _arr = arr_iter.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = iter.clone(); _arr};
            current = cons(exp.clone(), current.clone());
        }
        checkNominalThresholdSingle(current.clone(), map.clone(), set.clone(), index.clone())?;
        index = index.clone() + 1;
    }
    Ok(())
}

fn checkNominalThresholdSingle(mut lst_values: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut set: Arc<AliasSet::AliasSet>, mut index: i32) -> Result<()> {
    let mut constants: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut zeroes: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut real_constants: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut nom_min: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nom_max: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nom_quotient: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut r#str: ArcStr = arcstr::literal!("");
    (constants, rest) = List::splitOnTrue(lst_values.clone(), (std::sync::Arc::new(fnptr!(Expression::isConstNumber, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>));
    (zeroes, constants) = List::splitOnTrue(constants.clone(), (std::sync::Arc::new(fnptr!(Expression::isZero, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>));
    if Flags::isSet(Flags::FAILTRACE.clone())? && !(rest.clone().is_empty()) {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.checkNominalThresholdSingle")); __mm_s.push_str(&*literal!(": There are non literal nominal values in following alias set:\n")); __mm_s.push_str(&*AliasSet::toString(set.clone())?); __mm_s.push_str(&*literal!("\n\tNominal map after replacements (conflicting array index = ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!("):\n\t")); __mm_s.push_str(&*UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); ArcStr::from(__mm_s) }).clone();
        Error::addCompilerWarning((r#str.clone()).clone())?;
    }
    if !(constants.clone().is_empty()) {
        real_constants = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut val in (constants.clone()).into_iter().cloned() {
            let __x = Expression::realValue(val.clone())?.abs();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        nom_min = List::minElement(real_constants.clone(), (std::sync::Arc::new(fnptr!(realLt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))?;
        nom_max = List::maxElement(real_constants.clone(), (std::sync::Arc::new(fnptr!(realLt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))?;
        nom_quotient = nom_max.clone() / nom_min.clone();
        if nom_quotient.clone() > NOMINAL_THRESHOLD.clone() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.checkNominalThresholdSingle")); __mm_s.push_str(&*literal!(": The quotient of the greatest and lowest nominal value is greater than the nominal threshold = ")); __mm_s.push_str(&*realString(NOMINAL_THRESHOLD.clone())); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone();
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*AliasSet::toString(set.clone())?); __mm_s.push_str(&*literal!("\n\tNominal map after replacements (conflicting array index = ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!("):\n\t")); __mm_s.push_str(&*UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); ArcStr::from(__mm_s) }).clone();
            } else {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" Use -d=dumprepl for more information.\n")); ArcStr::from(__mm_s) }).clone();
            }
            Error::addCompilerWarning((r#str.clone()).clone())?;
        }
    }
    if !(zeroes.clone().is_empty()) {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.checkNominalThresholdSingle")); __mm_s.push_str(&*literal!(" failed because zero values are not allowed.")); ArcStr::from(__mm_s) }).clone();
        if Flags::isSet(Flags::DUMP_REPL.clone())? {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n\tNominal map after replacements (violating array index = ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!("):\n\t")); __mm_s.push_str(&*UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" Use -d=dumprepl for more information.\n")); ArcStr::from(__mm_s) }).clone();
        }
        Error::addCompilerError((r#str.clone()).clone())?;
        bail!("fail");
    }
    Ok(())
}

fn stateSelectAlways(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, StateSelect>>, mut set: Arc<AliasSet::AliasSet>) -> Result<()> {
    let mut lst_values: Arc<metamodelica::List<StateSelect>> = UnorderedMap::valueList(map.clone());
    let mut count: i32 = 0;
    for mut val in &*lst_values.clone() {
        let mut val = val.clone();
        if val.clone() == StateSelect::ALWAYS.clone() {
            count = count.clone() + 1;
        }
    }
    if count.clone() > 1 {
        if Flags::isSet(Flags::DUMP_REPL.clone())? {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.stateSelectAlways")); __mm_s.push_str(&*literal!(" failed because multiple variables have StateSelect = always!\n")); __mm_s.push_str(&*AliasSet::toString(set.clone())?); __mm_s.push_str(&*literal!("\n\tStateSelect map after replacements:\n\t")); __mm_s.push_str(&*UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(BackendExtension::VariableAttributes::stateSelectString) as std::sync::Arc<dyn ::std::ops::Fn(StateSelect) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.stateSelectAlways")); __mm_s.push_str(&*literal!(" failed because multiple variables have StateSelect = always! Use -d=dumprepl for more information.\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
    }
    Ok(())
}

fn diffTearingSelect(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, TearingSelect>>, mut set: Arc<AliasSet::AliasSet>) -> Result<()> {
    let mut lst_values: Arc<metamodelica::List<TearingSelect>> = UnorderedMap::valueList(map.clone());
    let mut first: TearingSelect = TearingSelect::NEVER;
    let mut rest: Arc<metamodelica::List<TearingSelect>> = metamodelica::nil();
    let mut equal: bool = true;
    if !(lst_values.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst_values.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        first = __pa0.clone();
        rest = __pa1.clone();
        for mut val in &*rest.clone() {
            let mut val = val.clone();
            if first.clone() != val.clone() {
                equal = false;
                break;
            }
        }
        if !(equal.clone()) {
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("There are different TearingSelect values.\n")); __mm_s.push_str(&*AliasSet::toString(set.clone())?); __mm_s.push_str(&*literal!("\n\tTearingSelect map after replacements:\n\t")); __mm_s.push_str(&*UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(BackendExtension::VariableAttributes::tearingSelectString) as std::sync::Arc<dyn ::std::ops::Fn(TearingSelect) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); ArcStr::from(__mm_s) }).clone())?;
            } else {
                Error::addCompilerNotification((literal!("There are different TearingSelect values. Use -d=dumprepl for more information.\n")).clone())?;
            }
        }
    }
    Ok(())
}

fn chooseStateSelect(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, StateSelect>>) -> Result<(Option<Arc<ComponentRef::NFComponentRef>>, Option<StateSelect>)> {
    let mut chosen_cref: Option<Arc<ComponentRef::NFComponentRef>> = None;
    let mut chosen_val: Option<StateSelect> = None;
    let mut lst_values: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, StateSelect)>> = UnorderedMap::toList(map.clone());
    let mut sval: StateSelect = StateSelect::NEVER;
    let mut state_select: StateSelect = StateSelect::NEVER.clone();
    let mut compref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    if lst_values.clone().is_empty() {
        chosen_val = None;
        chosen_cref = None;
    } else if List::hasOneElement(lst_values.clone()) {
        (compref, sval) = listHead(lst_values.clone())?;
        chosen_val = Some(sval.clone());
        chosen_cref = Some(compref.clone());
    } else {
        for mut tpl in &*lst_values.clone() {
            let mut tpl = tpl.clone();
            (cref, sval) = tpl.clone();
            if sval.clone() > state_select.clone() {
                state_select = sval.clone();
                compref = cref.clone();
            }
        }
        chosen_val = Some(state_select.clone());
        chosen_cref = Some(compref.clone());
    }
    Ok((chosen_cref, chosen_val))
}

fn chooseTearingSelect(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, TearingSelect>>) -> Result<Option<TearingSelect>> {
    let mut chosen_val: Option<TearingSelect> = None;
    let mut lst_values: Arc<metamodelica::List<TearingSelect>> = UnorderedMap::valueList(map.clone());
    let mut tearing_select: TearingSelect = TearingSelect::NEVER;
    if lst_values.clone().is_empty() {
        chosen_val = None;
    } else if List::hasOneElement(lst_values.clone()) {
        chosen_val = Some(listHead(lst_values.clone())?);
    } else {
        tearing_select = TearingSelect::NEVER.clone();
        for mut val in &*lst_values.clone() {
            let mut val = val.clone();
            if val.clone() > tearing_select.clone() {
                tearing_select = val.clone();
            }
        }
        chosen_val = Some(tearing_select.clone());
    }
    Ok(chosen_val)
}

fn mean(mut lst: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<metamodelica::Real> {
    let mut mean_val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut cur_sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    cur_sum = {
        let mut __acc: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        for mut val in (lst.clone()).into_iter().cloned() {
            let __x = Expression::realValue(val.clone())?;
            __acc += __x;
        }
        __acc
    };
    mean_val = cur_sum.clone() / metamodelica::OrderedFloat(((lst.clone().len() as i32)) as f64);
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Mean = ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", mean_val.clone()))); ArcStr::from(__mm_s) }).clone());
    Ok(mean_val)
}

fn optionMinMax(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut attr_min: Option<Arc<Expression::NFExpression>>, mut attr_max: Option<Arc<Expression::NFExpression>>, mut attrcollector: Arc<AttributeCollector::AttributeCollector>) -> Result<Arc<AttributeCollector::AttributeCollector>> {
    let mut attrcollector: Arc<AttributeCollector::AttributeCollector> = attrcollector;
    let mut min_val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut max_val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Util::isSome(attr_min.clone()) {
        min_val = Util::getOption(attr_min.clone())?;
        UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), min_val.clone(), attrcollector.min_val_map.clone())?;
    }
    if Util::isSome(attr_max.clone()) {
        max_val = Util::getOption(attr_max.clone())?;
        UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), max_val.clone(), attrcollector.max_val_map.clone())?;
    }
    Ok(attrcollector)
}

fn optionStartFixed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut attr_start: Option<Arc<Expression::NFExpression>>, mut attr_fixed: Option<Arc<Expression::NFExpression>>, mut attrcollector: Arc<AttributeCollector::AttributeCollector>) -> Result<Arc<AttributeCollector::AttributeCollector>> {
    let mut attrcollector: Arc<AttributeCollector::AttributeCollector> = attrcollector;
    let mut start_val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fixed_val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Util::isSome(attr_start.clone()) {
        start_val = Util::getOption(attr_start.clone())?;
        UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), start_val.clone(), attrcollector.start_map.clone())?;
    }
    if Util::isSome(attr_fixed.clone()) {
        fixed_val = Util::getOption(attr_fixed.clone())?;
        UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), fixed_val.clone(), attrcollector.fixed_map.clone())?;
    }
    Ok(attrcollector)
}

fn rateVar(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut attrcollector: Arc<AttributeCollector::AttributeCollector>) -> Result<(i32, Arc<AttributeCollector::AttributeCollector>)> {
    let mut rating: i32 = 0;
    let mut attrcollector: Arc<AttributeCollector::AttributeCollector> = attrcollector;
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut nominal_val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stateSelect_val: StateSelect = StateSelect::NEVER;
    let mut tearingSelect_val: TearingSelect = TearingSelect::NEVER;
    if BVariable::isFunctionAlias(var_ptr.clone())? || BVariable::isClockAlias(var_ptr.clone())? {
        rating = -10000;
    } else {
        name = BVariable::getVarName(var_ptr.clone());
        rating = -(ComponentRef::depth(name.clone()));
    }
    let _ = ({
        let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
        (::match_deref::match_deref! { match &(Pointer::access(var_ptr.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: attr @ Deref @ BackendExtension::VariableAttributes::VAR_ATTR_REAL { .. }, .. }, .. } => {
            attrcollector = optionMinMax(var_ptr.clone(), var_field!((**attr).min, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**attr).max, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), attrcollector.clone())?;
            attrcollector = optionStartFixed(var_ptr.clone(), var_field!((**attr).start, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**attr).fixed, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), attrcollector.clone())?;
            if Util::isSome(var_field!((**attr).nominal, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone()) {
                nominal_val = Util::getOption(var_field!((**attr).nominal, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone())?;
                UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), nominal_val.clone(), attrcollector.nominal_map.clone())?;
            }
            if Util::isSome(var_field!((**attr).stateSelect, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone()) {
                stateSelect_val = Util::getOption(var_field!((**attr).stateSelect, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone())?;
                if stateSelect_val.clone() == StateSelect::ALWAYS.clone() {
                    rating = rating.clone() + 100;
                }
                UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), stateSelect_val.clone(), attrcollector.stateSelect_map.clone())?;
            }
            if Util::isSome(var_field!((**attr).tearingSelect, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone()) {
                tearingSelect_val = Util::getOption(var_field!((**attr).tearingSelect, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone())?;
                UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), tearingSelect_val.clone(), attrcollector.tearingSelect_map.clone())?;
            }
            ()
        },
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: attr @ Deref @ BackendExtension::VariableAttributes::VAR_ATTR_INT { .. }, .. }, .. } => {
            attrcollector = optionMinMax(var_ptr.clone(), var_field!((**attr).min, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**attr).max, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), attrcollector.clone())?;
            attrcollector = optionStartFixed(var_ptr.clone(), var_field!((**attr).start, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**attr).fixed, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), attrcollector.clone())?;
            ()
        },
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: attr @ Deref @ BackendExtension::VariableAttributes::VAR_ATTR_BOOL { .. }, .. }, .. } => {
            attrcollector = optionStartFixed(var_ptr.clone(), var_field!((**attr).start, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), var_field!((**attr).fixed, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), attrcollector.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((rating, attrcollector))
}

pub mod AttributeCollector {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AttributeCollector {
        /// set containing all minimum values
        pub min_val_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>,
        /// set containing all maximum values
        pub max_val_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>,
        /// set containing all start values
        pub start_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>,
        /// set containing all fixed values
        pub fixed_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>,
        /// set containing all nominal values
        pub nominal_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>,
        /// set containing all stateSelect values
        pub stateSelect_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, StateSelect>>,
        /// set containing all tearingSelect values
        pub tearingSelect_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, TearingSelect>>,
    }

    pub type ATTRIBUTE_COLLECTOR = AttributeCollector;

    pub fn toString(mut attrcollector: Arc<AttributeCollector>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut array_maps: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>>;
        let mut array_names: metamodelica::Array<ArcStr>;
        array_maps = metamodelica::arrayFromVec(list![attrcollector.min_val_map.clone(), attrcollector.max_val_map.clone(), attrcollector.start_map.clone(), attrcollector.fixed_map.clone(), attrcollector.nominal_map.clone()].into_iter().cloned().collect());
        array_names = metamodelica::arrayFromVec(list![(literal!("Min map")).clone(), (literal!("Max map")).clone(), (literal!("Start map")).clone(), (literal!("Fixed map")).clone(), (literal!("Nominal map")).clone()].into_iter().cloned().collect());
        let __range0 = 1..=(array_maps.clone().borrow().len() as i32);
        for mut i in __range0 {
            if UnorderedMap::isEmpty(array_maps.borrow()[(i.clone()-1) as usize].clone()) == false {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*array_names.clone().borrow()[(i.clone()-1) as usize].clone()); __mm_s.push_str(&*literal!(":\n\t")); __mm_s.push_str(&*UnorderedMap::toString(array_maps.borrow()[(i.clone()-1) as usize].clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
        }
        if UnorderedMap::isEmpty(attrcollector.stateSelect_map.clone()) == false {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("StateSelect map")); __mm_s.push_str(&*literal!(":\n\t")); __mm_s.push_str(&*UnorderedMap::toString(attrcollector.stateSelect_map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(BackendExtension::VariableAttributes::stateSelectString) as std::sync::Arc<dyn ::std::ops::Fn(StateSelect) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        if UnorderedMap::isEmpty(attrcollector.tearingSelect_map.clone()) == false {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("TearingSelect map")); __mm_s.push_str(&*literal!(":\n\t")); __mm_s.push_str(&*UnorderedMap::toString(attrcollector.tearingSelect_map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(BackendExtension::VariableAttributes::tearingSelectString) as std::sync::Arc<dyn ::std::ops::Fn(TearingSelect) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn fixValues(mut attrcollector: Arc<AttributeCollector>, mut var_cref: Arc<ComponentRef::NFComponentRef>, mut solved_eq: Arc<Equation::Equation>) -> Result<Arc<AttributeCollector>> {
        let mut attrcollector: Arc<AttributeCollector> = attrcollector;
        let mut repl: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut new_rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut diff_rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut args: Arc<DifferentiationArguments::DifferentiationArguments>;
        let mut swap_min_max: bool = false;
        let mut min_val_opt: Option<Arc<Expression::NFExpression>> = UnorderedMap::get(var_cref.clone(), attrcollector.min_val_map.clone());
        let mut max_val_opt: Option<Arc<Expression::NFExpression>> = UnorderedMap::get(var_cref.clone(), attrcollector.max_val_map.clone());
        let mut start_opt: Option<Arc<Expression::NFExpression>> = UnorderedMap::get(var_cref.clone(), attrcollector.start_map.clone());
        let mut nominal_opt: Option<Arc<Expression::NFExpression>> = UnorderedMap::get(var_cref.clone(), attrcollector.nominal_map.clone());
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let __pa0 = ::match_deref::match_deref! { match &(BEquation::Equation::getRHS(solved_eq.clone())?) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        rhs = __pa0.clone();
        if Util::isSome(min_val_opt.clone()) {
            UnorderedMap::add(var_cref.clone(), Util::getOption(min_val_opt.clone())?, repl.clone())?;
            new_rhs = Expression::map(rhs.clone(), Arc::new({ let __pe_b1 = repl.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
            new_rhs = SimplifyExp::simplify(new_rhs.clone(), false)?;
            UnorderedMap::add(var_cref.clone(), new_rhs.clone(), attrcollector.min_val_map.clone())?;
            min_val_opt = UnorderedMap::get(var_cref.clone(), attrcollector.min_val_map.clone());
        }
        if Util::isSome(max_val_opt.clone()) {
            UnorderedMap::add(var_cref.clone(), Util::getOption(max_val_opt.clone())?, repl.clone())?;
            new_rhs = Expression::map(rhs.clone(), Arc::new({ let __pe_b1 = repl.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
            new_rhs = SimplifyExp::simplify(new_rhs.clone(), false)?;
            UnorderedMap::add(var_cref.clone(), new_rhs.clone(), attrcollector.max_val_map.clone())?;
            max_val_opt = UnorderedMap::get(var_cref.clone(), attrcollector.max_val_map.clone());
        }
        ty = Expression::typeOf(rhs.clone());
        if Type::isContinuous(ty.clone())? || Type::isInteger(Type::elementType(ty.clone())) {
            args = Differentiate::DifferentiationArguments::default(Differentiate::DifferentiationType::SIMPLE.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1));
            assign_field!(args.diffCref = var_cref.clone());
            (diff_rhs, _) = Differentiate::differentiateExpression(rhs.clone(), args.clone())?;
            diff_rhs = SimplifyExp::simplify(diff_rhs.clone(), false)?;
            swap_min_max = Expression::isNegative(diff_rhs.clone());
        } else {
            swap_min_max = false;
        }
        if swap_min_max.clone() && Util::isSome(min_val_opt.clone()) && Util::isSome(max_val_opt.clone()) {
            UnorderedMap::add(var_cref.clone(), Util::getOption(max_val_opt.clone())?, attrcollector.min_val_map.clone())?;
            UnorderedMap::add(var_cref.clone(), Util::getOption(min_val_opt.clone())?, attrcollector.max_val_map.clone())?;
        }
        if Util::isSome(start_opt.clone()) {
            UnorderedMap::add(var_cref.clone(), Util::getOption(start_opt.clone())?, repl.clone())?;
            new_rhs = Expression::map(rhs.clone(), Arc::new({ let __pe_b1 = repl.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
            new_rhs = SimplifyExp::simplify(new_rhs.clone(), false)?;
            UnorderedMap::add(var_cref.clone(), new_rhs.clone(), attrcollector.start_map.clone())?;
        }
        if Util::isSome(nominal_opt.clone()) {
            UnorderedMap::add(var_cref.clone(), Util::getOption(nominal_opt.clone())?, repl.clone())?;
            new_rhs = Expression::map(rhs.clone(), Arc::new({ let __pe_b1 = repl.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
            new_rhs = Expression::getNominal(new_rhs.clone())?;
            UnorderedMap::add(var_cref.clone(), new_rhs.clone(), attrcollector.nominal_map.clone())?;
        }
        Ok(attrcollector)
    }

}

