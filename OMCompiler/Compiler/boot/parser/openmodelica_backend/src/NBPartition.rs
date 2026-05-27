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
use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBJacobian as BJacobian;
use crate::NBMatching as Matching;
use crate::NBPartitioning::BClock;
use crate::NBPartitioning::ClockedInfo;
use crate::NBPartitioning;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as Jacobian;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// Backend Imports
// Util imports
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Kind {
    ODE = 1,
    ALG = 2,
    ODE_EVT = 3,
    ALG_EVT = 4,
    INI = 5,
    INI_0 = 6,
    DAE = 7,
    JAC = 8,
    CLK = 9,
}
impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for Kind {
    fn default() -> Self { Self::ODE }
}

pub mod Association {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Association {
        CONTINUOUS {
            kind: Kind,
            /// Analytic jacobian for the integrator
            jacobian: Option<Arc<Jacobian::NBackendDAE>>,
            /// Analytic adjoint jacobian for the integrator
            jacobianAdjoint: Option<Arc<Jacobian::NBackendDAE>>,
            /// Analytic jacobian of Lagrange term (L), ODE (f), Path Constraints (g) for MOO
            LFG_jacobian: Option<Arc<Jacobian::NBackendDAE>>,
            /// Analytic jacobian of Mayer term (Mf), Final Constraints (rf) for MOO
            MRF_jacobian: Option<Arc<Jacobian::NBackendDAE>>,
            /// Analytic jacobian of Initial Constraints (r0) for MOO
            R0_jacobian: Option<Arc<Jacobian::NBackendDAE>>,
        },
        CLOCKED {
            clock: Arc<BClock::BClock>,
            baseClock: Option<Arc<BClock::BClock>>,
            /// dependencies of this clocked partition
            clock_deps: Arc<UnorderedSet::UnorderedSet<Arc<BClock::BClock>>>,
            holdEvents: bool,
        },
    }
    impl Default for Association {
        fn default() -> Self {
            Self::CLOCKED {
                clock: Default::default(),
                baseClock: Default::default(),
                clock_deps: Default::default(),
                holdEvents: Default::default(),
            }
        }
    }
    pub use self::Association::{CONTINUOUS,CLOCKED};
    pub fn toStringShort(mut association: Arc<Association>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(association.clone()) {
        Deref @ CONTINUOUS { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Continuous ")); __mm_s.push_str(&*Partition::kindToString(var_field!((*association).kind, Association::CONTINUOUS).clone())?); ArcStr::from(__mm_s) },
        Deref @ CLOCKED { .. } => literal!("Clocked"),
        _ => literal!("Unknown"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn toString(mut association: Arc<Association>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(association.clone()) {
        Deref @ CONTINUOUS { .. } => {
            if Util::isSome(var_field!((*association).jacobian, Association::CONTINUOUS).clone()) {
                r#str = (BJacobian::toString(Util::getOption(var_field!((*association).jacobian, Association::CONTINUOUS).clone())?, (Partition::kindToString(var_field!((*association).kind, Association::CONTINUOUS).clone())?).clone())?).clone();
                if Flags::getConfigBool(Flags::MOO_DYNAMIC_OPTIMIZATION.clone())? {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*BJacobian::toString(Util::getOption(var_field!((*association).LFG_jacobian, Association::CONTINUOUS).clone())?, (Partition::kindToString(var_field!((*association).kind, Association::CONTINUOUS).clone())?).clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*BJacobian::toString(Util::getOption(var_field!((*association).MRF_jacobian, Association::CONTINUOUS).clone())?, (Partition::kindToString(var_field!((*association).kind, Association::CONTINUOUS).clone())?).clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*BJacobian::toString(Util::getOption(var_field!((*association).R0_jacobian, Association::CONTINUOUS).clone())?, (Partition::kindToString(var_field!((*association).kind, Association::CONTINUOUS).clone())?).clone())?); ArcStr::from(__mm_s) }).clone();
                }
            } else {
                r#str = (StringUtil::headline_1((literal!("No Jacobian")).clone())).clone();
            }
            if Util::isSome(var_field!((*association).jacobianAdjoint, Association::CONTINUOUS).clone()) {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BJacobian::toString(Util::getOption(var_field!((*association).jacobianAdjoint, Association::CONTINUOUS).clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Partition::kindToString(var_field!((*association).kind, Association::CONTINUOUS).clone())?); __mm_s.push_str(&*literal!(" Adjoint")); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        Deref @ CLOCKED { .. } => {
            r#str = (BClock::toString(var_field!((*association).clock, Association::CLOCKED).clone())?).clone();
            if Util::isSome(var_field!((*association).baseClock, Association::CLOCKED).clone()) {
                r#str = (StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Sub clock: ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" of base clock ")); __mm_s.push_str(&*BClock::toString(Util::getOption(var_field!((*association).baseClock, Association::CLOCKED).clone())?)?); ArcStr::from(__mm_s) }).clone())).clone();
            } else {
                r#str = (StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Base clock: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())).clone();
            }
            r#str.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Association.toString")); __mm_s.push_str(&*literal!(" failed. Unknown partition association in match.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn create(mut equations: Arc<EquationPointers::EquationPointers>, mut kind: Kind, mut info: Arc<ClockedInfo::ClockedInfo>, mut infer_del: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Association>> {
        let mut association: Arc<Association>;
        let mut clock_ptr: Pointer::Pointer<Option<(Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>)>> = Pointer::create(None);
        let mut infer_ptr: Pointer::Pointer<Option<Arc<ComponentRef::NFComponentRef>>> = Pointer::create(None);
        let mut failed_set: Arc<UnorderedSet::UnorderedSet<(Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>)>> = UnorderedSet::new((std::sync::Arc::new(hashClockTpl) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>)) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(isEqualClockTpl, (Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>), (Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>), (Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>)) -> Result<bool> + 'static>), 13);
        let mut clock_deps: Arc<UnorderedSet::UnorderedSet<Arc<BClock::BClock>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 13);
        let mut clock_tpl: Option<(Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>)> = None;
        let mut infer: Option<Arc<ComponentRef::NFComponentRef>> = None;
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut base_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut clock: Arc<BClock::BClock>;
        BEquation::EquationPointers::mapExp(equations.clone(), Arc::new({ let __pe_b1 = info.clone(); let __pe_b2 = clock_ptr.clone(); let __pe_b3 = infer_ptr.clone(); let __pe_b4 = failed_set.clone(); let __pe_b5 = clock_deps.clone(); let __pe_b6 = infer_del.clone(); move |__pe_a0| expClocked(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }), None, (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        clock_tpl = Pointer::access(clock_ptr.clone());
        infer = Pointer::access(infer_ptr.clone());
        if Util::isSome(clock_tpl.clone()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(clock_tpl.clone()) {
                Some((__pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            clock = __pa1.clone();
            if !(UnorderedSet::isEmpty(failed_set.clone())) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Association.create")); __mm_s.push_str(&*literal!(" failed because there are non-identical clocks in the same partition:\n")); __mm_s.push_str(&*literal!("### First clock found:\n")); __mm_s.push_str(&*clockTplString((name.clone(), clock.clone()))); __mm_s.push_str(&*literal!("\n### Conflicting clocks:\n")); __mm_s.push_str(&*UnorderedSet::toString(failed_set.clone(), (std::sync::Arc::new(fnptr!(clockTplString, (Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>)) -> Result<ArcStr> + 'static>), (literal!("\n")).clone())); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            if BClock::isBaseClock(clock.clone()) {
                if BClock::isInferredClock(clock.clone()) {
                    if Util::isNone(infer.clone()) {
                        clock = NBPartitioning::DEFAULT_BASE_CLOCK().clone();
                        UnorderedMap::add(name.clone(), clock.clone(), info.baseClocks.clone())?;
                    } else {
                        clock = Arc::new(BClock::BClock::INFERRED_CLOCK { base_ref: Util::getOption(infer.clone())? });
                    }
                }
                association = Arc::new(Association::CLOCKED { clock: clock.clone(), baseClock: None, clock_deps: clock_deps.clone(), holdEvents: false });
            } else {
                base_name = UnorderedMap::getSafe(name.clone(), info.subToBase.clone(), metamodelica::sourceInfo!())?;
                association = Arc::new(Association::CLOCKED { clock: clock.clone(), baseClock: Some(UnorderedMap::getSafe(base_name.clone(), info.baseClocks.clone(), metamodelica::sourceInfo!())?), clock_deps: clock_deps.clone(), holdEvents: false });
            }
        } else {
            association = Arc::new(Association::CONTINUOUS { kind: kind.clone(), jacobian: None, jacobianAdjoint: None, LFG_jacobian: None, MRF_jacobian: None, R0_jacobian: None });
        }
        Ok(association)
    }

    pub fn merge(mut ass1: Arc<Association>, mut ass2: Arc<Association>, mut strict: bool) -> Result<Arc<Association>> {
        let mut ass1: Arc<Association> = ass1;
        ass1 = (::match_deref::match_deref! { match &((ass1.clone(), ass2.clone())) {
        (Deref @ CONTINUOUS { jacobian: Some(jac1 @ Deref @ Jacobian::JACOBIAN { .. }), .. }, Deref @ CONTINUOUS { jacobian: Some(jac2), .. }) if (var_field!((*ass1).kind, Association::CONTINUOUS).clone() == var_field!((*ass2).kind, Association::CONTINUOUS).clone() || !(strict.clone())) => {
            assign_variant_field!(ass1 => Association::CONTINUOUS; jacobian = Some(BJacobian::combine(list![jac1.clone(), jac2.clone()], (var_field!((**jac1).name, Jacobian::NBackendDAE::JACOBIAN).clone()).clone())?));
            ass1.clone()
        },
        (Deref @ CONTINUOUS { .. }, Deref @ CONTINUOUS { .. }) if (var_field!((*ass1).kind, Association::CONTINUOUS).clone() == var_field!((*ass2).kind, Association::CONTINUOUS).clone() || !(strict.clone())) => {
            ass1.clone()
        },
        (Deref @ CLOCKED { .. }, Deref @ CLOCKED { .. }) if (!(strict.clone()) || BClock::isEqual(var_field!((*ass1).clock, Association::CLOCKED).clone(), var_field!((*ass2).clock, Association::CLOCKED).clone())? && Util::optionEqual(var_field!((*ass1).baseClock, Association::CLOCKED).clone(), var_field!((*ass2).baseClock, Association::CLOCKED).clone(), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>))) => {
            assign_variant_field!(ass1 => Association::CLOCKED;
                clock_deps = UnorderedSet::union(var_field!((*ass1).clock_deps, Association::CLOCKED).clone(), var_field!((*ass2).clock_deps, Association::CLOCKED).clone())?,
                holdEvents = var_field!((*ass1).holdEvents, Association::CLOCKED).clone() || var_field!((*ass2).holdEvents, Association::CLOCKED).clone()
            );
            ass1.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Association.merge")); __mm_s.push_str(&*literal!(" failed. Cannot merge\n")); __mm_s.push_str(&*toString(ass1.clone())?); __mm_s.push_str(&*literal!(" and\n")); __mm_s.push_str(&*toString(ass2.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(ass1)
    }

    pub fn isClocked(mut association: Arc<Association>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(association.clone()) {
        Deref @ CLOCKED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub type ClockTpl = (Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>);

    pub fn clockTplString(mut tpl: ClockTpl) -> ArcStr {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentRef::toString(Util::tuple21(tpl.clone())).unwrap()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*BClock::toString(Util::tuple22(tpl.clone())).unwrap()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
        r#str
    }

    pub fn hashClockTpl(mut tpl: ClockTpl) -> Result<i32> {
        let mut hash: i32 = 0;
        hash = ComponentRef::hash(Util::tuple21(tpl.clone()));
        hash = stringHashDjb2Continue((BClock::toString(Util::tuple22(tpl.clone()))?).clone(), hash.clone());
        Ok(hash)
    }

    pub fn isEqualClockTpl(mut tpl1: ClockTpl, mut tpl2: ClockTpl) -> bool {
        let mut b: bool = ComponentRef::isEqual(Util::tuple21(tpl1.clone()), Util::tuple21(tpl2.clone())).unwrap() && BClock::isEqual(Util::tuple22(tpl1.clone()), Util::tuple22(tpl2.clone())).unwrap();
        b
    }

    fn expClocked(mut exp: Arc<Expression::NFExpression>, mut info: Arc<ClockedInfo::ClockedInfo>, mut clock_ptr: Pointer::Pointer<Option<(Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>)>>, mut infer_ptr: Pointer::Pointer<Option<Arc<ComponentRef::NFComponentRef>>>, mut failed_set: Arc<UnorderedSet::UnorderedSet<(Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>)>>, mut clock_deps: Arc<UnorderedSet::UnorderedSet<Arc<BClock::BClock>>>, mut infer_del: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (BVariable::isClockOrClocked(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?)) => {
            let mut clock_opt: Option<Arc<BClock::BClock>> = None;
            if UnorderedMap::contains(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), info.baseClocks.clone()) {
                clock_opt = Some(UnorderedMap::getSafe(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), info.baseClocks.clone(), metamodelica::sourceInfo!())?);
            } else if UnorderedMap::contains(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), info.subClocks.clone()) {
                clock_opt = Some(UnorderedMap::getSafe(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), info.subClocks.clone(), metamodelica::sourceInfo!())?);
            } else {
                clock_opt = None;
            }
            let _ = (::match_deref::match_deref! { match &((clock_opt.clone(), Pointer::access(clock_ptr.clone()))) {
        (Some(Deref @ BClock::BASE_CLOCK { .. }), Some((name, Deref @ BClock::SUB_CLOCK { .. }))) => {
            removeInferredClock(name.clone(), var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), info.clone(), infer_del.clone())?;
            ()
        },
        (Some(new @ Deref @ BClock::SUB_CLOCK { .. }), Some((_, Deref @ BClock::BASE_CLOCK { .. }))) => {
            Pointer::update(clock_ptr.clone(), Some((var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), new.clone())));
            ()
        },
        (Some(new), Some((_, Deref @ BClock::BASE_CLOCK { clock: Deref @ ClockKind::INFERRED_CLOCK { .. } }))) => {
            Pointer::update(clock_ptr.clone(), Some((var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), new.clone())));
            ()
        },
        (Some(new), Some((_, old))) => {
            if BClock::isInferredClock(old.clone()) {
                Pointer::update(clock_ptr.clone(), Some((var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), new.clone())));
            } else if !(BClock::isInferredClock(new.clone()) || BClock::isEqual(new.clone(), old.clone())?) {
                UnorderedSet::add((var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), new.clone()), failed_set.clone())?;
            }
            ()
        },
        (Some(new), None) => {
            Pointer::update(clock_ptr.clone(), Some((var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), new.clone())));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: arg, .. }, tail: _ }, .. } } if (Expression::isClockOrSampleFunction(exp.clone())?) => {
            if UnorderedMap::contains(arg.clone(), info.subClocks.clone()) {
                UnorderedSet::add(UnorderedMap::getSafe(arg.clone(), info.subClocks.clone(), metamodelica::sourceInfo!())?, clock_deps.clone())?;
                Pointer::update(infer_ptr.clone(), Some(arg.clone()));
            }
            exp.clone()
        },
        _ => {
            Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = info.clone(); let __pe_b2 = clock_ptr.clone(); let __pe_b3 = infer_ptr.clone(); let __pe_b4 = failed_set.clone(); let __pe_b5 = clock_deps.clone(); let __pe_b6 = infer_del.clone(); move |__pe_a0| expClocked(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    fn removeInferredClock(mut name: Arc<ComponentRef::NFComponentRef>, mut new_name: Arc<ComponentRef::NFComponentRef>, mut info: Arc<ClockedInfo::ClockedInfo>, mut infer_del: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        let mut base: Arc<BClock::BClock>;
        let mut base_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut sub_clock_names1: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut sub_clock_names2: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        base_name = UnorderedMap::getSafe(name.clone(), info.subToBase.clone(), metamodelica::sourceInfo!())?;
        base = UnorderedMap::getSafe(base_name.clone(), info.baseClocks.clone(), metamodelica::sourceInfo!())?;
        if BClock::isInferredClock(base.clone()) {
            sub_clock_names1 = UnorderedMap::getSafe(base_name.clone(), info.baseToSub.clone(), metamodelica::sourceInfo!())?;
            for mut s_name in &*sub_clock_names1.clone() {
                let mut s_name = s_name.clone();
                UnorderedMap::add(s_name.clone(), new_name.clone(), info.subToBase.clone())?;
            }
            sub_clock_names2 = UnorderedMap::getOrDefault(new_name.clone(), info.baseToSub.clone(), metamodelica::nil());
            UnorderedMap::add(new_name.clone(), listAppend(sub_clock_names1.clone(), sub_clock_names2.clone()), info.baseToSub.clone())?;
            UnorderedSet::add(base_name.clone(), infer_del.clone())?;
        }
        Ok(())
    }

}

pub mod Partition {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Partition {
        /// Partition index
        pub index: i32,
        /// Clocked/Continuous
        pub association: Arc<Association::Association>,
        /// Variable array of unknowns, subset of full variable array
        pub unknowns: Arc<VariablePointers::VariablePointers>,
        /// Variable array of unknowns in the case of dae mode
        pub daeUnknowns: Option<Arc<VariablePointers::VariablePointers>>,
        /// Equations array, subset of the full equation array
        pub equations: Arc<EquationPointers::EquationPointers>,
        /// Adjacency matrix with all additional information
        pub adjacencyMatrix: Option<Arc<Adjacency::Matrix::Matrix>>,
        /// Matching (see 2.5)
        pub matching: Option<Arc<Matching::NBMatching>>,
        /// Strong Components
        pub strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>,
    }

    impl Default for Partition {
        fn default() -> Self {
            Self {
                index: Default::default(),
                association: Default::default(),
                unknowns: Default::default(),
                daeUnknowns: Default::default(),
                equations: Default::default(),
                adjacencyMatrix: Default::default(),
                matching: Default::default(),
                strongComponents: Default::default(),
            }
        }
    }

    pub type PARTITION = Partition;

    pub fn toString(mut partition: Arc<Partition>, mut level: i32) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(partition.index.clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*Association::toStringShort(partition.association.clone())?); __mm_s.push_str(&*literal!(" Partition")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str = ((match partition.strongComponents.clone() {
        Some(mut comps) => {
            let __range0 = 1..=(comps.clone().borrow().len() as i32);
            for mut i in __range0 {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StrongComponent::toString(comps.borrow()[(i.clone()-1) as usize].clone(), i.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        _ => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*BVariable::VariablePointers::toString(partition.unknowns.clone(), (literal!("Unknown")).clone(), None, true)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BEquation::EquationPointers::toString(partition.equations.clone(), (literal!("")).clone(), None, true, None)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
    })).clone();
        if level.clone() == 1 || level.clone() == 3 {
            if isSome(partition.adjacencyMatrix.clone()) {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Adjacency::Matrix::toString(Util::getOption(partition.adjacencyMatrix.clone())?, (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            if isSome(partition.matching.clone()) {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Matching::toString(Util::getOption(partition.matching.clone())?, (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
        }
        if level.clone() == 2 {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Association::toString(partition.association.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn toStringList(mut partitions: Arc<metamodelica::List<Arc<Partition>>>, mut header: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        if !(partitions.clone().is_empty()) {
            if header.clone() != literal!("") {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1((header.clone()).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            for mut part in &*partitions.clone() {
                let mut part = part.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toString(part.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
            }
        }
        Ok(r#str)
    }

    pub fn sort(mut partition: Arc<Partition>) -> Result<Arc<Partition>> {
        let mut partition: Arc<Partition> = partition;
        assign_field!(
            partition.unknowns = BVariable::VariablePointers::sort(partition.unknowns.clone())?,
            partition.equations = BEquation::EquationPointers::sort(partition.equations.clone())?
        );
        Ok(partition)
    }

    pub fn isEmpty(mut partition: Arc<Partition>) -> bool {
        use arrayEmpty as isEmptyArr;

        let mut b: bool = BEquation::EquationPointers::size(partition.equations.clone()) == 0 || Util::applyOptionOrDefault(partition.strongComponents.clone(), isEmptyArr, false);
        b
    }

    pub fn isODEorDAE(mut part: Arc<Partition>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(part.association.clone()) {
        Deref @ Association::CONTINUOUS { kind, .. } => {
            kind.clone() == Kind::ODE.clone() || kind.clone() == Kind::ODE_EVT.clone() || kind.clone() == Kind::DAE.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isClocked(mut part: Arc<Partition>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(part.association.clone()) {
        Deref @ Association::CLOCKED { .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn categorize(mut partition: Arc<Partition>, mut ode: DoubleEnded::MutableList<Arc<Partition>>, mut alg: DoubleEnded::MutableList<Arc<Partition>>, mut ode_evt: DoubleEnded::MutableList<Arc<Partition>>, mut alg_evt: DoubleEnded::MutableList<Arc<Partition>>, mut clocked: DoubleEnded::MutableList<Arc<Partition>>) -> Result<()> {
        fn isAlgebraicContinuous(mut part: Arc<Partition>) -> (bool, bool) {
            let mut alg: bool = true;
            let mut con: bool = true;
            for mut var in &*BVariable::VariablePointers::toList(part.unknowns.clone()).unwrap() {
                let mut var = var.clone();
                alg = if (alg.clone()) {!(BVariable::isStateDerivative(var.clone()))} else {false};
                con = if (con.clone()) {!(BVariable::isDiscrete(var.clone()))} else {false};
                if !(alg.clone() || con.clone()) {
                    break;
                }
            }
            (alg, con)
        }

        let mut algebraic: bool = false;
        let mut continuous: bool = false;
        let mut kind: Kind = Kind::ODE;
        let mut association: Arc<Association::Association>;
        (algebraic, continuous) = isAlgebraicContinuous(partition.clone());
        kind = (match (algebraic.clone(), continuous.clone()) {
        (true, true) => Kind::ALG.clone(),
        (false, true) => Kind::ODE.clone(),
        (true, false) => Kind::ALG_EVT.clone(),
        (false, false) => Kind::ODE_EVT.clone(),
        _ => bail!("fail"),
    });
        assign_field!(partition.association = (::match_deref::match_deref! { match &((kind.clone(), partition.association.clone())) {
        (_, Deref @ Association::CLOCKED { .. }) => {
            DoubleEnded::push_back(clocked.clone(), partition.clone());
            partition.association.clone()
        },
        (Kind::ALG, association @ Deref @ Association::CONTINUOUS { .. }) => {
            let mut association = (*association).clone();
            assign_variant_field!(association => Association::Association::CONTINUOUS; kind = kind.clone());
            assign_field!(partition.association = association.clone());
            DoubleEnded::push_back(alg.clone(), partition.clone());
            association.clone()
        },
        (Kind::ODE, association @ Deref @ Association::CONTINUOUS { .. }) => {
            let mut association = (*association).clone();
            assign_variant_field!(association => Association::Association::CONTINUOUS; kind = kind.clone());
            assign_field!(partition.association = association.clone());
            DoubleEnded::push_back(ode.clone(), partition.clone());
            association.clone()
        },
        (Kind::ALG_EVT, association @ Deref @ Association::CONTINUOUS { .. }) => {
            let mut association = (*association).clone();
            assign_variant_field!(association => Association::Association::CONTINUOUS; kind = kind.clone());
            assign_field!(partition.association = association.clone());
            DoubleEnded::push_back(alg_evt.clone(), partition.clone());
            association.clone()
        },
        (Kind::ODE_EVT, association @ Deref @ Association::CONTINUOUS { .. }) => {
            let mut association = (*association).clone();
            assign_variant_field!(association => Association::Association::CONTINUOUS; kind = kind.clone());
            assign_field!(partition.association = association.clone());
            DoubleEnded::push_back(ode_evt.clone(), partition.clone());
            association.clone()
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }));
        Ok(())
    }

    pub fn setIndex(mut part: Arc<Partition>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Partition>> {
        let mut part: Arc<Partition> = part;
        let mut clock_idx: i32 = Pointer::access(index.clone());
        assign_field!(part.index = clock_idx.clone());
        if isClocked(part.clone()) {
            assign_field!(part.equations = BEquation::EquationPointers::map(part.equations.clone(), Arc::new({ let __pe_b1 = EquationKind::CLOCKED.clone(); let __pe_b2 = Some(clock_idx.clone()); move |__pe_a0| Equation::setKind(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?);
        }
        Pointer::update(index.clone(), clock_idx.clone() + 1);
        Ok(part)
    }

    pub fn setKind(mut part: Arc<Partition>, mut kind: Kind) -> Result<Arc<Partition>> {
        let mut part: Arc<Partition> = part;
        assign_field!(part.association = (::match_deref::match_deref! { match &(part.association.clone()) {
        ass @ Deref @ Association::CONTINUOUS { .. } => {
            let mut ass = (*ass).clone();
            assign_variant_field!(ass => Association::Association::CONTINUOUS; kind = kind.clone());
            ass.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.setKind")); __mm_s.push_str(&*literal!(" failed. Cannot set kind for non-continuous partition:\n")); __mm_s.push_str(&*toString(part.clone(), 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }));
        Ok(part)
    }

    pub fn getJacobian(mut part: Arc<Partition>) -> Option<Arc<Jacobian::NBackendDAE>> {
        let mut jac: Option<Arc<Jacobian::NBackendDAE>> = None;
        jac = (::match_deref::match_deref! { match &(part.association.clone()) {
        Deref @ Association::CONTINUOUS { jacobian: jac, .. } => jac.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        jac
    }

    pub fn getJacobianAdjoint(mut part: Arc<Partition>) -> Option<Arc<Jacobian::NBackendDAE>> {
        let mut jac: Option<Arc<Jacobian::NBackendDAE>> = None;
        jac = (::match_deref::match_deref! { match &(part.association.clone()) {
        Deref @ Association::CONTINUOUS { jacobianAdjoint: jac, .. } => jac.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        jac
    }

    pub fn getKind(mut part: Arc<Partition>) -> Kind {
        let mut kind: Kind = Kind::ODE;
        kind = (::match_deref::match_deref! { match &(part.association.clone()) {
        Deref @ Association::CONTINUOUS { kind, .. } => kind.clone(),
        _ => Kind::CLK.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        kind
    }

    pub fn getClocks(mut part: Arc<Partition>) -> Result<(Arc<BClock::BClock>, Option<Arc<BClock::BClock>>, bool)> {
        let mut clock: Arc<BClock::BClock>;
        let mut baseClock: Option<Arc<BClock::BClock>> = None;
        let mut holdEvents: bool = false;
        (clock, baseClock, holdEvents) = (::match_deref::match_deref! { match &(part.association.clone()) {
        Deref @ Association::CLOCKED { holdEvents, baseClock, clock, .. } => (clock.clone(), baseClock.clone(), holdEvents.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.getClocks")); __mm_s.push_str(&*literal!(" failed. Cannot get clocks for continuous partition:\n")); __mm_s.push_str(&*toString(part.clone(), 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((clock, baseClock, holdEvents))
    }

    pub fn setClocks(mut part: Arc<Partition>, mut clock: Arc<BClock::BClock>, mut baseClock: Option<Arc<BClock::BClock>>) -> Result<Arc<Partition>> {
        let mut part: Arc<Partition> = part;
        part = (::match_deref::match_deref! { match &(part.association.clone()) {
        association @ Deref @ Association::CLOCKED { .. } => {
            let mut association = (*association).clone();
            assign_variant_field!(association => Association::Association::CLOCKED;
                clock = clock.clone(),
                baseClock = baseClock.clone()
            );
            assign_field!(part.association = association.clone());
            part.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.setClocks")); __mm_s.push_str(&*literal!(" failed. Cannot set clocks for continuous partition:\n")); __mm_s.push_str(&*toString(part.clone(), 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(part)
    }

    pub fn getClockDependencies(mut part: Arc<Partition>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<BClock::BClock>>>> {
        let mut clock_deps: Arc<UnorderedSet::UnorderedSet<Arc<BClock::BClock>>>;
        clock_deps = (::match_deref::match_deref! { match &(part.association.clone()) {
        Deref @ Association::CLOCKED { clock_deps, .. } => clock_deps.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.getClockDependencies")); __mm_s.push_str(&*literal!(" failed. Cannot get clock dependencies for continuous partition:\n")); __mm_s.push_str(&*toString(part.clone(), 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(clock_deps)
    }

    pub fn getLoopResiduals(mut part: Arc<Partition>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
        let mut residuals: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        if Util::isSome(part.strongComponents.clone()) {
            let __range0 = Util::getOption(part.strongComponents.clone()).unwrap().borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                residuals = listAppend(StrongComponent::getLoopResiduals(comp.clone()), residuals.clone());
            }
        }
        residuals
    }

    pub fn mapEqn(mut partition: Arc<Partition>, mut func: Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>) -> Result<Arc<Partition>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>;

        let mut partition: Arc<Partition> = partition;
        assign_field!(partition.equations = BEquation::EquationPointers::map(partition.equations.clone(), func.clone())?);
        Ok(partition)
    }

    pub fn mapExp(mut partition: Arc<Partition>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Partition>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

        let mut partition: Arc<Partition> = partition;
        assign_field!(partition.equations = BEquation::EquationPointers::mapExp(partition.equations.clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?);
        Ok(partition)
    }

    pub fn mapStrongComponents(mut partition: Arc<Partition>, mut func: Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>) -> Result<Arc<StrongComponent::NBStrongComponent>> + 'static>) -> Result<Arc<Partition>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>) -> Result<Arc<StrongComponent::NBStrongComponent>> + 'static>;

        let mut partition: Arc<Partition> = partition;
        let mut comps: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>;
        if Util::isSome(partition.strongComponents.clone()) {
            let Some(__pa0) = (partition.strongComponents.clone()) else { bail!("pattern mismatch") };
            comps = __pa0.clone();
            let __range1 = 1..=(comps.clone().borrow().len() as i32);
            for mut i in __range1 {
                {
                    let __cell2 = func(comps.borrow()[(i.clone()-1) as usize].clone())?;
                    comps.clone().borrow_mut()[(i.clone()-1) as usize] = __cell2;
                }
            }
            assign_field!(partition.strongComponents = Some(comps.clone()));
        }
        Ok(partition)
    }

    pub fn kindToString(mut kind: Kind) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = ((match kind.clone() {
        Kind::ODE => literal!("ODE"),
        Kind::ALG => literal!("ALG"),
        Kind::ODE_EVT => literal!("ODE_EVT"),
        Kind::ALG_EVT => literal!("ALG_EVT"),
        Kind::INI => literal!("INI"),
        Kind::INI_0 => literal!("INI_0"),
        Kind::DAE { .. } => literal!("DAE"),
        Kind::JAC => literal!("JAC"),
        Kind::CLK => literal!("CLK"),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.kindToString")); __mm_s.push_str(&*literal!(" failed. Unknown partition kind in match.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    })).clone();
        Ok(r#str)
    }

    pub fn kindToInteger(mut kind: Kind) -> Result<i32> {
        let mut i: i32 = 0;
        i = (match kind.clone() {
        Kind::ODE => 0,
        Kind::ALG => 1,
        Kind::ODE_EVT => 2,
        Kind::ALG_EVT => 3,
        Kind::INI => 4,
        Kind::INI_0 => 5,
        Kind::DAE { .. } => 6,
        Kind::JAC => 7,
        Kind::CLK => 8,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.kindToInteger")); __mm_s.push_str(&*literal!(" failed. Unknown partition kind in match.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
        Ok(i)
    }

    pub fn clone(mut par: Arc<Partition>, mut shallow: bool) -> Result<Arc<Partition>> {
        let mut par: Arc<Partition> = par;
        assign_field!(par.equations = BEquation::EquationPointers::clone(par.equations.clone(), shallow.clone())?);
        if !(shallow.clone()) {
            assign_field!(
                par.adjacencyMatrix = None,
                par.matching = None,
                par.strongComponents = None,
                par.association = (::match_deref::match_deref! { match &(par.association.clone()) {
        association @ Deref @ Association::CONTINUOUS { .. } => {
            let mut association = (*association).clone();
            assign_variant_field!(association => Association::Association::CONTINUOUS; jacobian = None);
            association.clone()
        },
        _ => {
            par.association.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
            );
        }
        Ok(par)
    }

    pub fn removeAlias(mut par: Arc<Partition>) -> Result<Arc<Partition>> {
        let mut par: Arc<Partition> = par;
        let mut comps: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>;
        if Util::isSome(par.strongComponents.clone()) {
            comps = Util::getOption(par.strongComponents.clone())?;
            let __range0 = 1..=(comps.clone().borrow().len() as i32);
            for mut i in __range0 {
                {
                    let __cell1 = StrongComponent::removeAlias(comps.borrow()[(i.clone()-1) as usize].clone());
                    comps.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                }
            }
        }
        Ok(par)
    }

    pub fn updateHeldVars(mut par: Arc<Partition>, mut held_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Partition>> {
        let mut par: Arc<Partition> = par;
        assign_field!(par.association = (::match_deref::match_deref! { match &(par.association.clone()) {
        association @ Deref @ Association::CLOCKED { .. } => {
            let mut association = (*association).clone();
            assign_variant_field!(association => Association::Association::CLOCKED; holdEvents = !(UnorderedSet::isDisjoint(held_crefs.clone(), UnorderedMap::keySet(par.unknowns.map.clone())?)?));
            association.clone()
        },
        _ => {
            par.association.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }));
        Ok(par)
    }

    pub fn merge(mut part1: Arc<Partition>, mut part2: Arc<Partition>, mut strict: bool) -> Result<Arc<Partition>> {
        let mut part1: Arc<Partition> = part1;
        if Util::isSome(part1.daeUnknowns.clone()) || Util::isSome(part2.daeUnknowns.clone()) {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.merge")); __mm_s.push_str(&*literal!(" failed. Cannot merge DAE-Mode partitions.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        } else if Util::isSome(part1.strongComponents.clone()) || Util::isSome(part2.strongComponents.clone()) {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.merge")); __mm_s.push_str(&*literal!(" failed. Should not merge sorted partitions.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        } else if Util::isSome(part1.matching.clone()) || Util::isSome(part2.matching.clone()) {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.merge")); __mm_s.push_str(&*literal!(" failed. Should not merge matched partitions.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        } else if Util::isSome(part1.adjacencyMatrix.clone()) || Util::isSome(part2.adjacencyMatrix.clone()) {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.merge")); __mm_s.push_str(&*literal!(" failed. Should not merge partitions with adjacency matrix.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        assign_field!(
            part1.association = Association::merge(part1.association.clone(), part2.association.clone(), strict.clone())?,
            part1.unknowns = BVariable::VariablePointers::addList(BVariable::VariablePointers::toList(part2.unknowns.clone())?, part1.unknowns.clone()),
            part1.equations = BEquation::EquationPointers::addList(BEquation::EquationPointers::toList(part2.equations.clone())?, part1.equations.clone())
        );
        Ok(part1)
    }

}

pub fn kindIsInitial(mut kind: Kind) -> bool {
    let mut b: bool = kind.clone() == Kind::INI.clone() || kind.clone() == Kind::INI_0.clone();
    b
}

