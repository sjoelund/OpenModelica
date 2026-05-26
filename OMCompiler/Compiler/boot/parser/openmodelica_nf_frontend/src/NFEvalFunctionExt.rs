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

use crate::NFCeval as Ceval;
use crate::NFEvalFunction as EvalFunction;
use crate::NFEvalFunction::assignVariable;
use crate::NFExpression as Expression;
use crate::NFType as Type;
use openmodelica_util::Lapack;

pub fn Lapack_dgeev(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut jobvl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut jobvr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldvl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldvr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut wr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut wi: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut vl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut vr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut INFO: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDVL: i32 = 0;
    let mut LDVR: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut N: i32 = 0;
    let mut JOBVL: ArcStr = arcstr::literal!("");
    let mut JOBVR: ArcStr = arcstr::literal!("");
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut VL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut VR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut WR: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut WI: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => bail!("pattern mismatch"),
    } };
    jobvl = __pa0.clone();
    jobvr = __pa1.clone();
    n = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    wr = __pa5.clone();
    wi = __pa6.clone();
    vl = __pa7.clone();
    ldvl = __pa8.clone();
    vr = __pa9.clone();
    ldvr = __pa10.clone();
    work = __pa11.clone();
    lwork = __pa12.clone();
    info = __pa13.clone();
    JOBVL = (evaluateExtStringArg(jobvl.clone())).clone();
    JOBVR = (evaluateExtStringArg(jobvr.clone())).clone();
    N = evaluateExtIntArg(n.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    LDVL = evaluateExtIntArg(ldvl.clone());
    LDVR = evaluateExtIntArg(ldvr.clone());
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (A, WR, WI, VL, VR, WORK, INFO) = Lapack::dgeev((JOBVL.clone()).clone(), (JOBVR.clone()).clone(), N.clone(), A.clone(), LDA.clone(), LDVL.clone(), LDVR.clone(), WORK.clone(), LWORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariable(wr.clone(), Expression::makeRealArray(WR.clone())?)?;
    assignVariable(wi.clone(), Expression::makeRealArray(WI.clone())?)?;
    assignVariableExt(vl.clone(), Expression::makeRealMatrix(VL.clone())?)?;
    assignVariableExt(vr.clone(), Expression::makeRealMatrix(VR.clone())?)?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgegv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut jobvl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut jobvr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut alphar: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut alphai: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut beta: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut vl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldvl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut vr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldvr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut JOBVL: ArcStr = arcstr::literal!("");
    let mut JOBVR: ArcStr = arcstr::literal!("");
    let mut N: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDB: i32 = 0;
    let mut LDVL: i32 = 0;
    let mut LDVR: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut VL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut VR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut ALPHAR: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut ALPHAI: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut BETA: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Cons { head: __pa14, tail: Deref @ metamodelica::List::Cons { head: __pa15, tail: Deref @ metamodelica::List::Cons { head: __pa16, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone()),
        _ => bail!("pattern mismatch"),
    } };
    jobvl = __pa0.clone();
    jobvr = __pa1.clone();
    n = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    alphar = __pa7.clone();
    alphai = __pa8.clone();
    beta = __pa9.clone();
    vl = __pa10.clone();
    ldvl = __pa11.clone();
    vr = __pa12.clone();
    ldvr = __pa13.clone();
    work = __pa14.clone();
    lwork = __pa15.clone();
    info = __pa16.clone();
    JOBVL = (evaluateExtStringArg(jobvl.clone())).clone();
    JOBVR = (evaluateExtStringArg(jobvr.clone())).clone();
    N = evaluateExtIntArg(n.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    LDVL = evaluateExtIntArg(ldvl.clone());
    LDVR = evaluateExtIntArg(ldvr.clone());
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (ALPHAR, ALPHAI, BETA, VL, VR, WORK, INFO) = Lapack::dgegv((JOBVL.clone()).clone(), (JOBVR.clone()).clone(), N.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), LDVL.clone(), LDVR.clone(), WORK.clone(), LWORK.clone());
    assignVariable(alphar.clone(), Expression::makeRealArray(ALPHAR.clone())?)?;
    assignVariable(alphai.clone(), Expression::makeRealArray(ALPHAI.clone())?)?;
    assignVariable(beta.clone(), Expression::makeRealArray(BETA.clone())?)?;
    assignVariableExt(vl.clone(), Expression::makeRealMatrix(VL.clone())?)?;
    assignVariableExt(vr.clone(), Expression::makeRealMatrix(VR.clone())?)?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgels(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut trans: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut m: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nrhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut TRANS: ArcStr = arcstr::literal!("");
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut NRHS: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDB: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
        _ => bail!("pattern mismatch"),
    } };
    trans = __pa0.clone();
    m = __pa1.clone();
    n = __pa2.clone();
    nrhs = __pa3.clone();
    a = __pa4.clone();
    lda = __pa5.clone();
    b = __pa6.clone();
    ldb = __pa7.clone();
    work = __pa8.clone();
    lwork = __pa9.clone();
    info = __pa10.clone();
    TRANS = (evaluateExtStringArg(trans.clone())).clone();
    M = evaluateExtIntArg(m.clone());
    N = evaluateExtIntArg(n.clone());
    NRHS = evaluateExtIntArg(nrhs.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (A, B, WORK, INFO) = Lapack::dgels((TRANS.clone()).clone(), M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), WORK.clone(), LWORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariableExt(b.clone(), Expression::makeRealMatrix(B.clone())?)?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgelsx(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nrhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut jpvt: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rcond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rank: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut NRHS: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDB: i32 = 0;
    let mut RANK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut JPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut RCOND: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    if (args.clone().len() as i32) == 12 {
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
            _ => bail!("pattern mismatch"),
        } };
        m = __pa0.clone();
        n = __pa1.clone();
        nrhs = __pa2.clone();
        a = __pa3.clone();
        lda = __pa4.clone();
        b = __pa5.clone();
        ldb = __pa6.clone();
        jpvt = __pa7.clone();
        rcond = __pa8.clone();
        rank = __pa9.clone();
        work = __pa10.clone();
        info = __pa11.clone();
    } else {
        let (__pa13, __pa14, __pa15, __pa16, __pa17, __pa18, __pa19, __pa20, __pa21, __pa22, __pa23, __pa24) = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Cons { head: __pa14, tail: Deref @ metamodelica::List::Cons { head: __pa15, tail: Deref @ metamodelica::List::Cons { head: __pa16, tail: Deref @ metamodelica::List::Cons { head: __pa17, tail: Deref @ metamodelica::List::Cons { head: __pa18, tail: Deref @ metamodelica::List::Cons { head: __pa19, tail: Deref @ metamodelica::List::Cons { head: __pa20, tail: Deref @ metamodelica::List::Cons { head: __pa21, tail: Deref @ metamodelica::List::Cons { head: __pa22, tail: Deref @ metamodelica::List::Cons { head: __pa23, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa24, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } => (__pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone(), __pa22.clone(), __pa23.clone(), __pa24.clone()),
            _ => bail!("pattern mismatch"),
        } };
        m = __pa13.clone();
        n = __pa14.clone();
        nrhs = __pa15.clone();
        a = __pa16.clone();
        lda = __pa17.clone();
        b = __pa18.clone();
        ldb = __pa19.clone();
        jpvt = __pa20.clone();
        rcond = __pa21.clone();
        rank = __pa22.clone();
        work = __pa23.clone();
        info = __pa24.clone();
    }
    M = evaluateExtIntArg(m.clone());
    N = evaluateExtIntArg(n.clone());
    NRHS = evaluateExtIntArg(nrhs.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    RCOND = evaluateExtRealArg(rcond.clone());
    WORK = evaluateExtRealArrayArg(work.clone())?;
    (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), JPVT.clone(), RCOND.clone(), WORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariableExt(b.clone(), Expression::makeRealMatrix(B.clone())?)?;
    assignVariable(jpvt.clone(), Expression::makeIntegerArray(JPVT.clone())?)?;
    assignVariable(rank.clone(), Expression::makeInteger(RANK.clone()))?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgelsy(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nrhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut jpvt: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rcond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rank: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut NRHS: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDB: i32 = 0;
    let mut RANK: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut JPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut RCOND: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    nrhs = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    jpvt = __pa7.clone();
    rcond = __pa8.clone();
    rank = __pa9.clone();
    work = __pa10.clone();
    lwork = __pa11.clone();
    info = __pa12.clone();
    M = evaluateExtIntArg(m.clone());
    N = evaluateExtIntArg(n.clone());
    NRHS = evaluateExtIntArg(nrhs.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    RCOND = evaluateExtRealArg(rcond.clone());
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (A, B, JPVT, RANK, WORK, INFO) = Lapack::dgelsy(M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), JPVT.clone(), RCOND.clone(), WORK.clone(), LWORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariableExt(b.clone(), Expression::makeRealMatrix(B.clone())?)?;
    assignVariable(jpvt.clone(), Expression::makeIntegerArray(JPVT.clone())?)?;
    assignVariable(rank.clone(), Expression::makeInteger(RANK.clone()))?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgesv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nrhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ipiv: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut N: i32 = 0;
    let mut NRHS: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDB: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    nrhs = __pa1.clone();
    a = __pa2.clone();
    lda = __pa3.clone();
    ipiv = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    info = __pa7.clone();
    N = evaluateExtIntArg(n.clone());
    NRHS = evaluateExtIntArg(nrhs.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    (A, IPIV, B, INFO) = Lapack::dgesv(N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariable(ipiv.clone(), Expression::makeIntegerArray(IPIV.clone())?)?;
    assignVariableExt(b.clone(), Expression::makeRealMatrix(B.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgglse(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut p: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut d: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut P: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDB: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut C: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut D: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut X: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    p = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    c = __pa7.clone();
    d = __pa8.clone();
    x = __pa9.clone();
    work = __pa10.clone();
    lwork = __pa11.clone();
    info = __pa12.clone();
    M = evaluateExtIntArg(m.clone());
    N = evaluateExtIntArg(n.clone());
    P = evaluateExtIntArg(p.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    C = evaluateExtRealArrayArg(c.clone())?;
    D = evaluateExtRealArrayArg(d.clone())?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (A, B, C, D, X, WORK, INFO) = Lapack::dgglse(M.clone(), N.clone(), P.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), C.clone(), D.clone(), WORK.clone(), LWORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariableExt(b.clone(), Expression::makeRealMatrix(B.clone())?)?;
    assignVariable(c.clone(), Expression::makeRealArray(C.clone())?)?;
    assignVariable(d.clone(), Expression::makeRealArray(D.clone())?)?;
    assignVariable(x.clone(), Expression::makeRealArray(X.clone())?)?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgtsv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nrhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut dl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut d: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut du: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut N: i32 = 0;
    let mut NRHS: i32 = 0;
    let mut LDB: i32 = 0;
    let mut INFO: i32 = 0;
    let mut DL: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut D: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut DU: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    nrhs = __pa1.clone();
    dl = __pa2.clone();
    d = __pa3.clone();
    du = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    info = __pa7.clone();
    N = evaluateExtIntArg(n.clone());
    NRHS = evaluateExtIntArg(nrhs.clone());
    DL = evaluateExtRealArrayArg(dl.clone())?;
    D = evaluateExtRealArrayArg(d.clone())?;
    DU = evaluateExtRealArrayArg(du.clone())?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    (DL, D, DU, B, INFO) = Lapack::dgtsv(N.clone(), NRHS.clone(), DL.clone(), D.clone(), DU.clone(), B.clone(), LDB.clone());
    assignVariable(dl.clone(), Expression::makeRealArray(DL.clone())?)?;
    assignVariable(d.clone(), Expression::makeRealArray(D.clone())?)?;
    assignVariable(du.clone(), Expression::makeRealArray(DU.clone())?)?;
    assignVariableExt(b.clone(), Expression::makeRealMatrix(B.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgbsv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut kl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ku: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nrhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ab: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldab: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ipiv: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut N: i32 = 0;
    let mut KL: i32 = 0;
    let mut KU: i32 = 0;
    let mut NRHS: i32 = 0;
    let mut LDAB: i32 = 0;
    let mut LDB: i32 = 0;
    let mut INFO: i32 = 0;
    let mut AB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    kl = __pa1.clone();
    ku = __pa2.clone();
    nrhs = __pa3.clone();
    ab = __pa4.clone();
    ldab = __pa5.clone();
    ipiv = __pa6.clone();
    b = __pa7.clone();
    ldb = __pa8.clone();
    info = __pa9.clone();
    N = evaluateExtIntArg(n.clone());
    KL = evaluateExtIntArg(kl.clone());
    KU = evaluateExtIntArg(ku.clone());
    NRHS = evaluateExtIntArg(nrhs.clone());
    AB = evaluateExtRealMatrixArg(ab.clone())?;
    LDAB = evaluateExtIntArg(ldab.clone());
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    (AB, IPIV, B, INFO) = Lapack::dgbsv(N.clone(), KL.clone(), KU.clone(), NRHS.clone(), AB.clone(), LDAB.clone(), B.clone(), LDB.clone());
    assignVariableExt(ab.clone(), Expression::makeRealMatrix(AB.clone())?)?;
    assignVariable(ipiv.clone(), Expression::makeIntegerArray(IPIV.clone())?)?;
    assignVariableExt(b.clone(), Expression::makeRealMatrix(B.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgesvd(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut jobu: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut jobvt: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut m: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut s: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut u: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldu: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut vt: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldvt: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut JOBU: ArcStr = arcstr::literal!("");
    let mut JOBVT: ArcStr = arcstr::literal!("");
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDU: i32 = 0;
    let mut LDVT: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut U: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut VT: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut S: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => bail!("pattern mismatch"),
    } };
    jobu = __pa0.clone();
    jobvt = __pa1.clone();
    m = __pa2.clone();
    n = __pa3.clone();
    a = __pa4.clone();
    lda = __pa5.clone();
    s = __pa6.clone();
    u = __pa7.clone();
    ldu = __pa8.clone();
    vt = __pa9.clone();
    ldvt = __pa10.clone();
    work = __pa11.clone();
    lwork = __pa12.clone();
    info = __pa13.clone();
    JOBU = (evaluateExtStringArg(jobu.clone())).clone();
    JOBVT = (evaluateExtStringArg(jobvt.clone())).clone();
    M = evaluateExtIntArg(m.clone());
    N = evaluateExtIntArg(n.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    LDU = evaluateExtIntArg(ldu.clone());
    LDVT = evaluateExtIntArg(ldvt.clone());
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (A, S, U, VT, WORK, INFO) = Lapack::dgesvd((JOBU.clone()).clone(), (JOBVT.clone()).clone(), M.clone(), N.clone(), A.clone(), LDA.clone(), LDU.clone(), LDVT.clone(), WORK.clone(), LWORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariable(s.clone(), Expression::makeRealArray(S.clone())?)?;
    assignVariableExt(u.clone(), Expression::makeRealMatrix(U.clone())?)?;
    assignVariableExt(vt.clone(), Expression::makeRealMatrix(VT.clone())?)?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgetrf(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ipiv: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut LDA: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    a = __pa2.clone();
    lda = __pa3.clone();
    ipiv = __pa4.clone();
    info = __pa5.clone();
    M = evaluateExtIntArg(m.clone());
    N = evaluateExtIntArg(n.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    (A, IPIV, INFO) = Lapack::dgetrf(M.clone(), N.clone(), A.clone(), LDA.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariable(ipiv.clone(), Expression::makeIntegerArray(IPIV.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgetrs(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut trans: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nrhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ipiv: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut TRANS: ArcStr = arcstr::literal!("");
    let mut N: i32 = 0;
    let mut NRHS: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LDB: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Nil } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    trans = __pa0.clone();
    n = __pa1.clone();
    nrhs = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    ipiv = __pa5.clone();
    b = __pa6.clone();
    ldb = __pa7.clone();
    info = __pa8.clone();
    TRANS = (evaluateExtStringArg(trans.clone())).clone();
    N = evaluateExtIntArg(n.clone());
    NRHS = evaluateExtIntArg(nrhs.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    IPIV = evaluateExtIntArrayArg(ipiv.clone())?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb.clone());
    (B, INFO) = Lapack::dgetrs((TRANS.clone()).clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), IPIV.clone(), B.clone(), LDB.clone());
    assignVariableExt(b.clone(), Expression::makeRealMatrix(B.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgetri(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ipiv: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut N: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    a = __pa1.clone();
    lda = __pa2.clone();
    ipiv = __pa3.clone();
    work = __pa4.clone();
    lwork = __pa5.clone();
    info = __pa6.clone();
    N = evaluateExtIntArg(n.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    IPIV = evaluateExtIntArrayArg(ipiv.clone())?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (A, WORK, INFO) = Lapack::dgetri(N.clone(), A.clone(), LDA.clone(), IPIV.clone(), WORK.clone(), LWORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dgeqpf(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut jpvt: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tau: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut LDA: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut JPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut TAU: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    a = __pa2.clone();
    lda = __pa3.clone();
    jpvt = __pa4.clone();
    tau = __pa5.clone();
    work = __pa6.clone();
    info = __pa7.clone();
    M = evaluateExtIntArg(m.clone());
    N = evaluateExtIntArg(n.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    (A, JPVT, TAU, INFO) = Lapack::dgeqpf(M.clone(), N.clone(), A.clone(), LDA.clone(), JPVT.clone(), WORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariable(jpvt.clone(), Expression::makeIntegerArray(JPVT.clone())?)?;
    assignVariable(tau.clone(), Expression::makeRealArray(TAU.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dorgqr(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut k: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut a: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lda: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tau: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut K: i32 = 0;
    let mut LDA: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut TAU: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Nil } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    k = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    tau = __pa5.clone();
    work = __pa6.clone();
    lwork = __pa7.clone();
    info = __pa8.clone();
    M = evaluateExtIntArg(m.clone());
    N = evaluateExtIntArg(n.clone());
    K = evaluateExtIntArg(k.clone());
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda.clone());
    TAU = evaluateExtRealArrayArg(tau.clone())?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (A, WORK, INFO) = Lapack::dorgqr(M.clone(), N.clone(), K.clone(), A.clone(), LDA.clone(), TAU.clone(), WORK.clone(), LWORK.clone());
    assignVariableExt(a.clone(), Expression::makeRealMatrix(A.clone())?)?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

pub fn Lapack_dhseqr(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut job: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut compz: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ilo: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ihi: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut h: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldh: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut wr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut wi: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut z: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ldz: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut work: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lwork: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut N: i32 = 0;
    let mut ILO: i32 = 0;
    let mut IHI: i32 = 0;
    let mut LDH: i32 = 0;
    let mut LDZ: i32 = 0;
    let mut LWORK: i32 = 0;
    let mut INFO: i32 = 0;
    let mut JOB: ArcStr = arcstr::literal!("");
    let mut COMPZ: ArcStr = arcstr::literal!("");
    let mut H: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut Z: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut WR: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut WI: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => bail!("pattern mismatch"),
    } };
    job = __pa0.clone();
    compz = __pa1.clone();
    n = __pa2.clone();
    ilo = __pa3.clone();
    ihi = __pa4.clone();
    h = __pa5.clone();
    ldh = __pa6.clone();
    wr = __pa7.clone();
    wi = __pa8.clone();
    z = __pa9.clone();
    ldz = __pa10.clone();
    work = __pa11.clone();
    lwork = __pa12.clone();
    info = __pa13.clone();
    JOB = (evaluateExtStringArg(job.clone())).clone();
    COMPZ = (evaluateExtStringArg(compz.clone())).clone();
    N = evaluateExtIntArg(n.clone());
    ILO = evaluateExtIntArg(ilo.clone());
    IHI = evaluateExtIntArg(ihi.clone());
    H = evaluateExtRealMatrixArg(h.clone())?;
    LDH = evaluateExtIntArg(ldh.clone());
    Z = evaluateExtRealMatrixArg(z.clone())?;
    LDZ = evaluateExtIntArg(ldz.clone());
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork.clone());
    (H, WR, WI, Z, WORK, INFO) = Lapack::dhseqr((JOB.clone()).clone(), (COMPZ.clone()).clone(), N.clone(), ILO.clone(), IHI.clone(), H.clone(), LDH.clone(), Z.clone(), LDZ.clone(), WORK.clone(), LWORK.clone());
    assignVariableExt(h.clone(), Expression::makeRealMatrix(H.clone())?)?;
    assignVariable(wr.clone(), Expression::makeRealArray(WR.clone())?)?;
    assignVariable(wi.clone(), Expression::makeRealArray(WI.clone())?)?;
    assignVariableExt(z.clone(), Expression::makeRealMatrix(Z.clone())?)?;
    assignVariable(work.clone(), Expression::makeRealArray(WORK.clone())?)?;
    assignVariable(info.clone(), Expression::makeInteger(INFO.clone()))?;
    Ok(())
}

fn evaluateExtIntArg(mut arg: Arc<Expression::NFExpression>) -> i32 {
    let mut value: i32 = getExtIntValue(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone()).unwrap()).unwrap();
    value
}

fn getExtIntValue(mut exp: Arc<Expression::NFExpression>) -> Result<i32> {
    let mut value: i32 = 0;
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => var_field!((*exp).value, Expression::NFExpression::INTEGER).clone(),
        Deref @ Expression::EMPTY { .. } => 0,
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

fn evaluateExtRealArg(mut arg: Arc<Expression::NFExpression>) -> metamodelica::Real {
    let mut value: metamodelica::Real = getExtRealValue(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone()).unwrap()).unwrap();
    value
}

fn getExtRealValue(mut exp: Arc<Expression::NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { .. } => var_field!((*exp).value, Expression::NFExpression::REAL).clone(),
        Deref @ Expression::EMPTY { .. } => metamodelica::OrderedFloat(0.0_f64),
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

fn evaluateExtStringArg(mut arg: Arc<Expression::NFExpression>) -> ArcStr {
    let mut value: ArcStr = getExtStringValue(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone()).unwrap()).unwrap();
    value
}

fn getExtStringValue(mut exp: Arc<Expression::NFExpression>) -> Result<ArcStr> {
    let mut value: ArcStr = arcstr::literal!("");
    value = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::STRING { .. } => var_field!((*exp).value, Expression::NFExpression::STRING).clone(),
        Deref @ Expression::EMPTY { .. } => literal!(""),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(value)
}

fn evaluateExtIntArrayArg(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut value: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    expl = Expression::arrayElementList(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone())?)?;
    value = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = getExtIntValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(value)
}

fn evaluateExtRealArrayArg(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<metamodelica::Real>>> {
    let mut value: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    expl = Expression::arrayElementList(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone())?)?;
    value = {
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = getExtRealValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(value)
}

fn evaluateExtRealMatrixArg(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>> {
    let mut value: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut expl: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone())?) {
        Deref @ Expression::ARRAY { elements: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expl = __pa0.clone();
    ty = __pa1.clone();
    value = (match Type::dimensionCount(ty.clone()) {
        1 => {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
        for mut e in (expl.clone()).borrow().iter() {
            let __x = list![getExtRealValue(e.clone())?];
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        2 => {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
        for mut row in (expl.clone()).borrow().iter() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut e in (Expression::arrayElements(row.clone())?).borrow().iter() {
            let __x = getExtRealValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        _ => bail!("match: no arm matched"),
    });
    Ok(value)
}

fn assignVariableExt(mut variable: Arc<Expression::NFExpression>, mut value: Arc<Expression::NFExpression>) -> Result<()> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((Expression::typeOf(variable.clone()), value.clone())) {
        (Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. }) => Expression::makeArray(Type::unliftArray(var_field!((*value).ty, Expression::NFExpression::ARRAY).clone())?, metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*value).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::arrayScalarElement(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), true),
        _ => value.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    assignVariable(variable.clone(), exp.clone())?;
    Ok(())
}

