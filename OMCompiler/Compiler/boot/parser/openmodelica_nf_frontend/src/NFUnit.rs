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

use crate::NFComponentRef as ComponentRef;
use crate::NFInstNode::InstNode;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type StringToUnitTable = Arc<UnorderedMap::UnorderedMap<ArcStr, Unit>>;

pub type UnitToStringTable = Arc<UnorderedMap::UnorderedMap<Unit, ArcStr>>;

pub type CrefToUnitTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    /// based on SI base units
    UNIT {
        /// second
        s: i32,
        /// meter
        m: i32,
        /// gram
        g: i32,
        /// ampere
        A: i32,
        /// kelvin
        K: i32,
        /// mole
        mol: i32,
        /// candela
        cd: i32,
        /// prefix
        factor: metamodelica::Real,
    },
    /// unknown unit that belongs to all the variables from varList
    MASTER {
        varList: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
    },
    /// unknown SI base unit decomposition
    UNKNOWN {
        unit: ArcStr,
    },
}
impl Default for Unit {
    fn default() -> Self {
        Self::MASTER {
            varList: Default::default(),
        }
    }
}
pub use self::Unit::{UNIT,MASTER,UNKNOWN};

pub const fn ONE() -> Unit { Unit::UNIT { s: 0, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) } }

pub const fn SECOND() -> Unit { Unit::UNIT { s: 1, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) } }

//public constant Unit THRICE = ?
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    T_NUMBER {
        number: i32,
    },
    T_UNIT {
        unit: ArcStr,
    },
    T_MUL,
    T_DIV,
    T_LPAREN,
    T_RPAREN,
}
pub use self::Token::{T_NUMBER,T_UNIT,T_MUL,T_DIV,T_LPAREN,T_RPAREN};

thread_local! { static __UPDATECREF_TLS: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::NFComponentRef::CREF { node: Arc::new(InstNode::InstNode::NAME_NODE { name: (literal!("jhagemann")).clone() }), subscripts: metamodelica::nil(), ty: Arc::new(crate::NFType::UNKNOWN), origin: ComponentRef::Origin::CREF.clone(), restCref: Arc::new(crate::NFComponentRef::EMPTY) }); }
pub fn UPDATECREF() -> Arc<ComponentRef::NFComponentRef> { __UPDATECREF_TLS.with(|__t| __t.clone()) }

/* from https://www.bipm.org/documents/d/guest/si-brochure-9-en-pdf */
thread_local! { static __LU_COMPLEXUNITS_TLS: Arc<metamodelica::List<(ArcStr, Unit)>> = list![(literal!("1"), Unit::UNIT { s: 0, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("s"), Unit::UNIT { s: 1, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("m"), Unit::UNIT { s: 0, m: 1, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("g"), Unit::UNIT { s: 0, m: 0, g: 1, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("A"), Unit::UNIT { s: 0, m: 0, g: 0, A: 1, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("K"), Unit::UNIT { s: 0, m: 0, g: 0, A: 0, K: 1, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("mol"), Unit::UNIT { s: 0, m: 0, g: 0, A: 0, K: 0, mol: 1, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("cd"), Unit::UNIT { s: 0, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 1, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("rad"), Unit::UNIT { s: 0, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("Hz"), Unit::UNIT { s: -1, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("N"), Unit::UNIT { s: -2, m: 1, g: 1, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("Pa"), Unit::UNIT { s: -2, m: -1, g: 1, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("J"), Unit::UNIT { s: -2, m: 2, g: 1, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("W"), Unit::UNIT { s: -3, m: 2, g: 1, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("C"), Unit::UNIT { s: 1, m: 0, g: 0, A: 1, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("V"), Unit::UNIT { s: -3, m: 2, g: 1, A: -1, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("F"), Unit::UNIT { s: 4, m: -2, g: -1, A: 2, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e-3_f64) }), (literal!("Ohm"), Unit::UNIT { s: -3, m: 2, g: 1, A: -2, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("S"), Unit::UNIT { s: 3, m: -2, g: -1, A: 2, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e-3_f64) }), (literal!("Wb"), Unit::UNIT { s: -2, m: 2, g: 1, A: -1, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("T"), Unit::UNIT { s: -2, m: 0, g: 1, A: -1, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("H"), Unit::UNIT { s: -2, m: 2, g: 1, A: -2, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e3_f64) }), (literal!("degC"), Unit::UNIT { s: 0, m: 0, g: 0, A: 0, K: 1, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("kat"), Unit::UNIT { s: -1, m: 0, g: 0, A: 0, K: 0, mol: 1, cd: 0, factor: metamodelica::OrderedFloat(1e0_f64) }), (literal!("min"), Unit::UNIT { s: 1, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat((60) as f64) }), (literal!("h"), Unit::UNIT { s: 1, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat((3600) as f64) }), (literal!("d"), Unit::UNIT { s: 1, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat((86400) as f64) }), (literal!("l"), Unit::UNIT { s: 0, m: 3, g: 0, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e-3_f64) }), (literal!("bar"), Unit::UNIT { s: -2, m: -1, g: 1, A: 0, K: 0, mol: 0, cd: 0, factor: metamodelica::OrderedFloat(1e8_f64) }), (literal!("degF"), Unit::UNIT { s: 0, m: 0, g: 0, A: 0, K: 0, mol: 0, cd: 1, factor: metamodelica::OrderedFloat(0.5555555555555556_f64) })]; }
pub fn LU_COMPLEXUNITS() -> Arc<metamodelica::List<(ArcStr, Unit)>> { __LU_COMPLEXUNITS_TLS.with(|__t| __t.clone()) }

pub fn getKnownUnits() -> Result<StringToUnitTable> {
    let mut outKnownUnits: StringToUnitTable;
    let mut s: ArcStr = arcstr::literal!("");
    let mut ut: Unit;
    outKnownUnits = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), Util::nextPrime((LU_COMPLEXUNITS().clone().len() as i32)));
    for mut unit in &*LU_COMPLEXUNITS().clone() {
        let mut unit = unit.clone();
        (s, ut) = unit.clone();
        UnorderedMap::add((s.clone()).clone(), ut.clone(), outKnownUnits.clone())?;
    }
    Ok(outKnownUnits)
}

pub fn getKnownUnitsInverse() -> Result<UnitToStringTable> {
    let mut outKnownUnitsInverse: UnitToStringTable;
    let mut s: ArcStr = arcstr::literal!("");
    let mut ut: Unit;
    outKnownUnitsInverse = UnorderedMap::new((std::sync::Arc::new(fnptr!(hash, Unit)) as std::sync::Arc<dyn ::std::ops::Fn(Unit) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(isEqual, Unit, Unit)) as std::sync::Arc<dyn ::std::ops::Fn(Unit, Unit) -> Result<bool> + 'static>), Util::nextPrime((LU_COMPLEXUNITS().clone().len() as i32)));
    for mut unit in &*LU_COMPLEXUNITS().clone() {
        let mut unit = unit.clone();
        (s, ut) = unit.clone();
        UnorderedMap::tryAdd(ut.clone(), (s.clone()).clone(), outKnownUnitsInverse.clone())?;
    }
    Ok(outKnownUnitsInverse)
}

pub fn newCrefUnitTable(mut size: i32) -> CrefToUnitTable {
    let mut table: CrefToUnitTable;
    table = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), size.clone());
    table
}

pub fn isUnit(mut inUnit: Unit) -> bool {
    let mut b: bool = false;
    b = (match inUnit.clone() {
        Unit::UNIT { .. } => true,
        _ => false,
    });
    b
}

pub fn isMaster(mut unit: Unit) -> bool {
    let mut res: bool = false;
    res = (match unit.clone() {
        Unit::MASTER { .. } => true,
        _ => false,
    });
    res
}

pub fn hash(mut inKey: Unit) -> i32 {
    let mut outHash: i32 = stringHashDjb2((unit2string(inKey.clone()).unwrap()).clone());
    outHash
}

pub fn realAlmostEqRel(mut a: metamodelica::Real, mut b: metamodelica::Real, mut relTol: metamodelica::Real) -> bool {
    let mut c: bool = false;
    c = if (a.clone() == b.clone()) {true} else {relTol.clone() > a.clone() - b.clone().abs() / (a.clone().abs() + b.clone().abs())};
    c
}

pub fn isEqual(mut unit1: Unit, mut unit2: Unit) -> bool {
    let mut res: bool = false;
    res = (match (unit1.clone(), unit2.clone()) {
        (Unit::UNIT { .. }, Unit::UNIT { .. }) => var_field!(unit1.s, Unit::UNIT).clone() == var_field!(unit2.s, Unit::UNIT).clone() && var_field!(unit1.m, Unit::UNIT).clone() == var_field!(unit2.m, Unit::UNIT).clone() && var_field!(unit1.g, Unit::UNIT).clone() == var_field!(unit2.g, Unit::UNIT).clone() && var_field!(unit1.A, Unit::UNIT).clone() == var_field!(unit2.A, Unit::UNIT).clone() && var_field!(unit1.K, Unit::UNIT).clone() == var_field!(unit2.K, Unit::UNIT).clone() && var_field!(unit1.mol, Unit::UNIT).clone() == var_field!(unit2.mol, Unit::UNIT).clone() && var_field!(unit1.cd, Unit::UNIT).clone() == var_field!(unit2.cd, Unit::UNIT).clone() && realAlmostEqRel(var_field!(unit1.factor, Unit::UNIT).clone(), var_field!(unit2.factor, Unit::UNIT).clone(), metamodelica::OrderedFloat(1e-3_f64)),
        (Unit::MASTER { .. }, Unit::MASTER { .. }) => true,
        (Unit::UNKNOWN { .. }, Unit::UNKNOWN { .. }) => var_field!(unit1.unit, Unit::UNKNOWN).clone() == var_field!(unit2.unit, Unit::UNKNOWN).clone(),
        _ => false,
    });
    res
}

pub fn unit2string(mut unit: Unit) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match unit.clone() {
        Unit::UNIT { .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut b: bool = false;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*realString(var_field!(unit.factor, Unit::UNIT).clone())); __mm_s.push_str(&*literal!(" * ")); ArcStr::from(__mm_s) }).clone();
            b = false;
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mol^(")); __mm_s.push_str(&*intString(var_field!(unit.mol, Unit::UNIT).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(var_field!(unit.mol, Unit::UNIT).clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.mol, Unit::UNIT).clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(var_field!(unit.cd, Unit::UNIT).clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd^(")); __mm_s.push_str(&*intString(var_field!(unit.cd, Unit::UNIT).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(var_field!(unit.cd, Unit::UNIT).clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.cd, Unit::UNIT).clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(var_field!(unit.m, Unit::UNIT).clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("m^(")); __mm_s.push_str(&*intString(var_field!(unit.m, Unit::UNIT).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(var_field!(unit.m, Unit::UNIT).clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.m, Unit::UNIT).clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(var_field!(unit.s, Unit::UNIT).clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("s^(")); __mm_s.push_str(&*intString(var_field!(unit.s, Unit::UNIT).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(var_field!(unit.s, Unit::UNIT).clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.s, Unit::UNIT).clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(var_field!(unit.A, Unit::UNIT).clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("A^(")); __mm_s.push_str(&*intString(var_field!(unit.A, Unit::UNIT).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(var_field!(unit.A, Unit::UNIT).clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.A, Unit::UNIT).clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(var_field!(unit.K, Unit::UNIT).clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("K^(")); __mm_s.push_str(&*intString(var_field!(unit.K, Unit::UNIT).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(var_field!(unit.K, Unit::UNIT).clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.K, Unit::UNIT).clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(var_field!(unit.g, Unit::UNIT).clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("g^(")); __mm_s.push_str(&*intString(var_field!(unit.g, Unit::UNIT).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(var_field!(unit.g, Unit::UNIT).clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.g, Unit::UNIT).clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*if (b.clone()) {literal!("")} else {literal!("1")}); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Unit::MASTER { .. } => {
            List::toString(var_field!(unit.varList, Unit::MASTER).clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("MASTER")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?
        },
        Unit::UNKNOWN { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNKOWN(")); __mm_s.push_str(&*var_field!(unit.unit, Unit::UNKNOWN).clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unitMul(mut inUnit1: Unit, mut inUnit2: Unit) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = (match (inUnit1.clone(), inUnit2.clone()) {
        (Unit::UNIT { .. }, Unit::UNIT { .. }) => Unit::UNIT { factor: var_field!(inUnit1.factor, Unit::UNIT).clone() * var_field!(inUnit2.factor, Unit::UNIT).clone(), cd: var_field!(inUnit1.cd, Unit::UNIT).clone() + var_field!(inUnit2.cd, Unit::UNIT).clone(), mol: var_field!(inUnit1.mol, Unit::UNIT).clone() + var_field!(inUnit2.mol, Unit::UNIT).clone(), K: var_field!(inUnit1.K, Unit::UNIT).clone() + var_field!(inUnit2.K, Unit::UNIT).clone(), A: var_field!(inUnit1.A, Unit::UNIT).clone() + var_field!(inUnit2.A, Unit::UNIT).clone(), g: var_field!(inUnit1.g, Unit::UNIT).clone() + var_field!(inUnit2.g, Unit::UNIT).clone(), m: var_field!(inUnit1.m, Unit::UNIT).clone() + var_field!(inUnit2.m, Unit::UNIT).clone(), s: var_field!(inUnit1.s, Unit::UNIT).clone() + var_field!(inUnit2.s, Unit::UNIT).clone() },
        _ => bail!("match: no arm matched"),
    });
    Ok(outUnit)
}

pub fn unitDiv(mut inUnit1: Unit, mut inUnit2: Unit) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = (match (inUnit1.clone(), inUnit2.clone()) {
        (Unit::UNIT { .. }, Unit::UNIT { .. }) => Unit::UNIT { factor: var_field!(inUnit1.factor, Unit::UNIT).clone() / var_field!(inUnit2.factor, Unit::UNIT).clone(), cd: var_field!(inUnit1.cd, Unit::UNIT).clone() - var_field!(inUnit2.cd, Unit::UNIT).clone(), mol: var_field!(inUnit1.mol, Unit::UNIT).clone() - var_field!(inUnit2.mol, Unit::UNIT).clone(), K: var_field!(inUnit1.K, Unit::UNIT).clone() - var_field!(inUnit2.K, Unit::UNIT).clone(), A: var_field!(inUnit1.A, Unit::UNIT).clone() - var_field!(inUnit2.A, Unit::UNIT).clone(), g: var_field!(inUnit1.g, Unit::UNIT).clone() - var_field!(inUnit2.g, Unit::UNIT).clone(), m: var_field!(inUnit1.m, Unit::UNIT).clone() - var_field!(inUnit2.m, Unit::UNIT).clone(), s: var_field!(inUnit1.s, Unit::UNIT).clone() - var_field!(inUnit2.s, Unit::UNIT).clone() },
        _ => bail!("match: no arm matched"),
    });
    Ok(outUnit)
}

pub fn unitPow(mut inUnit: Unit, mut inExp: i32) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = (match inUnit.clone() {
        Unit::UNIT { .. } => Unit::UNIT { factor: (var_field!(inUnit.factor, Unit::UNIT).clone()).powf(metamodelica::OrderedFloat((inExp.clone()) as f64)), cd: var_field!(inUnit.cd, Unit::UNIT).clone() * inExp.clone(), mol: var_field!(inUnit.mol, Unit::UNIT).clone() * inExp.clone(), K: var_field!(inUnit.K, Unit::UNIT).clone() * inExp.clone(), A: var_field!(inUnit.A, Unit::UNIT).clone() * inExp.clone(), g: var_field!(inUnit.g, Unit::UNIT).clone() * inExp.clone(), m: var_field!(inUnit.m, Unit::UNIT).clone() * inExp.clone(), s: var_field!(inUnit.s, Unit::UNIT).clone() * inExp.clone() },
        _ => bail!("match: no arm matched"),
    });
    Ok(outUnit)
}

pub fn unitMulReal(mut inUnit: Unit, mut inFactor: metamodelica::Real) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = (match inUnit.clone() {
        mut unit @ Unit::UNIT { .. } => {
            let __owned_variant_factor_0 = var_field!(unit.factor, Unit::UNIT).clone() * inFactor.clone();
            if let Unit::UNIT { factor, .. } = &mut unit {
                *factor = __owned_variant_factor_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Unit::UNIT"); }
            unit.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outUnit)
}

pub fn unitRoot(mut inUnit: Unit, mut inExponent: metamodelica::Real) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = (match inUnit.clone() {
        Unit::UNIT { .. } => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut factor: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut i: i32 = 0;
            let mut s: i32 = 0;
            let mut m: i32 = 0;
            let mut g: i32 = 0;
            let mut A: i32 = 0;
            let mut K: i32 = 0;
            let mut mol: i32 = 0;
            let mut cd: i32 = 0;
            i = ((inExponent.clone()).0 as i32);
            r = realDiv(metamodelica::OrderedFloat(1.0_f64), inExponent.clone());
            factor = realPow(var_field!(inUnit.factor, Unit::UNIT).clone(), r.clone());
            r = realDiv(intReal(var_field!(inUnit.s, Unit::UNIT).clone()), inExponent.clone());
            s = intDiv(var_field!(inUnit.s, Unit::UNIT).clone(), i.clone());
            let true = (realEq(r.clone(), intReal(s.clone()))) else { bail!("pattern mismatch") };
            r = realDiv(intReal(var_field!(inUnit.m, Unit::UNIT).clone()), inExponent.clone());
            m = intDiv(var_field!(inUnit.m, Unit::UNIT).clone(), i.clone());
            let true = (realEq(r.clone(), intReal(m.clone()))) else { bail!("pattern mismatch") };
            r = realDiv(intReal(var_field!(inUnit.g, Unit::UNIT).clone()), inExponent.clone());
            g = intDiv(var_field!(inUnit.g, Unit::UNIT).clone(), i.clone());
            let true = (realEq(r.clone(), intReal(g.clone()))) else { bail!("pattern mismatch") };
            r = realDiv(intReal(var_field!(inUnit.A, Unit::UNIT).clone()), inExponent.clone());
            A = intDiv(var_field!(inUnit.A, Unit::UNIT).clone(), i.clone());
            let true = (realEq(r.clone(), intReal(A.clone()))) else { bail!("pattern mismatch") };
            r = realDiv(intReal(var_field!(inUnit.K, Unit::UNIT).clone()), inExponent.clone());
            K = intDiv(var_field!(inUnit.K, Unit::UNIT).clone(), i.clone());
            let true = (realEq(r.clone(), intReal(K.clone()))) else { bail!("pattern mismatch") };
            r = realDiv(intReal(var_field!(inUnit.mol, Unit::UNIT).clone()), inExponent.clone());
            mol = intDiv(var_field!(inUnit.mol, Unit::UNIT).clone(), i.clone());
            let true = (realEq(r.clone(), intReal(mol.clone()))) else { bail!("pattern mismatch") };
            r = realDiv(intReal(var_field!(inUnit.cd, Unit::UNIT).clone()), inExponent.clone());
            cd = intDiv(var_field!(inUnit.cd, Unit::UNIT).clone(), i.clone());
            let true = (realEq(r.clone(), intReal(cd.clone()))) else { bail!("pattern mismatch") };
            Unit::UNIT { s: s.clone(), m: m.clone(), g: g.clone(), A: A.clone(), K: K.clone(), mol: mol.clone(), cd: cd.clone(), factor: factor.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outUnit)
}

pub fn unitString(mut inUnit: Unit, mut inHtU2S: UnitToStringTable) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut opt_s: Option<ArcStr> = None;
    let mut s: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    let mut s4: ArcStr = arcstr::literal!("");
    let mut s5: ArcStr = arcstr::literal!("");
    let mut s6: ArcStr = arcstr::literal!("");
    let mut s7: ArcStr = arcstr::literal!("");
    let mut sExponent: ArcStr = arcstr::literal!("");
    let mut b: bool = false;
    let mut unit: Unit;
    opt_s = UnorderedMap::get(inUnit.clone(), inHtU2S.clone());
    if isSome(opt_s.clone()) {
        let Some(__pa0) = (opt_s.clone()) else { bail!("pattern mismatch") };
        outString = __pa0.clone();
        return Ok(outString);
    }
    outString = ((match inUnit.clone() {
        mut unit @ Unit::UNIT { .. } => {
            s = (if (var_field!(unit.factor, Unit::UNIT).clone() == metamodelica::OrderedFloat(1.0_f64)) {literal!("")} else {prefix2String(var_field!(unit.factor, Unit::UNIT).clone())}).clone();
            b = false;
            sExponent = (if (intEq(var_field!(unit.mol, Unit::UNIT).clone(), 1)) {literal!("")} else {intString(var_field!(unit.mol, Unit::UNIT).clone())}).clone();
            s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mol")); __mm_s.push_str(&*sExponent.clone()); ArcStr::from(__mm_s) }).clone();
            s1 = (if (intEq(var_field!(unit.mol, Unit::UNIT).clone(), 0)) {literal!("")} else {s1.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.mol, Unit::UNIT).clone(), 0);
            s2 = (if (b.clone() && intNe(var_field!(unit.cd, Unit::UNIT).clone(), 0)) {literal!(".")} else {literal!("")}).clone();
            sExponent = (if (intEq(var_field!(unit.cd, Unit::UNIT).clone(), 1)) {literal!("")} else {intString(var_field!(unit.cd, Unit::UNIT).clone())}).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("cd")); __mm_s.push_str(&*sExponent.clone()); ArcStr::from(__mm_s) }).clone();
            s2 = (if (intEq(var_field!(unit.cd, Unit::UNIT).clone(), 0)) {literal!("")} else {s2.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.cd, Unit::UNIT).clone(), 0);
            s3 = (if (b.clone() && intNe(var_field!(unit.m, Unit::UNIT).clone(), 0)) {literal!(".")} else {literal!("")}).clone();
            sExponent = (if (intEq(var_field!(unit.m, Unit::UNIT).clone(), 1)) {literal!("")} else {intString(var_field!(unit.m, Unit::UNIT).clone())}).clone();
            s3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("m")); __mm_s.push_str(&*sExponent.clone()); ArcStr::from(__mm_s) }).clone();
            s3 = (if (intEq(var_field!(unit.m, Unit::UNIT).clone(), 0)) {literal!("")} else {s3.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.m, Unit::UNIT).clone(), 0);
            s4 = (if (b.clone() && intNe(var_field!(unit.s, Unit::UNIT).clone(), 0)) {literal!(".")} else {literal!("")}).clone();
            sExponent = (if (intEq(var_field!(unit.s, Unit::UNIT).clone(), 1)) {literal!("")} else {intString(var_field!(unit.s, Unit::UNIT).clone())}).clone();
            s4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("s")); __mm_s.push_str(&*sExponent.clone()); ArcStr::from(__mm_s) }).clone();
            s4 = (if (intEq(var_field!(unit.s, Unit::UNIT).clone(), 0)) {literal!("")} else {s4.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.s, Unit::UNIT).clone(), 0);
            s5 = (if (b.clone() && intNe(var_field!(unit.A, Unit::UNIT).clone(), 0)) {literal!(".")} else {literal!("")}).clone();
            sExponent = (if (intEq(var_field!(unit.A, Unit::UNIT).clone(), 1)) {literal!("")} else {intString(var_field!(unit.A, Unit::UNIT).clone())}).clone();
            s5 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s5.clone()); __mm_s.push_str(&*literal!("A")); __mm_s.push_str(&*sExponent.clone()); ArcStr::from(__mm_s) }).clone();
            s5 = (if (intEq(var_field!(unit.A, Unit::UNIT).clone(), 0)) {literal!("")} else {s5.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.A, Unit::UNIT).clone(), 0);
            s6 = (if (b.clone() && intNe(var_field!(unit.K, Unit::UNIT).clone(), 0)) {literal!(".")} else {literal!("")}).clone();
            sExponent = (if (intEq(var_field!(unit.K, Unit::UNIT).clone(), 1)) {literal!("")} else {intString(var_field!(unit.K, Unit::UNIT).clone())}).clone();
            s6 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s6.clone()); __mm_s.push_str(&*literal!("K")); __mm_s.push_str(&*sExponent.clone()); ArcStr::from(__mm_s) }).clone();
            s6 = (if (intEq(var_field!(unit.K, Unit::UNIT).clone(), 0)) {literal!("")} else {s6.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.K, Unit::UNIT).clone(), 0);
            s7 = (if (b.clone() && intNe(var_field!(unit.g, Unit::UNIT).clone(), 0)) {literal!(".")} else {literal!("")}).clone();
            sExponent = (if (intEq(var_field!(unit.g, Unit::UNIT).clone(), 1)) {literal!("")} else {intString(var_field!(unit.g, Unit::UNIT).clone())}).clone();
            s7 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s7.clone()); __mm_s.push_str(&*literal!("g")); __mm_s.push_str(&*sExponent.clone()); ArcStr::from(__mm_s) }).clone();
            s7 = (if (intEq(var_field!(unit.g, Unit::UNIT).clone(), 0)) {literal!("")} else {s7.clone()}).clone();
            b = b.clone() || intNe(var_field!(unit.g, Unit::UNIT).clone(), 0);
            s = (if (b.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*s5.clone()); __mm_s.push_str(&*s6.clone()); __mm_s.push_str(&*s7.clone()); ArcStr::from(__mm_s) }} else {literal!("1")}).clone();
            s.clone()
        },
        _ => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function Unit.unitString failed for \"")); __mm_s.push_str(&*unit2string(inUnit.clone())?); __mm_s.push_str(&*literal!("\".")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn prefix2String(mut inReal: metamodelica::Real) -> ArcStr {
    let mut outPrefix: ArcStr = arcstr::literal!("");
    outPrefix = ((match inReal.clone() {
        __rlit_0 if __rlit_0.eq(&metamodelica::OrderedFloat((1e30) as f64)) => literal!("Q"),
        __rlit_1 if __rlit_1.eq(&metamodelica::OrderedFloat((1e27) as f64)) => literal!("R"),
        __rlit_2 if __rlit_2.eq(&metamodelica::OrderedFloat((1e24) as f64)) => literal!("Y"),
        __rlit_3 if __rlit_3.eq(&metamodelica::OrderedFloat((1e21) as f64)) => literal!("Z"),
        __rlit_4 if __rlit_4.eq(&metamodelica::OrderedFloat((1e18) as f64)) => literal!("E"),
        __rlit_5 if __rlit_5.eq(&metamodelica::OrderedFloat((1e15) as f64)) => literal!("P"),
        __rlit_6 if __rlit_6.eq(&metamodelica::OrderedFloat((1e12) as f64)) => literal!("T"),
        __rlit_7 if __rlit_7.eq(&metamodelica::OrderedFloat((1e9) as f64)) => literal!("G"),
        __rlit_8 if __rlit_8.eq(&metamodelica::OrderedFloat((1e6) as f64)) => literal!("M"),
        __rlit_9 if __rlit_9.eq(&metamodelica::OrderedFloat((1e3) as f64)) => literal!("k"),
        __rlit_10 if __rlit_10.eq(&metamodelica::OrderedFloat((1e2) as f64)) => literal!("h"),
        __rlit_11 if __rlit_11.eq(&metamodelica::OrderedFloat((1e1) as f64)) => literal!("da"),
        __rlit_12 if __rlit_12.eq(&metamodelica::OrderedFloat((1e-1) as f64)) => literal!("d"),
        __rlit_13 if __rlit_13.eq(&metamodelica::OrderedFloat((1e-2) as f64)) => literal!("c"),
        __rlit_14 if __rlit_14.eq(&metamodelica::OrderedFloat((1e-3) as f64)) => literal!("m"),
        __rlit_15 if __rlit_15.eq(&metamodelica::OrderedFloat((1e-6) as f64)) => literal!("u"),
        __rlit_16 if __rlit_16.eq(&metamodelica::OrderedFloat((1e-9) as f64)) => literal!("n"),
        __rlit_17 if __rlit_17.eq(&metamodelica::OrderedFloat((1e-12) as f64)) => literal!("p"),
        __rlit_18 if __rlit_18.eq(&metamodelica::OrderedFloat((1e-15) as f64)) => literal!("f"),
        __rlit_19 if __rlit_19.eq(&metamodelica::OrderedFloat((1e-18) as f64)) => literal!("a"),
        __rlit_20 if __rlit_20.eq(&metamodelica::OrderedFloat((1e-21) as f64)) => literal!("z"),
        __rlit_21 if __rlit_21.eq(&metamodelica::OrderedFloat((1e-24) as f64)) => literal!("y"),
        __rlit_22 if __rlit_22.eq(&metamodelica::OrderedFloat((1e-27) as f64)) => literal!("r"),
        __rlit_23 if __rlit_23.eq(&metamodelica::OrderedFloat((1e-30) as f64)) => literal!("q"),
        _ => realString(inReal.clone()),
    })).clone();
    outPrefix
}

pub fn parseUnitString(mut inUnitString: ArcStr, mut inKnownUnits: StringToUnitTable, mut info: SourceInfo) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut charList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    charList = stringListStringChar((inUnitString.clone()).clone());
    if charList.clone().is_empty() {
        bail!("fail");
    }
    if let Ok(__iflet0) = lexer(charList.clone()) {
        tokenList = __iflet0;
    } else {
        Error::addSourceMessage(Error::INVALID_UNIT.clone(), list![(inUnitString.clone()).clone()], info.clone())?;
        bail!("fail");
    }
    outUnit = parser3(list![true, true], tokenList.clone(), ONE().clone(), inKnownUnits.clone())?;
    if !(isUnit(outUnit.clone())) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFUnit.parseUnitString")); __mm_s.push_str(&*literal!(": failed to parse unit string ")); __mm_s.push_str(&*inUnitString.clone()); ArcStr::from(__mm_s) }).clone())?;
        }
    }
    Ok(outUnit)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn parser3(mut inMul: Arc<metamodelica::List<bool>>, mut inTokenList: Arc<metamodelica::List<Token>>, mut inUnit: Unit, mut inHtS2U: StringToUnitTable) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = 'mc: {
        let __mc_input = (inMul.clone(), inTokenList.clone(), inUnit.clone(), inHtS2U.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: true, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(inUnit.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: bMul, tail: bRest }, Deref @ metamodelica::List::Cons { head: Token::T_NUMBER { number: 1 }, tail: tokens }, _, _) => {
                    let mut ut: Unit;
                    ut = ONE().clone();
                    ut = if (bMul.clone()) {unitMul(inUnit.clone(), ut.clone())?} else {unitDiv(inUnit.clone(), ut.clone())?};
                    ut = parser3(bRest.clone(), tokens.clone(), ut.clone(), inHtS2U.clone())?;
                    Ok(ut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: bMul, tail: bRest }, Deref @ metamodelica::List::Cons { head: Token::T_UNIT { unit: s }, tail: Deref @ metamodelica::List::Cons { head: Token::T_NUMBER { number: exponent }, tail: tokens } }, _, _) => {
                    let mut ut: Unit;
                    ut = unitToken2unit((s.clone()).clone(), inHtS2U.clone())?;
                    ut = unitPow(ut.clone(), exponent.clone())?;
                    ut = if (bMul.clone()) {unitMul(inUnit.clone(), ut.clone())?} else {unitDiv(inUnit.clone(), ut.clone())?};
                    ut = parser3(bRest.clone(), tokens.clone(), ut.clone(), inHtS2U.clone())?;
                    Ok(ut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: bMul, tail: bRest }, Deref @ metamodelica::List::Cons { head: Token::T_UNIT { unit: s }, tail: tokens }, _, _) => {
                    let mut ut: Unit;
                    ut = unitToken2unit((s.clone()).clone(), inHtS2U.clone())?;
                    ut = if (bMul.clone()) {unitMul(inUnit.clone(), ut.clone())?} else {unitDiv(inUnit.clone(), ut.clone())?};
                    ut = parser3(bRest.clone(), tokens.clone(), ut.clone(), inHtS2U.clone())?;
                    Ok(ut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: bMul, tail: _ }, Deref @ metamodelica::List::Cons { head: Token::T_MUL { .. }, tail: Deref @ metamodelica::List::Cons { head: Token::T_LPAREN { .. }, tail: tokens } }, _, _) => {
                    let mut ut: Unit;
                    ut = parser3(cons(bMul.clone(), cons(bMul.clone(), inMul.clone())), tokens.clone(), inUnit.clone(), inHtS2U.clone())?;
                    Ok(ut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: bMul, tail: _ }, Deref @ metamodelica::List::Cons { head: Token::T_DIV { .. }, tail: Deref @ metamodelica::List::Cons { head: Token::T_LPAREN { .. }, tail: tokens } }, _, _) => {
                    let mut ut: Unit;
                    let mut b: bool = false;
                    b = !(bMul.clone());
                    ut = parser3(cons(b.clone(), cons(b.clone(), inMul.clone())), tokens.clone(), inUnit.clone(), inHtS2U.clone())?;
                    Ok(ut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: bRest }, Deref @ metamodelica::List::Cons { head: Token::T_RPAREN { .. }, tail: tokens }, _, _) => {
                    let mut ut: Unit;
                    ut = parser3(bRest.clone(), tokens.clone(), inUnit.clone(), inHtS2U.clone())?;
                    Ok(ut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: bMul, tail: _ }, Deref @ metamodelica::List::Cons { head: Token::T_MUL { .. }, tail: tokens }, _, _) => {
                    let mut ut: Unit;
                    ut = parser3(cons(bMul.clone(), inMul.clone()), tokens.clone(), inUnit.clone(), inHtS2U.clone())?;
                    Ok(ut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: bMul, tail: _ }, Deref @ metamodelica::List::Cons { head: Token::T_DIV { .. }, tail: tokens }, _, _) => {
                    let mut ut: Unit;
                    let mut b: bool = false;
                    b = !(bMul.clone());
                    ut = parser3(cons(b.clone(), inMul.clone()), tokens.clone(), inUnit.clone(), inHtS2U.clone())?;
                    Ok(ut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Unit::UNKNOWN { unit: (literal!("")).clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outUnit)
}

fn unitToken2unit(mut inS: ArcStr, mut inHtS2U: StringToUnitTable) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut opt_unit: Option<Unit> = None;
    let mut s: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    opt_unit = UnorderedMap::get((inS.clone()).clone(), inHtS2U.clone());
    if isSome(opt_unit.clone()) {
        let Some(__pa0) = (opt_unit.clone()) else { bail!("pattern mismatch") };
        outUnit = __pa0.clone();
    } else {
        s = (stringGetStringChar((inS.clone()).clone(), 1)?).clone();
        (r, s) = getPrefix((s.clone()).clone(), (inS.clone()).clone())?;
        outUnit = unitToken2unit((s.clone()).clone(), inHtS2U.clone())?;
        outUnit = unitMulReal(outUnit.clone(), r.clone())?;
    }
    Ok(outUnit)
}

fn getPrefix(mut inS: ArcStr, mut inS2: ArcStr) -> Result<(metamodelica::Real, ArcStr)> {
    let mut outR: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut outUnit: ArcStr = arcstr::literal!("");
    (outR, outUnit) = 'mc: {
        let __mc_input = (inS.clone(), inS2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "y", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-24_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "z", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-21_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "a", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-18_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "f", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-15_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "p", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-12_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "u", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-6_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "m", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-3_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "c", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-2_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "d", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    strRest = stringListStringChar((inS2.clone()).clone());
                    let __pa0 = ::match_deref::match_deref! { match &(strRest.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "d", tail: Deref @ metamodelica::List::Cons { head: Deref @ "a", tail: __pa0 } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e1_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "d", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e-1_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "h", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e2_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "k", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e3_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "M", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e6_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "G", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e9_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "T", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e12_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "P", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e15_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "E", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e18_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "Z", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e21_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "Y", _) => {
                    let mut strRest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inS2.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    strRest = __pa0.clone();
                    s = (stringCharListString(strRest.clone())).clone();
                    Ok((metamodelica::OrderedFloat(1e24_f64), s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outR, outUnit))
}

fn lexer(mut inCharList: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Token>>> {
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    outTokenList = 'mc: {
        let __mc_input = inCharList.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ ".", tail: charList } => {
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    tokenList = lexer(charList.clone())?;
                    Ok(cons(crate::NFUnit::Token::T_MUL, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: charList } => {
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    tokenList = lexer(charList.clone())?;
                    Ok(cons(crate::NFUnit::Token::T_LPAREN, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ ")", tail: charList } => {
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    tokenList = lexer(charList.clone())?;
                    Ok(cons(crate::NFUnit::Token::T_RPAREN, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: charList } => {
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    tokenList = lexer(charList.clone())?;
                    Ok(cons(crate::NFUnit::Token::T_DIV, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "+", tail: charList } => {
                    let mut number: ArcStr = arcstr::literal!("");
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let mut i: i32 = 0;
                    let mut charList = (*charList).clone();
                    (charList, number) = popNumber(charList.clone())?;
                    let false = (number.clone() == literal!("")) else { bail!("pattern mismatch") };
                    tokenList = lexer(charList.clone())?;
                    i = stringInt((number.clone()).clone())?;
                    Ok(cons(Token::T_NUMBER { number: i.clone() }, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "-", tail: charList } => {
                    let mut number: ArcStr = arcstr::literal!("");
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let mut i: i32 = 0;
                    let mut charList = (*charList).clone();
                    (charList, number) = popNumber(charList.clone())?;
                    let false = (number.clone() == literal!("")) else { bail!("pattern mismatch") };
                    tokenList = lexer(charList.clone())?;
                    i = -(stringInt((number.clone()).clone())?);
                    Ok(cons(Token::T_NUMBER { number: i.clone() }, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                charList => {
                    let mut number: ArcStr = arcstr::literal!("");
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let mut i: i32 = 0;
                    let mut charList = (*charList).clone();
                    (charList, number) = popNumber(charList.clone())?;
                    let false = (number.clone() == literal!("")) else { bail!("pattern mismatch") };
                    tokenList = lexer(charList.clone())?;
                    i = stringInt((number.clone()).clone())?;
                    Ok(cons(Token::T_NUMBER { number: i.clone() }, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                charList => {
                    let mut unit: ArcStr = arcstr::literal!("");
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let mut charList = (*charList).clone();
                    (charList, unit) = popUnit(charList.clone())?;
                    let false = (unit.clone() == literal!("")) else { bail!("pattern mismatch") };
                    tokenList = lexer(charList.clone())?;
                    Ok(cons(Token::T_UNIT { unit: (unit.clone()).clone() }, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTokenList)
}

fn popUnit(mut inCharList: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr)> {
    let mut outCharList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outUnit: ArcStr = arcstr::literal!("");
    (outCharList, outUnit) = 'mc: {
        let __mc_input = inCharList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), literal!("")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: s1, tail: strRest } => {
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut strRest = (*strRest).clone();
                    let true = (stringCompare((s1.clone()).clone(), (literal!("a")).clone()) >= 0 && stringCompare((s1.clone()).clone(), (literal!("z")).clone()) <= 0) else { bail!("pattern mismatch") };
                    (strRest, s2) = popUnit(strRest.clone())?;
                    Ok((strRest.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: s1, tail: strRest } => {
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut strRest = (*strRest).clone();
                    let true = (stringCompare((s1.clone()).clone(), (literal!("A")).clone()) >= 0 && stringCompare((s1.clone()).clone(), (literal!("Z")).clone()) <= 0) else { bail!("pattern mismatch") };
                    (strRest, s2) = popUnit(strRest.clone())?;
                    Ok((strRest.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCharList.clone(), literal!("")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCharList, outUnit))
}

fn popNumber(mut inCharList: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr)> {
    let mut outCharList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outNumber: ArcStr = arcstr::literal!("");
    (outCharList, outNumber) = 'mc: {
        let __mc_input = inCharList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), literal!("")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: s1, tail: strRest } => {
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut i: i32 = 0;
                    let mut strRest = (*strRest).clone();
                    i = stringInt((s1.clone()).clone())?;
                    let true = (intString(i.clone()) == s1.clone()) else { bail!("pattern mismatch") };
                    (strRest, s2) = popNumber(strRest.clone())?;
                    Ok((strRest.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCharList.clone(), literal!("")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCharList, outNumber))
}

