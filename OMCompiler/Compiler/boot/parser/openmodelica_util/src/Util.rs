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

use crate::Autoconf;
use crate::ClockIndexes;
use crate::Global;
use crate::Print;
use crate::System;
use openmodelica_util_datatypes_basic::List;

/// Used to signal success or failure of a function call
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    SUCCESS,
    FAILURE,
}
pub use self::Status::{SUCCESS,FAILURE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    pub sec: i32,
    pub min: i32,
    pub hour: i32,
    pub mday: i32,
    pub mon: i32,
    pub year: i32,
}

pub type DATETIME = DateTime;


pub const HASH_SEED: i32 = 5381;

pub static dummyInfo: SourceInfo = SourceInfo { fileName: literal!(""), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };

pub fn isIntGreater(mut lhs: i32, mut rhs: i32) -> bool {
    let mut b: bool = lhs.clone() > rhs.clone();
    b
}

pub fn isRealGreater(mut lhs: metamodelica::Real, mut rhs: metamodelica::Real) -> bool {
    let mut b: bool = lhs.clone() > rhs.clone();
    b
}

pub fn linuxDotSlash() -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (arcstr::literal!(Autoconf::os)).clone();
    r#str = (if (r#str.clone() == literal!("linux") || r#str.clone() == literal!("darwin")) {literal!("./")} else {literal!("")}).clone();
    r#str
}

pub fn flagValue(mut flag: ArcStr, mut arguments: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut flagVal: ArcStr = arcstr::literal!("");
    let mut arg: ArcStr = arcstr::literal!("");
    let mut rest: Arc<metamodelica::List<ArcStr>> = arguments.clone();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest = __pa1.clone();
        if arg.clone() == flag.clone() {
            break;
        }
    }
    flagVal = (if (rest.clone().is_empty()) {literal!("")} else {listHead(rest.clone())?}).clone();
    Ok(flagVal)
}

pub fn selectFirstNonEmptyString(mut inStrings: Arc<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut outResult: ArcStr = arcstr::literal!("");
    for mut e in &*inStrings.clone() {
        let mut e = e.clone();
        if e.clone() != literal!("") {
            outResult = (e.clone()).clone();
            return outResult;
        }
    }
    outResult = (literal!("")).clone();
    outResult
}

pub fn compareTupleIntGt<T: Clone + 'static>(mut inTplA: (i32, T), mut inTplB: (i32, T)) -> bool {
    let mut res: bool = false;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    (a, _) = inTplA.clone();
    (b, _) = inTplB.clone();
    res = intGt(a.clone(), b.clone());
    res
}

pub fn compareTupleIntLt<T: Clone + 'static>(mut inTplA: (i32, T), mut inTplB: (i32, T)) -> bool {
    let mut res: bool = false;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    (a, _) = inTplA.clone();
    (b, _) = inTplB.clone();
    res = intLt(a.clone(), b.clone());
    res
}

pub fn compareTuple2IntGt<T: Clone + 'static>(mut inTplA: (T, i32), mut inTplB: (T, i32)) -> bool {
    let mut res: bool = false;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    (_, a) = inTplA.clone();
    (_, b) = inTplB.clone();
    res = intGt(a.clone(), b.clone());
    res
}

pub fn compareTuple2IntLt<T: Clone + 'static>(mut inTplA: (T, i32), mut inTplB: (T, i32)) -> bool {
    let mut res: bool = false;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    (_, a) = inTplA.clone();
    (_, b) = inTplB.clone();
    res = intLt(a.clone(), b.clone());
    res
}

pub fn tuple21<T1: Clone + 'static, T2: Clone + 'static>(mut inTuple: (T1, T2)) -> T1 {
    let mut outValue: T1;
    (outValue, _) = inTuple.clone();
    outValue
}

pub fn tuple22<T1: Clone + 'static, T2: Clone + 'static>(mut inTuple: (T1, T2)) -> T2 {
    let mut outValue: T2;
    (_, outValue) = inTuple.clone();
    outValue
}

pub fn optTuple22<T1: Clone + 'static, T2: Clone + 'static>(mut inTuple: Option<(T1, T2)>) -> Result<T2> {
    let mut outValue: T2;
    let Some((_, __pa0)) = (inTuple.clone()) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn tuple312<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(mut inTuple: (T1, T2, T3)) -> (T1, T2) {
    let mut outTuple: (T1, T2);
    let mut e1: T1;
    let mut e2: T2;
    (e1, e2, _) = inTuple.clone();
    outTuple = (e1.clone(), e2.clone());
    outTuple
}

pub fn tuple31<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(mut inValue: (T1, T2, T3)) -> T1 {
    let mut outValue: T1;
    (outValue, _, _) = inValue.clone();
    outValue
}

pub fn tuple32<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(mut inValue: (T1, T2, T3)) -> T2 {
    let mut outValue: T2;
    (_, outValue, _) = inValue.clone();
    outValue
}

pub fn tuple33<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(mut inValue: (T1, T2, T3)) -> T3 {
    let mut outValue: T3;
    (_, _, outValue) = inValue.clone();
    outValue
}

pub fn tuple41<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static>(mut inTuple: (T1, T2, T3, T4)) -> T1 {
    let mut outValue: T1;
    (outValue, _, _, _) = inTuple.clone();
    outValue
}

pub fn tuple42<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static>(mut inTuple: (T1, T2, T3, T4)) -> T2 {
    let mut outValue: T2;
    (_, outValue, _, _) = inTuple.clone();
    outValue
}

pub fn tuple43<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static>(mut inTuple: (T1, T2, T3, T4)) -> T3 {
    let mut outValue: T3;
    (_, _, outValue, _) = inTuple.clone();
    outValue
}

pub fn tuple44<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static>(mut inTuple: (T1, T2, T3, T4)) -> T4 {
    let mut outValue: T4;
    (_, _, _, outValue) = inTuple.clone();
    outValue
}

pub fn tuple51<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static, T5: Clone + 'static>(mut inTuple: (T1, T2, T3, T4, T5)) -> T1 {
    let mut outValue: T1;
    (outValue, _, _, _, _) = inTuple.clone();
    outValue
}

pub fn tuple52<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static, T5: Clone + 'static>(mut inTuple: (T1, T2, T3, T4, T5)) -> T2 {
    let mut outValue: T2;
    (_, outValue, _, _, _) = inTuple.clone();
    outValue
}

pub fn tuple53<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static, T5: Clone + 'static>(mut inTuple: (T1, T2, T3, T4, T5)) -> T3 {
    let mut outValue: T3;
    (_, _, outValue, _, _) = inTuple.clone();
    outValue
}

pub fn tuple54<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static, T5: Clone + 'static>(mut inTuple: (T1, T2, T3, T4, T5)) -> T4 {
    let mut outValue: T4;
    (_, _, _, outValue, _) = inTuple.clone();
    outValue
}

pub fn tuple55<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static, T5: Clone + 'static>(mut inTuple: (T1, T2, T3, T4, T5)) -> T5 {
    let mut outValue: T5;
    (_, _, _, _, outValue) = inTuple.clone();
    outValue
}

pub fn tuple61<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static, T5: Clone + 'static, T6: Clone + 'static>(mut inTuple: (T1, T2, T3, T4, T5, T6)) -> T1 {
    let mut outValue: T1;
    (outValue, _, _, _, _, _) = inTuple.clone();
    outValue
}

pub fn tuple62<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static, T5: Clone + 'static, T6: Clone + 'static>(mut inTuple: (T1, T2, T3, T4, T5, T6)) -> T2 {
    let mut outValue: T2;
    (_, outValue, _, _, _, _) = inTuple.clone();
    outValue
}

pub fn stringContainsChar(mut r#str: ArcStr, mut char: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let mut ch: i32 = 0;
    ch = stringCharInt((char.clone()).clone())?;
    for mut i in 1..=((r#str.clone()).clone().len() as i32) {
        if metamodelica::Dangerous::stringGet((r#str.clone()).clone(), i.clone())? == ch.clone() {
            res = true;
            return Ok(res);
        }
    }
    Ok(res)
}

pub fn stringDelimitListPrintBuf(mut inStringLst: Arc<metamodelica::List<ArcStr>>, mut inDelimiter: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inStringLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: f, tail: Deref @ metamodelica::List::Nil } => {
                    Print::printBuf((f.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: f, tail: r } => {
                    stringDelimitListPrintBuf(r.clone(), (inDelimiter.clone()).clone())?;
                    Print::printBuf((f.clone()).clone())?;
                    Print::printBuf((inDelimiter.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn stringDelimitListAndSeparate(mut r#str: Arc<metamodelica::List<ArcStr>>, mut sep1: ArcStr, mut sep2: ArcStr, mut n: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let mut handle: i32 = 0;
    handle = Print::saveAndClearBuf()?;
    stringDelimitListAndSeparate2(r#str.clone(), (sep1.clone()).clone(), (sep2.clone()).clone(), n.clone(), 0)?;
    res = (Print::getString()?).clone();
    Print::restoreBuf(handle.clone())?;
    Ok(res)
}

fn stringDelimitListAndSeparate2(mut inStringLst1: Arc<metamodelica::List<ArcStr>>, mut inString2: ArcStr, mut inString3: ArcStr, mut inInteger4: i32, mut inInteger5: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inStringLst1.clone(), inString2.clone(), inString3.clone(), inInteger4.clone(), inInteger5.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: s, tail: Deref @ metamodelica::List::Nil }, _, _, _, _) => {
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: f, tail: r }, sep1, sep2, n, 0) => {
                    Print::printBuf((f.clone()).clone())?;
                    Print::printBuf((sep1.clone()).clone())?;
                    stringDelimitListAndSeparate2(r.clone(), (sep1.clone()).clone(), (sep2.clone()).clone(), n.clone(), 1)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: f, tail: r }, sep1, sep2, n, iter) => {
                    let mut iter_1: i32 = 0;
                    let 0 = (intMod(iter.clone(), n.clone())) else { bail!("pattern mismatch") };
                    iter_1 = iter.clone() + 1;
                    Print::printBuf((f.clone()).clone())?;
                    Print::printBuf((sep1.clone()).clone())?;
                    Print::printBuf((sep2.clone()).clone())?;
                    stringDelimitListAndSeparate2(r.clone(), (sep1.clone()).clone(), (sep2.clone()).clone(), n.clone(), iter_1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: f, tail: r }, sep1, sep2, n, iter) => {
                    let mut iter_1: i32 = 0;
                    iter_1 = iter.clone() + 1;
                    Print::printBuf((f.clone()).clone())?;
                    Print::printBuf((sep1.clone()).clone())?;
                    stringDelimitListAndSeparate2(r.clone(), (sep1.clone()).clone(), (sep2.clone()).clone(), n.clone(), iter_1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("- stringDelimitListAndSeparate2 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn stringDelimitListNonEmptyElts(mut lst: Arc<metamodelica::List<ArcStr>>, mut delim: ArcStr) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut lst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    lst1 = List::select(lst.clone(), (std::sync::Arc::new(fnptr!(isNotEmptyString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>));
    r#str = stringDelimitList(lst1.clone(), (delim.clone()).clone());
    r#str
}

pub fn mulStringDelimit2Int(mut inString: ArcStr, mut delim: ArcStr) -> Result<i32> {
    let mut i: i32 = 0;
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut lst2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    lst = stringSplitAtChar((inString.clone()).clone(), (delim.clone()).clone())?;
    lst2 = List::map(lst.clone(), (std::sync::Arc::new(stringInt) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>));
    if !(lst2.clone().is_empty()) {
        i = List::fold(lst2.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1);
    } else {
        i = 0;
    }
    Ok(i)
}

pub fn stringReplaceChar(mut inString1: ArcStr, mut inString2: ArcStr, mut inString3: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (System::stringReplace((inString1.clone()).clone(), (inString2.clone()).clone(), (inString3.clone()).clone())?).clone();
    Ok(outString)
}

pub fn stringSplitAtChar(mut string: ArcStr, mut token: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut ch: i32 = stringCharInt((token.clone()).clone())?;
    let mut cur: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut c in &*stringListStringChar((string.clone()).clone()) {
        let mut c = c.clone();
        if stringCharInt((c.clone()).clone())? == ch.clone() {
            strings = cons(stringAppendList(cur.clone().reverse()), strings.clone());
            cur = metamodelica::nil();
        } else {
            cur = cons((c.clone()).clone(), cur.clone());
        }
    }
    if !(cur.clone().is_empty()) {
        strings = cons(stringAppendList(cur.clone().reverse()), strings.clone());
    }
    strings = strings.clone().reverse();
    Ok(strings)
}

pub fn optionToString<T: Clone + 'static>(mut ot: Option<T>, mut f: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> ArcStr {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr = arcstr::literal!("");
    let mut t: T;
    r#str = ((match ot.clone() {
        Some(mut t) => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SOME(")); __mm_s.push_str(&*f(t.clone()).unwrap()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => literal!("NONE()"),
    })).clone();
    r#str
}

pub fn applyOption<TI: Clone + 'static, TO: Clone + 'static>(mut inOption: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Option<TO> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outOption: Option<TO> = None;
    outOption = (match inOption.clone() {
        Some(mut ival) => {
            Some(inFunc(ival.clone()).unwrap())
        },
        _ => {
            None
        },
    });
    outOption
}

pub fn applyOption1<TI: Clone + 'static, TO: Clone + 'static, ArgT: Clone + 'static>(mut inOption: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>, mut inArg: ArgT) -> Option<TO> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outOption: Option<TO> = None;
    outOption = (match inOption.clone() {
        Some(mut ival) => {
            Some(inFunc(ival.clone(), inArg.clone()).unwrap())
        },
        _ => {
            None
        },
    });
    outOption
}

pub fn applyOptionOrDefault<TI: Clone + 'static, TO: Clone + 'static>(mut inValue: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>, mut inDefaultValue: TO) -> TO {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outValue: TO;
    outValue = (match inValue.clone() {
        Some(mut value) => {
            inFunc(value.clone()).unwrap()
        },
        _ => {
            inDefaultValue.clone()
        },
    });
    outValue
}

pub fn applyOptionOrDefault1<TI: Clone + 'static, TO: Clone + 'static, ArgT: Clone + 'static>(mut inValue: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>, mut inArg: ArgT, mut inDefaultValue: TO) -> TO {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outValue: TO;
    outValue = (match inValue.clone() {
        Some(mut value) => {
            inFunc(value.clone(), inArg.clone()).unwrap()
        },
        _ => {
            inDefaultValue.clone()
        },
    });
    outValue
}

pub fn applyOptionOrDefault2<TI: Clone + 'static, TO: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static>(mut inValue: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inDefaultValue: TO) -> TO {
    pub type FuncType<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outValue: TO;
    outValue = (match inValue.clone() {
        Some(mut value) => {
            inFunc(value.clone(), inArg1.clone(), inArg2.clone()).unwrap()
        },
        _ => {
            inDefaultValue.clone()
        },
    });
    outValue
}

pub fn applyOption_2<T: Clone + 'static>(mut inValue1: Option<T>, mut inValue2: Option<T>, mut inFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>) -> Result<Option<T>> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>;

    let mut outValue: Option<T> = None;
    outValue = (match (inValue1.clone(), inValue2.clone()) {
        (None, _) => inValue2.clone(),
        (_, None) => inValue1.clone(),
        _ => Some(inFunc(getOption(inValue1.clone())?, getOption(inValue2.clone())?)?),
    });
    Ok(outValue)
}

pub fn makeOption<T: Clone + 'static>(mut inValue: T) -> Option<T> {
    let mut outOption: Option<T> = Some(inValue.clone());
    outOption
}

pub fn makeOptionOnTrue<T: Clone + 'static>(mut inCondition: bool, mut inValue: T) -> Option<T> {
    let mut outOption: Option<T> = if (inCondition.clone()) {Some(inValue.clone())} else {None};
    outOption
}

pub fn getOption<T: Clone + 'static>(mut inOption: Option<T>) -> Result<T> {
    let mut outValue: T;
    let Some(__pa0) = (inOption.clone()) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getOptionOrDefault<T: Clone + 'static>(mut inOption: Option<T>, mut inDefault: T) -> T {
    let mut outValue: T;
    outValue = (match inOption.clone() {
        Some(mut value) => {
            value.clone()
        },
        _ => {
            inDefault.clone()
        },
    });
    outValue
}

pub fn intGreaterZero(mut v: i32) -> bool {
    let mut res: bool = v.clone() > 0;
    res
}

pub fn intPositive(mut v: i32) -> bool {
    let mut res: bool = v.clone() >= 0;
    res
}

pub fn intNegative(mut v: i32) -> bool {
    let mut res: bool = v.clone() < 0;
    res
}

pub fn intSign(mut i: i32) -> i32 {
    let mut o: i32 = if (i.clone() == 0) {0} else if (i.clone() > 0) {1} else {-1};
    o
}

pub fn intCompare(mut inN: i32, mut inM: i32) -> i32 {
    let mut outResult: i32 = if (inN.clone() == inM.clone()) {0} else if (inN.clone() > inM.clone()) {1} else {-1};
    outResult
}

pub fn intPow(mut base: i32, mut exponent: i32) -> Result<i32> {
    let mut result: i32 = 1;
    if exponent.clone() >= 0 {
        for mut i in 1..=exponent.clone() {
            result = result.clone() * base.clone();
        }
    } else {
        bail!("fail");
    }
    Ok(result)
}

pub fn realNegative(mut v: metamodelica::Real) -> bool {
    let mut res: bool = v.clone() < metamodelica::OrderedFloat((0) as f64);
    res
}

pub fn realCompare(mut inN: metamodelica::Real, mut inM: metamodelica::Real) -> i32 {
    let mut outResult: i32 = if (inN.clone() == inM.clone()) {0} else if (inN.clone() > inM.clone()) {1} else {-1};
    outResult
}

pub fn boolCompare(mut inN: bool, mut inM: bool) -> i32 {
    let mut outResult: i32 = if (inN.clone() == inM.clone()) {0} else if (inN.clone() > inM.clone()) {1} else {-1};
    outResult
}

pub fn isNotEmptyString(mut inString: ArcStr) -> bool {
    let mut outIsNotEmpty: bool = ((inString.clone()).clone().len() as i32) > 0;
    outIsNotEmpty
}

pub fn writeFileOrErrorMsg(mut inFilename: ArcStr, mut inString: ArcStr) -> Result<()> {
    if '__try0: {
        unwrap_break_err!(System::writeFile((inFilename.clone()).clone(), (inString.clone()).clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Print::printErrorBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("# Cannot write to file: ")); __mm_s.push_str(&*inFilename.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(())
}

pub fn strncmp(mut inString1: ArcStr, mut inString2: ArcStr, mut inLength: i32) -> bool {
    let mut outEqual: bool = false;
    outEqual = 0 == System::strncmp((inString1.clone()).clone(), (inString2.clone()).clone(), inLength.clone());
    outEqual
}

pub fn notStrncmp(mut inString1: ArcStr, mut inString2: ArcStr, mut inLength: i32) -> bool {
    let mut outEqual: bool = false;
    outEqual = 0 != System::strncmp((inString1.clone()).clone(), (inString2.clone()).clone(), inLength.clone());
    outEqual
}

pub fn tickStr() -> ArcStr {
    let mut s: ArcStr = intString(tick());
    s
}

pub fn replaceWindowsBackSlashWithPathDelimiter(mut inPath: ArcStr) -> Result<ArcStr> {
    let mut outPath: ArcStr = arcstr::literal!("");
    if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
        outPath = (System::stringReplace((inPath.clone()).clone(), (literal!("\\")).clone(), (arcstr::literal!(Autoconf::pathDelimiter)).clone())?).clone();
    } else {
        outPath = (inPath.clone()).clone();
    }
    Ok(outPath)
}

pub fn getAbsoluteDirectoryAndFile(mut filename: ArcStr) -> Result<(ArcStr, ArcStr)> {
    let mut dirname: ArcStr = arcstr::literal!("");
    let mut basename: ArcStr = arcstr::literal!("");
    let mut realpath: ArcStr = arcstr::literal!("");
    realpath = (System::realpath((filename.clone()).clone())?).clone();
    dirname = (System::dirname((realpath.clone()).clone())).clone();
    basename = (System::basename((realpath.clone()).clone())).clone();
    dirname = (replaceWindowsBackSlashWithPathDelimiter((dirname.clone()).clone())?).clone();
    Ok((dirname, basename))
}

pub fn rawStringToInputString(mut inString: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (System::stringReplace((inString.clone()).clone(), (literal!("\\\"")).clone(), (literal!("\"")).clone())?).clone();
    outString = (System::stringReplace((outString.clone()).clone(), (literal!("\\\\")).clone(), (literal!("\\")).clone())?).clone();
    Ok(outString)
}

pub fn escapeModelicaStringToCString(mut modelicaString: ArcStr) -> ArcStr {
    let mut cString: ArcStr = arcstr::literal!("");
    cString = (System::escapedString((modelicaString.clone()).clone(), true)).clone();
    cString
}

pub fn escapeModelicaStringToJLString(mut modelicaString: ArcStr) -> Result<ArcStr> {
    let mut cString: ArcStr = arcstr::literal!("");
    cString = (System::stringReplace((modelicaString.clone()).clone(), (literal!("$")).clone(), (literal!("")).clone())?).clone();
    cString = (System::stringReplace((cString.clone()).clone(), (literal!("\"")).clone(), (literal!("")).clone())?).clone();
    cString = (System::stringReplace((cString.clone()).clone(), (literal!("\"")).clone(), (literal!("")).clone())?).clone();
    cString = (System::stringReplace((cString.clone()).clone(), (literal!("\"\"")).clone(), (literal!("")).clone())?).clone();
    cString = (System::escapedString((cString.clone()).clone(), true)).clone();
    Ok(cString)
}

pub fn escapeModelicaStringToXmlString(mut modelicaString: ArcStr) -> Result<ArcStr> {
    let mut xmlString: ArcStr = arcstr::literal!("");
    xmlString = (System::stringReplace((modelicaString.clone()).clone(), (literal!("&")).clone(), (literal!("&amp;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString.clone()).clone(), (literal!("\"")).clone(), (literal!("&quot;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString.clone()).clone(), (literal!("<")).clone(), (literal!("&lt;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString.clone()).clone(), (literal!(">")).clone(), (literal!("&gt;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString.clone()).clone(), (literal!("\n")).clone(), (literal!("&#10;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString.clone()).clone(), (literal!("\r")).clone(), (literal!("&#13;")).clone())?).clone();
    Ok(xmlString)
}

pub fn makeQuotedIdentifier(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut quotedIdentifier: ArcStr = arcstr::literal!("");
    quotedIdentifier = (System::stringReplace((r#str.clone()).clone(), (literal!("\\")).clone(), (literal!("\\\\")).clone())?).clone();
    quotedIdentifier = (System::stringReplace((quotedIdentifier.clone()).clone(), (literal!("'")).clone(), (literal!("\\'")).clone())?).clone();
    quotedIdentifier = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*quotedIdentifier.clone()); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone();
    Ok(quotedIdentifier)
}

pub fn escapeQuotes(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut quotes: ArcStr = arcstr::literal!("");
    quotes = (System::stringReplace((r#str.clone()).clone(), (literal!("\\")).clone(), (literal!("\\\\")).clone())?).clone();
    quotes = (System::stringReplace((quotes.clone()).clone(), (literal!("'")).clone(), (literal!("\\'")).clone())?).clone();
    Ok(quotes)
}

pub fn makeTuple<T1: Clone + 'static, T2: Clone + 'static>(mut inValue1: T1, mut inValue2: T2) -> (T1, T2) {
    let mut outTuple: (T1, T2) = (inValue1.clone(), inValue2.clone());
    outTuple
}

pub fn makeTupleR<T1: Clone + 'static, T2: Clone + 'static>(mut inValue1: T1, mut inValue2: T2) -> (T2, T1) {
    let mut outTuple: (T2, T1) = (inValue2.clone(), inValue1.clone());
    outTuple
}

pub fn make3Tuple<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(mut inValue1: T1, mut inValue2: T2, mut inValue3: T3) -> (T1, T2, T3) {
    let mut outTuple: (T1, T2, T3) = (inValue1.clone(), inValue2.clone(), inValue3.clone());
    outTuple
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn mulListIntegerOpt(mut inList: Arc<metamodelica::List<Option<i32>>>, mut inAccum: i32) -> Result<i32> {
    let mut outResult: i32 = 0;
    outResult = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Nil => {
            inAccum.clone()
        },
        Deref @ metamodelica::List::Cons { head: Some(i), tail: rest } => {
            mulListIntegerOpt(rest.clone(), i.clone() * inAccum.clone())?
        },
        Deref @ metamodelica::List::Cons { head: None, tail: rest } => {
            mulListIntegerOpt(rest.clone(), inAccum.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outResult)
}

/// A single boolean value that can be updated (a destructive operation). NOTE: Use Mutable<Boolean> instead. This implementation is kept since Susan cannot use that type.
pub type StatefulBoolean = metamodelica::Array<bool>;

pub fn makeStatefulBoolean(mut b: bool) -> StatefulBoolean {
    let mut sb: StatefulBoolean = arrayCreate(1, b.clone());
    sb
}

pub fn getStatefulBoolean(mut sb: StatefulBoolean) -> bool {
    let mut b: bool = sb.borrow()[(1-1) as usize].clone();
    b
}

pub fn setStatefulBoolean(mut sb: StatefulBoolean, mut b: bool) -> Result<()> {
    {let _arr = sb.clone(); _arr.borrow_mut()[(1-1) as usize] = b.clone(); _arr};
    Ok(())
}

pub fn optionEqual<T1: Clone + 'static, T2: Clone + 'static>(mut inOption1: Option<T1>, mut inOption2: Option<T2>, mut inFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>) -> bool {
    pub type CompareFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut outEqual: bool = false;
    outEqual = (match (inOption1.clone(), inOption2.clone()) {
        (Some(mut val1), Some(mut val2)) => {
            inFunc(val1.clone(), val2.clone()).unwrap()
        },
        (None, None) => {
            true
        },
        _ => {
            false
        },
    });
    outEqual
}

pub fn makeValueOrDefault<TI: Clone + 'static, TO: Clone + 'static>(mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>, mut inArg: TI, mut inDefaultValue: TO) -> TO {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outValue: TO;
    match '__try0: {
        outValue = unwrap_break_err!(inFunc(inArg.clone()), '__try0);
        Ok::<_, anyhow::Error>((outValue.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outValue = __try0_o0;
        }
        Err(_) => {
            outValue = inDefaultValue.clone();
        }
    }
    outValue
}

pub fn xmlEscape(mut s1: ArcStr) -> Result<ArcStr> {
    let mut s2: ArcStr = arcstr::literal!("");
    s2 = (stringReplaceChar((s1.clone()).clone(), (literal!("&")).clone(), (literal!("&amp;")).clone())?).clone();
    s2 = (stringReplaceChar((s2.clone()).clone(), (literal!("<")).clone(), (literal!("&lt;")).clone())?).clone();
    s2 = (stringReplaceChar((s2.clone()).clone(), (literal!(">")).clone(), (literal!("&gt;")).clone())?).clone();
    s2 = (stringReplaceChar((s2.clone()).clone(), (literal!("\"")).clone(), (literal!("&quot;")).clone())?).clone();
    Ok(s2)
}

pub fn strcmpBool(mut s1: ArcStr, mut s2: ArcStr) -> bool {
    let mut b: bool = stringCompare((s1.clone()).clone(), (s2.clone()).clone()) > 0;
    b
}

pub fn strcmpNoCaseBool(mut s1: ArcStr, mut s2: ArcStr) -> bool {
    let mut b: bool = stringCompare((System::tolower((s1.clone()).clone())).clone(), (System::tolower((s2.clone()).clone())).clone()) > 0;
    b
}

pub fn stringAppendReverse(mut str1: ArcStr, mut str2: ArcStr) -> ArcStr {
    let mut r#str: ArcStr = stringAppend((str2.clone()).clone(), (str1.clone()).clone());
    r#str
}

pub fn stringAppendNonEmpty(mut inString1: ArcStr, mut inString2: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inString2.clone()) {
        Deref @ "" => inString2.clone(),
        _ => stringAppend((inString1.clone()).clone(), (inString2.clone()).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

pub fn getCurrentDateTime() -> DateTime {
    let mut dt: DateTime;
    let mut sec: i32 = 0;
    let mut min: i32 = 0;
    let mut hour: i32 = 0;
    let mut mday: i32 = 0;
    let mut mon: i32 = 0;
    let mut year: i32 = 0;
    (sec, min, hour, mday, mon, year) = System::getCurrentDateTime();
    dt = DateTime { sec: sec.clone(), min: min.clone(), hour: hour.clone(), mday: mday.clone(), mon: mon.clone(), year: year.clone() };
    dt
}

pub fn isSuccess(mut status: Status) -> Result<bool> {
    let mut bool: bool = false;
    bool = (match status.clone() {
        Status::SUCCESS { .. } => true,
        Status::FAILURE { .. } => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bool)
}

pub fn id<T: Clone + 'static>(mut inValue: T) -> T {
    let mut outValue: T = inValue.clone();
    outValue
}

pub fn buildMapStr(mut inLst1: Arc<metamodelica::List<ArcStr>>, mut inLst2: Arc<metamodelica::List<ArcStr>>, mut inMiddleDelimiter: ArcStr, mut inEndDelimiter: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ((::match_deref::match_deref! { match &((inLst1.clone(), inLst2.clone(), inMiddleDelimiter.clone(), inEndDelimiter.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            literal!("")
        },
        (Deref @ metamodelica::List::Cons { head: fa, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: fb, tail: Deref @ metamodelica::List::Nil }, md, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = stringAppendList(list![(fa.clone()).clone(), (md.clone()).clone(), (fb.clone()).clone()]);
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: fa, tail: ra }, Deref @ metamodelica::List::Cons { head: fb, tail: rb }, md, ed) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (buildMapStr(ra.clone(), rb.clone(), (md.clone()).clone(), (ed.clone()).clone())?).clone();
            r#str = stringAppendList(list![(fa.clone()).clone(), (md.clone()).clone(), (fb.clone()).clone(), (ed.clone()).clone(), (r#str.clone()).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outStr)
}

pub fn assoc<Key: Clone + 'static + PartialEq, Val: Clone + 'static>(mut inKey: Key, mut inList: Arc<metamodelica::List<(Key, Val)>>) -> Result<Val> {
    let mut outValue: Val;
    let mut k: Key;
    let mut v: Val;
    (k, v) = listHead(inList.clone())?;
    outValue = if (inKey.clone() == k.clone()) {v.clone()} else {assoc(inKey.clone(), listRest(inList.clone())?)?};
    Ok(outValue)
}

pub fn boolInt(mut inBoolean: bool) -> i32 {
    let mut outInteger: i32 = if (inBoolean.clone()) {1} else {0};
    outInteger
}

pub fn intBool(mut inInteger: i32) -> bool {
    let mut outBoolean: bool = inInteger.clone() > 0;
    outBoolean
}

pub fn stringBool(mut inString: ArcStr) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = stringBool2((System::tolower((inString.clone()).clone())).clone())?;
    Ok(outBoolean)
}

fn stringBool2(mut inString: ArcStr) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inString.clone()) {
        Deref @ "true" => true,
        Deref @ "false" => false,
        Deref @ "yes" => true,
        Deref @ "no" => false,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBoolean)
}

pub fn stringEqCaseInsensitive(mut str1: ArcStr, mut str2: ArcStr) -> bool {
    let mut eq: bool = false;
    eq = stringEq((System::tolower((str1.clone()).clone())).clone(), (System::tolower((str2.clone()).clone())).clone());
    eq
}

pub fn stringPadRight(mut inString: ArcStr, mut inPadWidth: i32, mut inPadString: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut pad_length: i32 = 0;
    let mut pad_str: ArcStr = arcstr::literal!("");
    pad_length = inPadWidth.clone() - ((inString.clone()).clone().len() as i32);
    if pad_length.clone() > 0 {
        pad_str = stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (1..=pad_length.clone()).into_iter() {
            let __x = inPadString.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inString.clone()); __mm_s.push_str(&*pad_str.clone()); ArcStr::from(__mm_s) }).clone();
    } else {
        outString = (inString.clone()).clone();
    }
    outString
}

pub fn stringPadLeft(mut inString: ArcStr, mut inPadWidth: i32, mut inPadString: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut pad_length: i32 = 0;
    let mut pad_str: ArcStr = arcstr::literal!("");
    pad_length = inPadWidth.clone() - ((inString.clone()).clone().len() as i32);
    if pad_length.clone() > 0 {
        pad_str = stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (1..=pad_length.clone()).into_iter() {
            let __x = inPadString.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*pad_str.clone()); __mm_s.push_str(&*inString.clone()); ArcStr::from(__mm_s) }).clone();
    } else {
        outString = (inString.clone()).clone();
    }
    outString
}

pub fn intProduct(mut lst: Arc<metamodelica::List<i32>>) -> i32 {
    let mut i: i32 = List::fold(lst.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1);
    i
}

pub fn nextPrime(mut inN: i32) -> i32 {
    let mut outNextPrime: i32 = 0;
    outNextPrime = if (inN.clone() <= 2) {2} else {nextPrime2(inN.clone() + intMod(inN.clone() + 1, 2))};
    outNextPrime
}

#[tailcall::tailcall]
fn nextPrime2(mut inN: i32) -> i32 {
    if (nextPrime_isPrime(inN.clone())) {inN.clone()} else {tailcall::call!{ nextPrime2(inN.clone() + 2) }}
}

fn nextPrime_isPrime(mut inN: i32) -> bool {
    let mut outIsPrime: bool = false;
    let mut i: i32 = 3;
    let mut q: i32 = intDiv(inN.clone(), 3);
    while q.clone() >= i.clone() {
        if inN.clone() == q.clone() * i.clone() {
            outIsPrime = false;
            return outIsPrime;
        }
        i = i.clone() + 2;
        q = intDiv(inN.clone(), i.clone());
    }
    outIsPrime = true;
    outIsPrime
}

pub fn anyToEmptyString<T: Clone + 'static>(mut a: T) -> ArcStr {
    let mut empty: ArcStr = literal!("");
    empty
}

pub fn removeLast3Char(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = substring((r#str.clone()).clone(), 1, ((r#str.clone()).clone().len() as i32) - 3)?;
    Ok(outStr)
}

pub fn removeLast4Char(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = substring((r#str.clone()).clone(), 1, ((r#str.clone()).clone().len() as i32) - 4)?;
    Ok(outStr)
}

pub fn removeLastNChar(mut r#str: ArcStr, mut n: i32) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = substring((r#str.clone()).clone(), 1, ((r#str.clone()).clone().len() as i32) - n.clone())?;
    Ok(outStr)
}

pub fn stringNotEqual(mut str1: ArcStr, mut str2: ArcStr) -> bool {
    let mut b: bool = !(stringEq((str1.clone()).clone(), (str2.clone()).clone()));
    b
}

pub fn swap<T: Clone + 'static>(mut cond: bool, mut in1: T, mut in2: T) -> (T, T) {
    let mut out1: T;
    let mut out2: T;
    (out1, out2) = (match cond.clone() {
        true => (in2.clone(), in1.clone()),
        _ => (in1.clone(), in2.clone()),
    });
    (out1, out2)
}

pub fn replace<T: Clone + 'static>(mut replaced: T, mut arg: T) -> T {
    let mut outArg: T = arg.clone();
    outArg
}

pub fn realRangeSize(mut inStart: metamodelica::Real, mut inStep: metamodelica::Real, mut inStop: metamodelica::Real) -> i32 {
    let mut outSize: i32 = 0;
    outSize = ((((inStop.clone() - inStart.clone()) / inStep.clone() + metamodelica::OrderedFloat(5e-15_f64)).floor()).0.floor() as i32) + 1;
    outSize = std::cmp::max(outSize.clone(), 0);
    outSize
}

fn createDirectoryTreeH(mut inString: ArcStr, mut parentDir: ArcStr, mut parentDirExists: bool) -> Result<bool> {
    let mut outBool: bool = false;
    outBool = 'mc: {
        let __mc_input = parentDirExists.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool = false;
            let true = (stringEqual((parentDir.clone()).clone(), (System::dirname((parentDir.clone()).clone())).clone())) else { bail!("pattern mismatch") };
            b = System::createDirectory((inString.clone()).clone());
            Ok(b.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let true = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool = false;
            b = System::createDirectory((inString.clone()).clone());
            Ok(b.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let false = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool = false;
            let true = (createDirectoryTree((parentDir.clone()).clone())?) else { bail!("pattern mismatch") };
            b = System::createDirectory((inString.clone()).clone());
            Ok(b.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBool)
}

pub fn createDirectoryTree(mut inString: ArcStr) -> Result<bool> {
    let mut outBool: bool = false;
    let mut parentDir: ArcStr = arcstr::literal!("");
    let mut parentDirExists: bool = false;
    if System::directoryExists((inString.clone()).clone()) {
        outBool = true;
    } else {
        parentDir = (System::dirname((inString.clone()).clone())).clone();
        parentDirExists = System::directoryExists((parentDir.clone()).clone());
        outBool = createDirectoryTreeH((inString.clone()).clone(), (parentDir.clone()).clone(), parentDirExists.clone())?;
    }
    Ok(outBool)
}

pub fn nextPowerOf2(mut i: i32) -> i32 {
    let mut v: i32 = 0;
    v = i.clone() - 1;
    v = intBitOr(v.clone(), intBitRShift(v.clone(), 1));
    v = intBitOr(v.clone(), intBitRShift(v.clone(), 2));
    v = intBitOr(v.clone(), intBitRShift(v.clone(), 4));
    v = intBitOr(v.clone(), intBitRShift(v.clone(), 8));
    v = intBitOr(v.clone(), intBitRShift(v.clone(), 16));
    v = v.clone() + 1;
    v
}

pub fn isCIdentifier(mut r#str: ArcStr) -> bool {
    let mut b: bool = false;
    let mut i: i32 = 0;
    (i, _) = System::regex((r#str.clone()).clone(), (literal!("^[_A-Za-z][_A-Za-z0-9]*$")).clone(), 0, true, false);
    b = i.clone() == 1;
    b
}

pub fn isIntegerString(mut r#str: ArcStr) -> bool {
    let mut b: bool = false;
    let mut i: i32 = 0;
    (i, _) = System::regex((r#str.clone()).clone(), (literal!("^[0-9][0-9]*$")).clone(), 0, true, false);
    b = i.clone() == 1;
    b
}

pub fn stringTrunc(mut r#str: ArcStr, mut len: i32) -> Result<ArcStr> {
    let mut truncatedStr: ArcStr = arcstr::literal!("");
    truncatedStr = (if (((r#str.clone()).clone().len() as i32) <= len.clone()) {r#str.clone()} else {substring((r#str.clone()).clone(), 0, len.clone())?}).clone();
    Ok(truncatedStr)
}

pub fn getTempVariableIndex() -> ArcStr {
    let mut name: ArcStr = arcstr::literal!("");
    name = (stringAppend((literal!("$tmpVar")).clone(), (intString(System::tmpTickIndex(Global::tmpVariableIndex.clone()))).clone())).clone();
    name
}

pub fn anyReturnTrue<T: Clone + 'static>(mut a: T) -> bool {
    let mut b: bool = true;
    b
}

pub fn absoluteOrRelative(mut inFileName: ArcStr) -> ArcStr {
    let mut outFileName: ArcStr = inFileName.clone();
    let mut pwd: ArcStr = arcstr::literal!("");
    let mut pd: ArcStr = arcstr::literal!("");
    let mut f: ArcStr = arcstr::literal!("");
    pwd = (System::pwd()).clone();
    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
    if !(System::regularFileExists((inFileName.clone()).clone())) {
        f = stringAppendList(list![(pwd.clone()).clone(), (pd.clone()).clone(), (inFileName.clone()).clone()]);
        outFileName = (if (System::regularFileExists((f.clone()).clone())) {f.clone()} else {outFileName.clone()}).clone();
    }
    outFileName
}

pub fn hashFileNamePrefix(mut inFileNamePrefix: ArcStr) -> ArcStr {
    let mut hashStr: ArcStr = substring((intString(stringHashDjb2((inFileNamePrefix.clone()).clone()))).clone(), 1, 3).unwrap();
    hashStr
}

pub fn intLstString(mut lst: Arc<metamodelica::List<i32>>) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = stringDelimitList(List::map(lst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone());
    s
}

pub fn sourceInfoIsEmpty(mut inInfo: SourceInfo) -> bool {
    let mut outIsEmpty: bool = false;
    outIsEmpty = (::match_deref::match_deref! { match &(inInfo.clone()) {
        SourceInfo { fileName: Deref @ "", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub fn sourceInfoIsEqual(mut inInfo1: SourceInfo, mut inInfo2: SourceInfo) -> bool {
    let mut outIsEqual: bool = false;
    outIsEqual = (match (inInfo1.clone(), inInfo2.clone()) {
        (SourceInfo { .. }, SourceInfo { .. }) => inInfo1.fileName.clone() == inInfo2.fileName.clone() && inInfo1.isReadOnly.clone() == inInfo2.isReadOnly.clone() && inInfo1.lineNumberStart.clone() == inInfo2.lineNumberStart.clone() && inInfo1.columnNumberStart.clone() == inInfo2.columnNumberStart.clone() && inInfo1.lineNumberEnd.clone() == inInfo2.lineNumberEnd.clone() && inInfo1.columnNumberEnd.clone() == inInfo2.columnNumberEnd.clone(),
        _ => false,
    });
    outIsEqual
}

/* ************************************************
 * profiler stuff
 ************************************************/
pub fn profilerinit() -> Result<()> {
    crate::Globals::profilerTime1Index.with(|__root| *__root.borrow_mut() = metamodelica::OrderedFloat(0.0_f64));
    crate::Globals::profilerTime2Index.with(|__root| *__root.borrow_mut() = metamodelica::OrderedFloat(0.0_f64));
    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
    Ok(())
}

pub fn profilerresults() -> Result<()> {
    let mut tg: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut t1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut t2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    tg = System::realtimeTock(ClockIndexes::RT_PROFILER0.clone())?;
    t1 = profilertime1();
    t2 = profilertime2();
    println!("{}", (literal!("Time all: ")).clone());
    println!("{}", (realString(tg.clone())).clone());
    println!("{}", (literal!("\n")).clone());
    println!("{}", (literal!("Time t1: ")).clone());
    println!("{}", (realString(t1.clone())).clone());
    println!("{}", (literal!("\n")).clone());
    println!("{}", (literal!("Time t2: ")).clone());
    println!("{}", (realString(t2.clone())).clone());
    println!("{}", (literal!("\n")).clone());
    println!("{}", (literal!("Time all-t1-t2: ")).clone());
    println!("{}", (realString(((tg.clone()) - (t1.clone())) - (t2.clone()))).clone());
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn profilertime1() -> metamodelica::Real {
    let mut t1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    t1 = crate::Globals::profilerTime1Index.with(|__root| __root.borrow().clone());
    t1
}

pub fn profilertime2() -> metamodelica::Real {
    let mut t2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    t2 = crate::Globals::profilerTime2Index.with(|__root| __root.borrow().clone());
    t2
}

pub fn profilerstart1() -> Result<()> {
    System::realtimeTick(ClockIndexes::RT_PROFILER1.clone())?;
    Ok(())
}

pub fn profilerstart2() -> Result<()> {
    System::realtimeTick(ClockIndexes::RT_PROFILER2.clone())?;
    Ok(())
}

pub fn profilerstop1() -> Result<()> {
    let mut t: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    t = System::realtimeTock(ClockIndexes::RT_PROFILER1.clone())?;
    crate::Globals::profilerTime1Index.with(|__root| *__root.borrow_mut() = (crate::Globals::profilerTime1Index.with(|__root| __root.borrow().clone())) + (t.clone()));
    Ok(())
}

pub fn profilerstop2() -> Result<()> {
    let mut t: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    t = System::realtimeTock(ClockIndexes::RT_PROFILER2.clone())?;
    crate::Globals::profilerTime2Index.with(|__root| *__root.borrow_mut() = (crate::Globals::profilerTime2Index.with(|__root| __root.borrow().clone())) + (t.clone()));
    Ok(())
}

pub fn profilerreset1() -> () {
    crate::Globals::profilerTime1Index.with(|__root| *__root.borrow_mut() = metamodelica::OrderedFloat(0.0_f64));
    ()
}

pub fn profilerreset2() -> () {
    crate::Globals::profilerTime2Index.with(|__root| *__root.borrow_mut() = metamodelica::OrderedFloat(0.0_f64));
    ()
}

pub fn profilertock1() -> Result<metamodelica::Real> {
    let mut t: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    t = System::realtimeTock(ClockIndexes::RT_PROFILER1.clone())?;
    Ok(t)
}

pub fn profilertock2() -> Result<metamodelica::Real> {
    let mut t: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    t = System::realtimeTock(ClockIndexes::RT_PROFILER2.clone())?;
    Ok(t)
}

pub fn applyTuple21<T1: Clone + 'static + PartialEq, T2: Clone + 'static>(mut inTuple: (T1, T2), mut func: Arc<dyn ::std::ops::Fn(T1) -> Result<T1> + 'static>) -> (T1, T2) {
    pub type FuncT<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<T1> + 'static>;

    let mut outTuple: (T1, T2);
    let mut e1_1: T1;
    let mut e1_2: T1;
    let mut e2: T2;
    (e1_1, e2) = inTuple.clone();
    e1_2 = func(e1_1.clone()).unwrap();
    outTuple = if (referenceEq(&e1_1.clone(),&e1_2.clone())) {inTuple.clone()} else {(e1_2.clone(), e2.clone())};
    outTuple
}

pub fn applyTuple22<T1: Clone + 'static, T2: Clone + 'static + PartialEq>(mut inTuple: (T1, T2), mut func: Arc<dyn ::std::ops::Fn(T2) -> Result<T2> + 'static>) -> (T1, T2) {
    pub type FuncT<T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T2) -> Result<T2> + 'static>;

    let mut outTuple: (T1, T2);
    let mut e1: T1;
    let mut e2_1: T2;
    let mut e2_2: T2;
    (e1, e2_1) = inTuple.clone();
    e2_2 = func(e2_1.clone()).unwrap();
    outTuple = if (referenceEq(&e2_1.clone(),&e2_2.clone())) {inTuple.clone()} else {(e1.clone(), e2_2.clone())};
    outTuple
}

pub fn applyTuple31<T1: Clone + 'static + PartialEq, T2: Clone + 'static, T3: Clone + 'static>(mut inTuple: (T1, T2, T3), mut func: Arc<dyn ::std::ops::Fn(T1) -> Result<T1> + 'static>) -> (T1, T2, T3) {
    pub type FuncT<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<T1> + 'static>;

    let mut outTuple: (T1, T2, T3);
    let mut t1: T1;
    let mut t1_new: T1;
    let mut t2: T2;
    let mut t3: T3;
    (t1, t2, t3) = inTuple.clone();
    t1_new = func(t1.clone()).unwrap();
    outTuple = if (referenceEq(&t1.clone(),&t1_new.clone())) {inTuple.clone()} else {(t1_new.clone(), t2.clone(), t3.clone())};
    outTuple
}

pub fn referenceCompare<T1: Clone + 'static, T2: Clone + 'static>(mut ref1: T1, mut ref2: T2) -> i32 {
    let mut result: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("referenceCompareExt"), lang: Some("C"), output_: Some(CREF_IDENT { name: "result", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "ref1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "ref2", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\n  static inline int referenceCompareExt(void *ref1, void *ref2)\n  {\n    return (ref1 < ref2) ? -1 : (ref1 > ref2);\n  }\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Util.mo", isReadOnly: false, lineNumberStart: 1650, columnNumberStart: 73, lineNumberEnd: 1655, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Util.mo", isReadOnly: false, lineNumberStart: 1650, columnNumberStart: 66, lineNumberEnd: 1655, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    result
}

#[tailcall::tailcall]
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    if (b.clone() == 0) {a.clone()} else {tailcall::call!{ gcd(b.clone(), intMod(a.clone(), b.clone())) }}
}

pub fn lcm(mut a: i32, mut b: i32) -> i32 {
    let mut res: i32 = 0;
    res = if (a.clone() < 0 || b.clone() < 0) {-1} else {intDiv(a.clone() * b.clone(), gcd(a.clone(), b.clone()))};
    res
}

pub fn msb(mut n: i32) -> i32 {
    let mut res: i32 = 0;
    let mut i: i32 = n.clone();
    while i.clone() > 0 {
        i = intBitRShift(i.clone(), 1);
        res = res.clone() + 1;
    }
    res
}

pub fn foldcallN<FT: Clone + 'static>(mut n: i32, mut inFoldFunc: Arc<dyn ::std::ops::Fn(FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
    pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut i in 1..=n.clone() {
        outResult = inFoldFunc(outResult.clone()).unwrap();
    }
    outResult
}

