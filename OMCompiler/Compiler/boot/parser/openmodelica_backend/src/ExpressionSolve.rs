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
use crate::BackendDAEUtil;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Differentiate;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::Inline;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
// =============================================================================
// section for postOptModule >>solveSimpleEquations<<
//
// solve simple equations otherwise detect EQUATIONSYSTEM
// =============================================================================
pub fn solveSimpleEquations(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    assign_field!(dae.eqs = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
        for mut syst in (dae.eqs.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2, ass1, comps }, .. } => {
            let mut comps = (*comps).clone();
            comps = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
        for mut comp in (comps.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { .. } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut var: BackendDAE::Var;
            let mut eindex: i32 = 0;
            let mut vindx: i32 = 0;
            let mut solved: bool = false;
            let mut tmpComp: Arc<BackendDAE::StrongComponent>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp.clone()) {
                Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: __pa0, eqn: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            vindx = __pa0.clone();
            eindex = __pa1.clone();
            eqn = BackendEquation::get(syst.orderedEqs.clone(), eindex.clone());
            tmpComp = comp.clone();
            if BackendEquation::isEquation(eqn.clone()) {
                var = BackendVariable::getVarAt(syst.orderedVars.clone(), vindx.clone())?;
                (eqn, solved) = solveSimpleEquation(eqn.clone(), var.clone(), dae.shared.clone())?;
                assign_field!(syst.orderedEqs = BackendEquation::setAtIndex(syst.orderedEqs.clone(), eindex.clone(), eqn.clone())?);
                if !(solved.clone()) {
                    tmpComp = Arc::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: list![eindex.clone()], vars: list![vindx.clone()], jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN), jacType: crate::BackendDAE::JacobianType::JAC_NONLINEAR, mixedSystem: false });
                }
            }
            tmpComp.clone()
        },
        _ => {
            comp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assign_field!(syst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() }));
            syst.clone()
        },
        _ => {
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(dae)
}

fn solveSimpleEquation(mut eqn: Arc<BackendDAE::Equation>, mut var: BackendDAE::Var, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::Equation>, bool)> {
    let mut eqn: Arc<BackendDAE::Equation> = eqn;
    let mut solved: bool = false;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut varexp: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut attr: BackendDAE::EquationAttributes;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut isContinuousIntegration: bool = BackendDAEUtil::isSimulationDAE(shared.clone());
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr: __pa0, source: __pa1, scalar: __pa2, exp: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    attr = __pa0.clone();
    source = __pa1.clone();
    e2 = __pa2.clone();
    e1 = __pa3.clone();
    let BackendDAE::VAR { varName: __pa4, .. } = (var.clone()) else { bail!("pattern mismatch") };
    cr = __pa4.clone();
    varexp = Expression::crefExp(cr.clone())?;
    if BackendVariable::isStateVar(var.clone()) {
        varexp = Expression::expDer(varexp.clone());
        cr = ComponentReference::crefPrefixDer(cr.clone());
    }
    if Types::isIntegerOrRealOrSubTypeOfEither(Expression::r#typeof(e1.clone())?)? && Types::isIntegerOrRealOrSubTypeOfEither(Expression::r#typeof(e2.clone())?)? {
        (e1, e2) = preprocessingSolve(e1.clone(), e2.clone(), varexp.clone(), None, Some(shared.functionTree.clone()), None, 0, false)?;
    }
    match '__try5: {
        e = unwrap_break_err!(solve2(e1.clone(), e2.clone(), varexp.clone(), Some(shared.functionTree.clone()), None, false, isContinuousIntegration.clone()), '__try5);
        source = unwrap_break_err!(ElementSource::addSymbolicTransformationSolve(true, source.clone(), cr.clone(), e1.clone(), e2.clone(), e.clone(), metamodelica::nil()), '__try5);
        eqn = unwrap_break_err!(BackendEquation::generateEquation(varexp.clone(), e.clone(), source.clone(), attr.clone()), '__try5);
        solved = true;
        Ok::<_, anyhow::Error>((e.clone(), eqn.clone(), solved.clone(), source.clone()))
    } {
        Ok((__try5_o0, __try5_o1, __try5_o2, __try5_o3)) => {
            e = __try5_o0;
            eqn = __try5_o1;
            solved = __try5_o2;
            source = __try5_o3;
        }
        Err(_) => {
            solved = false;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((eqn, solved))
}

fn printTryToSolve(mut instanceName: ArcStr, mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*instanceName.clone()); __mm_s.push_str(&*literal!(" tries to solve: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2.clone())?); __mm_s.push_str(&*literal!("\nwith respect to: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp3.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn solveSimple(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>, mut idepth: i32) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outAsserts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut eqnForNewVars: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVarsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut odepth: i32 = idepth.clone();
    (outExp, outAsserts) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone(), inExp3.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, _, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) if (ComponentReferenceBasics::crefEqual(cr.clone(), cr1.clone())? && !(Expression::expHasCrefNoPreOrStart(inExp2.clone(), cr.clone())?)) => {
            (inExp2.clone(), metamodelica::nil())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }) if (ComponentReferenceBasics::crefEqual(cr.clone(), cr1.clone())? && !(Expression::expHasDerCref(inExp2.clone(), cr.clone())?)) => {
            (inExp2.clone(), metamodelica::nil())
        },
        (Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::UMINUS { .. } }, _, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) if (ComponentReferenceBasics::crefEqual(cr1.clone(), cr.clone())? && !(Expression::expHasCrefNoPreOrStart(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::UMINUS_ARR { .. } }, _, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) if (ComponentReferenceBasics::crefEqual(cr1.clone(), cr.clone())? && !(Expression::expHasCrefNoPreOrStart(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, operator: DAE::Operator::UMINUS { .. } }, _, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }) if (ComponentReferenceBasics::crefEqual(cr1.clone(), cr.clone())? && !(Expression::expHasDerCref(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, operator: DAE::Operator::UMINUS_ARR { .. } }, _, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }) if (ComponentReferenceBasics::crefEqual(cr1.clone(), cr.clone())? && !(Expression::expHasDerCref(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::LUNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::NOT { .. } }, _, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) if (ComponentReferenceBasics::crefEqual(cr1.clone(), cr.clone())? && !(Expression::expHasCrefNoPreOrStart(inExp2.clone(), cr.clone())?)) => {
            (Expression::negate(inExp2.clone())?, metamodelica::nil())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, .. }, _, Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }) if (ComponentReferenceBasics::crefEqual(cr.clone(), cr1.clone())? && !(Expression::expHasCrefNoPreorDer(inExp2.clone(), cr.clone())?)) => {
            let mut asserts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            asserts = generateAssertType(tp.clone(), cr.clone(), inExp3.clone(), metamodelica::nil())?;
            (Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: inExp2.clone() }), asserts.clone())
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outAsserts, eqnForNewVars, newVarsCrefs, odepth))
}

fn generateAssertType(mut tp: Arc<DAE::Type>, mut cr: Arc<DAE::ComponentRef>, mut iExp: Arc<DAE::Exp>, mut inAsserts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outAsserts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outAsserts = (::match_deref::match_deref! { match &((tp.clone(), cr.clone(), iExp.clone(), inAsserts.clone())) {
        (Deref @ DAE::Type::T_ENUMERATION { names, path, .. }, _, _, _) => {
            let mut p1: Arc<Absyn::Path>;
            let mut pn: Arc<Absyn::Path>;
            let mut n: i32 = 0;
            let mut e1: Arc<DAE::Exp>;
            let mut en: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut es: Arc<DAE::Exp>;
            let mut s1: ArcStr = arcstr::literal!("");
            let mut sn: ArcStr = arcstr::literal!("");
            let mut estr: ArcStr = arcstr::literal!("");
            let mut crstr: ArcStr = arcstr::literal!("");
            p1 = AbsynUtil::suffixPath(path.clone(), (listHead(names.clone())?).clone())?;
            e1 = Arc::new(DAE::Exp::ENUM_LITERAL { name: p1.clone(), index: 1 });
            n = (names.clone().len() as i32);
            pn = AbsynUtil::suffixPath(path.clone(), ((names.clone()).get(n.clone())?).clone())?;
            en = Arc::new(DAE::Exp::ENUM_LITERAL { name: p1.clone(), index: n.clone() });
            s1 = (AbsynUtil::pathString(p1.clone(), (literal!(".")).clone(), true, false)?).clone();
            sn = (AbsynUtil::pathString(pn.clone(), (literal!(".")).clone(), true, false)?).clone();
            let _ = ExpressionBasics::printExpStr(iExp.clone())?;
            crstr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            estr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression for ")); __mm_s.push_str(&*crstr.clone()); __mm_s.push_str(&*literal!(" out of min(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")/max(")); __mm_s.push_str(&*sn.clone()); __mm_s.push_str(&*literal!(") = ")); ArcStr::from(__mm_s) }).clone();
            e = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::RELATION { exp1: iExp.clone(), operator: DAE::Operator::GREATEREQ { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: e1.clone(), index: -1, optionExpisASUB: None }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RELATION { exp1: iExp.clone(), operator: DAE::Operator::LESSEQ { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: en.clone(), index: -1, optionExpisASUB: None }) });
            es = Expression::makePureBuiltinCall((literal!("String")).clone(), list![iExp.clone(), Arc::new(DAE::Exp::SCONST { string: (literal!("d")).clone() })], DAE::T_STRING_DEFAULT().clone());
            es = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::SCONST { string: (estr.clone()).clone() }), operator: DAE::Operator::ADD { ty: DAE::T_STRING_DEFAULT().clone() }, exp2: es.clone() });
            cons(Arc::new(DAE::Statement::STMT_ASSERT { cond: e.clone(), msg: es.clone(), level: DAE::ASSERTIONLEVEL_ERROR().clone(), source: DAE::emptyElementSource().clone() }), inAsserts.clone())
        },
        _ => {
            inAsserts.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAsserts)
}

fn preprocessingSolve2(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, bool)> {
    let mut olhs: Arc<DAE::Exp>;
    let mut orhs: Arc<DAE::Exp>;
    let mut con: bool = false;
    (olhs, orhs, con) = (::match_deref::match_deref! { match &(inExp1.clone()) {
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: fa } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut b: Arc<DAE::Exp>;
            b = Expression::negate(inExp2.clone())?;
            (fa.clone(), b.clone(), true)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: fa } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut b: Arc<DAE::Exp>;
            b = Expression::negate(inExp2.clone())?;
            (fa.clone(), b.clone(), true)
        },
        Deref @ DAE::Exp::BINARY { exp1: b, operator: DAE::Operator::DIV { ty: _ }, exp2: fa } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(b.clone(), inExp3.clone())?) && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::makeDiv(b.clone(), inExp2.clone())?;
            (fa.clone(), e.clone(), true)
        },
        Deref @ DAE::Exp::BINARY { exp1: b, operator: DAE::Operator::MUL { ty: _ }, exp2: fa } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(b.clone(), inExp3.clone())?) && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            let mut eWithX: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut factorWithX: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut factorWithoutX: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut pWithX: Arc<DAE::Exp>;
            let mut pWithoutX: Arc<DAE::Exp>;
            eWithX = Expression::expandFactors(inExp1.clone())?;
            (factorWithX, factorWithoutX) = List::split1OnTrue(eWithX.clone(), (std::sync::Arc::new(expHasCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>), inExp3.clone());
            pWithX = makeProductLstSort(factorWithX.clone())?;
            pWithoutX = makeProductLstSort(factorWithoutX.clone())?;
            e = Expression::makeDiv(inExp2.clone(), pWithoutX.clone())?;
            (pWithX.clone(), e.clone(), true)
        },
        Deref @ DAE::Exp::BINARY { exp1: b, operator: DAE::Operator::MUL { ty: _ }, exp2: fa } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(b.clone(), inExp3.clone())?) && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::makeDiv(inExp2.clone(), b.clone())?;
            (fa.clone(), e.clone(), true)
        },
        Deref @ DAE::Exp::BINARY { exp1: fa, operator: DAE::Operator::MUL { ty: _ }, exp2: b } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(b.clone(), inExp3.clone())?) && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::makeDiv(inExp2.clone(), b.clone())?;
            (fa.clone(), e.clone(), true)
        },
        Deref @ DAE::Exp::BINARY { exp1: fa, operator: DAE::Operator::DIV { ty: _ }, exp2: b } if (expHasCref(fa.clone(), inExp3.clone())? && !(expHasCref(b.clone(), inExp3.clone())?) && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::expMul(inExp2.clone(), b.clone())?;
            (fa.clone(), e.clone(), true)
        },
        Deref @ DAE::Exp::BINARY { exp1: ga, operator: DAE::Operator::DIV { ty: tp }, exp2: fa } if (expHasCref(fa.clone(), inExp3.clone())? && expHasCref(ga.clone(), inExp3.clone())? && !(expHasCref(inExp2.clone(), inExp3.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::Exp>;
            e = Expression::expMul(inExp2.clone(), fa.clone())?;
            lhs = Expression::expSub(e.clone(), ga.clone())?;
            e = Expression::makeConstZero(tp.clone());
            (lhs.clone(), e.clone(), true)
        },
        _ => {
            (inExp1.clone(), inExp2.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((olhs, orhs, con))
}

fn preprocessingSolve3(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, bool)> {
    let mut olhs: Arc<DAE::Exp>;
    let mut orhs: Arc<DAE::Exp>;
    let mut con: bool = false;
    (olhs, orhs, con) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::RCONST { real: r1 }, operator: DAE::Operator::POW { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::RCONST { real: r2 }) if (r2.clone() > metamodelica::OrderedFloat(0.0_f64) && r1.clone() > metamodelica::OrderedFloat(0.0_f64) && !(Expression::isConstOne(e1.clone())) && expHasCref(e2.clone(), inExp3.clone())?) => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut res: Arc<DAE::Exp>;
            r = (r2.clone()).ln() / (r1.clone()).ln();
            res = Arc::new(DAE::Exp::RCONST { real: r.clone() });
            (e2.clone(), res.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::RCONST { real: __rlit_0 }) if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (expHasCref(e1.clone(), inExp3.clone())? && !(expHasCref(e2.clone(), inExp3.clone())?)) => {
            (e1.clone(), inExp2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::RCONST { real: r } }, _) if (!(expHasCref(inExp2.clone(), inExp3.clone())?) && expHasCref(e1.clone(), inExp3.clone())? && metamodelica::OrderedFloat(1.0_f64) == realMod(r.clone(), metamodelica::OrderedFloat(2.0_f64))) => {
            let mut res: Arc<DAE::Exp>;
            res = Expression::makeDiv(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), e2.clone())?;
            res = Expression::expPow(inExp2.clone(), res.clone())?;
            (e1.clone(), res.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_1 } }, _) if __rlit_1.eq(&metamodelica::OrderedFloat((0.5) as f64)) && (!(expHasCref(inExp2.clone(), inExp3.clone())?) && expHasCref(e1.clone(), inExp3.clone())?) => {
            let mut res: Arc<DAE::Exp>;
            res = Expression::expPow(inExp2.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            (e1.clone(), res.clone(), true)
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_2 }) if __rlit_2.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), inExp2.clone(), true)
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_3 }) if __rlit_3.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), inExp2.clone(), true)
        },
        _ => {
            (inExp1.clone(), inExp2.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((olhs, orhs, con))
}

fn preprocessingSolve4(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, bool)> {
    let mut oExp1: Arc<DAE::Exp>;
    let mut oExp2: Arc<DAE::Exp>;
    let mut newX: bool = false;
    (oExp1, oExp2, newX) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone(), inExp3.clone())) {
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_4 }, _) if __rlit_4.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_5 }, _) if __rlit_5.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_6 }, _) if __rlit_6.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_7 }, _) if __rlit_7.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_8 }, _) if __rlit_8.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_9 }, _) if __rlit_9.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), e2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_10 }, _) if __rlit_10.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            (e1.clone(), inExp2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_11 }, _) if __rlit_11.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            (e1.clone(), inExp2.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. } }, operator: DAE::Operator::SUB { ty: tp }, exp2: Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } } }, Deref @ DAE::Exp::RCONST { real: __rlit_12 }, _) if __rlit_12.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::makePureBuiltinCall((literal!("tanh")).clone(), list![e1.clone()], tp.clone());
            (Expression::expMul(e3.clone(), e.clone())?, e4.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } }, operator: DAE::Operator::SUB { ty: tp }, exp2: Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. } } }, Deref @ DAE::Exp::RCONST { real: __rlit_13 }, _) if __rlit_13.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::makePureBuiltinCall((literal!("tanh")).clone(), list![e1.clone()], tp.clone());
            (Expression::expMul(e3.clone(), e.clone())?, e4.clone(), true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, operator: DAE::Operator::SUB { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::RCONST { real: __rlit_14 }, _) if __rlit_14.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), Expression::expPow(e2.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?, true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::SUB { ty: _ }, exp2: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_15 }, _) if __rlit_15.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), Expression::expPow(e2.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?, true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 }, operator: DAE::Operator::SUB { ty: tp }, exp2: Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::POW { .. }, exp2: e4 } }, Deref @ DAE::Exp::RCONST { real: __rlit_16 }, _) if __rlit_16.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (ExpressionBasics::expEqual(e2.clone(), e4.clone())? && expHasCref(e1.clone(), inExp3.clone())? && expHasCref(e3.clone(), inExp3.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            let mut e_1: Arc<DAE::Exp>;
            let mut e_2: Arc<DAE::Exp>;
            e = Expression::expPow(Expression::makeDiv(e1.clone(), e3.clone())?, e2.clone())?;
            (e_1, e_2, _) = preprocessingSolve3(e.clone(), Expression::makeConstOne(tp.clone()), inExp3.clone())?;
            (e_1.clone(), e_2.clone(), true)
        },
        _ => {
            (inExp1.clone(), inExp2.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oExp1, oExp2, newX))
}

fn expAddX(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut ores: Arc<DAE::Exp>;
    ores = 'mc: {
        let __mc_input = (inExp1.clone(), inExp2.clone(), inExp3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e, expThen: e1, expElse: e2 }, _, _) => {
                    if !((expHasCref(e1.clone(), inExp3.clone())? && expHasCref(e2.clone(), inExp3.clone())? && !(expHasCref(e.clone(), inExp3.clone())?))) { bail!("guard") }
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    e3 = expAddX(inExp2.clone(), e1.clone(), inExp3.clone())?;
                    e4 = expAddX(inExp2.clone(), e2.clone(), inExp3.clone())?;
                    res = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e3.clone(), expElse: e4.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::IFEXP { expCond: e, expThen: e1, expElse: e2 }, _) => {
                    if !((expHasCref(e1.clone(), inExp3.clone())? && expHasCref(e2.clone(), inExp3.clone())? && !(expHasCref(e.clone(), inExp3.clone())?))) { bail!("guard") }
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    e3 = expAddX(inExp1.clone(), e1.clone(), inExp3.clone())?;
                    e4 = expAddX(inExp1.clone(), e2.clone(), inExp3.clone())?;
                    res = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e3.clone(), expElse: e4.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: Arc<DAE::Exp>;
                    res = expAddX2(inExp1.clone(), inExp2.clone(), inExp3.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ores)
}

fn expAddX2(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut ores: Arc<DAE::Exp>;
    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut e0: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut neg: bool = false;
    let mut factorWithX1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut factorWithoutX1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut factorWithX2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut factorWithoutX2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut pWithX1: Arc<DAE::Exp>;
    let mut pWithoutX1: Arc<DAE::Exp>;
    let mut pWithX2: Arc<DAE::Exp>;
    let mut pWithoutX2: Arc<DAE::Exp>;
    (e0, e1, neg) = (::match_deref::match_deref! { match &(inExp1.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: ee1, operator: DAE::Operator::ADD { .. }, exp2: ee2 } => {
            (ee1.clone(), ee2.clone(), false)
        },
        Deref @ DAE::Exp::BINARY { exp1: ee1, operator: DAE::Operator::SUB { .. }, exp2: ee2 } => {
            (ee1.clone(), ee2.clone(), true)
        },
        _ => {
            (Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), inExp1.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    f1 = Expression::expandFactors(e1.clone())?;
    (factorWithX1, factorWithoutX1) = List::split1OnTrue(f1.clone(), (std::sync::Arc::new(expHasCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>), inExp3.clone());
    pWithX1 = makeProductLstSort(factorWithX1.clone())?;
    pWithoutX1 = makeProductLstSort(factorWithoutX1.clone())?;
    f2 = Expression::expandFactors(inExp2.clone())?;
    (factorWithX2, factorWithoutX2) = List::split1OnTrue(f2.clone(), (std::sync::Arc::new(expHasCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>), inExp3.clone());
    (pWithX2, _) = ExpressionSimplify::simplify1(makeProductLstSort(factorWithX2.clone())?)?;
    pWithoutX2 = makeProductLstSort(factorWithoutX2.clone())?;
    if ExpressionBasics::expEqual(pWithX2.clone(), pWithX1.clone())? {
        if !(neg.clone()) {
            ores = Expression::expAdd(pWithoutX1.clone(), pWithoutX2.clone())?;
        } else {
            ores = Expression::expSub(pWithoutX2.clone(), pWithoutX1.clone())?;
        }
        ores = Expression::expMul(ores.clone(), pWithX2.clone())?;
    } else if ExpressionBasics::expEqual(pWithX2.clone(), Expression::negate(pWithX1.clone())?)? {
        if !(neg.clone()) {
            ores = Expression::expSub(pWithoutX2.clone(), pWithoutX1.clone())?;
        } else {
            ores = Expression::expAdd(pWithoutX1.clone(), pWithoutX2.clone())?;
        }
        ores = Expression::expMul(ores.clone(), pWithX2.clone())?;
    } else {
        e1 = Expression::expMul(pWithoutX1.clone(), pWithX1.clone())?;
        e2 = Expression::expMul(pWithoutX2.clone(), pWithX2.clone())?;
        ores = Expression::expAdd(e1.clone(), e2.clone())?;
    }
    ores = Expression::expAdd(e0.clone(), ores.clone())?;
    Ok(ores)
}

pub fn collectX(mut inExp1: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>, mut expand: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outLhs: Arc<DAE::Exp>;
    let mut outRhs: Arc<DAE::Exp>;
    (outLhs, outRhs) = preprocessingSolve5(inExp1.clone(), inExp3.clone(), expand.clone())?;
    Ok((outLhs, outRhs))
}

fn preprocessingSolve5(mut inExp1: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>, mut expand: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outLhs: Arc<DAE::Exp>;
    let mut outRhs: Arc<DAE::Exp>;
    let mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tmpLhs: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut b: bool = false;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    if expHasCref(inExp1.clone(), inExp3.clone())? {
        if expand.clone() {
            (cr, b) = Expression::expOrDerCref(inExp3.clone())?;
            if b.clone() {
                (lhs, rhs) = Expression::allTermsForCref(inExp1.clone(), cr.clone(), (std::sync::Arc::new(Expression::expHasDerCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            } else {
                (lhs, rhs) = Expression::allTermsForCref(inExp1.clone(), cr.clone(), (std::sync::Arc::new(Expression::expHasCrefNoPreOrStart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            }
        } else {
            (lhs, rhs) = List::split1OnTrue(Expression::terms(inExp1.clone())?, (std::sync::Arc::new(expHasCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>), inExp3.clone());
        }
        outLhs = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
        tmpLhs = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
        for mut e in &*lhs.clone() {
            let mut e = e.clone();
            if Expression::isNegativeUnary(e.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(e.clone()) {
                    Deref @ DAE::Exp::UNARY { exp: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e1 = __pa0.clone();
                tmpLhs = expAddX(e1.clone(), tmpLhs.clone(), inExp3.clone())?;
            } else {
                outLhs = expAddX(e.clone(), outLhs.clone(), inExp3.clone())?;
            }
        }
        outLhs = expAddX(outLhs.clone(), Expression::negate(tmpLhs.clone())?, inExp3.clone())?;
        outRhs = Expression::makeSum1(rhs.clone(), false)?;
        (outRhs, _) = ExpressionSimplify::simplify1(outRhs.clone())?;
        (outLhs, _) = ExpressionSimplify::simplify1(outLhs.clone())?;
    } else {
        outLhs = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
        outRhs = inExp1.clone();
    }
    Ok((outLhs, outRhs))
}

fn unifyFunCalls(mut inExp1: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut oExp: Arc<DAE::Exp>;
    let mut newX: bool = false;
    (oExp, _) = Expression::traverseExpTopDown(inExp1.clone(), (std::sync::Arc::new(unifyFunCallsWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool, Arc<DAE::Exp>)> + 'static>), inExp3.clone())?;
    newX = ExpressionBasics::expEqual(oExp.clone(), inExp1.clone())?;
    Ok((oExp, newX))
}

fn unifyFunCallsWork(mut inExp: Arc<DAE::Exp>, mut iT: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool, Arc<DAE::Exp>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut oT: Arc<DAE::Exp>;
    (outExp, cont, oT) = (::match_deref::match_deref! { match &((inExp.clone(), iT.clone())) {
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, _) if (!(Expression::isZero(e1.clone()))) => {
            let mut e: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(e1.clone())?;
            e = Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: Expression::makeConstZero(tp.clone()), index: -1, optionExpisASUB: None }), expThen: Expression::expMul(e1.clone(), e2.clone())?, expElse: Expression::expMul(e1.clone(), e3.clone())? });
            (e.clone(), true, iT.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_DF$DER" }, .. }, X) if (expHasCref(e1.clone(), X.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(e1.clone())?;
            e2 = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::symSolverDT)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            e3 = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![e1.clone()], tp.clone());
            e3 = Expression::expSub(e1.clone(), e3.clone())?;
            e = Expression::expDiv(e3.clone(), e2.clone())?;
            (e.clone(), true, iT.clone())
        },
        _ => {
            (inExp.clone(), true, iT.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, oT))
}

fn removeSimpleCalls(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, bool)> {
    let mut outLhs: Arc<DAE::Exp>;
    let mut outRhs: Arc<DAE::Exp>;
    let mut con: bool = false;
    (outLhs, outRhs, con) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone(), inExp3.clone())) {
        (Deref @ DAE::Exp::CALL { .. }, _, _) => removeSimpleCalls2(inExp1.clone(), inExp2.clone(), inExp3.clone())?,
        _ => (inExp1.clone(), inExp2.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outLhs, outRhs, con))
}

fn removeSimpleCalls2(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, bool)> {
    let mut outLhs: Arc<DAE::Exp>;
    let mut outRhs: Arc<DAE::Exp>;
    let mut con: bool = false;
    (outLhs, outRhs, con) = 'mc: {
        let __mc_input = (inExp1.clone(), inExp2.clone(), inExp3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, .. }, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    let true = (!(Expression::isCref(inExp2.clone()) || Expression::isConst(inExp2.clone())?)) else { bail!("pattern mismatch") };
                    e2 = Expression::expAdd(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), inExp2.clone())?;
                    e3 = Expression::expSub(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), inExp2.clone())?;
                    e2 = Expression::makeDiv(e2.clone(), e3.clone())?;
                    e2 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e2.clone()], DAE::T_REAL_DEFAULT().clone());
                    e2 = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), e2.clone())?;
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. }, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    let true = (!(Expression::isCref(inExp2.clone()) || Expression::isConst(inExp2.clone())?)) else { bail!("pattern mismatch") };
                    e2 = Expression::expPow(inExp2.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
                    e3 = Expression::expAdd(e2.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
                    e2 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![e3.clone()], DAE::T_REAL_DEFAULT().clone());
                    e3 = Expression::expAdd(inExp2.clone(), e2.clone())?;
                    e2 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e3.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. }, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    e2 = Expression::expPow(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) }), inExp2.clone())?;
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. }, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    e2 = Expression::makePureBuiltinCall((literal!("exp")).clone(), list![inExp2.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    e2 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![inExp2.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let true = (expHasCref(e1.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
                    e2 = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) });
                    e2 = Expression::expPow(inExp2.clone(), e2.clone())?;
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: __rlit_17 }, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_18 }, _) => {
                    if !(__rlit_17.eq(&metamodelica::OrderedFloat((0.0) as f64)) && __rlit_18.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp1.clone(), inExp2.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outLhs, outRhs, con))
}

fn preprocessingSolveTmpVars(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>, mut optCond: Option<Arc<DAE::Exp>>, mut uniqueEqIndex: i32, mut ieqnForNewVars: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inewVarsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut idepth: i32) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32)> {
    let mut x: Arc<DAE::Exp>;
    let mut y: Arc<DAE::Exp>;
    let mut new_x: bool = false;
    let mut eqnForNewVars: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVarsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut odepth: i32 = 0;
    (x, y, new_x, eqnForNewVars, newVarsCrefs, odepth) = 'mc: {
        let __mc_input = inExp1.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name }, .. } => {
                    if !((expHasCref(arg.clone(), inExp3.clone())? && !(expHasCref(inExp2.clone(), inExp3.clone())?))) { bail!("guard") }
                    let mut eqnForNewVars_: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut newVarsCrefs_: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut odepth: i32 = odepth.clone();
                    let mut new_x: bool = new_x.clone();
                    let mut y: Arc<DAE::Exp>;
                    (y, new_x, eqnForNewVars_, newVarsCrefs_, odepth) = preprocessingSolveFunctionCall((name.clone()).clone(), arg.clone(), inExp2.clone(), inExp3.clone(), optCond.clone(), uniqueEqIndex.clone(), idepth.clone())?;
                    if eqnForNewVars_.clone().is_empty() {
                        eqnForNewVars_ = ieqnForNewVars.clone();
                    } else {
                        eqnForNewVars_ = (::match_deref::match_deref! { match &(optCond.clone()) {
        Some(cond) => {
                    cons(Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: list![cond.clone()], eqnstrue: list![eqnForNewVars_.clone()], eqnsfalse: metamodelica::nil(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), ieqnForNewVars.clone())
        },
        _ => {
                    listAppend(eqnForNewVars_.clone(), ieqnForNewVars.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    }
                    Ok((if (new_x.clone()) {arg.clone()} else {inExp1.clone()}, y.clone(), new_x.clone(), eqnForNewVars_.clone(), listAppend(newVarsCrefs_.clone(), inewVarsCrefs.clone()), odepth.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 } => {
                    if !((expHasCref(e1.clone(), inExp3.clone())? && !(expHasCref(e2.clone(), inExp3.clone())?))) { bail!("guard") }
                    let mut e_1: Arc<DAE::Exp>;
                    let mut exP: Arc<DAE::Exp>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut eqnForNewVars_: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut newVarsCrefs_: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut tp = (*tp).clone();
                    tp = Expression::r#typeof(e1.clone())?;
                    exP = makeInitialGuess(tp.clone(), inExp3.clone(), e1.clone())?;
                    (exP, eqnForNewVars_, newVarsCrefs_) = makeTmpEqnAndCrefFromExp(exP.clone(), tp.clone(), (literal!("X$ABS")).clone(), uniqueEqIndex.clone(), idepth.clone(), ieqnForNewVars.clone(), inewVarsCrefs.clone(), false)?;
                    e_1 = Expression::makePureBuiltinCall((literal!("$_signNoNull")).clone(), list![exP.clone()], tp.clone());
                    lhs = Expression::expPow(inExp2.clone(), Expression::inverseFactors(e2.clone())?)?;
                    lhs = Expression::makePureBuiltinCall((literal!("abs")).clone(), list![lhs.clone()], tp.clone());
                    lhs = Expression::expMul(e_1.clone(), lhs.clone())?;
                    Ok((e1.clone(), lhs.clone(), true, eqnForNewVars_.clone(), newVarsCrefs_.clone(), idepth.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: ee1, operator: op1, exp2: ee2 } => {
                    if !((Expression::isAddOrSub(op1.clone()))) { bail!("guard") }
                    let mut e2: Arc<DAE::Exp>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut e6: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut a1: Arc<DAE::Exp>;
                    let mut x1: Arc<DAE::Exp>;
                    let mut a2: Arc<DAE::Exp>;
                    let mut x2: Arc<DAE::Exp>;
                    let mut z1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut z2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut z3: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut z4: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut eqnForNewVars_: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut newVarsCrefs_: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    (z1, z2) = List::split1OnTrue(Expression::factors(ee1.clone())?, (std::sync::Arc::new(expHasCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>), inExp3.clone());
                    (z3, z4) = List::split1OnTrue(Expression::factors(ee2.clone())?, (std::sync::Arc::new(expHasCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>), inExp3.clone());
                    x1 = makeProductLstSort(z1.clone())?;
                    a1 = makeProductLstSort(z2.clone())?;
                    x2 = makeProductLstSort(z3.clone())?;
                    a2 = if (Expression::isAdd(op1.clone())) {makeProductLstSort(z4.clone())?} else {Expression::negate(makeProductLstSort(z4.clone())?)?};
                    (e2, e3) = simplifyBinaryMulCoeff(x1.clone())?;
                    (e5, e6) = simplifyBinaryMulCoeff(x2.clone())?;
                    (lhs, rhs, eqnForNewVars_, newVarsCrefs_) = solveQE(a1.clone(), e2.clone(), e3.clone(), a2.clone(), e5.clone(), e6.clone(), inExp2.clone(), inExp3.clone(), ieqnForNewVars.clone(), inewVarsCrefs.clone(), uniqueEqIndex.clone(), idepth.clone())?;
                    Ok((lhs.clone(), rhs.clone(), true, eqnForNewVars_.clone(), newVarsCrefs_.clone(), idepth.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp1.clone(), inExp2.clone(), false, ieqnForNewVars.clone(), inewVarsCrefs.clone(), idepth.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((x, y, new_x, eqnForNewVars, newVarsCrefs, odepth))
}

fn preprocessingSolveFunctionCall(mut name: ArcStr, mut arg: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>, mut optCond: Option<Arc<DAE::Exp>>, mut uniqueEqIndex: i32, mut idepth: i32) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32)> {
    let mut result: Arc<DAE::Exp>;
    let mut new_x: bool = false;
    let mut newEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut odepth: i32 = 0;
    (result, new_x, newEqns, newVars, odepth) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "tanh" => {
            let mut y: Arc<DAE::Exp>;
            let mut inv: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), (literal!("Y$TANH")).clone(), uniqueEqIndex.clone(), idepth.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            e1 = Expression::expAdd(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), y.clone())?;
            e2 = Expression::expSub(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), y.clone())?;
            e1 = Expression::makeDiv(e1.clone(), e2.clone())?;
            e1 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], tp.clone());
            inv = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), e1.clone())?;
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((metamodelica::OrderedFloat(-1.0_f64), false)), Some((metamodelica::OrderedFloat(1.0_f64), false)))?;
            (inv.clone(), true, cons(ass.clone(), eqns.clone()), vars.clone(), idepth.clone() + 1)
        },
        Deref @ "sinh" => {
            let mut y: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), (literal!("Y$SINH")).clone(), uniqueEqIndex.clone(), idepth.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            e1 = Expression::expPow(y.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            e1 = Expression::expAdd(e1.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
            e1 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![e1.clone()], tp.clone());
            e1 = Expression::expAdd(y.clone(), e1.clone())?;
            e1 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], tp.clone());
            (e1.clone(), true, eqns.clone(), vars.clone(), idepth.clone() + 1)
        },
        Deref @ "cosh" => {
            let mut y: Arc<DAE::Exp>;
            let mut exP: Arc<DAE::Exp>;
            let mut sgn: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), (literal!("Y$COSH")).clone(), uniqueEqIndex.clone(), idepth.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            exP = makeInitialGuess(tp.clone(), inExp3.clone(), arg.clone())?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP.clone(), tp.clone(), (literal!("SIGN$COSH")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            sgn = Expression::makePureBuiltinCall((literal!("$_signNoNull")).clone(), list![exP.clone()], tp.clone());
            e1 = Expression::expPow(y.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            e1 = Expression::expSub(e1.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
            e1 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![e1.clone()], tp.clone());
            e1 = Expression::expMul(sgn.clone(), e1.clone())?;
            e1 = Expression::expAdd(y.clone(), e1.clone())?;
            e1 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], tp.clone());
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((metamodelica::OrderedFloat(1.0_f64), true)), None)?;
            (e1.clone(), true, cons(ass.clone(), eqns.clone()), vars.clone(), idepth.clone() + 1)
        },
        Deref @ "cos" => {
            let mut y: Arc<DAE::Exp>;
            let mut exP: Arc<DAE::Exp>;
            let mut inv: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut k1: Arc<DAE::Exp>;
            let mut k2: Arc<DAE::Exp>;
            let mut x1: Arc<DAE::Exp>;
            let mut x2: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), (literal!("Y$COS")).clone(), uniqueEqIndex.clone(), idepth.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            inv = Expression::makePureBuiltinCall((literal!("acos")).clone(), list![y.clone()], tp.clone());
            (inv, eqns, vars) = makeTmpEqnAndCrefFromExp(inv.clone(), tp.clone(), (literal!("INV$COS")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            exP = makeInitialGuess(tp.clone(), inExp3.clone(), arg.clone())?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP.clone(), tp.clone(), (literal!("PREX$COS")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            k1 = helpInvCos(inv.clone(), exP.clone(), tp.clone(), true)?;
            k2 = helpInvCos(inv.clone(), exP.clone(), tp.clone(), false)?;
            (k1, eqns, vars) = makeTmpEqnAndCrefFromExp(k1.clone(), tp.clone(), (literal!("k1$COS")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            (k2, eqns, vars) = makeTmpEqnAndCrefFromExp(k2.clone(), tp.clone(), (literal!("k2$COS")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            x1 = helpInvCos2(k1.clone(), inv.clone(), tp.clone(), true)?;
            x2 = helpInvCos2(k2.clone(), inv.clone(), tp.clone(), false)?;
            (x1, eqns, vars) = makeTmpEqnAndCrefFromExp(x1.clone(), tp.clone(), (literal!("x1$COS")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            (x2, eqns, vars) = makeTmpEqnAndCrefFromExp(x2.clone(), tp.clone(), (literal!("x2$COS")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            e1 = helpInvCos3(x1.clone(), x2.clone(), exP.clone(), tp.clone());
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((metamodelica::OrderedFloat(-1.0_f64), true)), Some((metamodelica::OrderedFloat(1.0_f64), true)))?;
            (e1.clone(), true, cons(ass.clone(), eqns.clone()), vars.clone(), idepth.clone() + 1)
        },
        Deref @ "sin" => {
            let mut y: Arc<DAE::Exp>;
            let mut exP: Arc<DAE::Exp>;
            let mut inv: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut k1: Arc<DAE::Exp>;
            let mut k2: Arc<DAE::Exp>;
            let mut x1: Arc<DAE::Exp>;
            let mut x2: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), (literal!("Y$SIN")).clone(), uniqueEqIndex.clone(), idepth.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            inv = Expression::makePureBuiltinCall((literal!("asin")).clone(), list![y.clone()], tp.clone());
            (inv, eqns, vars) = makeTmpEqnAndCrefFromExp(inv.clone(), tp.clone(), (literal!("INV$SIN")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            exP = makeInitialGuess(tp.clone(), inExp3.clone(), arg.clone())?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP.clone(), tp.clone(), (literal!("PREX$SIN")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            k1 = helpInvSin(inv.clone(), arg.clone(), tp.clone(), true)?;
            k2 = helpInvSin(inv.clone(), arg.clone(), tp.clone(), false)?;
            (k1, eqns, vars) = makeTmpEqnAndCrefFromExp(k1.clone(), tp.clone(), (literal!("k1$SIN")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            (k2, eqns, vars) = makeTmpEqnAndCrefFromExp(k2.clone(), tp.clone(), (literal!("k2$SIN")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            x1 = helpInvSin2(k1.clone(), inv.clone(), tp.clone(), true)?;
            x2 = helpInvSin2(k2.clone(), inv.clone(), tp.clone(), false)?;
            (x1, eqns, vars) = makeTmpEqnAndCrefFromExp(x1.clone(), tp.clone(), (literal!("x1$SIN")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            (x2, eqns, vars) = makeTmpEqnAndCrefFromExp(x2.clone(), tp.clone(), (literal!("x2$SIN")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            e1 = helpInvCos3(x1.clone(), x2.clone(), exP.clone(), tp.clone());
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((metamodelica::OrderedFloat(-1.0_f64), true)), Some((metamodelica::OrderedFloat(1.0_f64), true)))?;
            (e1.clone(), true, cons(ass.clone(), eqns.clone()), vars.clone(), idepth.clone() + 1)
        },
        Deref @ "tan" => {
            let mut y: Arc<DAE::Exp>;
            let mut exP: Arc<DAE::Exp>;
            let mut inv: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut k1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            tp = Expression::r#typeof(rhs.clone())?;
            (y, eqns, vars) = makeTmpEqnAndCrefFromExp(rhs.clone(), tp.clone(), (literal!("Y$TAN")).clone(), uniqueEqIndex.clone(), idepth.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            inv = Expression::makePureBuiltinCall((literal!("atan")).clone(), list![y.clone()], tp.clone());
            (inv, eqns, vars) = makeTmpEqnAndCrefFromExp(inv.clone(), tp.clone(), (literal!("INV$TAN")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            exP = makeInitialGuess(tp.clone(), inExp3.clone(), arg.clone())?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP.clone(), tp.clone(), (literal!("PREX$TAN")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqns.clone(), vars.clone(), false)?;
            k1 = Expression::expSub(exP.clone(), inv.clone())?;
            k1 = Expression::makeDiv(k1.clone(), DAE::PI().clone())?;
            k1 = Expression::makePureBuiltinCall((literal!("$_round")).clone(), list![k1.clone()], tp.clone());
            e1 = Expression::expMul(k1.clone(), DAE::PI().clone())?;
            e1 = Expression::expAdd(inv.clone(), e1.clone())?;
            (e1.clone(), true, eqns.clone(), vars.clone(), idepth.clone() + 1)
        },
        Deref @ "abs" => {
            let mut exP: Arc<DAE::Exp>;
            let mut sgn: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(arg.clone())?;
            exP = makeInitialGuess(tp.clone(), inExp3.clone(), arg.clone())?;
            (exP, eqns, vars) = makeTmpEqnAndCrefFromExp(exP.clone(), tp.clone(), (literal!("SIGN$ABS")).clone(), uniqueEqIndex.clone(), idepth.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            sgn = Expression::makePureBuiltinCall((literal!("$_signNoNull")).clone(), list![exP.clone()], tp.clone());
            e1 = Expression::expMul(sgn.clone(), rhs.clone())?;
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((metamodelica::OrderedFloat(0.0_f64), true)), None)?;
            (e1.clone(), true, cons(ass.clone(), eqns.clone()), vars.clone(), idepth.clone() + 1)
        },
        Deref @ "sqrt" => {
            let mut inv: Arc<DAE::Exp>;
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            inv = Expression::expPow(rhs.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((metamodelica::OrderedFloat(0.0_f64), true)), None)?;
            (inv.clone(), true, list![ass.clone()], metamodelica::nil(), idepth.clone() + 1)
        },
        Deref @ "asin" => {
            let mut inv: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall((literal!("sin")).clone(), list![rhs.clone()], tp.clone());
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((-(metamodelica::OrderedFloat(0.5_f64) * Expression::toReal(DAE::PI().clone())?), true)), Some((metamodelica::OrderedFloat(0.5_f64) * Expression::toReal(DAE::PI().clone())?, true)))?;
            (inv.clone(), true, list![ass.clone()], metamodelica::nil(), idepth.clone() + 1)
        },
        Deref @ "acos" => {
            let mut inv: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall((literal!("cos")).clone(), list![rhs.clone()], tp.clone());
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((metamodelica::OrderedFloat(0.0_f64), true)), Some((Expression::toReal(DAE::PI().clone())?, true)))?;
            (inv.clone(), true, list![ass.clone()], metamodelica::nil(), idepth.clone() + 1)
        },
        Deref @ "atan" => {
            let mut inv: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall((literal!("tan")).clone(), list![rhs.clone()], tp.clone());
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((-(metamodelica::OrderedFloat(0.5_f64) * Expression::toReal(DAE::PI().clone())?), true)), Some((metamodelica::OrderedFloat(0.5_f64) * Expression::toReal(DAE::PI().clone())?, true)))?;
            (inv.clone(), true, list![ass.clone()], metamodelica::nil(), idepth.clone() + 1)
        },
        Deref @ "exp" => {
            let mut inv: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ass: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall((literal!("log")).clone(), list![rhs.clone()], tp.clone());
            ass = makeDomainAssert((name.clone()).clone(), rhs.clone(), Some((metamodelica::OrderedFloat(0.0_f64), false)), None)?;
            (inv.clone(), true, list![ass.clone()], metamodelica::nil(), idepth.clone() + 1)
        },
        Deref @ "log" => {
            let mut inv: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(rhs.clone())?;
            inv = Expression::makePureBuiltinCall((literal!("exp")).clone(), list![rhs.clone()], tp.clone());
            (inv.clone(), true, metamodelica::nil(), metamodelica::nil(), idepth.clone() + 1)
        },
        Deref @ "log10" => {
            let mut inv: Arc<DAE::Exp>;
            inv = Expression::expPow(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) }), rhs.clone())?;
            (inv.clone(), true, metamodelica::nil(), metamodelica::nil(), idepth.clone() + 1)
        },
        Deref @ "sign" => {
            (rhs.clone(), false, metamodelica::nil(), metamodelica::nil(), idepth.clone())
        },
        Deref @ "$_DF$DER" => {
            let mut exP: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            e1 = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::symSolverDT)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            exP = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![arg.clone()], Expression::r#typeof(arg.clone())?);
            e1 = Expression::expAdd(Expression::expMul(rhs.clone(), e1.clone())?, exP.clone())?;
            (e1.clone(), true, metamodelica::nil(), metamodelica::nil(), idepth.clone() + 1)
        },
        _ => {
            (rhs.clone(), false, metamodelica::nil(), metamodelica::nil(), idepth.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((result, new_x, newEqns, newVars, odepth))
}

fn simplifyBinaryMulCoeff(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    (exp1, exp2) = (::match_deref::match_deref! { match &(inExp.clone()) {
        e @ Deref @ DAE::Exp::CREF { .. } => {
            (e.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
        },
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::UNARY { exp: coeff, operator: DAE::Operator::UMINUS { .. } }, operator: DAE::Operator::POW { .. }, exp1: e1 } => {
            (e1.clone(), Expression::negate(coeff.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp2: coeff, operator: DAE::Operator::POW { .. }, exp1: e1 } => {
            (e1.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { .. }, exp1: e1 } if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            (e1.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } if (Expression::isOne(e1.clone())) => {
            (e2.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1.0_f64) }))
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } => {
            (e.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }))
        },
        _ => {
            (inExp.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp1, exp2))
}

fn solveQE(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut e3: Arc<DAE::Exp>, mut e4: Arc<DAE::Exp>, mut e5: Arc<DAE::Exp>, mut e6: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>, mut ieqnForNewVars: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inewVarsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut uniqueEqIndex: i32, mut idepth: i32) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut rhs: Arc<DAE::Exp>;
    let mut lhs: Arc<DAE::Exp>;
    let mut eqnForNewVars: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVarsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut e: Arc<DAE::Exp>;
    let mut e7: Arc<DAE::Exp>;
    let mut con: Arc<DAE::Exp>;
    let mut invExp: Arc<DAE::Exp>;
    let mut x1: Arc<DAE::Exp>;
    let mut x2: Arc<DAE::Exp>;
    let mut x: Arc<DAE::Exp>;
    let mut exP: Arc<DAE::Exp>;
    let mut a: Arc<DAE::Exp>;
    let mut b: Arc<DAE::Exp>;
    let mut c: Arc<DAE::Exp>;
    let mut n: Arc<DAE::Exp>;
    let mut sgnb: Arc<DAE::Exp>;
    let mut b2: Arc<DAE::Exp>;
    let mut ac: Arc<DAE::Exp>;
    let mut sExp1: Arc<DAE::Exp>;
    let mut sExp2: Arc<DAE::Exp>;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut b1: bool = false;
    let mut b3: bool = false;
    let false = (Expression::isZero(e1.clone()) && Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
    let true = (ExpressionBasics::expEqual(e2.clone(), e5.clone())?) else { bail!("pattern mismatch") };
    b1 = ExpressionBasics::expEqual(e3.clone(), Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), e6.clone())?)?;
    b3 = ExpressionBasics::expEqual(e6.clone(), Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), e3.clone())?)?;
    let true = (b1.clone() || b3.clone()) else { bail!("pattern mismatch") };
    let false = (expHasCref(e1.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
    let true = (expHasCref(e2.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
    let false = (expHasCref(e3.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
    let false = (expHasCref(e4.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
    let true = (expHasCref(e5.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
    let false = (expHasCref(e6.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
    let false = (expHasCref(inExp2.clone(), inExp3.clone())?) else { bail!("pattern mismatch") };
    a = if (b1.clone()) {e1.clone()} else {e4.clone()};
    b = if (b1.clone()) {e4.clone()} else {e1.clone()};
    c = Expression::negate(inExp2.clone())?;
    n = if (b1.clone()) {e6.clone()} else {e3.clone()};
    tp = Expression::r#typeof(a.clone())?;
    (a, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(a.clone(), tp.clone(), (literal!("a$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), ieqnForNewVars.clone(), inewVarsCrefs.clone(), false)?;
    con = Arc::new(DAE::Exp::RELATION { exp1: a.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
    tp = Expression::r#typeof(b.clone())?;
    (b, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(b.clone(), tp.clone(), (literal!("b$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqnForNewVars.clone(), newVarsCrefs.clone(), false)?;
    sgnb = Expression::makePureBuiltinCall((literal!("$_signNoNull")).clone(), list![b.clone()], tp.clone());
    b2 = Expression::expPow(b.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
    (b2, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(b2.clone(), tp.clone(), (literal!("bPow2$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqnForNewVars.clone(), newVarsCrefs.clone(), false)?;
    tp = Expression::r#typeof(c.clone())?;
    (c, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(c.clone(), tp.clone(), (literal!("c$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqnForNewVars.clone(), newVarsCrefs.clone(), false)?;
    ac = Expression::expMul(a.clone(), c.clone())?;
    ac = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(4.0_f64) }), ac.clone())?;
    sExp1 = Expression::expSub(b2.clone(), ac.clone())?;
    sExp2 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![sExp1.clone()], tp.clone());
    sExp2 = Expression::expMul(sgnb.clone(), sExp2.clone())?;
    a = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: Expression::makeConstOne(tp.clone()), expElse: a.clone() });
    (a, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(a.clone(), tp.clone(), (literal!("a1$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqnForNewVars.clone(), newVarsCrefs.clone(), false)?;
    x1 = Expression::expAdd(b.clone(), sExp2.clone())?;
    x1 = Expression::makeDiv(x1.clone(), a.clone())?;
    x1 = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-0.5_f64) }), x1.clone())?;
    tp = Expression::r#typeof(x1.clone())?;
    x1 = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: Expression::makeConstOne(tp.clone()), expElse: x1.clone() });
    (x1, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(x1.clone(), tp.clone(), (literal!("x1$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqnForNewVars.clone(), newVarsCrefs.clone(), false)?;
    x2 = Expression::expMul(a.clone(), x1.clone())?;
    x2 = Expression::makeDiv(c.clone(), x2.clone())?;
    x2 = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: Expression::makeConstOne(tp.clone()), expElse: x2.clone() });
    x2 = Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: x1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: x2.clone() });
    (x2, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(x2.clone(), tp.clone(), (literal!("x2$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqnForNewVars.clone(), newVarsCrefs.clone(), false)?;
    tp = Expression::r#typeof(e2.clone())?;
    exP = makeInitialGuess(tp.clone(), inExp3.clone(), e2.clone())?;
    (exP, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(exP.clone(), tp.clone(), (literal!("prex$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqnForNewVars.clone(), newVarsCrefs.clone(), false)?;
    x = helpInvCos3(x1.clone(), x2.clone(), exP.clone(), tp.clone());
    (x, eqnForNewVars, newVarsCrefs) = makeTmpEqnAndCrefFromExp(x.clone(), tp.clone(), (literal!("x$QE")).clone(), uniqueEqIndex.clone(), idepth.clone(), eqnForNewVars.clone(), newVarsCrefs.clone(), false)?;
    e7 = Expression::makeDiv(inExp2.clone(), b.clone())?;
    invExp = Expression::inverseFactors(n.clone())?;
    (invExp, _) = ExpressionSimplify::simplify1(invExp.clone())?;
    e7 = Expression::expPow(e7.clone(), invExp.clone())?;
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: e7.clone(), expElse: x.clone() });
    lhs = if (b1.clone()) {Expression::expPow(e2.clone(), e6.clone())?} else {Expression::expPow(e2.clone(), e3.clone())?};
    Ok((rhs, lhs, eqnForNewVars, newVarsCrefs))
}

fn hasOnlyFactors(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (e1.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(Expression::factors(e2.clone())?) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::extractCrefsFromExp(e2.clone())?) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(Expression::factors(e1.clone())?) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::extractCrefsFromExp(e1.clone())?) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn expHasCref(mut inExp1: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((inExp1.clone(), inExp3.clone())) {
        (_, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            Expression::expHasCrefNoPreOrStart(inExp1.clone(), cr.clone())?
        },
        (_, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }) => {
            Expression::expHasDerCref(inExp1.clone(), cr.clone())?
        },
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                println!("{}", (literal!("\n-ExpressionSolve.solve failed:")).clone());
                println!("{}", (literal!(" with respect to: ")).clone());
                println!("{}", (ExpressionBasics::printExpStr(inExp3.clone())?).clone());
                println!("{}", (literal!(" not support!")).clone());
                println!("{}", (literal!("\n")).clone());
            }
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn makeProductLstSort(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut expLstDiv: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expLst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut e: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut op: DAE::Operator;
    if inExpLst.clone().is_empty() {
        outExp = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) });
        return Ok(outExp);
    }
    tp = Expression::r#typeof(listHead(inExpLst.clone())?)?;
    (expLstDiv, expLst) = List::splitOnTrue(inExpLst.clone(), (std::sync::Arc::new(fnptr!(Expression::isDivBinary, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
    outExp = makeProductLstSort2(expLst.clone(), tp.clone())?;
    if !(expLstDiv.clone().is_empty()) {
        expLst2 = metamodelica::nil();
        expLst = metamodelica::nil();
        for mut elem in &*expLstDiv.clone() {
            let mut elem = elem.clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elem.clone()) {
                Deref @ DAE::Exp::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            op = __pa1.clone();
            e2 = __pa2.clone();
            expLst = cons(e1.clone(), expLst.clone());
            expLst2 = cons(e2.clone(), expLst2.clone());
        }
        if !(expLst2.clone().is_empty()) {
            e = makeProductLstSort(expLst2.clone())?;
            if !(Expression::isOne(e.clone())) {
                outExp = Expression::makeDiv(outExp.clone(), e.clone())?;
            }
        }
        if !(expLst.clone().is_empty()) {
            e = makeProductLstSort(expLst.clone())?;
            outExp = Expression::expMul(outExp.clone(), e.clone())?;
        }
    }
    Ok(outExp)
}

fn makeProductLstSort2(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = Expression::makeConstOne(tp.clone());
    let mut rest: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    rest = ExpressionSimplify::simplifyList(inExpLst.clone())?;
    for mut elem in &*rest.clone() {
        let mut elem = elem.clone();
        if !(Expression::isOne(elem.clone())) {
            outExp = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: Expression::expMul(outExp.clone(), e2.clone())?, expElse: Expression::expMul(outExp.clone(), e3.clone())? })
        },
        _ => {
            Expression::expMul(outExp.clone(), elem.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
    }
    Ok(outExp)
}

fn makeTmpEqnAndCrefFromExp(mut iExp: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>, mut name: ArcStr, mut index1: i32, mut index2: i32, mut ieqnForNewVars: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inewVarsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut need: bool) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut oExp: Arc<DAE::Exp>;
    let mut oeqnForNewVars: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut onewVarsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    (oExp, _) = ExpressionSimplify::simplify1(iExp.clone())?;
    if need.clone() || !(Expression::isCref(oExp.clone()) || Expression::isConst(oExp.clone())?) {
        cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$TMP$VAR$")); __mm_s.push_str(&*intString(index1.clone())); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*intString(index2.clone())); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), tp.clone(), metamodelica::nil());
        eqn = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr.clone(), exp: oExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
        oExp = Expression::crefExp(cr.clone())?;
        oeqnForNewVars = cons(eqn.clone(), ieqnForNewVars.clone());
        onewVarsCrefs = cons(cr.clone(), inewVarsCrefs.clone());
    } else {
        oeqnForNewVars = ieqnForNewVars.clone();
        onewVarsCrefs = inewVarsCrefs.clone();
    }
    Ok((oExp, oeqnForNewVars, onewVarsCrefs))
}

fn makeDomainAssert(mut name: ArcStr, mut rhs: Arc<DAE::Exp>, mut lowerBound: Option<(metamodelica::Real, bool)>, mut upperBound: Option<(metamodelica::Real, bool)>) -> Result<Arc<BackendDAE::Equation>> {
    let mut assEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut msg: ArcStr = arcstr::literal!("");
    let mut cond: Arc<DAE::Exp>;
    let mut algo: Arc<DAE::Algorithm>;
    let mut tp: Arc<DAE::Type> = Expression::r#typeof(rhs.clone())?;
    (msg, cond) = (match (lowerBound.clone(), upperBound.clone()) {
        (Some((mut lower, true)), Some((mut upper, true))) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut l: Arc<DAE::Exp>;
            let mut u: Arc<DAE::Exp>;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Model error: Result of ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" outside the range ")); __mm_s.push_str(&*realString(lower.clone())); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*realString(upper.clone())); __mm_s.push_str(&*literal!(". Unable to invert.")); ArcStr::from(__mm_s) }).clone();
            l = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::RCONST { real: lower.clone() }), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: rhs.clone(), index: -1, optionExpisASUB: None });
            u = Arc::new(DAE::Exp::RELATION { exp1: rhs.clone(), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: upper.clone() }), index: -1, optionExpisASUB: None });
            (r#str.clone(), Arc::new(DAE::Exp::LBINARY { exp1: l.clone(), operator: DAE::Operator::AND { ty: tp.clone() }, exp2: u.clone() }))
        },
        (Some((mut lower, true)), Some((mut upper, false))) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut l: Arc<DAE::Exp>;
            let mut u: Arc<DAE::Exp>;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Model error: Result of ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" outside the range ")); __mm_s.push_str(&*realString(lower.clone())); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" < ")); __mm_s.push_str(&*realString(upper.clone())); __mm_s.push_str(&*literal!(". Unable to invert.")); ArcStr::from(__mm_s) }).clone();
            l = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::RCONST { real: lower.clone() }), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: rhs.clone(), index: -1, optionExpisASUB: None });
            u = Arc::new(DAE::Exp::RELATION { exp1: rhs.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: upper.clone() }), index: -1, optionExpisASUB: None });
            (r#str.clone(), Arc::new(DAE::Exp::LBINARY { exp1: l.clone(), operator: DAE::Operator::AND { ty: tp.clone() }, exp2: u.clone() }))
        },
        (Some((mut lower, false)), Some((mut upper, true))) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut l: Arc<DAE::Exp>;
            let mut u: Arc<DAE::Exp>;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Model error: Result of ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" outside the range ")); __mm_s.push_str(&*realString(lower.clone())); __mm_s.push_str(&*literal!(" < ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*realString(upper.clone())); __mm_s.push_str(&*literal!(". Unable to invert.")); ArcStr::from(__mm_s) }).clone();
            l = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::RCONST { real: lower.clone() }), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: rhs.clone(), index: -1, optionExpisASUB: None });
            u = Arc::new(DAE::Exp::RELATION { exp1: rhs.clone(), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: upper.clone() }), index: -1, optionExpisASUB: None });
            (r#str.clone(), Arc::new(DAE::Exp::LBINARY { exp1: l.clone(), operator: DAE::Operator::AND { ty: tp.clone() }, exp2: u.clone() }))
        },
        (Some((mut lower, false)), Some((mut upper, false))) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut l: Arc<DAE::Exp>;
            let mut u: Arc<DAE::Exp>;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Model error: Result of ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" outside the range ")); __mm_s.push_str(&*realString(lower.clone())); __mm_s.push_str(&*literal!(" < ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" < ")); __mm_s.push_str(&*realString(upper.clone())); __mm_s.push_str(&*literal!(". Unable to invert.")); ArcStr::from(__mm_s) }).clone();
            l = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::RCONST { real: lower.clone() }), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: rhs.clone(), index: -1, optionExpisASUB: None });
            u = Arc::new(DAE::Exp::RELATION { exp1: rhs.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: upper.clone() }), index: -1, optionExpisASUB: None });
            (r#str.clone(), Arc::new(DAE::Exp::LBINARY { exp1: l.clone(), operator: DAE::Operator::AND { ty: tp.clone() }, exp2: u.clone() }))
        },
        (Some((mut lower, true)), None) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut l: Arc<DAE::Exp>;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Model error: Result of ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" should be ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" >= ")); __mm_s.push_str(&*realString(lower.clone())); __mm_s.push_str(&*literal!(". Unable to invert.")); ArcStr::from(__mm_s) }).clone();
            l = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::RCONST { real: lower.clone() }), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: rhs.clone(), index: -1, optionExpisASUB: None });
            (r#str.clone(), l.clone())
        },
        (Some((mut lower, true)), None) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut l: Arc<DAE::Exp>;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Model error: Result of ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" should be ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" > ")); __mm_s.push_str(&*realString(lower.clone())); __mm_s.push_str(&*literal!(". Unable to invert.")); ArcStr::from(__mm_s) }).clone();
            l = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::RCONST { real: lower.clone() }), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: rhs.clone(), index: -1, optionExpisASUB: None });
            (r#str.clone(), l.clone())
        },
        (None, Some((mut upper, true))) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut u: Arc<DAE::Exp>;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Model error: Result of ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" should be ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*realString(upper.clone())); __mm_s.push_str(&*literal!(". Unable to invert.")); ArcStr::from(__mm_s) }).clone();
            u = Arc::new(DAE::Exp::RELATION { exp1: rhs.clone(), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: upper.clone() }), index: -1, optionExpisASUB: None });
            (r#str.clone(), u.clone())
        },
        (None, Some((mut upper, false))) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut u: Arc<DAE::Exp>;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Model error: Result of ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" should be ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" < ")); __mm_s.push_str(&*realString(upper.clone())); __mm_s.push_str(&*literal!(". Unable to invert.")); ArcStr::from(__mm_s) }).clone();
            u = Arc::new(DAE::Exp::RELATION { exp1: rhs.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: upper.clone() }), index: -1, optionExpisASUB: None });
            (r#str.clone(), u.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    algo = Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_ASSERT { cond: cond.clone(), msg: Arc::new(DAE::Exp::SCONST { string: (msg.clone()).clone() }), level: DAE::ASSERTIONLEVEL_ERROR().clone(), source: DAE::emptyElementSource().clone() })] });
    assEq = Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: algo.clone(), source: DAE::emptyElementSource().clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
    Ok(assEq)
}

fn makeInitialGuess(mut tp: Arc<DAE::Type>, mut iExp1: Arc<DAE::Exp>, mut iExp2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp>;
    let mut con: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    con = Expression::makePureBuiltinCall((literal!("initial")).clone(), metamodelica::nil(), tp.clone());
    (e, _) = Expression::traverseExpBottomUp(iExp2.clone(), (std::sync::Arc::new(makeInitialGuess2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, bool)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, bool))> + 'static>), (iExp1.clone(), literal!("pre"), tp.clone(), true))?;
    (oExp, _) = Expression::traverseExpBottomUp(iExp2.clone(), (std::sync::Arc::new(makeInitialGuess2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, bool)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, bool))> + 'static>), (iExp1.clone(), literal!("pre"), tp.clone(), false))?;
    oExp = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: e.clone(), expElse: oExp.clone() });
    Ok(oExp)
}

fn makeInitialGuess2(mut iExp: Arc<DAE::Exp>, mut itpl: (Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, bool)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, bool))> {
    let mut oExp: Arc<DAE::Exp>;
    let mut otpl: (Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, bool) = itpl.clone();
    oExp = (::match_deref::match_deref! { match &((iExp.clone(), itpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, (Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, fun, tp, _)) if (ComponentReferenceBasics::crefEqual(cr1.clone(), cr2.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::makePureBuiltinCall((fun.clone()).clone(), list![iExp.clone()], tp.clone());
            e.clone()
        },
        (_, (_, _, tp, true)) => {
            let mut e: Arc<DAE::Exp>;
            match '__try0: {
                let __pa1 = ::match_deref::match_deref! { match &(makeInitialGuess3(iExp.clone(), tp.clone())) {
                    Some(__pa1) => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                e = __pa1.clone();
                Ok::<_, anyhow::Error>((e.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    e = __try0_o0;
                }
                Err(_) => {
                    e = iExp.clone();
                }
            }
            e.clone()
        },
        _ => {
            iExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oExp, otpl))
}

fn makeInitialGuess3(mut iExp: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>) -> Option<Arc<DAE::Exp>> {
    let mut oExp: Option<Arc<DAE::Exp>> = None;
    oExp = (::match_deref::match_deref! { match &(iExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } => {
            let mut con: Arc<DAE::Exp>;
            let mut o: Arc<DAE::Exp>;
            con = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            o = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: Arc::new(DAE::Exp::RCONST { real: -(metamodelica::OrderedFloat((1) as f64) / metamodelica::OrderedFloat(0.000000001_f64)) }), expElse: iExp.clone() });
            Some(o.clone())
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. } => {
            let mut con: Arc<DAE::Exp>;
            let mut o: Arc<DAE::Exp>;
            con = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            o = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: Arc::new(DAE::Exp::RCONST { real: -(metamodelica::OrderedFloat((1) as f64) / metamodelica::OrderedFloat(0.000000001_f64)) }), expElse: iExp.clone() });
            Some(o.clone())
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } => {
            let mut con: Arc<DAE::Exp>;
            let mut o: Arc<DAE::Exp>;
            con = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            o = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: iExp.clone() });
            Some(o.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e, .. } => {
            let mut con: Arc<DAE::Exp>;
            let mut o: Arc<DAE::Exp>;
            con = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            o = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), expElse: iExp.clone() });
            Some(o.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oExp
}

fn helpInvCos(mut acosy: Arc<DAE::Exp>, mut x: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>, mut neg: bool) -> Result<Arc<DAE::Exp>> {
    let mut k: Arc<DAE::Exp>;
    k = if (neg.clone()) {Expression::expAdd(x.clone(), acosy.clone())?} else {Expression::expSub(x.clone(), acosy.clone())?};
    k = Expression::makeDiv(k.clone(), Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), DAE::PI().clone())?)?;
    k = Expression::makePureBuiltinCall((literal!("$_round")).clone(), list![k.clone()], tp.clone());
    Ok(k)
}

fn helpInvSin(mut asiny: Arc<DAE::Exp>, mut x: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>, mut neg: bool) -> Result<Arc<DAE::Exp>> {
    let mut k: Arc<DAE::Exp>;
    k = if (neg.clone()) {Expression::expAdd(x.clone(), asiny.clone())?} else {Expression::expSub(x.clone(), asiny.clone())?};
    k = Expression::makeDiv(k.clone(), Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), DAE::PI().clone())?)?;
    if neg.clone() {
        k = Expression::expSub(k.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }))?;
    }
    k = Expression::makePureBuiltinCall((literal!("$_round")).clone(), list![k.clone()], tp.clone());
    Ok(k)
}

fn helpInvCos2(mut k: Arc<DAE::Exp>, mut acosy: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>, mut neg: bool) -> Result<Arc<DAE::Exp>> {
    let mut x: Arc<DAE::Exp>;
    x = if (neg.clone()) {Expression::negate(acosy.clone())?} else {acosy.clone()};
    x = Expression::expAdd(x.clone(), Expression::expMul(k.clone(), Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), DAE::PI().clone())?)?)?;
    Ok(x)
}

fn helpInvSin2(mut k: Arc<DAE::Exp>, mut asiny: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>, mut neg: bool) -> Result<Arc<DAE::Exp>> {
    let mut x: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    x = if (neg.clone()) {Expression::negate(asiny.clone())?} else {asiny.clone()};
    e = Expression::expMul(k.clone(), Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), DAE::PI().clone())?)?;
    e = if (neg.clone()) {Expression::expAdd(e.clone(), DAE::PI().clone())?} else {e.clone()};
    x = Expression::expAdd(x.clone(), e.clone())?;
    Ok(x)
}

fn helpInvCos3(mut x1: Arc<DAE::Exp>, mut x2: Arc<DAE::Exp>, mut x: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>) -> Arc<DAE::Exp> {
    let mut y: Arc<DAE::Exp>;
    let mut diffx1: Arc<DAE::Exp> = absDiff(x1.clone(), x.clone(), tp.clone()).unwrap();
    let mut diffx2: Arc<DAE::Exp> = absDiff(x2.clone(), x.clone(), tp.clone()).unwrap();
    let mut con: Arc<DAE::Exp> = Arc::new(DAE::Exp::RELATION { exp1: diffx1.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: diffx2.clone(), index: -1, optionExpisASUB: None });
    con = Expression::makeNoEvent(con.clone());
    y = Arc::new(DAE::Exp::IFEXP { expCond: con.clone(), expThen: x1.clone(), expElse: x2.clone() });
    y
}

fn absDiff(mut x: Arc<DAE::Exp>, mut y: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut z: Arc<DAE::Exp>;
    z = Expression::expSub(x.clone(), y.clone())?;
    z = Expression::makePureBuiltinCall((literal!("abs")).clone(), list![z.clone()], tp.clone());
    Ok(z)
}

