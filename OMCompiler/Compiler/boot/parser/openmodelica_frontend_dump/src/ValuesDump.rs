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

use crate::AbsynUtil;
use crate::Dump;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub fn valString(mut inValue: Arc<Values::Value>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut handle: i32 = 0;
    handle = Print::saveAndClearBuf()?;
    valString2(inValue.clone())?;
    outString = (Print::getString()?).clone();
    Print::restoreBuf(handle.clone())?;
    Ok(outString)
}

pub fn valString2(mut inValue: Arc<Values::Value>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::INTEGER { integer: n } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (intString(n.clone())).clone();
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::REAL { real: x } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (realString(x.clone())).clone();
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::STRING { string: s } => {
                    Print::printBuf((literal!("\"")).clone())?;
                    Print::printBuf((System::escapedString((s.clone()).clone(), false)).clone())?;
                    Print::printBuf((literal!("\"")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::BOOL { boolean: false } => {
                    Print::printBuf((literal!("false")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::BOOL { boolean: true } => {
                    Print::printBuf((literal!("true")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ENUM_LITERAL { name: p, .. } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: vs, .. } => {
                    Print::printBuf((literal!("{")).clone())?;
                    valListString(vs.clone())?;
                    Print::printBuf((literal!("}")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Nil } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::TUPLE { valueLst: vs } => {
                    Print::printBuf((literal!("(")).clone())?;
                    valListString(vs.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_TUPLE { valueLst: Deref @ metamodelica::List::Nil } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_TUPLE { valueLst: vs } => {
                    Print::printBuf((literal!("(")).clone())?;
                    valListString(vs.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { comp: ids, orderd: xs, record_: Deref @ Absyn::Path::IDENT { name: Deref @ "SimulationResult" }, .. } => {
                    let mut ids = (*ids).clone();
                    let mut xs = (*xs).clone();
                    Print::printBuf((literal!("record SimulationResult\n")).clone())?;
                    (xs, ids) = filterSimulationResults(Flags::isSet(Flags::SHORT_OUTPUT.clone())?, xs.clone(), ids.clone(), metamodelica::nil(), metamodelica::nil())?;
                    valRecordString(xs.clone(), ids.clone())?;
                    Print::printBuf((literal!("end SimulationResult;")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { comp: ids, orderd: xs, record_: recordPath, .. } => {
                    let mut recordName: ArcStr = arcstr::literal!("");
                    recordName = AbsynUtil::pathStringNoQual(recordPath.clone(), (literal!(".")).clone(), true, false)?;
                    Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("record ")); __mm_s.push_str(&*recordName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    valRecordString(xs.clone(), ids.clone())?;
                    Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("end ")); __mm_s.push_str(&*recordName.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::OPTION { some: Some(r) } => {
                    Print::printBuf((literal!("SOME(")).clone())?;
                    valString2(r.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::OPTION { some: None } => {
                    Print::printBuf((literal!("NONE()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_BOX { value: r } => {
                    Print::printBuf((literal!("#(")).clone())?;
                    valString2(r.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } } => {
                    Print::printBuf((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } } => {
                    Print::printBuf((Dump::printComponentRefStr(cr.clone())?).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: c } => {
                    Print::printBuf((literal!("$Code(")).clone())?;
                    Print::printBuf((Dump::printCodeStr(c.clone())?).clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::LIST { valueLst: vs } => {
                    Print::printBuf((literal!("{")).clone())?;
                    valListString(vs.clone())?;
                    Print::printBuf((literal!("}")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_ARRAY { valueLst: vs } => {
                    Print::printBuf((literal!("meta_array(")).clone())?;
                    valListString(vs.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ENUM_LITERAL { name: p, index: n } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!(" /* ENUM: ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" */")); ArcStr::from(__mm_s) }).clone();
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::NORETCALL => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_FAIL => {
                    Print::printBuf((literal!("fail()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::EMPTY { tyStr, name, scope, .. } => {
                    Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("/* <EMPTY(scope: ")); __mm_s.push_str(&*scope.clone()); __mm_s.push_str(&*literal!(", name: ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(", ty: ")); __mm_s.push_str(&*tyStr.clone()); __mm_s.push_str(&*literal!(")> */")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ValuesDump.valString2 failed")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn filterSimulationResults(mut filter: bool, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inIds: Arc<metamodelica::List<ArcStr>>, mut valacc: Arc<metamodelica::List<Arc<Values::Value>>>, mut idacc: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<Arc<Values::Value>>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outValues: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut outIds: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outValues, outIds) = (::match_deref::match_deref! { match &((filter.clone(), inValues.clone(), inIds.clone(), valacc.clone(), idacc.clone())) {
        (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            (valacc.clone().reverse(), idacc.clone().reverse())
        },
        (true, Deref @ metamodelica::List::Cons { head: v, tail: vrest }, Deref @ metamodelica::List::Cons { head: id @ Deref @ "messages", tail: idrest }, _, _) => {
            (outValues, outIds) = filterSimulationResults(filter.clone(), vrest.clone(), idrest.clone(), cons(v.clone(), valacc.clone()), cons(id.clone(), idacc.clone()))?;
            (outValues.clone(), outIds.clone())
        },
        (true, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: vrest }, Deref @ metamodelica::List::Cons { head: id @ Deref @ "resultFile", tail: idrest }, _, _) => {
            let mut r#str = (*r#str).clone();
            r#str = (System::basename((r#str.clone()).clone())).clone();
            (outValues, outIds) = filterSimulationResults(filter.clone(), vrest.clone(), idrest.clone(), cons(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }), valacc.clone()), cons(id.clone(), idacc.clone()))?;
            (outValues.clone(), outIds.clone())
        },
        (true, Deref @ metamodelica::List::Cons { head: _, tail: vrest }, Deref @ metamodelica::List::Cons { head: _, tail: idrest }, _, _) => {
            (outValues, outIds) = filterSimulationResults(filter.clone(), vrest.clone(), idrest.clone(), valacc.clone(), idacc.clone())?;
            (outValues.clone(), outIds.clone())
        },
        (false, _, _, _, _) => {
            (inValues.clone(), inIds.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outValues, outIds))
}

fn valRecordString(mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inIds: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inValues.clone(), inIds.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: x, tail: xs @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, Deref @ metamodelica::List::Cons { head: id, tail: ids @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }) => {
                    Print::printBuf((literal!("    ")).clone())?;
                    Print::printBuf((id.clone()).clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    valString2(x.clone())?;
                    Print::printBuf((literal!(",\n")).clone())?;
                    valRecordString(xs.clone(), ids.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: id, tail: Deref @ metamodelica::List::Nil }) => {
                    Print::printBuf((literal!("    ")).clone())?;
                    Print::printBuf((id.clone()).clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    valString2(x.clone())?;
                    Print::printBuf((literal!("\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (xs, ids) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ValuesUtil.valRecordString failed:\nids: ")); __mm_s.push_str(&*stringDelimitList(ids.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\nvals: ")); __mm_s.push_str(&*stringDelimitList(List::map(xs.clone(), Arc::new(valString)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn valListString(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inValueLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: v, tail: Deref @ metamodelica::List::Nil } => {
            valString2(v.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: v, tail: vs } => {
            valString2(v.clone())?;
            Print::printBuf((literal!(", ")).clone())?;
            valListString(vs.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn printVal(mut v: Arc<Values::Value>) -> Result<()> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (valString(v.clone())?).clone();
    Print::printBuf((s.clone()).clone())?;
    Ok(())
}

pub fn printValStr(mut v: Arc<Values::Value>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (valString(v.clone())?).clone();
    Ok(s)
}

pub fn unparseValues(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inValueLst.clone()) {
        Deref @ metamodelica::List::Cons { head: v, tail: vallst } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            s1 = (unparseDescription(list![v.clone()])?).clone();
            s2 = (unparseValueNumbers(list![v.clone()])?).clone();
            s3 = (unparseValues(vallst.clone())?).clone();
            r#str = stringAppendList(list![(s1.clone()).clone(), (s2.clone()).clone(), (literal!("\n")).clone(), (s3.clone()).clone()]);
            r#str.clone()
        },
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn unparseValueNumbers(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inValueLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::TUPLE { valueLst: lst }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (unparseValueNumbers(lst.clone())?).clone();
            s2 = (unparseValueNumbers(xs.clone())?).clone();
            res = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::META_TUPLE { valueLst: lst }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (unparseValueNumbers(lst.clone())?).clone();
            s2 = (unparseValueNumbers(xs.clone())?).clone();
            res = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: lst, .. }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (unparseValueNumbers(lst.clone())?).clone();
            s2 = (unparseValueNumbers(xs.clone())?).clone();
            res = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            let mut istr: ArcStr = arcstr::literal!("");
            s1 = (unparseValueNumbers(xs.clone())?).clone();
            istr = (intString(i.clone())).clone();
            s2 = (stringAppend((istr.clone()).clone(), (literal!(" ")).clone())).clone();
            res = (stringAppend((s2.clone()).clone(), (s1.clone()).clone())).clone();
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            let mut istr: ArcStr = arcstr::literal!("");
            s1 = (unparseValueNumbers(xs.clone())?).clone();
            istr = (realString(r.clone())).clone();
            s2 = (stringAppend((istr.clone()).clone(), (literal!(" ")).clone())).clone();
            res = (stringAppend((s2.clone()).clone(), (s1.clone()).clone())).clone();
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: sval }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (unparseValueNumbers(xs.clone())?).clone();
            s2 = (stringAppend((sval.clone()).clone(), (literal!(" ")).clone())).clone();
            res = (stringAppend((s2.clone()).clone(), (s1.clone()).clone())).clone();
            res.clone()
        },
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn unparseDescription(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inValueLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            s1 = (unparseDescription(xs.clone())?).clone();
            r#str = (stringAppend((literal!("# i!\n")).clone(), (s1.clone()).clone())).clone();
            r#str.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { .. }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            s1 = (unparseDescription(xs.clone())?).clone();
            r#str = (stringAppend((literal!("# r!\n")).clone(), (s1.clone()).clone())).clone();
            r#str.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: sval }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut slenstr: ArcStr = arcstr::literal!("");
            let mut slen: i32 = 0;
            s1 = (unparseDescription(xs.clone())?).clone();
            slen = ((sval.clone()).clone().len() as i32);
            slenstr = (intString(slen.clone())).clone();
            r#str = stringAppendList(list![(literal!("# s! 1 ")).clone(), (slenstr.clone()).clone(), (literal!("\n")).clone(), (s1.clone()).clone()]);
            r#str.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vallst, .. }, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s4: ArcStr = arcstr::literal!("");
            s1 = (unparseDescription(xs.clone())?).clone();
            s2 = (unparseArrayDescription(vallst.clone())?).clone();
            s4 = (stringAppend((s2.clone()).clone(), (s1.clone()).clone())).clone();
            r#str = (stringAppend((s4.clone()).clone(), (literal!(" \n")).clone())).clone();
            r#str.clone()
        },
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn unparseArrayDescription(mut lst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut pt: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    let mut s4: ArcStr = arcstr::literal!("");
    let mut s5: ArcStr = arcstr::literal!("");
    let mut s6: ArcStr = arcstr::literal!("");
    let mut i1: i32 = 0;
    pt = (unparsePrimType(lst.clone())).clone();
    s1 = (stringAppend((literal!("# ")).clone(), (pt.clone()).clone())).clone();
    s2 = (stringAppend((s1.clone()).clone(), (literal!("[")).clone())).clone();
    i1 = unparseNumDims(lst.clone(), 0);
    s3 = (intString(i1.clone())).clone();
    s4 = (stringAppend((s2.clone()).clone(), (s3.clone()).clone())).clone();
    s5 = (stringAppend((s4.clone()).clone(), (literal!(" ")).clone())).clone();
    s6 = (unparseDimSizes(lst.clone())?).clone();
    r#str = (stringAppend((s5.clone()).clone(), (s6.clone()).clone())).clone();
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn unparsePrimType(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inValueLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: elts, .. }, tail: _ } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (unparsePrimType(elts.clone())).clone();
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ } => {
            literal!("i")
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ } => {
            literal!("r")
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { .. }, tail: _ } => {
            literal!("s")
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { .. }, tail: _ } => {
            literal!("b")
        },
        Deref @ metamodelica::List::Nil => {
            literal!("{}")
        },
        _ => {
            literal!("error")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn unparseNumDims(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut inInteger: i32) -> i32 {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inValueLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: _ } => {
            unparseNumDims(vals.clone(), inInteger.clone() + 1)
        },
        _ => {
            inInteger.clone() + 1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInteger
}

fn unparseDimSizes(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inValueLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                lst @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: _ } => {
                    let mut i1: i32 = 0;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    i1 = (lst.clone().len() as i32);
                    s1 = (intString(i1.clone())).clone();
                    s2 = (stringAppend((s1.clone()).clone(), (literal!(" ")).clone())).clone();
                    s3 = (unparseDimSizes(vals.clone())?).clone();
                    res = (stringAppend((s2.clone()).clone(), (s3.clone()).clone())).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                lst => {
                    let mut len: i32 = 0;
                    let mut res: ArcStr = arcstr::literal!("");
                    len = (lst.clone().len() as i32);
                    res = (intString(len.clone())).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

