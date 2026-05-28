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

use crate::SimCodeFunction;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTableStringToPath;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend::Mod;
use openmodelica_frontend::Patternm;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_program_util::ProgramUtil;
use openmodelica_util::Autoconf;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Graph;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn elementVars(mut ild: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> {
    let mut vars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
    let mut ld: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    ld = List::filterOnTrue(ild.clone(), (std::sync::Arc::new(fnptr!(isVarQ, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
    vars = List::map(ld.clone(), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
    vars
}

pub fn crefSubIsScalar(mut cref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut isScalar: bool = false;
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    subs = ComponentReferenceBasics::crefSubs(cref.clone())?;
    isScalar = subsToScalar(subs.clone())?;
    Ok(isScalar)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn subsToScalar(mut inExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inExpSubscriptLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { .. }, tail: _ } => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: _ } => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: r } => {
            let mut b: bool = false;
            b = subsToScalar(r.clone())?;
            b.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBoolean)
}

pub fn crefNoSub(mut cref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut noSub: bool = false;
    noSub = !(ComponentReference::crefHaveSubs(cref.clone())?);
    Ok(noSub)
}

pub fn inFunctionContext(mut inContext: SimCodeFunction::Context) -> bool {
    let mut outInFunction: bool = false;
    outInFunction = (match inContext.clone() {
        SimCodeFunction::Context::FUNCTION_CONTEXT { .. } => true,
        _ => false,
    });
    outInFunction
}

pub fn crefIsScalar(mut cref: Arc<DAE::ComponentRef>, mut context: SimCodeFunction::Context) -> Result<bool> {
    let mut isScalar: bool = false;
    if inFunctionContext(context.clone()) {
        isScalar = ComponentReference::crefLastSubs(cref.clone())?.is_empty();
    } else if Flags::isSet(Flags::NF_SCALARIZE.clone())? {
        isScalar = ComponentReference::crefHasScalarSubscripts(cref.clone())?;
    } else {
        isScalar = !(ComponentReference::crefHaveSubs(cref.clone())?);
    }
    Ok(isScalar)
}

pub fn buildCrefExpFromAsub(mut cref: Arc<DAE::Exp>, mut subs: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut cRefOut: Arc<DAE::Exp>;
    cRefOut = 'mc: {
        let __mc_input = (cref.clone(), subs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty, componentRef: crNew }, _) => {
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut indexes: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut crNew = (*crNew).clone();
                    indexes = List::map(subs.clone(), (std::sync::Arc::new(fnptr!(Expression::makeIndexSubscript, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Subscript>> + 'static>));
                    crNew = ComponentReference::subscriptCref(crNew.clone(), indexes.clone())?;
                    crefExp = Expression::makeCrefExp(crNew.clone(), ty.clone())?;
                    Ok(crefExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cRefOut)
}

pub fn buildCrefExpFromSubs(mut cref: Arc<DAE::Exp>, mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Exp>> {
    let mut cRefOut: Arc<DAE::Exp>;
    cRefOut = 'mc: {
        let __mc_input = (cref.clone(), subs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty, componentRef: crNew }, _) => {
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut crNew = (*crNew).clone();
                    crNew = ComponentReference::subscriptCref(crNew.clone(), subs.clone())?;
                    crefExp = Expression::makeCrefExp(crNew.clone(), ty.clone())?;
                    Ok(crefExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cRefOut)
}

pub fn incrementInt(mut inInt: i32, mut increment: i32) -> i32 {
    let mut outInt: i32 = 0;
    outInt = inInt.clone() + increment.clone();
    outInt
}

pub fn decrementInt(mut inInt: i32, mut decrement: i32) -> i32 {
    let mut outInt: i32 = 0;
    outInt = inInt.clone() - decrement.clone();
    outInt
}

pub fn protectedVars(mut InSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Arc<metamodelica::List<SimCodeVar::SimVar>> {
    let mut OutSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    OutSimVars = List::filterOnTrue(InSimVars.clone(), (std::sync::Arc::new(isNotProtected) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<bool> + 'static>));
    OutSimVars
}

fn isNotProtected(mut simVar: SimCodeVar::SimVar) -> Result<bool> {
    let mut isProtected: bool = false;
    let SimCodeVar::SIMVAR { isProtected: __pa0, .. } = (simVar.clone()) else { bail!("pattern mismatch") };
    isProtected = __pa0.clone();
    isProtected = !(isProtected.clone());
    Ok(isProtected)
}

pub fn makeCrefRecordExp(mut inCRefRecord: Arc<DAE::ComponentRef>, mut inVar: Arc<DAE::Var>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inCRefRecord.clone(), inVar.clone())) {
        (cr, Deref @ DAE::Var { ty: tp, name, .. }) => {
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (name.clone()).clone(), metamodelica::nil(), tp.clone())?;
            outExp = Expression::makeCrefExp(cr1.clone(), tp.clone())?;
            outExp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn splitRecordAssignmentToMemberAssignments(mut lhs_cref: Arc<DAE::ComponentRef>, mut lhs_type: Arc<DAE::Type>, mut rhs_cref_str: ArcStr) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outAssigns: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut rhs_cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outAssigns = metamodelica::nil();
    rhs_cref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (rhs_cref_str.clone()).clone(), identType: lhs_type.clone(), subscriptLst: metamodelica::nil() });
    let () = (::match_deref::match_deref! { match &(lhs_type.clone()) {
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            let mut l_v_exp: Arc<DAE::Exp>;
            let mut r_v_exp: Arc<DAE::Exp>;
            let mut stmt: Arc<DAE::Statement>;
            for mut v in &*var_field!((*lhs_type).varLst, DAE::Type::T_COMPLEX).clone() {
                let mut v = v.clone();
                l_v_exp = makeCrefRecordExp(lhs_cref.clone(), v.clone())?;
                r_v_exp = makeCrefRecordExp(rhs_cref.clone(), v.clone())?;
                if Types::isArray(v.ty.clone()) {
                    stmt = Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: v.ty.clone(), lhs: l_v_exp.clone(), exp: r_v_exp.clone(), source: DAE::emptyElementSource().clone() });
                } else {
                    stmt = Arc::new(DAE::Statement::STMT_ASSIGN { type_: v.ty.clone(), exp1: l_v_exp.clone(), exp: r_v_exp.clone(), source: DAE::emptyElementSource().clone() });
                }
                outAssigns = cons(stmt.clone(), outAssigns.clone());
            }
            outAssigns = outAssigns.clone().reverse();
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAssigns)
}

pub fn derComponentRef(mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut derCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    derCref = ComponentReference::crefPrefixDer(inCref.clone());
    derCref
}

pub fn hackArrayReverseToCref(mut inExp: Arc<DAE::Exp>, mut context: SimCodeFunction::Context) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: aRest }, scalar: true, ty: aty } => {
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    if '__try0: {
                        let SimCodeFunction::FUNCTION_CONTEXT { .. } = (context.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(cr.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } }, tail: Deref @ metamodelica::List::Nil } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    let true = (isArrayExpansion(aRest.clone(), cr.clone(), 2)?) else { bail!("pattern mismatch") };
                    crefExp = Expression::makeCrefExp(cr.clone(), aty.clone())?;
                    Ok(crefExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isArrayExpansion(mut inArrayElems: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCref: Arc<DAE::ComponentRef>, mut index: i32) -> Result<bool> {
    let mut isExpanded: bool = false;
    isExpanded = 'mc: {
        let __mc_input = inArrayElems.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: aRest } => {
                    let mut i: i32 = 0;
                    let mut cr = (*cr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(cr.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa0 } }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    let true = (i.clone() == index.clone()) else { bail!("pattern mismatch") };
                    cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(inCref.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(isArrayExpansion(aRest.clone(), inCref.clone(), index.clone() + 1)?)
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
    Ok(isExpanded)
}

pub fn hackMatrixReverseToCref(mut inExp: Arc<DAE::Exp>, mut context: SimCodeFunction::Context) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: rows @ Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: _ }, tail: _ }, ty: aty, .. } => {
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    if '__try0: {
                        let SimCodeFunction::FUNCTION_CONTEXT { .. } = (context.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(cr.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } }, tail: Deref @ metamodelica::List::Nil } } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    let true = (isMatrixExpansion(rows.clone(), cr.clone(), 1, 1)?) else { bail!("pattern mismatch") };
                    crefExp = Expression::makeCrefExp(cr.clone(), aty.clone())?;
                    Ok(crefExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isMatrixExpansion(mut rows: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inCref: Arc<DAE::ComponentRef>, mut rowIndex: i32, mut colIndex: i32) -> Result<bool> {
    let mut isExpanded: bool = false;
    isExpanded = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Nil, tail: restRows } => {
                    Ok(isMatrixExpansion(restRows.clone(), inCref.clone(), rowIndex.clone() + 1, 1)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: restElems }, tail: restRows } => {
                    let mut r: i32 = 0;
                    let mut c: i32 = 0;
                    let mut cr = (*cr).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(cr.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa0 } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa1 } }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r = __pa0.clone();
                    c = __pa1.clone();
                    let true = (r.clone() == rowIndex.clone() && c.clone() == colIndex.clone()) else { bail!("pattern mismatch") };
                    cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(inCref.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(isMatrixExpansion(cons(restElems.clone(), restRows.clone()), inCref.clone(), rowIndex.clone(), colIndex.clone() + 1)?)
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
    Ok(isExpanded)
}

pub fn hackGetFirstExternalFunctionLib(mut libs: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut outFirstLib: ArcStr = arcstr::literal!("");
    outFirstLib = ('mc: {
        let __mc_input = libs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lib: ArcStr = arcstr::literal!("");
                    lib = (List::last(libs.clone())?).clone();
                    lib = (System::stringReplace((lib.clone()).clone(), (literal!("-l")).clone(), (literal!("")).clone())?).clone();
                    Ok(lib.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("NO_LIB"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outFirstLib)
}

pub fn createAssertforSqrt(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        _ => {
            (outExp, _) = ExpressionSimplify::simplify(Arc::new(DAE::Exp::RELATION { exp1: inExp.clone(), operator: DAE::Operator::GREATEREQ { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }))?;
            outExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn createDAEString(mut inString: ArcStr) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::SCONST { string: (inString.clone()).clone() });
    outExp
}

/* end of TypeView published functions */
// =============================================================================
// section to generate SimCode from functions
//
// Finds the called functions in BackendDAE and transforms them to a list of
// libraries and a list of SimCodeFunction.Function uniontypes.
// =============================================================================
fn orderRecordDecls(mut decl1: SimCodeFunction::RecordDeclaration, mut decl2: SimCodeFunction::RecordDeclaration) -> Result<bool> {
    let mut b: bool = false;
    b = (match (decl1.clone(), decl2.clone()) {
        (SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: ref path1, .. }, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: ref path2, .. }) => {
            AbsynUtil::pathGe(path1.clone(), path2.clone())?
        },
        _ => {
            true
        },
    });
    Ok(b)
}

pub fn elaborateFunctions(mut program: Absyn::Program, mut daeElements: Arc<metamodelica::List<DAE::Function>>, mut metarecordTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut literals: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut includes: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut outIncludes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libpaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    let mut g: Arc<metamodelica::List<(SimCodeFunction::RecordDeclaration, Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>)>> = metamodelica::nil();
    let mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>;
    recDeclsMap = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    (functions, outIncludes, includeDirs, libs, libpaths) = elaborateFunctions2(program.clone(), daeElements.clone(), metamodelica::nil(), includes.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), recDeclsMap.clone())?;
    collectRecDeclsFromMetaRecCallExps(literals.clone(), recDeclsMap.clone())?;
    collectRecDeclsFromTypes(metarecordTypes.clone(), recDeclsMap.clone())?;
    recordDecls = UnorderedMap::valueList(recDeclsMap.clone());
    recordDecls = List::sort(recordDecls.clone(), (std::sync::Arc::new(orderRecordDecls) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, SimCodeFunction::RecordDeclaration) -> Result<bool> + 'static>))?;
    ht = HashTableStringToPath::emptyHashTableSized(BaseHashTable::lowBucketSize.clone());
    (recordDecls, _) = List::mapFold(recordDecls.clone(), (std::sync::Arc::new(aliasRecordDeclarations) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr))) -> Result<(SimCodeFunction::RecordDeclaration, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)))> + 'static>), ht.clone());
    g = Graph::buildGraph(recordDecls.clone(), (std::sync::Arc::new(fnptr!(getRecordDependencies, SimCodeFunction::RecordDeclaration, Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>)) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Result<Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>> + 'static>), recordDecls.clone());
    let __pa0 = ::match_deref::match_deref! { match &(Graph::topologicalSort(g.clone(), (std::sync::Arc::new(fnptr!(isRecordDeclEqual, SimCodeFunction::RecordDeclaration, SimCodeFunction::RecordDeclaration)) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, SimCodeFunction::RecordDeclaration) -> Result<bool> + 'static>))?) {
        (__pa0, Deref @ metamodelica::List::Nil) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    recordDecls = __pa0.clone();
    Ok((functions, recordDecls, outIncludes, includeDirs, libs, libpaths))
}

fn getRecordDependencies(mut decl: SimCodeFunction::RecordDeclaration, mut allDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> {
    let mut dependencies: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    dependencies = (match decl.clone() {
        SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { aliasName: Some(mut name), .. } => {
            List::select1(allDecls.clone(), (std::sync::Arc::new(fnptr!(recordDeclHasName, SimCodeFunction::RecordDeclaration, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, ArcStr) -> Result<bool> + 'static>), (name.clone()).clone())
        },
        SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR { name: mut name, .. } => {
            List::select1(allDecls.clone(), (std::sync::Arc::new(fnptr!(recordDeclHasName, SimCodeFunction::RecordDeclaration, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, ArcStr) -> Result<bool> + 'static>), (name.clone()).clone())
        },
        SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { variables: ref vars, .. } => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut tyss: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Type>>>>> = metamodelica::nil();
            tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = getVarType(v.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            tyss = List::map1(tys.clone(), (std::sync::Arc::new(fnptr!(Types::getAllInnerTypesOfType, Arc<DAE::Type>, Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> + 'static>), std::sync::Arc::new(fnptr!(Util::anyReturnTrue, _)));
            tys = List::flatten(tyss.clone());
            dependencies = List::filterMap1(tys.clone(), (std::sync::Arc::new(getRecordDependenciesFromType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Result<SimCodeFunction::RecordDeclaration> + 'static>), allDecls.clone());
            List::unique(dependencies.clone())
        },
        _ => {
            metamodelica::nil()
        },
    });
    dependencies
}

fn getVarType(mut var: Arc<SimCodeFunction::Variable::Variable>) -> Arc<DAE::Type> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ SimCodeFunction::Variable::VARIABLE { ty, .. } => ty.clone(),
        _ => DAE::T_ANYTYPE_DEFAULT().clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

fn getRecordDependenciesFromType(mut ty: Arc<DAE::Type>, mut allDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Result<SimCodeFunction::RecordDeclaration> {
    let mut decl: SimCodeFunction::RecordDeclaration;
    let mut path: Arc<Absyn::Path>;
    let mut name: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    name = (AbsynUtil::pathStringUnquoteReplaceDot(path.clone(), (literal!("_")).clone())?).clone();
    decl = List::find1(allDecls.clone(), (std::sync::Arc::new(fnptr!(recordDeclHasName, SimCodeFunction::RecordDeclaration, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, ArcStr) -> Result<bool> + 'static>), (name.clone()).clone())?;
    Ok(decl)
}

fn recordDeclHasName(mut decl: SimCodeFunction::RecordDeclaration, mut name: ArcStr) -> bool {
    let mut b: bool = false;
    b = (match decl.clone() {
        SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { .. } => stringEq((name.clone()).clone(), (var_field!(decl.name, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL).clone()).clone()),
        _ => false,
    });
    b
}

fn isRecordDeclEqual(mut decl1: SimCodeFunction::RecordDeclaration, mut decl2: SimCodeFunction::RecordDeclaration) -> bool {
    let mut b: bool = false;
    b = (match (decl1.clone(), decl2.clone()) {
        (SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { .. }, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { .. }) => stringEq((var_field!(decl1.name, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL).clone()).clone(), (var_field!(decl2.name, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL).clone()).clone()),
        (SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { .. }, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { .. }) => AbsynUtil::pathEqual(var_field!(decl1.path, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF).clone(), var_field!(decl2.path, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF).clone()),
        _ => false,
    });
    b
}

fn elaborateFunctions2(mut program: Absyn::Program, mut daeElements: Arc<metamodelica::List<DAE::Function>>, mut inFunctions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, mut inIncludes: Arc<metamodelica::List<ArcStr>>, mut inIncludeDirs: Arc<metamodelica::List<ArcStr>>, mut inLibs: Arc<metamodelica::List<ArcStr>>, mut inPaths: Arc<metamodelica::List<ArcStr>>, mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<(Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outFunctions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut outIncludes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outIncludeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLibsPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outFunctions, outIncludes, outIncludeDirs, outLibs, outLibsPaths) = (::match_deref::match_deref! { match &((daeElements.clone(), inFunctions.clone(), inIncludes.clone(), inIncludeDirs.clone(), inLibs.clone(), inPaths.clone())) {
        (Deref @ metamodelica::List::Nil, accfns, includes, includeDirs, libs, libPaths) => {
            (accfns.clone().reverse(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone())
        },
        (Deref @ metamodelica::List::Cons { head: DAE::Function::FUNCTION { type_: Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR, .. }, .. }, .. }, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
            let mut fns: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            let mut includes = (*includes).clone();
            let mut includeDirs = (*includeDirs).clone();
            let mut libs = (*libs).clone();
            let mut libPaths = (*libPaths).clone();
            (fns, includes, includeDirs, libs, libPaths) = elaborateFunctions2(program.clone(), rest.clone(), accfns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
            (fns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone())
        },
        (Deref @ metamodelica::List::Cons { head: DAE::Function::FUNCTION { partialPrefix: true, .. }, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
            let mut fns: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            let mut includes = (*includes).clone();
            let mut includeDirs = (*includeDirs).clone();
            let mut libs = (*libs).clone();
            let mut libPaths = (*libPaths).clone();
            (fns, includes, includeDirs, libs, libPaths) = elaborateFunctions2(program.clone(), rest.clone(), accfns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
            (fns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone())
        },
        (Deref @ metamodelica::List::Cons { head: fel @ DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { language: Deref @ "builtin", name, .. }, .. }, tail: _ }, path, .. }, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
            let mut b: bool = false;
            let mut fns: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            let mut r#fn: Arc<SimCodeFunction::Function::Function>;
            let mut fname: ArcStr = arcstr::literal!("");
            let mut includes = (*includes).clone();
            let mut includeDirs = (*includeDirs).clone();
            let mut libs = (*libs).clone();
            let mut libPaths = (*libPaths).clone();
            fname = (AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(path.clone()), (literal!(".")).clone(), true, false)?).clone();
            b = stringEq((fname.clone()).clone(), (name.clone()).clone());
            if !(b.clone()) {
                (r#fn, includes, includeDirs, libs, libPaths) = elaborateFunction(program.clone(), fel.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
            }
            (fns, includes, includeDirs, libs, libPaths) = elaborateFunctions2(program.clone(), rest.clone(), List::consOnTrue(!(b.clone()), r#fn.clone(), accfns.clone()), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
            (fns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone())
        },
        (Deref @ metamodelica::List::Cons { head: fel @ DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { language: Deref @ "C", name, .. }, .. }, tail: _ }, path, .. }, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
            let mut b: bool = false;
            let mut fns: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            let mut r#fn: Arc<SimCodeFunction::Function::Function>;
            let mut fname: ArcStr = arcstr::literal!("");
            let mut includes = (*includes).clone();
            let mut includeDirs = (*includeDirs).clone();
            let mut libs = (*libs).clone();
            let mut libPaths = (*libPaths).clone();
            fname = (AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(path.clone()), (literal!(".")).clone(), true, false)?).clone();
            b = listMember((name.clone()).clone(), SCodeUtil::knownExternalCFunctions.clone()) && stringEq((fname.clone()).clone(), (name.clone()).clone());
            if !(b.clone()) {
                (r#fn, includes, includeDirs, libs, libPaths) = elaborateFunction(program.clone(), fel.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
            }
            (fns, includes, includeDirs, libs, libPaths) = elaborateFunctions2(program.clone(), rest.clone(), List::consOnTrue(!(b.clone()), r#fn.clone(), accfns.clone()), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
            (fns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone())
        },
        (Deref @ metamodelica::List::Cons { head: fel, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
            let mut fns: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            let mut r#fn: Arc<SimCodeFunction::Function::Function>;
            let mut includes = (*includes).clone();
            let mut includeDirs = (*includeDirs).clone();
            let mut libs = (*libs).clone();
            let mut libPaths = (*libPaths).clone();
            (r#fn, includes, includeDirs, libs, libPaths) = elaborateFunction(program.clone(), fel.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
            (fns, includes, includeDirs, libs, libPaths) = elaborateFunctions2(program.clone(), rest.clone(), cons(r#fn.clone(), accfns.clone()), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
            (fns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outFunctions, outIncludes, outIncludeDirs, outLibs, outLibsPaths))
}

/* Does the actual work of transforming a DAE.FUNCTION to a SimCodeFunction.Function. */
fn elaborateFunction(mut program: Absyn::Program, mut inElement: DAE::Function, mut inIncludes: Arc<metamodelica::List<ArcStr>>, mut inIncludeDirs: Arc<metamodelica::List<ArcStr>>, mut inLibs: Arc<metamodelica::List<ArcStr>>, mut inLibPaths: Arc<metamodelica::List<ArcStr>>, mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<(Arc<SimCodeFunction::Function::Function>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outFunction: Arc<SimCodeFunction::Function::Function>;
    let mut outIncludes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outIncludeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLibPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outFunction, outIncludes, outIncludeDirs, outLibs, outLibPaths) = 'mc: {
        let __mc_input = (inElement.clone(), inIncludes.clone(), inIncludeDirs.clone(), inLibs.clone(), inLibPaths.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { partialPrefix: false, type_: Deref @ DAE::Type::T_FUNCTION { functionAttributes: funAttrs, funcArg: args, .. }, functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, visibility, source, path: fpath, .. }, includes, includeDirs, libs, libPaths) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut funArgs: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut varDecls: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut bodyStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut daeElts = (*daeElts).clone();
                    let DAE::FUNCTION_ATTRIBUTES { functionParallelism: DAE::FP_NON_PARALLEL { .. }, .. } = (funAttrs.clone()) else { bail!("pattern mismatch") };
                    daeElts = optMRFAElems(daeElts.clone())?;
                    outVars = List::map(DAEUtil::getOutputElements(daeElts.clone()), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    funArgs = List::map1(args.clone(), (std::sync::Arc::new(typesSimFunctionArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>), None);
                    collectRecDeclsFromElems(daeElts.clone(), recDeclsMap.clone())?;
                    vars = List::filterOnTrue(daeElts.clone(), (std::sync::Arc::new(fnptr!(isVarQ, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    varDecls = List::map(vars.clone(), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    bodyStmts = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for mut e in (daeElts.clone()).into_iter().cloned() {
                    if !(DAEUtil::isAlgorithm(e.clone())) { continue; }
                    let __x = elaborateStatement(e.clone())?;
                    __acc = __x.append(&__acc);
        }
        __acc
    });
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Ok((Arc::new(SimCodeFunction::Function::Function::FUNCTION { name: fpath.clone(), outVars: outVars.clone(), functionArguments: funArgs.clone(), variableDeclarations: varDecls.clone(), body: bodyStmts.clone(), visibility: visibility.clone(), info: info.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { partialPrefix: false, type_: Deref @ DAE::Type::T_FUNCTION { functionAttributes: funAttrs, funcArg: args, .. }, functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, source, path: fpath, .. }, includes, includeDirs, libs, libPaths) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut funArgs: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut varDecls: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut bodyStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut daeElts = (*daeElts).clone();
                    let DAE::FUNCTION_ATTRIBUTES { functionParallelism: DAE::FP_KERNEL_FUNCTION { .. }, .. } = (funAttrs.clone()) else { bail!("pattern mismatch") };
                    daeElts = optMRFAElems(daeElts.clone())?;
                    outVars = List::map(DAEUtil::getOutputElements(daeElts.clone()), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    funArgs = List::map1(args.clone(), (std::sync::Arc::new(typesSimFunctionArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>), None);
                    collectRecDeclsFromElems(daeElts.clone(), recDeclsMap.clone())?;
                    vars = List::filterOnTrue(daeElts.clone(), (std::sync::Arc::new(fnptr!(isVarNotInputNotOutput, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    varDecls = List::map(vars.clone(), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    bodyStmts = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for mut e in (daeElts.clone()).into_iter().cloned() {
                    if !(DAEUtil::isAlgorithm(e.clone())) { continue; }
                    let __x = elaborateStatement(e.clone())?;
                    __acc = __x.append(&__acc);
        }
        __acc
    });
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Ok((Arc::new(SimCodeFunction::Function::Function::KERNEL_FUNCTION { name: fpath.clone(), outVars: outVars.clone(), functionArguments: funArgs.clone(), variableDeclarations: varDecls.clone(), body: bodyStmts.clone(), info: info.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { partialPrefix: false, type_: Deref @ DAE::Type::T_FUNCTION { functionAttributes: funAttrs, funcArg: args, .. }, functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, source, path: fpath, .. }, includes, includeDirs, libs, libPaths) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut funArgs: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut varDecls: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut bodyStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut daeElts = (*daeElts).clone();
                    let DAE::FUNCTION_ATTRIBUTES { functionParallelism: DAE::FP_PARALLEL_FUNCTION { .. }, .. } = (funAttrs.clone()) else { bail!("pattern mismatch") };
                    daeElts = optMRFAElems(daeElts.clone())?;
                    outVars = List::map(DAEUtil::getOutputElements(daeElts.clone()), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    funArgs = List::map1(args.clone(), (std::sync::Arc::new(typesSimFunctionArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>), None);
                    collectRecDeclsFromElems(daeElts.clone(), recDeclsMap.clone())?;
                    vars = List::filterOnTrue(daeElts.clone(), (std::sync::Arc::new(fnptr!(isVarQ, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    varDecls = List::map(vars.clone(), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    bodyStmts = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for mut e in (daeElts.clone()).into_iter().cloned() {
                    if !(DAEUtil::isAlgorithm(e.clone())) { continue; }
                    let __x = elaborateStatement(e.clone())?;
                    __acc = __x.append(&__acc);
        }
        __acc
    });
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Ok((Arc::new(SimCodeFunction::Function::Function::PARALLEL_FUNCTION { name: fpath.clone(), outVars: outVars.clone(), functionArguments: funArgs.clone(), variableDeclarations: varDecls.clone(), body: bodyStmts.clone(), info: info.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { type_: Deref @ DAE::Type::T_FUNCTION { funcArg: args, .. }, functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: extdecl, body: daeElts }, tail: _ }, visibility, source, path: fpath, .. }, includes, includeDirs, libs, libPaths) => {
                    let mut extfnname: ArcStr = arcstr::literal!("");
                    let mut lang: ArcStr = arcstr::literal!("");
                    let mut fn_libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut fn_paths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut fn_includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut fn_includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut extargs: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
                    let mut simextargs: Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>> = metamodelica::nil();
                    let mut extReturn: Arc<SimCodeFunction::SimExtArg::SimExtArg> = Arc::new(SimCodeFunction::SimExtArg::SIMNOEXTARG);
                    let mut extretarg: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
                    let mut ann: Option<Arc<SCode::Annotation>> = None;
                    let mut outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut inVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut biVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut funArgs: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut dynamicLoad: bool = false;
                    let mut includes = (*includes).clone();
                    let mut includeDirs = (*includeDirs).clone();
                    let mut libs = (*libs).clone();
                    let mut libPaths = (*libPaths).clone();
                    let DAE::EXTERNALDECL { ann: __pa0, language: __pa1, returnArg: __pa2, args: __pa3, name: __pa4 } = (extdecl.clone()) else { bail!("pattern mismatch") };
                    ann = __pa0.clone();
                    lang = __pa1.clone();
                    extretarg = __pa2.clone();
                    extargs = __pa3.clone();
                    extfnname = __pa4.clone();
                    funArgs = List::map1(args.clone(), (std::sync::Arc::new(typesSimFunctionArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>), None);
                    outVars = List::map(DAEUtil::getOutputElements(daeElts.clone()), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    inVars = List::map(DAEUtil::getInputVars(daeElts.clone()), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    biVars = List::map(DAEUtil::getBidirElements(daeElts.clone()), (std::sync::Arc::new(daeInOutSimVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    collectRecDeclsFromElems(daeElts.clone(), recDeclsMap.clone())?;
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    (fn_includes, fn_includeDirs, fn_libs, fn_paths, dynamicLoad) = generateExtFunctionIncludes(program.clone(), fpath.clone(), ann.clone(), info.clone())?;
                    includes = List::union(fn_includes.clone(), includes.clone());
                    includeDirs = List::union(fn_includeDirs.clone(), includeDirs.clone());
                    libs = List::union(fn_libs.clone(), libs.clone());
                    libPaths = List::union(fn_paths.clone(), libPaths.clone());
                    simextargs = List::map(extargs.clone(), (std::sync::Arc::new(extArgsToSimExtArgs) as std::sync::Arc<dyn ::std::ops::Fn(DAE::ExtArg) -> Result<Arc<SimCodeFunction::SimExtArg::SimExtArg>> + 'static>));
                    extReturn = extArgsToSimExtArgs(extretarg.clone())?;
                    (simextargs, extReturn) = fixOutputIndex(outVars.clone(), simextargs.clone(), extReturn.clone())?;
                    lang = (System::toupper((lang.clone()).clone())).clone();
                    Ok((Arc::new(SimCodeFunction::Function::Function::EXTERNAL_FUNCTION { name: fpath.clone(), extName: (extfnname.clone()).clone(), funArgs: funArgs.clone(), extArgs: simextargs.clone(), extReturn: extReturn.clone(), inVars: inVars.clone(), outVars: outVars.clone(), biVars: biVars.clone(), includes: fn_includes.clone(), libs: fn_libs.clone(), language: (lang.clone()).clone(), visibility: visibility.clone(), info: info.clone(), dynamicLoad: dynamicLoad.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::RECORD_CONSTRUCTOR { type_: Deref @ DAE::Type::T_FUNCTION { funcResultType: restype @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: name }, .. }, funcArg: args, .. }, source, .. }, includes, includeDirs, libs, libPaths) => {
                    let mut funArgs: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut varDecls: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut varlst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    funArgs = List::map1(args.clone(), (std::sync::Arc::new(typesSimFunctionArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>), None);
                    collectRecDeclsFromType(restype.clone(), recDeclsMap.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(restype.clone()) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varlst = __pa0.clone();
                    varlst = List::filterOnFalse(varlst.clone(), (std::sync::Arc::new(Types::isModifiableTypesVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
                    varDecls = List::map(varlst.clone(), (std::sync::Arc::new(typesVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Ok((Arc::new(SimCodeFunction::Function::Function::RECORD_CONSTRUCTOR { name: name.clone(), funArgs: funArgs.clone(), locals: varDecls.clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, info: info.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#fn, _, _, _, _) => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function elaborateFunction failed for function:\n")); __mm_s.push_str(&*DAEDump::dumpFunctionStr(r#fn.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outFunction, outIncludes, outIncludeDirs, outLibs, outLibPaths))
}

fn typesSimFunctionArg(mut inFuncArg: Arc<DAE::FuncArg>, mut binding: Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> {
    let mut outVar: Arc<SimCodeFunction::Variable::Variable>;
    outVar = 'mc: {
        let __mc_input = inFuncArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_TUPLE { types: tys, .. }, funcArg: args, .. }, name, .. } => {
                    let mut var_args: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut tys = (*tys).clone();
                    var_args = List::map1(args.clone(), (std::sync::Arc::new(typesSimFunctionArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>), None);
                    tys = List::map(tys.clone(), (std::sync::Arc::new(Types::simplifyType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(SimCodeFunction::Variable::Variable::FUNCTION_PTR { name: (name.clone()).clone(), tys: tys.clone(), args: var_args.clone(), defaultValue: binding.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_NORETCALL, funcArg: args, .. }, name, .. } => {
                    let mut var_args: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    var_args = List::map1(args.clone(), (std::sync::Arc::new(typesSimFunctionArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>), None);
                    Ok(Arc::new(SimCodeFunction::Variable::Variable::FUNCTION_PTR { name: (name.clone()).clone(), tys: metamodelica::nil(), args: var_args.clone(), defaultValue: binding.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_FUNCTION { funcResultType: res_ty, funcArg: args, .. }, name, .. } => {
                    let mut var_args: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
                    let mut res_ty = (*res_ty).clone();
                    res_ty = Types::simplifyType(res_ty.clone())?;
                    var_args = List::map1(args.clone(), (std::sync::Arc::new(typesSimFunctionArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Option<Arc<DAE::Exp>>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>), None);
                    Ok(Arc::new(SimCodeFunction::Variable::Variable::FUNCTION_PTR { name: (name.clone()).clone(), tys: list![res_ty.clone()], args: var_args.clone(), defaultValue: binding.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::FuncArg { r#const, par: prl, ty: tty, name, .. } => {
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut kind: DAE::VarKind = DAE::VarKind::CONST;
                    let mut tty = (*tty).clone();
                    tty = Types::simplifyType(tty.clone())?;
                    cref_ = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), tty.clone(), metamodelica::nil());
                    kind = DAEUtil::const2VarKind(r#const.clone())?;
                    Ok(Arc::new(SimCodeFunction::Variable::Variable::VARIABLE { name: cref_.clone(), ty: tty.clone(), value: binding.clone(), instDims: metamodelica::nil(), parallelism: prl.clone(), kind: kind.clone(), bind_from_outside: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVar)
}

fn daeInOutSimVar(mut inElement: Arc<DAE::Element>) -> Result<Arc<SimCodeFunction::Variable::Variable>> {
    let mut outVar: Arc<SimCodeFunction::Variable::Variable>;
    outVar = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { binding, parallelism: prl, ty: daeType @ Deref @ DAE::Type::T_FUNCTION { .. }, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, .. } => {
                    let mut var: Arc<SimCodeFunction::Variable::Variable>;
                    var = typesSimFunctionArg(Arc::new(DAE::FuncArg { name: (name.clone()).clone(), ty: daeType.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: prl.clone(), defaultBinding: None }), binding.clone())?;
                    Ok(var.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { kind, dims: inst_dims, binding, ty: daeType, parallelism: prl, componentRef: id, .. } => {
                    let mut daeType = (*daeType).clone();
                    daeType = Types::simplifyType(daeType.clone())?;
                    Ok(Arc::new(SimCodeFunction::Variable::Variable::VARIABLE { name: id.clone(), ty: daeType.clone(), value: binding.clone(), instDims: inst_dims.clone(), parallelism: prl.clone(), kind: kind.clone(), bind_from_outside: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function daeInOutSimVar failed\n")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVar)
}

fn extArgsToSimExtArgs(mut extArg: DAE::ExtArg) -> Result<Arc<SimCodeFunction::SimExtArg::SimExtArg>> {
    let mut simExtArg: Arc<SimCodeFunction::SimExtArg::SimExtArg> = Arc::new(SimCodeFunction::SimExtArg::SIMNOEXTARG);
    simExtArg = (match extArg.clone() {
        DAE::ExtArg::EXTARG { componentRef: mut componentRef, direction: mut dir, type_: mut type_ } => {
            let mut isInput: bool = false;
            let mut isOutput: bool = false;
            let mut isArray: bool = false;
            let mut outputIndex: i32 = 0;
            let mut type_ = type_.clone();
            isInput = AbsynUtil::isInput(dir.clone());
            isOutput = AbsynUtil::isOutput(dir.clone());
            outputIndex = if (isOutput.clone()) {-1} else {0};
            isArray = Types::isArray(type_.clone());
            type_ = Types::simplifyType(type_.clone())?;
            Arc::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARG { cref: componentRef.clone(), isInput: isInput.clone(), outputIndex: outputIndex.clone(), isArray: isArray.clone(), hasBinding: false, type_: type_.clone() })
        },
        DAE::ExtArg::EXTARGEXP { exp: ref exp_, type_: mut type_ } => {
            let mut type_ = type_.clone();
            type_ = Types::simplifyType(type_.clone())?;
            Arc::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARGEXP { exp: exp_.clone(), type_: type_.clone() })
        },
        DAE::ExtArg::EXTARGSIZE { componentRef: mut componentRef, type_: mut type_, exp: ref exp_ } => {
            let mut type_ = type_.clone();
            type_ = Types::simplifyType(type_.clone())?;
            Arc::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARGSIZE { cref: componentRef.clone(), isInput: true, outputIndex: 0, type_: type_.clone(), exp: exp_.clone() })
        },
        DAE::ExtArg::NOEXTARG => {
            Arc::new(crate::SimCodeFunction::SimExtArg::SIMNOEXTARG)
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(simExtArg)
}

fn fixOutputIndex(mut outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>, mut simExtArgsIn: Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>>, mut extReturnIn: Arc<SimCodeFunction::SimExtArg::SimExtArg>) -> Result<(Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>>, Arc<SimCodeFunction::SimExtArg::SimExtArg>)> {
    let mut simExtArgsOut: Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>> = metamodelica::nil();
    let mut extReturnOut: Arc<SimCodeFunction::SimExtArg::SimExtArg> = Arc::new(SimCodeFunction::SimExtArg::SIMNOEXTARG);
    (simExtArgsOut, extReturnOut) = (::match_deref::match_deref! { match &(extReturnIn.clone()) {
        _ => {
            simExtArgsOut = List::map1(simExtArgsIn.clone(), (std::sync::Arc::new(assignOutputIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCodeFunction::SimExtArg::SimExtArg>, Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Arc<SimCodeFunction::SimExtArg::SimExtArg>> + 'static>), outVars.clone());
            extReturnOut = assignOutputIndex(extReturnIn.clone(), outVars.clone())?;
            (simExtArgsOut.clone(), extReturnOut.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((simExtArgsOut, extReturnOut))
}

fn assignOutputIndex(mut simExtArgIn: Arc<SimCodeFunction::SimExtArg::SimExtArg>, mut outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Arc<SimCodeFunction::SimExtArg::SimExtArg>> {
    let mut simExtArgOut: Arc<SimCodeFunction::SimExtArg::SimExtArg> = Arc::new(SimCodeFunction::SimExtArg::SIMNOEXTARG);
    simExtArgOut = 'mc: {
        let __mc_input = simExtArgIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref, isInput, outputIndex, isArray, hasBinding: _, type_ } => {
                    let mut fcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut hasBinding: bool = false;
                    let mut newOutputIndex: i32 = 0;
                    let true = (outputIndex.clone() == -1) else { bail!("pattern mismatch") };
                    fcref = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
                    (newOutputIndex, hasBinding) = findIndexInList(fcref.clone(), outVars.clone(), 1)?;
                    Ok(Arc::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARG { cref: cref.clone(), isInput: isInput.clone(), outputIndex: newOutputIndex.clone(), isArray: isArray.clone(), hasBinding: hasBinding.clone(), type_: type_.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::SimExtArg::SIMEXTARGSIZE { cref, isInput, outputIndex, type_, exp } => {
                    let mut newOutputIndex: i32 = 0;
                    let true = (outputIndex.clone() == -1) else { bail!("pattern mismatch") };
                    (newOutputIndex, _) = findIndexInList(cref.clone(), outVars.clone(), 1)?;
                    Ok(Arc::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARGSIZE { cref: cref.clone(), isInput: isInput.clone(), outputIndex: newOutputIndex.clone(), type_: type_.clone(), exp: exp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(simExtArgIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(simExtArgOut)
}

fn findIndexInList(mut cref: Arc<DAE::ComponentRef>, mut outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>, mut inCurrentIndex: i32) -> Result<(i32, bool)> {
    let mut crefIndexInOutVars: i32 = 0;
    let mut hasBinding: bool = false;
    (crefIndexInOutVars, hasBinding) = 'mc: {
        let __mc_input = (outVars.clone(), inCurrentIndex.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((-1, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCodeFunction::Variable::VARIABLE { value: v, name, .. }, tail: _ }, currentIndex) => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cref.clone(), name.clone())?) else { bail!("pattern mismatch") };
                    Ok((currentIndex.clone(), isSome(v.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: restOutVars }, currentIndex) => {
                    let mut currentIndex = (*currentIndex).clone();
                    let mut hasBinding: bool = hasBinding.clone();
                    currentIndex = currentIndex.clone() + 1;
                    (currentIndex, hasBinding) = findIndexInList(cref.clone(), restOutVars.clone(), currentIndex.clone())?;
                    Ok((currentIndex.clone(), hasBinding.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((crefIndexInOutVars, hasBinding))
}

fn elaborateStatement(mut inElement: Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: __pa0 }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stmts = __pa0.clone();
    Ok(stmts)
}

fn optMRFAElems(mut elems: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elems: Arc<metamodelica::List<Arc<DAE::Element>>> = elems;
    let mut processed: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut e2: Arc<DAE::Element>;
    for mut e in &*elems.clone() {
        let mut e = e.clone();
        (e2, tempVars) = optMRFAElem(e.clone(), tempVars.clone())?;
        processed = cons(e2.clone(), processed.clone());
    }
    elems = listAppend(tempVars.clone().reverse(), processed.clone().reverse());
    Ok(elems)
}

fn optMRFAElem(mut elem: Arc<DAE::Element>, mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<DAE::Element>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut elem: Arc<DAE::Element> = elem;
    let mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>> = tempVars;
    (elem, tempVars) = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            let mut stmts = (*stmts).clone();
            (stmts, tempVars) = DAEUtil::optimizeMetaRecordFieldAssigns(stmts.clone(), tempVars.clone())?;
            assign_variant_field!(elem => DAE::Element::ALGORITHM; algorithm_ = Arc::new(DAE::Algorithm { statementLst: stmts.clone() }));
            (elem.clone(), tempVars.clone())
        },
        _ => {
            (elem.clone(), tempVars.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((elem, tempVars))
}

pub fn checkValidMainFunction(mut name: ArcStr, mut r#fn: Arc<SimCodeFunction::Function::Function>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = r#fn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::Function::FUNCTION { functionArguments: inVars, .. } => {
                    if '__try0: {
                        unwrap_break_err!(List::find(inVars.clone(), (std::sync::Arc::new(fnptr!(isFunctionPtr, Arc<SimCodeFunction::Variable::Variable>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCodeFunction::Variable::Variable>) -> Result<bool> + 'static>)), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { inVars, .. } => {
                    if '__try0: {
                        unwrap_break_err!(List::find(inVars.clone(), (std::sync::Arc::new(fnptr!(isFunctionPtr, Arc<SimCodeFunction::Variable::Variable>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCodeFunction::Variable::Variable>) -> Result<bool> + 'static>)), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::GENERATECODE_INVARS_HAS_FUNCTION_PTR.clone(), list![(name.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn isBoxedFunction(mut r#fn: Arc<SimCodeFunction::Function::Function>) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = r#fn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::Function::FUNCTION { outVars, functionArguments: inVars, .. } => {
                    List::map_0(inVars.clone(), (std::sync::Arc::new(isBoxedArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCodeFunction::Variable::Variable>) -> Result<()> + 'static>));
                    List::map_0(outVars.clone(), (std::sync::Arc::new(isBoxedArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCodeFunction::Variable::Variable>) -> Result<()> + 'static>));
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { outVars, inVars, .. } => {
                    List::map_0(inVars.clone(), (std::sync::Arc::new(isBoxedArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCodeFunction::Variable::Variable>) -> Result<()> + 'static>));
                    List::map_0(outVars.clone(), (std::sync::Arc::new(isBoxedArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCodeFunction::Variable::Variable>) -> Result<()> + 'static>));
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
    Ok(b)
}

fn isFunctionPtr(mut var: Arc<SimCodeFunction::Variable::Variable>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ SimCodeFunction::Variable::FUNCTION_PTR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn isBoxedArg(mut var: Arc<SimCodeFunction::Variable::Variable>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ SimCodeFunction::Variable::FUNCTION_PTR { .. } => (),
        Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_METABOXED { .. }, .. } => (),
        Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_METATYPE { .. }, .. } => (),
        Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_STRING { .. }, .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn funcHasParallelInOutArrays(mut r#fn: Arc<SimCodeFunction::Function::Function>) -> Result<bool> {
    let mut b: bool = false;
    let mut inVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(r#fn.clone()) {
        Deref @ SimCodeFunction::Function::FUNCTION { outVars: __pa0, functionArguments: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outVars = __pa0.clone();
    inVars = __pa1.clone();
    for mut e in &*inVars.clone() {
        let mut e = e.clone();
        if isParallelArrayVar(e.clone()) {
            b = true;
            return Ok(b.clone());
        }
    }
    for mut e in &*outVars.clone() {
        let mut e = e.clone();
        if isParallelArrayVar(e.clone()) {
            b = true;
            return Ok(b.clone());
        }
    }
    b = false;
    Ok(b)
}

fn isParallelArrayVar(mut var: Arc<SimCodeFunction::Variable::Variable>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ SimCodeFunction::Variable::VARIABLE { parallelism: DAE::VarParallelism::PARGLOBAL, ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => true,
        Deref @ SimCodeFunction::Variable::VARIABLE { parallelism: DAE::VarParallelism::PARLOCAL, ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn findLiterals(mut fns: Arc<metamodelica::List<DAE::Function>>) -> (Arc<metamodelica::List<DAE::Function>>, Arc<metamodelica::List<Arc<DAE::Exp>>>) {
    let mut ofns: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
    let mut literals: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (__pa0, (_, _, __pa1)) = DAEUtil::traverseDAEFunctions(fns.clone(), (std::sync::Arc::new(findLiteralsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>), (0, HashTableExpToIndex::emptyHashTableSized(BaseHashTable::bigBucketSize.clone()), metamodelica::nil()));
    ofns = __pa0.clone();
    literals = __pa1.clone();
    literals = literals.clone().reverse();
    (ofns, literals)
}

pub fn findLiteralsHelper(mut inExp: Arc<DAE::Exp>, mut inTpl: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> {
    let mut exp: Arc<DAE::Exp>;
    let mut tpl: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    exp = inExp.clone();
    tpl = inTpl.clone();
    (exp, tpl) = Expression::traverseExpBottomUp(exp.clone(), Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(replaceLiteralExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>); move |__pe_a0, __pe_a1| Patternm::traverseConstantPatternsHelper(__pe_a0, __pe_a1, __pe_b2.clone()) }), tpl.clone())?;
    (exp, tpl) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(fnptr!(replaceLiteralArrayExp, Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, bool, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>), tpl.clone())?;
    Ok((exp, tpl))
}

fn replaceLiteralArrayExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> (Arc<DAE::Exp>, bool, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTpl: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::ARRAY { .. }, tpl) => {
            let mut exp2: Arc<DAE::Exp>;
            let mut tpl = (*tpl).clone();
            match '__try0: {
                unwrap_break_err!(isLiteralArrayExp(inExp.clone()), '__try0);
                (exp2, tpl) = unwrap_break_err!(replaceLiteralExp2(inExp.clone(), tpl.clone()), '__try0);
                cont = false;
                Ok::<_, anyhow::Error>((exp2.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp2 = __try0_o0;
                }
                Err(_) => {
                    exp2 = inExp.clone();
                }
            }
            (exp2.clone(), tpl.clone())
        },
        (Deref @ DAE::Exp::MATRIX { .. }, tpl) => {
            let mut exp2: Arc<DAE::Exp>;
            let mut tpl = (*tpl).clone();
            match '__try0: {
                unwrap_break_err!(isLiteralArrayExp(inExp.clone()), '__try0);
                (exp2, tpl) = unwrap_break_err!(replaceLiteralExp2(inExp.clone(), tpl.clone()), '__try0);
                cont = false;
                Ok::<_, anyhow::Error>((exp2.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp2 = __try0_o0;
                }
                Err(_) => {
                    exp2 = inExp.clone();
                }
            }
            (exp2.clone(), tpl.clone())
        },
        _ => {
            (inExp.clone(), inTpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outTpl)
}

fn replaceLiteralExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, t) => {
                    if '__try0: {
                        unwrap_break_err!(isLiteralExp(exp.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((exp.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, t) => {
                    isTrivialLiteralExp(exp.clone())?;
                    Ok((exp.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LIST { valList: es }, t) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut t = (*t).clone();
                    let true = ((es.clone().len() as i32) > 25) else { bail!("pattern mismatch") };
                    (exp, t) = replaceLiteralExp2(inExp.clone(), t.clone())?;
                    Ok((exp.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, t) => {
                    let mut exp = (*exp).clone();
                    let mut t = (*t).clone();
                    exp = listToCons(exp.clone())?;
                    (exp, t) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(replaceLiteralExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>), t.clone())?;
                    Ok((exp.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, _) => {
                    let mut t: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
                    let mut exp = (*exp).clone();
                    if '__try0: {
                        unwrap_break_err!(listToCons(exp.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (exp, t) = replaceLiteralExp2(exp.clone(), inTpl.clone())?;
                    Ok((exp.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, _) => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function replaceLiteralExp failed. Falling back to not replacing ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone();
                    Error::addInternalError((msg.clone()).clone(), metamodelica::sourceInfo!())?;
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

fn replaceLiteralExp2(mut inExp: Arc<DAE::Exp>, mut inTpl: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (_, ht, _)) => {
                    let mut nexp: Arc<DAE::Exp>;
                    let mut ix: i32 = 0;
                    ix = BaseHashTable::get(exp.clone(), ht.clone())?;
                    nexp = Arc::new(DAE::Exp::SHARED_LITERAL { index: ix.clone(), exp: exp.clone() });
                    Ok((nexp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (i, ht, l)) => {
                    let mut nexp: Arc<DAE::Exp>;
                    let mut ht = (*ht).clone();
                    ht = BaseHashTable::add((exp.clone(), i.clone()), ht.clone())?;
                    nexp = Arc::new(DAE::Exp::SHARED_LITERAL { index: i.clone(), exp: exp.clone() });
                    Ok((nexp.clone(), (i.clone() + 1, ht.clone(), cons(exp.clone(), l.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

fn listToCons(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut o: Arc<DAE::Exp>;
    o = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::LIST { valList: es @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
            listToCons2(es.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(o)
}

fn listToCons2(mut ies: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Arc<DAE::Exp> {
    let mut o: Arc<DAE::Exp>;
    o = (::match_deref::match_deref! { match &(ies.clone()) {
        Deref @ metamodelica::List::Nil => {
            Arc::new(DAE::Exp::LIST { valList: metamodelica::nil() })
        },
        Deref @ metamodelica::List::Cons { head: car, tail: es } => {
            let mut cdr: Arc<DAE::Exp>;
            cdr = listToCons2(es.clone());
            Arc::new(DAE::Exp::CONS { car: car.clone(), cdr: cdr.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    o
}

fn isTrivialLiteralExp(mut exp: Arc<DAE::Exp>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::BOX { exp: Deref @ DAE::Exp::SCONST { string: _ } } => bail!("fail"),
        Deref @ DAE::Exp::BOX { exp: Deref @ DAE::Exp::RCONST { real: _ } } => bail!("fail"),
        Deref @ DAE::Exp::BOX { exp: _ } => (),
        Deref @ DAE::Exp::ICONST { integer: _ } => (),
        Deref @ DAE::Exp::BCONST { bool: _ } => (),
        Deref @ DAE::Exp::RCONST { real: _ } => (),
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => (),
        Deref @ DAE::Exp::LIST { valList: Deref @ metamodelica::List::Nil } => (),
        Deref @ DAE::Exp::META_OPTION { exp: None } => (),
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => (),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn isLiteralArrayExp(mut iexp: Arc<DAE::Exp>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(iexp.clone()) {
        Deref @ DAE::Exp::SCONST { string: _ } => {
            ()
        },
        Deref @ DAE::Exp::ICONST { integer: _ } => {
            ()
        },
        Deref @ DAE::Exp::RCONST { real: _ } => {
            ()
        },
        Deref @ DAE::Exp::BCONST { bool: _ } => {
            ()
        },
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            List::map_0(expl.clone(), (std::sync::Arc::new(isLiteralArrayExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ DAE::Exp::MATRIX { matrix: expll, .. } => {
            List::map_0(List::flatten(expll.clone()), (std::sync::Arc::new(isLiteralArrayExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(exp) } => {
            isLiteralArrayExp(exp.clone())?;
            ()
        },
        Deref @ DAE::Exp::BOX { exp } => {
            isLiteralArrayExp(exp.clone())?;
            ()
        },
        Deref @ DAE::Exp::CONS { cdr: e2, car: e1 } => {
            isLiteralArrayExp(e1.clone())?;
            isLiteralArrayExp(e2.clone())?;
            ()
        },
        Deref @ DAE::Exp::LIST { valList: expl } => {
            List::map_0(expl.clone(), (std::sync::Arc::new(isLiteralArrayExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expl } => {
            List::map_0(expl.clone(), (std::sync::Arc::new(isLiteralArrayExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ DAE::Exp::METARECORDCALL { args: expl, .. } => {
            List::map_0(expl.clone(), (std::sync::Arc::new(isLiteralArrayExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            ()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn isLiteralExp(mut iexp: Arc<DAE::Exp>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(iexp.clone()) {
        Deref @ DAE::Exp::SCONST { string: _ } => {
            ()
        },
        Deref @ DAE::Exp::ICONST { integer: _ } => {
            ()
        },
        Deref @ DAE::Exp::RCONST { real: _ } => {
            ()
        },
        Deref @ DAE::Exp::BCONST { bool: _ } => {
            ()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(exp) } => {
            isLiteralExp(exp.clone())?;
            ()
        },
        Deref @ DAE::Exp::BOX { exp } => {
            isLiteralExp(exp.clone())?;
            ()
        },
        Deref @ DAE::Exp::CONS { cdr: e2, car: e1 } => {
            isLiteralExp(e1.clone())?;
            isLiteralExp(e2.clone())?;
            ()
        },
        Deref @ DAE::Exp::LIST { valList: expl } => {
            List::map_0(expl.clone(), (std::sync::Arc::new(isLiteralExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expl } => {
            List::map_0(expl.clone(), (std::sync::Arc::new(isLiteralExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ DAE::Exp::METARECORDCALL { args: expl, .. } => {
            List::map_0(expl.clone(), (std::sync::Arc::new(isLiteralExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            ()
        },
        Deref @ DAE::Exp::CALL { expLst: expl, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listArrayLiteral" }, .. } => {
            List::map_0(expl.clone(), (std::sync::Arc::new(isLiteralExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<()> + 'static>));
            ()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn collectRecDeclsFromTypes(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<()> {
    for mut ty in &*inTypes.clone() {
        let mut ty = ty.clone();
        collectRecDeclsFromType(ty.clone(), recDeclsMap.clone())?;
    }
    Ok(())
}

fn collectRecDeclsFromElems(mut inElems: Arc<metamodelica::List<Arc<DAE::Element>>>, mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<()> {
    for mut elem in &*inElems.clone() {
        let mut elem = elem.clone();
        let () = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ DAE::Element::VAR { .. } => {
            collectRecDeclsFromType(var_field!((*elem).ty, DAE::Element::VAR).clone(), recDeclsMap.clone())?;
            if isSome(var_field!((*elem).binding, DAE::Element::VAR).clone()) && Config::acceptMetaModelicaGrammar()? {
                Expression::traverseExpBottomUp(Util::getOption(var_field!((*elem).binding, DAE::Element::VAR).clone())?, (std::sync::Arc::new(collectRecDeclsFromMetaRecCallExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>)> + 'static>), recDeclsMap.clone())?;
            }
            ()
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            if Config::acceptMetaModelicaGrammar()? {
                DAEUtil::traverseAlgorithmExps(var_field!((*elem).algorithm_, DAE::Element::ALGORITHM).clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectRecDeclsFromMetaRecCallExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>)> + 'static>), recDeclsMap.clone()))?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

fn isVarQ(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: vd, kind: vk, .. } if (isVarKindVarOrParameter(vk.clone()) && isDirectionNotInput(vd.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn isVarNotInputNotOutput(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: vd, kind: vk, .. } if (isVarKindVarOrParameter(vk.clone()) && isDirectionNotInputNotOutput(vd.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn isVarKindVarOrParameter(mut inVarKind: DAE::VarKind) -> bool {
    let mut outB: bool = false;
    outB = (match inVarKind.clone() {
        DAE::VarKind::VARIABLE => true,
        DAE::VarKind::PARAM => true,
        DAE::VarKind::CONST => true,
        _ => false,
    });
    outB
}

fn isDirectionNotInput(mut inVarDirection: DAE::VarDirection) -> bool {
    let mut outB: bool = false;
    outB = (match inVarDirection.clone() {
        DAE::VarDirection::OUTPUT => true,
        DAE::VarDirection::BIDIR => true,
        _ => false,
    });
    outB
}

fn isDirectionNotInputNotOutput(mut inVarDirection: DAE::VarDirection) -> bool {
    let mut outB: bool = false;
    outB = (match inVarDirection.clone() {
        DAE::VarDirection::BIDIR => true,
        _ => false,
    });
    outB
}

fn filterNg(mut ng: i32) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = if (useZerocrossing()?) {ng.clone()} else {0};
    Ok(outInteger)
}

fn useZerocrossing() -> Result<bool> {
    let mut res: bool = false;
    res = Flags::isSet(Flags::EVENTS.clone())?;
    Ok(res)
}

fn getCrefFromExp(mut e: Arc<DAE::Exp>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut c: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    c = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: crefe, .. } => {
            let mut crefa: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            crefa = ComponentReference::unelabCref(crefe.clone())?;
            crefa.clone()
        },
        _ => {
            Error::addInternalError((literal!("function getCrefFromExp failed: input was not of type DAE.CREF")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(c)
}

fn collectRecDeclsFromType(mut inRecordType: Arc<DAE::Type>, mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inRecordType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { usedExternally, varLst: varlst, complexClassType: ClassInf::State::RECORD { path }, .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut sname: ArcStr = arcstr::literal!("");
            let mut vars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
            let mut recDecl: SimCodeFunction::RecordDeclaration;
            let mut optRecDecl: Option<SimCodeFunction::RecordDeclaration> = None;
            let mut is_default: bool = false;
            let mut bool1: bool = false;
            name = (AbsynUtil::pathStringUnquoteReplaceDot(path.clone(), (literal!("_")).clone())?).clone();
            (sname, is_default) = checkBindingsandGetConstructorName((name.clone()).clone(), varlst.clone());
            optRecDecl = UnorderedMap::get((sname.clone()).clone(), recDeclsMap.clone());
            if is_default.clone() {
                if isSome(optRecDecl.clone()) {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(optRecDecl.clone()) {
                        Some(SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: _, aliasName: _, defPath: _, variables: __pa0, usedExternally: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vars = __pa0.clone();
                    bool1 = __pa1.clone();
                    if usedExternally.clone() && !(bool1.clone()) {
                        recDecl = SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: (sname.clone()).clone(), aliasName: None, defPath: path.clone(), variables: vars.clone(), usedExternally: true };
                        UnorderedMap::add((sname.clone()).clone(), recDecl.clone(), recDeclsMap.clone())?;
                    }
                } else {
                    vars = List::map(varlst.clone(), (std::sync::Arc::new(typesVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    recDecl = SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: (sname.clone()).clone(), aliasName: None, defPath: path.clone(), variables: vars.clone(), usedExternally: usedExternally.clone() };
                    UnorderedMap::add((sname.clone()).clone(), recDecl.clone(), recDeclsMap.clone())?;
                    collectRecDeclsFromTypesVars(varlst.clone(), recDeclsMap.clone())?;
                }
            } else {
                if isNone(optRecDecl.clone()) {
                    vars = List::map(varlst.clone(), (std::sync::Arc::new(typesVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<SimCodeFunction::Variable::Variable>> + 'static>));
                    recDecl = SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR { ctor_name: (sname.clone()).clone(), name: (name.clone()).clone(), variables: vars.clone() };
                    UnorderedMap::add((sname.clone()).clone(), recDecl.clone(), recDeclsMap.clone())?;
                }
            }
            ()
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } => {
            ()
        },
        Deref @ DAE::Type::T_METARECORD { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "SourceInfo", .. }, .. } => {
            ()
        },
        Deref @ DAE::Type::T_METARECORD { path, fields: varlst, .. } => {
            let mut sname: ArcStr = arcstr::literal!("");
            let mut fieldNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            sname = (AbsynUtil::pathStringUnquoteReplaceDot(path.clone(), (literal!("_")).clone())?).clone();
            fieldNames = List::map(varlst.clone(), (std::sync::Arc::new(fnptr!(generateVarName, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
            UnorderedMap::tryAdd((sname.clone()).clone(), SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: path.clone(), fieldNames: fieldNames.clone() }, recDeclsMap.clone())?;
            collectRecDeclsFromTypesVars(varlst.clone(), recDeclsMap.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn typesVarNoBinding(mut inTypesVar: Arc<DAE::Var>) -> Result<Arc<SimCodeFunction::Variable::Variable>> {
    let mut outVar: Arc<SimCodeFunction::Variable::Variable>;
    outVar = (::match_deref::match_deref! { match &(inTypesVar.clone()) {
        Deref @ DAE::Var { ty, attributes: attr, name, .. } => {
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut scPrl: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
            let mut prl: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
            let mut ty = (*ty).clone();
            ty = Types::simplifyType(ty.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), ty.clone(), metamodelica::nil());
            let __pa0 = ::match_deref::match_deref! { match &(attr.clone()) {
                Deref @ DAE::Attributes { parallelism: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            scPrl = __pa0.clone();
            prl = scodeParallelismToDAEParallelism(scPrl.clone())?;
            Arc::new(SimCodeFunction::Variable::Variable::VARIABLE { name: cref_.clone(), ty: ty.clone(), value: None, instDims: metamodelica::nil(), parallelism: prl.clone(), kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE, bind_from_outside: false })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

fn typesVar(mut inTypesVar: Arc<DAE::Var>) -> Result<Arc<SimCodeFunction::Variable::Variable>> {
    let mut outVar: Arc<SimCodeFunction::Variable::Variable>;
    outVar = (::match_deref::match_deref! { match &(inTypesVar.clone()) {
        Deref @ DAE::Var { ty, attributes: attr, name, .. } => {
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut scPrl: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
            let mut prl: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
            let mut bindExp: Option<Arc<DAE::Exp>> = None;
            let mut ty = (*ty).clone();
            ty = Types::simplifyType(ty.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), ty.clone(), metamodelica::nil());
            let __pa0 = ::match_deref::match_deref! { match &(attr.clone()) {
                Deref @ DAE::Attributes { parallelism: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            scPrl = __pa0.clone();
            prl = scodeParallelismToDAEParallelism(scPrl.clone())?;
            bindExp = checkSourceAndGetBindingExp(inTypesVar.binding.clone());
            Arc::new(SimCodeFunction::Variable::Variable::VARIABLE { name: cref_.clone(), ty: ty.clone(), value: bindExp.clone(), instDims: metamodelica::nil(), parallelism: prl.clone(), kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE, bind_from_outside: inTypesVar.bind_from_outside.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

fn checkBindingsandGetConstructorName(mut rec_name: ArcStr, mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> (ArcStr, bool) {
    let mut ctor_name: ArcStr = arcstr::literal!("");
    let mut is_default: bool = false;
    let mut varnum: i32 = 0;
    is_default = true;
    ctor_name = (rec_name.clone()).clone();
    varnum = 1;
    for mut var in &*vars.clone() {
        let mut var = var.clone();
        if var.bind_from_outside.clone() && !(isBindingFromDerivedRecordDeclaration(var.binding.clone())) {
            is_default = false;
            ctor_name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ctor_name.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varnum.clone())); ArcStr::from(__mm_s) }).clone();
        }
        varnum = intAdd(varnum.clone(), 1);
    }
    (ctor_name, is_default)
}

fn isBindingFromDerivedRecordDeclaration(mut bind: Arc<DAE::Binding>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(bind.clone()) {
        Deref @ DAE::Binding::EQBOUND { source: DAE::BindingSource::BINDING_FROM_DERIVED_RECORD_DECL, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn checkSourceAndGetBindingExp(mut inBinding: Arc<DAE::Binding>) -> Option<Arc<DAE::Exp>> {
    let mut bindExp: Option<Arc<DAE::Exp>> = None;
    bindExp = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Deref @ DAE::Binding::EQBOUND { source: DAE::BindingSource::BINDING_FROM_RECORD_SUBMODS, .. } => None,
        Deref @ DAE::Binding::EQBOUND { .. } => Some(var_field!((*inBinding).exp, DAE::Binding::EQBOUND).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    bindExp
}

fn scodeParallelismToDAEParallelism(mut inParallelism: SCode::Parallelism) -> Result<DAE::VarParallelism> {
    let mut outParallelism: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    outParallelism = (match inParallelism.clone() {
        SCode::Parallelism::PARGLOBAL => openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL,
        SCode::Parallelism::PARLOCAL => openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL,
        SCode::Parallelism::NON_PARALLEL => openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        _ => bail!("match: no arm matched"),
    });
    Ok(outParallelism)
}

fn variableName(mut v: Arc<SimCodeFunction::Variable::Variable>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = ((::match_deref::match_deref! { match &(v.clone()) {
        Deref @ SimCodeFunction::Variable::VARIABLE { name: Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, .. }, .. } => s.clone(),
        Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: s, .. } => s.clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(s)
}

fn compareVariable(mut v1: Arc<SimCodeFunction::Variable::Variable>, mut v2: Arc<SimCodeFunction::Variable::Variable>) -> Result<bool> {
    let mut b: bool = false;
    b = stringCompare((variableName(v1.clone())?).clone(), (variableName(v2.clone())?).clone()) > 0;
    Ok(b)
}

fn generateVarName(mut inVar: Arc<DAE::Var>) -> ArcStr {
    let mut outName: ArcStr = arcstr::literal!("");
    outName = ((::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { name, .. } => {
            name.clone()
        },
        _ => {
            literal!("NULL")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outName
}

fn collectRecDeclsFromTypesVars(mut inRecordTypeVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<()> {
    for mut recTyVar in &*inRecordTypeVars.clone() {
        let mut recTyVar = recTyVar.clone();
        let () = (::match_deref::match_deref! { match &(recTyVar.clone()) {
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. } => {
            collectRecDeclsFromType(recTyVar.ty.clone(), recDeclsMap.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

fn collectRecDeclsFromMetaRecCallExps(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<()> {
    for mut exp in &*inExpl.clone() {
        let mut exp = exp.clone();
        collectRecDeclsFromMetaRecCallExp(exp.clone(), recDeclsMap.clone())?;
    }
    Ok(())
}

fn collectRecDeclsFromMetaRecCallExp(mut inExp: Arc<DAE::Exp>, mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>) -> Result<(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>)> {
    let mut inExp: Arc<DAE::Exp> = inExp;
    let mut recDeclsMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>> = recDeclsMap;
    let mut name: ArcStr = arcstr::literal!("");
    let () = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::METARECORDCALL { .. } => {
            if var_field!((*inExp).index, DAE::Exp::METARECORDCALL).clone() != -1 {
                name = (AbsynUtil::pathStringUnquoteReplaceDot(var_field!((*inExp).path, DAE::Exp::METARECORDCALL).clone(), (literal!("_")).clone())?).clone();
                UnorderedMap::tryAdd((name.clone()).clone(), SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: var_field!((*inExp).path, DAE::Exp::METARECORDCALL).clone(), fieldNames: var_field!((*inExp).fieldNames, DAE::Exp::METARECORDCALL).clone() }, recDeclsMap.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((inExp, recDeclsMap))
}

fn generateExtFunctionIncludes(mut program: Absyn::Program, mut path: Arc<Absyn::Path>, mut inAbsynAnnotationOption: Option<Arc<SCode::Annotation>>, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, bool)> {
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut paths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut dynamcLoad: bool = false;
    (includes, includeDirs, libs, paths, dynamcLoad) = (::match_deref::match_deref! { match &(inAbsynAnnotationOption.clone()) {
        Some(Deref @ SCode::Annotation { modification: r#mod }) => {
            let mut b: bool = false;
            let mut target: ArcStr = arcstr::literal!("");
            let mut resources: Option<ArcStr> = None;
            let mut libNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut fullLibNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut dirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            b = generateExtFunctionDynamicLoad(r#mod.clone())?;
            target = (Flags::getConfigString(Flags::TARGET.clone())?).clone();
            (libs, libNames) = generateExtFunctionIncludesLibstr((target.clone()).clone(), r#mod.clone())?;
            includes = generateExtFunctionIncludesIncludestr(r#mod.clone())?;
            (libs, dirs, resources) = generateExtFunctionLibraryDirectoryFlags(program.clone(), path.clone(), r#mod.clone(), libs.clone())?;
            for mut name in &*if (Flags::isSet(Flags::CHECK_EXT_LIBS.clone())?) {libNames.clone()} else {metamodelica::nil()} {
                let mut name = name.clone();
                if getGerneralTarget((target.clone()).clone())? == literal!("msvc") || arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
                    fullLibNames = list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".a")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".lib")); ArcStr::from(__mm_s) }).clone()];
                } else {
                    fullLibNames = list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".a")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) }).clone()];
                }
                lookForExtFunctionLibrary(fullLibNames.clone(), dirs.clone(), (name.clone()).clone(), resources.clone(), path.clone(), info.clone())?;
            }
            paths = generateExtFunctionLibraryDirectoryPaths(program.clone(), path.clone(), r#mod.clone())?;
            includeDirs = generateExtFunctionIncludeDirectoryFlags(program.clone(), path.clone(), r#mod.clone(), includes.clone())?;
            (includes.clone(), includeDirs.clone(), libs.clone(), paths.clone(), b.clone())
        },
        None => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((includes, includeDirs, libs, paths, dynamcLoad))
}

fn lookForExtFunctionLibrary(mut names: Arc<metamodelica::List<ArcStr>>, mut dirs: Arc<metamodelica::List<ArcStr>>, mut name: ArcStr, mut resources: Option<ArcStr>, mut path: Arc<Absyn::Path>, mut info: SourceInfo) -> Result<()> {
    let mut dirs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    dirs2 = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); ArcStr::from(__mm_s) }).clone(), cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("/usr/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); ArcStr::from(__mm_s) }).clone(), cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); ArcStr::from(__mm_s) }).clone(), cons((literal!("/usr/lib/")).clone(), cons((literal!("/lib/")).clone(), dirs.clone())))));
    if !(({
        let mut __acc: Option<bool> = None;
        for mut d in (dirs2.clone()).into_iter().cloned() {
            for mut n in (names.clone()).into_iter().cloned() {
                let __x = System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*d.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone());
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
            }
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    })) {
        let () = (match resources.clone() {
        Some(mut resourcesStr) => {
            let mut tmpdir: ArcStr = arcstr::literal!("");
            let mut cmd: ArcStr = arcstr::literal!("");
            let mut pwd: ArcStr = arcstr::literal!("");
            let mut contents: ArcStr = arcstr::literal!("");
            let mut found: ArcStr = arcstr::literal!("");
            let mut status: i32 = 0;
            let mut didFind: bool = false;
            if System::directoryExists((resourcesStr.clone()).clone()) {
                didFind = false;
                for mut dir in &*({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut dir in (System::subDirectories(({ let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesStr.clone()); __mm_s.push_str(&*literal!("/BuildProjects")); ArcStr::from(__mm_s) }).clone())).into_iter().cloned() {
            if !(System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesStr.clone()); __mm_s.push_str(&*literal!("/BuildProjects/")); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("/autogen.sh")); ArcStr::from(__mm_s) }).clone())) { continue; }
            let __x = dir.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) {
                    let mut dir = dir.clone();
                    tmpdir = (System::createTemporaryDirectory(({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getTempDirectoryPath()); __mm_s.push_str(&*literal!("/omc_compile_")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); ArcStr::from(__mm_s) }).clone())?).clone();
                    Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created directory ")); __mm_s.push_str(&*tmpdir.clone()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                    cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cp -a \"")); __mm_s.push_str(&*resourcesStr.clone()); __mm_s.push_str(&*literal!("\"/* \"")); __mm_s.push_str(&*tmpdir.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![(cmd.clone()).clone()], info.clone())?;
                    System::systemCall((cmd.clone()).clone(), (literal!("")).clone());
                    pwd = (System::pwd()).clone();
                    if 0 == System::cd(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpdir.clone()); __mm_s.push_str(&*literal!("/BuildProjects/")); __mm_s.push_str(&*dir.clone()); ArcStr::from(__mm_s) }).clone()) {
                        Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Changed directory to ")); __mm_s.push_str(&*System::pwd()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                        cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("sh ./autogen.sh && ./configure --libdir='")); __mm_s.push_str(&*userCompiledBinariesDirectory(path.clone())); __mm_s.push_str(&*literal!("' && make && make install")); ArcStr::from(__mm_s) }).clone();
                        status = System::systemCall((cmd.clone()).clone(), (literal!("log")).clone());
                        contents = (System::readFile((literal!("log")).clone())?).clone();
                        if status.clone() != 0 {
                            Error::addSourceMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run ")); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*contents.clone()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                        } else {
                            Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Succeeded with compilation and installation of the library using:\ncommand: ")); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*contents.clone()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                            didFind = true;
                            /* todo stmt: multi-iterator-for */
                            if didFind.clone() {
                                found = listHead(({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut x in (List::flatten(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut d in (dirs2.clone()).into_iter().cloned() {
            for mut n in (names.clone()).into_iter().cloned() {
                let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*d.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) };
                __acc = cons(__x, __acc);
            }
        }
        __acc.reverse()
    }))).into_iter().cloned() {
            if !(System::regularFileExists(x.clone())) { continue; }
            let __x = x.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                                Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Compiled ")); __mm_s.push_str(&*found.clone()); __mm_s.push_str(&*literal!(" by running build project ")); __mm_s.push_str(&*resourcesStr.clone()); __mm_s.push_str(&*literal!("/BuildProjects/")); __mm_s.push_str(&*dir.clone()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                            }
                        }
                    } else {
                        Error::addSourceMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to change directory to ")); __mm_s.push_str(&*tmpdir.clone()); __mm_s.push_str(&*literal!("/BuildProjects/")); __mm_s.push_str(&*dir.clone()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                    }
                    System::cd((pwd.clone()).clone());
                    System::removeDirectory((tmpdir.clone()).clone());
                    Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Removed directory ")); __mm_s.push_str(&*tmpdir.clone()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                    if didFind.clone() {
                        break;
                    }
                }
            }
            ()
        },
        _ => {
            ()
        },
    });
        if !(({
        let mut __acc: Option<bool> = None;
        for mut d in (dirs2.clone()).into_iter().cloned() {
            for mut n in (names.clone()).into_iter().cloned() {
                let __x = System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*d.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone());
                __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
            }
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    })) {
            if !(Testsuite::isRunning()?) {
                Error::addSourceMessage(Error::EXT_LIBRARY_NOT_FOUND.clone(), list![(name.clone()).clone(), (({
        let mut __acc = String::new();
        for mut d in (dirs2.clone()).into_iter().cloned() {
            for mut n in (names.clone()).into_iter().cloned() {
                let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*d.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) };
                __acc.push_str(&__x);
            }
        }
        ArcStr::from(__acc)
    })).clone()], info.clone())?;
            }
        }
    }
    Ok(())
}

fn generateExtFunctionIncludeDirectoryFlags(mut program: Absyn::Program, mut path: Arc<Absyn::Path>, mut inMod: Arc<SCode::Mod>, mut includes: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outDirs = 'mc: {
        let __mc_input = includes.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut istr: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("IncludeDirectory")).clone())?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::STRING { value: __pa0 }), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    r#str = (ProgramUtil::getFullPathFromUri(program.clone(), (r#str.clone()).clone(), false)?).clone();
                    istr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"-I")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                    Ok(if (System::directoryExists((r#str.clone()).clone())) {list![(istr.clone()).clone()]} else {metamodelica::nil()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut istr: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path.clone())?); __mm_s.push_str(&*literal!("/Resources/Include")); ArcStr::from(__mm_s) }).clone();
                    r#str = (ProgramUtil::getFullPathFromUri(program.clone(), (r#str.clone()).clone(), false)?).clone();
                    istr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"-I")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                    Ok(if (System::directoryExists((r#str.clone()).clone())) {list![(istr.clone()).clone()]} else {metamodelica::nil()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDirs)
}

fn getLinkerLibraryPaths(mut uri: ArcStr, mut path: Arc<Absyn::Path>, mut inLibs: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut installationDir: ArcStr = arcstr::literal!("");
    installationDir = (Settings::getInstallationDirectoryPath()?).clone();
    let () = 'mc: {
        let __mc_input = (uri.clone(), path.clone(), inLibs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: Deref @ "-lWinmm", tail: Deref @ metamodelica::List::Nil }) => {
                    if !((arcstr::literal!(Autoconf::os) == literal!("Windows_NT"))) { bail!("guard") }
                    let mut libPaths: Arc<metamodelica::List<ArcStr>> = libPaths.clone();
                    libPaths = list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*installationDir.clone()); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); ArcStr::from(__mm_s) }).clone()];
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut libPaths: Arc<metamodelica::List<ArcStr>> = libPaths.clone();
                    libPaths = list![(uri.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*uri.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::modelicaPlatform()); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*uri.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::openModelicaPlatform()); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*uri.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::openModelicaPlatformAlternative()); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getHomeDir(false)); __mm_s.push_str(&*literal!("/.openmodelica/binaries/")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path.clone())?); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*installationDir.clone()); __mm_s.push_str(&*literal!("/lib/")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*installationDir.clone()); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); ArcStr::from(__mm_s) }).clone()];
                    if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
                        libPaths = List::appendElt(({ let mut __mm_s = String::new(); __mm_s.push_str(&*installationDir.clone()); __mm_s.push_str(&*literal!("/bin/")); ArcStr::from(__mm_s) }).clone(), libPaths.clone());
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(libPaths)
}

fn generateExtFunctionLibraryDirectoryFlags(mut program: Absyn::Program, mut path: Arc<Absyn::Path>, mut inMod: Arc<SCode::Mod>, mut inLibs: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Option<ArcStr>)> {
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut installDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut resources: Option<ArcStr> = None;
    (outLibs, installDirs, resources) = 'mc: {
        let __mc_input = inLibs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                libs => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut target: ArcStr = arcstr::literal!("");
                    let mut resourcesStr: ArcStr = arcstr::literal!("");
                    let mut libs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut isLinux: bool = false;
                    let mut libs = (*libs).clone();
                    r#str = ('mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            let mut r#str: ArcStr;
                            let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("LibraryDirectory")).clone())?) {
                                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::STRING { value: __pa0 }), .. } => __pa0.clone(),
                                        _ => bail!("pattern mismatch"),
                            } };
                            r#str = __pa0.clone();
                            Ok(r#str.clone())
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path.clone())?); __mm_s.push_str(&*literal!("/Resources/Library")); ArcStr::from(__mm_s) })
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
                    r#str = (ProgramUtil::getFullPathFromUri(program.clone(), (r#str.clone()).clone(), false)?).clone();
                    resourcesStr = (ProgramUtil::getFullPathFromUri(program.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path.clone())?); __mm_s.push_str(&*literal!("/Resources")); ArcStr::from(__mm_s) }).clone(), false)?).clone();
                    isLinux = stringEq((literal!("linux")).clone(), (arcstr::literal!(Autoconf::os)).clone());
                    target = (Flags::getConfigString(Flags::TARGET.clone())?).clone();
                    libs2 = getLinkerLibraryPaths((r#str.clone()).clone(), path.clone(), inLibs.clone())?;
                    libs = List::fold2(libs2.clone(), (std::sync::Arc::new(generateExtFunctionLibraryDirectoryFlags2) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, bool, ArcStr, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), isLinux.clone(), (target.clone()).clone(), libs.clone());
                    Ok((libs.clone(), libs2.clone().reverse(), Some((resourcesStr.clone()).clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inLibs.clone(), metamodelica::nil(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outLibs, installDirs, resources))
}

fn generateExtFunctionLibraryDirectoryFlags2(mut dir: ArcStr, mut isLinux: bool, mut target: ArcStr, mut inLibs: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    libs = if (isLinux.clone()) {cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Wl,-rpath=\"")); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone(), inLibs.clone())} else {inLibs.clone()};
    libs = cons((if (getGerneralTarget((target.clone()).clone())? == literal!("msvc")) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("/LIBPATH:\"")); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"-L")); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }}).clone(), libs.clone());
    Ok(libs)
}

fn getGerneralTarget(mut target: ArcStr) -> Result<ArcStr> {
    let mut generalTarget: ArcStr = arcstr::literal!("");
    generalTarget = (if (System::stringFind((target.clone()).clone(), (literal!("msvc")).clone())? == 0) {literal!("msvc")} else {target.clone()}).clone();
    Ok(generalTarget)
}

fn userCompiledBinariesDirectory(mut path: Arc<Absyn::Path>) -> ArcStr {
    let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getHomeDir(false)); __mm_s.push_str(&*literal!("/.openmodelica/binaries/")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path.clone()).unwrap()); ArcStr::from(__mm_s) };
    r#str
}

fn generateExtFunctionLibraryDirectoryPaths(mut program: Absyn::Program, mut path: Arc<Absyn::Path>, mut inMod: Arc<SCode::Mod>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outLibs = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut platform1: ArcStr = arcstr::literal!("");
                    let mut platform2: ArcStr = arcstr::literal!("");
                    let mut platform3: ArcStr = arcstr::literal!("");
                    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut isLinux: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("LibraryDirectory")).clone())?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::STRING { value: __pa0 }), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    r#str = (ProgramUtil::getFullPathFromUri(program.clone(), (r#str.clone()).clone(), false)?).clone();
                    platform1 = (System::openModelicaPlatform()).clone();
                    platform2 = (System::openModelicaPlatformAlternative()).clone();
                    platform3 = (System::modelicaPlatform()).clone();
                    isLinux = stringEq((literal!("linux")).clone(), (arcstr::literal!(Autoconf::os)).clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(true, (r#str.clone()).clone(), isLinux.clone(), metamodelica::nil());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq((platform3.clone()).clone(), (literal!("")).clone())), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform3.clone()); ArcStr::from(__mm_s) }).clone(), isLinux.clone(), libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq((platform2.clone()).clone(), (literal!("")).clone())), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform2.clone()); ArcStr::from(__mm_s) }).clone(), isLinux.clone(), libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq((platform1.clone()).clone(), (literal!("")).clone())), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform1.clone()); ArcStr::from(__mm_s) }).clone(), isLinux.clone(), libs.clone());
                    Ok(libs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut platform1: ArcStr = arcstr::literal!("");
                    let mut platform2: ArcStr = arcstr::literal!("");
                    let mut platform3: ArcStr = arcstr::literal!("");
                    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut isLinux: bool = false;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path.clone())?); __mm_s.push_str(&*literal!("/Resources/Library")); ArcStr::from(__mm_s) }).clone();
                    r#str = (ProgramUtil::getFullPathFromUri(program.clone(), (r#str.clone()).clone(), false)?).clone();
                    platform1 = (System::openModelicaPlatform()).clone();
                    platform2 = (System::openModelicaPlatformAlternative()).clone();
                    platform3 = (System::modelicaPlatform()).clone();
                    isLinux = stringEq((literal!("linux")).clone(), (arcstr::literal!(Autoconf::os)).clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(true, (r#str.clone()).clone(), isLinux.clone(), metamodelica::nil());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq((platform3.clone()).clone(), (literal!("")).clone())), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform3.clone()); ArcStr::from(__mm_s) }).clone(), isLinux.clone(), libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq((platform2.clone()).clone(), (literal!("")).clone())), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform2.clone()); ArcStr::from(__mm_s) }).clone(), isLinux.clone(), libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq((platform1.clone()).clone(), (literal!("")).clone())), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform1.clone()); ArcStr::from(__mm_s) }).clone(), isLinux.clone(), libs.clone());
                    Ok(libs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLibs)
}

fn generateExtFunctionLibraryDirectoryPaths2(mut add: bool, mut dir: ArcStr, mut isLinux: bool, mut inLibs: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    libs = (::match_deref::match_deref! { match &((add.clone(), inLibs.clone())) {
        (true, libs) => {
            let mut b: bool = false;
            let mut libs = (*libs).clone();
            b = System::directoryExists((dir.clone()).clone());
            libs = List::consOnTrue(b.clone(), (dir.clone()).clone(), libs.clone());
            libs.clone()
        },
        _ => {
            inLibs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    libs
}

fn getLibraryStringInMSVCFormat(mut exp: Arc<Absyn::Exp>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (strs, names) = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str } => {
                    if !((r#str.clone() == literal!("Lapack") || r#str.clone() == literal!("lapack"))) { bail!("guard") }
                    Ok((list![(literal!("lapack_win32_MT.lib")).clone(), (literal!("f2c.lib")).clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "omcruntime" } => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = strs.clone();
                    let true = (literal!("Windows_NT") == arcstr::literal!(Autoconf::os)) else { bail!("pattern mismatch") };
                    strs = list![(literal!("f2c.lib")).clone(), (literal!("initialization.lib")).clone(), (literal!("libexpat.lib")).clone(), (literal!("math-support.lib")).clone(), (literal!("meta.lib")).clone(), (literal!("ModelicaExternalC.lib")).clone(), (literal!("results.lib")).clone(), (literal!("simulation.lib")).clone(), (literal!("solver.lib")).clone(), (literal!("sundials_kinsol.lib")).clone(), (literal!("sundials_nvecserial.lib")).clone(), (literal!("sundials_sunlinsolklu")).clone(), (literal!("util.lib")).clone(), (literal!("lapack_win32_MT.lib")).clone()];
                    Ok((strs.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "OpenModelicaCorba" } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (arcstr::literal!(Autoconf::corbaLibs)).clone();
                    Ok((list![(r#str.clone()).clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "fmilib" } => {
                    Ok((list![(literal!("fmilib.lib")).clone(), (literal!("shlwapi.lib")).clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str } => {
                    let true = (literal!("-") == stringGetStringChar((r#str.clone()).clone(), 1)?) else { bail!("pattern mismatch") };
                    Ok((list![(r#str.clone()).clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str } => {
                    let mut r#str = (*r#str).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(".lib")); ArcStr::from(__mm_s) }).clone();
                    Ok((list![(r#str.clone()).clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("Failed to process Library annotation for external function")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((strs, names))
}

fn getLibraryStringInGccFormat(mut exp: Arc<Absyn::Exp>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (strs, names) = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "lapack" } => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "Lapack" } => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "pthread" } => {
                    if !((arcstr::literal!(Autoconf::os) == literal!("Windows_NT"))) { bail!("guard") }
                    Error::addCompilerNotification((literal!("pthreads library is already available. It is not linked from the external library resource directory.\n")).clone())?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "rt" } => {
                    if !((arcstr::literal!(Autoconf::os) == literal!("Windows_NT"))) { bail!("guard") }
                    Error::addCompilerNotification((literal!("rt library is not needed under Windows. It is not linked from the external library resource directory.\n")).clone())?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "Ws2_32" } => {
                    if !((arcstr::literal!(Autoconf::os) == literal!("Windows_NT"))) { bail!("guard") }
                    Error::addCompilerNotification((literal!("Ws2_32 library is not needed under Windows. It is not linked from the external library resource directory.\n")).clone())?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "User32" } => {
                    if !((arcstr::literal!(Autoconf::os) == literal!("Windows_NT"))) { bail!("guard") }
                    Error::addCompilerNotification((literal!("User32 library is already available. It is not linked from the external library resource directory.\n")).clone())?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str @ Deref @ "Winmm" } => {
                    if !((arcstr::literal!(Autoconf::os) == literal!("Windows_NT"))) { bail!("guard") }
                    let mut r#str = (*r#str).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-l")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addCompilerNotification((literal!("Winmm library is a windows system library. It is not linked from the external library resource directory.\n")).clone())?;
                    Ok((list![(r#str.clone()).clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "X11" } => {
                    if !((arcstr::literal!(Autoconf::os) == literal!("Windows_NT"))) { bail!("guard") }
                    Error::addCompilerNotification((literal!("X11 library is not needed under Windows. It is not linked from the external library resource directory.\n")).clone())?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str @ Deref @ "omcruntime" } => {
                    let mut r#str = (*r#str).clone();
                    let mut strs: Arc<metamodelica::List<ArcStr>> = strs.clone();
                    if literal!("Windows_NT") == arcstr::literal!(Autoconf::os) {
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-l")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                        strs = cons((r#str.clone()).clone(), cons((literal!("-lintl")).clone(), cons((literal!("-liconv")).clone(), cons((literal!("-lexpat")).clone(), cons((literal!("-lsqlite3")).clone(), cons((literal!("-ltre")).clone(), cons((literal!("-lws2_32")).clone(), cons((literal!("-lRpcrt4")).clone(), cons((literal!("-lregex")).clone(), metamodelica::nil())))))))));
                    } else {
                        strs = Autoconf::systemLibs.clone();
                    }
                    Ok((strs.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "OpenModelicaCorba" } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (arcstr::literal!(Autoconf::corbaLibs)).clone();
                    Ok((list![(r#str.clone()).clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "fmilib" } => {
                    Ok((if (arcstr::literal!(Autoconf::os) == literal!("Windows_NT")) {list![(literal!("-lfmilib")).clone(), (literal!("-lshlwapi")).clone()]} else {list![(literal!("-lfmilib")).clone()]}, metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str } => {
                    let mut strs1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strs3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut names1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut names2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut names3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strs: Arc<metamodelica::List<ArcStr>> = strs.clone();
                    let mut names: Arc<metamodelica::List<ArcStr>> = names.clone();
                    if r#str.clone() == literal!("ModelicaStandardTables") {
                        (strs1, names1) = getLibraryStringInGccFormat(Arc::new(Absyn::Exp::STRING { value: (literal!("ModelicaIO")).clone() }))?;
                        (strs2, names2) = getLibraryStringInGccFormat(Arc::new(Absyn::Exp::STRING { value: (literal!("ModelicaMatIO")).clone() }))?;
                        (strs3, names3) = getLibraryStringInGccFormat(Arc::new(Absyn::Exp::STRING { value: (literal!("zlib")).clone() }))?;
                        strs = listAppend(strs1.clone(), listAppend(strs2.clone(), strs3.clone()));
                        names = listAppend(names1.clone(), listAppend(names2.clone(), names3.clone()));
                    } else {
                        strs = metamodelica::nil();
                        names = metamodelica::nil();
                    }
                    if System::regularFileExists((r#str.clone()).clone()) || literal!("-") == stringGetStringChar((r#str.clone()).clone(), 1)? {
                        strs = cons((r#str.clone()).clone(), strs.clone());
                    } else {
                        strs = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-l")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone(), strs.clone());
                        names = cons((r#str.clone()).clone(), names.clone());
                    }
                    Ok((strs.clone(), names.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("Failed to process Library annotation for external function")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((strs, names))
}

fn generateExtFunctionIncludesLibstr(mut target: ArcStr, mut inMod: Arc<SCode::Mod>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outStringLst, names) = 'mc: {
        let __mc_input = getGerneralTarget((target.clone()).clone())?;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "msvc" => {
                    let mut arr: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut libsList: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut namesList: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("Library")).clone())?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::ARRAY { arrayExp: __pa0 }), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    arr = __pa0.clone();
                    (libsList, namesList) = List::map_2(arr.clone(), (std::sync::Arc::new(getLibraryStringInMSVCFormat) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> + 'static>));
                    Ok((List::flatten(libsList.clone()), List::flatten(namesList.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "msvc" => {
                    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut names: Arc<metamodelica::List<ArcStr>> = names.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("Library")).clone())?) {
                        Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    (libs, names) = getLibraryStringInMSVCFormat(exp.clone())?;
                    Ok((libs.clone(), names.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut arr: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut libsList: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut namesList: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("Library")).clone())?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::ARRAY { arrayExp: __pa0 }), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    arr = __pa0.clone();
                    (libsList, namesList) = List::map_2(arr.clone(), (std::sync::Arc::new(getLibraryStringInGccFormat) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> + 'static>));
                    Ok((List::flatten(libsList.clone()), List::flatten(namesList.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut names: Arc<metamodelica::List<ArcStr>> = names.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("Library")).clone())?) {
                        Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    (libs, names) = getLibraryStringInGccFormat(exp.clone())?;
                    Ok((libs.clone(), names.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStringLst, names))
}

fn generateExtFunctionIncludesIncludestr(mut inMod: Arc<SCode::Mod>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    includes = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut inc: ArcStr = arcstr::literal!("");
                    let mut inc_1: ArcStr = arcstr::literal!("");
                    let mut lineNumberStart: i32 = 0;
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut includes: Arc<metamodelica::List<ArcStr>> = includes.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("Include")).clone())?) {
                        Deref @ SCode::Mod::MOD { info: SourceInfo { lineNumberStart: __pa0, fileName: __pa1, .. }, binding: Some(Deref @ Absyn::Exp::STRING { value: __pa2 }), .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lineNumberStart = __pa0.clone();
                    fileName = __pa1.clone();
                    inc = __pa2.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#line ")); __mm_s.push_str(&*intString(lineNumberStart.clone())); __mm_s.push_str(&*literal!(" \"")); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                    inc_1 = (System::unescapedString((inc.clone()).clone())).clone();
                    includes = if (Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?) {list![(r#str.clone()).clone(), (inc_1.clone()).clone()]} else {list![(inc_1.clone()).clone()]};
                    Ok(includes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(includes)
}

fn generateExtFunctionDynamicLoad(mut inMod: Arc<SCode::Mod>) -> Result<bool> {
    let mut outDynamicLoad: bool = false;
    outDynamicLoad = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod.clone(), (literal!("DynamicLoad")).clone())?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __pa0 }), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    b = __pa0.clone();
                    Ok(b.clone())
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
    Ok(outDynamicLoad)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getImplicitRecordConstructors(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = 'mc: {
        let __mc_input = inExpLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: record_path }, .. }, componentRef: cref }, tail: rest_expr } => {
                    let mut record_cref: Arc<DAE::Exp>;
                    let mut cref = (*cref).clone();
                    let mut rest_expr = (*rest_expr).clone();
                    ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(cref.clone())?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    cref = ComponentReference::pathToCref(record_path.clone())?;
                    record_cref = Expression::crefExp(cref.clone())?;
                    rest_expr = getImplicitRecordConstructors(rest_expr.clone())?;
                    Ok(cons(record_cref.clone(), rest_expr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_expr } => {
                    let mut rest_expr = (*rest_expr).clone();
                    rest_expr = getImplicitRecordConstructors(rest_expr.clone())?;
                    Ok(rest_expr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getCalledFunctionsInFunctions(mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)), mut funcs: Arc<AvlTreePathFunction::Tree>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    outHt = (::match_deref::match_deref! { match &((paths.clone(), inHt.clone())) {
        (Deref @ metamodelica::List::Nil, ht) => {
            ht.clone()
        },
        (Deref @ metamodelica::List::Cons { head: path, tail: rest }, ht) => {
            let mut ht = (*ht).clone();
            ht = getCalledFunctionsInFunction2(path.clone(), AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?, ht.clone(), funcs.clone())?;
            ht = getCalledFunctionsInFunctions(rest.clone(), ht.clone(), funcs.clone())?;
            ht.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outHt)
}

pub fn getCalledFunctionsInFunction2(mut inPath: Arc<Absyn::Path>, mut pathstr: ArcStr, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)), mut funcs: Arc<AvlTreePathFunction::Tree>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    outHt = 'mc: {
        let __mc_input = (inPath.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ht) => {
                    if !((BaseHashTable::hasKey((pathstr.clone()).clone(), ht.clone()))) { bail!("guard") }
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, ht) => {
                    let mut funcelem: DAE::Function;
                    let mut calledfuncs: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut varfuncs: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut ht = (*ht).clone();
                    funcelem = DAEUtil::getNamedFunction(path.clone(), funcs.clone())?;
                    els = DAEUtil::getFunctionElements(funcelem.clone())?;
                    varfuncs = List::fold(els.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::collectFunctionRefVarPaths, Arc<DAE::Element>, Arc<metamodelica::List<Arc<Absyn::Path>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> + 'static>), metamodelica::nil());
                    let (_, (_, __pa0)) = DAEUtil::traverseDAEElementList(els.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(DAEUtil::collectValueblockFunctionRefVars, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Absyn::Path>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> + 'static>), varfuncs.clone()));
                    varfuncs = __pa0.clone();
                    let (_, (_, (__pa1, _))) = DAEUtil::traverseDAEElementList(els.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(matchNonBuiltinCallsAndFnRefPaths) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>))> + 'static>), (metamodelica::nil(), varfuncs.clone())));
                    calledfuncs = __pa1.clone();
                    ht = BaseHashTable::add((pathstr.clone(), path.clone()), ht.clone())?;
                    ht = addDestructor(funcelem.clone(), ht.clone())?;
                    ht = getCalledFunctionsInFunctions(calledfuncs.clone(), ht.clone(), funcs.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        unwrap_break_err!(DAEUtil::getNamedFunction(path.clone(), funcs.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function getCalledFunctionsInFunction2: Class ")); __mm_s.push_str(&*pathstr.clone()); __mm_s.push_str(&*literal!(" not found in global scope.")); ArcStr::from(__mm_s) }).clone();
                    Error::addInternalError((r#str.clone()).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHt)
}

fn addDestructor(mut func: DAE::Function, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    outHt = (::match_deref::match_deref! { match &(func.clone()) {
        DAE::Function::FUNCTION { type_: Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path }, .. }, .. }, .. } => {
            let mut path = (*path).clone();
            path = AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("destructor")).clone() }))?;
            addDestructor2(path.clone(), AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?, inHt.clone())?
        },
        _ => {
            inHt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outHt)
}

fn addDestructor2(mut path: Arc<Absyn::Path>, mut pathstr: ArcStr, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))> {
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)) = inHt.clone();
    if !(BaseHashTable::hasKey((pathstr.clone()).clone(), ht.clone())) {
        ht = BaseHashTable::add((pathstr.clone(), path.clone()), ht.clone())?;
    }
    Ok(ht)
}

fn matchNonBuiltinCallsAndFnRefPaths(mut inExp: Arc<DAE::Exp>, mut itpl: (Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut otpl: (Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>);
    (outExp, otpl) = 'mc: {
        let __mc_input = (inExp.clone(), itpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, path, .. }, (acc, filter)) => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeNotFullyQualified(path.clone());
                    let false = (List::isMemberOnTrue(path.clone(), filter.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), (cons(path.clone(), acc.clone()), filter.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path, .. }, .. }, (acc, filter)) => {
                    let false = (List::isMemberOnTrue(path.clone(), list![Arc::new(Absyn::Path::IDENT { name: (literal!("list")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("listReverse")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("min")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("sum")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("product")).clone() })], (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    let false = (List::isMemberOnTrue(path.clone(), filter.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), (cons(path.clone(), acc.clone()), filter.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { path, .. }, (acc, filter)) => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeNotFullyQualified(path.clone());
                    let false = (List::isMemberOnTrue(path.clone(), filter.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), (cons(path.clone(), acc.clone()), filter.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: false, .. }, .. }, (acc, filter)) => {
                    let mut path: Arc<Absyn::Path>;
                    path = AbsynUtil::crefToPath(getCrefFromExp(inExp.clone())?)?;
                    let false = (List::isMemberOnTrue(path.clone(), filter.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), (cons(path.clone(), acc.clone()), filter.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, otpl))
}

fn aliasRecordDeclarations(mut inDecl: SimCodeFunction::RecordDeclaration, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(SimCodeFunction::RecordDeclaration, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)))> {
    let mut decl: SimCodeFunction::RecordDeclaration;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    (decl, ht) = (match inDecl.clone() {
        SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: mut sname, aliasName: _, defPath: ref name, variables: ref vars, usedExternally: mut extConvert } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut alias: Option<ArcStr> = None;
            r#str = stringDelimitList(List::map(vars.clone(), (std::sync::Arc::new(variableString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCodeFunction::Variable::Variable>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone());
            (alias, ht) = aliasRecordDeclarations2((r#str.clone()).clone(), name.clone(), inHt.clone())?;
            (SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: (sname.clone()).clone(), aliasName: alias.clone(), defPath: name.clone(), variables: vars.clone(), usedExternally: extConvert.clone() }, ht.clone())
        },
        _ => {
            (inDecl.clone(), inHt.clone())
        },
    });
    Ok((decl, ht))
}

fn aliasRecordDeclarations2(mut r#str: ArcStr, mut path: Arc<Absyn::Path>, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(Option<ArcStr>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)))> {
    let mut alias: Option<ArcStr> = None;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    (alias, ht) = 'mc: {
        let __mc_input = inHt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut aliasStr: ArcStr = arcstr::literal!("");
            aliasStr = (AbsynUtil::pathStringUnquoteReplaceDot(BaseHashTable::get((r#str.clone()).clone(), inHt.clone())?, (literal!("_")).clone())?).clone();
            Ok((Some((aliasStr.clone()).clone()), inHt.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
            ht = BaseHashTable::add((r#str.clone(), path.clone()), inHt.clone())?;
            Ok((None, ht.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((alias, ht))
}

fn variableString(mut var: Arc<SimCodeFunction::Variable::Variable>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(var.clone()) {
        Deref @ SimCodeFunction::Variable::VARIABLE { ty, name, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*TypesDump::unparseType(ty.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(name.clone())?); ArcStr::from(__mm_s) }
        },
        Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: r#str, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica_fnptr ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn createMakefileParams(mut includes: Arc<metamodelica::List<ArcStr>>, mut libs: Arc<metamodelica::List<ArcStr>>, mut libPaths: Arc<metamodelica::List<ArcStr>>, mut isFunction: bool, mut isFMU: bool) -> Result<SimCodeFunction::MakefileParams> {
    let mut makefileParams: SimCodeFunction::MakefileParams = <SimCodeFunction::MakefileParams as ::std::default::Default>::default();
    let mut omhome: ArcStr = arcstr::literal!("");
    let mut ccompiler: ArcStr = arcstr::literal!("");
    let mut cxxcompiler: ArcStr = arcstr::literal!("");
    let mut linker: ArcStr = arcstr::literal!("");
    let mut exeext: ArcStr = arcstr::literal!("");
    let mut dllext: ArcStr = arcstr::literal!("");
    let mut cflags: ArcStr = arcstr::literal!("");
    let mut ldflags: ArcStr = arcstr::literal!("");
    let mut rtlibs: ArcStr = arcstr::literal!("");
    let mut platform: ArcStr = arcstr::literal!("");
    let mut compileDir: ArcStr = arcstr::literal!("");
    ccompiler = (if (stringEq((Config::simCodeTarget()?).clone(), (literal!("JavaScript")).clone())) {literal!("emcc")} else {if (Flags::isSet(Flags::HPCOM.clone())?) {System::getOMPCCompiler()} else {System::getCCompiler()}}).clone();
    cxxcompiler = (if (stringEq((Config::simCodeTarget()?).clone(), (literal!("JavaScript")).clone())) {literal!("emcc")} else {System::getCXXCompiler()}).clone();
    linker = (if (stringEq((Config::simCodeTarget()?).clone(), (literal!("JavaScript")).clone())) {literal!("emcc")} else {System::getLinker()}).clone();
    exeext = (if (stringEq((Config::simCodeTarget()?).clone(), (literal!("JavaScript")).clone())) {literal!(".js")} else {arcstr::literal!(Autoconf::exeExt)}).clone();
    dllext = (arcstr::literal!(Autoconf::dllExt)).clone();
    omhome = (Settings::getInstallationDirectoryPath()?).clone();
    omhome = (System::trim((omhome.clone()).clone(), (literal!("\"")).clone())).clone();
    cflags = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::getCFlags()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*if (Flags::isSet(Flags::HPCOM.clone())?) {literal!("-fopenmp")} else {literal!("")}); ArcStr::from(__mm_s) }).clone();
    cflags = (if (stringEq((Config::simCodeTarget()?).clone(), (literal!("JavaScript")).clone())) {literal!("-Os -Wno-warn-absolute-paths")} else {cflags.clone()}).clone();
    ldflags = (System::getLDFlags()).clone();
    if Flags::getConfigBool(Flags::PARMODAUTO.clone())? {
        ldflags = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" -lParModelicaAuto -ltbb_static ")); __mm_s.push_str(&*ldflags.clone()); ArcStr::from(__mm_s) }).clone();
    }
    rtlibs = (if (isFunction.clone()) {arcstr::literal!(Autoconf::ldflags_runtime)} else {if (isFMU.clone()) {arcstr::literal!(Autoconf::ldflags_runtime_fmu)} else {arcstr::literal!(Autoconf::ldflags_runtime_sim)}}).clone();
    platform = (System::modelicaPlatform()).clone();
    compileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }).clone();
    makefileParams = SimCodeFunction::MakefileParams { ccompiler: (ccompiler.clone()).clone(), cxxcompiler: (cxxcompiler.clone()).clone(), linker: (linker.clone()).clone(), exeext: (exeext.clone()).clone(), dllext: (dllext.clone()).clone(), omhome: (omhome.clone()).clone(), cflags: (cflags.clone()).clone(), ldflags: (ldflags.clone()).clone(), runtimelibs: (rtlibs.clone()).clone(), includes: includes.clone(), libs: libs.clone(), libPaths: libPaths.clone(), platform: (platform.clone()).clone(), compileDir: (compileDir.clone()).clone() };
    Ok(makefileParams)
}

pub fn codegenResetTryThrowIndex() -> () {
    { let __v = metamodelica::nil(); openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| *__root.borrow_mut() = __v) };
    ()
}

pub fn codegenPushTryThrowIndex(mut i: i32) -> () {
    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    lst = openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| __root.borrow().clone());
    { let __v = cons(i.clone(), lst.clone()); openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| *__root.borrow_mut() = __v) };
    ()
}

pub fn codegenPopTryThrowIndex() -> Result<()> {
    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    lst = openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| __root.borrow().clone());
    let __pa0 = ::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lst = __pa0.clone();
    { let __v = lst.clone(); openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| *__root.borrow_mut() = __v) };
    Ok(())
}

pub fn codegenPeekTryThrowIndex() -> i32 {
    let mut i: i32 = 0;
    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    lst = openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| __root.borrow().clone());
    i = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Cons { head: i, tail: _ } => i.clone(),
        _ => -1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    i
}

pub fn varIndex(mut var: SimCodeVar::SimVar) -> Result<i32> {
    let mut index: i32 = 0;
    let SimCodeVar::SIMVAR { index: __pa0, .. } = (var.clone()) else { bail!("pattern mismatch") };
    index = __pa0.clone();
    Ok(index)
}

pub fn isParallelFunctionContext(mut context: SimCodeFunction::Context) -> bool {
    let mut outBool: bool = false;
    outBool = (match context.clone() {
        SimCodeFunction::Context::FUNCTION_CONTEXT { .. } => var_field!(context.is_parallel, SimCodeFunction::Context::FUNCTION_CONTEXT).clone(),
        _ => false,
    });
    outBool
}

pub fn twodigit(mut i: i32) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    outS = ('mc: {
        let __mc_input = i.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if !((i.clone() < 10)) { bail!("guard") }
            let mut s: ArcStr = arcstr::literal!("");
            s = (intString(i.clone())).clone();
            s = (stringAppend((literal!("0")).clone(), (s.clone()).clone())).clone();
            Ok(s.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(intString(i.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outS)
}

pub fn generateSubPalceholders(mut cr: Arc<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outdef: ArcStr = arcstr::literal!("");
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut nrdims: i32 = 0;
    let mut idxstrlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    dims = ComponentReferenceBasics::crefDims(cr.clone())?;
    nrdims = (dims.clone().len() as i32);
    idxstrlst = List::map(List::intRange(nrdims.clone()), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
    outdef = stringDelimitList(List::threadMap(List::fill((literal!("i_")).clone(), nrdims.clone()), idxstrlst.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
    Ok(outdef)
}

/* This functions are used to get/append cref prefixes in function contexts.The cref prefix is appended
  to all crefs generated. We use this to generate dependent names in some cases (for example when generating
  code for record constructors) so that every cref we generate while this is set has this prefix applied to it*/
pub fn getCurrentCrefPrefix(mut context: SimCodeFunction::Context) -> Result<ArcStr> {
    let mut cref_pref: ArcStr = arcstr::literal!("");
    cref_pref = ((match context.clone() {
        SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: mut cref_pref, is_parallel: _ } => cref_pref.clone(),
        _ => {
            Error::addInternalError((literal!("Tried to get cref prefix from a non FUNCTION_CONTEXT() context. cref_pref is only avaiable in FUNCTION_CONTEXT.")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    })).clone();
    Ok(cref_pref)
}

pub fn appendCurrentCrefPrefix(mut context: SimCodeFunction::Context, mut in_cref_pref: ArcStr) -> Result<SimCodeFunction::Context> {
    let mut out_context: SimCodeFunction::Context = SimCodeFunction::Context::DAE_MODE_CONTEXT;
    let mut cref_pref: ArcStr = arcstr::literal!("");
    let mut prl: bool = false;
    out_context = (match context.clone() {
        SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: mut cref_pref, is_parallel: mut prl } => SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cref_pref.clone()); __mm_s.push_str(&*in_cref_pref.clone()); ArcStr::from(__mm_s) }).clone(), is_parallel: prl.clone() },
        _ => {
            Error::addInternalError((literal!("Tried to append cref prefix from a non FUNCTION_CONTEXT() context. cref_pref is only avaiable in FUNCTION_CONTEXT.")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(out_context)
}

