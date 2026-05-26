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

use crate::NFAttributes as Attributes;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFClockKind;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEquation::ScalarizeMode;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;

// ============================================================
// Internal data types
// ============================================================
#[derive(Clone, Debug, PartialEq)]
pub struct Transition {
    pub from: i32,
    pub to: i32,
    pub condition: Arc<Expression::NFExpression>,
    pub immediate: bool,
    pub reset: bool,
    pub synchronize: bool,
    pub priority: i32,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            condition: Default::default(),
            immediate: Default::default(),
            reset: Default::default(),
            synchronize: Default::default(),
            priority: Default::default(),
        }
    }
}

pub type TRANSITION = Transition;


#[derive(Clone, Debug, PartialEq)]
pub struct FlatSmSemantics {
    /// Cref of the initial state (used as prefix for smOf vars)
    pub initStateRef: Arc<ComponentRef::NFComponentRef>,
    /// State crefs; index 1 = initial state
    pub smComps: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>,
    /// Transitions sorted by priority
    pub t: Arc<metamodelica::List<Transition>>,
    /// Conditions sorted by priority
    pub c: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
    /// SMS discrete variables
    pub vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>,
    /// SMS parameters/constants
    pub knowns: Arc<metamodelica::List<Arc<Variable::NFVariable>>>,
    /// SMS equations
    pub eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
    /// Propagation variables
    pub pvars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>,
    /// Propagation equations
    pub peqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
    /// Enclosing state if hierarchical SM
    pub enclosingState: Option<Arc<ComponentRef::NFComponentRef>>,
}

pub type FLAT_SM_SEMANTICS = FlatSmSemantics;


pub const SMS_PRE: &'static str = "smOf";

// ============================================================
// Public entry point
// ============================================================
pub fn flatten(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut initStates: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut smGroups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut smEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut otherEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut resultEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut smVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut resultVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut allStateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>;
    let mut stateToSem: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, FlatSmSemantics>>;
    let mut smGroupPairs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    let mut smGroupsSorted: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    let mut sem: FlatSmSemantics;
    let mut initState: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut parentPrefix: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>> = None;
    let mut enclosingSmSemOpt: Option<FlatSmSemantics> = None;
    if !(List::any(flatModel.equations.clone(), Arc::new(fnptr!(isTransitionOrInitialState, Arc<Equation::NFEquation>)))) && !(List::any(flatModel.initialEquations.clone(), Arc::new(fnptr!(isTransitionOrInitialState, Arc<Equation::NFEquation>)))) {
        return Ok(flatModel);
    }
    (initStates, smGroups) = groupStateMachines(flatModel.equations.clone(), flatModel.initialEquations.clone())?;
    if initStates.clone().is_empty() {
        return Ok(flatModel);
    }
    allStateCrefs = List::flatten(smGroups.clone());
    otherEqs = List::filterOnFalse(flatModel.equations.clone(), Arc::new(fnptr!(isTransitionOrInitialState, Arc<Equation::NFEquation>)));
    otherEqs = List::filterOnFalse(otherEqs.clone(), Arc::new({ let __pe_b1 = allStateCrefs.clone(); move |__pe_a0| isOuterStateEquation(__pe_a0, __pe_b1.clone()) }));
    outerVarMap = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
    smGroupPairs = List::zip(initStates.clone(), smGroups.clone());
    smGroupsSorted = List::sort(smGroupPairs.clone(), Arc::new(fnptr!(smGroupDepthLt, (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>), (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>))))?;
    stateToSem = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
    smVars = metamodelica::nil();
    smEqs = metamodelica::nil();
    for mut smPair in &*smGroupsSorted.clone() {
        let mut smPair = smPair.clone();
        (initState, stateCrefs) = smPair.clone();
        parentPrefix = ComponentRef::rest(initState.clone())?;
        if ComponentRef::isEmpty(parentPrefix.clone()) {
            enclosingStateCrefOpt = None;
            enclosingSmSemOpt = None;
        } else {
            enclosingSmSemOpt = UnorderedMap::get(parentPrefix.clone(), stateToSem.clone());
            enclosingStateCrefOpt = if (isSome(enclosingSmSemOpt.clone())) {Some(parentPrefix.clone())} else {None};
        }
        (smEqs, smVars, sem) = flatSmToDataFlow(initState.clone(), stateCrefs.clone(), flatModel.equations.clone(), flatModel.variables.clone(), enclosingStateCrefOpt.clone(), enclosingSmSemOpt.clone(), smEqs.clone(), smVars.clone(), outerVarMap.clone())?;
        for mut sc in &*stateCrefs.clone() {
            let mut sc = sc.clone();
            UnorderedMap::addUnique(sc.clone(), sem.clone(), stateToSem.clone())?;
        }
    }
    for mut outerVarCref in &*UnorderedMap::keyList(outerVarMap.clone()) {
        let mut outerVarCref = outerVarCref.clone();
        (smEqs, smVars) = generateMergeEquation(outerVarCref.clone(), outerVarMap.clone(), flatModel.variables.clone(), smEqs.clone(), smVars.clone())?;
    }
    resultEqs = listAppend({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (smEqs.clone()).into_iter().cloned() {
            let __x = subsActiveStateInEq(eq.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (otherEqs.clone()).into_iter().cloned() {
            let __x = subsActiveStateInEq(eq.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    resultVars = listAppend(smVars.clone(), flatModel.variables.clone());
    assign_field!(
        flatModel.equations = resultEqs.clone(),
        flatModel.initialEquations = List::filterOnFalse(flatModel.initialEquations.clone(), Arc::new(fnptr!(isTransitionOrInitialState, Arc<Equation::NFEquation>))),
        flatModel.variables = resultVars.clone()
    );
    execStat((literal!("NFStateMachineFlatten.flatten")).clone())?;
    Ok(flatModel)
}

// ============================================================
// SM group detection
// ============================================================
fn groupStateMachines(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut initialEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>)> {
    let mut initStates: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut smGroups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut allFroms: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut allTos: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut allInits: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cr1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cr2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut groups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut group: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    for mut eq in &*listAppend(equations.clone(), initialEquations.clone()) {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } => {
            let mut fname: ArcStr = arcstr::literal!("");
            fname = (Call::functionNameLast(eqCall.clone())).clone();
            if stringEq((fname.clone()).clone(), (literal!("transition")).clone()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::firstN(Call::arguments(eqCall.clone())?, 2)?) {
                    Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa1, .. }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                cr1 = __pa0.clone();
                cr2 = __pa1.clone();
                allFroms = cons(cr1.clone(), allFroms.clone());
                allTos = cons(cr2.clone(), allTos.clone());
            } else if stringEq((fname.clone()).clone(), (literal!("initialState")).clone()) {
                let __pa3 = ::match_deref::match_deref! { match &(List::firstN(Call::arguments(eqCall.clone())?, 1)?) {
                    Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa3, .. }, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                cr1 = __pa3.clone();
                allInits = cons(cr1.clone(), allInits.clone());
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    for mut initCref in &*allInits.clone() {
        let mut initCref = initCref.clone();
        group = collectReachableStates(initCref.clone(), allFroms.clone(), allTos.clone())?;
        initStates = cons(initCref.clone(), initStates.clone());
        smGroups = cons(group.clone(), smGroups.clone());
    }
    initStates = initStates.clone().reverse();
    smGroups = smGroups.clone().reverse();
    Ok((initStates, smGroups))
}

fn collectReachableStates(mut initCref: Arc<ComponentRef::NFComponentRef>, mut froms: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut tos: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut states: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut queue: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = list![initCref.clone()];
    let mut visited: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cur: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    states = metamodelica::nil();
    while !(queue.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(queue.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cur = __pa0.clone();
        queue = __pa1.clone();
        if !(List::isMemberOnTrue(cur.clone(), visited.clone(), Arc::new(ComponentRef::isEqual))) {
            visited = cons(cur.clone(), visited.clone());
            states = cons(cur.clone(), states.clone());
            for mut i in 1..=(froms.clone().len() as i32) {
                if ComponentRef::isEqual((froms.clone()).get(i.clone())?, cur.clone())? {
                    queue = cons((tos.clone()).get(i.clone())?, queue.clone());
                }
                if ComponentRef::isEqual((tos.clone()).get(i.clone())?, cur.clone())? {
                    queue = cons((froms.clone()).get(i.clone())?, queue.clone());
                }
            }
        }
    }
    states = List::sort(states.clone(), Arc::new({ let __pe_b2 = initCref.clone(); move |__pe_a0, __pe_a1| statePriorityGt(__pe_a0, __pe_a1, __pe_b2.clone()) }))?;
    Ok(states)
}

fn statePriorityGt(mut cr1: Arc<ComponentRef::NFComponentRef>, mut cr2: Arc<ComponentRef::NFComponentRef>, mut initCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut gt: bool = false;
    if ComponentRef::isEqual(cr2.clone(), initCref.clone())? {
        gt = true;
    } else if ComponentRef::isEqual(cr1.clone(), initCref.clone())? {
        gt = false;
    } else {
        gt = ComponentRef::toString(cr1.clone())? > ComponentRef::toString(cr2.clone())?;
    }
    Ok(gt)
}

// ============================================================
// Flat SM to data-flow transformation
// ============================================================
fn smGroupDepthLt(mut g1: (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>), mut g2: (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)) -> bool {
    let mut lt: bool = false;
    let mut c1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut c2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    (c1, _) = g1.clone();
    (c2, _) = g2.clone();
    lt = ComponentRef::depth(c1.clone()) < ComponentRef::depth(c2.clone());
    lt
}

fn flatSmToDataFlow(mut initStateCref: Arc<ComponentRef::NFComponentRef>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut allEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut allVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>, mut enclosingSmSemOpt: Option<FlatSmSemantics>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>, FlatSmSemantics)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut outSem: FlatSmSemantics;
    let mut transitionEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut initialStateEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut stateEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut sem: FlatSmSemantics;
    let mut semWithProp: FlatSmSemantics;
    let mut semFinal: FlatSmSemantics;
    let mut parentPrefix: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut varCrefStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    transitionEqs = List::filterOnTrue(allEquations.clone(), Arc::new({ let __pe_b1 = stateCrefs.clone(); move |__pe_a0| isTransitionForGroup(__pe_a0, __pe_b1.clone()) }));
    initialStateEqs = List::filterOnTrue(allEquations.clone(), Arc::new({ let __pe_b1 = initStateCref.clone(); move |__pe_a0| isInitialStateForGroup(__pe_a0, __pe_b1.clone()) }));
    sem = basicFlatSmSemantics(initStateCref.clone(), stateCrefs.clone(), transitionEqs.clone())?;
    semWithProp = addPropagationEquations(sem.clone(), enclosingStateCrefOpt.clone(), enclosingSmSemOpt.clone())?;
    semFinal = elabXInStateOps(semWithProp.clone(), enclosingStateCrefOpt.clone())?;
    parentPrefix = ComponentRef::rest(listHead(stateCrefs.clone())?)?;
    if !(ComponentRef::isEmpty(parentPrefix.clone())) {
        varCrefStrings = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (allVariables.clone()).into_iter().cloned() {
            let __x = ComponentRef::toString(v.name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        semFinal.eqs = List::map(semFinal.eqs.clone(), Arc::new({ let __pe_b1 = { let __pe_b1 = parentPrefix.clone(); let __pe_b2 = varCrefStrings.clone(); move |__pe_a0| qualifyOuterVarExpr(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }; move |__pe_a0| Ok(Equation::mapExp(__pe_a0, __pe_b1.clone())) }));
    }
    accVars = List::flatten(list![accVars.clone(), semFinal.vars.clone(), semFinal.knowns.clone(), semFinal.pvars.clone()]);
    accEqs = List::flatten(list![accEqs.clone(), semFinal.eqs.clone(), semFinal.peqs.clone()]);
    for mut stateCref in &*stateCrefs.clone() {
        let mut stateCref = stateCref.clone();
        (accEqs, accVars) = smCompToDataFlow(stateCref.clone(), semFinal.clone(), allEquations.clone(), allVariables.clone(), accEqs.clone(), accVars.clone(), outerVarMap.clone())?;
    }
    outSem = semFinal.clone();
    Ok((accEqs, accVars, outSem))
}

fn qualifyOuterVarExpr(mut e: Arc<Expression::NFExpression>, mut parentPrefix: Arc<ComponentRef::NFComponentRef>, mut varCrefStrings: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Expression::NFExpression>> {
    let mut e: Arc<Expression::NFExpression> = e;
    e = Expression::map(e.clone(), Arc::new({ let __pe_b1 = parentPrefix.clone(); let __pe_b2 = varCrefStrings.clone(); move |__pe_a0| qualifyOuterVarCref(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    Ok(e)
}

fn qualifyOuterVarCref(mut e: Arc<Expression::NFExpression>, mut parentPrefix: Arc<ComponentRef::NFComponentRef>, mut varCrefStrings: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Expression::NFExpression>> {
    let mut e: Arc<Expression::NFExpression> = e;
    let mut qualCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isSimple(var_field!((*e).cref, Expression::NFExpression::CREF).clone())) => {
            qualCref = ComponentRef::append(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), parentPrefix.clone())?;
            if listMember((ComponentRef::toString(qualCref.clone())?).clone(), varCrefStrings.clone()) {
                e = Arc::new(Expression::NFExpression::CREF { ty: var_field!((*e).ty, Expression::NFExpression::CREF).clone(), cref: qualCref.clone() });
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(e)
}

// ============================================================
// State machine component to data-flow
// ============================================================
fn smCompToDataFlow(mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut allEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut allVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut stateEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut stateVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut transformedEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut extraVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    stateEqs = List::filterOnTrue(allEquations.clone(), Arc::new({ let __pe_b1 = stateCref.clone(); move |__pe_a0| isEquationOfState(__pe_a0, __pe_b1.clone()) }));
    stateVars = List::filterOnTrue(allVariables.clone(), Arc::new({ let __pe_b1 = stateCref.clone(); move |__pe_a0| isVariableOfState(__pe_a0, __pe_b1.clone()) }));
    crToStart = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
    for mut v in &*stateVars.clone() {
        let mut v = v.clone();
        if List::any(stateEqs.clone(), Arc::new({ let __pe_b1 = v.name.clone(); move |__pe_a0| equationHasPrevious(__pe_a0, __pe_b1.clone()) })) {
            UnorderedMap::addUnique(v.name.clone(), getStartValue(v.clone())?, crToStart.clone())?;
        }
    }
    transformedEqs = metamodelica::nil();
    extraVars = metamodelica::nil();
    for mut eq in &*stateEqs.clone() {
        let mut eq = eq.clone();
        (transformedEqs, extraVars) = addStateActivationAndReset(eq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), transformedEqs.clone(), extraVars.clone(), outerVarMap.clone())?;
    }
    accEqs = listAppend(transformedEqs.clone().reverse(), accEqs.clone());
    accVars = listAppend(extraVars.clone().reverse(), accVars.clone());
    addHierarchicalPassThroughs(stateCref.clone(), sem.clone(), allVariables.clone(), outerVarMap.clone())?;
    Ok((accEqs, accVars))
}

fn addHierarchicalPassThroughs(mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut allVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<()> {
    let mut stateStr: ArcStr = arcstr::literal!("");
    let mut leafName: ArcStr = arcstr::literal!("");
    let mut activeRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut topVarCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut topVar: Arc<Variable::NFVariable>;
    stateStr = (ComponentRef::toString(stateCref.clone())?).clone();
    activeRef = qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), stateCref.clone())?;
    for mut v in &*allVariables.clone() {
        let mut v = v.clone();
        if !(ComponentRef::isSimple(v.name.clone())) && stringEqual((ComponentRef::toString(ComponentRef::rest(v.name.clone())?)?).clone(), (stateStr.clone()).clone()) {
            leafName = (ComponentRef::firstName(v.name.clone(), false)?).clone();
            if '__try0: {
                topVar = unwrap_break_err!(List::find(allVariables.clone(), Arc::new({ let __pe_b1 = (leafName.clone()).clone(); move |__pe_a0| isSimpleVarNamed(__pe_a0, __pe_b1.clone()) })), '__try0);
                topVarCref = topVar.name.clone();
                if !(UnorderedMap::contains(topVarCref.clone(), outerVarMap.clone())) {
                    unwrap_break_err!(UnorderedMap::add(topVarCref.clone(), list![(activeRef.clone(), v.name.clone())], outerVarMap.clone()), '__try0);
                }
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
    }
    Ok(())
}

fn isSimpleVarNamed(mut v: Arc<Variable::NFVariable>, mut name: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    res = ComponentRef::isSimple(v.name.clone()) && stringEqual((ComponentRef::firstName(v.name.clone(), false)?).clone(), (name.clone()).clone());
    Ok(res)
}

// ============================================================
// addStateActivationAndReset
// ============================================================
fn addStateActivationAndReset(mut inEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let () = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            (accEqs, accVars) = addStateActivationAndReset1(inEq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), accEqs.clone(), accVars.clone(), outerVarMap.clone())?;
            ()
        },
        Deref @ Equation::WHEN { .. } => {
            (accEqs, accVars) = transformWhenBranchesAndAccumulate(inEq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), outerVarMap.clone(), accEqs.clone(), accVars.clone())?;
            ()
        },
        _ => {
            accEqs = cons(inEq.clone(), accEqs.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((accEqs, accVars))
}

fn transformWhenBranchesAndAccumulate(mut whenEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut firstBranch: Arc<Equation::Branch::Branch>;
    let mut branchCond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outEq: Arc<Equation::NFEquation>;
    let mut extraVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut innerEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut innerVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(whenEq.clone()) {
        Deref @ Equation::WHEN { branches: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    branches = __pa0.clone();
    firstBranch = listHead(branches.clone())?;
    let __pa1 = ::match_deref::match_deref! { match &(firstBranch.clone()) {
        Deref @ Equation::Branch::BRANCH { condition: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    branchCond = __pa1.clone();
    if Type::isClock(Expression::typeOf(branchCond.clone())) {
        (innerEqs, innerVars) = transformWhenInnerAsPlain(whenEq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), outerVarMap.clone())?;
        accEqs = listAppend(innerEqs.clone(), accEqs.clone());
        accVars = listAppend(innerVars.clone(), accVars.clone());
    } else {
        (outEq, extraVars) = transformWhenBranches(whenEq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), outerVarMap.clone())?;
        accEqs = cons(outEq.clone(), accEqs.clone());
        accVars = listAppend(extraVars.clone(), accVars.clone());
    }
    Ok((accEqs, accVars))
}

fn transformWhenInnerAsPlain(mut whenEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut outEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut branchBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut transformedBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut branchVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(whenEq.clone()) {
        Deref @ Equation::WHEN { branches: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    branches = __pa0.clone();
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { body: branchBody, .. } => {
            transformedBody = metamodelica::nil();
            branchVars = metamodelica::nil();
            for mut eq in &*branchBody.clone() {
                let mut eq = eq.clone();
                (transformedBody, branchVars) = addStateActivationAndReset(eq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), transformedBody.clone(), branchVars.clone(), outerVarMap.clone())?;
            }
            outEqs = listAppend(transformedBody.clone().reverse(), outEqs.clone());
            outVars = listAppend(branchVars.clone(), outVars.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((outEqs, outVars))
}

fn transformWhenBranches(mut whenEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<Equation::NFEquation>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut outEq: Arc<Equation::NFEquation>;
    let mut extraVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut newBranches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut transformedBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut branchVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut whenScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut whenSource: Arc<DAE::ElementSource>;
    let mut branchCond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut branchCondVar: Variability = Variability::CONSTANT;
    let mut branchBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(whenEq.clone()) {
        Deref @ Equation::WHEN { source: __pa0, scope: __pa1, branches: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    whenSource = __pa0.clone();
    whenScope = __pa1.clone();
    branches = __pa2.clone();
    newBranches = metamodelica::nil();
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        branch = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { body: branchBody, conditionVar: branchCondVar, condition: branchCond } => {
            transformedBody = metamodelica::nil();
            branchVars = metamodelica::nil();
            for mut eq in &*branchBody.clone() {
                let mut eq = eq.clone();
                (transformedBody, branchVars) = addStateActivationAndReset(eq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), transformedBody.clone(), branchVars.clone(), outerVarMap.clone())?;
            }
            extraVars = listAppend(branchVars.clone(), extraVars.clone());
            Arc::new(Equation::Branch::Branch::BRANCH { condition: branchCond.clone(), conditionVar: branchCondVar.clone(), body: transformedBody.clone().reverse() })
        },
        _ => branch.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        newBranches = cons(branch.clone(), newBranches.clone());
    }
    outEq = Arc::new(Equation::NFEquation::WHEN { branches: newBranches.clone().reverse(), scope: whenScope.clone(), source: whenSource.clone() });
    Ok((outEq, extraVars))
}

fn addStateActivationAndReset1(mut inEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lhsCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut perStateVarCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut stateActiveCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut lhsTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut eqScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut eqSource: Arc<DAE::ElementSource>;
    let mut stateVarCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut hasStateVarOnLHS: bool = false;
    let mut isOuterOutput: bool = false;
    let mut newRhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut perStateVarExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eq1: Arc<Equation::NFEquation>;
    let mut eq2: Arc<Equation::NFEquation>;
    let mut perStateVar: Arc<Variable::NFVariable>;
    let mut prevList: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ Equation::EQUALITY { source: __pa0, scope: __pa1, ty: __pa2, rhs: __pa3, lhs: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqSource = __pa0.clone();
    eqScope = __pa1.clone();
    lhsTy = __pa2.clone();
    rhs = __pa3.clone();
    lhs = __pa4.clone();
    stateVarCrefs = UnorderedMap::keyList(crToStart.clone());
    match '__try5: {
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(lhs.clone()) {
            Deref @ Expression::CREF { cref: __pa6, ty: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => break '__try5 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        lhsCref = __pa6.clone();
        lhsTy = __pa7.clone();
        (newRhs, _) = unwrap_break_err!(Expression::mapFold(rhs.clone(), Arc::new({ let __pe_b1 = stateVarCrefs.clone(); move |__pe_a0, __pe_a2| Ok(subsPreviousCrefs(__pe_a0, __pe_b1.clone(), __pe_a2)) }), false), '__try5);
        eq1 = Arc::new(Equation::NFEquation::EQUALITY { lhs: lhs.clone(), rhs: newRhs.clone(), ty: lhsTy.clone(), scope: eqScope.clone(), source: eqSource.clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
        isOuterOutput = !(unwrap_break_err!(crefHasPrefix(stateCref.clone(), lhsCref.clone()), '__try5)) && stringEqual((InstNode::name(eqScope.clone())?).clone(), (ComponentRef::firstName(stateCref.clone(), false)?).clone());
        if isOuterOutput.clone() {
            perStateVarCref = ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: (ComponentRef::firstName(lhsCref.clone(), false)?).clone() }), lhsTy.clone(), metamodelica::nil(), stateCref.clone());
            perStateVar = makeVarWithStart(perStateVarCref.clone(), lhsTy.clone(), Variability::DISCRETE.clone(), getDefaultStart(lhsTy.clone()));
            perStateVarExp = makeCrefExp(perStateVarCref.clone(), lhsTy.clone());
            eq1 = Arc::new(Equation::NFEquation::EQUALITY { lhs: perStateVarExp.clone(), rhs: newRhs.clone(), ty: lhsTy.clone(), scope: eqScope.clone(), source: eqSource.clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
            eq1 = unwrap_break_err!(wrapInStateActivationConditional(eq1.clone(), stateCref.clone(), false), '__try5);
            accEqs = cons(eq1.clone(), accEqs.clone());
            accVars = cons(perStateVar.clone(), accVars.clone());
            stateActiveCref = unwrap_break_err!(qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), stateCref.clone()), '__try5);
            prevList = UnorderedMap::getOrDefault(lhsCref.clone(), outerVarMap.clone(), metamodelica::nil());
            unwrap_break_err!(UnorderedMap::add(lhsCref.clone(), cons((stateActiveCref.clone(), perStateVarCref.clone()), prevList.clone()), outerVarMap.clone()), '__try5);
        } else {
            hasStateVarOnLHS = false;
            for mut svc in &*stateVarCrefs.clone() {
                let mut svc = svc.clone();
                hasStateVarOnLHS = unwrap_break_err!(ComponentRef::isEqual(svc.clone(), lhsCref.clone()), '__try5);
                if hasStateVarOnLHS.clone() {
                    break;
                }
            }
            if hasStateVarOnLHS.clone() {
                eq1 = unwrap_break_err!(wrapInStateActivationConditional(eq1.clone(), stateCref.clone(), true), '__try5);
                eq2 = unwrap_break_err!(createResetEquation(lhsCref.clone(), lhsTy.clone(), stateCref.clone(), sem.clone(), crToStart.clone()), '__try5);
                accEqs = cons(eq1.clone(), cons(eq2.clone(), accEqs.clone()));
                accVars = cons(makeVar(ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::firstName(lhsCref.clone(), false)?); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) }).clone() }), lhsTy.clone(), metamodelica::nil(), ComponentRef::rest(lhsCref.clone())?), lhsTy.clone(), Variability::CONTINUOUS.clone()), accVars.clone());
            } else {
                accEqs = cons(unwrap_break_err!(wrapInStateActivationConditional(eq1.clone(), stateCref.clone(), false), '__try5), accEqs.clone());
            }
        }
        Ok::<_, anyhow::Error>((accEqs.clone(),))
    } {
        Ok((__try5_o0,)) => {
            accEqs = __try5_o0;
        }
        Err(_) => {
            accEqs = cons(inEq.clone(), accEqs.clone());
        }
    }
    Ok((accEqs, accVars))
}

fn equationHasPrevious(mut eq: Arc<Equation::NFEquation>, mut varCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut found: bool = false;
    found = Equation::containsExp(eq.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static> = Arc::new({ let __pe_b1 = varCref.clone(); move |__pe_a0| isPreviousOfCref(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Expression::contains(__pe_a0, __pe_b1.clone()) }))?;
    Ok(found)
}

fn isPreviousOfCref(mut e: Arc<Expression::NFExpression>, mut varCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    let mut expCall: Arc<Call::NFCall>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut argCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    res = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::CALL { call: expCall } if (stringEq((Call::functionNameLast(expCall.clone())).clone(), (literal!("previous")).clone())) => {
            args = Call::arguments(expCall.clone())?;
            res = false;
            if (args.clone().len() as i32) == 1 {
                res = (::match_deref::match_deref! { match &(listHead(args.clone())?) {
        Deref @ Expression::CREF { cref: argCref, .. } => ComponentRef::isEqual(argCref.clone(), varCref.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn getDefaultStart(mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }),
        Deref @ Type::BOOLEAN => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        Deref @ Type::STRING => Arc::new(Expression::NFExpression::STRING { value: (literal!("")).clone() }),
        _ => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

// ============================================================
// basicFlatSmSemantics
// ============================================================
fn basicFlatSmSemantics(mut initStateCref: Arc<ComponentRef::NFComponentRef>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut transitionEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<FlatSmSemantics> {
    let mut sem: FlatSmSemantics;
    let mut preRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut nStates: i32 = 0;
    let mut nTransitions: i32 = 0;
    let mut i: i32 = 0;
    let mut t: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut cExps: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut knowns: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut nStatesRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activeRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut resetRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut selectedStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut selectedResetRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut firedRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activeStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activeResetRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut nextStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut nextResetRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut stateMachineInFinalStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut tArrayBool: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tArrayInt: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut activeResetStatesRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut nextResetStatesRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut finalStatesRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cImmediateRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tTArrayBool: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tTArrayInt: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tFromRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tToRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tImmediateRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tResetRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tSynchronizeRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tPriorityRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expCond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expThen: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expElse: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expIf: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expLst: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut bindExp: Option<Arc<Expression::NFExpression>> = None;
    let mut immediateVal: bool = false;
    let mut tDim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut nStatesDim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    preRef = makeSMSPrefix(initStateCref.clone())?;
    (t, cExps) = createTandC(stateCrefs.clone(), transitionEqs.clone())?;
    nStates = (stateCrefs.clone().len() as i32);
    nTransitions = (t.clone().len() as i32);
    tDim = Arc::new(Dimension::NFDimension::INTEGER { size: nTransitions.clone(), var: Variability::STRUCTURAL_PARAMETER.clone() });
    nStatesDim = Arc::new(Dimension::NFDimension::INTEGER { size: nStates.clone(), var: Variability::STRUCTURAL_PARAMETER.clone() });
    tTArrayBool = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::BOOLEAN), dimensions: list![tDim.clone()] });
    tTArrayInt = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: list![tDim.clone()] });
    tArrayBool = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::BOOLEAN), dimensions: list![nStatesDim.clone()] });
    tArrayInt = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: list![nStatesDim.clone()] });
    nStatesRef = qCref((literal!("nState")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), preRef.clone())?;
    knowns = cons(makeVarWithBinding(nStatesRef.clone(), Arc::new(crate::NFType::INTEGER), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::INTEGER { value: nStates.clone() })), knowns.clone());
    i = 0;
    for mut tr in &*t.clone() {
        let mut tr = tr.clone();
        i = i.clone() + 1;
        tFromRefs = cons(qCref((literal!("tFrom")).clone(), tTArrayInt.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, tFromRefs.clone());
        knowns = cons(makeVarWithBinding(listHead(tFromRefs.clone())?, Arc::new(crate::NFType::INTEGER), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::INTEGER { value: tr.from.clone() })), knowns.clone());
        tToRefs = cons(qCref((literal!("tTo")).clone(), tTArrayInt.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, tToRefs.clone());
        knowns = cons(makeVarWithBinding(listHead(tToRefs.clone())?, Arc::new(crate::NFType::INTEGER), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::INTEGER { value: tr.to.clone() })), knowns.clone());
        tImmediateRefs = cons(qCref((literal!("tImmediate")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, tImmediateRefs.clone());
        knowns = cons(makeVarWithBinding(listHead(tImmediateRefs.clone())?, Arc::new(crate::NFType::BOOLEAN), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: tr.immediate.clone() })), knowns.clone());
        tResetRefs = cons(qCref((literal!("tReset")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, tResetRefs.clone());
        knowns = cons(makeVarWithBinding(listHead(tResetRefs.clone())?, Arc::new(crate::NFType::BOOLEAN), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: tr.reset.clone() })), knowns.clone());
        tSynchronizeRefs = cons(qCref((literal!("tSynchronize")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, tSynchronizeRefs.clone());
        knowns = cons(makeVarWithBinding(listHead(tSynchronizeRefs.clone())?, Arc::new(crate::NFType::BOOLEAN), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: tr.synchronize.clone() })), knowns.clone());
        tPriorityRefs = cons(qCref((literal!("tPriority")).clone(), tTArrayInt.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, tPriorityRefs.clone());
        knowns = cons(makeVarWithBinding(listHead(tPriorityRefs.clone())?, Arc::new(crate::NFType::INTEGER), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::INTEGER { value: tr.priority.clone() })), knowns.clone());
    }
    tFromRefs = tFromRefs.clone().reverse();
    tToRefs = tToRefs.clone().reverse();
    tImmediateRefs = tImmediateRefs.clone().reverse();
    tResetRefs = tResetRefs.clone().reverse();
    tSynchronizeRefs = tSynchronizeRefs.clone().reverse();
    tPriorityRefs = tPriorityRefs.clone().reverse();
    i = 0;
    for mut cExp in &*cExps.clone() {
        let mut cExp = cExp.clone();
        i = i.clone() + 1;
        cImmediateRefs = cons(qCref((literal!("cImmediate")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, cImmediateRefs.clone());
        cRefs = cons(qCref((literal!("c")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, cRefs.clone());
        vars = cons(makeVarWithStart(listHead(cImmediateRefs.clone())?, Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false })), vars.clone());
        vars = cons(makeVar(listHead(cRefs.clone())?, Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone()), vars.clone());
    }
    cImmediateRefs = cImmediateRefs.clone().reverse();
    cRefs = cRefs.clone().reverse();
    activeRef = qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVar(activeRef.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone()), vars.clone());
    resetRef = qCref((literal!("reset")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVar(resetRef.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone()), vars.clone());
    selectedStateRef = qCref((literal!("selectedState")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVar(selectedStateRef.clone(), Arc::new(crate::NFType::INTEGER), Variability::DISCRETE.clone()), vars.clone());
    selectedResetRef = qCref((literal!("selectedReset")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVar(selectedResetRef.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone()), vars.clone());
    firedRef = qCref((literal!("fired")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVar(firedRef.clone(), Arc::new(crate::NFType::INTEGER), Variability::DISCRETE.clone()), vars.clone());
    activeStateRef = qCref((literal!("activeState")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVar(activeStateRef.clone(), Arc::new(crate::NFType::INTEGER), Variability::DISCRETE.clone()), vars.clone());
    activeResetRef = qCref((literal!("activeReset")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVar(activeResetRef.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone()), vars.clone());
    nextStateRef = qCref((literal!("nextState")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVarWithStart(nextStateRef.clone(), Arc::new(crate::NFType::INTEGER), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 })), vars.clone());
    nextResetRef = qCref((literal!("nextReset")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVarWithStart(nextResetRef.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false })), vars.clone());
    for mut j in 1..=nStates.clone() {
        activeResetStatesRefs = cons(qCref((literal!("activeResetStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }) })], preRef.clone())?, activeResetStatesRefs.clone());
        vars = cons(makeVar(listHead(activeResetStatesRefs.clone())?, Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone()), vars.clone());
        nextResetStatesRefs = cons(qCref((literal!("nextResetStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }) })], preRef.clone())?, nextResetStatesRefs.clone());
        vars = cons(makeVarWithStart(listHead(nextResetStatesRefs.clone())?, Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false })), vars.clone());
        finalStatesRefs = cons(qCref((literal!("finalStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }) })], preRef.clone())?, finalStatesRefs.clone());
        vars = cons(makeVar(listHead(finalStatesRefs.clone())?, Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone()), vars.clone());
    }
    activeResetStatesRefs = activeResetStatesRefs.clone().reverse();
    nextResetStatesRefs = nextResetStatesRefs.clone().reverse();
    finalStatesRefs = finalStatesRefs.clone().reverse();
    stateMachineInFinalStateRef = qCref((literal!("stateMachineInFinalState")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    vars = cons(makeVar(stateMachineInFinalStateRef.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone()), vars.clone());
    i = 0;
    for mut cExp in &*cExps.clone() {
        let mut cExp = cExp.clone();
        i = i.clone() + 1;
        eqs = cons(makeEq(makeCrefExp((cImmediateRefs.clone()).get(i.clone())?, Arc::new(crate::NFType::BOOLEAN)), cExp.clone(), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
        let Transition { immediate: __pa0, .. } = ((t.clone()).get(i.clone())?) else { bail!("pattern mismatch") };
        immediateVal = __pa0.clone();
        rhs = if (immediateVal.clone()) {makeCrefExp((cImmediateRefs.clone()).get(i.clone())?, Arc::new(crate::NFType::BOOLEAN))} else {makePreviousCall(makeCrefExp((cImmediateRefs.clone()).get(i.clone())?, Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN))};
        eqs = cons(makeEq(makeCrefExp((cRefs.clone()).get(i.clone())?, Arc::new(crate::NFType::BOOLEAN)), rhs.clone(), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
    }
    eqs = cons(makeEq(makeCrefExp(selectedStateRef.clone(), Arc::new(crate::NFType::INTEGER)), makeIfExp(makeCrefExp(resetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::INTEGER { value: 1 }), makePreviousCall(makeCrefExp(nextStateRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER)), eqs.clone());
    eqs = cons(makeEq(makeCrefExp(selectedResetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), makeIfExp(makeCrefExp(resetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), makePreviousCall(makeCrefExp(nextResetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
    expLst = metamodelica::nil();
    for mut j in 1..=nTransitions.clone() {
        expCond = makeRelationEq(makeCrefExp((tFromRefs.clone()).get(j.clone())?, Arc::new(crate::NFType::INTEGER)), makeCrefExp(selectedStateRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER));
        expIf = makeIfExp(expCond.clone(), makeCrefExp((cRefs.clone()).get(j.clone())?, Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), Arc::new(crate::NFType::BOOLEAN));
        expLst = cons(makeIfExp(expIf.clone(), Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Arc::new(crate::NFType::INTEGER)), expLst.clone());
    }
    expLst = expLst.clone().reverse();
    rhs = if ((expLst.clone().len() as i32) > 1) {makeMaxIntArrCall(expLst.clone())} else if ((expLst.clone().len() as i32) == 1) {listHead(expLst.clone())?} else {Arc::new(Expression::NFExpression::INTEGER { value: 0 })};
    eqs = cons(makeEq(makeCrefExp(firedRef.clone(), Arc::new(crate::NFType::INTEGER)), rhs.clone(), Arc::new(crate::NFType::INTEGER)), eqs.clone());
    exp1 = makeRelationGt(makeCrefExp(firedRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Arc::new(crate::NFType::INTEGER));
    exp2 = makeCrefExp(qCref((literal!("tTo")).clone(), tTArrayInt.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: makeCrefExp(firedRef.clone(), Arc::new(crate::NFType::INTEGER)) })], preRef.clone())?, Arc::new(crate::NFType::INTEGER));
    expElse = makeIfExp(exp1.clone(), exp2.clone(), makeCrefExp(selectedStateRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER));
    eqs = cons(makeEq(makeCrefExp(activeStateRef.clone(), Arc::new(crate::NFType::INTEGER)), makeIfExp(makeCrefExp(resetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::INTEGER { value: 1 }), expElse.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER)), eqs.clone());
    exp1 = makeRelationGt(makeCrefExp(firedRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Arc::new(crate::NFType::INTEGER));
    exp2 = makeCrefExp(qCref((literal!("tReset")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: makeCrefExp(firedRef.clone(), Arc::new(crate::NFType::INTEGER)) })], preRef.clone())?, Arc::new(crate::NFType::BOOLEAN));
    expElse = makeIfExp(exp1.clone(), exp2.clone(), makeCrefExp(selectedResetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN));
    eqs = cons(makeEq(makeCrefExp(activeResetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), makeIfExp(makeCrefExp(resetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), expElse.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
    eqs = cons(makeEq(makeCrefExp(nextStateRef.clone(), Arc::new(crate::NFType::INTEGER)), makeIfExp(makeCrefExp(activeRef.clone(), Arc::new(crate::NFType::BOOLEAN)), makeCrefExp(activeStateRef.clone(), Arc::new(crate::NFType::INTEGER)), makePreviousCall(makeCrefExp(nextStateRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER)), eqs.clone());
    eqs = cons(makeEq(makeCrefExp(nextResetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), makeIfExp(makeCrefExp(activeRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), makePreviousCall(makeCrefExp(nextResetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
    for mut j in 1..=nStates.clone() {
        eqs = cons(makeEq(makeCrefExp((activeResetStatesRefs.clone()).get(j.clone())?, Arc::new(crate::NFType::BOOLEAN)), makeIfExp(makeCrefExp(resetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), makePreviousCall(makeCrefExp((nextResetStatesRefs.clone()).get(j.clone())?, Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
    }
    for mut j in 1..=nStates.clone() {
        exp1 = makeRelationEq(makeCrefExp(activeStateRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }), Arc::new(crate::NFType::INTEGER));
        expThen = makeIfExp(exp1.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), makeCrefExp((activeResetStatesRefs.clone()).get(j.clone())?, Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN));
        expElse = makePreviousCall(makeCrefExp((nextResetStatesRefs.clone()).get(j.clone())?, Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN));
        eqs = cons(makeEq(makeCrefExp((nextResetStatesRefs.clone()).get(j.clone())?, Arc::new(crate::NFType::BOOLEAN)), makeIfExp(makeCrefExp(activeRef.clone(), Arc::new(crate::NFType::BOOLEAN)), expThen.clone(), expElse.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
    }
    for mut j in 1..=nStates.clone() {
        expLst = metamodelica::nil();
        for mut k in 1..=nTransitions.clone() {
            expCond = makeRelationEq(makeCrefExp((tFromRefs.clone()).get(k.clone())?, Arc::new(crate::NFType::INTEGER)), Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }), Arc::new(crate::NFType::INTEGER));
            expLst = cons(makeIfExp(expCond.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Arc::new(crate::NFType::INTEGER)), expLst.clone());
        }
        expLst = expLst.clone().reverse();
        rhs = if ((expLst.clone().len() as i32) > 1) {makeRelationEq(makeMaxIntArrCall(expLst.clone()), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Arc::new(crate::NFType::INTEGER))} else if ((expLst.clone().len() as i32) == 1) {makeRelationEq(listHead(expLst.clone())?, Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Arc::new(crate::NFType::INTEGER))} else {Arc::new(Expression::NFExpression::BOOLEAN { value: true })};
        eqs = cons(makeEq(makeCrefExp((finalStatesRefs.clone()).get(j.clone())?, Arc::new(crate::NFType::BOOLEAN)), rhs.clone(), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
    }
    eqs = cons(makeEq(makeCrefExp(stateMachineInFinalStateRef.clone(), Arc::new(crate::NFType::BOOLEAN)), makeCrefExp(qCref((literal!("finalStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: makeCrefExp(activeStateRef.clone(), Arc::new(crate::NFType::INTEGER)) })], preRef.clone())?, Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), eqs.clone());
    sem = FlatSmSemantics { initStateRef: initStateCref.clone(), smComps: metamodelica::arrayFromVec(stateCrefs.clone().into_iter().cloned().collect()), t: t.clone(), c: cExps.clone(), vars: vars.clone(), knowns: knowns.clone(), eqs: eqs.clone(), pvars: metamodelica::nil(), peqs: metamodelica::nil(), enclosingState: None };
    Ok(sem)
}

// ============================================================
// addPropagationEquations
// ============================================================
fn addPropagationEquations(mut inSem: FlatSmSemantics, mut enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>, mut enclosingSmSemOpt: Option<FlatSmSemantics>) -> Result<FlatSmSemantics> {
    let mut outSem: FlatSmSemantics = inSem.clone();
    let mut preRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut initStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activeRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut resetRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut initRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut pvars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut peqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut nStates: i32 = 0;
    let mut posOfEnclosing: i32 = 0;
    let mut tArrayBool: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut enclosingStateCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut enclosingPreRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut enclosingActiveResetStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut enclosingActiveResetRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut enclosingActiveStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut enclosingInitStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut enclosingSem: FlatSmSemantics;
    let mut enclosingComps: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
    let mut stateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activePlotRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ticksRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut timeEnteredRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut timeInRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activePlotVar: Arc<Variable::NFVariable>;
    let mut ticksVar: Arc<Variable::NFVariable>;
    let mut timeEnteredVar: Arc<Variable::NFVariable>;
    let mut timeInVar: Arc<Variable::NFVariable>;
    let mut activePlotEq: Arc<Equation::NFEquation>;
    let mut ticksEq: Arc<Equation::NFEquation>;
    let mut timeEnteredEq: Arc<Equation::NFEquation>;
    let mut timeInEq: Arc<Equation::NFEquation>;
    initStateRef = inSem.initStateRef.clone();
    preRef = makeSMSPrefix(initStateRef.clone())?;
    activeRef = qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    resetRef = qCref((literal!("reset")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    nStates = (inSem.smComps.clone().borrow().len() as i32);
    tArrayBool = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::BOOLEAN), dimensions: list![Arc::new(Dimension::NFDimension::INTEGER { size: nStates.clone(), var: Variability::STRUCTURAL_PARAMETER.clone() })] });
    if isNone(enclosingSmSemOpt.clone()) {
        initRef = qCref((literal!("init")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
        pvars = cons(makeVarWithStart(initRef.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: true })), pvars.clone());
        peqs = cons(makeEq(makeCrefExp(initRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), Arc::new(crate::NFType::BOOLEAN)), peqs.clone());
        peqs = cons(makeEq(makeCrefExp(resetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), makePreviousCall(makeCrefExp(initRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), Arc::new(crate::NFType::BOOLEAN)), peqs.clone());
        peqs = cons(makeEq(makeCrefExp(activeRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), Arc::new(crate::NFType::BOOLEAN)), peqs.clone());
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(enclosingStateCrefOpt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        enclosingStateCref = __pa0.clone();
        let Some(__pa1) = (enclosingSmSemOpt.clone()) else { bail!("pattern mismatch") };
        enclosingSem = __pa1.clone();
        enclosingComps = enclosingSem.smComps.clone();
        enclosingInitStateRef = enclosingComps.clone().borrow()[(1-1) as usize].clone();
        enclosingPreRef = makeSMSPrefix(enclosingInitStateRef.clone())?;
        posOfEnclosing = 1;
        let __range2 = &*Arc::new(enclosingComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
        for mut sc in __range2 {
            let mut sc = sc.clone();
            if ComponentRef::isEqual(sc.clone(), enclosingStateCref.clone())? {
                break;
            }
            posOfEnclosing = posOfEnclosing.clone() + 1;
        }
        enclosingActiveStateRef = qCref((literal!("activeState")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), enclosingPreRef.clone())?;
        enclosingActiveResetRef = qCref((literal!("activeReset")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), enclosingPreRef.clone())?;
        enclosingActiveResetStateRef = qCref((literal!("activeResetStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: posOfEnclosing.clone() }) })], enclosingPreRef.clone())?;
        peqs = cons(makeEq(makeCrefExp(resetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::LBINARY { exp1: makeCrefExp(enclosingActiveResetStateRef.clone(), Arc::new(crate::NFType::BOOLEAN)), operator: Operator::makeOr(Arc::new(crate::NFType::BOOLEAN)), exp2: Arc::new(Expression::NFExpression::LBINARY { exp1: makeCrefExp(enclosingActiveResetRef.clone(), Arc::new(crate::NFType::BOOLEAN)), operator: Operator::makeAnd(Arc::new(crate::NFType::BOOLEAN)), exp2: makeRelationEq(makeCrefExp(enclosingActiveStateRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(Expression::NFExpression::INTEGER { value: posOfEnclosing.clone() }), Arc::new(crate::NFType::INTEGER)) }) }), Arc::new(crate::NFType::BOOLEAN)), peqs.clone());
        peqs = cons(makeEq(makeCrefExp(activeRef.clone(), Arc::new(crate::NFType::BOOLEAN)), makeRelationEq(makeCrefExp(enclosingActiveStateRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(Expression::NFExpression::INTEGER { value: posOfEnclosing.clone() }), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::BOOLEAN)), peqs.clone());
    }
    for mut j in 1..=nStates.clone() {
        stateRef = inSem.smComps.clone().borrow()[(j.clone()-1) as usize].clone();
        (activePlotVar, activePlotEq) = createActiveIndicator(stateRef.clone(), preRef.clone(), j.clone())?;
        pvars = cons(activePlotVar.clone(), pvars.clone());
        peqs = cons(activePlotEq.clone(), peqs.clone());
        activePlotRef = activePlotVar.name.clone();
        (ticksVar, ticksEq) = createTicksInStateIndicator(stateRef.clone(), activePlotRef.clone())?;
        pvars = cons(ticksVar.clone(), pvars.clone());
        peqs = cons(ticksEq.clone(), peqs.clone());
        (timeEnteredVar, timeEnteredEq) = createTimeEnteredStateIndicator(stateRef.clone(), activePlotRef.clone())?;
        (timeInVar, timeInEq) = createTimeInStateIndicator(stateRef.clone(), activePlotRef.clone(), timeEnteredVar.clone())?;
        pvars = cons(timeEnteredVar.clone(), cons(timeInVar.clone(), pvars.clone()));
        peqs = cons(timeEnteredEq.clone(), cons(timeInEq.clone(), peqs.clone()));
    }
    outSem.pvars = pvars.clone();
    outSem.peqs = peqs.clone();
    outSem.enclosingState = enclosingStateCrefOpt.clone();
    Ok(outSem)
}

// ============================================================
// elabXInStateOps
// ============================================================
fn elabXInStateOps(mut sem: FlatSmSemantics, mut enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>) -> Result<FlatSmSemantics> {
    let mut sem: FlatSmSemantics = sem;
    let mut tElab: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut cElab: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut eqsElab: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut stateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut substTickExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut substTimeExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c4: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut found: bool = false;
    let mut curT: Transition;
    let mut curFrom: i32 = 0;
    let mut curTo: i32 = 0;
    let mut curPriority: i32 = 0;
    let mut curImmediate: bool = false;
    let mut curReset: bool = false;
    let mut curSynchronize: bool = false;
    i = 0;
    for mut tc in &*List::zip(sem.t.clone(), sem.c.clone()) {
        let mut tc = tc.clone();
        i = i.clone() + 1;
        (_, c3) = tc.clone();
        curT = (sem.t.clone()).get(i.clone())?;
        let Transition { priority: __pa0, synchronize: __pa1, reset: __pa2, immediate: __pa3, to: __pa4, from: __pa5, .. } = (curT.clone()) else { bail!("pattern mismatch") };
        curPriority = __pa0.clone();
        curSynchronize = __pa1.clone();
        curReset = __pa2.clone();
        curImmediate = __pa3.clone();
        curTo = __pa4.clone();
        curFrom = __pa5.clone();
        stateRef = sem.smComps.clone().borrow()[(curFrom.clone()-1) as usize].clone();
        substTickExp = makeCrefExp(qCref((literal!("$ticksInState")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), stateRef.clone())?, Arc::new(crate::NFType::INTEGER));
        (c4, found) = subsXInState(c3.clone(), (literal!("ticksInState")).clone(), substTickExp.clone())?;
        if found.clone() && isSome(enclosingStateCrefOpt.clone()) {
            Error::addCompilerError((literal!("Found 'ticksInState()' within a state of a hierarchical state machine.")).clone())?;
            bail!("fail");
        }
        if found.clone() {
            sem.eqs = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (sem.eqs.clone()).into_iter().cloned() {
            let __x = smeqsSubsXInState(eq.clone(), sem.smComps.clone().borrow()[(1-1) as usize].clone(), i.clone(), (sem.t.clone().len() as i32), substTickExp.clone(), (literal!("ticksInState")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        }
        substTimeExp = makeCrefExp(qCref((literal!("$timeInState")).clone(), Arc::new(crate::NFType::REAL), metamodelica::nil(), stateRef.clone())?, Arc::new(crate::NFType::REAL));
        (c4, found) = subsXInState(c4.clone(), (literal!("timeInState")).clone(), substTimeExp.clone())?;
        if found.clone() && isSome(enclosingStateCrefOpt.clone()) {
            Error::addCompilerError((literal!("Found 'timeInState()' within a state of a hierarchical state machine.")).clone())?;
            bail!("fail");
        }
        if found.clone() {
            sem.eqs = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (sem.eqs.clone()).into_iter().cloned() {
            let __x = smeqsSubsXInState(eq.clone(), sem.smComps.clone().borrow()[(1-1) as usize].clone(), i.clone(), (sem.t.clone().len() as i32), substTimeExp.clone(), (literal!("timeInState")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        }
        tElab = cons(Transition { from: curFrom.clone(), to: curTo.clone(), condition: c4.clone(), immediate: curImmediate.clone(), reset: curReset.clone(), synchronize: curSynchronize.clone(), priority: curPriority.clone() }, tElab.clone());
        cElab = cons(c4.clone(), cElab.clone());
    }
    sem.t = tElab.clone().reverse();
    sem.c = cElab.clone().reverse();
    Ok(sem)
}

fn subsXInState(mut inExp: Arc<Expression::NFExpression>, mut funcName: ArcStr, mut substExp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut found: bool = false;
    (outExp, found) = Expression::mapFold(inExp.clone(), Arc::new({ let __pe_b1 = (funcName.clone()).clone(); let __pe_b2 = substExp.clone(); move |__pe_a0, __pe_a3| Ok(subsXInStateHelper(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }), false)?;
    Ok((outExp, found))
}

fn subsXInStateHelper(mut exp: Arc<Expression::NFExpression>, mut funcName: ArcStr, mut substExp: Arc<Expression::NFExpression>, mut found: bool) -> (Arc<Expression::NFExpression>, bool) {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut found: bool = found;
    let mut expCall: Arc<Call::NFCall>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        expCall = __pa1.clone();
        if !(stringEq((Call::functionNameLast(expCall.clone())).clone(), (funcName.clone()).clone())) {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        if !(unwrap_break_err!(Call::arguments(expCall.clone()), '__try0).is_empty()) {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        exp = substExp.clone();
        found = true;
        Ok::<_, anyhow::Error>((exp.clone(), expCall.clone(), found.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            exp = __try0_o0;
            expCall = __try0_o1;
            found = __try0_o2;
        }
        Err(_) => {
            panic!("try/else: outputs not set in else branch");
        }
    }
    (exp, found)
}

fn smeqsSubsXInState(mut eq: Arc<Equation::NFEquation>, mut initStateComp: Arc<ComponentRef::NFComponentRef>, mut i: i32, mut nTransitions: i32, mut substExp: Arc<Expression::NFExpression>, mut xInState: ArcStr) -> Result<Arc<Equation::NFEquation>> {
    let mut outEq: Arc<Equation::NFEquation> = eq.clone();
    let mut preRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut lhsRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut tArrayBool: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut newRhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut found: bool = false;
    outEq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            preRef = makeSMSPrefix(initStateComp.clone())?;
            tArrayBool = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::BOOLEAN), dimensions: list![Arc::new(Dimension::NFDimension::INTEGER { size: nTransitions.clone(), var: Variability::STRUCTURAL_PARAMETER.clone() })] });
            cRef = qCref((literal!("cImmediate")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone()) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            lhsRef = __pa0.clone();
            if ComponentRef::isEqual(cRef.clone(), lhsRef.clone())? {
                (newRhs, _) = subsXInState(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), (xInState.clone()).clone(), substExp.clone())?;
                outEq = Arc::new(Equation::NFEquation::EQUALITY { lhs: var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), rhs: newRhs.clone(), ty: var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone(), scope: var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
            }
            outEq.clone()
        },
        _ => eq.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEq)
}

// ============================================================
// State indicator helpers
// ============================================================
fn createActiveIndicator(mut stateRef: Arc<ComponentRef::NFComponentRef>, mut preRef: Arc<ComponentRef::NFComponentRef>, mut i: i32) -> Result<(Arc<Variable::NFVariable>, Arc<Equation::NFEquation>)> {
    let mut activePlotVar: Arc<Variable::NFVariable>;
    let mut eqn: Arc<Equation::NFEquation>;
    let mut activePlotRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activeRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activeStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut andExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eqExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    activePlotRef = qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), stateRef.clone())?;
    activePlotVar = makeVarWithStart(activePlotRef.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false }));
    activeRef = qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?;
    activeStateRef = qCref((literal!("activeState")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), preRef.clone())?;
    eqExp = makeRelationEq(makeCrefExp(activeStateRef.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }), Arc::new(crate::NFType::INTEGER));
    andExp = Arc::new(Expression::NFExpression::LBINARY { exp1: makeCrefExp(activeRef.clone(), Arc::new(crate::NFType::BOOLEAN)), operator: Operator::makeAnd(Arc::new(crate::NFType::BOOLEAN)), exp2: eqExp.clone() });
    eqn = makeEq(makeCrefExp(activePlotRef.clone(), Arc::new(crate::NFType::BOOLEAN)), andExp.clone(), Arc::new(crate::NFType::BOOLEAN));
    Ok((activePlotVar, eqn))
}

fn createTicksInStateIndicator(mut stateRef: Arc<ComponentRef::NFComponentRef>, mut stateActiveRef: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<Variable::NFVariable>, Arc<Equation::NFEquation>)> {
    let mut ticksVar: Arc<Variable::NFVariable>;
    let mut ticksEq: Arc<Equation::NFEquation>;
    let mut ticksRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ticksExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expCond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expThen: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expElse: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    ticksRef = qCref((literal!("$ticksInState")).clone(), Arc::new(crate::NFType::INTEGER), metamodelica::nil(), stateRef.clone())?;
    ticksVar = makeVarWithStart(ticksRef.clone(), Arc::new(crate::NFType::INTEGER), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }));
    ticksExp = makeCrefExp(ticksRef.clone(), Arc::new(crate::NFType::INTEGER));
    expCond = Arc::new(Expression::NFExpression::LUNARY { operator: Operator::makeNot(Arc::new(crate::NFType::BOOLEAN)), exp: makeCrefExp(stateActiveRef.clone(), Arc::new(crate::NFType::BOOLEAN)) });
    expThen = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    expElse = Arc::new(Expression::NFExpression::BINARY { exp1: makePreviousCall(ticksExp.clone(), Arc::new(crate::NFType::INTEGER)), operator: Operator::makeAdd(Arc::new(crate::NFType::INTEGER)), exp2: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
    ticksEq = makeEq(ticksExp.clone(), makeIfExp(expCond.clone(), expThen.clone(), expElse.clone(), Arc::new(crate::NFType::INTEGER)), Arc::new(crate::NFType::INTEGER));
    Ok((ticksVar, ticksEq))
}

fn createTimeEnteredStateIndicator(mut stateRef: Arc<ComponentRef::NFComponentRef>, mut stateActiveRef: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<Variable::NFVariable>, Arc<Equation::NFEquation>)> {
    let mut timeEnteredVar: Arc<Variable::NFVariable>;
    let mut timeEnteredEq: Arc<Equation::NFEquation>;
    let mut timeEnteredRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut timeEnteredExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expCond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expThen: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expElse: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut activeExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    timeEnteredRef = qCref((literal!("$timeEnteredState")).clone(), Arc::new(crate::NFType::REAL), metamodelica::nil(), stateRef.clone())?;
    timeEnteredVar = makeVarWithStart(timeEnteredRef.clone(), Arc::new(crate::NFType::REAL), Variability::CONTINUOUS.clone(), Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }));
    timeEnteredExp = makeCrefExp(timeEnteredRef.clone(), Arc::new(crate::NFType::REAL));
    activeExp = makeCrefExp(stateActiveRef.clone(), Arc::new(crate::NFType::BOOLEAN));
    expCond = Arc::new(Expression::NFExpression::LBINARY { exp1: makeRelationEq(makePreviousCall(activeExp.clone(), Arc::new(crate::NFType::BOOLEAN)), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), Arc::new(crate::NFType::BOOLEAN)), operator: Operator::makeAnd(Arc::new(crate::NFType::BOOLEAN)), exp2: makeRelationEq(activeExp.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), Arc::new(crate::NFType::BOOLEAN)) });
    expThen = makeSampleTimeCall();
    expElse = makePreviousCall(timeEnteredExp.clone(), Arc::new(crate::NFType::REAL));
    timeEnteredEq = makeEq(timeEnteredExp.clone(), makeIfExp(expCond.clone(), expThen.clone(), expElse.clone(), Arc::new(crate::NFType::REAL)), Arc::new(crate::NFType::REAL));
    Ok((timeEnteredVar, timeEnteredEq))
}

fn createTimeInStateIndicator(mut stateRef: Arc<ComponentRef::NFComponentRef>, mut stateActiveRef: Arc<ComponentRef::NFComponentRef>, mut timeEnteredVar: Arc<Variable::NFVariable>) -> Result<(Arc<Variable::NFVariable>, Arc<Equation::NFEquation>)> {
    let mut timeInVar: Arc<Variable::NFVariable>;
    let mut timeInEq: Arc<Equation::NFEquation>;
    let mut timeInRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut timeInExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expCond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expThen: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expElse: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut timeEnteredExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    timeInRef = qCref((literal!("$timeInState")).clone(), Arc::new(crate::NFType::REAL), metamodelica::nil(), stateRef.clone())?;
    timeInVar = makeVarWithStart(timeInRef.clone(), Arc::new(crate::NFType::REAL), Variability::CONTINUOUS.clone(), Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }));
    timeInExp = makeCrefExp(timeInRef.clone(), Arc::new(crate::NFType::REAL));
    timeEnteredExp = makeCrefExp(timeEnteredVar.name.clone(), Arc::new(crate::NFType::REAL));
    expCond = makeCrefExp(stateActiveRef.clone(), Arc::new(crate::NFType::BOOLEAN));
    expThen = Arc::new(Expression::NFExpression::BINARY { exp1: makeSampleTimeCall(), operator: Operator::makeSub(Arc::new(crate::NFType::REAL)), exp2: timeEnteredExp.clone() });
    expElse = Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) });
    timeInEq = makeEq(timeInExp.clone(), makeIfExp(expCond.clone(), expThen.clone(), expElse.clone(), Arc::new(crate::NFType::REAL)), Arc::new(crate::NFType::REAL));
    Ok((timeInVar, timeInEq))
}

// ============================================================
// Reset and activation wrapping
// ============================================================
fn wrapInStateActivationConditional(mut inEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut isResetEquation: bool) -> Result<Arc<Equation::NFEquation>> {
    let mut outEq: Arc<Equation::NFEquation>;
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut activeRef: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expElse: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lhsCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut eqScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut eqSource: Arc<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ Equation::EQUALITY { source: __pa0, scope: __pa1, ty: __pa2, rhs: __pa3, lhs: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqSource = __pa0.clone();
    eqScope = __pa1.clone();
    ty = __pa2.clone();
    rhs = __pa3.clone();
    lhs = __pa4.clone();
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ Expression::CREF { cref: __pa5, ty: __pa6 } => (__pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhsCref = __pa5.clone();
    ty = __pa6.clone();
    activeRef = makeCrefExp(qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), stateCref.clone())?, Arc::new(crate::NFType::BOOLEAN));
    if isResetEquation.clone() {
        expElse = makeCrefExp(ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::firstName(lhsCref.clone(), false)?); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) }).clone() }), ty.clone(), metamodelica::nil(), ComponentRef::rest(lhsCref.clone())?), ty.clone());
    } else {
        expElse = makePreviousCall(lhs.clone(), ty.clone());
    }
    outEq = Arc::new(Equation::NFEquation::EQUALITY { lhs: lhs.clone(), rhs: makeIfExp(activeRef.clone(), rhs.clone(), expElse.clone(), ty.clone()), ty: ty.clone(), scope: eqScope.clone(), source: eqSource.clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
    Ok(outEq)
}

fn createResetEquation(mut lhsCref: Arc<ComponentRef::NFComponentRef>, mut lhsTy: Arc<Type::NFType>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<Equation::NFEquation>> {
    let mut outEq: Arc<Equation::NFEquation>;
    let mut preRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut initStateRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut activeExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut activeResetExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut activeResetStatesExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut orExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut andExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut prevExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut startExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ifExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lhsPrevExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut i: i32 = 0;
    let mut nStates: i32 = 0;
    let mut tArrayBool: Arc<Type::NFType> = Arc::new(Type::ANY);
    initStateRef = sem.smComps.clone().borrow()[(1-1) as usize].clone();
    preRef = makeSMSPrefix(initStateRef.clone())?;
    i = 1;
    let __range0 = &*Arc::new(sem.smComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    for mut sc in __range0 {
        let mut sc = sc.clone();
        if ComponentRef::isEqual(sc.clone(), stateCref.clone())? {
            break;
        }
        i = i.clone() + 1;
    }
    nStates = (sem.smComps.clone().borrow().len() as i32);
    tArrayBool = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::BOOLEAN), dimensions: list![Arc::new(Dimension::NFDimension::INTEGER { size: nStates.clone(), var: Variability::STRUCTURAL_PARAMETER.clone() })] });
    activeResetExp = makeCrefExp(qCref((literal!("activeReset")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), preRef.clone())?, Arc::new(crate::NFType::BOOLEAN));
    activeResetStatesExp = makeCrefExp(qCref((literal!("activeResetStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) })], preRef.clone())?, Arc::new(crate::NFType::BOOLEAN));
    orExp = Arc::new(Expression::NFExpression::LBINARY { exp1: activeResetExp.clone(), operator: Operator::makeOr(Arc::new(crate::NFType::BOOLEAN)), exp2: activeResetStatesExp.clone() });
    activeExp = makeCrefExp(qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), stateCref.clone())?, Arc::new(crate::NFType::BOOLEAN));
    andExp = Arc::new(Expression::NFExpression::LBINARY { exp1: activeExp.clone(), operator: Operator::makeAnd(Arc::new(crate::NFType::BOOLEAN)), exp2: orExp.clone() });
    prevExp = makePreviousCall(makeCrefExp(lhsCref.clone(), lhsTy.clone()), lhsTy.clone());
    startExp = UnorderedMap::getOrDefault(lhsCref.clone(), crToStart.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }));
    ifExp = makeIfExp(andExp.clone(), startExp.clone(), prevExp.clone(), lhsTy.clone());
    lhsPrevExp = makeCrefExp(ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::firstName(lhsCref.clone(), false)?); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) }).clone() }), lhsTy.clone(), metamodelica::nil(), ComponentRef::rest(lhsCref.clone())?), lhsTy.clone());
    outEq = makeEq(lhsPrevExp.clone(), ifExp.clone(), lhsTy.clone());
    Ok(outEq)
}

// ============================================================
// Expression substitution helpers
// ============================================================
fn subsActiveStateInEq(mut eq: Arc<Equation::NFEquation>) -> Arc<Equation::NFEquation> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    eq = Equation::mapExp(eq.clone(), subsActiveStateInExp);
    eq
}

fn subsActiveStateInExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), Arc::new(fnptr!(subsActiveStateHelper, Arc<Expression::NFExpression>)))?;
    Ok(exp)
}

fn subsActiveStateHelper(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut expCall: Arc<Call::NFCall>;
    let mut argCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut newExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        expCall = __pa1.clone();
        if !(stringEq((Call::functionNameLast(expCall.clone())).clone(), (literal!("activeState")).clone())) {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(Call::arguments(expCall.clone()), '__try0)) {
            Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa2, .. }, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        argCref = __pa2.clone();
        newExp = makeCrefExp(qCref((literal!("active")).clone(), Arc::new(crate::NFType::BOOLEAN), metamodelica::nil(), argCref.clone()).unwrap(), Arc::new(crate::NFType::BOOLEAN));
        exp = newExp.clone();
        Ok::<_, anyhow::Error>((argCref.clone(), exp.clone(), expCall.clone(), newExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            argCref = __try0_o0;
            exp = __try0_o1;
            expCall = __try0_o2;
            newExp = __try0_o3;
        }
        Err(_) => {
            panic!("try/else: outputs not set in else branch");
        }
    }
    exp
}

fn subsPreviousCrefs(mut exp: Arc<Expression::NFExpression>, mut stateVarCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut found: bool) -> (Arc<Expression::NFExpression>, bool) {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut found: bool = found;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut argTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut argCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut expCall: Arc<Call::NFCall>;
    let mut newExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        expCall = __pa1.clone();
        if !(stringEq((Call::functionNameLast(expCall.clone())).clone(), (literal!("previous")).clone())) {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        args = unwrap_break_err!(Call::arguments(expCall.clone()), '__try0);
        if (args.clone().len() as i32) != 1 {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        arg1 = unwrap_break_err!(listHead(args.clone()), '__try0);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(arg1.clone()) {
            Deref @ Expression::CREF { cref: __pa2, ty: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        argCref = __pa2.clone();
        argTy = __pa3.clone();
        for mut svc in &*stateVarCrefs.clone() {
            let mut svc = svc.clone();
            if unwrap_break_err!(ComponentRef::isEqual(svc.clone(), argCref.clone()), '__try0) {
                newExp = makeCrefExp(ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::firstName(argCref.clone(), false).unwrap()); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) }).clone() }), argTy.clone(), metamodelica::nil(), ComponentRef::rest(argCref.clone()).unwrap()), argTy.clone());
                exp = newExp.clone();
                found = true;
                break;
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    (exp, found)
}

// ============================================================
// createTandC
// ============================================================
fn createTandC(mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut transitionEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<(Arc<metamodelica::List<Transition>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>)> {
    let mut t: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut c: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut transitions: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    transitions = List::filterMap(transitionEqs.clone(), Arc::new({ let __pe_b1 = stateCrefs.clone(); move |__pe_a0| extractTransition(__pe_a0, __pe_b1.clone()) }));
    t = List::sort(transitions.clone(), Arc::new(fnptr!(priorityGt, Transition, Transition)))?;
    c = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut tr in (t.clone()).into_iter().cloned() {
            let __x = tr.condition.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((t, c))
}

fn extractTransition(mut eq: Arc<Equation::NFEquation>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Transition> {
    let mut trans: Transition;
    let mut crFrom: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut crTo: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut imm: bool = true;
    let mut rst: bool = true;
    let mut syn: bool = false;
    let mut prio: i32 = 1;
    let mut from: i32 = 0;
    let mut to: i32 = 0;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut eqCall: Arc<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: __pa0 }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqCall = __pa0.clone();
    if !(stringEq((Call::functionNameLast(eqCall.clone())).clone(), (literal!("transition")).clone())) {
        bail!("fail");
    }
    args = Call::arguments(eqCall.clone())?;
    let __pa2 = ::match_deref::match_deref! { match &((args.clone()).get(1)?) {
        Deref @ Expression::CREF { cref: __pa2, .. } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crFrom = __pa2.clone();
    let __pa3 = ::match_deref::match_deref! { match &((args.clone()).get(2)?) {
        Deref @ Expression::CREF { cref: __pa3, .. } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crTo = __pa3.clone();
    cond = (args.clone()).get(3)?;
    if (args.clone().len() as i32) >= 4 {
        let __pa4 = ::match_deref::match_deref! { match &((args.clone()).get(4)?) {
            Deref @ Expression::BOOLEAN { value: __pa4 } => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        imm = __pa4.clone();
    }
    if (args.clone().len() as i32) >= 5 {
        let __pa5 = ::match_deref::match_deref! { match &((args.clone()).get(5)?) {
            Deref @ Expression::BOOLEAN { value: __pa5 } => __pa5.clone(),
            _ => bail!("pattern mismatch"),
        } };
        rst = __pa5.clone();
    }
    if (args.clone().len() as i32) >= 6 {
        let __pa6 = ::match_deref::match_deref! { match &((args.clone()).get(6)?) {
            Deref @ Expression::BOOLEAN { value: __pa6 } => __pa6.clone(),
            _ => bail!("pattern mismatch"),
        } };
        syn = __pa6.clone();
    }
    if (args.clone().len() as i32) >= 7 {
        let __pa7 = ::match_deref::match_deref! { match &((args.clone()).get(7)?) {
            Deref @ Expression::INTEGER { value: __pa7 } => __pa7.clone(),
            _ => bail!("pattern mismatch"),
        } };
        prio = __pa7.clone();
    }
    from = 1;
    for mut sc in &*stateCrefs.clone() {
        let mut sc = sc.clone();
        if ComponentRef::isEqual(sc.clone(), crFrom.clone())? {
            break;
        }
        from = from.clone() + 1;
    }
    to = 1;
    for mut sc in &*stateCrefs.clone() {
        let mut sc = sc.clone();
        if ComponentRef::isEqual(sc.clone(), crTo.clone())? {
            break;
        }
        to = to.clone() + 1;
    }
    trans = Transition { from: from.clone(), to: to.clone(), condition: cond.clone(), immediate: imm.clone(), reset: rst.clone(), synchronize: syn.clone(), priority: prio.clone() };
    Ok(trans)
}

fn priorityGt(mut t1: Transition, mut t2: Transition) -> bool {
    let mut gt: bool = false;
    gt = t1.priority.clone() > t2.priority.clone();
    gt
}

// ============================================================
// Predicate helpers
// ============================================================
fn isTransitionOrInitialState(mut eq: Arc<Equation::NFEquation>) -> bool {
    let mut res: bool = false;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } => {
            res = (::match_deref::match_deref! { match &(Call::functionNameLast(eqCall.clone())) {
        Deref @ "transition" => true,
        Deref @ "initialState" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn isTransitionForGroup(mut eq: Arc<Equation::NFEquation>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
    let mut res: bool = false;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } if (stringEq((Call::functionNameLast(eqCall.clone())).clone(), (literal!("transition")).clone())) => {
            let __pa0 = ::match_deref::match_deref! { match &(listHead(Call::arguments(eqCall.clone())?)?) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            for mut sc in &*stateCrefs.clone() {
                let mut sc = sc.clone();
                if ComponentRef::isEqual(cr.clone(), sc.clone())? {
                    res = true;
                    break;
                }
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isInitialStateForGroup(mut eq: Arc<Equation::NFEquation>, mut initStateCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } if (stringEq((Call::functionNameLast(eqCall.clone())).clone(), (literal!("initialState")).clone())) => {
            let __pa0 = ::match_deref::match_deref! { match &(listHead(Call::arguments(eqCall.clone())?)?) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            res = ComponentRef::isEqual(cr.clone(), initStateCref.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isEquationOfState(mut eq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    let mut eqScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut stateName: ArcStr = arcstr::literal!("");
    stateName = (ComponentRef::firstName(stateCref.clone(), false)?).clone();
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { scope: eqScope, .. } => {
            res = stringEqual((InstNode::name(eqScope.clone())?).clone(), (stateName.clone()).clone());
            ()
        },
        Deref @ Equation::WHEN { scope: eqScope, .. } => {
            res = stringEqual((InstNode::name(eqScope.clone())?).clone(), (stateName.clone()).clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isVariableOfState(mut var: Arc<Variable::NFVariable>, mut stateCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    res = crefHasPrefix(stateCref.clone(), var.name.clone())?;
    Ok(res)
}

fn isOuterStateEquation(mut eq: Arc<Equation::NFEquation>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
    let mut res: bool = false;
    let mut eqScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut scopeName: ArcStr = arcstr::literal!("");
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { scope: eqScope, .. } => {
            scopeName = (InstNode::name(eqScope.clone())?).clone();
            for mut stateCref in &*stateCrefs.clone() {
                let mut stateCref = stateCref.clone();
                if stringEqual((scopeName.clone()).clone(), (ComponentRef::firstName(stateCref.clone(), false)?).clone()) {
                    res = true;
                    return Ok(res);
                }
            }
            ()
        },
        Deref @ Equation::WHEN { scope: eqScope, .. } => {
            scopeName = (InstNode::name(eqScope.clone())?).clone();
            for mut stateCref in &*stateCrefs.clone() {
                let mut stateCref = stateCref.clone();
                if stringEqual((scopeName.clone()).clone(), (ComponentRef::firstName(stateCref.clone(), false)?).clone()) {
                    res = true;
                    return Ok(res);
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn generateMergeEquation(mut outerVarCref: Arc<ComponentRef::NFComponentRef>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>, mut allVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut stateEntries: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>> = metamodelica::nil();
    let mut mergeRhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outerVarExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut activeRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut perStateVarRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut src: Arc<DAE::ElementSource>;
    stateEntries = UnorderedMap::getOrDefault(outerVarCref.clone(), outerVarMap.clone(), metamodelica::nil());
    if stateEntries.clone().is_empty() {
        return Ok((accEqs, accVars));
    }
    ty = Arc::new(crate::NFType::INTEGER);
    for mut v in &*allVariables.clone() {
        let mut v = v.clone();
        if ComponentRef::isEqual(v.name.clone(), outerVarCref.clone())? {
            ty = v.ty.clone();
            break;
        }
    }
    outerVarExp = makeCrefExp(outerVarCref.clone(), ty.clone());
    mergeRhs = makePreviousCall(outerVarExp.clone(), ty.clone());
    for mut entry in &*stateEntries.clone() {
        let mut entry = entry.clone();
        (activeRef, perStateVarRef) = entry.clone();
        mergeRhs = makeIfExp(makeCrefExp(activeRef.clone(), Arc::new(crate::NFType::BOOLEAN)), makeCrefExp(perStateVarRef.clone(), ty.clone()), mergeRhs.clone(), ty.clone());
    }
    src = ElementSource::createElementSource(Absyn::dummyInfo.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
    accEqs = cons(Arc::new(Equation::NFEquation::EQUALITY { lhs: outerVarExp.clone(), rhs: mergeRhs.clone(), ty: ty.clone(), scope: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), source: src.clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() }), accEqs.clone());
    Ok((accEqs, accVars))
}

// ============================================================
// ComponentRef utilities
// ============================================================
fn qCref(mut name: ArcStr, mut ty: Arc<Type::NFType>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut prefixCr: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    cref = ComponentRef::fromNode(Arc::new(InstNode::InstNode::NAME_NODE { name: (name.clone()).clone() }), ty.clone(), subs.clone(), ComponentRef::Origin::CREF.clone());
    cref = ComponentRef::prepend(prefixCr.clone(), cref.clone())?;
    Ok(cref)
}

fn makeSMSPrefix(mut initStateCref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut preRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    preRef = ComponentRef::fromNode(Arc::new(InstNode::InstNode::NAME_NODE { name: (arcstr::literal!(SMS_PRE)).clone() }), Arc::new(crate::NFType::UNKNOWN), metamodelica::nil(), ComponentRef::Origin::CREF.clone());
    preRef = ComponentRef::append(initStateCref.clone(), preRef.clone())?;
    Ok(preRef)
}

// ============================================================
// Variable creation helpers
// ============================================================
fn makeVar(mut name: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut var: Variability) -> Arc<Variable::NFVariable> {
    let mut v: Arc<Variable::NFVariable>;
    let mut attr: Arc<Attributes::NFAttributes>;
    attr = Attributes::DEFAULT_ATTR().clone();
    assign_field!(attr.variability = var.clone());
    v = Arc::new(Variable::NFVariable { name: name.clone(), ty: ty.clone(), binding: Binding::EMPTY_BINDING().clone(), visibility: Visibility::PUBLIC.clone(), attributes: attr.clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: Arc::new(SCode::Comment { annotation_: None, comment: None }), info: Absyn::dummyInfo.clone(), backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone() });
    v
}

fn makeVarWithStart(mut name: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut var: Variability, mut startExp: Arc<Expression::NFExpression>) -> Arc<Variable::NFVariable> {
    let mut v: Arc<Variable::NFVariable>;
    v = makeVar(name.clone(), ty.clone(), var.clone());
    assign_field!(v.typeAttributes = list![(literal!("start"), Arc::new(Binding::NFBinding::FLAT_BINDING { bindingExp: startExp.clone(), variability: Variability::CONSTANT.clone(), source: Binding::Source::GENERATED.clone() })), (literal!("fixed"), Arc::new(Binding::NFBinding::FLAT_BINDING { bindingExp: Arc::new(Expression::NFExpression::BOOLEAN { value: true }), variability: Variability::CONSTANT.clone(), source: Binding::Source::GENERATED.clone() }))]);
    v
}

fn makeVarWithBinding(mut name: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut var: Variability, mut bindExp: Arc<Expression::NFExpression>) -> Arc<Variable::NFVariable> {
    let mut v: Arc<Variable::NFVariable>;
    v = makeVar(name.clone(), ty.clone(), var.clone());
    assign_field!(v.binding = Arc::new(Binding::NFBinding::FLAT_BINDING { bindingExp: bindExp.clone(), variability: var.clone(), source: Binding::Source::GENERATED.clone() }));
    v
}

// ============================================================
// Equation creation helpers
// ============================================================
fn makeEq(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Equation::NFEquation> {
    let mut eq: Arc<Equation::NFEquation>;
    eq = Arc::new(Equation::NFEquation::EQUALITY { lhs: lhs.clone(), rhs: rhs.clone(), ty: ty.clone(), scope: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), source: DAE::emptyElementSource.clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
    eq
}

// ============================================================
// Expression creation helpers
// ============================================================
fn makeCrefExp(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: cref.clone() });
    exp
}

fn makeIfExp(mut cond: Arc<Expression::NFExpression>, mut thenExp: Arc<Expression::NFExpression>, mut elseExp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: thenExp.clone(), falseBranch: elseExp.clone() });
    exp
}

fn makePreviousCall(mut exp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::PREVIOUS().clone(), list![exp.clone()], Variability::DISCRETE.clone(), Purity::IMPURE.clone(), ty.clone()) });
    result
}

fn makeInitialCall() -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INITIAL().clone(), metamodelica::nil(), Variability::DISCRETE.clone(), Purity::IMPURE.clone(), Arc::new(crate::NFType::BOOLEAN)) });
    result
}

fn makeMaxIntArrCall(mut exps: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arrTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    arrTy = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: list![Arc::new(Dimension::NFDimension::INTEGER { size: (exps.clone().len() as i32), var: Variability::STRUCTURAL_PARAMETER.clone() })] });
    result = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_INT_ARR().clone(), list![Arc::new(Expression::NFExpression::ARRAY { ty: arrTy.clone(), elements: metamodelica::arrayFromVec(exps.clone().into_iter().cloned().collect()), literal: true })], Variability::DISCRETE.clone(), Purity::PURE.clone(), Arc::new(crate::NFType::INTEGER)) });
    result
}

fn makeSampleTimeCall() -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut timeExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut clockExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = Arc::new(crate::NFType::REAL);
    timeExp = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: (literal!("time")).clone() }), ty.clone(), metamodelica::nil(), Arc::new(crate::NFComponentRef::EMPTY)) });
    clockExp = Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) });
    result = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::SAMPLE_CLOCKED().clone(), list![timeExp.clone(), clockExp.clone()], Variability::CONTINUOUS.clone(), Purity::IMPURE.clone(), ty.clone()) });
    result
}

fn makeRelationEq(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: Operator::makeEqual(ty.clone()), exp2: exp2.clone(), index: 0 });
    result
}

fn makeRelationGt(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: Operator::makeGreater(ty.clone()), exp2: exp2.clone(), index: 0 });
    result
}

// ============================================================
// Start value helpers
// ============================================================
fn getStartValue(mut var: Arc<Variable::NFVariable>) -> Result<Arc<Expression::NFExpression>> {
    let mut startExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut attrName: ArcStr = arcstr::literal!("");
    let mut attrBinding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut startOpt: Option<Arc<Expression::NFExpression>> = None;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    for mut attr in &*var.typeAttributes.clone() {
        let mut attr = attr.clone();
        (attrName, attrBinding) = attr.clone();
        if attrName.clone() == literal!("start") {
            startOpt = Binding::typedExp(attrBinding.clone());
            if isSome(startOpt.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(startOpt.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                startExp = __pa0.clone();
                return Ok(startExp);
            }
        }
    }
    ty = var.ty.clone();
    startExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }),
        Deref @ Type::BOOLEAN => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        Deref @ Type::STRING => Arc::new(Expression::NFExpression::STRING { value: (literal!("")).clone() }),
        _ => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(startExp)
}

// ============================================================
// ComponentRef prefix check
// ============================================================
fn crefHasPrefix(mut prefix: Arc<ComponentRef::NFComponentRef>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    if ComponentRef::isEqual(prefix.clone(), cref.clone())? {
        res = true;
    } else if ComponentRef::isEmpty(cref.clone()) {
        res = false;
    } else {
        res = crefHasPrefix(prefix.clone(), ComponentRef::rest(cref.clone())?)?;
    }
    Ok(res)
}

