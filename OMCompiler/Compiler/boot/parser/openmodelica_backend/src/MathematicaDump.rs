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

use crate::BackendDAE;
use crate::BackendDump;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn dumpMmaDAEStr(mut inTuple: (BackendDAE::Variables, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ((::match_deref::match_deref! { match &(inTuple.clone()) {
        (vars, knvars, eqns, ieqns) => {
            let mut allVarStr: ArcStr = arcstr::literal!("");
            let mut s1_1: ArcStr = arcstr::literal!("");
            let mut s1_2: ArcStr = arcstr::literal!("");
            let mut s1_3: ArcStr = arcstr::literal!("");
            let mut s1_4: ArcStr = arcstr::literal!("");
            let mut s1_5: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s4: ArcStr = arcstr::literal!("");
            let mut params: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut inputs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut states: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut algs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut outputs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (states, algs, outputs, _) = printMmaVarsStr(vars.clone())?;
            (params, inputs) = printMmaParamsStr(knvars.clone())?;
            s1_1 = (Util::stringDelimitListNonEmptyElts(states.clone(), (literal!(",")).clone())).clone();
            s1_2 = (Util::stringDelimitListNonEmptyElts(algs.clone(), (literal!(",")).clone())).clone();
            s1_3 = (Util::stringDelimitListNonEmptyElts(outputs.clone(), (literal!(",")).clone())).clone();
            s1_4 = (Util::stringDelimitListNonEmptyElts(inputs.clone(), (literal!(",")).clone())).clone();
            s1_5 = (Util::stringDelimitListNonEmptyElts(params.clone(), (literal!(",")).clone())).clone();
            allVarStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*s1_1.clone()); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*s1_2.clone()); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*s1_3.clone()); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*s1_4.clone()); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*s1_5.clone()); __mm_s.push_str(&*literal!("}}")); ArcStr::from(__mm_s) }).clone();
            s3 = (printMmaEqnsStr(eqns.clone(), (vars.clone(), knvars.clone()))).clone();
            s4 = (printMmaEqnsStr(ieqns.clone(), (vars.clone(), knvars.clone()))).clone();
            res = stringAppendList(list![(literal!("{")).clone(), (allVarStr.clone()).clone(), (literal!(",")).clone(), (s3.clone()).clone(), (literal!(",")).clone(), (s4.clone()).clone(), (literal!("}")).clone()]);
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(res)
}

pub fn printMmaEqnsStr(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inTuple: (BackendDAE::Variables, BackendDAE::Variables)) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    res = ((::match_deref::match_deref! { match &((inEqns.clone(), inTuple.clone())) {
        (eqns, _) => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (Util::stringDelimitListNonEmptyElts(List::map1(eqns.clone(), Arc::new(printMmaEqnStr), inTuple.clone()), (literal!(",")).clone())).clone();
            res = stringAppendList(list![(literal!("{")).clone(), (s1.clone()).clone(), (literal!("}")).clone()]);
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    res
}

pub fn printMmaEqnStr(mut eqn: Arc<BackendDAE::Equation>, mut inTuple: (BackendDAE::Variables, BackendDAE::Variables)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &((eqn.clone(), inTuple.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, (vars, knvars)) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpMmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
            s2 = (printExpMmaStr(e2.clone(), vars.clone(), knvars.clone())?).clone();
            r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("==")).clone(), (s2.clone()).clone()]);
            r#str.clone()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. }, (vars, knvars)) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printComponentRefMmaStr(cr.clone(), vars.clone(), knvars.clone())?).clone();
            s2 = (printExpMmaStr(e2.clone(), vars.clone(), knvars.clone())?).clone();
            r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("==")).clone(), (s2.clone()).clone()]);
            r#str.clone()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }, (vars, knvars)) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpMmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
            s2 = (printExpMmaStr(e2.clone(), vars.clone(), knvars.clone())?).clone();
            r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("==")).clone(), (s2.clone()).clone()]);
            r#str.clone()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, .. }, (vars, knvars)) => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (printExpMmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
            r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("== 0")).clone()]);
            r#str.clone()
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { alg, .. }, (_, _)) => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"Algorithm\",\"")); __mm_s.push_str(&*escapeMmaString((dumpSingleAlgorithmStr(alg.clone())?).clone())?); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: whenEq, .. }, (_, _)) => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"When\",\"")); __mm_s.push_str(&*escapeMmaString((BackendDump::whenEquationString(whenEq.clone(), true)?).clone())?); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. }, (vars, knvars)) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpMmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
            s2 = (printExpMmaStr(e2.clone(), vars.clone(), knvars.clone())?).clone();
            r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("==")).clone(), (s2.clone()).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

/* Printing of equations and variables on Mathematica format*/
pub fn printExpMmaStr(mut e: Arc<DAE::Exp>, mut vars: BackendDAE::Variables, mut knvars: BackendDAE::Variables) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (printExp2MmaStr(e.clone(), vars.clone(), knvars.clone())?).clone();
    Ok(s)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn printExp2MmaStr(mut inExp: Arc<DAE::Exp>, mut vars: BackendDAE::Variables, mut knvars: BackendDAE::Variables) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inExp.clone(), vars.clone(), knvars.clone());
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
                (Deref @ DAE::Exp::RCONST { real: x }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut x2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    x2 = intReal(((x.clone()).0 as i32));
                    let true = (realEq(x2.clone(), x.clone())) else { bail!("pattern mismatch") };
                    s = (intString(((x.clone()).0 as i32))).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: x }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (realString(x.clone())).clone();
                    s = stringAppendList(list![(literal!("ToExpression[StringReplace[\"")).clone(), (s.clone()).clone(), (literal!("\",\"e\"->\"*1.0*10^\"]]")).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: s }, _, _) => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s_1 = (stringAppend((literal!("\"")).clone(), (s.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!("\"")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, _) => {
                    Ok(literal!("False"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, _, _) => {
                    Ok(literal!("True"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (printComponentRefMmaStr(cr.clone(), vars.clone(), knvars.clone())?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s2_1: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut p1: i32 = 0;
                    let mut p2: i32 = 0;
                    let mut s1: ArcStr = arcstr::literal!("");
                    sym = (ExpressionDump::binopSymbol(op.clone())?).clone();
                    s1 = (printExp2MmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
                    s2 = (printExp2MmaStr(e2.clone(), vars.clone(), knvars.clone())?).clone();
                    p = ExpressionDump::expPriority(e.clone());
                    p1 = ExpressionDump::expPriority(e1.clone());
                    p2 = ExpressionDump::expPriority(e2.clone());
                    s1_1 = (ExpressionDump::parenthesize((s1.clone()).clone(), p1.clone(), p.clone(), false)?).clone();
                    s2_1 = (ExpressionDump::parenthesize((s2.clone()).clone(), p2.clone(), p.clone(), true)?).clone();
                    s = (stringAppend((s1_1.clone()).clone(), (sym.clone()).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (s2_1.clone()).clone())).clone();
                    Ok(s_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { exp: e1, operator: op }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut p1: i32 = 0;
                    sym = (ExpressionDump::unaryopSymbol(op.clone())?).clone();
                    s = (printExp2MmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
                    p = ExpressionDump::expPriority(e.clone());
                    p1 = ExpressionDump::expPriority(e1.clone());
                    s_1 = (ExpressionDump::parenthesize((s.clone()).clone(), p1.clone(), p.clone(), true)?).clone();
                    s_2 = (stringAppend((sym.clone()).clone(), (s_1.clone()).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s2_1: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut p1: i32 = 0;
                    let mut p2: i32 = 0;
                    let mut s1: ArcStr = arcstr::literal!("");
                    sym = (lbinopSymbolMma(op.clone())?).clone();
                    s1 = (printExp2MmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
                    s2 = (printExp2MmaStr(e2.clone(), vars.clone(), knvars.clone())?).clone();
                    p = ExpressionDump::expPriority(e.clone());
                    p1 = ExpressionDump::expPriority(e1.clone());
                    p2 = ExpressionDump::expPriority(e2.clone());
                    s1_1 = (ExpressionDump::parenthesize((s1.clone()).clone(), p1.clone(), p.clone(), false)?).clone();
                    s2_1 = (ExpressionDump::parenthesize((s2.clone()).clone(), p2.clone(), p.clone(), true)?).clone();
                    s = (stringAppend((s1_1.clone()).clone(), (sym.clone()).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (s2_1.clone()).clone())).clone();
                    Ok(s_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LUNARY { exp: e1, operator: op }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut p1: i32 = 0;
                    sym = (lunaryopSymbolMma(op.clone())?).clone();
                    s = (printExp2MmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
                    p = ExpressionDump::expPriority(e.clone());
                    p1 = ExpressionDump::expPriority(e1.clone());
                    s_1 = (ExpressionDump::parenthesize((s.clone()).clone(), p1.clone(), p.clone(), true)?).clone();
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
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s2_1: ArcStr = arcstr::literal!("");
                    let mut sym: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut p1: i32 = 0;
                    let mut s1: ArcStr = arcstr::literal!("");
                    sym = (relopSymbolMma(op.clone())?).clone();
                    s1 = (printExp2MmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
                    s2 = (printExp2MmaStr(e2.clone(), vars.clone(), knvars.clone())?).clone();
                    p = ExpressionDump::expPriority(e.clone());
                    p1 = ExpressionDump::expPriority(e1.clone());
                    let _ = ExpressionDump::expPriority(e2.clone());
                    s1_1 = (ExpressionDump::parenthesize((s1.clone()).clone(), p1.clone(), p.clone(), false)?).clone();
                    s2_1 = (ExpressionDump::parenthesize((s2.clone()).clone(), p1.clone(), p.clone(), true)?).clone();
                    s = (stringAppend((s1_1.clone()).clone(), (sym.clone()).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (s2_1.clone()).clone())).clone();
                    Ok(s_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expElse: f, expThen: t, expCond: c }, _, _) => {
                    let mut ifstr: ArcStr = arcstr::literal!("");
                    let mut thenstr: ArcStr = arcstr::literal!("");
                    let mut elsestr: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    ifstr = (printExp2MmaStr(c.clone(), vars.clone(), knvars.clone())?).clone();
                    thenstr = (printExp2MmaStr(t.clone(), vars.clone(), knvars.clone())?).clone();
                    elsestr = (printExp2MmaStr(f.clone(), vars.clone(), knvars.clone())?).clone();
                    res = stringAppendList(list![(literal!("If[ ")).clone(), (ifstr.clone()).clone(), (literal!(", ")).clone(), (thenstr.clone()).clone(), (literal!(" ,")).clone(), (elsestr.clone()).clone(), (literal!("]")).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _, _) => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s_1 = (printExpMmaStr(e.clone(), vars.clone(), knvars.clone())?).clone();
                    s_2 = stringAppendList(list![(literal!("D[")).clone(), (s_1.clone()).clone(), (literal!(",\\[FormalT]]")).clone()]);
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Modelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Math", path } }, expLst, attr: call_attr }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (printExp2MmaStr(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: call_attr.clone() }), vars.clone(), knvars.clone())?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Modelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Math", path } } }, expLst, attr: call_attr }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (printExp2MmaStr(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: call_attr.clone() }), vars.clone(), knvars.clone())?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst, path: Deref @ Absyn::Path::IDENT { name: fname }, .. }, _, _) => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = (printBuiltinMmaFunc((fname.clone()).clone())?).clone();
                    s_1 = stringDelimitList(List::map2(expLst.clone(), Arc::new(printExpMmaStr), vars.clone(), knvars.clone()), (literal!(",")).clone());
                    s_2 = stringAppendList(list![(s1.clone()).clone(), (literal!("[")).clone(), (s_1.clone()).clone(), (literal!("]")).clone()]);
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, .. }, _, _) => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut s_11: ArcStr = arcstr::literal!("");
                    s_1 = (printExpMmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
                    s_11 = (printExpMmaStr(e2.clone(), vars.clone(), knvars.clone())?).clone();
                    s_2 = stringAppendList(list![(literal!("ArcTan[")).clone(), (s_1.clone()).clone(), (literal!(",")).clone(), (s_11.clone()).clone(), (literal!("]")).clone()]);
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. }, _, _) => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s_1 = (printExpMmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
                    s_2 = stringAppendList(list![(literal!("Log[")).clone(), (s_1.clone()).clone(), (literal!(",10]")).clone()]);
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: args, path: fcn, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argstr: ArcStr = arcstr::literal!("");
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    fs = (translateKnownMmaFuncs((fs.clone()).clone())?).clone();
                    argstr = stringDelimitList(List::map2(args.clone(), Arc::new(printExpMmaStr), vars.clone(), knvars.clone()), (literal!(",")).clone());
                    s = (stringAppend((fs.clone()).clone(), (literal!("[")).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (argstr.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!("]")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: args, path: fcn, .. }, _, _) => {
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argstr: ArcStr = arcstr::literal!("");
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    argstr = stringDelimitList(List::map2(args.clone(), Arc::new(printExpMmaStr), vars.clone(), knvars.clone()), (literal!(",")).clone());
                    s_2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FunctionCall[\"")); __mm_s.push_str(&*fs.clone()); __mm_s.push_str(&*literal!("\"][")); __mm_s.push_str(&*argstr.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { exps: args, path: fcn, .. }, _, _) => {
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argstr: ArcStr = arcstr::literal!("");
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    argstr = stringDelimitList(List::map2(args.clone(), Arc::new(printExpMmaStr), vars.clone(), knvars.clone()), (literal!(",")).clone());
                    s_2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FunctionCall[\"")); __mm_s.push_str(&*fs.clone()); __mm_s.push_str(&*literal!("\"][")); __mm_s.push_str(&*argstr.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: es, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s = stringDelimitList(List::map2(es.clone(), Arc::new(printExpMmaStr), vars.clone(), knvars.clone()), (literal!(",")).clone());
                    s_1 = (stringAppend((literal!("{")).clone(), (s.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!("}")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: es }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s = stringDelimitList(List::map2(es.clone(), Arc::new(printExpMmaStr), vars.clone(), knvars.clone()), (literal!(",")).clone());
                    s_1 = (stringAppend((literal!("{")).clone(), (s.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!("}")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s = stringDelimitList(List::map2(matrix.clone(), Arc::new(fnptr!(printRowMmaStr, Arc<metamodelica::List<Arc<DAE::Exp>>>, BackendDAE::Variables, BackendDAE::Variables)), vars.clone(), knvars.clone()), (literal!("},{")).clone());
                    s_1 = (stringAppend((literal!("{{")).clone(), (s.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!("}}")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { stop, step: None, start, .. }, _, _) => {
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s3_1: ArcStr = arcstr::literal!("");
                    let mut s_3: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut pstart: i32 = 0;
                    let mut pstop: i32 = 0;
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = (printExp2MmaStr(start.clone(), vars.clone(), knvars.clone())?).clone();
                    s3 = (printExp2MmaStr(stop.clone(), vars.clone(), knvars.clone())?).clone();
                    p = ExpressionDump::expPriority(e.clone());
                    pstart = ExpressionDump::expPriority(start.clone());
                    pstop = ExpressionDump::expPriority(stop.clone());
                    s1_1 = (ExpressionDump::parenthesize((s1.clone()).clone(), pstart.clone(), p.clone(), false)?).clone();
                    s3_1 = (ExpressionDump::parenthesize((s3.clone()).clone(), pstop.clone(), p.clone(), false)?).clone();
                    s_3 = stringAppendList(list![(literal!("Range[")).clone(), (s1_1.clone()).clone(), (literal!(",")).clone(), (s3_1.clone()).clone(), (literal!("]")).clone()]);
                    Ok(s_3.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { stop, step: Some(step), start, .. }, _, _) => {
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s_5: ArcStr = arcstr::literal!("");
                    s2 = (printExp2MmaStr(start.clone(), vars.clone(), knvars.clone())?).clone();
                    s3 = (printExp2MmaStr(step.clone(), vars.clone(), knvars.clone())?).clone();
                    s4 = (printExp2MmaStr(stop.clone(), vars.clone(), knvars.clone())?).clone();
                    s_5 = stringAppendList(list![(literal!("Range[")).clone(), (s2.clone()).clone(), (literal!(",")).clone(), (s4.clone()).clone(), (literal!(",")).clone(), (s3.clone()).clone(), (literal!("]")).clone()]);
                    Ok(s_5.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: Deref @ DAE::Exp::ICONST { integer: ival }, ty: Deref @ DAE::Type::T_REAL { .. } }, _, _) => {
                    let mut res: ArcStr = arcstr::literal!("");
                    res = (intString(ival.clone())).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::ICONST { integer: ival }, operator: DAE::Operator::UMINUS { .. } }, ty: Deref @ DAE::Type::T_REAL { .. } }, _, _) => {
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut res2: ArcStr = arcstr::literal!("");
                    res = (intString(ival.clone())).clone();
                    res2 = (stringAppend((literal!("-")).clone(), (res.clone()).clone())).clone();
                    Ok(res2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: e, ty: Deref @ DAE::Type::T_REAL { .. } }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (printExpMmaStr(e.clone(), vars.clone(), knvars.clone())?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ASUB { sub: subs, exp: e1 }, _, _) => {
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s_4: ArcStr = arcstr::literal!("");
                    let mut p: i32 = 0;
                    let mut pe1: i32 = 0;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ae1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    ae1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    p = ExpressionDump::expPriority(e.clone());
                    pe1 = ExpressionDump::expPriority(e1.clone());
                    s1 = (printExp2MmaStr(e1.clone(), vars.clone(), knvars.clone())?).clone();
                    s1_1 = (ExpressionDump::parenthesize((s1.clone()).clone(), pe1.clone(), p.clone(), false)?).clone();
                    s4 = stringDelimitList(List::map2(ae1.clone(), Arc::new(printExp2MmaStr), vars.clone(), knvars.clone()), (literal!(", ")).clone());
                    s_4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Index[")); __mm_s.push_str(&*s1_1.clone()); __mm_s.push_str(&*literal!(",{")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("}]")); ArcStr::from(__mm_s) }).clone();
                    Ok(s_4.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { sz: Some(dim), exp: e }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut crstr: ArcStr = arcstr::literal!("");
                    let mut dimstr: ArcStr = arcstr::literal!("");
                    crstr = (printExpMmaStr(e.clone(), vars.clone(), knvars.clone())?).clone();
                    dimstr = (printExpMmaStr(dim.clone(), vars.clone(), knvars.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("Dimensions[")).clone(), (crstr.clone()).clone(), (literal!("][[")).clone(), (dimstr.clone()).clone(), (literal!("]]")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { sz: None, exp: e }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut crstr: ArcStr = arcstr::literal!("");
                    crstr = (printExpMmaStr(e.clone(), vars.clone(), knvars.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("Dimensions[")).clone(), (crstr.clone()).clone(), (literal!("]")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: fcn, .. }, expr: exp, iterators: Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: iterexp, id, .. }, tail: _ } }, _, _) => {
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut expstr: ArcStr = arcstr::literal!("");
                    let mut iterstr: ArcStr = arcstr::literal!("");
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    expstr = (printExpMmaStr(exp.clone(), vars.clone(), knvars.clone())?).clone();
                    iterstr = (printExpMmaStr(iterexp.clone(), vars.clone(), knvars.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("Table[")).clone(), (fs.clone()).clone(), (literal!("[")).clone(), (expstr.clone()).clone(), (literal!("],{")).clone(), (id.clone()).clone(), (literal!(", ")).clone(), (iterstr.clone()).clone(), (literal!("}]")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ENUM_LITERAL { name: path, .. }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"ModelicaName\",\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"UnknownExpression\",\"")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn printComponentRefMmaStr(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables, mut knvars: BackendDAE::Variables) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ('mc: {
        let __mc_input = (cr.clone(), vars.clone(), knvars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", identType: _, subscriptLst: _ }, _, _) => {
                    Ok(literal!("\\[FormalT]"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut nameStr: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = res.clone();
                    (_, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    nameStr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$p")).clone(), (literal!(".")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$lb")).clone(), (literal!("[")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$rb")).clone(), (literal!("]")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$leftParentesis")).clone(), (literal!("[")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$rightParentesis")).clone(), (literal!("]")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("(")).clone(), (literal!("[")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!(")")).clone(), (literal!("]")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("_")).clone(), (literal!("\\[UnderBracket]")).clone())?).clone();
                    nameStr = (wrapInMember((nameStr.clone()).clone())?).clone();
                    nameStr = (addMissingForQuotedNames((nameStr.clone()).clone())?).clone();
                    res = stringAppendList(list![(nameStr.clone()).clone(), (literal!("[\\[FormalT]]")).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut nameStr: ArcStr = arcstr::literal!("");
                    let mut isInput: bool = false;
                    let mut isOutput: bool = false;
                    let mut v: BackendDAE::Var;
                    let mut res: ArcStr = res.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), knvars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    isInput = BackendVariable::isInput(v.clone());
                    isOutput = BackendVariable::isOutputVar(v.clone());
                    let true = (boolOr(isInput.clone(), isOutput.clone())) else { bail!("pattern mismatch") };
                    nameStr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$p")).clone(), (literal!(".")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$lb")).clone(), (literal!("[")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$rb")).clone(), (literal!("]")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$leftParentesis")).clone(), (literal!("(")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$rightParentesis")).clone(), (literal!(")")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("_")).clone(), (literal!("\\[UnderBracket]")).clone())?).clone();
                    nameStr = (wrapInMember((nameStr.clone()).clone())?).clone();
                    nameStr = (addMissingForQuotedNames((nameStr.clone()).clone())?).clone();
                    res = stringAppendList(list![(nameStr.clone()).clone(), (literal!("[\\[FormalT]]")).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut nameStr: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        (_, _) = unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    nameStr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$p")).clone(), (literal!(".")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$lb")).clone(), (literal!("[")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$rb")).clone(), (literal!("]")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$leftParentesis")).clone(), (literal!("(")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("$rightParentesis")).clone(), (literal!(")")).clone())?).clone();
                    nameStr = (System::stringReplace((nameStr.clone()).clone(), (literal!("_")).clone(), (literal!("\\[UnderBracket]")).clone())?).clone();
                    nameStr = (wrapInMember((nameStr.clone()).clone())?).clone();
                    Ok(nameStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(res)
}

fn wrapInMember(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    s3 = (System::stringReplace((r#str.clone()).clone(), (literal!(".")).clone(), (literal!("\\[UpPointer]")).clone())?).clone();
    outStr = (s3.clone()).clone();
    Ok(outStr)
}

fn addMissingForQuotedNames(mut name: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ('mc: {
        let __mc_input = name.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut res: ArcStr = res.clone();
            let false = (-1 == System::stringFind((name.clone()).clone(), (literal!("'")).clone())?) else { bail!("pattern mismatch") };
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"QuotedName\",\"")); __mm_s.push_str(&*System::stringReplace((name.clone()).clone(), (literal!("\\")).clone(), (literal!("\\\\")).clone())?); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) }).clone();
            Ok(res.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(name.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(res)
}

fn lbinopSymbolMma(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::AND { ty: _ } => literal!(" && "),
        DAE::Operator::OR { ty: _ } => literal!(" || "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn lunaryopSymbolMma(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::NOT { ty: _ } => literal!(" ! "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn relopSymbolMma(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::LESS { .. } => literal!(" < "),
        DAE::Operator::LESSEQ { .. } => literal!(" <= "),
        DAE::Operator::GREATER { .. } => literal!(" > "),
        DAE::Operator::GREATEREQ { .. } => literal!(" >= "),
        DAE::Operator::EQUAL { .. } => literal!(" == "),
        DAE::Operator::NEQUAL { .. } => literal!(" != "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn printBuiltinMmaFunc(mut modelicaFuncName: ArcStr) -> Result<ArcStr> {
    let mut mathematicaFuncName: ArcStr = arcstr::literal!("");
    mathematicaFuncName = ((::match_deref::match_deref! { match &(modelicaFuncName.clone()) {
        Deref @ "sqrt" => literal!("Sqrt"),
        Deref @ "abs" => literal!("Abs"),
        Deref @ "sign" => literal!("Sign"),
        Deref @ "Integer" => literal!("IntegerPart"),
        Deref @ "div" => literal!("Rational"),
        Deref @ "max" => literal!("Max"),
        Deref @ "min" => literal!("Min"),
        Deref @ "mod" => literal!("Quotient"),
        Deref @ "rem" => literal!("Mod"),
        Deref @ "ceil" => literal!("Cieling"),
        Deref @ "floor" => literal!("Floor"),
        Deref @ "integer" => literal!("IntegerPart"),
        Deref @ "sin" => literal!("Sin"),
        Deref @ "cos" => literal!("Cos"),
        Deref @ "tan" => literal!("Tan"),
        Deref @ "asin" => literal!("ArcSin"),
        Deref @ "acos" => literal!("ArcCos"),
        Deref @ "atan" => literal!("ArcTan"),
        Deref @ "sinh" => literal!("Sinh"),
        Deref @ "cosh" => literal!("Cosh"),
        Deref @ "tanh" => literal!("Tanh"),
        Deref @ "exp" => literal!("Exp"),
        Deref @ "log" => literal!("Log"),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(mathematicaFuncName)
}

fn translateKnownMmaFuncs(mut func: ArcStr) -> Result<ArcStr> {
    let mut mmaFunc: ArcStr = arcstr::literal!("");
    mmaFunc = ((::match_deref::match_deref! { match &(func.clone()) {
        Deref @ "sin" => literal!("Sin"),
        Deref @ "Modelica.Math.sin" => literal!("Sin"),
        Deref @ "cos" => literal!("Cos"),
        Deref @ "Modelica.Math.cos" => literal!("Cos"),
        Deref @ "tan" => literal!("Tan"),
        Deref @ "Modelica.Math.tan" => literal!("Tan"),
        Deref @ "exp" => literal!("Exp"),
        Deref @ "Modelica.Math.exp" => literal!("Exp"),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(mmaFunc)
}

fn printRowMmaStr(mut es: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut vars: BackendDAE::Variables, mut knvars: BackendDAE::Variables) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    s = stringDelimitList(List::map2(es.clone(), Arc::new(printExpMmaStr), vars.clone(), knvars.clone()), (literal!(",")).clone());
    s
}

fn escapeMmaString(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = (System::stringReplace((r#str.clone()).clone(), (literal!("\"")).clone(), (literal!("\\\"")).clone())?).clone();
    Ok(res)
}

fn dumpSingleAlgorithmStr(mut algs: Arc<DAE::Algorithm>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(algs.clone()) {
        Deref @ DAE::Algorithm { statementLst: stmts } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut myStream: IOStream::IOStream;
            myStream = IOStream::create((literal!("")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = DAEDump::dumpAlgorithmStream(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts.clone() }), source: DAE::emptyElementSource.clone() }), myStream.clone())?;
            r#str = (IOStream::string(myStream.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printMmaVarsStr(mut vars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut states: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut algs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outputs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut inputs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (states, algs, outputs, inputs) = (match vars.clone() {
        _ => {
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            varLst = BackendVariable::varList(vars.clone())?;
            varLst = varLst.clone().reverse();
            states = List::map2(varLst.clone(), Arc::new(printMmaVarStr), true, vars.clone());
            algs = List::map2(varLst.clone(), Arc::new(printMmaVarStr), false, vars.clone());
            outputs = List::map(varLst.clone(), Arc::new(printMmaOutputStr));
            inputs = List::map(varLst.clone(), Arc::new(printMmaInputStr));
            (states.clone(), algs.clone(), outputs.clone(), inputs.clone())
        },
    });
    Ok((states, algs, outputs, inputs))
}

pub fn printMmaVarStr(mut v: BackendDAE::Var, mut selectKind: bool, mut allVars: BackendDAE::Variables) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = (v.clone(), selectKind.clone(), allVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varName: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$dummy", identType: Deref @ DAE::Type::T_UNKNOWN, subscriptLst: Deref @ metamodelica::List::Nil }, .. }, _, _) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, varName: name, .. }, true, _) => {
                    let mut nameStr: ArcStr = arcstr::literal!("");
                    nameStr = (printComponentRefMmaStr(name.clone(), allVars.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    Ok(nameStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE, varName: name, .. }, false, _) => {
                    let mut nameStr: ArcStr = arcstr::literal!("");
                    nameStr = (printComponentRefMmaStr(name.clone(), allVars.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    Ok(nameStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER, varName: name, .. }, false, _) => {
                    let mut nameStr: ArcStr = arcstr::literal!("");
                    nameStr = (printComponentRefMmaStr(name.clone(), allVars.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    Ok(nameStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE, varName: name, .. }, false, _) => {
                    let mut nameStr: ArcStr = arcstr::literal!("");
                    nameStr = (printComponentRefMmaStr(name.clone(), allVars.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    Ok(nameStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, varName: name, .. }, false, _) => {
                    let mut nameStr: ArcStr = arcstr::literal!("");
                    nameStr = (printComponentRefMmaStr(name.clone(), allVars.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    Ok(nameStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn printMmaOutputStr(mut param: BackendDAE::Var) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = param.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v @ BackendDAE::Var { varDirection: DAE::VarDirection::OUTPUT, varName: name @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    let true = (BackendVariable::isVarOnTopLevelAndOutput(v.clone())) else { bail!("pattern mismatch") };
                    r#str = (printComponentRefMmaStr(name.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn printMmaInputStr(mut param: BackendDAE::Var) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = param.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v @ BackendDAE::Var { varDirection: DAE::VarDirection::INPUT, varName: name @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    let true = (BackendVariable::isVarOnTopLevelAndInput(v.clone())) else { bail!("pattern mismatch") };
                    r#str = (printComponentRefMmaStr(name.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

pub fn printMmaParamsStr(mut knvars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut params: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut inputs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (params, inputs) = (match knvars.clone() {
        _ => {
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            varLst = BackendVariable::varList(knvars.clone())?;
            params = List::map(varLst.clone(), Arc::new(printMmaParamStr));
            inputs = List::map(varLst.clone(), Arc::new(printMmaInputStr));
            (params.clone(), inputs.clone())
        },
    });
    Ok((params, inputs))
}

fn printMmaParamStr(mut param: BackendDAE::Var) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = param.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { bindExp: Some(exp), varKind: BackendDAE::VarKind::PARAM, varName: name, .. } => {
                    let mut expStr: ArcStr = arcstr::literal!("");
                    let mut paramStr: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = r#str.clone();
                    expStr = (printExpMmaStr(exp.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    paramStr = (printComponentRefMmaStr(name.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    r#str = stringAppendList(list![(paramStr.clone()).clone(), (literal!("->")).clone(), (expStr.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { values: val, bindExp: None, varKind: BackendDAE::VarKind::PARAM, varName: name, .. } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut expStr: ArcStr = arcstr::literal!("");
                    let mut paramStr: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = r#str.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(getStartAttribute(val.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    expStr = (printExpMmaStr(exp.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    paramStr = (printComponentRefMmaStr(name.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    r#str = stringAppendList(list![(paramStr.clone()).clone(), (literal!("->")).clone(), (expStr.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { values: val, bindExp: None, varKind: BackendDAE::VarKind::PARAM, varName: name, .. } => {
                    let mut expStr: ArcStr = arcstr::literal!("");
                    let mut paramStr: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = r#str.clone();
                    ::match_deref::match_deref! { match &(getStartAttribute(val.clone())?) {
                        None => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    expStr = (printExpMmaStr(Arc::new(DAE::Exp::ICONST { integer: 0 }), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    paramStr = (printComponentRefMmaStr(name.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    r#str = stringAppendList(list![(paramStr.clone()).clone(), (literal!("->")).clone(), (expStr.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM, varName: name, .. } => {
                    let mut paramStr: ArcStr = arcstr::literal!("");
                    paramStr = (printComponentRefMmaStr(name.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
                    Ok(paramStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn getStartAttribute(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut out: Option<Arc<DAE::Exp>> = None;
    out = 'mc: {
        let __mc_input = inVariableAttributesOption.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: e, .. }) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: e, .. }) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: e, .. }) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: e, .. }) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

