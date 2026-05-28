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

use crate::Error;
use crate::LexerJSON::Token;
use crate::LexerJSON::TokenId;
use crate::LexerJSON::printToken;
use crate::LexerJSON::tokenContent;
use crate::LexerJSON::tokenSourceInfo;
use crate::LexerJSON;
use crate::Print;
use crate::System;
use crate::UnorderedMap;
use crate::Util;
use crate::Vector;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JSON {
    OBJECT {
        values: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<JSON>>>,
    },
    LIST_OBJECT {
        values: Arc<metamodelica::List<(ArcStr, Arc<JSON>)>>,
    },
    ARRAY {
        values: Arc<Vector::Vector<Arc<JSON>>>,
    },
    LIST {
        values: Arc<metamodelica::List<Arc<JSON>>>,
    },
    STRING {
        r#str: ArcStr,
    },
    INTEGER {
        i: i32,
    },
    NUMBER {
        r: metamodelica::Real,
    },
    TRUE,
    FALSE,
    NULL,
}
impl Default for JSON {
    fn default() -> Self { Self::TRUE }
}
pub use self::JSON::{OBJECT,LIST_OBJECT,ARRAY,LIST,STRING,INTEGER,NUMBER,TRUE,FALSE,NULL};
pub fn emptyObject() -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::FALSE);
    obj = Arc::new(JSON::OBJECT { values: UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1) });
    obj
}

pub fn emptyListObject() -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::LIST_OBJECT { values: metamodelica::nil() });
    obj
}

pub fn fromPair(mut key: ArcStr, mut value: Arc<JSON>) -> Result<Arc<JSON>> {
    let mut obj: Arc<JSON> = Arc::new(JSON::FALSE);
    obj = emptyObject();
    obj = addPair((key.clone()).clone(), value.clone(), obj.clone())?;
    Ok(obj)
}

pub fn listObjectFromPair(mut key: ArcStr, mut value: Arc<JSON>) -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::LIST_OBJECT { values: list![(key.clone(), value.clone())] });
    obj
}

pub fn emptyArray(mut capacity: i32) -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::ARRAY { values: Vector::new(capacity.clone()) });
    obj
}

pub fn makeArray(mut elements: Arc<metamodelica::List<Arc<JSON>>>) -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::ARRAY { values: Vector::fromList(elements.clone()) });
    obj
}

pub fn makeList(mut elements: Arc<metamodelica::List<Arc<JSON>>>) -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::LIST { values: elements.clone() });
    obj
}

pub fn makeString(mut r#str: ArcStr) -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::STRING { r#str: (r#str.clone()).clone() });
    obj
}

pub fn makeInteger(mut i: i32) -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::INTEGER { i: i.clone() });
    obj
}

pub fn makeNumber(mut r: metamodelica::Real) -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(JSON::NUMBER { r: r.clone() });
    obj
}

pub fn makeBoolean(mut b: bool) -> Arc<JSON> {
    let mut obj: Arc<JSON> = if (b.clone()) {Arc::new(crate::JSON::TRUE)} else {Arc::new(crate::JSON::FALSE)};
    obj
}

pub fn makeNull() -> Arc<JSON> {
    let mut obj: Arc<JSON> = Arc::new(crate::JSON::NULL);
    obj
}

pub fn isNull(mut obj: Arc<JSON>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ NULL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn addElement(mut value: Arc<JSON>, mut obj: Arc<JSON>) -> Result<Arc<JSON>> {
    let mut outObj: Arc<JSON> = Arc::new(JSON::FALSE);
    outObj = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ ARRAY { .. } => {
            Vector::push(var_field!((*obj).values, JSON::ARRAY).clone(), value.clone());
            obj.clone()
        },
        Deref @ NULL { .. } => addElement(value.clone(), emptyArray(0))?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outObj)
}

pub fn addElementNotNull(mut value: Arc<JSON>, mut obj: Arc<JSON>) -> Result<Arc<JSON>> {
    let mut outObj: Arc<JSON> = Arc::new(JSON::FALSE);
    outObj = if (isNull(value.clone())) {obj.clone()} else {addElement(value.clone(), obj.clone())?};
    Ok(outObj)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn addPair(mut key: ArcStr, mut value: Arc<JSON>, mut obj: Arc<JSON>) -> Result<Arc<JSON>> {
    let mut outObj: Arc<JSON> = Arc::new(JSON::FALSE);
    outObj = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ OBJECT { .. } => {
            UnorderedMap::add((key.clone()).clone(), value.clone(), var_field!((*obj).values, JSON::OBJECT).clone())?;
            obj.clone()
        },
        Deref @ LIST_OBJECT { .. } => Arc::new(JSON::LIST_OBJECT { values: cons((key.clone(), value.clone()), var_field!((*obj).values, JSON::LIST_OBJECT).clone()) }),
        Deref @ NULL { .. } => addPair((key.clone()).clone(), value.clone(), emptyListObject())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outObj)
}

pub fn addPairNotNull(mut key: ArcStr, mut value: Arc<JSON>, mut obj: Arc<JSON>) -> Result<Arc<JSON>> {
    let mut outObj: Arc<JSON> = Arc::new(JSON::FALSE);
    outObj = if (isNull(value.clone())) {obj.clone()} else {addPair((key.clone()).clone(), value.clone(), obj.clone())?};
    Ok(outObj)
}

pub fn toString(mut value: Arc<JSON>, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut handle: i32 = 0;
    handle = Print::saveAndClearBuf()?;
    if prettyPrint.clone() {
        toStringPP_work(value.clone(), (literal!("")).clone())?;
    } else {
        toString_work(value.clone())?;
    }
    r#str = (Print::getString()?).clone();
    Print::restoreBuf(handle.clone())?;
    Ok(r#str)
}

pub fn toString_work(mut value: Arc<JSON>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(value.clone()) {
        Deref @ STRING { .. } => {
            Print::printBuf((literal!("\"")).clone())?;
            Print::printBuf((System::escapedString((var_field!((*value).r#str, JSON::STRING).clone()).clone(), true)).clone())?;
            Print::printBuf((literal!("\"")).clone())?;
            ()
        },
        Deref @ TRUE { .. } => {
            Print::printBuf((literal!("true")).clone())?;
            ()
        },
        Deref @ FALSE { .. } => {
            Print::printBuf((literal!("false")).clone())?;
            ()
        },
        Deref @ NULL { .. } => {
            Print::printBuf((literal!("null")).clone())?;
            ()
        },
        Deref @ INTEGER { .. } => {
            Print::printBuf(ArcStr::from(::std::format!("{}", var_field!((*value).i, JSON::INTEGER).clone())))?;
            ()
        },
        Deref @ NUMBER { .. } => {
            Print::printBuf(ArcStr::from(::std::format!("{}", var_field!((*value).r, JSON::NUMBER).clone())))?;
            ()
        },
        Deref @ ARRAY { .. } => {
            toString_array(var_field!((*value).values, JSON::ARRAY).clone())?;
            ()
        },
        Deref @ LIST { .. } => {
            toString_list(var_field!((*value).values, JSON::LIST).clone())?;
            ()
        },
        Deref @ OBJECT { .. } => {
            toString_object(var_field!((*value).values, JSON::OBJECT).clone())?;
            ()
        },
        Deref @ LIST_OBJECT { .. } => {
            toString_listObject(var_field!((*value).values, JSON::LIST_OBJECT).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn toString_array(mut values: Arc<Vector::Vector<Arc<JSON>>>) -> Result<()> {
    Print::printBuf((literal!("[")).clone())?;
    for mut i in 1..=Vector::size(values.clone()) {
        if i.clone() != 1 {
            Print::printBuf((literal!(", ")).clone())?;
        }
        toString_work(Vector::getNoBounds(values.clone(), i.clone()))?;
    }
    Print::printBuf((literal!("]")).clone())?;
    Ok(())
}

pub fn toString_list(mut values: Arc<metamodelica::List<Arc<JSON>>>) -> Result<()> {
    let mut first: bool = true;
    Print::printBuf((literal!("[")).clone())?;
    for mut v in &*values.clone() {
        let mut v = v.clone();
        if first.clone() {
            first = false;
        } else {
            Print::printBuf((literal!(", ")).clone())?;
        }
        toString_work(v.clone())?;
    }
    Print::printBuf((literal!("]")).clone())?;
    Ok(())
}

pub fn toString_object(mut map: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<JSON>>>) -> Result<()> {
    Print::printBuf((literal!("{")).clone())?;
    for mut i in 1..=UnorderedMap::size(map.clone()) {
        if i.clone() != 1 {
            Print::printBuf((literal!(", ")).clone())?;
        }
        Print::printBuf((literal!("\"")).clone())?;
        Print::printBuf((UnorderedMap::keyAt(map.clone(), i.clone())).clone())?;
        Print::printBuf((literal!("\":")).clone())?;
        toString_work(UnorderedMap::valueAt(map.clone(), i.clone()))?;
    }
    Print::printBuf((literal!("}")).clone())?;
    Ok(())
}

pub fn toString_listObject(mut object: Arc<metamodelica::List<(ArcStr, Arc<JSON>)>>) -> Result<()> {
    let mut first: bool = true;
    let mut key: ArcStr = arcstr::literal!("");
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    Print::printBuf((literal!("{")).clone())?;
    for mut entry in &*object.clone().reverse() {
        let mut entry = entry.clone();
        (key, value) = entry.clone();
        if first.clone() {
            first = false;
        } else {
            Print::printBuf((literal!(", ")).clone())?;
        }
        Print::printBuf((literal!("\"")).clone())?;
        Print::printBuf((key.clone()).clone())?;
        Print::printBuf((literal!("\":")).clone())?;
        toString_work(value.clone())?;
    }
    Print::printBuf((literal!("}")).clone())?;
    Ok(())
}

pub fn toStringPP_work(mut value: Arc<JSON>, mut indent: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(value.clone()) {
        Deref @ STRING { .. } => {
            Print::printBuf((literal!("\"")).clone())?;
            Print::printBuf((System::escapedString((var_field!((*value).r#str, JSON::STRING).clone()).clone(), true)).clone())?;
            Print::printBuf((literal!("\"")).clone())?;
            ()
        },
        Deref @ TRUE { .. } => {
            Print::printBuf((literal!("true")).clone())?;
            ()
        },
        Deref @ FALSE { .. } => {
            Print::printBuf((literal!("false")).clone())?;
            ()
        },
        Deref @ NULL { .. } => {
            Print::printBuf((literal!("null")).clone())?;
            ()
        },
        Deref @ INTEGER { .. } => {
            Print::printBuf(ArcStr::from(::std::format!("{}", var_field!((*value).i, JSON::INTEGER).clone())))?;
            ()
        },
        Deref @ NUMBER { .. } => {
            Print::printBuf(ArcStr::from(::std::format!("{}", var_field!((*value).r, JSON::NUMBER).clone())))?;
            ()
        },
        Deref @ ARRAY { .. } => {
            toStringPP_array(var_field!((*value).values, JSON::ARRAY).clone(), (indent.clone()).clone())?;
            ()
        },
        Deref @ LIST { .. } => {
            toStringPP_list(var_field!((*value).values, JSON::LIST).clone(), (indent.clone()).clone())?;
            ()
        },
        Deref @ OBJECT { .. } => {
            toStringPP_object(var_field!((*value).values, JSON::OBJECT).clone(), (indent.clone()).clone())?;
            ()
        },
        Deref @ LIST_OBJECT { .. } => {
            toStringPP_listObject(var_field!((*value).values, JSON::LIST_OBJECT).clone(), (indent.clone()).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn toStringPP_array(mut values: Arc<Vector::Vector<Arc<JSON>>>, mut indent: ArcStr) -> Result<()> {
    let mut next_indent: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) };
    Print::printBuf((literal!("[\n")).clone())?;
    for mut i in 1..=Vector::size(values.clone()) {
        if i.clone() != 1 {
            Print::printBuf((literal!(",\n")).clone())?;
        }
        Print::printBuf((next_indent.clone()).clone())?;
        toStringPP_work(Vector::getNoBounds(values.clone(), i.clone()), (next_indent.clone()).clone())?;
    }
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf((indent.clone()).clone())?;
    Print::printBuf((literal!("]")).clone())?;
    Ok(())
}

pub fn toStringPP_list(mut values: Arc<metamodelica::List<Arc<JSON>>>, mut indent: ArcStr) -> Result<()> {
    let mut next_indent: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) };
    let mut first: bool = true;
    Print::printBuf((literal!("[\n")).clone())?;
    for mut v in &*values.clone() {
        let mut v = v.clone();
        if first.clone() {
            first = false;
        } else {
            Print::printBuf((literal!(",\n")).clone())?;
        }
        Print::printBuf((next_indent.clone()).clone())?;
        toStringPP_work(v.clone(), (next_indent.clone()).clone())?;
    }
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf((indent.clone()).clone())?;
    Print::printBuf((literal!("]")).clone())?;
    Ok(())
}

pub fn toStringPP_object(mut map: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<JSON>>>, mut indent: ArcStr) -> Result<()> {
    let mut next_indent: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) };
    Print::printBuf((literal!("{")).clone())?;
    for mut i in 1..=UnorderedMap::size(map.clone()) {
        Print::printBuf((if (i.clone() == 1) {literal!("\n")} else {literal!(",\n")}).clone())?;
        Print::printBuf((next_indent.clone()).clone())?;
        Print::printBuf((literal!("\"")).clone())?;
        Print::printBuf((UnorderedMap::keyAt(map.clone(), i.clone())).clone())?;
        Print::printBuf((literal!("\": ")).clone())?;
        toStringPP_work(UnorderedMap::valueAt(map.clone(), i.clone()), (next_indent.clone()).clone())?;
    }
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf((indent.clone()).clone())?;
    Print::printBuf((literal!("}")).clone())?;
    Ok(())
}

pub fn toStringPP_listObject(mut object: Arc<metamodelica::List<(ArcStr, Arc<JSON>)>>, mut indent: ArcStr) -> Result<()> {
    let mut first: bool = true;
    let mut key: ArcStr = arcstr::literal!("");
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut next_indent: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) };
    Print::printBuf((literal!("{\n")).clone())?;
    for mut entry in &*object.clone().reverse() {
        let mut entry = entry.clone();
        (key, value) = entry.clone();
        if first.clone() {
            first = false;
        } else {
            Print::printBuf((literal!(",\n")).clone())?;
        }
        Print::printBuf((next_indent.clone()).clone())?;
        Print::printBuf((literal!("\"")).clone())?;
        Print::printBuf((key.clone()).clone())?;
        Print::printBuf((literal!("\": ")).clone())?;
        toStringPP_work(value.clone(), (next_indent.clone()).clone())?;
    }
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf((indent.clone()).clone())?;
    Print::printBuf((literal!("}")).clone())?;
    Ok(())
}

pub type partialParser = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Token>>) -> Result<(Arc<JSON>, Arc<metamodelica::List<Token>>)> + 'static>;

pub fn parseFile(mut fileName: ArcStr) -> Result<Arc<JSON>> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut errTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (tokens, errTokens) = LexerJSON::scan((fileName.clone()).clone())?;
    reportErrors(errTokens.clone())?;
    value = parse_value_check_empty(tokens.clone())?;
    Ok(value)
}

pub fn hasKey(mut obj: Arc<JSON>, mut r#str: ArcStr) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ OBJECT { .. } => UnorderedMap::contains((r#str.clone()).clone(), var_field!((*obj).values, JSON::OBJECT).clone()),
        Deref @ LIST_OBJECT { .. } => {
            b = false;
            for mut entry in &*var_field!((*obj).values, JSON::LIST_OBJECT).clone() {
                let mut entry = entry.clone();
                if Util::tuple21(entry.clone()) == r#str.clone() {
                    b = true;
                }
            }
            b.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

pub fn get(mut obj: Arc<JSON>, mut r#str: ArcStr) -> Result<Arc<JSON>> {
    let mut out: Arc<JSON> = Arc::new(JSON::FALSE);
    out = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ OBJECT { .. } => UnorderedMap::getOrFail((r#str.clone()).clone(), var_field!((*obj).values, JSON::OBJECT).clone()),
        Deref @ LIST_OBJECT { .. } => {
            for mut entry in &*var_field!((*obj).values, JSON::LIST_OBJECT).clone() {
                let mut entry = entry.clone();
                if Util::tuple21(entry.clone()) == r#str.clone() {
                    out = Util::tuple22(entry.clone());
                    return Ok(out.clone());
                }
            }
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

pub fn getOrDefault(mut obj: Arc<JSON>, mut r#str: ArcStr, mut default: Arc<JSON>) -> Arc<JSON> {
    let mut out: Arc<JSON> = Arc::new(JSON::FALSE);
    out = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ OBJECT { .. } => UnorderedMap::getOrDefault((r#str.clone()).clone(), var_field!((*obj).values, JSON::OBJECT).clone(), default.clone()),
        Deref @ LIST_OBJECT { .. } => {
            for mut entry in &*var_field!((*obj).values, JSON::LIST_OBJECT).clone() {
                let mut entry = entry.clone();
                if Util::tuple21(entry.clone()) == r#str.clone() {
                    out = Util::tuple22(entry.clone());
                    return out.clone();
                }
            }
            default.clone()
        },
        _ => default.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub fn at(mut obj: Arc<JSON>, mut index: i32) -> Result<Arc<JSON>> {
    let mut out: Arc<JSON> = Arc::new(JSON::FALSE);
    out = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ ARRAY { .. } => Vector::get(var_field!((*obj).values, JSON::ARRAY).clone(), index.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

pub fn getString(mut obj: Arc<JSON>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ STRING { r#str: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#str = __pa0.clone();
    Ok(r#str)
}

pub fn getStringList(mut obj: Arc<JSON>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strl = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ OBJECT { .. } => ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (UnorderedMap::valueList(var_field!((*obj).values, JSON::OBJECT).clone())).into_iter().cloned() {
            let __x = getString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ LIST_OBJECT { .. } => ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (var_field!((*obj).values, JSON::LIST_OBJECT).clone()).into_iter().cloned() {
            let __x = getString(Util::tuple22(v.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc
    }),
        Deref @ ARRAY { .. } => Vector::mapToList(var_field!((*obj).values, JSON::ARRAY).clone(), (std::sync::Arc::new(getString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<JSON>) -> Result<ArcStr> + 'static>)),
        Deref @ LIST { .. } => ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (var_field!((*obj).values, JSON::LIST).clone()).into_iter().cloned() {
            let __x = getString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(strl)
}

pub fn getKeys(mut obj: Arc<JSON>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut keys: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    keys = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ OBJECT { .. } => UnorderedMap::keyList(var_field!((*obj).values, JSON::OBJECT).clone()),
        Deref @ LIST_OBJECT { .. } => ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*obj).values, JSON::LIST_OBJECT).clone()).into_iter().cloned() {
            let __x = Util::tuple21(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc
    }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(keys)
}

pub fn getBoolean(mut obj: Arc<JSON>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ TRUE { .. } => true,
        Deref @ FALSE { .. } => false,
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

pub fn size(mut obj: Arc<JSON>) -> i32 {
    let mut sz: i32 = 0;
    sz = (::match_deref::match_deref! { match &(obj.clone()) {
        Deref @ OBJECT { .. } => UnorderedMap::size(var_field!((*obj).values, JSON::OBJECT).clone()),
        Deref @ LIST_OBJECT { .. } => (var_field!((*obj).values, JSON::LIST_OBJECT).clone().len() as i32),
        Deref @ ARRAY { .. } => Vector::size(var_field!((*obj).values, JSON::ARRAY).clone()),
        Deref @ LIST { .. } => (var_field!((*obj).values, JSON::LIST).clone().len() as i32),
        _ => 1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sz
}

pub fn parse(mut content: ArcStr, mut fileName: ArcStr) -> Result<Arc<JSON>> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut errTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (tokens, errTokens) = LexerJSON::scanString((content.clone()).clone(), (fileName.clone()).clone())?;
    reportErrors(errTokens.clone())?;
    value = parse_value_check_empty(tokens.clone())?;
    Ok(value)
}

pub fn parse_value_check_empty(mut inTokens: Arc<metamodelica::List<Token>>) -> Result<Arc<JSON>> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (value, tokens) = parse_value(inTokens.clone())?;
    check_empty(tokens.clone())?;
    Ok(value)
}

pub fn parse_value(mut inTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<JSON>, Arc<metamodelica::List<Token>>)> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    not_eof(tokens.clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    tok = __pa0.clone();
    tokens = __pa1.clone();
    (value, tokens) = (match tok.id.clone() {
        LexerJSON::TokenId::STRING { .. } => {
            (value, tokens) = parse_string(inTokens.clone())?;
            (value.clone(), tokens.clone())
        },
        LexerJSON::TokenId::INTEGER { .. } => {
            (value, tokens) = parse_integer(inTokens.clone())?;
            (value.clone(), tokens.clone())
        },
        LexerJSON::TokenId::NUMBER { .. } => {
            (value, tokens) = parse_number(inTokens.clone())?;
            (value.clone(), tokens.clone())
        },
        LexerJSON::TokenId::OBJECTBEGIN => {
            (value, tokens) = parse_object(inTokens.clone())?;
            (value.clone(), tokens.clone())
        },
        LexerJSON::TokenId::ARRAYBEGIN => {
            (value, tokens) = parse_array(inTokens.clone())?;
            (value.clone(), tokens.clone())
        },
        LexerJSON::TokenId::TRUE => (Arc::new(crate::JSON::TRUE), tokens.clone()),
        LexerJSON::TokenId::FALSE => (Arc::new(crate::JSON::FALSE), tokens.clone()),
        LexerJSON::TokenId::NULL => (Arc::new(crate::JSON::NULL), tokens.clone()),
        _ => {
            errorExpected((literal!("a value")).clone(), tok.clone())?;
            bail!("fail")
        },
    });
    Ok((value, tokens))
}

pub fn parse_string(mut inTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<JSON>, Arc<metamodelica::List<Token>>)> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    let mut content: ArcStr = arcstr::literal!("");
    not_eof(tokens.clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    tok = __pa0.clone();
    tokens = __pa1.clone();
    if tok.id.clone() != TokenId::STRING.clone() {
        errorExpected((literal!("a String")).clone(), tok.clone())?;
    }
    content = (tokenContent(tok.clone())?).clone();
    if ((content.clone()).clone().len() as i32) == 2 {
        content = (literal!("")).clone();
    } else {
        content = (System::unescapedString(substring((content.clone()).clone(), 2, ((content.clone()).clone().len() as i32) - 1)?)).clone();
    }
    value = Arc::new(JSON::STRING { r#str: (content.clone()).clone() });
    Ok((value, tokens))
}

pub fn parse_integer(mut inTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<JSON>, Arc<metamodelica::List<Token>>)> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    let mut content: ArcStr = arcstr::literal!("");
    not_eof(tokens.clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    tok = __pa0.clone();
    tokens = __pa1.clone();
    if tok.id.clone() != TokenId::INTEGER.clone() {
        errorExpected((literal!("an integer")).clone(), tok.clone())?;
    }
    content = (tokenContent(tok.clone())?).clone();
    value = Arc::new(JSON::INTEGER { i: stringInt((content.clone()).clone())? });
    Ok((value, tokens))
}

pub fn parse_number(mut inTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<JSON>, Arc<metamodelica::List<Token>>)> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    let mut content: ArcStr = arcstr::literal!("");
    not_eof(tokens.clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    tok = __pa0.clone();
    tokens = __pa1.clone();
    if tok.id.clone() != TokenId::NUMBER.clone() {
        errorExpected((literal!("a (real) number")).clone(), tok.clone())?;
    }
    content = (tokenContent(tok.clone())?).clone();
    value = Arc::new(JSON::NUMBER { r: stringReal((content.clone()).clone())? });
    Ok((value, tokens))
}

pub fn parse_array(mut inTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<JSON>, Arc<metamodelica::List<Token>>)> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    let mut values: Arc<Vector::Vector<Arc<JSON>>> = Vector::new(0);
    let mut cont: bool = false;
    value = emptyObject();
    tokens = parse_expected_token(tokens.clone(), TokenId::ARRAYBEGIN.clone())?;
    cont = peek_id(tokens.clone())? != TokenId::ARRAYEND.clone();
    while cont.clone() {
        (value, tokens) = parse_value(tokens.clone())?;
        Vector::push(values.clone(), value.clone());
        (tokens, cont) = eat_if_next_token_matches(tokens.clone(), TokenId::COMMA.clone())?;
    }
    tokens = parse_expected_token(tokens.clone(), TokenId::ARRAYEND.clone())?;
    value = Arc::new(JSON::ARRAY { values: values.clone() });
    Ok((value, tokens))
}

pub fn parse_object(mut inTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<JSON>, Arc<metamodelica::List<Token>>)> {
    let mut value: Arc<JSON> = Arc::new(JSON::FALSE);
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    let mut values: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<JSON>>>;
    let mut key: ArcStr = arcstr::literal!("");
    let mut cont: bool = false;
    values = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    tokens = parse_expected_token(tokens.clone(), TokenId::OBJECTBEGIN.clone())?;
    cont = peek_id(tokens.clone())? != TokenId::ARRAYEND.clone();
    while cont.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(parse_string(tokens.clone())?) {
            (Deref @ STRING { r#str: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        key = __pa0.clone();
        tokens = __pa1.clone();
        tokens = parse_expected_token(tokens.clone(), TokenId::COLON.clone())?;
        (value, tokens) = parse_value(tokens.clone())?;
        UnorderedMap::add((key.clone()).clone(), value.clone(), values.clone())?;
        (tokens, cont) = eat_if_next_token_matches(tokens.clone(), TokenId::COMMA.clone())?;
    }
    tokens = parse_expected_token(tokens.clone(), TokenId::OBJECTEND.clone())?;
    value = Arc::new(JSON::OBJECT { values: values.clone() });
    Ok((value, tokens))
}

fn reportErrors(mut tokens: Arc<metamodelica::List<Token>>) -> Result<()> {
    let mut i: i32 = 0;
    for mut t in &*tokens.clone() {
        let mut t = t.clone();
        i = i.clone() + 1;
        if i.clone() > 10 {
            Error::addMessage(Error::SCANNER_ERROR_LIMIT.clone(), metamodelica::nil())?;
        }
        Error::addSourceMessage(Error::SCANNER_ERROR.clone(), list![(tokenContent(t.clone())?).clone()], tokenSourceInfo(t.clone())?)?;
    }
    if !(tokens.clone().is_empty()) {
        bail!("fail");
    }
    Ok(())
}

fn not_eof(mut tokens: Arc<metamodelica::List<Token>>) -> Result<Arc<metamodelica::List<Token>>> {
    let mut tokens: Arc<metamodelica::List<Token>> = tokens;
    if tokens.clone().is_empty() {
        Error::addCompilerError((literal!("JSON expected value, got <EOF>...")).clone())?;
        bail!("fail");
    }
    Ok(tokens)
}

fn peek_id(mut tokens: Arc<metamodelica::List<Token>>) -> Result<TokenId> {
    let mut nextToken: TokenId = TokenId::_NO_TOKEN;
    let mut tok: Token = <Token as ::std::default::Default>::default();
    if tokens.clone().is_empty() {
        nextToken = TokenId::_NO_TOKEN.clone();
    }
    tok = listHead(tokens.clone())?;
    nextToken = tok.id.clone();
    Ok(nextToken)
}

fn eat_if_next_token_matches(mut tokens: Arc<metamodelica::List<Token>>, mut expectedToken: TokenId) -> Result<(Arc<metamodelica::List<Token>>, bool)> {
    let mut tokens: Arc<metamodelica::List<Token>> = tokens;
    let mut matched: bool = false;
    let mut tok: Token = <Token as ::std::default::Default>::default();
    if tokens.clone().is_empty() {
        return Ok((tokens.clone(), matched.clone()));
    }
    tok = listHead(tokens.clone())?;
    if tok.id.clone() != expectedToken.clone() {
        return Ok((tokens.clone(), matched.clone()));
    }
    matched = true;
    let __pa0 = ::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tokens = __pa0.clone();
    Ok((tokens, matched))
}

fn parse_expected_token(mut tokens: Arc<metamodelica::List<Token>>, mut expectedToken: TokenId) -> Result<Arc<metamodelica::List<Token>>> {
    let mut tokens: Arc<metamodelica::List<Token>> = tokens;
    let mut tok: Token = <Token as ::std::default::Default>::default();
    not_eof(tokens.clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    tok = __pa0.clone();
    tokens = __pa1.clone();
    if tok.id.clone() != expectedToken.clone() {
        Error::addSourceMessage(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected a ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", expectedToken.clone()))); __mm_s.push_str(&*literal!(", got token: ")); __mm_s.push_str(&*tokenContent(tok.clone())?); ArcStr::from(__mm_s) }).clone()], tokenSourceInfo(tok.clone())?)?;
        bail!("fail");
    }
    Ok(tokens)
}

fn check_empty(mut tokens: Arc<metamodelica::List<Token>>) -> Result<()> {
    let mut tok: Token = <Token as ::std::default::Default>::default();
    if tokens.clone().is_empty() {
        return Ok(());
    }
    tok = listHead(tokens.clone())?;
    Error::addSourceMessage(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected <EOF>, got more tokens, starting with: ")); __mm_s.push_str(&*tokenContent(tok.clone())?); ArcStr::from(__mm_s) }).clone()], tokenSourceInfo(tok.clone())?)?;
    bail!("fail");
    Ok(())
}

fn errorExpected(mut expected: ArcStr, mut tok: Token) -> Result<()> {
    Error::addSourceMessage(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("JSON expected ")); __mm_s.push_str(&*expected.clone()); __mm_s.push_str(&*literal!(", got token ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", tok.id.clone()))); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*tokenContent(tok.clone())?); ArcStr::from(__mm_s) }).clone()], tokenSourceInfo(tok.clone())?)?;
    bail!("fail");
    Ok(())
}


