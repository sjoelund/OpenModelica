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
use crate::DAEDump;
use crate::Expression;
use crate::Patternm;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_dump::Graphviz;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
/*
 * - Printing expressions
 *   This module provides some functions to print data to the standard
 *   output.  This is used for error messages, and for debugging the
 *   semantic description.
 */
pub fn subscriptString(mut subscript: Arc<DAE::Subscript>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (intString(i.clone())).clone();
            res.clone()
        },
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { name: enum_lit, .. } } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (AbsynUtil::pathString(enum_lit.clone(), (literal!(".")).clone(), true, false)?).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn binopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (if (Config::typeinfo()?) {binopSymbol2(inOperator.clone())?} else {binopSymbol1(inOperator.clone())}).clone();
    Ok(outString)
}

pub fn binopSymbol1(mut inOperator: DAE::Operator) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::ADD { .. } => literal!(" + "),
        DAE::Operator::SUB { .. } => literal!(" - "),
        DAE::Operator::MUL { .. } => literal!(" * "),
        DAE::Operator::DIV { .. } => literal!(" / "),
        DAE::Operator::POW { .. } => literal!(" ^ "),
        DAE::Operator::ADD_ARR { .. } => literal!(" + "),
        DAE::Operator::SUB_ARR { .. } => literal!(" - "),
        DAE::Operator::MUL_ARR { .. } => literal!(" * "),
        DAE::Operator::DIV_ARR { .. } => literal!(" / "),
        DAE::Operator::POW_ARR { .. } => literal!(" ^ "),
        DAE::Operator::POW_ARR2 { .. } => literal!(" ^ "),
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => literal!(" * "),
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => literal!(" + "),
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => literal!(" - "),
        DAE::Operator::POW_SCALAR_ARRAY { .. } => literal!(" ^ "),
        DAE::Operator::POW_ARRAY_SCALAR { .. } => literal!(" ^ "),
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => literal!(" * "),
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => literal!(" * "),
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => literal!(" / "),
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => literal!(" / "),
        _ => literal!(" <UNKNOWN_SYMBOL> "),
    })).clone();
    outString
}

pub fn debugBinopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::ADD { .. } => literal!(" + "),
        DAE::Operator::SUB { .. } => literal!(" - "),
        DAE::Operator::MUL { .. } => literal!(" * "),
        DAE::Operator::DIV { .. } => literal!(" / "),
        DAE::Operator::POW { .. } => literal!(" ^ "),
        DAE::Operator::EQUAL { .. } => literal!(" = "),
        DAE::Operator::ADD_ARR { .. } => literal!(" +ARR "),
        DAE::Operator::SUB_ARR { .. } => literal!(" -ARR "),
        DAE::Operator::MUL_ARR { .. } => literal!(" *ARR "),
        DAE::Operator::DIV_ARR { .. } => literal!(" /ARR "),
        DAE::Operator::POW_ARR { .. } => literal!(" ^ARR "),
        DAE::Operator::POW_ARR2 { .. } => literal!(" ^ARR2 "),
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => literal!(" ARR*S "),
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => literal!(" ARR+S "),
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => literal!(" - "),
        DAE::Operator::POW_SCALAR_ARRAY { .. } => literal!(" S^ARR "),
        DAE::Operator::POW_ARRAY_SCALAR { .. } => literal!(" ARR^S "),
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => literal!(" Dot "),
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => literal!(" MatrixProd "),
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => literal!(" S/ARR "),
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => literal!(" ARR/S "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn binopSymbol2(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::ADD { ty: ref t } => {
            let mut ts: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            ts = (TypesDump::unparseType(t.clone())?).clone();
            s = stringAppendList(list![(literal!(" +<")).clone(), (ts.clone()).clone(), (literal!("> ")).clone()]);
            s.clone()
        },
        DAE::Operator::SUB { ty: ref t } => {
            let mut ts: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            ts = (TypesDump::unparseType(t.clone())?).clone();
            s = stringAppendList(list![(literal!(" -<")).clone(), (ts.clone()).clone(), (literal!("> ")).clone()]);
            s.clone()
        },
        DAE::Operator::MUL { ty: ref t } => {
            let mut ts: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            ts = (TypesDump::unparseType(t.clone())?).clone();
            s = stringAppendList(list![(literal!(" *<")).clone(), (ts.clone()).clone(), (literal!("> ")).clone()]);
            s.clone()
        },
        DAE::Operator::DIV { ty: ref t } => {
            let mut ts: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            ts = (TypesDump::unparseType(t.clone())?).clone();
            s = stringAppendList(list![(literal!(" /<")).clone(), (ts.clone()).clone(), (literal!("> ")).clone()]);
            s.clone()
        },
        DAE::Operator::POW { .. } => {
            literal!(" ^ ")
        },
        DAE::Operator::ADD_ARR { ty: ref t } => {
            let mut ts: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            ts = (TypesDump::unparseType(t.clone())?).clone();
            s = stringAppendList(list![(literal!(" +<ADD_ARR><")).clone(), (ts.clone()).clone(), (literal!("> ")).clone()]);
            s.clone()
        },
        DAE::Operator::SUB_ARR { ty: ref t } => {
            let mut ts: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            ts = (TypesDump::unparseType(t.clone())?).clone();
            s = stringAppendList(list![(literal!(" -<SUB_ARR><")).clone(), (ts.clone()).clone(), (literal!("> ")).clone()]);
            s.clone()
        },
        DAE::Operator::MUL_ARR { .. } => {
            literal!(" *<MUL_ARRAY> ")
        },
        DAE::Operator::DIV_ARR { ty: ref t } => {
            let mut ts: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            ts = (TypesDump::unparseType(t.clone())?).clone();
            s = stringAppendList(list![(literal!(" /<DIV_ARR><")).clone(), (ts.clone()).clone(), (literal!("> ")).clone()]);
            s.clone()
        },
        DAE::Operator::POW_ARR { .. } => {
            literal!(" ^<POW_ARR> ")
        },
        DAE::Operator::POW_ARR2 { .. } => {
            literal!(" ^<POW_ARR2> ")
        },
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => {
            literal!(" *<MUL_ARRAY_SCALAR> ")
        },
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => {
            literal!(" +<ADD_ARRAY_SCALAR> ")
        },
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => {
            literal!(" -<SUB_SCALAR_ARRAY> ")
        },
        DAE::Operator::POW_SCALAR_ARRAY { .. } => {
            literal!(" ^<POW_SCALAR_ARRAY> ")
        },
        DAE::Operator::POW_ARRAY_SCALAR { .. } => {
            literal!(" ^<POW_ARRAY_SCALAR> ")
        },
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => {
            literal!(" *<MUL_SCALAR_PRODUCT> ")
        },
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => {
            literal!(" *<MUL_MATRIX_PRODUCT> ")
        },
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => {
            literal!(" /<DIV_SCALAR_ARRAY> ")
        },
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => {
            literal!(" /<DIV_ARRAY_SCALAR> ")
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unaryopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::UMINUS { .. } => if (Config::typeinfo()?) {literal!("-<UMINUS>")} else {literal!("-")},
        DAE::Operator::UMINUS_ARR { .. } => if (Config::typeinfo()?) {literal!("-<UMINUS_ARR>")} else {literal!("-")},
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn lbinopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::AND { ty: _ } => literal!(" and "),
        DAE::Operator::OR { ty: _ } => literal!(" or "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn lunaryopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::NOT { ty: _ } => literal!("not "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn relopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::LESS { .. } => literal!(" < "),
        DAE::Operator::LESSEQ { .. } => literal!(" <= "),
        DAE::Operator::GREATER { .. } => literal!(" > "),
        DAE::Operator::GREATEREQ { .. } => literal!(" >= "),
        DAE::Operator::EQUAL { .. } => literal!(" == "),
        DAE::Operator::NEQUAL { .. } => literal!(" <> "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn printList<Type_a: Clone + 'static>(mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>, mut inString: ArcStr) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>;

    let () = 'mc: {
        let __mc_input = (inTypeALst.clone(), inFuncTypeTypeATo.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, r, _) => {
                    r(h.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, r, sep) => {
                    r(h.clone())?;
                    Print::printBuf((sep.clone()).clone())?;
                    printList(t.clone(), r.clone(), (sep.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printRow(mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<()> {
    printList(es_1.clone(), (std::sync::Arc::new(printExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>), (literal!(",")).clone())?;
    Ok(())
}

pub fn debugPrintSubscriptStr(mut inSubscript: Arc<DAE::Subscript>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ DAE::Subscript::WHOLEDIM => {
            literal!(":")
        },
        Deref @ DAE::Subscript::INDEX { exp: e1 } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (dumpExpStr(e1.clone(), 0)?).clone();
            s = (System::stringReplace((s.clone()).clone(), (literal!("\n")).clone(), (literal!("")).clone())?).clone();
            s.clone()
        },
        Deref @ DAE::Subscript::SLICE { exp: e1 } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (dumpExpStr(e1.clone(), 0)?).clone();
            s = (System::stringReplace((s.clone()).clone(), (literal!("\n")).clone(), (literal!("")).clone())?).clone();
            s.clone()
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e1 } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (dumpExpStr(e1.clone(), 0)?).clone();
            s = (System::stringReplace((s.clone()).clone(), (literal!("\n")).clone(), (literal!("")).clone())?).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1:")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printSubscriptLstStr(mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = stringDelimitList(List::map(inSubscriptLst.clone(), (std::sync::Arc::new(ExpressionBasics::printSubscriptStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>)), (literal!(" , ")).clone());
    outString
}

pub fn printExpListStr(mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    res = stringDelimitList(List::map(expl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone());
    res
}

// stefan
pub fn printExpListStrNoSpace(mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    res = stringAppendList(List::map(expl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)));
    res
}

pub fn printOptExpStr(mut oexp: Option<Arc<DAE::Exp>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(oexp.clone()) {
        Some(e) => {
            ExpressionBasics::printExpStr(e.clone())?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn printCrefsFromExpStr(mut e: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (Tpl::tplString2((std::sync::Arc::new(ExpressionDumpTpl::dumpExpCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<DAE::Exp>, ArcStr) -> Result<Tpl::Text> + 'static>), e.clone(), (literal!("")).clone())?).clone();
    Ok(s)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn printExp2Str<Type_a: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut stringDelimiter: ArcStr, mut opcreffunc: Option<(Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>, Type_a)>, mut opcallfunc: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, Option<(Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>, Type_a)>) -> Result<ArcStr> + 'static>>) -> Result<ArcStr> {
    pub type printComponentRefStrFunc<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>;

    pub type printCallFunc<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, Option<(Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>, Type_a)>) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inExp.clone(), opcreffunc.clone(), opcallfunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::EMPTY { tyStr, name, scope, .. }, _, _) => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<EMPTY(scope: ")); __mm_s.push_str(&*scope.clone()); __mm_s.push_str(&*literal!(", name: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(name.clone())?); __mm_s.push_str(&*literal!(", ty: ")); __mm_s.push_str(&*tyStr.clone()); __mm_s.push_str(&*literal!(")>")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: i }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (intString(i.clone())).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (realString(r.clone())).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: s }, _, _) => {
                    let mut s = (*s).clone();
                    s = (System::escapedString((s.clone()).clone(), false)).clone();
                    s = stringAppendList(list![(stringDelimiter.clone()).clone(), (s.clone()).clone(), (stringDelimiter.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: b }, _, _) => {
                    Ok(boolString(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: c, .. }, Some((pcreffunc, creffuncparam)), _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = pcreffunc(c.clone(), creffuncparam.clone())?;
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: c, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    if listMember((literal!("dataReconciliation")).clone(), Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())?) || listMember((literal!("dataReconciliationStateEstimation")).clone(), Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())?) || listMember((literal!("dataReconciliationBoundaryConditions")).clone(), Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())?) {
                        s = (System::stringReplace((s.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
                    }
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ENUM_LITERAL { name: lit, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (AbsynUtil::pathString(lit.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s2_1: ArcStr = arcstr::literal!("");
                    let mut p1: i32 = 0;
                    let mut p2: i32 = 0;
                    let mut p: i32 = 0;
                    sym = (binopSymbol(op.clone())?).clone();
                    s1 = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s2 = (printExp2Str(e2.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    p = expPriority(e.clone());
                    p1 = expPriority(e1.clone());
                    p2 = expPriority(e2.clone());
                    s1_1 = (parenthesize((s1.clone()).clone(), p1.clone(), p.clone(), false)?).clone();
                    s2_1 = (parenthesize((s2.clone()).clone(), p2.clone(), p.clone(), true)?).clone();
                    s = stringAppendList(list![(s1_1.clone()).clone(), (sym.clone()).clone(), (s2_1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { operator: op, exp: e1 }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut p1: i32 = 0;
                    let mut p: i32 = 0;
                    sym = (unaryopSymbol(op.clone())?).clone();
                    s = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    p = expPriority(e.clone());
                    p1 = expPriority(e1.clone());
                    s_1 = (parenthesize((s.clone()).clone(), p1.clone(), p.clone(), true)?).clone();
                    s_2 = (stringAppend((sym.clone()).clone(), (s_1.clone()).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s2_1: ArcStr = arcstr::literal!("");
                    let mut p1: i32 = 0;
                    let mut p2: i32 = 0;
                    let mut p: i32 = 0;
                    sym = (lbinopSymbol(op.clone())?).clone();
                    s1 = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s2 = (printExp2Str(e2.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    p = expPriority(e.clone());
                    p1 = expPriority(e1.clone());
                    p2 = expPriority(e2.clone());
                    s1_1 = (parenthesize((s1.clone()).clone(), p1.clone(), p.clone(), false)?).clone();
                    s2_1 = (parenthesize((s2.clone()).clone(), p2.clone(), p.clone(), true)?).clone();
                    s = stringAppendList(list![(s1_1.clone()).clone(), (sym.clone()).clone(), (s2_1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut p1: i32 = 0;
                    let mut p: i32 = 0;
                    sym = (lunaryopSymbol(op.clone())?).clone();
                    s = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    p = expPriority(e.clone());
                    p1 = expPriority(e1.clone());
                    s_1 = (parenthesize((s.clone()).clone(), p1.clone(), p.clone(), false)?).clone();
                    s_2 = (stringAppend((sym.clone()).clone(), (s_1.clone()).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { exp2: e2, operator: op, exp1: e1, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s2_1: ArcStr = arcstr::literal!("");
                    let mut p1: i32 = 0;
                    let mut p2: i32 = 0;
                    let mut p: i32 = 0;
                    sym = (relopSymbol(op.clone())?).clone();
                    s1 = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s2 = (printExp2Str(e2.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    p = expPriority(e.clone());
                    p1 = expPriority(e1.clone());
                    p2 = expPriority(e2.clone());
                    s1_1 = (parenthesize((s1.clone()).clone(), p1.clone(), p.clone(), false)?).clone();
                    s2_1 = (parenthesize((s2.clone()).clone(), p2.clone(), p.clone(), true)?).clone();
                    s = stringAppendList(list![(s1_1.clone()).clone(), (sym.clone()).clone(), (s2_1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: tb, expElse: fb }, _, _) => {
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut cs: ArcStr = arcstr::literal!("");
                    let mut ts: ArcStr = arcstr::literal!("");
                    let mut cs_1: ArcStr = arcstr::literal!("");
                    let mut ts_1: ArcStr = arcstr::literal!("");
                    let mut fs_1: ArcStr = arcstr::literal!("");
                    let mut pc: i32 = 0;
                    let mut pt: i32 = 0;
                    let mut pf: i32 = 0;
                    let mut p: i32 = 0;
                    cs = (printExp2Str(cond.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    ts = (printExp2Str(tb.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    fs = (printExp2Str(fb.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    p = expPriority(e.clone());
                    pc = expPriority(cond.clone());
                    pt = expPriority(tb.clone());
                    pf = expPriority(fb.clone());
                    cs_1 = (parenthesize((cs.clone()).clone(), pc.clone(), p.clone(), false)?).clone();
                    ts_1 = (parenthesize((ts.clone()).clone(), pt.clone(), p.clone(), false)?).clone();
                    fs_1 = (parenthesize((fs.clone()).clone(), pf.clone(), p.clone(), false)?).clone();
                    r#str = stringAppendList(list![(literal!("if ")).clone(), (cs_1.clone()).clone(), (literal!(" then ")).clone(), (ts_1.clone()).clone(), (literal!(" else ")).clone(), (fs_1.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { .. }, _, Some(pcallfunc)) => {
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s_2 = pcallfunc(e.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone())?;
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: args, path: fcn, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argstr: ArcStr = arcstr::literal!("");
                    fs = (AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(fcn.clone()), (literal!(".")).clone(), true, false)?).clone();
                    argstr = stringDelimitList(List::map3(args.clone(), (std::sync::Arc::new(printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone()), (literal!(",")).clone());
                    s = stringAppendList(list![(fs.clone()).clone(), (literal!("(")).clone(), (argstr.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { expList: args, path: fcn, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argstr: ArcStr = arcstr::literal!("");
                    fs = (AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(fcn.clone()), (literal!(".")).clone(), true, false)?).clone();
                    argstr = stringDelimitList(List::map3(args.clone(), (std::sync::Arc::new(printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone()), (literal!(",")).clone());
                    s = stringAppendList(list![(literal!("function ")).clone(), (fs.clone()).clone(), (literal!("(")).clone(), (argstr.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: es, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = stringDelimitList(List::map3(es.clone(), (std::sync::Arc::new(printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone()), (literal!(",")).clone());
                    s = stringAppendList(list![(literal!("{")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: es }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = stringDelimitList(List::map3(es.clone(), (std::sync::Arc::new(printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone()), (literal!(",")).clone());
                    s = stringAppendList(list![(literal!("(")).clone(), (s.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: lstes, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = stringDelimitList(List::map1(lstes.clone(), (std::sync::Arc::new(fnptr!(printRowStr, Arc<metamodelica::List<Arc<DAE::Exp>>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, ArcStr) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone()), (literal!("},{")).clone());
                    s = stringAppendList(list![(literal!("{{")).clone(), (s.clone()).clone(), (literal!("}}")).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { ty: _, start, step: None, stop }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s3_1: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut pstop: i32 = 0;
                    let mut pstart: i32 = 0;
                    s1 = (printExp2Str(start.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s3 = (printExp2Str(stop.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    p = expPriority(e.clone());
                    pstart = expPriority(start.clone());
                    pstop = expPriority(stop.clone());
                    s1_1 = (parenthesize((s1.clone()).clone(), pstart.clone(), p.clone(), false)?).clone();
                    s3_1 = (parenthesize((s3.clone()).clone(), pstop.clone(), p.clone(), false)?).clone();
                    s = stringAppendList(list![(s1_1.clone()).clone(), (literal!(":")).clone(), (s3_1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { ty: _, start, step: Some(step), stop }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s2_1: ArcStr = arcstr::literal!("");
                    let mut s3_1: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut pstop: i32 = 0;
                    let mut pstart: i32 = 0;
                    let mut pstep: i32 = 0;
                    s1 = (printExp2Str(start.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s2 = (printExp2Str(step.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s3 = (printExp2Str(stop.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    p = expPriority(e.clone());
                    pstart = expPriority(start.clone());
                    pstop = expPriority(stop.clone());
                    pstep = expPriority(step.clone());
                    s1_1 = (parenthesize((s1.clone()).clone(), pstart.clone(), p.clone(), false)?).clone();
                    s3_1 = (parenthesize((s3.clone()).clone(), pstop.clone(), p.clone(), false)?).clone();
                    s2_1 = (parenthesize((s2.clone()).clone(), pstep.clone(), p.clone(), false)?).clone();
                    s = stringAppendList(list![(s1_1.clone()).clone(), (literal!(":")).clone(), (s2_1.clone()).clone(), (literal!(":")).clone(), (s3_1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: e, ty: tp }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (TypesDump::unparseType(tp.clone())?).clone();
                    s = (printExp2Str(e.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    res = stringAppendList(list![(literal!("DAE.CAST(")).clone(), (r#str.clone()).clone(), (literal!(", ")).clone(), (s.clone()).clone(), (literal!(")")).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ASUB { sub: subs, exp: e1 }, _, _) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s_4: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut pe1: i32 = 0;
                    let mut p: i32 = 0;
                    let mut aexpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    aexpl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    p = expPriority(e.clone());
                    pe1 = expPriority(e1.clone());
                    s1 = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s1_1 = (parenthesize((s1.clone()).clone(), pe1.clone(), p.clone(), false)?).clone();
                    s4 = stringDelimitList(List::map3(aexpl.clone(), (std::sync::Arc::new(printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone()), (literal!(",")).clone());
                    s_4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1_1.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    Ok(s_4.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { sz: Some(dim), exp: cr }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut crstr: ArcStr = arcstr::literal!("");
                    let mut dimstr: ArcStr = arcstr::literal!("");
                    crstr = (printExp2Str(cr.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    dimstr = (printExp2Str(dim.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("size(")).clone(), (crstr.clone()).clone(), (literal!(",")).clone(), (dimstr.clone()).clone(), (literal!(")")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { sz: None, exp: cr }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut crstr: ArcStr = arcstr::literal!("");
                    crstr = (printExp2Str(cr.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("size(")).clone(), (crstr.clone()).clone(), (literal!(")")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { iterators: riters, expr: exp, reductionInfo: Deref @ DAE::ReductionInfo { path: fcn, .. } }, _, _) => {
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut expstr: ArcStr = arcstr::literal!("");
                    let mut iterstr: ArcStr = arcstr::literal!("");
                    fs = AbsynUtil::pathStringNoQual(fcn.clone(), (literal!(".")).clone(), true, false)?;
                    expstr = (printExp2Str(exp.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    iterstr = stringDelimitList(List::map(riters.clone(), (std::sync::Arc::new(reductionIteratorStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
                    r#str = stringAppendList(list![(literal!("<reduction>")).clone(), (fs.clone()).clone(), (literal!("(")).clone(), (expstr.clone()).clone(), (literal!(" for ")).clone(), (iterstr.clone()).clone(), (literal!(")")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_TUPLE { listExp: es }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tuple")); __mm_s.push_str(&*printExp2Str(Arc::new(DAE::Exp::TUPLE { PR: es.clone() }), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LIST { valList: es }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = stringDelimitList(List::map3(es.clone(), (std::sync::Arc::new(printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone()), (literal!(",")).clone());
                    s = stringAppendList(list![(literal!("List(")).clone(), (s.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CONS { cdr: e2, car: e1 }, _, _) => {
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s1 = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s2 = (printExp2Str(e2.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s_2 = stringAppendList(list![(literal!("listCons(")).clone(), (s1.clone()).clone(), (literal!(",")).clone(), (s2.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_OPTION { exp: None }, _, _) => {
                    Ok(literal!("NONE()"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_OPTION { exp: Some(e1) }, _, _) => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s_1 = stringAppendList(list![(literal!("SOME(")).clone(), (s1.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BOX { exp: e1 }, _, _) => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s_1 = stringAppendList(list![(literal!("#(")).clone(), (s1.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { exp: e1, ty: _ }, _, _) => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = (printExp2Str(e1.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s_1 = stringAppendList(list![(literal!("unbox(")).clone(), (s1.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::METARECORDCALL { args, path: fcn, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argstr: ArcStr = arcstr::literal!("");
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    argstr = stringDelimitList(List::map3(args.clone(), (std::sync::Arc::new(printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone()), (literal!(",")).clone());
                    s = stringAppendList(list![(fs.clone()).clone(), (literal!("(")).clone(), (argstr.clone()).clone(), (literal!(")")).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATCHEXPRESSION { cases, inputs: es, matchType: matchTy, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    s1 = (printMatchType(matchTy.clone())?).clone();
                    s2 = (printExp2Str(Arc::new(DAE::Exp::TUPLE { PR: es.clone() }), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?).clone();
                    s3 = stringAppendList(List::map(cases.clone(), (std::sync::Arc::new(printCase2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::MatchCase>) -> Result<ArcStr> + 'static>)));
                    s = stringAppendList(list![(s1.clone()).clone(), (s2.clone()).clone(), (literal!("\n")).clone(), (s3.clone()).clone(), (literal!("  end ")).clone(), (s1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SHARED_LITERAL { exp: e, .. }, _, _) => {
                    Ok(printExp2Str(e.clone(), (stringDelimiter.clone()).clone(), opcreffunc.clone(), opcallfunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PATTERN { pattern: pat }, _, _) => {
                    Ok(Patternm::patternStr(pat.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CODE { code, .. }, _, _) => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$Code(")); __mm_s.push_str(&*Dump::printCodeStr(code.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(printExpTypeStr(inExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn printExpTypeStr(mut inExp: Arc<DAE::Exp>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: _ } => literal!("ICONST"),
        Deref @ DAE::Exp::RCONST { real: _ } => literal!("RCONST"),
        Deref @ DAE::Exp::SCONST { string: _ } => literal!("SCONST"),
        Deref @ DAE::Exp::BCONST { bool: _ } => literal!("BCONST"),
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => literal!("ENUM_LITERAL"),
        Deref @ DAE::Exp::CREF { .. } => literal!("CREF"),
        Deref @ DAE::Exp::BINARY { .. } => literal!("BINARY"),
        Deref @ DAE::Exp::UNARY { .. } => literal!("UNARY"),
        Deref @ DAE::Exp::LBINARY { .. } => literal!("LBINARY"),
        Deref @ DAE::Exp::LUNARY { .. } => literal!("LUNARY"),
        Deref @ DAE::Exp::RELATION { .. } => literal!("RELATION"),
        Deref @ DAE::Exp::IFEXP { .. } => literal!("IFEXP"),
        Deref @ DAE::Exp::CALL { .. } => literal!("CALL"),
        Deref @ DAE::Exp::PARTEVALFUNCTION { .. } => literal!("PARTEVALFUNCTION"),
        Deref @ DAE::Exp::ARRAY { .. } => literal!("ARRAY"),
        Deref @ DAE::Exp::MATRIX { .. } => literal!("MATRIX"),
        Deref @ DAE::Exp::RANGE { .. } => literal!("RANGE"),
        Deref @ DAE::Exp::TUPLE { .. } => literal!("TUPLE"),
        Deref @ DAE::Exp::CAST { .. } => literal!("CAST"),
        Deref @ DAE::Exp::ASUB { .. } => literal!("ASUB"),
        Deref @ DAE::Exp::TSUB { .. } => literal!("TSUB"),
        Deref @ DAE::Exp::SIZE { .. } => literal!("SIZE"),
        Deref @ DAE::Exp::CODE { .. } => literal!("CODE"),
        Deref @ DAE::Exp::EMPTY { .. } => literal!("EMPTY"),
        Deref @ DAE::Exp::REDUCTION { .. } => literal!("REDUCTION"),
        Deref @ DAE::Exp::LIST { .. } => literal!("LIST"),
        Deref @ DAE::Exp::CONS { .. } => literal!("CAR"),
        Deref @ DAE::Exp::META_TUPLE { .. } => literal!("META_TUPLE"),
        Deref @ DAE::Exp::META_OPTION { .. } => literal!("META_OPTION"),
        Deref @ DAE::Exp::METARECORDCALL { .. } => literal!("METARECORDCALL"),
        Deref @ DAE::Exp::MATCHEXPRESSION { .. } => literal!("MATCHEXPRESSION"),
        Deref @ DAE::Exp::BOX { .. } => literal!("BOX"),
        Deref @ DAE::Exp::UNBOX { .. } => literal!("UNBOX"),
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => literal!("SHARED_LITERAL"),
        Deref @ DAE::Exp::PATTERN { .. } => literal!("PATTERN"),
        _ => literal!("#UNKNOWN EXPRESSION#"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

fn reductionIteratorStr(mut riter: Arc<DAE::ReductionIterator>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(riter.clone()) {
        Deref @ DAE::ReductionIterator { guardExp: None, exp, id, .. } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ DAE::ReductionIterator { guardExp: Some(gexp), exp, id, .. } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" guard ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(gexp.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

fn printMatchType(mut ty: DAE::MatchType) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(ty.clone()) {
        DAE::MatchType::MATCHCONTINUE => literal!("matchcontinue"),
        DAE::MatchType::MATCH { switch: None } => literal!("match"),
        DAE::MatchType::MATCH { switch: Some(_) } => literal!("match /* switch */"),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

fn printCase2Str(mut matchCase: Arc<DAE::MatchCase>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (::match_deref::match_deref! { match &(matchCase.clone()) {
        Deref @ DAE::MatchCase { result: Some(result), body: Deref @ metamodelica::List::Nil, patterns, .. } => {
            let mut resultStr: ArcStr = arcstr::literal!("");
            let mut patternsStr: ArcStr = arcstr::literal!("");
            patternsStr = (Patternm::patternStr(Arc::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() }))?).clone();
            resultStr = (ExpressionBasics::printExpStr(result.clone())?).clone();
            stringAppendList(list![(literal!("    case ")).clone(), (patternsStr.clone()).clone(), (literal!(" then ")).clone(), (resultStr.clone()).clone(), (literal!(";\n")).clone()])
        },
        Deref @ DAE::MatchCase { result: None, body: Deref @ metamodelica::List::Nil, patterns, .. } => {
            let mut patternsStr: ArcStr = arcstr::literal!("");
            patternsStr = (Patternm::patternStr(Arc::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() }))?).clone();
            stringAppendList(list![(literal!("    case ")).clone(), (patternsStr.clone()).clone(), (literal!(" then fail();\n")).clone()])
        },
        Deref @ DAE::MatchCase { result: Some(result), body, patterns, .. } => {
            let mut resultStr: ArcStr = arcstr::literal!("");
            let mut patternsStr: ArcStr = arcstr::literal!("");
            let mut bodyStr: ArcStr = arcstr::literal!("");
            patternsStr = (Patternm::patternStr(Arc::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() }))?).clone();
            resultStr = (ExpressionBasics::printExpStr(result.clone())?).clone();
            bodyStr = stringAppendList(List::map1(body.clone(), (std::sync::Arc::new(DAEDump::ppStmtStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, i32) -> Result<ArcStr> + 'static>), 8));
            stringAppendList(list![(literal!("    case ")).clone(), (patternsStr.clone()).clone(), (literal!("\n      algorithm\n")).clone(), (bodyStr.clone()).clone(), (literal!("      then ")).clone(), (resultStr.clone()).clone(), (literal!(";\n")).clone()])
        },
        Deref @ DAE::MatchCase { result: None, body, patterns, .. } => {
            let mut patternsStr: ArcStr = arcstr::literal!("");
            let mut bodyStr: ArcStr = arcstr::literal!("");
            patternsStr = (Patternm::patternStr(Arc::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() }))?).clone();
            bodyStr = stringAppendList(List::map1(body.clone(), (std::sync::Arc::new(DAEDump::ppStmtStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, i32) -> Result<ArcStr> + 'static>), 8));
            stringAppendList(list![(literal!("    case ")).clone(), (patternsStr.clone()).clone(), (literal!("\n      algorithm\n")).clone(), (bodyStr.clone()).clone(), (literal!("      then fail();\n")).clone()])
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(r#str)
}

pub fn expPriority(mut inExp: Arc<DAE::Exp>) -> i32 {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: _ } => 0,
        Deref @ DAE::Exp::RCONST { real: _ } => 0,
        Deref @ DAE::Exp::SCONST { string: _ } => 0,
        Deref @ DAE::Exp::BCONST { bool: _ } => 0,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => 0,
        Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => 0,
        Deref @ DAE::Exp::ASUB { exp: _, sub: _ } => 0,
        Deref @ DAE::Exp::CAST { ty: _, exp: _ } => 0,
        Deref @ DAE::Exp::CALL { .. } => 0,
        Deref @ DAE::Exp::PARTEVALFUNCTION { .. } => 0,
        Deref @ DAE::Exp::ARRAY { .. } => 0,
        Deref @ DAE::Exp::MATRIX { .. } => 0,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW { ty: _ }, .. } => 3,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW_ARR { ty: _ }, .. } => 3,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW_ARR2 { ty: _ }, .. } => 3,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW_SCALAR_ARRAY { ty: _ }, .. } => 3,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW_ARRAY_SCALAR { ty: _ }, .. } => 3,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { ty: _ }, .. } => 5,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV_ARR { ty: _ }, .. } => 5,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }, .. } => 5,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }, .. } => 5,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL { ty: _ }, .. } => 7,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL_ARR { ty: _ }, .. } => 7,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }, .. } => 7,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ }, .. } => 7,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ }, .. } => 7,
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, .. } => 8,
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, .. } => 8,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD { ty: _ }, .. } => 9,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD_ARR { ty: _ }, .. } => 9,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD_ARRAY_SCALAR { ty: _ }, .. } => 9,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::SUB { ty: _ }, .. } => 9,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::SUB_ARR { ty: _ }, .. } => 9,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::SUB_SCALAR_ARRAY { ty: _ }, .. } => 9,
        Deref @ DAE::Exp::RELATION { operator: DAE::Operator::LESS { ty: _ }, .. } => 11,
        Deref @ DAE::Exp::RELATION { operator: DAE::Operator::LESSEQ { ty: _ }, .. } => 11,
        Deref @ DAE::Exp::RELATION { operator: DAE::Operator::GREATER { ty: _ }, .. } => 11,
        Deref @ DAE::Exp::RELATION { operator: DAE::Operator::GREATEREQ { ty: _ }, .. } => 11,
        Deref @ DAE::Exp::RELATION { operator: DAE::Operator::EQUAL { ty: _ }, .. } => 11,
        Deref @ DAE::Exp::RELATION { operator: DAE::Operator::NEQUAL { ty: _ }, .. } => 11,
        Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, .. } => 13,
        Deref @ DAE::Exp::LBINARY { operator: DAE::Operator::AND { ty: _ }, .. } => 15,
        Deref @ DAE::Exp::LBINARY { operator: DAE::Operator::OR { ty: _ }, .. } => 17,
        Deref @ DAE::Exp::RANGE { .. } => 19,
        Deref @ DAE::Exp::IFEXP { .. } => 21,
        Deref @ DAE::Exp::TUPLE { PR: _ } => 23,
        _ => 25,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInteger
}

pub fn printRowStr(mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut stringDelimiter: ArcStr) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = stringDelimitList(List::map3(es_1.clone(), (std::sync::Arc::new(printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), None, None), (literal!(",")).clone());
    s
}

pub fn dumpExpGraphviz(mut inExp: Arc<DAE::Exp>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { integer: i } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (intString(i.clone())).clone();
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("ICONST")).clone(), labelLst: list![(s.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { real: r } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (realString(r.clone())).clone();
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("RCONST")).clone(), labelLst: list![(s.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    let mut s = (*s).clone();
                    s = (System::escapedString((s.clone()).clone(), true)).clone();
                    s = stringAppendList(list![(literal!("\"")).clone(), (s.clone()).clone(), (literal!("\"")).clone()]);
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("SCONST")).clone(), labelLst: list![(s.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: b } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (boolString(b.clone())).clone();
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("BCONST")).clone(), labelLst: list![(s.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: c, .. } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("CREF")).clone(), labelLst: list![(s.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut lt: Arc<Graphviz::Node>;
                    let mut rt: Arc<Graphviz::Node>;
                    sym = (binopSymbol(op.clone())?).clone();
                    lt = dumpExpGraphviz(e1.clone())?;
                    rt = dumpExpGraphviz(e2.clone())?;
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("BINARY")).clone(), labelLst: list![(sym.clone()).clone()], attributes: metamodelica::nil(), children: list![lt.clone(), rt.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { exp: e, operator: op } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut ct: Arc<Graphviz::Node>;
                    sym = (unaryopSymbol(op.clone())?).clone();
                    ct = dumpExpGraphviz(e.clone())?;
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("UNARY")).clone(), labelLst: list![(sym.clone()).clone()], attributes: metamodelica::nil(), children: list![ct.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut lt: Arc<Graphviz::Node>;
                    let mut rt: Arc<Graphviz::Node>;
                    sym = (lbinopSymbol(op.clone())?).clone();
                    lt = dumpExpGraphviz(e1.clone())?;
                    rt = dumpExpGraphviz(e2.clone())?;
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("LBINARY")).clone(), labelLst: list![(sym.clone()).clone()], attributes: metamodelica::nil(), children: list![lt.clone(), rt.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { exp: e, operator: op } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut ct: Arc<Graphviz::Node>;
                    sym = (lunaryopSymbol(op.clone())?).clone();
                    ct = dumpExpGraphviz(e.clone())?;
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("LUNARY")).clone(), labelLst: list![(sym.clone()).clone()], attributes: metamodelica::nil(), children: list![ct.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { exp2: e2, operator: op, exp1: e1, .. } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut lt: Arc<Graphviz::Node>;
                    let mut rt: Arc<Graphviz::Node>;
                    sym = (relopSymbol(op.clone())?).clone();
                    lt = dumpExpGraphviz(e1.clone())?;
                    rt = dumpExpGraphviz(e2.clone())?;
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("RELATION")).clone(), labelLst: list![(sym.clone()).clone()], attributes: metamodelica::nil(), children: list![lt.clone(), rt.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expElse: f, expThen: t, expCond: cond } => {
                    let mut ct: Arc<Graphviz::Node>;
                    let mut tt: Arc<Graphviz::Node>;
                    let mut ft: Arc<Graphviz::Node>;
                    ct = dumpExpGraphviz(cond.clone())?;
                    tt = dumpExpGraphviz(t.clone())?;
                    ft = dumpExpGraphviz(f.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("IFEXP")).clone(), attributes: metamodelica::nil(), children: list![ct.clone(), tt.clone(), ft.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: fcn, .. } => {
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argnodes: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    argnodes = List::map(args.clone(), (std::sync::Arc::new(dumpExpGraphviz) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Graphviz::Node>> + 'static>));
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("CALL")).clone(), labelLst: list![(fs.clone()).clone()], attributes: metamodelica::nil(), children: argnodes.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::PARTEVALFUNCTION { expList: args, .. } => {
                    let mut argnodes: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    argnodes = List::map(args.clone(), (std::sync::Arc::new(dumpExpGraphviz) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Graphviz::Node>> + 'static>));
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("PARTEVALFUNCTION")).clone(), attributes: metamodelica::nil(), children: argnodes.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: es, .. } => {
                    let mut nodes: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    nodes = List::map(es.clone(), (std::sync::Arc::new(dumpExpGraphviz) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Graphviz::Node>> + 'static>));
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("ARRAY")).clone(), attributes: metamodelica::nil(), children: nodes.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: es } => {
                    let mut nodes: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    nodes = List::map(es.clone(), (std::sync::Arc::new(dumpExpGraphviz) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Graphviz::Node>> + 'static>));
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("TUPLE")).clone(), attributes: metamodelica::nil(), children: nodes.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: lstes, .. } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = stringDelimitList(List::map1(lstes.clone(), (std::sync::Arc::new(fnptr!(printRowStr, Arc<metamodelica::List<Arc<DAE::Exp>>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, ArcStr) -> Result<ArcStr> + 'static>), (literal!("\"")).clone()), (literal!("},{")).clone());
                    s = stringAppendList(list![(literal!("{{")).clone(), (s.clone()).clone(), (literal!("}}")).clone()]);
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("MATRIX")).clone(), labelLst: list![(s.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { stop, step: None, start, .. } => {
                    let mut t1: Arc<Graphviz::Node>;
                    let mut t2: Arc<Graphviz::Node>;
                    let mut t3: Arc<Graphviz::Node>;
                    t1 = dumpExpGraphviz(start.clone())?;
                    t2 = Arc::new(Graphviz::Node::NODE { type_: (literal!(":")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() });
                    t3 = dumpExpGraphviz(stop.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("RANGE")).clone(), attributes: metamodelica::nil(), children: list![t1.clone(), t2.clone(), t3.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { stop, step: Some(step), start, .. } => {
                    let mut t1: Arc<Graphviz::Node>;
                    let mut t2: Arc<Graphviz::Node>;
                    let mut t3: Arc<Graphviz::Node>;
                    t1 = dumpExpGraphviz(start.clone())?;
                    t2 = dumpExpGraphviz(step.clone())?;
                    t3 = dumpExpGraphviz(stop.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("RANGE")).clone(), attributes: metamodelica::nil(), children: list![t1.clone(), t2.clone(), t3.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp: e, ty } => {
                    let mut tystr: ArcStr = arcstr::literal!("");
                    let mut ct: Arc<Graphviz::Node>;
                    tystr = (TypesDump::unparseType(ty.clone())?).clone();
                    ct = dumpExpGraphviz(e.clone())?;
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("CAST")).clone(), labelLst: list![(tystr.clone()).clone()], attributes: metamodelica::nil(), children: list![ct.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: Deref @ metamodelica::List::Nil }, exp: e } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut istr: ArcStr = arcstr::literal!("");
                    let mut ct: Arc<Graphviz::Node>;
                    ct = dumpExpGraphviz(e.clone())?;
                    istr = (intString(i.clone())).clone();
                    s = stringAppendList(list![(literal!("[")).clone(), (istr.clone()).clone(), (literal!("]")).clone()]);
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("ASUB")).clone(), labelLst: list![(s.clone()).clone()], attributes: metamodelica::nil(), children: list![ct.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { sz: Some(dim), exp: cr } => {
                    let mut crt: Arc<Graphviz::Node>;
                    let mut dimt: Arc<Graphviz::Node>;
                    crt = dumpExpGraphviz(cr.clone())?;
                    dimt = dumpExpGraphviz(dim.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("SIZE")).clone(), attributes: metamodelica::nil(), children: list![crt.clone(), dimt.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { sz: None, exp: cr } => {
                    let mut crt: Arc<Graphviz::Node>;
                    crt = dumpExpGraphviz(cr.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("SIZE")).clone(), attributes: metamodelica::nil(), children: list![crt.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: iterexp, .. }, tail: Deref @ metamodelica::List::Nil }, expr: exp, reductionInfo: Deref @ DAE::ReductionInfo { path: fcn, .. } } => {
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut expt: Arc<Graphviz::Node>;
                    let mut itert: Arc<Graphviz::Node>;
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    expt = dumpExpGraphviz(exp.clone())?;
                    itert = dumpExpGraphviz(iterexp.clone())?;
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("REDUCTION")).clone(), labelLst: list![(fs.clone()).clone()], attributes: metamodelica::nil(), children: list![expt.clone(), itert.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("#UNKNOWN EXPRESSION# ----eeestr ")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

pub fn dumpExpStr(mut inExp: Arc<DAE::Exp>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inExp.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: x }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    s = (intString(x.clone())).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("ICONST ")).clone(), (s.clone()).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    s = (realString(r.clone())).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("RCONST ")).clone(), (s.clone()).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: s }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s = (*s).clone();
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    s = (System::escapedString((s.clone()).clone(), true)).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("SCONST ")).clone(), (literal!("\"")).clone(), (s.clone()).clone(), (literal!("\"\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("BCONST ")).clone(), (literal!("false")).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("BCONST ")).clone(), (literal!("true")).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    s = (clockKindString(clk.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("CLKCONST ")).clone(), (s.clone()).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ENUM_LITERAL { index: i, name: fcn }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut istr: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    s = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    istr = (intString(i.clone())).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("ENUM_LITERAL ")).clone(), (s.clone()).clone(), (literal!(" [")).clone(), (istr.clone()).clone(), (literal!("]")).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty, componentRef: c }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut tpStr: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    s = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    tpStr = (TypesDump::unparseType(ty.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("CREF ")).clone(), (s.clone()).clone(), (literal!(" CREFTYPE:")).clone(), (tpStr.clone()).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut lt: ArcStr = arcstr::literal!("");
                    let mut rt: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut new_level2: i32 = 0;
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    sym = (debugBinopSymbol(op.clone())?).clone();
                    tp = Expression::r#typeof(exp.clone())?;
                    r#str = (TypesDump::unparseType(tp.clone())?).clone();
                    lt = (dumpExpStr(e1.clone(), new_level1.clone())?).clone();
                    rt = (dumpExpStr(e2.clone(), new_level2.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("BINARY ")).clone(), (sym.clone()).clone(), (literal!(" ")).clone(), (r#str.clone()).clone(), (literal!("\n")).clone(), (lt.clone()).clone(), (rt.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: e, operator: op }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    sym = (unaryopSymbol(op.clone())?).clone();
                    ct = (dumpExpStr(e.clone(), new_level1.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("expType:")); __mm_s.push_str(&*TypesDump::unparseType(Expression::r#typeof(e.clone())?)?); __mm_s.push_str(&*literal!(" optype:")); __mm_s.push_str(&*TypesDump::unparseType(Expression::typeofOp(op.clone())?)?); ArcStr::from(__mm_s) }).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("UNARY ")).clone(), (sym.clone()).clone(), (literal!(" ")).clone(), (r#str.clone()).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut lt: ArcStr = arcstr::literal!("");
                    let mut rt: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut new_level2: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    sym = (lbinopSymbol(op.clone())?).clone();
                    lt = (dumpExpStr(e1.clone(), new_level1.clone())?).clone();
                    rt = (dumpExpStr(e2.clone(), new_level2.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("LBINARY ")).clone(), (sym.clone()).clone(), (literal!("\n")).clone(), (lt.clone()).clone(), (rt.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { exp: e, operator: op }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    sym = (lunaryopSymbol(op.clone())?).clone();
                    ct = (dumpExpStr(e.clone(), new_level1.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("LUNARY ")).clone(), (sym.clone()).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp2: e2, operator: op, exp1: e1, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut lt: ArcStr = arcstr::literal!("");
                    let mut rt: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut new_level2: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    sym = (relopSymbol(op.clone())?).clone();
                    lt = (dumpExpStr(e1.clone(), new_level1.clone())?).clone();
                    rt = (dumpExpStr(e2.clone(), new_level2.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("RELATION ")).clone(), (sym.clone()).clone(), (literal!("\n")).clone(), (lt.clone()).clone(), (rt.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expElse: f, expThen: t, expCond: cond }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut tt: ArcStr = arcstr::literal!("");
                    let mut ft: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut new_level2: i32 = 0;
                    let mut new_level3: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    new_level3 = level.clone() + 1;
                    ct = (dumpExpStr(cond.clone(), new_level1.clone())?).clone();
                    tt = (dumpExpStr(t.clone(), new_level2.clone())?).clone();
                    ft = (dumpExpStr(f.clone(), new_level3.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("IFEXP ")).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (tt.clone()).clone(), (ft.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: args, path: fcn, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argnodes_1: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut argnodes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    new_level1 = level.clone() + 1;
                    argnodes = List::map1(args.clone(), (std::sync::Arc::new(dumpExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<ArcStr> + 'static>), new_level1.clone());
                    argnodes_1 = stringAppendList(argnodes.clone());
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("CALL ")).clone(), (fs.clone()).clone(), (literal!("\n")).clone(), (argnodes_1.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { expList: args, path: fcn, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argnodes_1: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut argnodes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    new_level1 = level.clone() + 1;
                    argnodes = List::map1(args.clone(), (std::sync::Arc::new(dumpExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<ArcStr> + 'static>), new_level1.clone());
                    argnodes_1 = stringAppendList(argnodes.clone());
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("CALL ")).clone(), (fs.clone()).clone(), (literal!("\n")).clone(), (argnodes_1.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: tp, scalar: b, array: es }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut nodes_1: ArcStr = arcstr::literal!("");
                    let mut tpStr: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut nodes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    nodes = List::map1(es.clone(), (std::sync::Arc::new(dumpExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<ArcStr> + 'static>), new_level1.clone());
                    nodes_1 = stringAppendList(nodes.clone());
                    s = (boolString(b.clone())).clone();
                    tpStr = (TypesDump::unparseType(tp.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("ARRAY scalar:")).clone(), (s.clone()).clone(), (literal!(" tp: ")).clone(), (tpStr.clone()).clone(), (literal!("\n")).clone(), (nodes_1.clone()).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: es }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut nodes_1: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut nodes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    nodes = List::map1(es.clone(), (std::sync::Arc::new(dumpExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<ArcStr> + 'static>), new_level1.clone());
                    nodes_1 = stringAppendList(nodes.clone());
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("TUPLE ")).clone(), (nodes_1.clone()).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: lstes, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    s = stringDelimitList(List::map1(lstes.clone(), (std::sync::Arc::new(fnptr!(printRowStr, Arc<metamodelica::List<Arc<DAE::Exp>>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, ArcStr) -> Result<ArcStr> + 'static>), (literal!("\"")).clone()), (literal!("},{")).clone());
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("MATRIX ")).clone(), (literal!("\n")).clone(), (literal!("{{")).clone(), (s.clone()).clone(), (literal!("}}")).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { stop, step: None, start, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut t1: ArcStr = arcstr::literal!("");
                    let mut t2: ArcStr = arcstr::literal!("");
                    let mut t3: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut new_level2: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    t1 = (dumpExpStr(start.clone(), new_level1.clone())?).clone();
                    t2 = (literal!(":")).clone();
                    t3 = (dumpExpStr(stop.clone(), new_level2.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("RANGE ")).clone(), (literal!("\n")).clone(), (t1.clone()).clone(), (t2.clone()).clone(), (t3.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { stop, step: Some(step), start, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut t1: ArcStr = arcstr::literal!("");
                    let mut t2: ArcStr = arcstr::literal!("");
                    let mut t3: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut new_level2: i32 = 0;
                    let mut new_level3: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    new_level3 = level.clone() + 1;
                    t1 = (dumpExpStr(start.clone(), new_level1.clone())?).clone();
                    t2 = (dumpExpStr(step.clone(), new_level2.clone())?).clone();
                    t3 = (dumpExpStr(stop.clone(), new_level3.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("RANGE ")).clone(), (literal!("\n")).clone(), (t1.clone()).clone(), (t2.clone()).clone(), (t3.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: e, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    ct = (dumpExpStr(e.clone(), new_level1.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("CAST ")).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: Deref @ metamodelica::List::Nil }, exp: e }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut istr: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    ct = (dumpExpStr(e.clone(), new_level1.clone())?).clone();
                    istr = (intString(i.clone())).clone();
                    s = stringAppendList(list![(literal!("[")).clone(), (istr.clone()).clone(), (literal!("]")).clone()]);
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("ASUB ")).clone(), (s.clone()).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: e, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    ct = (dumpExpStr(e.clone(), new_level1.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("ASUB ")).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { sz: Some(dim), exp: cr }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut crt: ArcStr = arcstr::literal!("");
                    let mut dimt: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut new_level2: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    crt = (dumpExpStr(cr.clone(), new_level1.clone())?).clone();
                    dimt = (dumpExpStr(dim.clone(), new_level2.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("SIZE ")).clone(), (literal!("\n")).clone(), (crt.clone()).clone(), (dimt.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { sz: None, exp: cr }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut crt: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    crt = (dumpExpStr(cr.clone(), new_level1.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("SIZE ")).clone(), (literal!("\n")).clone(), (crt.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: iterexp, .. }, tail: Deref @ metamodelica::List::Nil }, expr: exp, reductionInfo: Deref @ DAE::ReductionInfo { .. } }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut expt: ArcStr = arcstr::literal!("");
                    let mut itert: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut new_level2: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    expt = (dumpExpStr(exp.clone(), new_level1.clone())?).clone();
                    itert = (dumpExpStr(iterexp.clone(), new_level2.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("REDUCTION ")).clone(), (literal!("\n")).clone(), (expt.clone()).clone(), (itert.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { exps: args, path: fcn, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argnodes_1: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    let mut argnodes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    new_level1 = level.clone() + 1;
                    argnodes = List::map1(args.clone(), (std::sync::Arc::new(dumpExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<ArcStr> + 'static>), new_level1.clone());
                    argnodes_1 = stringAppendList(argnodes.clone());
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("RECORD ")).clone(), (fs.clone()).clone(), (literal!("\n")).clone(), (argnodes_1.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RSUB { ty: tp, fieldName: fs, ix: i, exp: e }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut istr: ArcStr = arcstr::literal!("");
                    let mut tpStr: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    ct = (dumpExpStr(e.clone(), new_level1.clone())?).clone();
                    istr = (intString(i.clone())).clone();
                    s = stringAppendList(list![(literal!("[")).clone(), (istr.clone()).clone(), (literal!("]")).clone()]);
                    tpStr = (TypesDump::unparseType(tp.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("RSUB ")).clone(), (s.clone()).clone(), (literal!(" fieldName: ")).clone(), (fs.clone()).clone(), (literal!(" tp: ")).clone(), (tpStr.clone()).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BOX { exp: e }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    ct = (dumpExpStr(e.clone(), new_level1.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("BOX ")).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { exp: e, .. }, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    let mut ct: ArcStr = arcstr::literal!("");
                    let mut new_level1: i32 = 0;
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    new_level1 = level.clone() + 1;
                    ct = (dumpExpStr(e.clone(), new_level1.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), (literal!("UNBOX ")).clone(), (literal!("\n")).clone(), (ct.clone()).clone(), (literal!("")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, level) => {
                    let mut gen_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    gen_str = (genStringNTime((literal!("   |")).clone(), level.clone())?).clone();
                    res_str = stringAppendList(list![(gen_str.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" UNKNOWN EXPRESSION (")); __mm_s.push_str(&*printExpTypeStr(inExp.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone(), (literal!("\n")).clone()]);
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn genStringNTime(mut inString: ArcStr, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inString.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, 0) = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!(""))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r#str, mut level) = __mc_input.clone() else { bail!("nomatch") };
            let mut new_str: ArcStr = arcstr::literal!("");
            let mut res_str: ArcStr = arcstr::literal!("");
            let mut new_level: i32 = 0;
            new_level = level.clone() + -1;
            new_str = (genStringNTime((r#str.clone()).clone(), new_level.clone())?).clone();
            res_str = (stringAppend((r#str.clone()).clone(), (new_str.clone()).clone())).clone();
            Ok(res_str.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn dumpExp(mut exp: Arc<DAE::Exp>) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (dumpExpStr(exp.clone(), 0)?).clone();
    println!("{}", (r#str.clone()).clone());
    println!("{}", (literal!("--------------------\n")).clone());
    Ok(())
}

fn printExpIfDiff(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(" =!= ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }}).clone();
    Ok(s)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn printArraySizes(mut inLst: Arc<metamodelica::List<Option<i32>>>) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ('mc: {
        let __mc_input = inLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Some(x), tail: lst } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s = (printArraySizes(lst.clone())?).clone();
                    s2 = (intString(x.clone())).clone();
                    s = stringAppendList(list![(s2.clone()).clone(), (s.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: lst } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (printArraySizes(lst.clone())?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(out)
}

pub fn typeOfString(mut inExp: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Expression::r#typeof(inExp.clone())?;
    r#str = (TypesDump::unparseType(ty.clone())?).clone();
    Ok(r#str)
}

pub fn debugPrintComponentRefExp(mut inExp: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ } => {
                    Ok(ComponentReference::debugPrintComponentRefTypeStr(cr.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringAppendList(List::map(expl.clone(), (std::sync::Arc::new(debugPrintComponentRefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)))); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
                    Ok(s1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ExpressionBasics::printExpStr(inExp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

pub fn dimensionIntString(mut dim: Arc<DAE::Dimension>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_UNKNOWN => {
            literal!(":")
        },
        Deref @ DAE::Dimension::DIM_ENUM { size, .. } => {
            intString(size.clone())
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN => {
            literal!("1")
        },
        Deref @ DAE::Dimension::DIM_INTEGER { integer: x } => {
            intString(x.clone())
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: e } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn dumpExpWithTitle(mut title: ArcStr, mut exp: Arc<DAE::Exp>) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (dumpExpStr(exp.clone(), 0)?).clone();
    println!("{}", (title.clone()).clone());
    println!("{}", (r#str.clone()).clone());
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn printSubscript(mut inSubscript: Arc<DAE::Subscript>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ DAE::Subscript::WHOLEDIM => {
            Print::printBuf((literal!(":")).clone())?;
            ()
        },
        Deref @ DAE::Subscript::INDEX { exp: e1 } => {
            printExp(e1.clone())?;
            ()
        },
        Deref @ DAE::Subscript::SLICE { exp: e1 } => {
            printExp(e1.clone())?;
            ()
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e1 } => {
            Print::printBuf((literal!("1:")).clone())?;
            printExp(e1.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printExp(mut e: Arc<DAE::Exp>) -> Result<()> {
    Tpl::tplPrint2((std::sync::Arc::new(ExpressionDumpTpl::dumpExp) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<DAE::Exp>, ArcStr) -> Result<Tpl::Text> + 'static>), e.clone(), (literal!("\"")).clone())?;
    Ok(())
}

pub fn parenthesize(mut inString1: ArcStr, mut inInteger2: i32, mut inInteger3: i32, mut rightOpParenthesis: bool) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inString1.clone(), inInteger2.clone(), inInteger3.clone(), rightOpParenthesis.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r#str, mut pparent, mut pexpr, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut str_1: ArcStr = arcstr::literal!("");
            let true = (pparent.clone() > pexpr.clone()) else { bail!("pattern mismatch") };
            str_1 = stringAppendList(list![(literal!("(")).clone(), (r#str.clone()).clone(), (literal!(")")).clone()]);
            Ok(str_1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r#str, mut pparent, mut pexpr, true) = __mc_input.clone() else { bail!("nomatch") };
            let mut str_1: ArcStr = arcstr::literal!("");
            let true = (pparent.clone() == pexpr.clone()) else { bail!("pattern mismatch") };
            str_1 = stringAppendList(list![(literal!("(")).clone(), (r#str.clone()).clone(), (literal!(")")).clone()]);
            Ok(str_1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r#str, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok(r#str.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn clockKindString(mut inClockKind: Arc<DAE::ClockKind>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClockKind.clone()) {
        Deref @ DAE::ClockKind::INFERRED_CLOCK => {
            literal!("Clock()")
        },
        Deref @ DAE::ClockKind::RATIONAL_CLOCK { resolution, intervalCounter } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Clock(")); __mm_s.push_str(&*dumpExpStr(intervalCounter.clone(), 0)?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*dumpExpStr(resolution.clone(), 0)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::ClockKind::REAL_CLOCK { interval } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Clock(")); __mm_s.push_str(&*dumpExpStr(interval.clone(), 0)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::ClockKind::EVENT_CLOCK { startInterval, condition } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Clock(")); __mm_s.push_str(&*dumpExpStr(condition.clone(), 0)?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*dumpExpStr(startInterval.clone(), 0)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::ClockKind::SOLVER_CLOCK { solverMethod, c } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Clock(")); __mm_s.push_str(&*dumpExpStr(c.clone(), 0)?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*dumpExpStr(solverMethod.clone(), 0)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn constraintDTtoString(mut con: Arc<DAE::Constraint>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut c: Arc<DAE::Exp>;
    let mut localCon: bool = false;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(con.clone()) {
        Deref @ DAE::Constraint::CONSTRAINT_DT { localCon: __pa0, constraint: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    localCon = __pa0.clone();
    c = __pa1.clone();
    r#str = (ExpressionBasics::printExpStr(c.clone())?).clone();
    r#str = (if (localCon.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (local)")); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (global)")); ArcStr::from(__mm_s) }}).clone();
    Ok(r#str)
}

pub fn constraintDTlistToString(mut cons: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut delim: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    let mut con: Arc<DAE::Constraint>;
    for mut con in &*cons.clone() {
        let mut con = con.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*delim.clone()); __mm_s.push_str(&*constraintDTtoString(con.clone())?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

