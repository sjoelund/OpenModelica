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

use crate::HashTable;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::MMath;

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn check(mut tms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, mut ist: UnitAbsyn::InstStore) -> Result<UnitAbsyn::InstStore> {
    let mut outSt: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    outSt = 'mc: {
        let __mc_input = (tms.clone(), ist.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st) => {
                    Ok(st.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, UnitAbsyn::InstStore::INSTSTORE { store: st1, ht, checkResult: _ }) => {
                    Ok(UnitAbsyn::InstStore::INSTSTORE { store: st1.clone(), ht: ht.clone(), checkResult: Some(crate::UnitAbsyn::UnitCheckResult::CONSISTENT) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: tm1, tail: rest1 }, UnitAbsyn::InstStore::INSTSTORE { store: st1, ht, checkResult: _ }) => {
                    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
                    let (UnitAbsyn::CONSISTENT { .. }, _, __pa0) = (checkTerm(tm1.clone(), st1.clone())?) else { bail!("pattern mismatch") };
                    st2 = __pa0.clone();
                    st = check(rest1.clone(), UnitAbsyn::InstStore::INSTSTORE { store: st2.clone(), ht: ht.clone(), checkResult: Some(crate::UnitAbsyn::UnitCheckResult::CONSISTENT) })?;
                    Ok(st.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: tm1, tail: _ }, UnitAbsyn::InstStore::INSTSTORE { store: st1, ht, checkResult: _ }) => {
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let (UnitAbsyn::INCONSISTENT { u1: __pa0, u2: __pa1 }, _, _) = (checkTerm(tm1.clone(), st1.clone())?) else { bail!("pattern mismatch") };
                    su1 = __pa0.clone();
                    su2 = __pa1.clone();
                    s1 = (UnitAbsynBuilder::printTermsStr(list![tm1.clone()])).clone();
                    s2 = (UnitAbsynBuilder::unit2str(UnitAbsyn::Unit::SPECIFIED { specified: su1.clone() })?).clone();
                    s3 = (UnitAbsynBuilder::unit2str(UnitAbsyn::Unit::SPECIFIED { specified: su2.clone() })?).clone();
                    Error::addMessage(Error::INCONSISTENT_UNITS.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone()])?;
                    Ok(UnitAbsyn::InstStore::INSTSTORE { store: st1.clone(), ht: ht.clone(), checkResult: Some(UnitAbsyn::UnitCheckResult::INCONSISTENT { u1: su1.clone(), u2: su2.clone() }) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::check() failed\n")).clone())?;
                    println!("{}", (literal!("check failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSt)
}

pub fn isComplete(mut st: UnitAbsyn::Store) -> Result<(bool, UnitAbsyn::Store)> {
    let mut complete: bool = false;
    let mut stout: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    (complete, stout) = (match st.clone() {
        UnitAbsyn::Store { storeVector: mut vector, numElts: mut indx } => {
            let mut lst: Arc<metamodelica::List<Option<UnitAbsyn::Unit>>> = metamodelica::nil();
            let mut comp: bool = false;
            let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
            lst = Arc::new(vector.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            (comp, st2) = completeCheck(lst.clone(), 1, UnitAbsyn::Store { storeVector: vector.clone(), numElts: indx.clone() })?;
            (comp.clone(), st2.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((complete, stout))
}

fn completeCheck(mut ilst: Arc<metamodelica::List<Option<UnitAbsyn::Unit>>>, mut indx: i32, mut st: UnitAbsyn::Store) -> Result<(bool, UnitAbsyn::Store)> {
    let mut isComplete: bool = false;
    let mut stout: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    (isComplete, stout) = 'mc: {
        let __mc_input = (ilst.clone(), st.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, st2) => {
                    Ok((true, st2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Some(_), tail: lst }, st2) => {
                    let mut u2: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut comp1: bool = false;
                    let mut st3: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    (u2, st3) = normalize(indx.clone(), st2.clone())?;
                    let false = (unitHasUnknown(u2.clone())?) else { bail!("pattern mismatch") };
                    (comp1, _) = completeCheck(lst.clone(), indx.clone() + 1, st3.clone())?;
                    Ok((comp1.clone(), st3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Some(_), tail: _ }, st2) => {
                    let mut u2: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    (u2, _) = normalize(indx.clone(), st2.clone())?;
                    let true = (unitHasUnknown(u2.clone())?) else { bail!("pattern mismatch") };
                    Ok((false, st2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: None, tail: _ }, st2) => {
                    Ok((true, st2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((isComplete, stout))
}

pub fn checkTerm(mut tm: Arc<UnitAbsyn::UnitTerm>, mut st: UnitAbsyn::Store) -> Result<(UnitAbsyn::UnitCheckResult, UnitAbsyn::SpecUnit, UnitAbsyn::Store)> {
    let mut result: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
    let mut outUnit: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    let mut outSt: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    (result, outUnit, outSt) = 'mc: {
        let __mc_input = (tm.clone(), st.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::ADD { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st3: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st4: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut res1: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res2: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res3: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res4: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    (res1, su1, st2) = checkTerm(ut1.clone(), st1.clone())?;
                    (res2, su2, st3) = checkTerm(ut2.clone(), st2.clone())?;
                    (res3, st4) = unify(su1.clone(), su2.clone(), st3.clone())?;
                    res4 = chooseResult(res1.clone(), res2.clone(), res3.clone())?;
                    Ok((res4.clone(), su1.clone(), st4.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::SUB { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st3: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st4: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut res1: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res2: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res3: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res4: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    (res1, su1, st2) = checkTerm(ut1.clone(), st1.clone())?;
                    (res2, su2, st3) = checkTerm(ut2.clone(), st2.clone())?;
                    (res3, st4) = unify(su1.clone(), su2.clone(), st3.clone())?;
                    res4 = chooseResult(res1.clone(), res2.clone(), res3.clone())?;
                    Ok((res4.clone(), su1.clone(), st4.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::MUL { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st3: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut res1: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res2: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res4: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su3: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    (res1, su1, st2) = checkTerm(ut1.clone(), st1.clone())?;
                    (res2, su2, st3) = checkTerm(ut2.clone(), st2.clone())?;
                    su3 = mulSpecUnit(su1.clone(), su2.clone())?;
                    res4 = chooseResult(res1.clone(), res2.clone(), crate::UnitAbsyn::UnitCheckResult::CONSISTENT)?;
                    Ok((res4.clone(), su3.clone(), st3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::DIV { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st3: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut res1: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res2: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res4: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su3: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    (res1, su1, st2) = checkTerm(ut1.clone(), st1.clone())?;
                    (res2, su2, st3) = checkTerm(ut2.clone(), st2.clone())?;
                    su3 = divSpecUnit(su1.clone(), su2.clone())?;
                    res4 = chooseResult(res1.clone(), res2.clone(), crate::UnitAbsyn::UnitCheckResult::CONSISTENT)?;
                    Ok((res4.clone(), su3.clone(), st3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::EQN { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st3: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st4: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut res1: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res2: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res3: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut res4: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    (res1, su1, st2) = checkTerm(ut1.clone(), st1.clone())?;
                    (res2, su2, st3) = checkTerm(ut2.clone(), st2.clone())?;
                    (res3, st4) = unify(su1.clone(), su2.clone(), st3.clone())?;
                    res4 = chooseResult(res1.clone(), res2.clone(), res3.clone())?;
                    Ok((res4.clone(), su1.clone(), st4.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::LOC { loc, origExp: _ }, st1) => {
                    let UnitAbsyn::UNSPECIFIED { .. } = (UnitAbsynBuilder::find(loc.clone(), st1.clone())?) else { bail!("pattern mismatch") };
                    Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, UnitAbsyn::SpecUnit { typeParameters: cons((MMath::Rational { nom: 1, denom: 1 }, UnitAbsyn::TypeParameter { name: (literal!("")).clone(), indx: loc.clone() }), metamodelica::nil()), units: metamodelica::nil() }, st1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::LOC { loc, origExp: _ }, st1) => {
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let UnitAbsyn::SPECIFIED { specified: __pa0 } = (UnitAbsynBuilder::find(loc.clone(), st1.clone())?) else { bail!("pattern mismatch") };
                    su1 = __pa0.clone();
                    Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, su1.clone(), st1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::POW { ut1, exponent: expo1, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut res1: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    (res1, su1, st2) = checkTerm(ut1.clone(), st1.clone())?;
                    su2 = powSpecUnit(su1.clone(), expo1.clone())?;
                    Ok((res1.clone(), su2.clone(), st2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::checkTerm() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((result, outUnit, outSt))
}

fn chooseResult(mut res1: UnitAbsyn::UnitCheckResult, mut res2: UnitAbsyn::UnitCheckResult, mut res3: UnitAbsyn::UnitCheckResult) -> Result<UnitAbsyn::UnitCheckResult> {
    let mut resout: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
    let mut incon: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
    resout = (match (res1.clone(), res2.clone(), res3.clone()) {
        (UnitAbsyn::UnitCheckResult::CONSISTENT, UnitAbsyn::UnitCheckResult::CONSISTENT, UnitAbsyn::UnitCheckResult::CONSISTENT) => crate::UnitAbsyn::UnitCheckResult::CONSISTENT,
        (UnitAbsyn::UnitCheckResult::CONSISTENT, UnitAbsyn::UnitCheckResult::CONSISTENT, mut incon) => incon.clone(),
        (UnitAbsyn::UnitCheckResult::CONSISTENT, mut incon, _) => incon.clone(),
        (mut incon, _, _) => incon.clone(),
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("UnitChecker::chooseResult() failed\n")).clone())?;
            bail!("fail")
        },
    });
    Ok(resout)
}

fn unify(mut insu1: UnitAbsyn::SpecUnit, mut insu2: UnitAbsyn::SpecUnit, mut st: UnitAbsyn::Store) -> Result<(UnitAbsyn::UnitCheckResult, UnitAbsyn::Store)> {
    let mut outresult: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
    let mut outSt: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    let mut st1: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    let (UnitAbsyn::SPECIFIED { specified: __pa0 }, __pa1) = (normalizeOnUnit(UnitAbsyn::Unit::SPECIFIED { specified: insu1.clone() }, st.clone())?) else { bail!("pattern mismatch") };
    su1 = __pa0.clone();
    st1 = __pa1.clone();
    let (UnitAbsyn::SPECIFIED { specified: __pa2 }, __pa3) = (normalizeOnUnit(UnitAbsyn::Unit::SPECIFIED { specified: insu2.clone() }, st1.clone())?) else { bail!("pattern mismatch") };
    su2 = __pa2.clone();
    st2 = __pa3.clone();
    (outresult, outSt) = unifyunits(su1.clone(), su2.clone(), st2.clone())?;
    Ok((outresult, outSt))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isSpecUnitEq(mut insu1: UnitAbsyn::SpecUnit, mut insu2: UnitAbsyn::SpecUnit) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (insu1.clone(), insu2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::List::Nil }, UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::List::Nil }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::List::Nil }, UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::List::Cons { head: MMath::Rational { nom: 0, denom: _ }, tail: rest1 } }) => {
                    let mut r1: bool = false;
                    r1 = isSpecUnitEq(UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: metamodelica::nil() }, UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: rest1.clone() })?;
                    Ok(r1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::List::Cons { head: MMath::Rational { nom: 0, denom: _ }, tail: rest1 } }, UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::List::Nil }) => {
                    let mut r1: bool = false;
                    r1 = isSpecUnitEq(UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: rest1.clone() }, UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: metamodelica::nil() })?;
                    Ok(r1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::List::Cons { head: MMath::Rational { nom: i1a, denom: i1b }, tail: rest1 } }, UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::List::Cons { head: MMath::Rational { nom: i2a, denom: i2b }, tail: rest2 } }) => {
                    let mut r1: bool = false;
                    let true = (intEq(i1a.clone(), i2a.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(i1b.clone(), i2b.clone())) else { bail!("pattern mismatch") };
                    r1 = isSpecUnitEq(UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: rest1.clone() }, UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: rest2.clone() })?;
                    Ok(r1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn unifyunits(mut insu1: UnitAbsyn::SpecUnit, mut insu2: UnitAbsyn::SpecUnit, mut st: UnitAbsyn::Store) -> Result<(UnitAbsyn::UnitCheckResult, UnitAbsyn::Store)> {
    let mut outresult: UnitAbsyn::UnitCheckResult = UnitAbsyn::UnitCheckResult::CONSISTENT;
    let mut outSt: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    (outresult, outSt) = 'mc: {
        let __mc_input = (insu1.clone(), insu2.clone(), st.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut su1, mut su2, mut st1) = __mc_input.clone() else { bail!("nomatch") };
            let false = (hasUnknown(su1.clone())?) else { bail!("pattern mismatch") };
            let false = (hasUnknown(su2.clone())?) else { bail!("pattern mismatch") };
            let true = (isSpecUnitEq(su1.clone(), su2.clone())?) else { bail!("pattern mismatch") };
            Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, st1.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut su1, mut su2, mut st1) = __mc_input.clone() else { bail!("nomatch") };
            let false = (hasUnknown(su1.clone())?) else { bail!("pattern mismatch") };
            let false = (hasUnknown(su2.clone())?) else { bail!("pattern mismatch") };
            Ok((UnitAbsyn::UnitCheckResult::INCONSISTENT { u1: su1.clone(), u2: su2.clone() }, st1.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut su1, mut su2, mut st1) = __mc_input.clone() else { bail!("nomatch") };
            let mut su3: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
            let mut su4: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
            let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
            let mut loc1: i32 = 0;
            su3 = divSpecUnit(su2.clone(), su1.clone())?;
            (loc1, su4) = getUnknown(su3.clone())?;
            st2 = UnitAbsynBuilder::update(UnitAbsyn::Unit::SPECIFIED { specified: su4.clone() }, loc1.clone(), st1.clone())?;
            Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, st2.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, mut st1) = __mc_input.clone() else { bail!("nomatch") };
            Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, st1.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outresult, outSt))
}

pub fn newDimlessSpecUnit() -> Result<UnitAbsyn::SpecUnit> {
    let mut su: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    let UnitAbsyn::SPECIFIED { specified: __pa0 } = (UnitAbsynBuilder::str2unit((literal!("1")).clone(), None)?) else { bail!("pattern mismatch") };
    su = __pa0.clone();
    Ok(su)
}

pub fn getUnknown(mut suin: UnitAbsyn::SpecUnit) -> Result<(i32, UnitAbsyn::SpecUnit)> {
    let mut loc: i32 = 0;
    let mut suout: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    (loc, suout) = 'mc: {
        let __mc_input = suin.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::SpecUnit { typeParameters: Deref @ metamodelica::List::Cons { head: (expo1, UnitAbsyn::TypeParameter { name: _, indx: loc1 }), tail: rest1 }, units: unitvec1 } => {
                    let mut su1: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut expo2: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    su1 = divSpecUnit(newDimlessSpecUnit()?, UnitAbsyn::SpecUnit { typeParameters: rest1.clone(), units: unitvec1.clone() })?;
                    expo2 = MMath::divRational(MMath::Rational { nom: 1, denom: 1 }, expo1.clone())?;
                    su2 = powSpecUnit(su1.clone(), expo2.clone())?;
                    Ok((loc1.clone(), su2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::getUnknown() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((loc, suout))
}

pub fn hasUnknown(mut su: UnitAbsyn::SpecUnit) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = su.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::SpecUnit { typeParameters: Deref @ metamodelica::List::Nil, units: _ } => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::SpecUnit { typeParameters: _, units: _ } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::hasUnknown() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn unitHasUnknown(mut u: UnitAbsyn::Unit) -> Result<bool> {
    let mut res: bool = false;
    res = (match u.clone() {
        UnitAbsyn::Unit::SPECIFIED { specified: mut su } => {
            let mut unk: bool = false;
            unk = hasUnknown(su.clone())?;
            unk.clone()
        },
        _ => {
            true
        },
    });
    Ok(res)
}

pub fn mulSpecUnit(mut u1: UnitAbsyn::SpecUnit, mut u2: UnitAbsyn::SpecUnit) -> Result<UnitAbsyn::SpecUnit> {
    let mut u: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    u = 'mc: {
        let __mc_input = (u1.clone(), u2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (UnitAbsyn::SpecUnit { typeParameters: ref tparams1, units: ref units1 }, UnitAbsyn::SpecUnit { typeParameters: ref tparams2, units: ref units2 }) = __mc_input.clone() else { bail!("nomatch") };
            let mut tparams3: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut tparams4: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut units: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
            tparams3 = listAppend(tparams1.clone(), tparams2.clone());
            tparams4 = normalizeParamsExponents(tparams3.clone())?;
            units = mulUnitVec(units1.clone(), units2.clone())?;
            Ok(UnitAbsyn::SpecUnit { typeParameters: tparams4.clone(), units: units.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("UnitChecker::mulSpecUnit() failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(u)
}

pub fn mulUnitVec(mut inunitvec1: Arc<metamodelica::List<MMath::Rational>>, mut inunitvec2: Arc<metamodelica::List<MMath::Rational>>) -> Result<Arc<metamodelica::List<MMath::Rational>>> {
    let mut outunitvec: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
    outunitvec = 'mc: {
        let __mc_input = (inunitvec1.clone(), inunitvec2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: expo1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: expo2, tail: rest2 }) => {
                    let mut expo3: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let mut rest3: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
                    expo3 = MMath::addRational(expo1.clone(), expo2.clone())?;
                    rest3 = mulUnitVec(rest1.clone(), rest2.clone())?;
                    Ok(cons(expo3.clone(), rest3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: expo1, tail: rest1 }, Deref @ metamodelica::List::Nil) => {
                    let mut rest3: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
                    rest3 = mulUnitVec(rest1.clone(), metamodelica::nil())?;
                    Ok(cons(expo1.clone(), rest3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: expo1, tail: rest1 }) => {
                    let mut rest3: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
                    rest3 = mulUnitVec(metamodelica::nil(), rest1.clone())?;
                    Ok(cons(expo1.clone(), rest3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::powUnitVec() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outunitvec)
}

pub fn divSpecUnit(mut u1: UnitAbsyn::SpecUnit, mut u2: UnitAbsyn::SpecUnit) -> Result<UnitAbsyn::SpecUnit> {
    let mut u: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    u = 'mc: {
        let __mc_input = (u1.clone(), u2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (UnitAbsyn::SpecUnit { typeParameters: ref tparams1, units: ref units1 }, UnitAbsyn::SpecUnit { typeParameters: ref tparams2, units: ref units2 }) = __mc_input.clone() else { bail!("nomatch") };
            let mut tparams3: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut tparams4: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut tparams5: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut units: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
            tparams3 = negParamList(tparams2.clone(), metamodelica::nil())?;
            tparams4 = listAppend(tparams1.clone(), tparams3.clone());
            tparams5 = normalizeParamsExponents(tparams4.clone())?;
            units = divUnitVec(units1.clone(), units2.clone())?;
            Ok(UnitAbsyn::SpecUnit { typeParameters: tparams5.clone(), units: units.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("UnitChecker::divSpecUnit() failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(u)
}

pub fn divUnitVec(mut inunitvec1: Arc<metamodelica::List<MMath::Rational>>, mut inunitvec2: Arc<metamodelica::List<MMath::Rational>>) -> Result<Arc<metamodelica::List<MMath::Rational>>> {
    let mut outunitvec: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
    outunitvec = 'mc: {
        let __mc_input = (inunitvec1.clone(), inunitvec2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: expo1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: expo2, tail: rest2 }) => {
                    let mut expo3: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let mut rest3: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
                    expo3 = MMath::subRational(expo1.clone(), expo2.clone())?;
                    rest3 = divUnitVec(rest1.clone(), rest2.clone())?;
                    Ok(cons(expo3.clone(), rest3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: expo1, tail: rest1 }, Deref @ metamodelica::List::Nil) => {
                    let mut rest3: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
                    rest3 = divUnitVec(rest1.clone(), metamodelica::nil())?;
                    Ok(cons(expo1.clone(), rest3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: expo1, tail: rest1 }) => {
                    let mut expo2: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let mut rest3: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
                    expo2 = MMath::subRational(MMath::Rational { nom: 0, denom: 1 }, expo1.clone())?;
                    rest3 = divUnitVec(metamodelica::nil(), rest1.clone())?;
                    Ok(cons(expo2.clone(), rest3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::powUnitVec() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outunitvec)
}

pub fn powSpecUnit(mut suin: UnitAbsyn::SpecUnit, mut expo: MMath::Rational) -> Result<UnitAbsyn::SpecUnit> {
    let mut uout: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    uout = 'mc: {
        let __mc_input = suin.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::SpecUnit { typeParameters: ref params1, units: ref unitvec1 } = __mc_input.clone() else { bail!("nomatch") };
            let mut params2: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut unitvec2: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
            params2 = powUnitParams(params1.clone(), expo.clone())?;
            unitvec2 = powUnitVec(unitvec1.clone(), expo.clone())?;
            Ok(UnitAbsyn::SpecUnit { typeParameters: params2.clone(), units: unitvec2.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("UnitChecker::powSpecUnit() failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(uout)
}

pub fn powUnitParams(mut inparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>, mut expo: MMath::Rational) -> Result<Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>> {
    let mut outparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
    outparams = 'mc: {
        let __mc_input = (inparams.clone(), expo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (expo1, param), tail: rest1 }, expo2) => {
                    let mut expo3: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let mut rest2: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
                    expo3 = MMath::multRational(expo1.clone(), expo2.clone())?;
                    rest2 = powUnitParams(rest1.clone(), expo2.clone())?;
                    Ok(cons((expo3.clone(), param.clone()), rest2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::powUnitParams() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outparams)
}

pub fn powUnitVec(mut inunitvec: Arc<metamodelica::List<MMath::Rational>>, mut expo: MMath::Rational) -> Result<Arc<metamodelica::List<MMath::Rational>>> {
    let mut outunitvec: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
    outunitvec = 'mc: {
        let __mc_input = (inunitvec.clone(), expo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: expo1, tail: rest1 }, expo2) => {
                    let mut expo3: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let mut rest2: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
                    expo3 = MMath::multRational(expo1.clone(), expo2.clone())?;
                    rest2 = powUnitVec(rest1.clone(), expo2.clone())?;
                    Ok(cons(expo3.clone(), rest2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::powUnitVec() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outunitvec)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn negParamList(mut ine: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>, mut ac: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>) -> Result<Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>> {
    let mut oute: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
    oute = 'mc: {
        let __mc_input = (ine.clone(), ac.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, ac2) => {
                    Ok(ac2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (MMath::Rational { nom: i1, denom: i2 }, UnitAbsyn::TypeParameter { name, indx }), tail: rest }, ac2) => {
                    let mut qr: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let mut pres: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
                    qr = MMath::multRational(MMath::Rational { nom: -1, denom: 1 }, MMath::Rational { nom: i1.clone(), denom: i2.clone() })?;
                    pres = negParamList(rest.clone(), cons((qr.clone(), UnitAbsyn::TypeParameter { name: (name.clone()).clone(), indx: indx.clone() }), ac2.clone()))?;
                    Ok(pres.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::negParamList() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oute)
}

pub fn normalize(mut loc: i32, mut st: UnitAbsyn::Store) -> Result<(UnitAbsyn::Unit, UnitAbsyn::Store)> {
    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut outSt: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    let mut u1: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut u2: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    u1 = UnitAbsynBuilder::find(loc.clone(), st.clone())?;
    (u2, st2) = normalizeOnUnit(u1.clone(), st.clone())?;
    outSt = UnitAbsynBuilder::update(u2.clone(), loc.clone(), st2.clone())?;
    unit = u2.clone();
    Ok((unit, outSt))
}

pub fn normalizeOnUnit(mut u: UnitAbsyn::Unit, mut st: UnitAbsyn::Store) -> Result<(UnitAbsyn::Unit, UnitAbsyn::Store)> {
    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut outSt: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    (unit, outSt) = 'mc: {
        let __mc_input = u.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::Unit::UNSPECIFIED = __mc_input.clone() else { bail!("nomatch") };
            Ok((crate::UnitAbsyn::Unit::UNSPECIFIED, st.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: ref params1, units: ref unitvec1 } } = __mc_input.clone() else { bail!("nomatch") };
            let mut params2: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut params3: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut unitvec2: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
            let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
            let (UnitAbsyn::SPECUNIT { typeParameters: __pa0, units: __pa1 }, __pa2) = (normalizeParamsValues(params1.clone(), UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: unitvec1.clone() }, st.clone())?) else { bail!("pattern mismatch") };
            params2 = __pa0.clone();
            unitvec2 = __pa1.clone();
            st2 = __pa2.clone();
            params3 = normalizeParamsExponents(params2.clone())?;
            Ok((UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: params3.clone(), units: unitvec2.clone() } }, st2.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("UnitChecker::normalizeOnUnit() failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((unit, outSt))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn normalizeParamsExponents(mut inparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>) -> Result<Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>> {
    let mut outparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
    outparams = 'mc: {
        let __mc_input = inparams.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (expo1, UnitAbsyn::TypeParameter { name, indx: loc1 }), tail: rest1 } => {
                    let mut rest2: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
                    let mut rest3: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
                    let mut expo2: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let mut expo3: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getParam(rest1.clone(), loc1.clone())?) {
                        (true, __pa0, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expo2 = __pa0.clone();
                    rest2 = __pa1.clone();
                    expo3 = MMath::addRational(expo1.clone(), expo2.clone())?;
                    rest3 = normalizeParamsExponents(cons((expo3.clone(), UnitAbsyn::TypeParameter { name: (name.clone()).clone(), indx: loc1.clone() }), rest2.clone()))?;
                    Ok(rest3.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (MMath::Rational { nom: 0, denom: 1 }, _), tail: rest1 } => {
                    let mut rest2: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
                    rest2 = normalizeParamsExponents(rest1.clone())?;
                    Ok(rest2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: param, tail: rest1 } => {
                    let mut rest2: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
                    rest2 = normalizeParamsExponents(rest1.clone())?;
                    Ok(cons(param.clone(), rest2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::normalizeParamsExponents() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outparams)
}

fn getParam(mut inparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>, mut loc: i32) -> Result<(bool, MMath::Rational, Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>)> {
    let mut found: bool = false;
    let mut outexpo: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
    let mut outparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
    (found, outexpo, outparams) = 'mc: {
        let __mc_input = inparams.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((false, MMath::Rational { nom: 1, denom: 1 }, metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (expo, UnitAbsyn::TypeParameter { name: _, indx: loc2 }), tail: rest } => {
                    let true = (intEq(loc2.clone(), loc.clone())) else { bail!("pattern mismatch") };
                    Ok((true, expo.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: param, tail: rest } => {
                    let mut rest2: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
                    let mut expo: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
                    let mut found2: bool = false;
                    (found2, expo, rest2) = getParam(rest.clone(), loc.clone())?;
                    Ok((found2.clone(), expo.clone(), cons(param.clone(), rest2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::getParam() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((found, outexpo, outparams))
}

fn normalizeParamsValues(mut inparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>, mut suin: UnitAbsyn::SpecUnit, mut st: UnitAbsyn::Store) -> Result<(UnitAbsyn::SpecUnit, UnitAbsyn::Store)> {
    let mut uout: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    let mut outSt: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
    (uout, outSt) = 'mc: {
        let __mc_input = inparams.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((suin.clone(), st.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (expo, UnitAbsyn::TypeParameter { name, indx: loc }), tail: rest } => {
                    let mut st2: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut st3: UnitAbsyn::Store = <UnitAbsyn::Store as ::std::default::Default>::default();
                    let mut u2: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut su2: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    let mut su3: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
                    (u2, st2) = normalize(loc.clone(), st.clone())?;
                    su2 = mulSpecUnitWithNorm(suin.clone(), u2.clone(), (name.clone()).clone(), loc.clone(), expo.clone())?;
                    (su3, st3) = normalizeParamsValues(rest.clone(), su2.clone(), st2.clone())?;
                    Ok((su3.clone(), st3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("UnitChecker::normalizeParamsValues() failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((uout, outSt))
}

fn mulSpecUnitWithNorm(mut suin: UnitAbsyn::SpecUnit, mut normunit: UnitAbsyn::Unit, mut name: ArcStr, mut loc: i32, mut expo: MMath::Rational) -> Result<UnitAbsyn::SpecUnit> {
    let mut suout: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
    suout = 'mc: {
        let __mc_input = (suin.clone(), normunit.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (UnitAbsyn::SpecUnit { typeParameters: ref params, units: ref unitvec }, UnitAbsyn::Unit::UNSPECIFIED) = __mc_input.clone() else { bail!("nomatch") };
            Ok(UnitAbsyn::SpecUnit { typeParameters: cons((expo.clone(), UnitAbsyn::TypeParameter { name: (name.clone()).clone(), indx: loc.clone() }), params.clone()), units: unitvec.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut su2, UnitAbsyn::Unit::SPECIFIED { specified: mut sunorm }) = __mc_input.clone() else { bail!("nomatch") };
            let mut su3: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
            let mut su4: UnitAbsyn::SpecUnit = <UnitAbsyn::SpecUnit as ::std::default::Default>::default();
            su3 = powSpecUnit(sunorm.clone(), expo.clone())?;
            su4 = mulSpecUnit(su2.clone(), su3.clone())?;
            Ok(su4.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("UnitChecker::mulSpecUnitWithNorm() failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(suout)
}

pub fn printSpecUnit(mut text: ArcStr, mut su: UnitAbsyn::SpecUnit) -> Result<()> {
    let () = (match (text.clone(), su.clone()) {
        (mut r#str, UnitAbsyn::SpecUnit { typeParameters: ref params, units: _ }) => {
            println!("{}", (r#str.clone()).clone());
            println!("{}", (literal!(" \"")).clone());
            println!("{}", (UnitAbsynBuilder::unit2str(UnitAbsyn::Unit::SPECIFIED { specified: su.clone() })?).clone());
            println!("{}", (literal!("\" {")).clone());
            printSpecUnitParams(params.clone())?;
            println!("{}", (literal!("}\n")).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn printSpecUnitParams(mut params: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(params.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (MMath::Rational { nom: i1, denom: i2 }, UnitAbsyn::TypeParameter { name, indx: loc }), tail: rest } => {
            println!("{}", (literal!("(\"")).clone());
            println!("{}", (name.clone()).clone());
            println!("{}", (literal!("\",")).clone());
            println!("{}", (intString(loc.clone())).clone());
            println!("{}", (literal!(")^(")).clone());
            println!("{}", (intString(i1.clone())).clone());
            println!("{}", (literal!("/")).clone());
            println!("{}", (intString(i2.clone())).clone());
            println!("{}", (literal!("),")).clone());
            printSpecUnitParams(rest.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn testUnitOp() -> () {
    println!("{}", (literal!("test")).clone());
    ()
}

pub fn printResult(mut res: UnitAbsyn::UnitCheckResult) -> Result<()> {
    let () = (match res.clone() {
        UnitAbsyn::UnitCheckResult::CONSISTENT => {
            println!("{}", (literal!("\n---\nThe system of units is consistent.\n---\n")).clone());
            ()
        },
        UnitAbsyn::UnitCheckResult::INCONSISTENT { u1: mut u1, u2: mut u2 } => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            println!("{}", (literal!("\n---\nThe system of units is inconsistent. \"")).clone());
            str1 = (UnitAbsynBuilder::unit2str(UnitAbsyn::Unit::SPECIFIED { specified: u1.clone() })?).clone();
            println!("{}", (str1.clone()).clone());
            println!("{}", (literal!("\" != \"")).clone());
            str2 = (UnitAbsynBuilder::unit2str(UnitAbsyn::Unit::SPECIFIED { specified: u2.clone() })?).clone();
            println!("{}", (str2.clone()).clone());
            println!("{}", (literal!("\"\n---\n")).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

