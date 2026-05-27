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

use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::WhenStatement;
use crate::NBEquation;
use crate::NBPartitioning::BClock;
use crate::NSimCode::SimCodeIndices;
use crate::NSimStrongComponent::Block;
use crate::NSimVar::SimVar;
use crate::SimCode as OldSimCode;
use crate::SimCodeVar;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBuiltinFuncs as BuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_util::Error;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;

/// file:        NSimPartition.mo
/// package:     NSimPartition
/// description: This file contains the data types and functions for clocked partitions
///              in simulation code phase.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSimPartition {
    BASE_PARTITION {
        baseClock: Arc<BClock::BClock>,
        subPartitions: Arc<metamodelica::List<Arc<NSimPartition>>>,
    },
    SUB_PARTITION {
        variables: Arc<metamodelica::List<(Arc<SimVar::SimVar>, bool)>>,
        equations: Arc<metamodelica::List<Arc<Block::Block>>>,
        removedEquations: Arc<metamodelica::List<Arc<Block::Block>>>,
        subClock: Arc<BClock::BClock>,
        holdEvents: bool,
    },
}
impl Default for NSimPartition {
    fn default() -> Self {
        Self::BASE_PARTITION {
            baseClock: Default::default(),
            subPartitions: Default::default(),
        }
    }
}
pub use self::NSimPartition::{BASE_PARTITION,SUB_PARTITION};
pub fn createSubPartition(mut subClock: Arc<BClock::BClock>, mut equations: Arc<metamodelica::List<Arc<Block::Block>>>, mut variables: Arc<metamodelica::List<Arc<SimVar::SimVar>>>, mut holdEvents: bool) -> Arc<NSimPartition> {
    let mut part: Arc<NSimPartition>;
    part = Arc::new(NSimPartition::SUB_PARTITION { variables: {
        let mut __acc: Arc<metamodelica::List<(Arc<SimVar::SimVar>, bool)>> = metamodelica::nil();
        for mut v in (variables.clone()).into_iter().cloned() {
            let __x = (v.clone(), true);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, equations: equations.clone(), removedEquations: metamodelica::nil(), subClock: subClock.clone(), holdEvents: holdEvents.clone() });
    part
}

pub fn createBasePartitions(mut clock_collector: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<metamodelica::List<Arc<NSimPartition>>>>>, mut simCodeIndices: SimCodeIndices) -> (Arc<metamodelica::List<Arc<NSimPartition>>>, Arc<metamodelica::List<Arc<Block::Block>>>, SimCodeIndices) {
    let mut baseParts: Arc<metamodelica::List<Arc<NSimPartition>>> = metamodelica::nil();
    let mut eventClocks: Arc<metamodelica::List<Arc<Block::Block>>> = metamodelica::nil();
    let mut simCodeIndices: SimCodeIndices = simCodeIndices;
    let mut baseClock: Arc<BClock::BClock>;
    let mut subClocks: Arc<metamodelica::List<Arc<NSimPartition>>> = metamodelica::nil();
    let mut clock_idx: i32 = 1;
    for mut tpl in &*UnorderedMap::toList(clock_collector.clone()) {
        let mut tpl = tpl.clone();
        (baseClock, subClocks) = tpl.clone();
        if !(BClock::isInferredClock(baseClock.clone())) {
            baseParts = cons(Arc::new(NSimPartition::BASE_PARTITION { baseClock: baseClock.clone(), subPartitions: subClocks.clone() }), baseParts.clone());
        }
    }
    for mut base in &*baseParts.clone() {
        let mut base = base.clone();
        let _ = (::match_deref::match_deref! { match &(base.clone()) {
        Deref @ BASE_PARTITION { baseClock: Deref @ BClock::BASE_CLOCK { clock: Deref @ ClockKind::EVENT_CLOCK { condition: Deref @ Expression::CREF { cref: cond, .. }, .. } }, .. } => {
            let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
            let mut fire: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut stmt: Arc<WhenStatement::WhenStatement>;
            let mut attr: Arc<EquationAttributes::EquationAttributes>;
            let mut blck: Arc<Block::Block>;
            source = DAE::emptyElementSource().clone();
            fire = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::CLOCK_FIRE().clone(), list![Arc::new(Expression::NFExpression::INTEGER { value: clock_idx.clone() })], Prefixes::Variability::CONSTANT.clone(), Prefixes::Purity::PURE.clone(), BuiltinFuncs::CLOCK_FIRE().returnType.clone()) });
            stmt = Arc::new(WhenStatement::WhenStatement::NORETCALL { exp: fire.clone(), source: source.clone() });
            attr = NBEquation::default(EquationKind::EMPTY.clone(), false, None, None);
            blck = Arc::new(Block::Block::WHEN { index: simCodeIndices.equationIndex.clone(), initialCall: false, conditions: list![cond.clone()], when_stmts: list![stmt.clone()], else_when: None, source: source.clone(), attr: attr.clone() });
            eventClocks = cons(blck.clone(), eventClocks.clone());
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        clock_idx = clock_idx.clone() + 1;
    }
    (baseParts, eventClocks, simCodeIndices)
}

pub fn getClock(mut part: Arc<NSimPartition>) -> Result<Arc<BClock::BClock>> {
    let mut clock: Arc<BClock::BClock>;
    clock = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ BASE_PARTITION { .. } => var_field!((*part).baseClock, NSimPartition::BASE_PARTITION).clone(),
        Deref @ SUB_PARTITION { .. } => var_field!((*part).subClock, NSimPartition::SUB_PARTITION).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimPartition.getClock")); __mm_s.push_str(&*literal!(" failed for unknown partition:\n")); __mm_s.push_str(&*toString(part.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(clock)
}

pub fn listToString(mut parts: Arc<metamodelica::List<Arc<NSimPartition>>>, mut r#str: ArcStr, mut header: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut indent: ArcStr = r#str.clone();
    r#str = (if (header.clone() != literal!("")) {StringUtil::headline_3((header.clone()).clone())} else {literal!("")}).clone();
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toString(part.clone(), (indent.clone()).clone())?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn toString(mut part: Arc<NSimPartition>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = ((::match_deref::match_deref! { match &(part.clone()) {
        Deref @ BASE_PARTITION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[BASE] Partition ")); __mm_s.push_str(&*BClock::toString(var_field!((*part).baseClock, NSimPartition::BASE_PARTITION).clone())?); __mm_s.push_str(&*List::toString(var_field!((*part).subPartitions, NSimPartition::BASE_PARTITION).clone(), Arc::new({ let __pe_b1 = (r#str.clone()).clone(); move |__pe_a0| toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        Deref @ SUB_PARTITION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[SUB-] Partition ")); __mm_s.push_str(&*BClock::toString(var_field!((*part).subClock, NSimPartition::SUB_PARTITION).clone())?); __mm_s.push_str(&*List::toString(var_field!((*part).equations, NSimPartition::SUB_PARTITION).clone(), Arc::new({ let __pe_b1 = (r#str.clone()).clone(); move |__pe_a0| Block::toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        _ => literal!("[ERR-]"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn toStringShort(mut part: Arc<NSimPartition>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(part.clone()) {
        Deref @ BASE_PARTITION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[BASE] Partition ")); __mm_s.push_str(&*BClock::toString(var_field!((*part).baseClock, NSimPartition::BASE_PARTITION).clone())?); ArcStr::from(__mm_s) },
        Deref @ SUB_PARTITION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[SUB-] Partition ")); __mm_s.push_str(&*BClock::toString(var_field!((*part).subClock, NSimPartition::SUB_PARTITION).clone())?); ArcStr::from(__mm_s) },
        _ => literal!("[ERR-]"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn convertBase(mut part: Arc<NSimPartition>) -> Result<OldSimCode::ClockedPartition> {
    let mut oldPart: OldSimCode::ClockedPartition;
    oldPart = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ BASE_PARTITION { .. } => OldSimCode::ClockedPartition { subPartitions: {
        let mut __acc: Arc<metamodelica::List<OldSimCode::SubPartition>> = metamodelica::nil();
        for mut sub in (var_field!((*part).subPartitions, NSimPartition::BASE_PARTITION).clone()).into_iter().cloned() {
            let __x = convertSub(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, baseClock: BClock::convertBase(var_field!((*part).baseClock, NSimPartition::BASE_PARTITION).clone())? },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimPartition.convertBase")); __mm_s.push_str(&*literal!(" failed for non-base partition:\n")); __mm_s.push_str(&*toString(part.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oldPart)
}

pub fn convertSub(mut part: Arc<NSimPartition>) -> Result<OldSimCode::SubPartition> {
    let mut oldPart: OldSimCode::SubPartition;
    oldPart = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ SUB_PARTITION { .. } => OldSimCode::SubPartition { holdEvents: var_field!((*part).holdEvents, NSimPartition::SUB_PARTITION).clone(), subClock: BClock::convertSub(var_field!((*part).subClock, NSimPartition::SUB_PARTITION).clone())?, removedEquations: {
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck in (var_field!((*part).removedEquations, NSimPartition::SUB_PARTITION).clone()).into_iter().cloned() {
            let __x = Block::convert(blck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, equations: {
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck in (var_field!((*part).equations, NSimPartition::SUB_PARTITION).clone()).into_iter().cloned() {
            let __x = Block::convert(blck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, vars: {
        let mut __acc: Arc<metamodelica::List<(SimCodeVar::SimVar, bool)>> = metamodelica::nil();
        for mut tpl in (var_field!((*part).variables, NSimPartition::SUB_PARTITION).clone()).into_iter().cloned() {
            let __x = SimVar::convertTpl(tpl.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimPartition.convertSub")); __mm_s.push_str(&*literal!(" failed for non-base partition:\n")); __mm_s.push_str(&*toString(part.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oldPart)
}


