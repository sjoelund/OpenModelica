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

use crate::System;

pub const NO_POS: i32 = 0;

pub const CHAR_NEWLINE: i32 = 10;

pub const CHAR_SPACE: i32 = 32;

pub const CHAR_DASH: i32 = 45;

pub const CHAR_DOT: i32 = 46;

pub fn headline_1(mut title: ArcStr) -> ArcStr {
    let mut header: ArcStr = arcstr::literal!("");
    header = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*repeat((literal!("#")).clone(), ((title.clone()).clone().len() as i32) + 8)); __mm_s.push_str(&*literal!("\n\n    ")); __mm_s.push_str(&*title.clone()); __mm_s.push_str(&*literal!("\n\n")); __mm_s.push_str(&*repeat((literal!("#")).clone(), ((title.clone()).clone().len() as i32) + 8)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    header
}

pub fn headline_2(mut title: ArcStr) -> ArcStr {
    let mut header: ArcStr = arcstr::literal!("");
    header = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*repeat((literal!("=")).clone(), ((title.clone()).clone().len() as i32) + 4)); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*title.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*repeat((literal!("=")).clone(), ((title.clone()).clone().len() as i32) + 4)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    header
}

pub fn headline_3(mut title: ArcStr) -> ArcStr {
    let mut header: ArcStr = arcstr::literal!("");
    header = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*title.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*repeat((literal!("-")).clone(), ((title.clone()).clone().len() as i32) + 2)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    header
}

pub fn headline_4(mut title: ArcStr) -> ArcStr {
    let mut header: ArcStr = arcstr::literal!("");
    header = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*title.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*repeat((literal!("*")).clone(), ((title.clone()).clone().len() as i32) + 2)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    header
}

pub fn findChar(mut inString: ArcStr, mut inChar: i32, mut inStartPos: i32, mut inEndPos: i32) -> i32 {
    let mut outIndex: i32 = NO_POS.clone();
    let len: i32 = ((inString.clone()).clone().len() as i32);
    let mut start_pos: i32 = 0;
    let mut end_pos: i32 = 0;
    start_pos = std::cmp::max(inStartPos.clone(), 1);
    end_pos = if (inEndPos.clone() > 0) {std::cmp::min(inEndPos.clone(), len.clone())} else {len.clone()};
    for mut i in start_pos.clone()..=end_pos.clone() {
        if stringGet((inString.clone()).clone(),i.clone()).unwrap() == inChar.clone() {
            outIndex = i.clone();
            break;
        }
    }
    outIndex
}

pub fn rfindChar(mut inString: ArcStr, mut inChar: i32, mut inStartPos: i32, mut inEndPos: i32) -> i32 {
    let mut outIndex: i32 = NO_POS.clone();
    let len: i32 = ((inString.clone()).clone().len() as i32);
    let mut start_pos: i32 = 0;
    let mut end_pos: i32 = 0;
    start_pos = if (inStartPos.clone() > 0) {std::cmp::min(inStartPos.clone(), len.clone())} else {len.clone()};
    end_pos = std::cmp::max(inEndPos.clone(), 1);
    for mut i in (end_pos.clone()..=start_pos.clone()).rev() {
        if stringGet((inString.clone()).clone(),i.clone()).unwrap() == inChar.clone() {
            outIndex = i.clone();
            break;
        }
    }
    outIndex
}

pub fn findCharNot(mut inString: ArcStr, mut inChar: i32, mut inStartPos: i32, mut inEndPos: i32) -> i32 {
    let mut outIndex: i32 = NO_POS.clone();
    let len: i32 = ((inString.clone()).clone().len() as i32);
    let mut start_pos: i32 = 0;
    let mut end_pos: i32 = 0;
    start_pos = std::cmp::max(inStartPos.clone(), 1);
    end_pos = if (inEndPos.clone() > 0) {std::cmp::min(inEndPos.clone(), len.clone())} else {len.clone()};
    for mut i in start_pos.clone()..=end_pos.clone() {
        if stringGet((inString.clone()).clone(),i.clone()).unwrap() != inChar.clone() {
            outIndex = i.clone();
            break;
        }
    }
    outIndex
}

pub fn rfindCharNot(mut inString: ArcStr, mut inChar: i32, mut inStartPos: i32, mut inEndPos: i32) -> i32 {
    let mut outIndex: i32 = NO_POS.clone();
    let len: i32 = ((inString.clone()).clone().len() as i32);
    let mut start_pos: i32 = 0;
    let mut end_pos: i32 = 0;
    start_pos = if (inStartPos.clone() > 0) {std::cmp::min(inStartPos.clone(), len.clone())} else {len.clone()};
    end_pos = std::cmp::max(inEndPos.clone(), 1);
    for mut i in (end_pos.clone()..=start_pos.clone()).rev() {
        if stringGet((inString.clone()).clone(),i.clone()).unwrap() != inChar.clone() {
            outIndex = i.clone();
            break;
        }
    }
    outIndex
}

pub fn isAlpha(mut inChar: i32) -> bool {
    let mut outIsAlpha: bool = inChar.clone() >= 65 && inChar.clone() <= 90 || inChar.clone() >= 97 && inChar.clone() <= 122;
    outIsAlpha
}

pub fn wordWrap(mut inString: ArcStr, mut inWrapLength: i32, mut inDelimiter: ArcStr, mut inRaggedness: metamodelica::Real) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut start_pos: i32 = 1;
    let mut end_pos: i32 = inWrapLength.clone();
    let mut line_len: i32 = 0;
    let mut pos: i32 = 0;
    let mut next_char: i32 = 0;
    let mut char: i32 = 0;
    let mut gap_size: i32 = 0;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut delim: ArcStr = literal!("");
    let mut lines: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    if ((inDelimiter.clone()).clone().len() as i32) >= inWrapLength.clone() - 1 {
        outStrings = list![(inString.clone()).clone()];
        return Ok(outStrings.clone());
    }
    lines = System::strtok((inString.clone()).clone(), (literal!("\n")).clone());
    line_len = inWrapLength.clone() - ((inDelimiter.clone()).clone().len() as i32) - 1;
    gap_size = std::cmp::max((((metamodelica::OrderedFloat((line_len.clone()) as f64)) * (inRaggedness.clone())).0 as i32), 0);
    for mut line in &*lines.clone() {
        let mut line = line.clone();
        while end_pos.clone() < ((line.clone()).clone().len() as i32) {
            next_char = stringGet((line.clone()).clone(),end_pos.clone() + 1)?;
            if next_char.clone() != CHAR_SPACE.clone() && next_char.clone() != CHAR_DASH.clone() {
                pos = rfindChar((line.clone()).clone(), CHAR_SPACE.clone(), end_pos.clone(), end_pos.clone() - gap_size.clone());
                if pos.clone() != NO_POS.clone() {
                    r#str = substring((line.clone()).clone(), start_pos.clone(), pos.clone() - 1)?;
                    start_pos = pos.clone() + 1;
                } else {
                    pos = rfindChar((line.clone()).clone(), CHAR_DASH.clone(), end_pos.clone(), start_pos.clone() + gap_size.clone());
                    if pos.clone() > 1 {
                        char = stringGet((line.clone()).clone(),pos.clone() - 1)?;
                        pos = if (isAlpha(char.clone()) && isAlpha(next_char.clone())) {pos.clone()} else {NO_POS.clone()};
                    }
                    if pos.clone() != NO_POS.clone() {
                        r#str = substring((line.clone()).clone(), start_pos.clone(), pos.clone())?;
                        start_pos = pos.clone() + 1;
                    } else {
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*substring((line.clone()).clone(), start_pos.clone(), end_pos.clone() - 1)?); __mm_s.push_str(&*literal!("-")); ArcStr::from(__mm_s) }).clone();
                        start_pos = end_pos.clone();
                    }
                }
            } else {
                r#str = substring((line.clone()).clone(), start_pos.clone(), end_pos.clone())?;
                start_pos = end_pos.clone() + if (next_char.clone() == CHAR_SPACE.clone()) {2} else {1};
            }
            outStrings = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*delim.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone(), outStrings.clone());
            end_pos = start_pos.clone() + line_len.clone();
            delim = (inDelimiter.clone()).clone();
        }
        if start_pos.clone() < ((line.clone()).clone().len() as i32) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*delim.clone()); __mm_s.push_str(&*substring((line.clone()).clone(), start_pos.clone(), ((line.clone()).clone().len() as i32))?); ArcStr::from(__mm_s) }).clone();
            outStrings = cons((r#str.clone()).clone(), outStrings.clone());
        }
        start_pos = 1;
        end_pos = line_len.clone();
        delim = (inDelimiter.clone()).clone();
    }
    outStrings = outStrings.clone().reverse();
    Ok(outStrings)
}

pub fn repeat(mut r#str: ArcStr, mut n: i32) -> ArcStr {
    let mut res: ArcStr = literal!("");
    let mut len: i32 = ((r#str.clone()).clone().len() as i32);
    let mut ext: System::StringAllocator = System::StringAllocator(len.clone() * n.clone()).unwrap();
    for mut i in 0..=n.clone() - 1 {
        System::stringAllocatorStringCopy(ext.clone(), (r#str.clone()).clone(), len.clone() * i.clone());
    }
    res = (System::stringAllocatorResult(ext.clone(), (res.clone()).clone())).clone();
    res
}

pub fn quote(mut inString: ArcStr) -> ArcStr {
    let mut outString: ArcStr = stringAppendList(list![(literal!("\"")).clone(), (inString.clone()).clone(), (literal!("\"")).clone()]);
    outString
}

pub fn equalIgnoreSpace(mut s1: ArcStr, mut s2: ArcStr) -> Result<bool> {
    let mut b: bool = false;
    let mut j: i32 = 1;
    b = true;
    for mut i in 1..=((s1.clone()).clone().len() as i32) {
        if metamodelica::Dangerous::stringGet((s1.clone()).clone(), i.clone())? != stringCharInt((literal!(" ")).clone())? {
            b = false;
            for mut j2 in j.clone()..=((s2.clone()).clone().len() as i32) {
                if metamodelica::Dangerous::stringGet((s2.clone()).clone(), j2.clone())? != stringCharInt((literal!(" ")).clone())? {
                    if metamodelica::Dangerous::stringGet((s2.clone()).clone(), j2.clone())? != metamodelica::Dangerous::stringGet((s1.clone()).clone(), i.clone())? {
                        return Ok(b.clone());
                    }
                    j = j2.clone() + 1;
                    b = true;
                    break;
                }
            }
            if !(b.clone()) {
                return Ok(b.clone());
            }
        }
    }
    for mut j2 in j.clone()..=((s2.clone()).clone().len() as i32) {
        if metamodelica::Dangerous::stringGet((s2.clone()).clone(), j2.clone())? != stringCharInt((literal!(" ")).clone())? {
            b = false;
            return Ok(b.clone());
        }
    }
    Ok(b)
}

pub fn bytesToReadableUnit(mut bytes: metamodelica::Real, mut significantDigits: i32, mut maxSizeInUnit: metamodelica::Real) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let TB: metamodelica::Real = (metamodelica::OrderedFloat((1024) as f64)).powf(metamodelica::OrderedFloat((4) as f64));
    let GB: metamodelica::Real = (metamodelica::OrderedFloat((1024) as f64)).powf(metamodelica::OrderedFloat((3) as f64));
    let MB: metamodelica::Real = (metamodelica::OrderedFloat((1024) as f64)).powf(metamodelica::OrderedFloat((2) as f64));
    let kB: metamodelica::Real = metamodelica::OrderedFloat((1024) as f64);
    if bytes.clone() > maxSizeInUnit.clone() * GB.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*{ let __mm_unimpl: ArcStr = todo!("String() builtin with named args [significantDigits] not yet lowered"); __mm_unimpl }); __mm_s.push_str(&*literal!(" TB")); ArcStr::from(__mm_s) }).clone();
    } else if bytes.clone() > maxSizeInUnit.clone() * MB.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*{ let __mm_unimpl: ArcStr = todo!("String() builtin with named args [significantDigits] not yet lowered"); __mm_unimpl }); __mm_s.push_str(&*literal!(" GB")); ArcStr::from(__mm_s) }).clone();
    } else if bytes.clone() > maxSizeInUnit.clone() * kB.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*{ let __mm_unimpl: ArcStr = todo!("String() builtin with named args [significantDigits] not yet lowered"); __mm_unimpl }); __mm_s.push_str(&*literal!(" MB")); ArcStr::from(__mm_s) }).clone();
    } else if bytes.clone() > maxSizeInUnit.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*{ let __mm_unimpl: ArcStr = todo!("String() builtin with named args [significantDigits] not yet lowered"); __mm_unimpl }); __mm_s.push_str(&*literal!(" kB")); ArcStr::from(__mm_s) }).clone();
    } else {
        r#str = ArcStr::from(::std::format!("{}", ((bytes.clone()).0.floor() as i32)));
    }
    r#str
}

pub fn startsWith(mut r#str: ArcStr, mut prefix: ArcStr) -> bool {
    let mut startsWith: bool = 0 == System::strncmp((r#str.clone()).clone(), (prefix.clone()).clone(), ((prefix.clone()).clone().len() as i32));
    startsWith
}

pub fn endsWith(mut r#str: ArcStr, mut suffix: ArcStr) -> bool {
    let mut endsWith: bool = false;
    let mut str_len: i32 = ((r#str.clone()).clone().len() as i32);
    let mut suf_len: i32 = ((suffix.clone()).clone().len() as i32);
    if str_len.clone() >= suf_len.clone() {
        endsWith = 0 == System::strcmp_offset((r#str.clone()).clone(), str_len.clone() - suf_len.clone() + 1, str_len.clone(), (suffix.clone()).clone(), 1, suf_len.clone());
    }
    endsWith
}

pub fn endsWithNewline(mut r#str: ArcStr) -> bool {
    let mut b: bool = false;
    b = CHAR_NEWLINE.clone() == metamodelica::Dangerous::stringGet((r#str.clone()).clone(), ((r#str.clone()).clone().len() as i32)).unwrap();
    b
}

pub fn convertCharNonAsciiToHex(mut s: ArcStr) -> Result<ArcStr> {
    let mut s: ArcStr = s;
    let mut i: i32 = 0;
    let hex: metamodelica::Array<ArcStr> = metamodelica::Dangerous::listArray(list![(literal!("0")).clone(), (literal!("1")).clone(), (literal!("2")).clone(), (literal!("3")).clone(), (literal!("4")).clone(), (literal!("5")).clone(), (literal!("6")).clone(), (literal!("7")).clone(), (literal!("8")).clone(), (literal!("9")).clone(), (literal!("A")).clone(), (literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("E")).clone(), (literal!("F")).clone()]);
    i = stringCharInt((s.clone()).clone())?;
    if i.clone() < 128 {
        return Ok(s.clone());
    }
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("0x")); __mm_s.push_str(&*hex.clone().borrow()[(intDiv(i.clone(), 16) + 1-1) as usize].clone()); __mm_s.push_str(&*hex.clone().borrow()[(intMod(i.clone(), 16) + 1-1) as usize].clone()); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

pub fn stripBOM(mut s: ArcStr) -> Result<(ArcStr, ArcStr)> {
    let mut s: ArcStr = s;
    let mut bom: ArcStr = literal!("");
    if ((s.clone()).clone().len() as i32) < 3 {
        return Ok((s.clone(), bom.clone()));
    }
    if stringGet((s.clone()).clone(),1)? == 239 && stringGet((s.clone()).clone(),2)? == 187 && stringGet((s.clone()).clone(),3)? == 191 {
        bom = substring((s.clone()).clone(), 1, 3)?;
        s = substring((s.clone()).clone(), 4, ((s.clone()).clone().len() as i32))?;
    }
    Ok((s, bom))
}

pub fn stripFileExtension(mut filename: ArcStr) -> Result<ArcStr> {
    let mut filename: ArcStr = filename;
    let mut pos: i32 = 0;
    pos = rfindChar((filename.clone()).clone(), CHAR_DOT.clone(), 0, 1);
    if pos.clone() != NO_POS.clone() {
        filename = substring((filename.clone()).clone(), 1, pos.clone() - 1)?;
    }
    Ok(filename)
}

pub fn rest(mut r#str: ArcStr) -> ArcStr {
    let mut rest: ArcStr = substring((r#str.clone()).clone(), 2, ((r#str.clone()).clone().len() as i32)).unwrap();
    rest
}

