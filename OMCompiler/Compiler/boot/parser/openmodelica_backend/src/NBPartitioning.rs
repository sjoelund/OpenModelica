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
use crate::NBAdjacency;
use crate::NBBackendUtil::Rational;
use crate::NBCausalize as Causalize;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE as OldDAE;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF
// Backend
// Util
// Old imports
pub mod BClock {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum BClock {
        BASE_CLOCK {
            clock: Arc<ClockKind::NFClockKind>,
        },
        SUB_CLOCK {
            factor: Arc<Rational::Rational>,
            shift: Arc<Rational::Rational>,
            solver: Option<ArcStr>,
        },
        INFERRED_CLOCK {
            base_ref: Arc<ComponentRef::NFComponentRef>,
        },
    }
    impl Default for BClock {
        fn default() -> Self {
            Self::BASE_CLOCK {
                clock: Default::default(),
            }
        }
    }
    pub use self::BClock::{BASE_CLOCK,SUB_CLOCK,INFERRED_CLOCK};
    pub fn toString(mut clock: Arc<BClock>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { .. } => ClockKind::toDebugString(var_field!((*clock).clock, BClock::BASE_CLOCK).clone())?,
        Deref @ SUB_CLOCK { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SUB_CLOCK(")); __mm_s.push_str(&*Rational::toString(var_field!((*clock).factor, BClock::SUB_CLOCK).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Rational::toString(var_field!((*clock).shift, BClock::SUB_CLOCK).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ INFERRED_CLOCK { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("INFERRED_CLOCK(")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*clock).base_ref, BClock::INFERRED_CLOCK).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => literal!("UNKNOWN_CLOCK()"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn hash(mut clock: Arc<BClock>) -> i32 {
        let mut i: i32 = stringHashDjb2((toString(clock.clone()).unwrap()).clone());
        i
    }

    pub fn isEqual(mut clock1: Arc<BClock>, mut clock2: Arc<BClock>) -> Result<bool> {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &((clock1.clone(), clock2.clone())) {
        (Deref @ BASE_CLOCK { .. }, Deref @ BASE_CLOCK { .. }) => ClockKind::compare(var_field!((*clock1).clock, BClock::BASE_CLOCK).clone(), var_field!((*clock2).clock, BClock::BASE_CLOCK).clone())? == 0,
        (Deref @ SUB_CLOCK { .. }, Deref @ SUB_CLOCK { .. }) => Rational::isEqual(var_field!((*clock1).factor, BClock::SUB_CLOCK).clone(), var_field!((*clock2).factor, BClock::SUB_CLOCK).clone()) && Rational::isEqual(var_field!((*clock1).shift, BClock::SUB_CLOCK).clone(), var_field!((*clock2).shift, BClock::SUB_CLOCK).clone()) && Util::optionEqual(var_field!((*clock1).solver, BClock::SUB_CLOCK).clone(), var_field!((*clock2).solver, BClock::SUB_CLOCK).clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)),
        (Deref @ INFERRED_CLOCK { .. }, Deref @ INFERRED_CLOCK { .. }) => ComponentRef::isEqual(var_field!((*clock1).base_ref, BClock::INFERRED_CLOCK).clone(), var_field!((*clock2).base_ref, BClock::INFERRED_CLOCK).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(b)
    }

    pub fn add(mut eqn: Arc<Equation::Equation>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<()> {
        let _ = (::match_deref::match_deref! { match &((BEquation::Equation::getLHS(eqn.clone())?, BEquation::Equation::getRHS(eqn.clone())?)) {
        (Some(Deref @ Expression::CREF { cref: clock_name, .. }), Some(exp)) if (Expression::isClockOrSampleFunction(exp.clone())?) => {
            create(clock_name.clone(), exp.clone(), info.clone())?;
            ()
        },
        (Some(exp), Some(Deref @ Expression::CREF { cref: clock_name, .. })) if (Expression::isClockOrSampleFunction(exp.clone())?) => {
            create(clock_name.clone(), exp.clone(), info.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn isBaseClock(mut clock: Arc<BClock>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isInferredClock(mut clock: Arc<BClock>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { clock: Deref @ ClockKind::INFERRED_CLOCK { .. } } => true,
        Deref @ INFERRED_CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isEventClock(mut clock: Arc<BClock>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { clock: Deref @ ClockKind::EVENT_CLOCK { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn baseClockInferrence(mut clock: Arc<BClock>, mut base_clock_inferrence: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<BClock>>>) -> Result<Arc<BClock>> {
        let mut clock: Arc<BClock> = clock;
        clock = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ INFERRED_CLOCK { .. } => {
            let mut base_clock: Arc<BClock>;
            base_clock = UnorderedMap::getSafe(var_field!((*clock).base_ref, BClock::INFERRED_CLOCK).clone(), base_clock_inferrence.clone(), metamodelica::sourceInfo!())?;
            baseClockInferrence(base_clock.clone(), base_clock_inferrence.clone())?
        },
        Deref @ BASE_CLOCK { clock: Deref @ ClockKind::INFERRED_CLOCK { .. } } => {
            DEFAULT_BASE_CLOCK().clone()
        },
        _ => {
            clock.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(clock)
    }

    pub fn convertBase(mut clock: Arc<BClock>) -> Result<Arc<OldDAE::ClockKind>> {
        let mut oldClock: Arc<OldDAE::ClockKind> = Arc::new(OldDAE::ClockKind::INFERRED_CLOCK);
        oldClock = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { .. } => ClockKind::toDAE(var_field!((*clock).clock, BClock::BASE_CLOCK).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.convertBase")); __mm_s.push_str(&*literal!(" failed for non-base clock: ")); __mm_s.push_str(&*toString(clock.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldClock)
    }

    pub fn convertSub(mut clock: Arc<BClock>) -> Result<OldBackendDAE::SubClock> {
        let mut oldClock: OldBackendDAE::SubClock = OldBackendDAE::SubClock::INFERED_SUBCLOCK;
        oldClock = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ SUB_CLOCK { .. } => OldBackendDAE::SubClock::SUBCLOCK { solver: var_field!((*clock).solver, BClock::SUB_CLOCK).clone(), shift: Rational::convert(var_field!((*clock).shift, BClock::SUB_CLOCK).clone()), factor: Rational::convert(var_field!((*clock).factor, BClock::SUB_CLOCK).clone()) },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.convertSub")); __mm_s.push_str(&*literal!(" failed for non-sub clock: ")); __mm_s.push_str(&*toString(clock.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldClock)
    }

    pub fn toExp(mut clock: Arc<BClock>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        exp = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { .. } => Arc::new(Expression::NFExpression::CLKCONST { clk: var_field!((*clock).clock, BClock::BASE_CLOCK).clone() }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.toExp")); __mm_s.push_str(&*literal!(" failed for non-base clock: ")); __mm_s.push_str(&*toString(clock.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    fn create(mut clock_name: Arc<ComponentRef::NFComponentRef>, mut exp: Arc<Expression::NFExpression>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<()> {
        let mut clock: Arc<BClock>;
        let mut baseClock: Option<Arc<ComponentRef::NFComponentRef>> = None;
        let mut clock_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        match '__try0: {
            (clock, baseClock) = unwrap_break_err!(fromExp(exp.clone()), '__try0);
            if Util::isSome(baseClock.clone()) {
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), clock.clone(), info.subClocks.clone()), '__try0);
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), Util::getOption(baseClock.clone())?, info.subToBase.clone()), '__try0);
            } else {
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), clock.clone(), info.baseClocks.clone()), '__try0);
            }
            clock_var = unwrap_break_err!(BVariable::getVarPointer(clock_name.clone(), metamodelica::sourceInfo!()), '__try0);
            if !(BVariable::isClockOrClocked(clock_var.clone())) {
                BVariable::setVarKind(clock_var.clone(), Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::CLOCKED));
            }
            Ok::<_, anyhow::Error>((baseClock.clone(), clock.clone(), clock_var.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                baseClock = __try0_o0;
                clock = __try0_o1;
                clock_var = __try0_o2;
            }
            Err(_) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.create")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(clock_name.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
        }
        Ok(())
    }

    fn fromExp(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<BClock>, Option<Arc<ComponentRef::NFComponentRef>>)> {
        let mut subClock: Arc<BClock>;
        let mut baseClock: Option<Arc<ComponentRef::NFComponentRef>> = None;
        (subClock, baseClock) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CLKCONST { .. } => {
            (Arc::new(BClock::BASE_CLOCK { clock: var_field!((*exp).clk, Expression::NFExpression::CLKCONST).clone() }), None)
        },
        Deref @ Expression::CREF { .. } => {
            (DEFAULT_SUB_CLOCK().clone(), Some(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            (baseClock, subClock) = (::match_deref::match_deref! { match &((AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?, Call::arguments(call.clone())?)) {
        (Deref @ "sample", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            (baseClock.clone(), subClock.clone())
        },
        (Deref @ "subSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock.clone(), Arc::new(BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: i1.clone(), d: 1 }), shift: Arc::new(Rational::Rational { n: 0, d: 1 }), solver: None }))?;
            (baseClock.clone(), subClock.clone())
        },
        (Deref @ "superSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock.clone(), Arc::new(BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: 1, d: i1.clone() }), shift: Arc::new(Rational::Rational { n: 0, d: 1 }), solver: None }))?;
            (baseClock.clone(), subClock.clone())
        },
        (Deref @ "shiftSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock.clone(), Arc::new(BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: 1, d: 1 }), shift: Arc::new(Rational::Rational { n: i1.clone(), d: 1 }), solver: None }))?;
            (baseClock.clone(), subClock.clone())
        },
        (Deref @ "shiftSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock.clone(), Arc::new(BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: 1, d: 1 }), shift: Arc::new(Rational::Rational { n: i1.clone(), d: i2.clone() }), solver: None }))?;
            (baseClock.clone(), subClock.clone())
        },
        (Deref @ "backSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock.clone(), Arc::new(BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: 1, d: 1 }), shift: Arc::new(Rational::Rational { n: -(i1.clone()), d: 1 }), solver: None }))?;
            (baseClock.clone(), subClock.clone())
        },
        (Deref @ "backSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock.clone(), Arc::new(BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: 1, d: 1 }), shift: Arc::new(Rational::Rational { n: -(i1.clone()), d: i2.clone() }), solver: None }))?;
            (baseClock.clone(), subClock.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.fromExp")); __mm_s.push_str(&*literal!(" failed for exp with unhandled call: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (subClock.clone(), baseClock.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.fromExp")); __mm_s.push_str(&*literal!(" failed for exp with unhandled expression kind: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((subClock, baseClock))
    }

    fn updateSubClock(mut dest: Arc<BClock>, mut src: Arc<BClock>) -> Result<Arc<BClock>> {
        let mut dest: Arc<BClock> = dest;
        dest = (::match_deref::match_deref! { match &((dest.clone(), src.clone())) {
        (Deref @ SUB_CLOCK { .. }, Deref @ SUB_CLOCK { .. }) => {
            assign_variant_field!(dest => BClock::SUB_CLOCK;
                shift = Rational::add(var_field!((*dest).shift, BClock::SUB_CLOCK).clone(), Rational::multiply(var_field!((*src).shift, BClock::SUB_CLOCK).clone(), var_field!((*dest).factor, BClock::SUB_CLOCK).clone())),
                factor = Rational::multiply(var_field!((*dest).factor, BClock::SUB_CLOCK).clone(), var_field!((*src).factor, BClock::SUB_CLOCK).clone())
            );
            dest.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.updateSubClock")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*toString(dest.clone())?); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*toString(src.clone())?); __mm_s.push_str(&*literal!(" because of incorrect clock types.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(dest)
    }

}

thread_local! { static __DEFAULT_BASE_CLOCK_TLS: Arc<BClock::BClock> = Arc::new(BClock::BClock::BASE_CLOCK { clock: Arc::new(ClockKind::NFClockKind::REAL_CLOCK { interval: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) }) }) }); }
pub fn DEFAULT_BASE_CLOCK() -> Arc<BClock::BClock> { __DEFAULT_BASE_CLOCK_TLS.with(|__t| __t.clone()) }

thread_local! { static __DEFAULT_SUB_CLOCK_TLS: Arc<BClock::BClock> = Arc::new(BClock::BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: 1, d: 1 }), shift: Arc::new(Rational::Rational { n: 0, d: 1 }), solver: None }); }
pub fn DEFAULT_SUB_CLOCK() -> Arc<BClock::BClock> { __DEFAULT_SUB_CLOCK_TLS.with(|__t| __t.clone()) }

pub type CrefLst = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

pub mod ClockedInfo {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ClockedInfo {
        pub baseClocks: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>>>,
        pub subClocks: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>>>,
        pub subToBase: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>,
        pub baseToSub: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>,
    }

    pub type CLOCKED_INFO = ClockedInfo;

    pub fn new() -> Arc<ClockedInfo> {
        let mut info: Arc<ClockedInfo> = Arc::new(ClockedInfo { baseToSub: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), subToBase: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), subClocks: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), baseClocks: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1) });
        info
    }

    pub fn toString(mut info: Arc<ClockedInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        if !(isEmpty(info.clone())) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Clocked Info")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Base Clocks")).clone())); __mm_s.push_str(&*UnorderedMap::toString(info.baseClocks.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(BClock::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Sub Clocks")).clone())); __mm_s.push_str(&*UnorderedMap::toString(info.subClocks.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(BClock::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Sub to Base Clocks")).clone())); __mm_s.push_str(&*UnorderedMap::toString(info.subToBase.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Base to Sub Clocks")).clone())); __mm_s.push_str(&*UnorderedMap::toString(info.baseToSub.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(fnptr!(ComponentRef::listToString, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn isEmpty(mut info: Arc<ClockedInfo>) -> bool {
        let mut b: bool = UnorderedMap::isEmpty(info.baseClocks.clone());
        b
    }

    pub fn resolveSubClocks(mut info: Arc<ClockedInfo>, mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        for mut cref in &*UnorderedMap::keyList(clock_map.clone()) {
            let mut cref = cref.clone();
            resolveImplicitSubClock(cref.clone(), info.clone(), clock_map.clone())?;
        }
        for mut sub_clock in &*UnorderedMap::keyList(info.subClocks.clone()) {
            let mut sub_clock = sub_clock.clone();
            resolveSubClock(sub_clock.clone(), info.clone(), clock_map.clone())?;
        }
        for mut sub_clock in &*UnorderedMap::keyList(info.subClocks.clone()) {
            let mut sub_clock = sub_clock.clone();
            addSubClock(sub_clock.clone(), info.clone())?;
        }
        Ok(())
    }

    pub fn baseClockCount(mut info: Arc<ClockedInfo>, mut countInferred: bool) -> i32 {
        let mut count: i32 = UnorderedMap::size(info.baseClocks.clone());
        if !(countInferred.clone()) {
            count = count.clone() - List::count(UnorderedMap::valueList(info.baseClocks.clone()), (std::sync::Arc::new(fnptr!(BClock::isInferredClock, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<bool> + 'static>));
        }
        count
    }

    pub fn subClockCount(mut info: Arc<ClockedInfo>) -> i32 {
        let mut count: i32 = UnorderedMap::size(info.subClocks.clone());
        count
    }

    fn resolveImplicitSubClock(mut key: Arc<ComponentRef::NFComponentRef>, mut info: Arc<ClockedInfo>, mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut clock: Arc<ComponentRef::NFComponentRef> = key.clone();
        if UnorderedMap::contains(key.clone(), clock_map.clone()) {
            clock = UnorderedMap::getSafe(key.clone(), clock_map.clone(), metamodelica::sourceInfo!())?;
            if !(UnorderedMap::contains(clock.clone(), info.subClocks.clone()) || UnorderedMap::contains(clock.clone(), info.baseClocks.clone())) {
                clock = resolveImplicitSubClock(clock.clone(), info.clone(), clock_map.clone())?;
                UnorderedMap::add(key.clone(), clock.clone(), clock_map.clone())?;
            }
        }
        Ok(clock)
    }

    fn resolveSubClock(mut clock_name: Arc<ComponentRef::NFComponentRef>, mut info: Arc<ClockedInfo>, mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut base_clock: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut implicit_clock: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut parent_clock: Arc<ComponentRef::NFComponentRef> = UnorderedMap::getSafe(clock_name.clone(), info.subToBase.clone(), metamodelica::sourceInfo!())?;
        let mut implicit_clock_opt: Option<Arc<ComponentRef::NFComponentRef>> = None;
        let mut dest: Arc<BClock::BClock>;
        let mut src: Arc<BClock::BClock>;
        if UnorderedMap::contains(parent_clock.clone(), info.baseClocks.clone()) {
            base_clock = parent_clock.clone();
        } else {
            if !(UnorderedMap::contains(parent_clock.clone(), info.subClocks.clone())) {
                implicit_clock_opt = Some(parent_clock.clone());
                parent_clock = UnorderedMap::getSafe(parent_clock.clone(), clock_map.clone(), metamodelica::sourceInfo!())?;
            }
            base_clock = resolveSubClock(parent_clock.clone(), info.clone(), clock_map.clone())?;
            dest = UnorderedMap::getSafe(parent_clock.clone(), info.subClocks.clone(), metamodelica::sourceInfo!())?;
            src = UnorderedMap::getSafe(clock_name.clone(), info.subClocks.clone(), metamodelica::sourceInfo!())?;
            UnorderedMap::add(clock_name.clone(), BClock::updateSubClock(dest.clone(), src.clone())?, info.subClocks.clone())?;
            UnorderedMap::add(clock_name.clone(), base_clock.clone(), info.subToBase.clone())?;
            if Util::isSome(implicit_clock_opt.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(implicit_clock_opt.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                implicit_clock = __pa0.clone();
                UnorderedMap::add(implicit_clock.clone(), dest.clone(), info.subClocks.clone())?;
                UnorderedMap::add(implicit_clock.clone(), base_clock.clone(), info.subToBase.clone())?;
            }
        }
        Ok(base_clock)
    }

    fn addSubClock(mut clock_name: Arc<ComponentRef::NFComponentRef>, mut info: Arc<ClockedInfo>) -> Result<()> {
        let mut base_clock: Arc<ComponentRef::NFComponentRef> = UnorderedMap::getSafe(clock_name.clone(), info.subToBase.clone(), metamodelica::sourceInfo!())?;
        let mut current_clocks: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        current_clocks = UnorderedMap::getOrDefault(base_clock.clone(), info.baseToSub.clone(), metamodelica::nil());
        UnorderedMap::add(base_clock.clone(), cons(clock_name.clone(), current_clocks.clone()), info.baseToSub.clone())?;
        Ok(())
    }

}

// =========================================================================
//                      MAIN ROUTINE, PLEASE DO NOT CHANGE
// =========================================================================
pub fn getModule() -> Result<Module::partitioningInterface> {
    let mut func: Module::partitioningInterface;
    let mut flag: ArcStr = literal!("clocked");
    func = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "default" => partitioningClocked.clone(),
        Deref @ "clocked" => partitioningClocked.clone(),
        Deref @ "none" => partitioningNone.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub fn extractClocksEqn(mut eqn: Arc<Equation::Equation>, mut clck_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut infr_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut new_clocks: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut new_infers: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut idx: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::Equation::WHEN_EQUATION; body = Util::getOption(extractClocksWhenCond(Some(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone()), clck_coll.clone(), infr_coll.clone(), new_clocks.clone(), new_infers.clone(), idx.clone())?)?);
            eqn.clone()
        },
        _ => eqn.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eqn = BEquation::Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = clck_coll.clone(); let __pe_b2 = infr_coll.clone(); let __pe_b3 = new_clocks.clone(); let __pe_b4 = new_infers.clone(); let __pe_b5 = idx.clone(); let __pe_b6 = false; move |__pe_a0| extractClocks(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(eqn)
}

pub fn extractClocksWhenCond(mut body_opt: Option<Arc<WhenEquationBody::WhenEquationBody>>, mut clck_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut infr_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut new_clocks: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut new_infers: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut idx: Pointer::Pointer<i32>) -> Result<Option<Arc<WhenEquationBody::WhenEquationBody>>> {
    let mut body_opt: Option<Arc<WhenEquationBody::WhenEquationBody>> = body_opt;
    body_opt = (::match_deref::match_deref! { match &(body_opt.clone()) {
        Some(body) => {
            let mut body = (*body).clone();
            assign_field!(
                body.condition = Expression::map(body.condition.clone(), Arc::new({ let __pe_b1 = clck_coll.clone(); let __pe_b2 = infr_coll.clone(); let __pe_b3 = new_clocks.clone(); let __pe_b4 = new_infers.clone(); let __pe_b5 = idx.clone(); let __pe_b6 = true; move |__pe_a0| extractClocks(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }))?,
                body.else_when = extractClocksWhenCond(body.else_when.clone(), clck_coll.clone(), infr_coll.clone(), new_clocks.clone(), new_infers.clone(), idx.clone())?
            );
            Some(body.clone())
        },
        _ => {
            body_opt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(body_opt)
}

pub fn extractClocks(mut exp: Arc<Expression::NFExpression>, mut clck_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut infr_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut new_clocks: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut new_infers: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut idx: Pointer::Pointer<i32>, mut when_cond: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CLKCONST { .. } if (when_cond.clone() || !(ClockKind::isInferred(var_field!((*exp).clk, Expression::NFExpression::CLKCONST).clone()))) => {
            let mut clock: Arc<BClock::BClock>;
            let mut clock_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut clock_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            clock = Arc::new(BClock::BClock::BASE_CLOCK { clock: var_field!((*exp).clk, Expression::NFExpression::CLKCONST).clone() });
            if UnorderedMap::contains(clock.clone(), clck_coll.clone()) {
                clock_name = UnorderedMap::getSafe(clock.clone(), clck_coll.clone(), metamodelica::sourceInfo!())?;
            } else if UnorderedMap::contains(clock.clone(), infr_coll.clone()) {
                clock_name = UnorderedMap::getSafe(clock.clone(), infr_coll.clone(), metamodelica::sourceInfo!())?;
            } else {
                (clock_var, clock_name) = BVariable::makeClockVar(Pointer::access(idx.clone()), Expression::typeOf(exp.clone()))?;
                if BClock::isInferredClock(clock.clone()) {
                    UnorderedMap::add(clock.clone(), clock_name.clone(), infr_coll.clone())?;
                    Pointer::update(new_infers.clone(), cons(clock_var.clone(), Pointer::access(new_infers.clone())));
                } else {
                    UnorderedMap::add(clock.clone(), clock_name.clone(), clck_coll.clone())?;
                    Pointer::update(new_clocks.clone(), cons(clock_var.clone(), Pointer::access(new_clocks.clone())));
                }
                Pointer::update(idx.clone(), Pointer::access(idx.clone()) + 1);
            }
            Expression::fromCref(clock_name.clone(), false)?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ClusterElementType {
    EQUATION = 1,
    VARIABLE = 2,
}
impl PartialOrd for ClusterElementType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ClusterElementType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub mod Cluster {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Cluster {
        /// set of all variables in this cluster
        pub variables: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>,
        /// set of all equations in this cluster
        pub eqn_idnts: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>,
    }

    impl Default for Cluster {
        fn default() -> Self {
            Self {
                variables: Default::default(),
                eqn_idnts: Default::default(),
            }
        }
    }

    pub type CLUSTER = Cluster;

    pub fn toString(mut cluster: Arc<Cluster>) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Cluster Variables:\n")); __mm_s.push_str(&*UnorderedSet::toString(cluster.variables.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n### Cluster Equation Identifiers:\n")); __mm_s.push_str(&*UnorderedSet::toString(cluster.eqn_idnts.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
        r#str
    }

    pub fn addElement(mut cluster_opt: Option<Arc<Cluster>>, mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: ClusterElementType) -> Result<Arc<Cluster>> {
        let mut cluster: Arc<Cluster> = Arc::new(<Cluster as ::std::default::Default>::default());
        cluster = (::match_deref::match_deref! { match &(cluster_opt.clone()) {
        Some(cluster) => cluster.clone(),
        _ => Arc::new(Cluster { eqn_idnts: UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13), variables: UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        cluster = (match ty.clone() {
        ClusterElementType::VARIABLE => {
            UnorderedSet::add(cref.clone(), cluster.variables.clone())?;
            cluster.clone()
        },
        ClusterElementType::EQUATION { .. } => {
            UnorderedSet::add(cref.clone(), cluster.eqn_idnts.clone())?;
            cluster.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.Cluster.addElement")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" because of unknown cluster element type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
        Ok(cluster)
    }

    pub fn addToClockMap(mut cluster: Arc<Cluster>, mut equations: Arc<EquationPointers::EquationPointers>, mut info: Arc<ClockedInfo::ClockedInfo>, mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        fn findClock(mut exp: Arc<Expression::NFExpression>, mut info: Arc<ClockedInfo::ClockedInfo>, mut clock_ptr: Pointer::Pointer<Option<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
            let mut exp: Arc<Expression::NFExpression> = exp;
            let mut clock_opt: Option<Arc<ComponentRef::NFComponentRef>> = Pointer::access(clock_ptr.clone());
            exp = (::match_deref::match_deref! { match &((exp.clone(), clock_opt.clone())) {
        (_, Some(_)) => exp.clone(),
        (Deref @ Expression::CREF { .. }, None) if (BVariable::isClockOrClocked(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?)) => {
            Pointer::update(clock_ptr.clone(), Some(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()));
            exp.clone()
        },
        (Deref @ Expression::CALL { .. }, _) if (Expression::isClockOrSampleFunction(exp.clone())?) => exp.clone(),
        _ => Expression::mapShallow(exp.clone(), Arc::new(todo!("PARTEVALFUNCTION of findClock: function signature not resolved")))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Ok(exp)
        }

        let mut clock_ptr: Pointer::Pointer<Option<Arc<ComponentRef::NFComponentRef>>> = Pointer::create(None);
        let mut clock_opt: Option<Arc<ComponentRef::NFComponentRef>> = None;
        let mut clock: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        for mut eqn_name in &*UnorderedSet::toList(cluster.eqn_idnts.clone()) {
            let mut eqn_name = eqn_name.clone();
            BEquation::Equation::map(Pointer::access(BEquation::EquationPointers::getEqnByName(equations.clone(), eqn_name.clone())?), Arc::new({ let __pe_b1 = info.clone(); let __pe_b2 = clock_ptr.clone(); move |__pe_a0| findClock(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), None, (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            clock_opt = Pointer::access(clock_ptr.clone());
            if Util::isSome(clock_opt.clone()) {
                break;
            }
        }
        if Util::isSome(clock_opt.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(clock_opt.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            clock = __pa0.clone();
            for mut var_name in &*UnorderedSet::toList(cluster.variables.clone()) {
                let mut var_name = var_name.clone();
                UnorderedMap::add(var_name.clone(), clock.clone(), clock_map.clone())?;
            }
        }
        Ok(())
    }

    pub fn toPartition(mut cluster: Arc<Cluster>, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut kind: Partition::Kind, mut info: Arc<ClockedInfo::ClockedInfo>, mut held_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut infer_del: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Partition::Partition::Partition>> {
        let mut partition: Arc<Partition::Partition::Partition> = Arc::new(<Partition::Partition::Partition as ::std::default::Default>::default());
        let mut cvars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::toList(cluster.variables.clone());
        let mut cidnt: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::toList(cluster.eqn_idnts.clone());
        let mut isInit: bool = Partition::kindIsInitial(kind.clone());
        let mut association: Arc<Partition::Association::Association>;
        let mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut filtered_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut eqn_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut partVariables: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
        let mut partEquations: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
        let mut var_idx: i32 = 0;
        let mut inferred_clocks: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
        var_lst = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut cref in (cvars.clone()).into_iter().cloned() {
            let __x = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        filtered_vars = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (var_lst.clone()).into_iter().cloned() {
            if !(BVariable::VariablePointers::contains(var.clone(), variables.clone())) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        eqn_lst = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut name in (cidnt.clone()).into_iter().cloned() {
            let __x = BEquation::EquationPointers::getEqnByName(equations.clone(), name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        partVariables = BVariable::VariablePointers::fromList(filtered_vars.clone(), false);
        partEquations = BEquation::EquationPointers::fromList(eqn_lst.clone());
        association = Partition::Association::create(partEquations.clone(), kind.clone(), info.clone(), infer_del.clone())?;
        partEquations = BEquation::EquationPointers::mapExp(partEquations.clone(), Arc::new({ let __pe_b1 = held_crefs.clone(); move |__pe_a0| replaceClockedFunctions(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        if Partition::Association::isClocked(association.clone()) {
            partVariables = BVariable::VariablePointers::mapRemovePtr(partVariables.clone(), Arc::new({ let __pe_b1 = inferred_clocks.clone(); move |__pe_a0| collectInferredClock(__pe_a0, __pe_b1.clone()) }))?;
            partEquations = BEquation::EquationPointers::mapRemovePtr(partEquations.clone(), Arc::new({ let __pe_b1 = inferred_clocks.clone(); move |__pe_a0| removeInferredClock(__pe_a0, __pe_b1.clone()) }))?;
            partEquations = BEquation::EquationPointers::map(partEquations.clone(), (std::sync::Arc::new(replaceClockedWhen) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>))?;
            partVariables = BVariable::VariablePointers::mapPtr(partVariables.clone(), Arc::new({ let __pe_b1 = Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::CLOCKED); move |__pe_a0| Ok(BVariable::setVarKind(__pe_a0, __pe_b1.clone())) }))?;
            if BEquation::EquationPointers::size(partEquations.clone()) == 0 {
                UnorderedSet::merge(infer_del.clone(), inferred_clocks.clone())?;
            }
        }
        partition = Arc::new(Partition::Partition::Partition { strongComponents: None, matching: None, adjacencyMatrix: None, equations: partEquations.clone(), daeUnknowns: None, unknowns: partVariables.clone(), association: association.clone(), index: 0 });
        Ok(partition)
    }

    fn collectInferredClock(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut inferred_clocks: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
        let mut delete: bool = BVariable::isClock(var.clone());
        if delete.clone() {
            UnorderedSet::add(BVariable::getVarName(var.clone()), inferred_clocks.clone())?;
        }
        Ok(delete)
    }

    fn removeInferredClock(mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut inferred_clocks: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
        let mut delete: bool = false;
        delete = (::match_deref::match_deref! { match &(Pointer::access(eqn.clone())) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref: lhs, .. }, .. } => {
            UnorderedSet::contains(lhs.clone(), inferred_clocks.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(delete)
    }

}

// Perhaps this deserves its own place in Util/*.mo
pub mod DisjointSetForest {
    use super::*;
    /// Custom implementation of disjoint-set data structure with constant number of elements.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DisjointSetForest {
        pub parent: Pointer::Pointer<metamodelica::Array<i32>>,
        pub rank: Pointer::Pointer<metamodelica::Array<i32>>,
    }

    pub type FOREST = DisjointSetForest;

    pub fn new(mut n: i32) -> Arc<DisjointSetForest> {
        let mut dsf: Arc<DisjointSetForest>;
        dsf = Arc::new(DisjointSetForest { rank: Pointer::create(arrayCreate(n.clone(), 0)), parent: Pointer::create(metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=n.clone()).into_iter() {
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect())) });
        dsf
    }

    pub fn find(mut dsf: Arc<DisjointSetForest>, mut index: i32) -> i32 {
        let mut index: i32 = index;
        let mut parent: metamodelica::Array<i32> = Pointer::access(dsf.parent.clone());
        while index.clone() != parent.borrow()[(index.clone()-1) as usize].clone() {
            {
                let __cell0 = parent.borrow()[(parent.borrow()[(index.clone()-1) as usize].clone()-1) as usize].clone();
                parent.clone().borrow_mut()[(index.clone()-1) as usize] = __cell0;
            }
            index = parent.borrow()[(index.clone()-1) as usize].clone();
        }
        Pointer::update(dsf.parent.clone(), parent.clone());
        index
    }

    pub fn unite(mut dsf: Arc<DisjointSetForest>, mut indices: Arc<metamodelica::List<i32>>) -> Result<i32> {
        let mut root: i32 = 0;
        let mut roots: Arc<metamodelica::List<i32>> = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (indices.clone()).into_iter().cloned() {
            let __x = find(dsf.clone(), i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        let mut parent: metamodelica::Array<i32> = Pointer::access(dsf.parent.clone());
        let mut rank: metamodelica::Array<i32> = Pointer::access(dsf.rank.clone());
        let mut maxRank: i32 = 0;
        let mut tied: bool = false;
        root = listHead(roots.clone())?;
        maxRank = rank.borrow()[(root.clone()-1) as usize].clone();
        for mut r in &*listRest(roots.clone())? {
            let mut r = r.clone();
            if r.clone() != root.clone() {
                if rank.borrow()[(r.clone()-1) as usize].clone() > maxRank.clone() {
                    root = r.clone();
                    maxRank = rank.borrow()[(root.clone()-1) as usize].clone();
                    tied = false;
                } else if rank.borrow()[(r.clone()-1) as usize].clone() == maxRank.clone() {
                    tied = true;
                }
            }
        }
        for mut r in &*roots.clone() {
            let mut r = r.clone();
            {
                let __cell0 = root.clone();
                parent.clone().borrow_mut()[(find(dsf.clone(), r.clone())-1) as usize] = __cell0;
            }
        }
        if tied.clone() {
            {
                let __cell1 = rank.borrow()[(root.clone()-1) as usize].clone() + 1;
                rank.clone().borrow_mut()[(root.clone()-1) as usize] = __cell1;
            }
        }
        Pointer::update(dsf.parent.clone(), parent.clone());
        Pointer::update(dsf.rank.clone(), rank.clone());
        Ok(root)
    }

}

fn sortAndMergeClockedPartitions(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> {
    let mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = partitions;
    let mut clocked_partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
    let mut new_clocked: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>>> = metamodelica::nil();
    let mut clock_collector: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<Partition::Partition::Partition>>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1);
    let mut base_clock_inferrence: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut clock: Arc<BClock::BClock>;
    let mut baseClock: Arc<BClock::BClock>;
    let mut subClock: Arc<BClock::BClock>;
    let mut baseClock_opt: Option<Arc<BClock::BClock>> = None;
    let mut subClockMap: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<Partition::Partition::Partition>>>;
    let mut new_part: Arc<Partition::Partition::Partition> = Arc::new(<Partition::Partition::Partition as ::std::default::Default>::default());
    (clocked_partitions, partitions) = List::splitOnTrue(partitions.clone(), (std::sync::Arc::new(fnptr!(Partition::Partition::isClocked, Arc<Partition::Partition::Partition>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Partition::Partition::Partition>) -> Result<bool> + 'static>));
    for mut baseClock in &*UnorderedMap::valueList(info.baseClocks.clone()) {
        let mut baseClock = baseClock.clone();
        UnorderedMap::add(baseClock.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1), clock_collector.clone())?;
    }
    for mut partition in &*clocked_partitions.clone() {
        let mut partition = partition.clone();
        (clock, baseClock_opt, _) = Partition::Partition::getClocks(partition.clone())?;
        clock = (::match_deref::match_deref! { match &(baseClock_opt.clone()) {
        Some(clock) => clock.clone(),
        _ => clock.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        for mut var in &*BVariable::VariablePointers::toList(partition.unknowns.clone())? {
            let mut var = var.clone();
            UnorderedMap::add(BVariable::getVarName(var.clone()), clock.clone(), base_clock_inferrence.clone())?;
        }
    }
    for mut partition in &*clocked_partitions.clone() {
        let mut partition = partition.clone();
        (clock, baseClock_opt, _) = Partition::Partition::getClocks(partition.clone())?;
        if Util::isSome(baseClock_opt.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(baseClock_opt.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            baseClock = __pa0.clone();
            baseClock = BClock::baseClockInferrence(baseClock.clone(), base_clock_inferrence.clone())?;
            subClock = clock.clone();
        } else {
            baseClock = BClock::baseClockInferrence(clock.clone(), base_clock_inferrence.clone())?;
            subClock = DEFAULT_SUB_CLOCK().clone();
        }
        partition = Partition::Partition::setClocks(partition.clone(), subClock.clone(), Some(baseClock.clone()))?;
        subClockMap = UnorderedMap::getSafe(baseClock.clone(), clock_collector.clone(), metamodelica::sourceInfo!())?;
        new_part = (::match_deref::match_deref! { match &(UnorderedMap::get(subClock.clone(), subClockMap.clone())) {
        Some(new_part) => Partition::Partition::merge(new_part.clone(), partition.clone(), true)?,
        _ => partition.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        UnorderedMap::add(subClock.clone(), new_part.clone(), subClockMap.clone())?;
    }
    for mut tpl in &*UnorderedMap::toList(clock_collector.clone()) {
        let mut tpl = tpl.clone();
        (baseClock, subClockMap) = tpl.clone();
        new_clocked = cons(sortClockedPartitions(UnorderedMap::valueList(subClockMap.clone()))?, new_clocked.clone());
    }
    partitions = {
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut partition in (cons(partitions.clone(), new_clocked.clone()).reverse()).into_iter().cloned() {
            let __x = partition.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    };
    Ok(partitions)
}

fn sortClockedPartitions(mut unsorted: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> {
    let mut sorted: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
    let mut n: i32 = (unsorted.clone().len() as i32);
    let mut partitions: metamodelica::Array<Arc<Partition::Partition::Partition>> = metamodelica::arrayFromVec(unsorted.clone().reverse().into_iter().cloned().collect());
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = arrayCreate(n.clone(), metamodelica::nil());
    let mut matching: Arc<Matching::NBMatching> = Matching::trivial(n.clone());
    let mut index_map: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, i32>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1);
    let mut partition_order: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut j: i32 = 0;
    for mut i in 1..=n.clone() {
        UnorderedMap::add((Partition::Partition::getClocks(partitions.borrow()[(i.clone()-1) as usize].clone())?).0, i.clone(), index_map.clone())?;
    }
    for mut i in 1..=n.clone() {
        let __range0 = &*UnorderedSet::toList(Partition::Partition::getClockDependencies(partitions.borrow()[(i.clone()-1) as usize].clone())?);
        for mut clock in __range0 {
            let mut clock = clock.clone();
            j = UnorderedMap::getSafe(clock.clone(), index_map.clone(), metamodelica::sourceInfo!())?;
            {
                let __cell1 = cons(j.clone(), m.borrow()[(i.clone()-1) as usize].clone());
                m.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
            }
        }
    }
    partition_order = Sorting::tarjanScalar(m.clone(), matching.clone())?;
    for mut comp in &*partition_order.clone().reverse() {
        let mut comp = comp.clone();
        sorted = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ metamodelica::List::Cons { head: j, tail: Deref @ metamodelica::List::Nil } => {
            cons(partitions.borrow()[(j.clone()-1) as usize].clone(), sorted.clone())
        },
        _ => {
            let mut var_clock_map: Arc<UnorderedMap::UnorderedMap<Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<BClock::BClock>>>;
            let mut part: Arc<Partition::Partition::Partition> = Arc::new(<Partition::Partition::Partition as ::std::default::Default>::default());
            let mut sub_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
            let mut sub_comp_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut sub_comp_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
            let mut collector: Option<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<BClock::BClock>)> = None;
            let mut var_clocks: Arc<UnorderedSet::UnorderedSet<Arc<BClock::BClock>>>;
            let mut baseClock: Option<Arc<BClock::BClock>> = None;
            let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
            let mut clock: Arc<BClock::BClock>;
            let mut new_clock: Arc<BClock::BClock>;
            var_clock_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 1);
            for mut i in &*comp.clone() {
                let mut i = i.clone();
                part = partitions.borrow()[(i.clone()-1) as usize].clone();
                for mut var in &*BVariable::VariablePointers::toList(part.unknowns.clone())? {
                    let mut var = var.clone();
                    UnorderedMap::add(var.clone(), (Partition::Partition::getClocks(part.clone())?).0, var_clock_map.clone())?;
                }
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            j = __pa0.clone();
            comp = __pa1.clone();
            part = partitions.borrow()[(j.clone()-1) as usize].clone();
            for mut i in &*comp.clone() {
                let mut i = i.clone();
                part = Partition::Partition::merge(part.clone(), partitions.borrow()[(i.clone()-1) as usize].clone(), false)?;
            }
            (_, baseClock, _) = Partition::Partition::getClocks(part.clone())?;
            (_, sub_comps) = Causalize::simple(part.unknowns.clone(), part.equations.clone(), Partition::Partition::getKind(part.clone()), NBAdjacency::MatrixStrictness::MATCHING.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY))?;
            collector = None;
            for mut sub_comp in &*sub_comps.clone().reverse() {
                let mut sub_comp = sub_comp.clone();
                sub_comp_vars = StrongComponent::getVariables(sub_comp.clone())?;
                sub_comp_eqns = StrongComponent::getEquations(sub_comp.clone())?;
                var_clocks = UnorderedSet::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 13);
                for mut var in &*sub_comp_vars.clone() {
                    let mut var = var.clone();
                    UnorderedSet::add(UnorderedMap::getSafe(var.clone(), var_clock_map.clone(), metamodelica::sourceInfo!())?, var_clocks.clone())?;
                }
                collector = (::match_deref::match_deref! { match &((collector.clone(), UnorderedSet::toList(var_clocks.clone()))) {
        (None, Deref @ metamodelica::List::Cons { head: new_clock, tail: Deref @ metamodelica::List::Nil }) => Some((sub_comp_vars.clone(), sub_comp_eqns.clone(), new_clock.clone())),
        (Some((vars, eqns, clock)), Deref @ metamodelica::List::Cons { head: new_clock, tail: Deref @ metamodelica::List::Nil }) => {
            if BClock::isEqual(clock.clone(), new_clock.clone())? {
                collector = Some((listAppend(sub_comp_vars.clone(), vars.clone()), listAppend(sub_comp_eqns.clone(), eqns.clone()), clock.clone()));
            } else {
                part = Arc::new(Partition::Partition::Partition { index: 0, association: Arc::new(Partition::Association::Association::CLOCKED { clock: clock.clone(), baseClock: baseClock.clone(), clock_deps: UnorderedSet::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 13), holdEvents: false }), unknowns: BVariable::VariablePointers::fromList(vars.clone(), false), daeUnknowns: None, equations: BEquation::EquationPointers::fromList(eqns.clone()), adjacencyMatrix: None, matching: None, strongComponents: None });
                sorted = cons(part.clone(), sorted.clone());
                collector = Some((sub_comp_vars.clone(), sub_comp_eqns.clone(), new_clock.clone()));
            }
            collector.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.sortClockedPartitions")); __mm_s.push_str(&*literal!(" failed for sub-partitions with cyclic dependency that could not be resolved:\n")); __mm_s.push_str(&*literal!("There are contradicting sub-clocks: ")); __mm_s.push_str(&*List::toString(UnorderedSet::toList(var_clocks.clone()), (std::sync::Arc::new(BClock::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!(" in strong component:\n")); __mm_s.push_str(&*StrongComponent::toString(sub_comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            if Util::isSome(collector.clone()) {
                let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(collector.clone()) {
                    Some((__pa2, __pa3, __pa4)) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                vars = __pa2.clone();
                eqns = __pa3.clone();
                clock = __pa4.clone();
                part = Arc::new(Partition::Partition::Partition { index: 0, association: Arc::new(Partition::Association::Association::CLOCKED { clock: clock.clone(), baseClock: baseClock.clone(), clock_deps: UnorderedSet::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 13), holdEvents: false }), unknowns: BVariable::VariablePointers::fromList(vars.clone(), false), daeUnknowns: None, equations: BEquation::EquationPointers::fromList(eqns.clone()), adjacencyMatrix: None, matching: None, strongComponents: None });
                sorted = cons(part.clone(), sorted.clone());
            }
            sorted.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(sorted)
}

fn collectPartitioningCrefs(mut exp: Arc<Expression::NFExpression>, mut var_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut newExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            newExp = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?) {
        Deref @ "subSample" => exp.clone(),
        Deref @ "superSample" => exp.clone(),
        Deref @ "shiftSample" => exp.clone(),
        Deref @ "backSample" => exp.clone(),
        Deref @ "previous" => exp.clone(),
        Deref @ "hold" => exp.clone(),
        Deref @ "sample" => {
            arg = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil } } => arg.clone(),
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil } } } => arg.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.collectPartitioningCrefs")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Expression::mapShallow(arg.clone(), Arc::new({ let __pe_b1 = var_crefs.clone(); move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }))?
        },
        _ => Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = var_crefs.clone(); move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            newExp.clone()
        },
        Deref @ Expression::CREF { .. } => {
            let mut children: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut stripped: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            children = (::match_deref::match_deref! { match &(BVariable::getVar(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::RECORD { children: children_vars, .. }, .. }, .. } => {
            {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (children_vars.clone()).into_iter().cloned() {
            let __x = BVariable::getVarName(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        _ => {
            list![var_field!((*exp).cref, Expression::NFExpression::CREF).clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            for mut child in &*children.clone() {
                let mut child = child.clone();
                stripped = ComponentRef::stripSubscriptsAll(child.clone());
                if !(BVariable::checkCref(stripped.clone(), (std::sync::Arc::new(fnptr!(BVariable::isParamOrConst, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!())) {
                    addCrefToSet(stripped.clone(), var_crefs.clone())?;
                }
            }
            exp.clone()
        },
        _ => {
            Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = var_crefs.clone(); move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn addCrefToSet(mut cref: Arc<ComponentRef::NFComponentRef>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
    if BVariable::isState(var_ptr.clone()) {
        UnorderedSet::add(BVariable::getPartnerCref(cref.clone(), (std::sync::Arc::new(fnptr!(BVariable::getVarDer, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>), false)?, set.clone())?;
    } else if BVariable::isPrevious(var_ptr.clone()) {
        UnorderedSet::add(BVariable::getPartnerCref(cref.clone(), (std::sync::Arc::new(fnptr!(BVariable::getVarPre, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>), false)?, set.clone())?;
    } else {
        UnorderedSet::add(cref.clone(), set.clone())?;
    }
    Ok(())
}

fn replaceClockedFunctions(mut exp: Arc<Expression::NFExpression>, mut held_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    pub fn replaceSample(mut exp: Arc<Expression::NFExpression>, mut call: Arc<Call::NFCall>, mut basic: bool) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((::match_deref::match_deref! { match &(Call::arguments(call.clone())?) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => list![arg1.clone(), arg2.clone()],
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } } if (basic.clone()) => list![arg1.clone(), arg2.clone()],
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } => list![arg1.clone(), arg2.clone()],
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctions.replaceSample")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg1 = __pa0.clone();
        arg2 = __pa1.clone();
        if basic.clone() {
            exp = if (Type::isClock(Expression::typeOf(arg2.clone()))) {replaceClockedFunctionExp(arg1.clone())?} else {exp.clone()};
        } else {
            exp = replaceClockedFunctionExp(arg1.clone())?;
        }
        Ok(exp)
    }

    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut newExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            newExp = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?) {
        Deref @ "sample" => replaceSample(exp.clone(), call.clone(), true)?,
        Deref @ "subSample" => replaceSample(exp.clone(), call.clone(), false)?,
        Deref @ "superSample" => replaceSample(exp.clone(), call.clone(), false)?,
        Deref @ "shiftSample" => replaceSample(exp.clone(), call.clone(), false)?,
        Deref @ "backSample" => replaceSample(exp.clone(), call.clone(), false)?,
        Deref @ "hold" => {
            arg = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: arg @ Deref @ Expression::CREF { .. }, tail: Deref @ metamodelica::List::Nil } => {
            UnorderedSet::add(var_field!((**arg).cref, Expression::NFExpression::CREF).clone(), held_crefs.clone())?;
            arg.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctions")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            replaceClockedFunctionExp(arg.clone())?
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            newExp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn replaceClockedFunctionExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut func: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    func = (::match_deref::match_deref! { match &(Expression::typeOf(exp.clone())) {
        Deref @ Type::REAL => NFBuiltinFuncs::GET_PART_REAL().clone(),
        Deref @ Type::INTEGER => NFBuiltinFuncs::GET_PART_INT().clone(),
        Deref @ Type::BOOLEAN => NFBuiltinFuncs::GET_PART_BOOL().clone(),
        Deref @ Type::CLOCK => NFBuiltinFuncs::GET_PART_CLOCK().clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctionExp")); __mm_s.push_str(&*literal!(" failed. ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(" is of type ")); __mm_s.push_str(&*Type::toString(Expression::typeOf(exp.clone()))?); __mm_s.push_str(&*literal!(", only real, integer, boolean and clock are allowed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(func.clone(), list![exp.clone()], Expression::variability(exp.clone())?, NFPrefixes::Purity::PURE.clone(), func.returnType.clone()) });
    Ok(exp)
}

fn replaceClockedWhen(mut eqn: Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::WHEN_EQUATION { body: Deref @ BEquation::WhenEquationBody::WHEN_EQUATION_BODY { else_when: None, when_stmts: Deref @ metamodelica::List::Cons { head: stmt, tail: Deref @ metamodelica::List::Nil }, condition: cond }, .. } if (Type::isClock(Expression::typeOf(cond.clone()))) => {
            BEquation::WhenStatement::toEquation(stmt.clone(), var_field!((*eqn).attr, Equation::Equation::WHEN_EQUATION).clone(), false)?
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

