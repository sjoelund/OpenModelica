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

use crate::ComponentReference;
use crate::FHashTableStringToUnit as HashTableStringToUnit;
use crate::FHashTableUnitToString as HashTableUnitToString;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Util;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Unit {
    /// based on SI base units
    UNIT {
        /// prefix
        factor: metamodelica::Real,
        /// exponent
        mol: i32,
        /// exponent
        cd: i32,
        /// exponent
        m: i32,
        /// exponent
        s: i32,
        /// exponent
        A: i32,
        /// exponent
        K: i32,
        /// exponent
        g: i32,
    },
    /// unknown unit that belongs to all the variables from varList
    MASTER {
        varList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
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
impl Default for Token {
    fn default() -> Self { Self::T_MUL }
}
pub use self::Token::{T_NUMBER,T_UNIT,T_MUL,T_DIV,T_LPAREN,T_RPAREN};

pub static UPDATECREF: std::sync::LazyLock<Arc<DAE::ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("jhagemann")).clone(), identType: DAE::T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

pub static LU_COMPLEXUNITS: std::sync::LazyLock<Arc<metamodelica::List<(ArcStr, Unit)>>> = std::sync::LazyLock::new(|| { list![(literal!("mol"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 1, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 }), (literal!("cd"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 1, m: 0, s: 0, A: 0, K: 0, g: 0 }), (literal!("m"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 1, s: 0, A: 0, K: 0, g: 0 }), (literal!("s"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 }), (literal!("A"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 1, K: 0, g: 0 }), (literal!("K"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 1, g: 0 }), (literal!("g"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 1 }), (literal!("V"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -3, A: -1, K: 0, g: 1 }), (literal!("W"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -3, A: 0, K: 0, g: 1 }), (literal!("Hz"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: -1, A: 0, K: 0, g: 0 }), (literal!("Ohm"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -3, A: -2, K: 0, g: 1 }), (literal!("F"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e-3_f64), mol: 0, cd: 0, m: -2, s: 4, A: 2, K: 0, g: -1 }), (literal!("H"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -2, A: -2, K: 0, g: 1 }), (literal!("C"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 1, K: 0, g: 0 }), (literal!("T"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 0, s: -2, A: -1, K: 0, g: 1 }), (literal!("S"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e-3_f64), mol: 0, cd: 0, m: -2, s: 3, A: 2, K: 0, g: -1 }), (literal!("Wb"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -2, A: -1, K: 0, g: 1 }), (literal!("N"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 1, s: -2, A: 0, K: 0, g: 1 }), (literal!("Pa"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: -1, s: -2, A: 0, K: 0, g: 1 }), (literal!("J"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -2, A: 0, K: 0, g: 1 }), (literal!("min"), Unit::UNIT { factor: metamodelica::OrderedFloat(6e1_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 }), (literal!("h"), Unit::UNIT { factor: metamodelica::OrderedFloat(3.6e3_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 }), (literal!("d"), Unit::UNIT { factor: metamodelica::OrderedFloat(8.64e4_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 }), (literal!("l"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e-3_f64), mol: 0, cd: 0, m: 3, s: 0, A: 0, K: 0, g: 0 }), (literal!("kg"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 1 }), (literal!("kat"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 1, cd: 0, m: 0, s: -1, A: 0, K: 0, g: 0 }), (literal!("1"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 }), (literal!("rad"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 }), (literal!("degC"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 1, g: 0 }), (literal!("degF"), Unit::UNIT { factor: metamodelica::OrderedFloat(0.55555555555555555555555555555555555555_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 1, g: 0 })] });

//°Fahrenheit
//("degF", UNIT(5.0 / 9.0, 0, 0, 0, 0, 0, 1, 0, 459.67)), //°Fahrenheit
//("degC",       UNIT(1e0, 0, 0, 0, 0, 0, 1, 0, 273.15))};//°Celsius
/*                 fac, mol, cd, m, s, A, K, g*/
pub fn getKnownUnits() -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr))> {
    let mut outKnownUnits: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr));
    outKnownUnits = HashTableStringToUnit::emptyHashTableSized(Util::nextPrime(4 * (LU_COMPLEXUNITS.clone().len() as i32)));
    for mut unit in &*LU_COMPLEXUNITS.clone() {
        let mut unit = unit.clone();
        outKnownUnits = BaseHashTable::add(unit.clone(), outKnownUnits.clone())?;
    }
    Ok(outKnownUnits)
}

pub fn getKnownUnitsInverse() -> Result<(metamodelica::Array<Arc<metamodelica::List<(Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))> {
    let mut outKnownUnitsInverse: (metamodelica::Array<Arc<metamodelica::List<(Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr));
    let mut s: ArcStr = arcstr::literal!("");
    let mut ut: Unit;
    outKnownUnitsInverse = HashTableUnitToString::emptyHashTableSized(Util::nextPrime(4 * (LU_COMPLEXUNITS.clone().len() as i32)));
    for mut unit in &*LU_COMPLEXUNITS.clone() {
        let mut unit = unit.clone();
        (s, ut) = unit.clone();
        if !(BaseHashTable::hasKey(ut.clone(), outKnownUnitsInverse.clone())) {
            outKnownUnitsInverse = BaseHashTable::add((ut.clone(), s.clone()), outKnownUnitsInverse.clone())?;
        }
    }
    Ok(outKnownUnitsInverse)
}

pub fn isUnit(mut inUnit: Unit) -> bool {
    let mut b: bool = false;
    b = (match inUnit.clone() {
        Unit::UNIT { .. } => true,
        _ => false,
    });
    b
}

pub fn hashUnit(mut inKey: Unit) -> Result<i32> {
    let mut outHash: i32 = 0;
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (unit2string(inKey.clone())?).clone();
    outHash = stringHashDjb2((r#str.clone()).clone());
    Ok(outHash)
}

pub fn unitEqual(mut inKey: Unit, mut inKey2: Unit) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (inKey.clone(), inKey2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::UNIT { factor: mut factor1, mol: mut i1, cd: mut i2, m: mut i3, s: mut i4, A: mut i5, K: mut i6, g: mut i7 }, Unit::UNIT { factor: mut factor2, mol: mut j1, cd: mut j2, m: mut j3, s: mut j4, A: mut j5, K: mut j6, g: mut j7 }) = __mc_input.clone() else { bail!("nomatch") };
            let true = (realEq(factor1.clone(), factor2.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i1.clone(), j1.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i2.clone(), j2.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i3.clone(), j3.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i4.clone(), j4.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i5.clone(), j5.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i6.clone(), j6.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i7.clone(), j7.clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::UNIT { factor: mut factor1, mol: mut i1, cd: mut i2, m: mut i3, s: mut i4, A: mut i5, K: mut i6, g: mut i7 }, Unit::UNIT { factor: mut factor2, mol: mut j1, cd: mut j2, m: mut j3, s: mut j4, A: mut j5, K: mut j6, g: mut j7 }) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r = realMax(realAbs(factor1.clone()), realAbs(factor2.clone()));
            let true = (realLe(realDiv(realAbs((factor1.clone()) - (factor2.clone())), r.clone()), metamodelica::OrderedFloat(1e-3_f64))) else { bail!("pattern mismatch") };
            let true = (intEq(i1.clone(), j1.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i2.clone(), j2.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i3.clone(), j3.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i4.clone(), j4.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i5.clone(), j5.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i6.clone(), j6.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i7.clone(), j7.clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::MASTER { .. }, Unit::MASTER { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::UNKNOWN { unit: mut s }, Unit::UNKNOWN { unit: mut s2 }) = __mc_input.clone() else { bail!("nomatch") };
            let true = (stringEqual((s.clone()).clone(), (s2.clone()).clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn unit2string(mut inUnit: Unit) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inUnit.clone() {
        Unit::UNIT { factor: mut factor1, mol: mut i1, cd: mut i2, m: mut i3, s: mut i4, A: mut i5, K: mut i6, g: mut i7 } => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut b: bool = false;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*realString(factor1.clone())); __mm_s.push_str(&*literal!(" * ")); ArcStr::from(__mm_s) }).clone();
            b = false;
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mol^(")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(i1.clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(i1.clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(i2.clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd^(")); __mm_s.push_str(&*intString(i2.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(i2.clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(i2.clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(i3.clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("m^(")); __mm_s.push_str(&*intString(i3.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(i3.clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(i3.clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(i4.clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("s^(")); __mm_s.push_str(&*intString(i4.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(i4.clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(i4.clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(i5.clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("A^(")); __mm_s.push_str(&*intString(i5.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(i5.clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(i5.clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(i6.clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("K^(")); __mm_s.push_str(&*intString(i6.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(i6.clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(i6.clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone() && intNe(i7.clone(), 0)) {literal!(" * ")} else {literal!("")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("g^(")); __mm_s.push_str(&*intString(i7.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s = (if (intEq(i7.clone(), 0)) {literal!("")} else {s.clone()}).clone();
            b = b.clone() || intNe(i7.clone(), 0);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s = (if (b.clone()) {literal!("")} else {literal!("1")}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Unit::MASTER { varList: ref crefList } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (literal!("MASTER(")).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*printListCr(crefList.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Unit::UNKNOWN { unit: mut s } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNKOWN(")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn printListCr(mut inlCr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    outS = ((::match_deref::match_deref! { match &(inlCr.clone()) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: cr, tail: Deref @ metamodelica::List::Nil } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (ComponentReference::crefStr(cr.clone())?).clone();
            s.clone()
        },
        Deref @ metamodelica::List::Cons { head: cr, tail: lCr } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (ComponentReference::crefStr(cr.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*printListCr(lCr.clone())?); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outS)
}

pub fn unitMul(mut inUnit1: Unit, mut inUnit2: Unit) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut factor1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut factor2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i3: i32 = 0;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;
    let mut i6: i32 = 0;
    let mut i7: i32 = 0;
    let mut j1: i32 = 0;
    let mut j2: i32 = 0;
    let mut j3: i32 = 0;
    let mut j4: i32 = 0;
    let mut j5: i32 = 0;
    let mut j6: i32 = 0;
    let mut j7: i32 = 0;
    let Unit::UNIT { factor: __pa0, mol: __pa1, cd: __pa2, m: __pa3, s: __pa4, A: __pa5, K: __pa6, g: __pa7 } = (inUnit1.clone()) else { bail!("pattern mismatch") };
    factor1 = __pa0.clone();
    i1 = __pa1.clone();
    i2 = __pa2.clone();
    i3 = __pa3.clone();
    i4 = __pa4.clone();
    i5 = __pa5.clone();
    i6 = __pa6.clone();
    i7 = __pa7.clone();
    let Unit::UNIT { factor: __pa8, mol: __pa9, cd: __pa10, m: __pa11, s: __pa12, A: __pa13, K: __pa14, g: __pa15 } = (inUnit2.clone()) else { bail!("pattern mismatch") };
    factor2 = __pa8.clone();
    j1 = __pa9.clone();
    j2 = __pa10.clone();
    j3 = __pa11.clone();
    j4 = __pa12.clone();
    j5 = __pa13.clone();
    j6 = __pa14.clone();
    j7 = __pa15.clone();
    factor1 = factor1.clone() * factor2.clone();
    i1 = i1.clone() + j1.clone();
    i2 = i2.clone() + j2.clone();
    i3 = i3.clone() + j3.clone();
    i4 = i4.clone() + j4.clone();
    i5 = i5.clone() + j5.clone();
    i6 = i6.clone() + j6.clone();
    i7 = i7.clone() + j7.clone();
    outUnit = Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() };
    Ok(outUnit)
}

pub fn unitDiv(mut inUnit1: Unit, mut inUnit2: Unit) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut factor1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut factor2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i3: i32 = 0;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;
    let mut i6: i32 = 0;
    let mut i7: i32 = 0;
    let mut j1: i32 = 0;
    let mut j2: i32 = 0;
    let mut j3: i32 = 0;
    let mut j4: i32 = 0;
    let mut j5: i32 = 0;
    let mut j6: i32 = 0;
    let mut j7: i32 = 0;
    let Unit::UNIT { factor: __pa0, mol: __pa1, cd: __pa2, m: __pa3, s: __pa4, A: __pa5, K: __pa6, g: __pa7 } = (inUnit1.clone()) else { bail!("pattern mismatch") };
    factor1 = __pa0.clone();
    i1 = __pa1.clone();
    i2 = __pa2.clone();
    i3 = __pa3.clone();
    i4 = __pa4.clone();
    i5 = __pa5.clone();
    i6 = __pa6.clone();
    i7 = __pa7.clone();
    let Unit::UNIT { factor: __pa8, mol: __pa9, cd: __pa10, m: __pa11, s: __pa12, A: __pa13, K: __pa14, g: __pa15 } = (inUnit2.clone()) else { bail!("pattern mismatch") };
    factor2 = __pa8.clone();
    j1 = __pa9.clone();
    j2 = __pa10.clone();
    j3 = __pa11.clone();
    j4 = __pa12.clone();
    j5 = __pa13.clone();
    j6 = __pa14.clone();
    j7 = __pa15.clone();
    factor1 = factor1.clone() / factor2.clone();
    i1 = i1.clone() - j1.clone();
    i2 = i2.clone() - j2.clone();
    i3 = i3.clone() - j3.clone();
    i4 = i4.clone() - j4.clone();
    i5 = i5.clone() - j5.clone();
    i6 = i6.clone() - j6.clone();
    i7 = i7.clone() - j7.clone();
    outUnit = Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() };
    Ok(outUnit)
}

pub fn unitPow(mut inUnit: Unit, mut inExp: i32) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut factor: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i3: i32 = 0;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;
    let mut i6: i32 = 0;
    let mut i7: i32 = 0;
    let Unit::UNIT { factor: __pa0, mol: __pa1, cd: __pa2, m: __pa3, s: __pa4, A: __pa5, K: __pa6, g: __pa7 } = (inUnit.clone()) else { bail!("pattern mismatch") };
    factor = __pa0.clone();
    i1 = __pa1.clone();
    i2 = __pa2.clone();
    i3 = __pa3.clone();
    i4 = __pa4.clone();
    i5 = __pa5.clone();
    i6 = __pa6.clone();
    i7 = __pa7.clone();
    factor = realPow(factor.clone(), intReal(inExp.clone()));
    i1 = i1.clone() * inExp.clone();
    i2 = i2.clone() * inExp.clone();
    i3 = i3.clone() * inExp.clone();
    i4 = i4.clone() * inExp.clone();
    i5 = i5.clone() * inExp.clone();
    i6 = i6.clone() * inExp.clone();
    i7 = i7.clone() * inExp.clone();
    outUnit = Unit::UNIT { factor: factor.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() };
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
        _ => {
            bail!("fail")
        },
    });
    Ok(outUnit)
}

pub fn unitRoot(mut inUnit: Unit, mut inExponent: metamodelica::Real) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut factor: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut i: i32 = 0;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i3: i32 = 0;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;
    let mut i6: i32 = 0;
    let mut i7: i32 = 0;
    i = ((inExponent.clone()).0 as i32);
    r = realDiv(metamodelica::OrderedFloat(1.0_f64), inExponent.clone());
    let Unit::UNIT { factor: __pa0, mol: __pa1, cd: __pa2, m: __pa3, s: __pa4, A: __pa5, K: __pa6, g: __pa7 } = (inUnit.clone()) else { bail!("pattern mismatch") };
    factor = __pa0.clone();
    i1 = __pa1.clone();
    i2 = __pa2.clone();
    i3 = __pa3.clone();
    i4 = __pa4.clone();
    i5 = __pa5.clone();
    i6 = __pa6.clone();
    i7 = __pa7.clone();
    factor = realPow(factor.clone(), r.clone());
    r = realDiv(intReal(i1.clone()), inExponent.clone());
    i1 = intDiv(i1.clone(), i.clone());
    let true = (realEq(r.clone(), intReal(i1.clone()))) else { bail!("pattern mismatch") };
    r = realDiv(intReal(i2.clone()), inExponent.clone());
    i2 = intDiv(i2.clone(), i.clone());
    let true = (realEq(r.clone(), intReal(i2.clone()))) else { bail!("pattern mismatch") };
    r = realDiv(intReal(i3.clone()), inExponent.clone());
    i3 = intDiv(i3.clone(), i.clone());
    let true = (realEq(r.clone(), intReal(i3.clone()))) else { bail!("pattern mismatch") };
    r = realDiv(intReal(i4.clone()), inExponent.clone());
    i4 = intDiv(i4.clone(), i.clone());
    let true = (realEq(r.clone(), intReal(i4.clone()))) else { bail!("pattern mismatch") };
    r = realDiv(intReal(i5.clone()), inExponent.clone());
    i5 = intDiv(i5.clone(), i.clone());
    let true = (realEq(r.clone(), intReal(i5.clone()))) else { bail!("pattern mismatch") };
    r = realDiv(intReal(i6.clone()), inExponent.clone());
    i6 = intDiv(i6.clone(), i.clone());
    let true = (realEq(r.clone(), intReal(i6.clone()))) else { bail!("pattern mismatch") };
    r = realDiv(intReal(i7.clone()), inExponent.clone());
    i7 = intDiv(i7.clone(), i.clone());
    let true = (realEq(r.clone(), intReal(i7.clone()))) else { bail!("pattern mismatch") };
    outUnit = Unit::UNIT { factor: factor.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() };
    Ok(outUnit)
}

pub fn unitString(mut inUnit: Unit, mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inUnit.clone() {
        _ if (BaseHashTable::hasKey(inUnit.clone(), inHtU2S.clone())) => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (BaseHashTable::get(inUnit.clone(), inHtU2S.clone())?).clone();
            s.clone()
        },
        mut unit @ Unit::UNIT { .. } => {
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
            s = (prefix2String(var_field!(unit.factor, Unit::UNIT).clone())).clone();
            s = (if (realEq(var_field!(unit.factor, Unit::UNIT).clone(), metamodelica::OrderedFloat(1.0_f64))) {literal!("")} else {s.clone()}).clone();
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
        __rlit_0 if __rlit_0.eq(&metamodelica::OrderedFloat((1e-24) as f64)) => literal!("y"),
        __rlit_1 if __rlit_1.eq(&metamodelica::OrderedFloat((1e-21) as f64)) => literal!("z"),
        __rlit_2 if __rlit_2.eq(&metamodelica::OrderedFloat((1e-18) as f64)) => literal!("a"),
        __rlit_3 if __rlit_3.eq(&metamodelica::OrderedFloat((1e-15) as f64)) => literal!("f"),
        __rlit_4 if __rlit_4.eq(&metamodelica::OrderedFloat((1e-12) as f64)) => literal!("p"),
        __rlit_5 if __rlit_5.eq(&metamodelica::OrderedFloat((1e-6) as f64)) => literal!("u"),
        __rlit_6 if __rlit_6.eq(&metamodelica::OrderedFloat((1e-3) as f64)) => literal!("m"),
        __rlit_7 if __rlit_7.eq(&metamodelica::OrderedFloat((1e-2) as f64)) => literal!("c"),
        __rlit_8 if __rlit_8.eq(&metamodelica::OrderedFloat((1e-1) as f64)) => literal!("d"),
        __rlit_9 if __rlit_9.eq(&metamodelica::OrderedFloat((1e1) as f64)) => literal!("da"),
        __rlit_10 if __rlit_10.eq(&metamodelica::OrderedFloat((1e2) as f64)) => literal!("h"),
        __rlit_11 if __rlit_11.eq(&metamodelica::OrderedFloat((1e3) as f64)) => literal!("k"),
        __rlit_12 if __rlit_12.eq(&metamodelica::OrderedFloat((1e6) as f64)) => literal!("M"),
        __rlit_13 if __rlit_13.eq(&metamodelica::OrderedFloat((1e9) as f64)) => literal!("G"),
        __rlit_14 if __rlit_14.eq(&metamodelica::OrderedFloat((1e12) as f64)) => literal!("T"),
        __rlit_15 if __rlit_15.eq(&metamodelica::OrderedFloat((1e15) as f64)) => literal!("P"),
        __rlit_16 if __rlit_16.eq(&metamodelica::OrderedFloat((1e18) as f64)) => literal!("E"),
        __rlit_17 if __rlit_17.eq(&metamodelica::OrderedFloat((1e21) as f64)) => literal!("Z"),
        __rlit_18 if __rlit_18.eq(&metamodelica::OrderedFloat((1e24) as f64)) => literal!("Y"),
        _ => realString(inReal.clone()),
    })).clone();
    outPrefix
}

pub fn parseUnitString(mut inUnitString: ArcStr, mut inKnownUnits: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr))) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut charList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    charList = stringListStringChar((inUnitString.clone()).clone());
    if charList.clone().is_empty() {
        bail!("fail");
    }
    tokenList = lexer(charList.clone())?;
    outUnit = parser3(list![true, true], tokenList.clone(), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 }, inKnownUnits.clone())?;
    if !(isUnit(outUnit.clone())) {
        bail!("fail");
    }
    Ok(outUnit)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn parser3(mut inMul: Arc<metamodelica::List<bool>>, mut inTokenList: Arc<metamodelica::List<Token>>, mut inUnit: Unit, mut inHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr))) -> Result<Unit> {
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
                    ut = Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 };
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
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outUnit)
}

fn unitToken2unit(mut inS: ArcStr, mut inHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr))) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = 'mc: {
        let __mc_input = (inS.clone(), inHtS2U.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut ut: Unit;
            ut = BaseHashTable::get((inS.clone()).clone(), inHtS2U.clone())?;
            Ok(ut.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut s: ArcStr = arcstr::literal!("");
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ut: Unit;
            s = (stringGetStringChar((inS.clone()).clone(), 1)?).clone();
            (r, s) = getPrefix((s.clone()).clone(), (inS.clone()).clone())?;
            ut = unitToken2unit((s.clone()).clone(), inHtS2U.clone())?;
            ut = unitMulReal(ut.clone(), r.clone())?;
            Ok(ut.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
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
                    Ok(cons(crate::FUnit::Token::T_MUL, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "(", tail: charList } => {
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    tokenList = lexer(charList.clone())?;
                    Ok(cons(crate::FUnit::Token::T_LPAREN, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ ")", tail: charList } => {
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    tokenList = lexer(charList.clone())?;
                    Ok(cons(crate::FUnit::Token::T_RPAREN, tokenList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: charList } => {
                    let mut tokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    tokenList = lexer(charList.clone())?;
                    Ok(cons(crate::FUnit::Token::T_DIV, tokenList.clone()))
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
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function lexer failed")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
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

