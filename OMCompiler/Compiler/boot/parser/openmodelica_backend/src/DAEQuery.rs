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
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub const matlabStringDelim: &'static str = "'";

pub fn writeAdjacencyMatrix(mut dlow: Arc<BackendDAE::BackendDAE>, mut fileNamePrefix: ArcStr, mut flatModelicaStr: ArcStr) -> Result<ArcStr> {
    let mut fileName: ArcStr = arcstr::literal!("");
    fileName = ((::match_deref::match_deref! { match &((dlow.clone(), fileNamePrefix.clone(), flatModelicaStr.clone())) {
        (_, _, flatStr) => {
            let mut file: ArcStr = arcstr::literal!("");
            let mut strIMatrix: ArcStr = arcstr::literal!("");
            let mut strVariables: ArcStr = arcstr::literal!("");
            let mut strEquations: ArcStr = arcstr::literal!("");
            let mut m: metamodelica::Array<Arc<metamodelica::List<ArcStr>>>;
            file = (stringAppend((fileNamePrefix.clone()).clone(), (literal!("_imatrix.m")).clone())).clone();
            m = adjacencyMatrix(dlow.clone())?;
            strIMatrix = (getAdjacencyMatrix(m.clone())?).clone();
            strVariables = (getVariables(dlow.clone())?).clone();
            strEquations = (getEquations(dlow.clone())?).clone();
            strIMatrix = stringAppendList(list![(strIMatrix.clone()).clone(), (literal!("\n")).clone(), (strVariables.clone()).clone(), (literal!("\n\n\n")).clone(), (strEquations.clone()).clone(), (literal!("\n\n\n")).clone(), (flatStr.clone()).clone()]);
            System::writeFile((file.clone()).clone(), (strIMatrix.clone()).clone())?;
            file.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(fileName)
}

pub fn getEquations(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<ArcStr> {
    let mut strEqs: ArcStr = arcstr::literal!("");
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut ls1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
        Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: _, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    syst = __pa0.clone();
    ls1 = List::map(BackendEquation::equationList(syst.orderedEqs.clone()), (std::sync::Arc::new(equationStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>));
    strEqs = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EqStr = {")); __mm_s.push_str(&*stringDelimitList(ls1.clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!("};")); ArcStr::from(__mm_s) }).clone();
    Ok(strEqs)
}

pub fn equationStr(mut inEquation: Arc<BackendDAE::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!(";'")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!(";'")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!(";'")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!(";'")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e2, left: e1, .. }, tail: Deref @ metamodelica::List::Nil }, condition, .. }, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            s3 = (ExpressionBasics::printExpStr(condition.clone())?).clone();
            res = stringAppendList(list![(literal!("'when ")).clone(), (s3.clone()).clone(), (literal!(" then ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!("; end when;'")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1.clone()).clone(), (literal!("= 0")).clone(), (literal!(";'")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = stringAppendList(list![(literal!("Algorithm\n")).clone()]);
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<ArcStr>>>) -> Result<ArcStr> {
    let mut strIMatrix: ArcStr = arcstr::literal!("");
    let mut mlen: i32 = 0;
    let mut mlen_str: ArcStr = arcstr::literal!("");
    let mut m_1: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut mstr: ArcStr = arcstr::literal!("");
    mlen = (m.clone().borrow().len() as i32);
    mlen_str = (intString(mlen.clone())).clone();
    m_1 = Arc::new(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    mstr = (getAdjacencyMatrix2(m_1.clone(), 1)?).clone();
    strIMatrix = stringAppendList(list![(literal!("% Adjacency Matrix\n")).clone(), (literal!("% ====================================\n")).clone(), (literal!("% number of rows: ")).clone(), (mlen_str.clone()).clone(), (literal!("\n")).clone(), (literal!("IM={")).clone(), (mstr.clone()).clone(), (literal!("};")).clone()]);
    Ok(strIMatrix)
}

fn getAdjacencyMatrix2(mut inStringLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut rowIndex: i32) -> Result<ArcStr> {
    let mut strIMatrix: ArcStr = arcstr::literal!("");
    strIMatrix = ((::match_deref::match_deref! { match &((inStringLstLst.clone(), rowIndex.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            literal!("")
        },
        (Deref @ metamodelica::List::Cons { head: row, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut str1: ArcStr = arcstr::literal!("");
            str1 = (getAdjacencyRow(row.clone())).clone();
            r#str = stringAppendList(list![(literal!("{")).clone(), (str1.clone()).clone(), (literal!("}")).clone()]);
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: row, tail: rows }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            str1 = (getAdjacencyRow(row.clone())).clone();
            str2 = (getAdjacencyMatrix2(rows.clone(), rowIndex.clone() + 1)?).clone();
            r#str = stringAppendList(list![(literal!("{")).clone(), (str1.clone()).clone(), (literal!("},")).clone(), (str2.clone()).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(strIMatrix)
}

fn getAdjacencyRow(mut inStringLst: Arc<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut strRow: ArcStr = arcstr::literal!("");
    strRow = ((::match_deref::match_deref! { match &(inStringLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Nil } => {
            x.clone()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: xs } => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s2 = (getAdjacencyRow(xs.clone())).clone();
            s = stringAppendList(list![(x.clone()).clone(), (literal!(",")).clone(), (s2.clone()).clone()]);
            s.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    strRow
}

pub fn getVariables(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<ArcStr> {
    let mut strVars: ArcStr = arcstr::literal!("");
    strVars = ((::match_deref::match_deref! { match &(inBackendDAE.clone()) {
        Deref @ DAE { eqs: metamodelica::List::Cons { head: BackendDAE::EqSystem { orderedVars: vars1, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            vars = BackendVariable::varList(vars1.clone())?;
            s = (dumpVars(vars.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("VL = {")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("};")); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(strVars)
}

pub fn dumpVars(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut strVars: ArcStr = arcstr::literal!("");
    strVars = (dumpVars2(vars.clone(), 1)?).clone();
    Ok(strVars)
}

fn dumpVars2(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut strVars: ArcStr = arcstr::literal!("");
    strVars = ('mc: {
        let __mc_input = (inVarLst.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr, .. }, tail: Deref @ metamodelica::List::Nil }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    str1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("'")).clone(), (str1.clone()).clone(), (literal!("'")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr, .. }, tail: xs }, varno) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut varno_1: i32 = 0;
                    str1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    varno_1 = varno.clone() + 1;
                    str2 = (dumpVars2(xs.clone(), varno_1.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("'")).clone(), (str1.clone()).clone(), (literal!("',")).clone(), (str2.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(strVars)
}

pub fn adjacencyMatrix(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<metamodelica::Array<Arc<metamodelica::List<ArcStr>>>> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<ArcStr>>>;
    outAdjacencyMatrix = 'mc: {
        let __mc_input = inBackendDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE { eqs: metamodelica::List::Cons { head: BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut eqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut lstlst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut arr: metamodelica::Array<Arc<metamodelica::List<ArcStr>>>;
                    eqnsl = BackendEquation::equationList(eqns.clone());
                    lstlst = adjacencyMatrix2(vars.clone(), eqnsl.clone())?;
                    arr = metamodelica::arrayFromVec(lstlst.clone().into_iter().cloned().collect());
                    Ok(arr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("DAEQuery.adjacencyMatrix failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAdjacencyMatrix)
}

fn adjacencyMatrix2(mut inVariables: BackendDAE::Variables, mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outStringLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    outStringLstLst = 'mc: {
        let __mc_input = (inVariables.clone(), inEquationLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ metamodelica::List::Cons { head: e, tail: eqns }) => {
                    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut row: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lst = adjacencyMatrix2(vars.clone(), eqns.clone())?;
                    row = adjacencyRow(vars.clone(), e.clone())?;
                    Ok(cons(row.clone(), lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    println!("{}", (literal!("adjacency_matrix2 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLstLst)
}

fn adjacencyRow(mut inVariables: BackendDAE::Variables, mut inEquation: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outIntegerLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outIntegerLst = 'mc: {
        let __mc_input = (inVariables.clone(), inEquation.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    lst2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    lst2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    lst2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, componentRef: cr, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lst1 = adjacencyRowExp(Expression::crefExp(cr.clone())?, vars.clone())?;
                    lst2 = adjacencyRowExp(e.clone(), vars.clone())?;
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, componentRef: cr, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lst1 = adjacencyRowExp(Expression::crefExp(cr.clone())?, vars.clone())?;
                    lst2 = adjacencyRowExp(e.clone(), vars.clone())?;
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }) => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = adjacencyRowExp(e.clone(), vars.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: we, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    (cr, e2) = BackendEquation::getWhenEquationExpr(we.clone())?;
                    e1 = Expression::crefExp(cr.clone())?;
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    lst2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::ALGORITHM { alg, .. }) => {
                    let mut res_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut lstres: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    expl = Algorithm::getAllExps(alg.clone())?;
                    lstres = List::map1(expl.clone(), (std::sync::Arc::new(adjacencyRowExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone());
                    res_1 = List::flatten(lstres.clone());
                    Ok(res_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    println!("{}", (literal!("- DAEQuery.adjacencyRow failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIntegerLst)
}

// protected function adjacencyRowStmts "author: PA
//   Helper function to adjacencyRow, investigates statements for
//   variables, returning variable indexes."
//   input list<DAE.Statement> inAlgorithmStatementLst;
//   input BackendDAE.Variables inVariables;
//   output list<String> outStringLst;
// algorithm
//   outStringLst := matchcontinue (inAlgorithmStatementLst,inVariables)
//     local
//       list<String> lst1,lst2,lst3,res,lst3_1;
//       DAE.Type tp;
//       DAE.ComponentRef cr;
//       DAE.Exp e, e1;
//       list<DAE.Statement> rest,stmts;
//       BackendDAE.Variables vars;
//       list<DAE.Exp> expl;
//       DAE.Else else_;
//       list<list<String>> lstlst;
//
//     case ({},_) then {};
//
//     case ((DAE.STMT_ASSIGN(type_ = tp,exp1 = e1,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lst3 = adjacencyRowExp(e1, vars);
//         res = List.flatten({lst1,lst2,lst3});
//       then
//         res;
//
//     case ((DAE.STMT_TUPLE_ASSIGN(type_ = tp,expExpLst = expl,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lstlst = List.map1(expl, adjacencyRowExp, vars);
//         lst3_1 = List.flatten(lstlst);
//         res = List.flatten({lst1,lst2,lst3_1});
//       then
//         res;
//
//     case ((DAE.STMT_ASSIGN_ARR(type_ = tp,componentRef = cr,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lst3 = adjacencyRowExp(Expression.crefExp(cr), vars);
//         res = List.flatten({lst1,lst2,lst3});
//       then
//         res;
//
//     case ((DAE.STMT_IF(exp = e,statementLst = stmts,else_ = else_) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on IF not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_FOR(type_ = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on FOR not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_PARFOR(type_ = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on PARFOR not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_WHILE(exp = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on WHILE not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_WHEN(exp = e) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on WHEN not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_ASSERT(cond = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on ASSERT not implemented\n");
//       then
//         {};
//   end matchcontinue;
// end adjacencyRowStmts;
// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn adjacencyRowExp(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = (inExp.clone(), inVariables.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut p_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    p_1 = List::map1r(p.clone(), (std::sync::Arc::new(fnptr!(intSub, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0);
                    pStr = List::map(p_1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, exp1: e1, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    s2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: e, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    pStr = adjacencyRowExp(e.clone(), vars.clone())?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp2: e2, exp1: e1, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    s2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { exp: e, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    pStr = adjacencyRowExp(e.clone(), vars.clone())?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp2: e2, exp1: e1, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    s2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 @ Deref @ DAE::Exp::RELATION { exp2: ee2, operator: op1, .. } }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut ss: ArcStr = arcstr::literal!("");
                    let mut ss1: ArcStr = arcstr::literal!("");
                    let mut ss2: ArcStr = arcstr::literal!("");
                    let mut ss3: ArcStr = arcstr::literal!("");
                    let mut opStr: ArcStr = arcstr::literal!("");
                    opStr = (ExpressionDump::relopSymbol(op1.clone())?).clone();
                    s = (printExpStr(ee2.clone())?).clone();
                    s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    ss1 = (getAdjacencyRow(s1.clone())).clone();
                    s2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    ss2 = (getAdjacencyRow(s2.clone())).clone();
                    s3 = adjacencyRowExp(e3.clone(), vars.clone())?;
                    ss3 = (getAdjacencyRow(s3.clone())).clone();
                    ss = stringAppendList(list![(literal!("{'if', ")).clone(), (s.clone()).clone(), (literal!(",'")).clone(), (opStr.clone()).clone(), (literal!("' {")).clone(), (ss1.clone()).clone(), (literal!("}")).clone(), (literal!(",{")).clone(), (ss2.clone()).clone(), (literal!("},")).clone(), (ss3.clone()).clone(), (literal!("}")).clone()]);
                    pStr = list![(ss.clone()).clone()];
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 @ Deref @ DAE::Exp::LBINARY { .. } }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut ss: ArcStr = arcstr::literal!("");
                    let mut ss1: ArcStr = arcstr::literal!("");
                    let mut ss2: ArcStr = arcstr::literal!("");
                    let mut ss3: ArcStr = arcstr::literal!("");
                    let mut sb: ArcStr = arcstr::literal!("");
                    let _ = printExpStr(e1.clone())?;
                    sb = stringAppendList(list![(literal!("'true',")).clone(), (literal!("'=='")).clone()]);
                    s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    ss1 = (getAdjacencyRow(s1.clone())).clone();
                    s2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    ss2 = (getAdjacencyRow(s2.clone())).clone();
                    s3 = adjacencyRowExp(e3.clone(), vars.clone())?;
                    ss3 = (getAdjacencyRow(s3.clone())).clone();
                    ss = stringAppendList(list![(literal!("{'if', ")).clone(), (sb.clone()).clone(), (literal!(",")).clone(), (literal!("{")).clone(), (ss1.clone()).clone(), (literal!("}")).clone(), (literal!(",{")).clone(), (ss2.clone()).clone(), (literal!("},")).clone(), (ss3.clone()).clone(), (literal!("}")).clone()]);
                    pStr = list![(ss.clone()).clone()];
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 @ Deref @ DAE::Exp::CREF { .. } }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut ss: ArcStr = arcstr::literal!("");
                    let mut ss1: ArcStr = arcstr::literal!("");
                    let mut ss2: ArcStr = arcstr::literal!("");
                    let mut ss3: ArcStr = arcstr::literal!("");
                    let mut sb: ArcStr = arcstr::literal!("");
                    sb = stringAppendList(list![(literal!("'true',")).clone(), (literal!("'=='")).clone()]);
                    s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    ss1 = (getAdjacencyRow(s1.clone())).clone();
                    s2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    ss2 = (getAdjacencyRow(s2.clone())).clone();
                    s3 = adjacencyRowExp(e3.clone(), vars.clone())?;
                    ss3 = (getAdjacencyRow(s3.clone())).clone();
                    ss = stringAppendList(list![(literal!("{'if', ")).clone(), (sb.clone()).clone(), (literal!(" {")).clone(), (ss1.clone()).clone(), (literal!("}")).clone(), (literal!(",{")).clone(), (ss2.clone()).clone(), (literal!("},")).clone(), (ss3.clone()).clone(), (literal!("}")).clone()]);
                    pStr = list![(ss.clone()).clone()];
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut ss: ArcStr = arcstr::literal!("");
                    let mut ss1: ArcStr = arcstr::literal!("");
                    let mut ss2: ArcStr = arcstr::literal!("");
                    let mut ss3: ArcStr = arcstr::literal!("");
                    let mut sb: ArcStr = arcstr::literal!("");
                    sb = (printExpStr(e1.clone())?).clone();
                    s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    ss1 = (getAdjacencyRow(s1.clone())).clone();
                    s2 = adjacencyRowExp(e2.clone(), vars.clone())?;
                    ss2 = (getAdjacencyRow(s2.clone())).clone();
                    s3 = adjacencyRowExp(e3.clone(), vars.clone())?;
                    ss3 = (getAdjacencyRow(s3.clone())).clone();
                    ss = stringAppendList(list![(literal!("{'if', ")).clone(), (literal!("'")).clone(), (sb.clone()).clone(), (literal!("' {")).clone(), (ss1.clone()).clone(), (literal!("}")).clone(), (literal!(",{")).clone(), (ss2.clone()).clone(), (literal!("},")).clone(), (ss3.clone()).clone(), (literal!("}")).clone()]);
                    pStr = list![(ss.clone()).clone()];
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let _ = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: expl, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    lst = List::map1(expl.clone(), (std::sync::Arc::new(adjacencyRowExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone());
                    pStr = List::flatten(lst.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expl, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    lst = List::map1(expl.clone(), (std::sync::Arc::new(adjacencyRowExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone());
                    pStr = List::flatten(lst.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: explTpl, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    pStr = adjacencyRowMatrixExp(explTpl.clone(), vars.clone())?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { .. }, _) => {
                    println!("{}", (literal!("- DAEQuery.adjacency_row_exp TUPLE not impl. yet.")).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: e, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    pStr = adjacencyRowExp(e.clone(), vars.clone())?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: e, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    pStr = adjacencyRowExp(e.clone(), vars.clone())?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { iterators: iters, expr: e1, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
                    lst = List::map1(iters.clone(), (std::sync::Arc::new(adjacencyRowIter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone());
                    pStr = List::flatten(cons(s1.clone(), lst.clone()));
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

fn adjacencyRowIter(mut iter: Arc<DAE::ReductionIterator>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strs = (::match_deref::match_deref! { match &((iter.clone(), vars.clone())) {
        (Deref @ DAE::ReductionIterator { exp: e2, guardExp: Some(e1), .. }, _) => {
            let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            s1 = adjacencyRowExp(e1.clone(), vars.clone())?;
            s2 = adjacencyRowExp(e2.clone(), vars.clone())?;
            listAppend(s1.clone(), s2.clone())
        },
        (Deref @ DAE::ReductionIterator { exp: e1, .. }, _) => {
            adjacencyRowExp(e1.clone(), vars.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(strs)
}

fn adjacencyRowMatrixExp(mut inTplExpExpBooleanLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = (::match_deref::match_deref! { match &((inTplExpExpBooleanLstLst.clone(), inVariables.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: expl_1, tail: es }, vars) => {
            let mut res1: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            let mut pStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut res1_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut res2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            res1 = List::map1(expl_1.clone(), (std::sync::Arc::new(adjacencyRowExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone());
            res2 = adjacencyRowMatrixExp(es.clone(), vars.clone())?;
            res1_1 = List::flatten(res1.clone());
            pStr = listAppend(res1_1.clone(), res2.clone());
            pStr.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStringLst)
}

fn printExpStr(mut e: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (ExpressionDump::printExp2Str(e.clone(), (literal!("'")).clone(), None, None)?).clone();
    Ok(s)
}

