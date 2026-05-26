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

use crate::BackendDAE as OldBackendDAE;
use crate::BackendDAE::SimIterator as OldSimIterator;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Frame;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBModule as Module;
use crate::NBSolve as Solve;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimGenericCall::SimIterator;
use crate::NSimStrongComponent::Block;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBuiltin as Builtin;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
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

// OF imports
// NF
// OB
// New Backend
// SimCode
// Util
// =========================================================================
//                      MAIN ROUTINE, PLEASE DO NOT CHANGE
// =========================================================================
pub fn getModule() -> Result<Module::eventsInterface> {
    let mut func: Module::eventsInterface;
    let mut flag: ArcStr = literal!("default");
    func = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "default" => eventsDefault.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

// =========================================================================
//                    TYPES, UNIONTYPES AND MEMBER FUNCTIONS
// =========================================================================
pub mod EventInfo {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct EventInfo {
        /// tracks compact time events (SINGLE or SAMPLE)
        pub time_set: Arc<UnorderedSet::UnorderedSet<Arc<TimeEvent::TimeEvent>>>,
        /// tracks full time events of the form $TEV_11 = ...
        pub time_map: Arc<UnorderedMap::UnorderedMap<Arc<Condition::Condition>, Arc<CompositeEvent::CompositeEvent>>>,
        /// tracks full state events of the form $SEV_4 = ...
        pub state_map: Arc<UnorderedMap::UnorderedMap<Arc<Condition::Condition>, Arc<StateEvent::StateEvent>>>,
        /// stores the number of math function that trigger events e.g. floor, ceil, integer, ...
        pub numberMathEvents: i32,
    }

    impl Default for EventInfo {
        fn default() -> Self {
            Self {
                time_set: Default::default(),
                time_map: Default::default(),
                state_map: Default::default(),
                numberMathEvents: Default::default(),
            }
        }
    }

    pub type EVENT_INFO = EventInfo;

    pub fn toString(mut eventInfo: Arc<EventInfo>) -> Result<ArcStr> {
        fn tplString<T1: Clone + 'static, T2: Clone + 'static>(mut tpl: (T1, T2), mut f1: Arc<dyn ::std::ops::Fn(T1) -> Result<ArcStr> + 'static>, mut f2: Arc<dyn ::std::ops::Fn(T2) -> Result<ArcStr> + 'static>) -> ArcStr {
            type F1<T1: Clone> = fn(T1) -> Result<ArcStr>;

            type F2<T2: Clone> = fn(T2) -> Result<ArcStr>;

            let mut r#str: ArcStr = arcstr::literal!("");
            let mut t1: T1;
            let mut t2: T2;
            (t1, t2) = tpl.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*f2(t2.clone()).unwrap()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*f1(t1.clone()).unwrap()); ArcStr::from(__mm_s) }).clone();
            r#str
        }

        let mut r#str: ArcStr = literal!("");
        let mut tev_lst: Arc<metamodelica::List<Arc<TimeEvent::TimeEvent>>> = metamodelica::nil();
        let mut cev_lst: Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<CompositeEvent::CompositeEvent>)>> = metamodelica::nil();
        let mut sev_lst: Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<StateEvent::StateEvent>)>> = metamodelica::nil();
        if !(isEmpty(eventInfo.clone())) {
            (tev_lst, cev_lst, sev_lst) = toLists(eventInfo.clone())?;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Event Info")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("Time Events")).clone())); __mm_s.push_str(&*List::toString(tev_lst.clone(), Arc::new({ let __pe_b1 = true; move |__pe_a0| TimeEvent::toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("Composite Events")).clone())); __mm_s.push_str(&*List::toString(cev_lst.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> = Arc::new(Condition::toString); let __pe_b2: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> = Arc::new(fnptr!(CompositeEvent::toString, Arc<CompositeEvent::CompositeEvent>)); move |__pe_a0| Ok(tplString(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("State Events")).clone())); __mm_s.push_str(&*List::toString(sev_lst.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> = Arc::new(Condition::toString); let __pe_b2: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> = Arc::new(fnptr!(StateEvent::toString, Arc<StateEvent::StateEvent>)); move |__pe_a0| Ok(tplString(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn toLists(mut eventInfo: Arc<EventInfo>) -> Result<(Arc<metamodelica::List<Arc<TimeEvent::TimeEvent>>>, Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<CompositeEvent::CompositeEvent>)>>, Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<StateEvent::StateEvent>)>>)> {
        let mut tev_lst: Arc<metamodelica::List<Arc<TimeEvent::TimeEvent>>> = metamodelica::nil();
        let mut cev_lst: Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<CompositeEvent::CompositeEvent>)>> = metamodelica::nil();
        let mut sev_lst: Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<StateEvent::StateEvent>)>> = metamodelica::nil();
        tev_lst = List::sort(UnorderedSet::toList(eventInfo.time_set.clone()), Arc::new(fnptr!(TimeEvent::indexGt, Arc<TimeEvent::TimeEvent>, Arc<TimeEvent::TimeEvent>)))?;
        cev_lst = List::sort(UnorderedMap::toList(eventInfo.time_map.clone()), Arc::new(fnptr!(CompositeEvent::indexGt, (Arc<Condition::Condition>, Arc<CompositeEvent::CompositeEvent>), (Arc<Condition::Condition>, Arc<CompositeEvent::CompositeEvent>))))?;
        sev_lst = List::sort(UnorderedMap::toList(eventInfo.state_map.clone()), Arc::new(fnptr!(StateEvent::indexGt, (Arc<Condition::Condition>, Arc<StateEvent::StateEvent>), (Arc<Condition::Condition>, Arc<StateEvent::StateEvent>))))?;
        Ok((tev_lst, cev_lst, sev_lst))
    }

    pub fn create(mut bucket: Arc<Bucket>, mut variables: Arc<VariablePointers::VariablePointers>, mut idx: Pointer::Pointer<i32>) -> Result<(Arc<EventInfo>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> {
        let mut eventInfo: Arc<EventInfo>;
        let mut auxiliary_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut auxiliary_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut cond: Arc<Condition::Condition>;
        let mut cev: Arc<CompositeEvent::CompositeEvent>;
        let mut sev: Arc<StateEvent::StateEvent>;
        for mut tpl in &*UnorderedMap::toList(bucket.time_map.clone()) {
            let mut tpl = tpl.clone();
            (cond, cev) = tpl.clone();
            (auxiliary_vars, auxiliary_eqns) = createAux(cond.clone(), cev.auxiliary.clone(), variables.clone(), idx.clone(), auxiliary_vars.clone(), auxiliary_eqns.clone())?;
        }
        for mut tpl in &*UnorderedMap::toList(bucket.state_map.clone()) {
            let mut tpl = tpl.clone();
            (cond, sev) = tpl.clone();
            (auxiliary_vars, auxiliary_eqns) = createAux(cond.clone(), sev.auxiliary.clone(), variables.clone(), idx.clone(), auxiliary_vars.clone(), auxiliary_eqns.clone())?;
        }
        eventInfo = Arc::new(EventInfo { numberMathEvents: 0, state_map: bucket.state_map.clone(), time_map: bucket.time_map.clone(), time_set: bucket.time_set.clone() });
        if Flags::isSet(Flags::DUMP_EVENTS.clone())? {
            println!("{}", (toString(eventInfo.clone())?).clone());
            println!("{}", (List::toString(auxiliary_eqns.clone(), Arc::new({ let __pe_b1 = (literal!("  ")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), (StringUtil::headline_4((literal!("Event Equations")).clone())).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("\n\n")).clone(), true, 0)?).clone());
        }
        Ok((eventInfo, auxiliary_vars, auxiliary_eqns))
    }

    pub fn createAux(mut cond: Arc<Condition::Condition>, mut aux_var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut variables: Arc<VariablePointers::VariablePointers>, mut idx: Pointer::Pointer<i32>, mut auxiliary_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut auxiliary_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> {
        let mut auxiliary_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = auxiliary_vars;
        let mut auxiliary_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = auxiliary_eqns;
        let mut lhs_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut aux_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
        if cond.stmt_index.clone() == 0 {
            lhs_cref = ComponentRef::mapSubscripts(BVariable::getVarName(aux_var.clone()), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = true; move |__pe_a0| OldBackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }); move |__pe_a0| Subscript::mapExp(__pe_a0, __pe_b1.clone()) }), false);
            aux_eqn = BEquation::Equation::makeAssignment(Expression::fromCref(lhs_cref.clone(), false)?, cond.exp.clone(), idx.clone(), (literal!("EVT")).clone(), cond.iter.clone(), BEquation::default(EquationKind::DISCRETE.clone(), false, None, None))?;
            auxiliary_eqns = cons(aux_eqn.clone(), auxiliary_eqns.clone());
        }
        BVariable::setVarName(aux_var.clone(), ComponentRef::stripSubscriptsAll(BVariable::getVarName(aux_var.clone())));
        auxiliary_vars = cons(aux_var.clone(), auxiliary_vars.clone());
        Ok((auxiliary_vars, auxiliary_eqns))
    }

    pub fn createAuxStatements(mut new_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut bucket_ptr: Pointer::Pointer<Arc<Bucket>>, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
        let mut new_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = new_stmts;
        let mut bucket: Arc<Bucket> = Pointer::access(bucket_ptr.clone());
        let mut new_stmt: Arc<Statement::NFStatement>;
        let mut cond: Arc<Condition::Condition>;
        let mut aux: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut lhs_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        if Util::isSome(bucket.aux_stmts.clone()) {
            for mut tpl in &*Util::getOption(bucket.aux_stmts.clone())? {
                let mut tpl = tpl.clone();
                (cond, aux) = tpl.clone();
                aux = ComponentRef::mapSubscripts(aux.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = true; move |__pe_a0| OldBackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }); move |__pe_a0| Subscript::mapExp(__pe_a0, __pe_b1.clone()) }), false);
                new_stmt = Statement::makeAssignment(Expression::fromCref(aux.clone(), false)?, cond.exp.clone(), ComponentRef::getSubscriptedType(aux.clone(), false)?, DAE::emptyElementSource.clone());
                new_stmts = cons(new_stmt.clone(), new_stmts.clone());
            }
            assign_field!(bucket.aux_stmts = None);
            Pointer::update(bucket_ptr.clone(), bucket.clone());
        }
        Ok(new_stmts)
    }

    pub fn empty() -> Arc<EventInfo> {
        let mut eventInfo: Arc<EventInfo>;
        eventInfo = Arc::new(EventInfo { numberMathEvents: 0, state_map: UnorderedMap::new(fnptr!(Condition::hash, Arc<Condition::Condition>), fnptr!(Condition::isEqual, Arc<Condition::Condition>, Arc<Condition::Condition>), 1), time_map: UnorderedMap::new(fnptr!(Condition::hash, Arc<Condition::Condition>), fnptr!(Condition::isEqual, Arc<Condition::Condition>, Arc<Condition::Condition>), 1), time_set: UnorderedSet::new(fnptr!(TimeEvent::hash, Arc<TimeEvent::TimeEvent>), TimeEvent::isEqual, 13) });
        eventInfo
    }

    pub fn isEmpty(mut eventInfo: Arc<EventInfo>) -> bool {
        let mut b: bool = false;
        b = UnorderedSet::isEmpty(eventInfo.time_set.clone()) && UnorderedMap::isEmpty(eventInfo.time_map.clone()) && UnorderedMap::isEmpty(eventInfo.state_map.clone()) && eventInfo.numberMathEvents.clone() == 0;
        b
    }

    pub fn convert(mut eventInfo: Arc<EventInfo>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block::Block>>>) -> Result<(Arc<metamodelica::List<OldBackendDAE::ZeroCrossing>>, Arc<metamodelica::List<OldBackendDAE::ZeroCrossing>>, Arc<metamodelica::List<OldBackendDAE::TimeEvent>>)> {
        let mut zeroCrossings: Arc<metamodelica::List<OldBackendDAE::ZeroCrossing>> = metamodelica::nil();
        let mut relations: Arc<metamodelica::List<OldBackendDAE::ZeroCrossing>> = metamodelica::nil();
        let mut timeEvents: Arc<metamodelica::List<OldBackendDAE::TimeEvent>> = metamodelica::nil();
        let mut tev_lst: Arc<metamodelica::List<Arc<TimeEvent::TimeEvent>>> = metamodelica::nil();
        let mut cev_lst: Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<CompositeEvent::CompositeEvent>)>> = metamodelica::nil();
        let mut sev_lst: Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<StateEvent::StateEvent>)>> = metamodelica::nil();
        (tev_lst, cev_lst, sev_lst) = toLists(eventInfo.clone())?;
        zeroCrossings = {
        let mut __acc: Arc<metamodelica::List<OldBackendDAE::ZeroCrossing>> = metamodelica::nil();
        for mut sev_tpl in (sev_lst.clone()).into_iter().cloned() {
            let __x = StateEvent::convert(sev_tpl.clone(), equation_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        relations = zeroCrossings.clone();
        timeEvents = {
        let mut __acc: Arc<metamodelica::List<OldBackendDAE::TimeEvent>> = metamodelica::nil();
        for mut tev in (tev_lst.clone()).into_iter().cloned() {
            let __x = TimeEvent::convert(tev.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        Ok((zeroCrossings, relations, timeEvents))
    }

}

pub mod TimeEvent {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    pub enum TimeEvent {
        /// e.g. time > 0.5
        SINGLE {
            /// unique sample index
            index: i32,
            /// single point in time that triggers it
            trigger: Arc<Expression::NFExpression>,
        },
        /// e.g. sample(1, 1)
        SAMPLE {
            /// unique sample index
            index: i32,
            /// first trigger point
            start: Arc<Expression::NFExpression>,
            /// equidistant intervals
            interval: Arc<Expression::NFExpression>,
        },
    }
    impl Default for TimeEvent {
        fn default() -> Self {
            Self::SINGLE {
                index: Default::default(),
                trigger: Default::default(),
            }
        }
    }
    pub use self::TimeEvent::{SINGLE,SAMPLE};
    pub fn toString(mut timeEvent: Arc<TimeEvent>, mut printIndex: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(timeEvent.clone()) {
        Deref @ SINGLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("time > ")); __mm_s.push_str(&*Expression::toString(var_field!((*timeEvent).trigger, TimeEvent::SINGLE).clone())?); ArcStr::from(__mm_s) },
        Deref @ SAMPLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("sample(")); __mm_s.push_str(&*intString(var_field!((*timeEvent).index, TimeEvent::SAMPLE).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(var_field!((*timeEvent).start, TimeEvent::SAMPLE).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(var_field!((*timeEvent).interval, TimeEvent::SAMPLE).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEvents.TimeEvent.toString")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        if printIndex.clone() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(getIndex(timeEvent.clone())?)); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn toStringList(mut events_lst: Arc<metamodelica::List<Arc<TimeEvent>>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (StringUtil::headline_4((literal!("Time Events")).clone())).clone();
        if events_lst.clone().is_empty() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\t<No Time Events>\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut te in (events_lst.clone()).into_iter().cloned() {
            let __x = toString(te.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn hash(mut tev: Arc<TimeEvent>) -> i32 {
        let mut h: i32 = stringHashDjb2((toString(tev.clone(), false).unwrap()).clone());
        h
    }

    pub fn isEqual(mut tev1: Arc<TimeEvent>, mut tev2: Arc<TimeEvent>) -> Result<bool> {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &((tev1.clone(), tev2.clone())) {
        (Deref @ SINGLE { .. }, Deref @ SINGLE { .. }) => Expression::isEqual(var_field!((*tev1).trigger, TimeEvent::SINGLE).clone(), var_field!((*tev2).trigger, TimeEvent::SINGLE).clone())?,
        (Deref @ SAMPLE { .. }, Deref @ SAMPLE { .. }) => Expression::isEqual(var_field!((*tev1).start, TimeEvent::SAMPLE).clone(), var_field!((*tev2).start, TimeEvent::SAMPLE).clone())? && Expression::isEqual(var_field!((*tev1).interval, TimeEvent::SAMPLE).clone(), var_field!((*tev2).interval, TimeEvent::SAMPLE).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(b)
    }

    pub fn indexGt(mut tev1: Arc<TimeEvent>, mut tev2: Arc<TimeEvent>) -> bool {
        let mut b: bool = getIndex(tev1.clone()).unwrap() > getIndex(tev2.clone()).unwrap();
        b
    }

    pub fn create(mut exp: Arc<Expression::NFExpression>, mut bucket: Arc<Bucket>, mut iter: Arc<Iterator::Iterator>, mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut createEqn: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Bucket>, bool)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut bucket: Arc<Bucket> = bucket;
        let mut failed: bool = false;
        (exp, bucket, failed) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::LBINARY { .. } if (Operator::getMathClassification(var_field!((*exp).operator, Expression::NFExpression::LBINARY).clone())? == Operator::MathClassification::LOGICAL.clone()) => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut b1: bool = false;
            let mut b2: bool = false;
            (exp1, bucket, b1) = create(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), bucket.clone(), iter.clone(), eqn.clone(), funcMap.clone(), createEqn.clone())?;
            (exp2, bucket, b2) = create(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), bucket.clone(), iter.clone(), eqn.clone(), funcMap.clone(), createEqn.clone())?;
            failed = b1.clone() || b2.clone();
            if !(failed.clone()) {
                assign_variant_field!(exp => Expression::NFExpression::LBINARY;
                    exp1 = exp1.clone(),
                    exp2 = exp2.clone()
                );
            }
            (exp.clone(), bucket.clone(), failed.clone())
        },
        _ => {
            createSingleOrSample(exp.clone(), bucket.clone(), iter.clone(), eqn.clone(), funcMap.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(failed.clone()) {
            (exp, bucket) = CompositeEvent::add(exp.clone(), iter.clone(), bucket.clone(), createEqn.clone())?;
        }
        Ok((exp, bucket, failed))
    }

    pub fn createSingleOrSample(mut exp: Arc<Expression::NFExpression>, mut bucket: Arc<Bucket>, mut iter: Arc<Iterator::Iterator>, mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Expression::NFExpression>, Arc<Bucket>, bool)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut bucket: Arc<Bucket> = bucket;
        let mut failed: bool = false;
        (exp, failed) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } => {
            let mut call: Arc<Call::NFCall>;
            let mut containsTime: Pointer::Pointer<bool> = Pointer::create(false);
            (call, bucket, failed, _) = createSample(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), bucket.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            (exp.clone(), failed.clone())
        },
        Deref @ Expression::RELATION { .. } if (Operator::getMathClassification(var_field!((*exp).operator, Expression::NFExpression::RELATION).clone())? == Operator::MathClassification::RELATION.clone()) => {
            let mut tmpEqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut status: Solve::Status = Solve::Status::UNPROCESSED;
            let mut can_trigger: bool = false;
            let mut invert: Solve::RelationInversion = Solve::RelationInversion::TRUE;
            let mut trigger: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut timeEvent: Arc<TimeEvent>;
            let mut containsTime: Pointer::Pointer<bool> = Pointer::create(false);
            tmpEqn = Pointer::access(BEquation::Equation::makeAssignment(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), Pointer::create(0), (arcstr::literal!(BVariable::TEMPORARY_STR)).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), BEquation::default(EquationKind::UNKNOWN.clone(), false, None, None))?);
            let _ = BEquation::Equation::map(tmpEqn.clone(), { let __pe_b1 = containsTime.clone(); move |__pe_a0| Ok(containsTimeTraverseExp(__pe_a0, __pe_b1.clone())) }, Some({ let __pe_b1 = containsTime.clone(); move |__pe_a0| Ok(containsTimeTraverseCref(__pe_a0, __pe_b1.clone())) }), Expression::map)?;
            if Pointer::access(containsTime.clone()) {
                (tmpEqn, status, invert) = Solve::solveBody(tmpEqn.clone(), Builtin::TIME_CREF().clone(), funcMap.clone())?;
                if status.clone() == Solve::Status::EXPLICIT.clone() && invert.clone() != Solve::RelationInversion::UNKNOWN.clone() {
                    let __pa0 = ::match_deref::match_deref! { match &(BEquation::Equation::getRHS(tmpEqn.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    trigger = __pa0.clone();
                    assign_variant_field!(exp => Expression::NFExpression::RELATION; operator = if (invert.clone() == Solve::RelationInversion::TRUE.clone()) {Operator::invert(var_field!((*exp).operator, Expression::NFExpression::RELATION).clone())?} else {var_field!((*exp).operator, Expression::NFExpression::RELATION).clone()});
                    if BEquation::Equation::isWhenEquation(eqn.clone()) {
                        can_trigger = (match var_field!((*exp).operator, Expression::NFExpression::RELATION).op.clone() {
        Operator::Op::GREATER => true,
        Operator::Op::GREATEREQ => true,
        _ => false,
    });
                        new_exp = if (can_trigger.clone()) {Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::SAMPLE().clone(), list![Arc::new(Expression::NFExpression::INTEGER { value: UnorderedSet::size(bucket.time_set.clone()) + 1 }), trigger.clone(), Expression::makeMaxValue(Arc::new(openmodelica_nf_frontend::NFType::REAL))?], Prefixes::Variability::DISCRETE.clone(), Prefixes::Purity::PURE.clone(), NFBuiltinFuncs::SAMPLE().returnType.clone()) })} else {Arc::new(Expression::NFExpression::BOOLEAN { value: false })};
                    } else {
                        can_trigger = true;
                        new_exp = exp.clone();
                    }
                    if can_trigger.clone() {
                        timeEvent = Arc::new(TimeEvent::SINGLE { index: UnorderedSet::size(bucket.time_set.clone()), trigger: trigger.clone() });
                        if !(UnorderedSet::contains(timeEvent.clone(), bucket.time_set.clone())?) {
                            UnorderedSet::add(timeEvent.clone(), bucket.time_set.clone())?;
                        }
                    }
                    failed = false;
                } else {
                    failed = true;
                    new_exp = exp.clone();
                }
            } else {
                failed = true;
                new_exp = exp.clone();
            }
            (new_exp.clone(), failed.clone())
        },
        _ => {
            let mut containsTime: Pointer::Pointer<bool> = Pointer::create(false);
            (exp.clone(), true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((exp, bucket, failed))
    }

    pub fn createSample(mut call: Arc<Call::NFCall>, mut bucket: Arc<Bucket>) -> Result<(Arc<Call::NFCall>, Arc<Bucket>, bool, bool)> {
        let mut call: Arc<Call::NFCall> = call;
        let mut bucket: Arc<Bucket> = bucket;
        let mut failed: bool = false;
        let mut clocked: bool = false;
        (failed, clocked) = (::match_deref::match_deref! { match &((AbsynUtil::pathLastIdent(Call::functionName(call.clone())?)?, Call::arguments(call.clone())?)) {
        (Deref @ "sample", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: clock, tail: Deref @ metamodelica::List::Nil } }) if (Type::isClock(Expression::typeOf(clock.clone()))) => {
            (false, true)
        },
        (Deref @ "sample", Deref @ metamodelica::List::Cons { head: start, tail: Deref @ metamodelica::List::Cons { head: interval, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut timeEvent: Arc<TimeEvent>;
            timeEvent = Arc::new(TimeEvent::SAMPLE { index: UnorderedSet::size(bucket.time_set.clone()), start: start.clone(), interval: interval.clone() });
            if !(UnorderedSet::contains(timeEvent.clone(), bucket.time_set.clone())?) {
                UnorderedSet::add(timeEvent.clone(), bucket.time_set.clone())?;
            }
            call = Call::setArguments(call.clone(), list![Arc::new(Expression::NFExpression::INTEGER { value: getIndex(timeEvent.clone())? + 1 }), start.clone(), interval.clone()])?;
            (false, false)
        },
        (Deref @ "sample", _) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEvents.TimeEvent.createSample")); __mm_s.push_str(&*literal!(" failed for sample operator: ")); __mm_s.push_str(&*Call::toString(call.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            (true, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((call, bucket, failed, clocked))
    }

    pub fn createSampleTraverse(mut exp: Arc<Expression::NFExpression>, mut bucket: Arc<Bucket>, mut clocked: Pointer::Pointer<bool>) -> Result<(Arc<Expression::NFExpression>, Arc<Bucket>)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut bucket: Arc<Bucket> = bucket;
        let mut c: bool = false;
        exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call } => {
            let mut call = (*call).clone();
            (call, bucket, _, c) = createSample(call.clone(), bucket.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            if c.clone() {
                Pointer::update(clocked.clone(), c.clone());
            }
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((exp, bucket))
    }

    pub fn getIndex(mut timeEvent: Arc<TimeEvent>) -> Result<i32> {
        let mut index: i32 = 0;
        index = (::match_deref::match_deref! { match &(timeEvent.clone()) {
        Deref @ SINGLE { .. } => var_field!((*timeEvent).index, TimeEvent::SINGLE).clone(),
        Deref @ SAMPLE { .. } => var_field!((*timeEvent).index, TimeEvent::SAMPLE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(index)
    }

    pub fn setIndex(mut timeEvent: Arc<TimeEvent>, mut index: i32) -> Arc<TimeEvent> {
        let mut timeEvent: Arc<TimeEvent> = timeEvent;
        timeEvent = (::match_deref::match_deref! { match &(timeEvent.clone()) {
        Deref @ SINGLE { .. } => {
            assign_variant_field!(timeEvent => TimeEvent::SINGLE; index = index.clone());
            timeEvent.clone()
        },
        Deref @ SAMPLE { .. } => {
            assign_variant_field!(timeEvent => TimeEvent::SAMPLE; index = index.clone());
            timeEvent.clone()
        },
        _ => timeEvent.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        timeEvent
    }

    pub fn convert(mut timeEvent: Arc<TimeEvent>) -> Result<OldBackendDAE::TimeEvent> {
        let mut oldTimeEvent: OldBackendDAE::TimeEvent = OldBackendDAE::TimeEvent::SIMPLE_TIME_EVENT;
        oldTimeEvent = (::match_deref::match_deref! { match &(timeEvent.clone()) {
        Deref @ SINGLE { .. } => OldBackendDAE::TimeEvent::SAMPLE_TIME_EVENT { intervalExp: Expression::toDAE(Expression::makeMaxValue(Arc::new(openmodelica_nf_frontend::NFType::REAL))?, false)?, startExp: Expression::toDAE(var_field!((*timeEvent).trigger, TimeEvent::SINGLE).clone(), false)?, index: var_field!((*timeEvent).index, TimeEvent::SINGLE).clone() },
        Deref @ SAMPLE { .. } => OldBackendDAE::TimeEvent::SAMPLE_TIME_EVENT { intervalExp: Expression::toDAE(var_field!((*timeEvent).interval, TimeEvent::SAMPLE).clone(), false)?, startExp: Expression::toDAE(var_field!((*timeEvent).start, TimeEvent::SAMPLE).clone(), false)?, index: var_field!((*timeEvent).index, TimeEvent::SAMPLE).clone() },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEvents.TimeEvent.convert")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldTimeEvent)
    }

}

pub mod StateEvent {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct StateEvent {
        /// index for simcode
        pub index: i32,
        /// auxiliary variable representing the relation
        pub auxiliary: Pointer::Pointer<Arc<Variable::NFVariable>>,
        /// equations where the function occurs
        pub eqns: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Equation::Equation>>>>,
    }

    impl Default for StateEvent {
        fn default() -> Self {
            Self {
                index: Default::default(),
                auxiliary: Default::default(),
                eqns: Default::default(),
            }
        }
    }

    pub type STATE_EVENT = StateEvent;

    pub fn toString(mut sev: Arc<StateEvent>) -> ArcStr {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(sev.index.clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*BVariable::toString(Pointer::access(sev.auxiliary.clone()), (literal!("")).clone()).unwrap()); ArcStr::from(__mm_s) };
        r#str
    }

    pub fn toStringList(mut events_lst: Arc<metamodelica::List<Arc<StateEvent>>>) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (StringUtil::headline_4((literal!("State Events")).clone())).clone();
        if events_lst.clone().is_empty() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\t<No State Events>\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut te in (events_lst.clone()).into_iter().cloned() {
            let __x = toString(te.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
        }
        r#str
    }

    pub fn indexGt(mut tpl1: (Arc<Condition::Condition>, Arc<StateEvent>), mut tpl2: (Arc<Condition::Condition>, Arc<StateEvent>)) -> bool {
        let mut b: bool = false;
        let mut sev1: Arc<StateEvent>;
        let mut sev2: Arc<StateEvent>;
        (_, sev1) = tpl1.clone();
        (_, sev2) = tpl2.clone();
        b = sev1.index.clone() > sev2.index.clone();
        b
    }

    pub fn fromStatement(mut stmt: Arc<Statement::NFStatement>, mut bucket_ptr: Pointer::Pointer<Arc<Bucket>>, mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut variables: Arc<VariablePointers::VariablePointers>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>) -> Result<Arc<Statement::NFStatement>> {
        let mut stmt: Arc<Statement::NFStatement> = stmt;
        stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::FOR { range: Some(range), .. } => {
            let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut new_frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
            let mut new_stmt: Arc<Statement::NFStatement>;
            let mut new_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            new_stmts = metamodelica::nil();
            name = ComponentRef::fromNode(var_field!((*stmt).iterator, Statement::NFStatement::FOR).clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), metamodelica::nil(), ComponentRef::Origin::CREF.clone());
            name = BackendDAE::lowerComponentReference(name.clone(), variables.clone(), true)?;
            new_frames = cons((name.clone(), range.clone(), None), frames.clone());
            for mut elem in &*var_field!((*stmt).body, Statement::NFStatement::FOR).clone() {
                let mut elem = elem.clone();
                new_stmt = fromStatement(elem.clone(), bucket_ptr.clone(), eqn.clone(), variables.clone(), funcMap.clone(), new_frames.clone())?;
                new_stmts = cons(new_stmt.clone(), new_stmts.clone());
                new_stmts = EventInfo::createAuxStatements(new_stmts.clone(), bucket_ptr.clone(), variables.clone())?;
            }
            assign_variant_field!(stmt => Statement::NFStatement::FOR; body = new_stmts.clone().reverse());
            stmt.clone()
        },
        _ => {
            let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
            iter = BEquation::Iterator::fromFrames(frames.clone().reverse());
            stmt = Statement::mapExp(stmt.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b1 = bucket_ptr.clone(); let __pe_b2 = iter.clone(); let __pe_b3 = eqn.clone(); let __pe_b4 = funcMap.clone(); let __pe_b5 = false; move |__pe_a0| collectEventsTraverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }); move |__pe_a0| Ok(Expression::fakeMap(__pe_a0, __pe_b1.clone())) }));
            stmt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(stmt)
    }

    pub fn create(mut exp: Arc<Expression::NFExpression>, mut bucket: Arc<Bucket>, mut iter: Arc<Iterator::Iterator>, mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut createEqn: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Bucket>)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut bucket: Arc<Bucket> = bucket;
        let mut condition: Arc<Condition::Condition>;
        let mut sev_opt: Option<Arc<StateEvent>> = None;
        let mut sev: Arc<StateEvent>;
        let mut aux_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut aux_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut clocked: Pointer::Pointer<bool> = Pointer::create(false);
        (exp, bucket) = Expression::mapFold(exp.clone(), Arc::new({ let __pe_b2 = clocked.clone(); move |__pe_a0, __pe_a1| TimeEvent::createSampleTraverse(__pe_a0, __pe_a1, __pe_b2.clone()) }), bucket.clone())?;
        if createEqn.clone() {
            condition = Arc::new(Condition::Condition { exp: exp.clone(), iter: iter.clone(), stmt_index: 0 });
        } else {
            condition = Arc::new(Condition::Condition { exp: exp.clone(), iter: iter.clone(), stmt_index: bucket.stmt_index.clone() });
            assign_field!(bucket.stmt_index = bucket.stmt_index.clone() + 1);
        }
        sev_opt = UnorderedMap::get(condition.clone(), bucket.state_map.clone());
        if Util::isSome(sev_opt.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(sev_opt.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            sev = __pa0.clone();
            UnorderedSet::add(eqn.clone(), sev.eqns.clone())?;
            UnorderedMap::add(condition.clone(), sev.clone(), bucket.state_map.clone())?;
            aux_cref = BVariable::getVarName(sev.auxiliary.clone());
            exp = Expression::fromCref(aux_cref.clone(), false)?;
        } else if !(Pointer::access(clocked.clone())) {
            (aux_var, aux_cref) = BVariable::makeEventVar((arcstr::literal!(BVariable::STATE_EVENT_STR)).clone(), UnorderedMap::size(bucket.state_map.clone()), Expression::typeOf(exp.clone()), iter.clone())?;
            exp = Expression::fromCref(aux_cref.clone(), false)?;
            sev = Arc::new(StateEvent { index: UnorderedMap::size(bucket.state_map.clone()), auxiliary: aux_var.clone(), eqns: UnorderedSet::fromList(list![eqn.clone()], fnptr!(Equation::hash, Pointer::Pointer<Arc<Equation::Equation>>), fnptr!(Equation::equalName, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>))? });
            condition = Condition::setRelationIndex(condition.clone(), sev.index.clone());
            UnorderedMap::add(condition.clone(), sev.clone(), bucket.state_map.clone())?;
        }
        if !(createEqn.clone() || Pointer::access(clocked.clone())) {
            assign_field!(bucket.aux_stmts = Some(cons((condition.clone(), aux_cref.clone()), Util::getOptionOrDefault(bucket.aux_stmts.clone(), metamodelica::nil()))));
        }
        Ok((exp, bucket))
    }

    pub fn convert(mut sev_tpl: (Arc<Condition::Condition>, Arc<StateEvent>), mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block::Block>>>) -> Result<OldBackendDAE::ZeroCrossing> {
        let mut oldZc: OldBackendDAE::ZeroCrossing;
        let mut cond: Arc<Condition::Condition>;
        let mut sev: Arc<StateEvent>;
        let mut iter: Option<Arc<metamodelica::List<OldSimIterator>>> = None;
        let mut eqn_names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut eqn_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
        (cond, sev) = sev_tpl.clone();
        iter = if (BEquation::Iterator::isEmpty(cond.iter.clone())) {None} else {Some({
        let mut __acc: Arc<metamodelica::List<OldSimIterator>> = metamodelica::nil();
        for mut it in (SimIterator::fromIterator(cond.iter.clone())?).into_iter().cloned() {
            let __x = SimIterator::convert(it.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })};
        eqn_names = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut eqn in (UnorderedSet::toList(sev.eqns.clone())).into_iter().cloned() {
            if !(!(BEquation::Equation::isDummy(Pointer::access(eqn.clone())))) { continue; }
            let __x = BEquation::Equation::getEqnName(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        eqn_indices = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut name in (eqn_names.clone()).into_iter().cloned() {
            if !(UnorderedMap::contains(name.clone(), equation_map.clone())) { continue; }
            let __x = Block::getIndex(UnorderedMap::getSafe(name.clone(), equation_map.clone(), metamodelica::sourceInfo!())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        oldZc = OldBackendDAE::ZeroCrossing { iter: iter.clone(), occurEquLst: eqn_indices.clone(), relation_: Expression::toDAE(cond.exp.clone(), false)?, index: sev.index.clone() };
        Ok(oldZc)
    }

}

pub mod CompositeEvent {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct CompositeEvent {
        pub index: i32,
        pub auxiliary: Pointer::Pointer<Arc<Variable::NFVariable>>,
    }

    impl Default for CompositeEvent {
        fn default() -> Self {
            Self {
                index: Default::default(),
                auxiliary: Default::default(),
            }
        }
    }

    pub type COMPOSITE_EVENT = CompositeEvent;

    pub fn toString(mut cev: Arc<CompositeEvent>) -> ArcStr {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(cev.index.clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*BVariable::pointerToString(cev.auxiliary.clone())); ArcStr::from(__mm_s) };
        r#str
    }

    pub fn indexGt(mut tpl1: (Arc<Condition::Condition>, Arc<CompositeEvent>), mut tpl2: (Arc<Condition::Condition>, Arc<CompositeEvent>)) -> bool {
        let mut b: bool = false;
        let mut cev1: Arc<CompositeEvent>;
        let mut cev2: Arc<CompositeEvent>;
        (_, cev1) = tpl1.clone();
        (_, cev2) = tpl2.clone();
        b = cev1.index.clone() > cev2.index.clone();
        b
    }

    pub fn create(mut exp: Arc<Expression::NFExpression>, mut bucket: Arc<Bucket>, mut iter: Arc<Iterator::Iterator>, mut createEqn: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Bucket>, bool)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut bucket: Arc<Bucket> = bucket;
        let mut failed: bool = false;
        let mut aux_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut aux_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        (exp, bucket, failed) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::LBINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::AND, .. }, exp1: exp1 @ Deref @ Expression::CALL { call }, .. } if (BackendUtil::isOnlyTimeDependent(exp1.clone())?) => {
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp1 = (*exp1).clone();
            let mut call = (*call).clone();
            (call, exp2, bucket, failed) = checkDirectComposite(call.clone(), var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), bucket.clone(), iter.clone(), createEqn.clone())?;
            if !(failed.clone()) {
                assign_variant_field!(exp1 => Expression::NFExpression::CALL; call = call.clone());
                assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp1 = exp1.clone());
                if !(referenceEq(&exp2.clone(),&var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone())) {
                    assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp2 = exp2.clone());
                }
            }
            (exp.clone(), bucket.clone(), failed.clone())
        },
        Deref @ Expression::LBINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::AND, .. }, exp2: exp2 @ Deref @ Expression::CALL { call }, .. } if (BackendUtil::isOnlyTimeDependent(exp2.clone())?) => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2 = (*exp2).clone();
            let mut call = (*call).clone();
            (call, exp1, bucket, failed) = checkDirectComposite(call.clone(), var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), bucket.clone(), iter.clone(), createEqn.clone())?;
            if !(failed.clone()) {
                assign_variant_field!(exp2 => Expression::NFExpression::CALL; call = call.clone());
                assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp2 = exp2.clone());
                if !(referenceEq(&exp1.clone(),&var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone())) {
                    assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp1 = exp1.clone());
                }
            }
            (exp.clone(), bucket.clone(), failed.clone())
        },
        Deref @ Expression::LBINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::AND, .. }, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp1, bucket, failed) = create(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), bucket.clone(), iter.clone(), createEqn.clone())?;
            if !(failed.clone()) {
                assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp1 = exp1.clone());
                (exp2, bucket, failed) = create(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), bucket.clone(), iter.clone(), createEqn.clone())?;
                if !(failed.clone()) {
                    assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp2 = exp2.clone());
                }
                failed = false;
            } else {
                (exp2, bucket, failed) = create(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), bucket.clone(), iter.clone(), createEqn.clone())?;
                if !(failed.clone()) {
                    assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp2 = exp2.clone());
                }
            }
            (exp.clone(), bucket.clone(), failed.clone())
        },
        _ => {
            (exp.clone(), bucket.clone(), true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(failed.clone()) {
            (exp, bucket) = add(exp.clone(), iter.clone(), bucket.clone(), createEqn.clone())?;
        }
        Ok((exp, bucket, failed))
    }

    pub fn checkDirectComposite(mut call: Arc<Call::NFCall>, mut exp: Arc<Expression::NFExpression>, mut bucket: Arc<Bucket>, mut iter: Arc<Iterator::Iterator>, mut createEqn: bool) -> Result<(Arc<Call::NFCall>, Arc<Expression::NFExpression>, Arc<Bucket>, bool)> {
        let mut call: Arc<Call::NFCall> = call;
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut bucket: Arc<Bucket> = bucket;
        let mut failed: bool = false;
        let mut failed2: bool = false;
        (call, bucket, failed, _) = TimeEvent::createSample(call.clone(), bucket.clone())?;
        if !(failed.clone()) {
            (exp, bucket, failed2) = create(exp.clone(), bucket.clone(), iter.clone(), createEqn.clone())?;
            if !(failed2.clone()) {
            }
        }
        Ok((call, exp, bucket, failed))
    }

    pub fn add(mut cond: Arc<Expression::NFExpression>, mut iter: Arc<Iterator::Iterator>, mut bucket: Arc<Bucket>, mut createEqn: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Bucket>)> {
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut bucket: Arc<Bucket> = bucket;
        let mut condition: Arc<Condition::Condition>;
        let mut cev_opt: Option<Arc<CompositeEvent>> = None;
        let mut cev: Arc<CompositeEvent>;
        let mut aux_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut aux_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        if createEqn.clone() {
            condition = Arc::new(Condition::Condition { exp: cond.clone(), iter: iter.clone(), stmt_index: 0 });
        } else {
            condition = Arc::new(Condition::Condition { exp: cond.clone(), iter: iter.clone(), stmt_index: bucket.stmt_index.clone() });
            assign_field!(bucket.stmt_index = bucket.stmt_index.clone() + 1);
        }
        cev_opt = UnorderedMap::get(condition.clone(), bucket.time_map.clone());
        if Util::isSome(cev_opt.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(cev_opt.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cev = __pa0.clone();
            aux_cref = BVariable::getVarName(cev.auxiliary.clone());
            exp = Expression::fromCref(aux_cref.clone(), false)?;
        } else {
            (aux_var, aux_cref) = BVariable::makeEventVar((arcstr::literal!(BVariable::TIME_EVENT_STR)).clone(), UnorderedMap::size(bucket.time_map.clone()), Expression::typeOf(condition.exp.clone()), iter.clone())?;
            exp = Expression::fromCref(aux_cref.clone(), false)?;
            cev = Arc::new(CompositeEvent { index: UnorderedMap::size(bucket.time_map.clone()), auxiliary: aux_var.clone() });
            UnorderedMap::add(condition.clone(), cev.clone(), bucket.time_map.clone())?;
        }
        if !(createEqn.clone()) {
            assign_field!(bucket.aux_stmts = Some(cons((condition.clone(), aux_cref.clone()), Util::getOptionOrDefault(bucket.aux_stmts.clone(), metamodelica::nil()))));
        }
        Ok((exp, bucket))
    }

}

pub mod Condition {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct Condition {
        pub exp: Arc<Expression::NFExpression>,
        pub iter: Arc<Iterator::Iterator>,
        pub stmt_index: i32,
    }

    impl Default for Condition {
        fn default() -> Self {
            Self {
                exp: Default::default(),
                iter: Default::default(),
                stmt_index: Default::default(),
            }
        }
    }

    pub type CONDITION = Condition;

    pub fn toString(mut cond: Arc<Condition>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (Expression::toString(cond.exp.clone())?).clone();
        if !(BEquation::Iterator::isEmpty(cond.iter.clone())) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" for {")); __mm_s.push_str(&*BEquation::Iterator::toString(cond.iter.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
        }
        if !(cond.stmt_index.clone() == 0) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(cond.stmt_index.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn hash(mut cond: Arc<Condition>) -> i32 {
        let mut h: i32 = stringHashDjb2((toString(cond.clone()).unwrap()).clone());
        h
    }

    pub fn isEqual(mut cond1: Arc<Condition>, mut cond2: Arc<Condition>) -> bool {
        let mut b: bool = Expression::isEqual(cond1.exp.clone(), cond2.exp.clone()).unwrap() && BEquation::Iterator::isEqual(cond1.iter.clone(), cond2.iter.clone()).unwrap() && cond1.stmt_index.clone() == cond2.stmt_index.clone();
        b
    }

    pub fn size(mut cond: Arc<Condition>) -> i32 {
        let mut s: i32 = BEquation::Iterator::size(cond.iter.clone(), false);
        s
    }

    pub fn setRelationIndex(mut cond: Arc<Condition>, mut index: i32) -> Arc<Condition> {
        let mut cond: Arc<Condition> = cond;
        assign_field!(cond.exp = (::match_deref::match_deref! { match &(cond.exp.clone()) {
        exp @ Deref @ Expression::RELATION { .. } => {
            let mut exp = (*exp).clone();
            assign_variant_field!(exp => Expression::NFExpression::RELATION; index = index.clone());
            exp.clone()
        },
        _ => {
            cond.exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }));
        cond
    }

}

// =========================================================================
//                    PROTECTED UNIONTYPES AND FUNCTIONS
// =========================================================================
#[derive(Clone, Debug, PartialEq)]
pub struct Bucket {
    /// tracks compact time events (SINGLE or SAMPLE)
    pub time_set: Arc<UnorderedSet::UnorderedSet<Arc<TimeEvent::TimeEvent>>>,
    /// tracks full time events of the form $TEV_11 = ...
    pub time_map: Arc<UnorderedMap::UnorderedMap<Arc<Condition::Condition>, Arc<CompositeEvent::CompositeEvent>>>,
    /// tracks full state events of the form $SEV_4 = ...
    pub state_map: Arc<UnorderedMap::UnorderedMap<Arc<Condition::Condition>, Arc<StateEvent::StateEvent>>>,
    /// optional statement conditions in algorithms
    pub aux_stmts: Option<Arc<metamodelica::List<(Arc<Condition::Condition>, Arc<ComponentRef::NFComponentRef>)>>>,
    /// index to be used for unique statement auxiliaries
    pub stmt_index: i32,
}

impl Default for Bucket {
    fn default() -> Self {
        Self {
            time_set: Default::default(),
            time_map: Default::default(),
            state_map: Default::default(),
            aux_stmts: Default::default(),
            stmt_index: Default::default(),
        }
    }
}

pub type BUCKET = Bucket;


fn collectEvents(mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>, mut bucket_ptr: Pointer::Pointer<Arc<Bucket>>, mut variables: Arc<VariablePointers::VariablePointers>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
    let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>> = eqn_ptr;
    let mut eqn: Arc<Equation::Equation> = Pointer::access(eqn_ptr.clone());
    let mut body_eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut createEqn: bool = !(BEquation::Equation::isAlgorithm(eqn_ptr.clone()));
    let mut collector: BEquation::MapFuncExp;
    let mut alg: Arc<Algorithm::NFAlgorithm>;
    let mut new_stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    iter = BEquation::Equation::getForIterator(eqn.clone());
    collector = { let __pe_b1 = bucket_ptr.clone(); let __pe_b2 = iter.clone(); let __pe_b3 = eqn_ptr.clone(); let __pe_b4 = funcMap.clone(); let __pe_b5 = createEqn.clone(); move |__pe_a0| collectEventsTraverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) };
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::ALGORITHM { alg, .. } => {
            let mut alg = (*alg).clone();
            new_stmts = metamodelica::nil();
            for mut stmt in &*alg.statements.clone() {
                let mut stmt = stmt.clone();
                stmt = StateEvent::fromStatement(stmt.clone(), bucket_ptr.clone(), eqn_ptr.clone(), variables.clone(), funcMap.clone(), metamodelica::nil())?;
                new_stmts = EventInfo::createAuxStatements(new_stmts.clone(), bucket_ptr.clone(), variables.clone())?;
                new_stmts = cons(stmt.clone(), new_stmts.clone());
            }
            assign_field!(alg.statements = new_stmts.clone().reverse());
            assign_variant_field!(eqn => Equation::Equation::ALGORITHM;
                alg = Algorithm::setInputsOutputs(alg.clone())?,
                size = {
        let mut __acc: i32 = 0;
        for mut out in (var_field!((*eqn).alg, Equation::Equation::ALGORITHM).outputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::size(out.clone(), true, false);
            __acc += __x;
        }
        __acc
    }
            );
            eqn.clone()
        },
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::Equation::WHEN_EQUATION; body = BEquation::WhenEquationBody::mapCondition(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone(), collector, None, fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>)));
            eqn.clone()
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body_eqn @ Deref @ BEquation::Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut body_eqn = (*body_eqn).clone();
            todo!("unhandled field-assign shape: body_eqn.body");
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![body_eqn.clone()]);
            eqn.clone()
        },
        Deref @ BEquation::Equation::IF_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; body = BEquation::IfEquationBody::mapEqnExpCref(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), { let __pe_b1 = bucket_ptr.clone(); let __pe_b2 = variables.clone(); let __pe_b3 = funcMap.clone(); move |__pe_a0| collectEvents(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }, collector, None, Expression::mapReverse)?);
            eqn.clone()
        },
        _ => BEquation::Equation::map(eqn.clone(), collector, None, fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(referenceEq(&eqn.clone(),&Pointer::access(eqn_ptr.clone()))) {
        Pointer::update(eqn_ptr.clone(), eqn.clone());
    }
    Ok(eqn_ptr)
}

fn collectEventsTraverse(mut exp: Arc<Expression::NFExpression>, mut bucket_ptr: Pointer::Pointer<Arc<Bucket>>, mut iter: Arc<Iterator::Iterator>, mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut createEqn: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::LUNARY { .. } => {
            let mut bucket: Arc<Bucket>;
            (exp, bucket) = collectEventsCondition(exp.clone(), Pointer::access(bucket_ptr.clone()), iter.clone(), eqn.clone(), funcMap.clone(), createEqn.clone())?;
            Pointer::update(bucket_ptr.clone(), bucket.clone());
            exp.clone()
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut bucket: Arc<Bucket>;
            (exp, bucket) = collectEventsCondition(exp.clone(), Pointer::access(bucket_ptr.clone()), iter.clone(), eqn.clone(), funcMap.clone(), createEqn.clone())?;
            Pointer::update(bucket_ptr.clone(), bucket.clone());
            exp.clone()
        },
        Deref @ Expression::RELATION { .. } => {
            let mut bucket: Arc<Bucket>;
            (exp, bucket) = collectEventsCondition(exp.clone(), Pointer::access(bucket_ptr.clone()), iter.clone(), eqn.clone(), funcMap.clone(), createEqn.clone())?;
            Pointer::update(bucket_ptr.clone(), bucket.clone());
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (Call::isNamed(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), (literal!("sample")).clone())?) => {
            let mut bucket: Arc<Bucket>;
            (exp, bucket) = collectEventsCondition(exp.clone(), Pointer::access(bucket_ptr.clone()), iter.clone(), eqn.clone(), funcMap.clone(), createEqn.clone())?;
            Pointer::update(bucket_ptr.clone(), bucket.clone());
            exp.clone()
        },
        Deref @ Expression::CLKCONST { clk: clk @ Deref @ ClockKind::EVENT_CLOCK { condition, .. } } => {
            let mut clk = (*clk).clone();
            assign_variant_field!(clk => ClockKind::NFClockKind::EVENT_CLOCK; condition = collectEventsTraverse(condition.clone(), bucket_ptr.clone(), iter.clone(), eqn.clone(), funcMap.clone(), createEqn.clone())?);
            assign_variant_field!(exp => Expression::NFExpression::CLKCONST; clk = clk.clone());
            exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } } if (Call::isNamed(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), (literal!("pre")).clone())?) => {
            let mut bucket: Arc<Bucket>;
            (exp, bucket) = CompositeEvent::add(exp.clone(), iter.clone(), Pointer::access(bucket_ptr.clone()), createEqn.clone())?;
            Pointer::update(bucket_ptr.clone(), bucket.clone());
            exp.clone()
        },
        Deref @ Expression::CREF { .. } if (BVariable::isPrevious(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?)) => {
            let mut bucket: Arc<Bucket>;
            (exp, bucket) = CompositeEvent::add(exp.clone(), iter.clone(), Pointer::access(bucket_ptr.clone()), createEqn.clone())?;
            Pointer::update(bucket_ptr.clone(), bucket.clone());
            exp.clone()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
            let mut new_frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
            let mut call = (*call).clone();
            new_frames = {
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<_>)>> = metamodelica::nil();
        for mut tpl in (var_field!((*call).iters, Call::NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = (ComponentRef::fromNode(Util::tuple21(tpl.clone()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), metamodelica::nil(), ComponentRef::Origin::CREF.clone()), Util::tuple22(tpl.clone()), None);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assign_variant_field!(call => Call::NFCall::TYPED_REDUCTION; exp = collectEventsTraverse(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), bucket_ptr.clone(), BEquation::Iterator::addFrames(iter.clone(), new_frames.clone())?, eqn.clone(), funcMap.clone(), createEqn.clone())?);
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp.clone()
        },
        Deref @ Expression::CALL { .. } if (Call::isNamed(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), (literal!("noEvent")).clone())?) => {
            exp.clone()
        },
        Deref @ Expression::CREF { .. } => {
            exp.clone()
        },
        _ => {
            Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = bucket_ptr.clone(); let __pe_b2 = iter.clone(); let __pe_b3 = eqn.clone(); let __pe_b4 = funcMap.clone(); let __pe_b5 = createEqn.clone(); move |__pe_a0| collectEventsTraverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn collectEventsCondition(mut exp: Arc<Expression::NFExpression>, mut bucket: Arc<Bucket>, mut iter: Arc<Iterator::Iterator>, mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut createEqn: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Bucket>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut bucket: Arc<Bucket> = bucket;
    let mut failed: bool = true;
    if BackendUtil::isOnlyTimeDependent(exp.clone())? {
        (exp, bucket, failed) = TimeEvent::create(exp.clone(), bucket.clone(), iter.clone(), eqn.clone(), funcMap.clone(), createEqn.clone())?;
    } else {
        (exp, bucket, failed) = CompositeEvent::create(exp.clone(), bucket.clone(), iter.clone(), createEqn.clone())?;
    }
    if failed.clone() {
        (exp, bucket) = StateEvent::create(exp.clone(), bucket.clone(), iter.clone(), eqn.clone(), createEqn.clone())?;
    }
    Ok((exp, bucket))
}

fn containsTimeTraverseExp(mut exp: Arc<Expression::NFExpression>, mut b: Pointer::Pointer<bool>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    if !(Pointer::access(b.clone())) && Expression::isTime(exp.clone()) {
        Pointer::update(b.clone(), true);
    }
    exp
}

fn containsTimeTraverseCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut b: Pointer::Pointer<bool>) -> Arc<ComponentRef::NFComponentRef> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    if !(Pointer::access(b.clone())) && ComponentRef::isTime(cref.clone()) {
        Pointer::update(b.clone(), true);
    }
    cref
}

