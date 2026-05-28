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

/// represents a rational number, e.g. 6/7
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rational {
    /// numerator
    pub nom: i32,
    /// denominator
    pub denom: i32,
}

impl Default for Rational {
    fn default() -> Self {
        Self {
            nom: Default::default(),
            denom: Default::default(),
        }
    }
}

pub type RATIONAL = Rational;


pub static RAT0: Rational = Rational { nom: 0, denom: 1 };

pub static RAT1: Rational = Rational { nom: 1, denom: 1 };

pub fn isGreaterThan(mut r1: Rational, mut r2: Rational) -> bool {
    let mut b: bool = false;
    b = realGt(metamodelica::OrderedFloat((r1.nom.clone() / r1.denom.clone()) as f64), metamodelica::OrderedFloat((r2.nom.clone() / r2.denom.clone()) as f64));
    b
}

pub fn addRational(mut r1: Rational, mut r2: Rational) -> Result<Rational> {
    let mut r: Rational = <Rational as ::std::default::Default>::default();
    r = (match (r1.clone(), r2.clone()) {
        (Rational { nom: mut i1, denom: mut i2 }, Rational { nom: mut i3, denom: mut i4 }) => {
            let mut ri1: i32 = 0;
            let mut ri2: i32 = 0;
            let mut d: i32 = 0;
            ri1 = i1.clone() * i4.clone() + i3.clone() * i2.clone();
            ri2 = i2.clone() * i4.clone();
            d = intGcd(ri1.clone(), ri2.clone());
            ri1 = intDiv(ri1.clone(), d.clone());
            ri2 = intDiv(ri2.clone(), d.clone());
            normalizeZero(Rational { nom: ri1.clone(), denom: ri2.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(r)
}

fn normalizeZero(mut r: Rational) -> Rational {
    let mut outR: Rational = <Rational as ::std::default::Default>::default();
    outR = (match r.clone() {
        Rational { nom: 0, denom: _ } => Rational { nom: 0, denom: 1 },
        _ => r.clone(),
    });
    outR
}

pub fn rationalString(mut r: Rational) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match r.clone() {
        Rational { nom: mut n, denom: mut d } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(d.clone())); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn equals(mut r1: Rational, mut r2: Rational) -> Result<bool> {
    let mut res: bool = false;
    res = (match (r1.clone(), r2.clone()) {
        (Rational { nom: mut i1, denom: mut i2 }, Rational { nom: mut i3, denom: mut i4 }) => {
            i1.clone() * i4.clone() - i3.clone() * i2.clone() == 0
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(res)
}

pub fn subRational(mut r1: Rational, mut r2: Rational) -> Result<Rational> {
    let mut r: Rational = <Rational as ::std::default::Default>::default();
    r = (match (r1.clone(), r2.clone()) {
        (Rational { nom: mut i1, denom: mut i2 }, Rational { nom: mut i3, denom: mut i4 }) => {
            let mut ri1: i32 = 0;
            let mut ri2: i32 = 0;
            let mut d: i32 = 0;
            ri1 = i1.clone() * i4.clone() - i3.clone() * i2.clone();
            ri2 = i2.clone() * i4.clone();
            d = intGcd(ri1.clone(), ri2.clone());
            ri1 = intDiv(ri1.clone(), d.clone());
            ri2 = intDiv(ri2.clone(), d.clone());
            normalizeZero(Rational { nom: ri1.clone(), denom: ri2.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(r)
}

pub fn multRational(mut r1: Rational, mut r2: Rational) -> Result<Rational> {
    let mut r: Rational = <Rational as ::std::default::Default>::default();
    r = (match (r1.clone(), r2.clone()) {
        (Rational { nom: mut i1, denom: mut i2 }, Rational { nom: mut i3, denom: mut i4 }) => {
            let mut ri1: i32 = 0;
            let mut ri2: i32 = 0;
            let mut d: i32 = 0;
            ri1 = i1.clone() * i3.clone();
            ri2 = i2.clone() * i4.clone();
            d = intGcd(ri1.clone(), ri2.clone());
            ri1 = intDiv(ri1.clone(), d.clone());
            ri2 = intDiv(ri2.clone(), d.clone());
            normalizeZero(Rational { nom: ri1.clone(), denom: ri2.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(r)
}

pub fn divRational(mut r1: Rational, mut r2: Rational) -> Result<Rational> {
    let mut r: Rational = <Rational as ::std::default::Default>::default();
    r = (match (r1.clone(), r2.clone()) {
        (Rational { nom: mut i1, denom: mut i2 }, Rational { nom: mut i3, denom: mut i4 }) => {
            let mut ri1: i32 = 0;
            let mut ri2: i32 = 0;
            let mut d: i32 = 0;
            ri1 = i1.clone() * i4.clone();
            ri2 = i3.clone() * i2.clone();
            d = intGcd(ri1.clone(), ri2.clone());
            ri1 = intDiv(ri1.clone(), d.clone());
            ri2 = intDiv(ri2.clone(), d.clone());
            normalizeZero(Rational { nom: ri1.clone(), denom: ri2.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(r)
}

#[tailcall::tailcall]
pub fn intGcd(mut i1: i32, mut i2: i32) -> i32 {
    match i2.clone() {
        0 => i1.clone(),
        _ => tailcall::call!{ intGcd(i2.clone(), intMod(i1.clone(), i2.clone())) },
    }
}

/* Tests */
pub fn testRational() -> Result<()> {
    let () = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let Rational { nom: 7, denom: 6 } = (addRational(Rational { nom: 1, denom: 2 }, Rational { nom: 2, denom: 3 })?) else { bail!("pattern mismatch") };
            let Rational { nom: 2, denom: 1 } = (addRational(Rational { nom: 1, denom: 2 }, Rational { nom: 3, denom: 2 })?) else { bail!("pattern mismatch") };
            let Rational { nom: 1, denom: 1 } = (subRational(Rational { nom: 3, denom: 2 }, Rational { nom: 1, denom: 2 })?) else { bail!("pattern mismatch") };
            let Rational { nom: 1, denom: 3 } = (subRational(Rational { nom: 1, denom: 2 }, Rational { nom: 1, denom: 6 })?) else { bail!("pattern mismatch") };
            let Rational { nom: 4, denom: 3 } = (multRational(Rational { nom: 2, denom: 3 }, Rational { nom: 4, denom: 2 })?) else { bail!("pattern mismatch") };
            let Rational { nom: 1, denom: 1 } = (multRational(Rational { nom: 1, denom: 1 }, Rational { nom: 1, denom: 1 })?) else { bail!("pattern mismatch") };
            let Rational { nom: 1, denom: 2 } = (divRational(Rational { nom: 1, denom: 3 }, Rational { nom: 2, denom: 3 })?) else { bail!("pattern mismatch") };
            println!("{}", (literal!("testRational succeeded\n")).clone());
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("testRationals failed\n")).clone());
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

