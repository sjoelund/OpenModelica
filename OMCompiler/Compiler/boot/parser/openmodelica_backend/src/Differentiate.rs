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
use crate::BackendDAECreate;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendUtil;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::SymbolicJacobian::DAE_CJ;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::Inline;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTpl;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub const defaultMaxIter: i32 = 20;

// =============================================================================
// differentiation interfaces:
//  - createDifferentiatedCrefName
//  - createSeedCrefName
//  - differentiateEquation
//  - differentiateEquationTime
//  - differentiateExpCrefFullJacobian
//  - differentiateExpSolve
//  - differentiateExpTime
// =============================================================================
// =============================================================================
// further interface functions to differentiation
//  - differentiateEquation
//  - differentiateBackendDAE
//
// =============================================================================
// =============================================================================
// main differentiation functions
//  - differentiateExp
//  - differentiateStatements
//
// =============================================================================
fn isDiscreteAssignStatment(mut inStmt: Arc<DAE::Statement>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(inStmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

// =============================================================================
// help functions for differentiation
//  - differentiateCrefs
//  - differentiateCalls
//  - differentiateBinary (e.g.: ADD, SUB, MUL, DIV, POW, ...
//
// =============================================================================
pub fn createDiffedCrefName(mut inCref: Arc<DAE::ComponentRef>, mut inMatrixName: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    subs = ComponentReference::crefLastSubs(inCref.clone())?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?;
    outCref = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), outCref.clone())?;
    outCref = ComponentReference::prependStringCref((inMatrixName.clone()).clone(), outCref.clone())?;
    outCref = ComponentReference::crefSetLastSubs(outCref.clone(), subs.clone())?;
    outCref = ComponentReference::crefSetLastType(outCref.clone(), ComponentReference::crefLastType(inCref.clone())?)?;
    Ok(outCref)
}

pub fn createSeedCrefName(mut inCref: Arc<DAE::ComponentRef>, mut inMatrixName: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let debug: bool = false;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("inCref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after full type  ")); __mm_s.push_str(&*TypesDump::printTypeStr(ComponentReference::crefTypeConsiderSubs(inCref.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    subs = ComponentReference::crefLastSubs(inCref.clone())?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?;
    outCref = ComponentReference::crefSetLastType(outCref.clone(), DAE::T_UNKNOWN_DEFAULT.clone())?;
    outCref = ComponentReference::joinCrefs(outCref.clone(), ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Seed")); __mm_s.push_str(&*inMatrixName.clone()); ArcStr::from(__mm_s) }).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after join: ")); __mm_s.push_str(&*ComponentReference::printComponentRefListStr(ComponentReference::expandCref(outCref.clone(), true)?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    outCref = ComponentReference::crefSetLastSubs(outCref.clone(), subs.clone())?;
    outCref = ComponentReference::crefSetLastType(outCref.clone(), ComponentReference::crefLastType(inCref.clone())?)?;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("outCref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(outCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(outCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isSeedCref(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => StringUtil::startsWith((var_field!((*cr).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (literal!("Seed")).clone()),
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => isSeedCref(var_field!((*cr).componentRef, DAE::ComponentRef::CREF_QUAL).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn createFromNCall2ArgsCall(mut funcName: ArcStr, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut result: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut rest: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(expl.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    rest = __pa2.clone();
    result = Expression::makePureBuiltinCall((funcName.clone()).clone(), list![e1.clone(), e2.clone()], tp.clone());
    for mut elem in &*rest.clone() {
        let mut elem = elem.clone();
        result = Expression::makePureBuiltinCall((funcName.clone()).clone(), list![result.clone(), elem.clone()], tp.clone());
    }
    Ok(result)
}

// =============================================================================
// functions to generate derivative of a function
// =============================================================================
fn addFunctionConstantsAndParameters(mut knownVars_opt: Option<BackendDAE::Variables>, mut func: DAE::Function) -> Result<Option<BackendDAE::Variables>> {
    let mut knownVars_opt: Option<BackendDAE::Variables> = knownVars_opt;
    knownVars_opt = (::match_deref::match_deref! { match &(func.clone()) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body }, tail: _ }, .. } => {
            let mut var_opt: Option<BackendDAE::Var> = None;
            let mut body_knowns: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            for mut element in &*body.clone() {
                let mut element = element.clone();
                var_opt = BackendDAECreate::lowerKnownVarSingle(element.clone())?;
                if isSome(var_opt.clone()) {
                    body_knowns = cons(Util::getOption(var_opt.clone())?, body_knowns.clone());
                }
            }
            if body_knowns.clone().is_empty() {
                knownVars_opt = knownVars_opt.clone();
            } else if isSome(knownVars_opt.clone()) {
                knownVars_opt = Some(BackendVariable::addVars(body_knowns.clone(), Util::getOption(knownVars_opt.clone())?));
            } else {
                knownVars_opt = Some(BackendVariable::listVar(body_knowns.clone()));
            }
            knownVars_opt.clone()
        },
        _ => {
            let mut body_knowns: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            knownVars_opt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(knownVars_opt)
}

fn createPartialArguments(mut outputType: Arc<DAE::Type>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (outputType.clone(), inCall.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: rPath }, .. }, Deref @ DAE::Exp::CALL { path, .. }) => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut varNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    tys = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = DAEUtil::varType(v.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    varNames = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = DAEUtil::typeVarIdent(v.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    expLst = createPartialArgumentsRecord(tys.clone(), varNames.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone())?;
                    Ok(Arc::new(DAE::Exp::RECORD { path: rPath.clone(), exps: expLst.clone(), comp: varNames.clone(), ty: outputType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::CALL { attr, path, .. }, .. }) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: tys, .. }, _) => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expLst = createPartialArgumentsTuple(tys.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone())?;
                    Ok(Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut ezero: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    dims = Expression::arrayDimension(outputType.clone());
                    (ezero, _) = Expression::makeZeroExpression(dims.clone())?;
                    e = createPartialDifferentiatedExp(inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone(), 1, ezero.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CALL { attr, path, .. }) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn createPartialArgumentsTuple(mut inTypesLst: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for (tp, number) in (&(inTypesLst.clone())).into_iter().zip((1..=(inTypesLst.clone().len() as i32)).into_iter()) {
            let __x = createPartialArguments(tp.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), Arc::new(DAE::Exp::TSUB { exp: inCall.clone(), ix: number.clone(), ty: tp.clone() }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outExpLst)
}

fn createPartialArgumentsRecord(mut inTypesLst: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inVarNames: Arc<metamodelica::List<ArcStr>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for (tp, name) in (&(inTypesLst.clone())).into_iter().zip((&(inVarNames.clone())).into_iter()) {
            let __x = createPartialArguments(tp.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), Arc::new(DAE::Exp::RSUB { exp: inCall.clone(), ix: -1, fieldName: (name.clone()).clone(), ty: tp.clone() }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outExpLst)
}

fn createPartialDifferentiatedExp(mut inDiffExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffExplZero: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>, mut currentLstElement: i32, mut inAccum: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = inAccum.clone();
    let mut i: i32 = currentLstElement.clone();
    for mut de in &*inDiffExpl.clone() {
        let mut de = de.clone();
        outExp = (::match_deref::match_deref! { match &((de.clone(), inCall.clone())) {
        (_, Deref @ DAE::Exp::CALL { attr, path, .. }) if (Types::isRecord(Expression::r#typeof(de.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            dexpLst = List::set(inDiffExplZero.clone(), i.clone(), de.clone())?;
            expLst = listAppend(inOrginalExpl.clone(), dexpLst.clone());
            e = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() });
            e.clone()
        },
        (Deref @ DAE::Exp::ARRAY { array: expl, scalar: b, ty: tp }, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut eArray: Arc<DAE::Exp>;
            let mut arrayArgs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            eArray = (inDiffExplZero.clone()).get(i.clone())?;
            dexpLst = Expression::arrayElements(eArray.clone())?;
            arrayArgs = prepareArgumentsExplArray(expl.clone(), dexpLst.clone(), 1, metamodelica::nil())?;
            expLst = List::map2(arrayArgs.clone(), Arc::new(fnptr!(Expression::makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)), tp.clone(), b.clone());
            arrayArgs = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut exp in (expLst.clone()).into_iter().cloned() {
            let __x = List::set(inDiffExplZero.clone(), i.clone(), exp.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            arrayArgs = List::map1r(arrayArgs.clone(), Arc::new(listAppend.clone()), inOrginalExpl.clone());
            e = createPartialSum(arrayArgs.clone(), expl.clone(), inCall.clone(), outExp.clone())?;
            e.clone()
        },
        _ => {
            let mut e: Arc<DAE::Exp>;
            let mut eone: Arc<DAE::Exp>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            tp = Expression::r#typeof(de.clone())?;
            dims = Expression::arrayDimension(tp.clone());
            (eone, _) = Expression::makeOneExpression(dims.clone())?;
            dexpLst = List::set(inDiffExplZero.clone(), i.clone(), eone.clone())?;
            expLst = listAppend(inOrginalExpl.clone(), dexpLst.clone());
            e = createPartialSum(list![expLst.clone()], list![de.clone()], inCall.clone(), outExp.clone())?;
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        i = i.clone() + 1;
    }
    Ok(outExp)
}

fn createPartialSum(mut inArgsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inDiff: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>, mut inAccum: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = inAccum.clone();
    let mut restDiff: Arc<metamodelica::List<Arc<DAE::Exp>>> = inDiff.clone();
    let mut de: Arc<DAE::Exp>;
    let mut res: Arc<DAE::Exp>;
    for mut expLst in &*inArgsLst.clone() {
        let mut expLst = expLst.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(restDiff.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        de = __pa0.clone();
        restDiff = __pa1.clone();
        if !(Expression::isZero(de.clone())) {
            res = (::match_deref::match_deref! { match &(inCall.clone()) {
        Deref @ DAE::Exp::RSUB { ty, fieldName: name, ix, exp: Deref @ DAE::Exp::CALL { attr, path, .. } } => {
            Arc::new(DAE::Exp::RSUB { exp: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() }), ix: ix.clone(), fieldName: (name.clone()).clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::TSUB { ty, ix, exp: Deref @ DAE::Exp::CALL { attr, path, .. } } => {
            Arc::new(DAE::Exp::TSUB { exp: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() }), ix: ix.clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { attr, path, .. } => {
            Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            res = Expression::expMul(de.clone(), res.clone())?;
            outExp = Expression::expAdd(outExp.clone(), res.clone())?;
        }
    }
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn prepareArgumentsExplArray(mut inWorkLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCurrentArg: i32, mut inAccum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>> {
    let mut outExpLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    outExpLstLst = (::match_deref::match_deref! { match &((inWorkLst.clone(), inArgs.clone(), inCurrentArg.clone(), inAccum.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            inAccum.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _) => {
            let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut eone: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            tp = Expression::r#typeof(e.clone())?;
            dims = Expression::arrayDimension(tp.clone());
            (eone, _) = Expression::makeOneExpression(dims.clone())?;
            args = List::set(inArgs.clone(), inCurrentArg.clone(), eone.clone())?;
            prepareArgumentsExplArray(rest.clone(), inArgs.clone(), inCurrentArg.clone() + 1, cons(args.clone(), inAccum.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpLstLst)
}

fn getDiffedTypeandName(mut inFunction: DAE::Function, mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut blst: Arc<metamodelica::List<bool>>) -> Result<(Arc<Absyn::Path>, Arc<DAE::Type>)> {
    let mut diffedName: Arc<Absyn::Path>;
    let mut diffedType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    diffedType = Types::extendsFunctionTypeArgs(DAEUtil::getFunctionType(inFunction.clone())?, inputVarsDer.clone(), outputVarsDer.clone(), blst.clone())?;
    diffedName = AbsynUtil::stringPath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$DER")); __mm_s.push_str(&*BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(DAEUtil::functionName(inFunction.clone())?, (literal!(".")).clone(), true, false)?).clone(), false)?); ArcStr::from(__mm_s) }).clone())?;
    Ok((diffedName, diffedType))
}

fn checkDerivativeFunctionInputs(mut blst: Arc<metamodelica::List<bool>>, mut tp: Arc<DAE::Type>, mut dtp: Arc<DAE::Type>) -> Result<(bool, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outBoolean: bool = false;
    let mut outExpectedTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outBoolean, outExpectedTypeLst) = 'mc: {
        let __mc_input = (blst.clone(), tp.clone(), dtp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_FUNCTION { funcArg: falst, .. }, Deref @ DAE::Type::T_FUNCTION { funcArg: dfalst, .. }) => {
                    let mut falst1: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut falst2: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut tlst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut dtlst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut ret: bool = false;
                    (falst1, _) = List::splitOnBoolList(falst.clone(), blst.clone())?;
                    falst2 = listAppend(falst.clone(), falst1.clone());
                    tlst = List::map(falst2.clone(), Arc::new(Types::funcArgType));
                    dtlst = List::map(dfalst.clone(), Arc::new(Types::funcArgType));
                    ret = List::isEqualOnTrue(tlst.clone(), dtlst.clone(), Arc::new(Types::equivtypes));
                    Ok((ret.clone(), tlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Differentiate.checkDerivativeFunctionInputs failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outBoolean, outExpectedTypeLst))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getFunctionMapper1(mut inFuncDefs: Arc<metamodelica::List<DAE::FunctionDefinition>>) -> Result<DAE::FunctionDefinition> {
    let mut mapper: DAE::FunctionDefinition;
    mapper = 'mc: {
        let __mc_input = inFuncDefs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: m @ DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. }, tail: _ } => {
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: funcDefs } => {
                    let mut m: DAE::FunctionDefinition;
                    m = getFunctionMapper1(funcDefs.clone())?;
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Differentiate.getFunctionMapper1 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(mapper)
}

fn diffableTypes(mut inType: Arc<DAE::Type>) -> bool {
    let mut out: bool = Types::isRealOrSubTypeReal(inType.clone()).unwrap() || Types::isRecord(inType.clone());
    out
}

//
// util functions for Types: DifferentiateInputData, DifferentiateInputArguments, DifferentiationType
//
fn addDependentVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut depVars: BackendDAE::Variables;
    if isSome(outDiffData.dependenentVars.clone()) {
        depVars = BackendVariable::addVars(inVarsLst.clone(), Util::getOption(outDiffData.dependenentVars.clone())?);
    } else {
        depVars = BackendVariable::listVar(inVarsLst.clone());
    }
    outDiffData.dependenentVars = Some(depVars.clone());
    Ok(outDiffData)
}

fn addAllVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut allVars: BackendDAE::Variables;
    if isSome(outDiffData.allVars.clone()) {
        allVars = BackendVariable::addVars(inVarsLst.clone(), Util::getOption(outDiffData.allVars.clone())?);
    } else {
        allVars = BackendVariable::listVar(inVarsLst.clone());
    }
    outDiffData.allVars = Some(allVars.clone());
    Ok(outDiffData)
}

fn addGlobalVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut glVars: BackendDAE::Variables;
    if isSome(outDiffData.knownVars.clone()) {
        glVars = BackendVariable::addVars(inVarsLst.clone(), Util::getOption(outDiffData.knownVars.clone())?);
    } else {
        glVars = BackendVariable::listVar(inVarsLst.clone());
    }
    outDiffData.knownVars = Some(glVars.clone());
    Ok(outDiffData)
}

fn dumpInputData(mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<()> {
    let mut independenentVars: Option<BackendDAE::Variables> = None;
    let mut dependenentVars: Option<BackendDAE::Variables> = None;
    let mut knownVars: Option<BackendDAE::Variables> = None;
    let mut allVars: Option<BackendDAE::Variables> = None;
    let mut controlVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut diffCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut matrixName: Option<ArcStr> = None;
    println!("{}", (literal!("### dumpInputData ###\n")).clone());
    if isSome(inDiffData.matrixName.clone()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### for ")); __mm_s.push_str(&*Util::getOption(inDiffData.matrixName.clone())?); __mm_s.push_str(&*literal!(" ###\n")); ArcStr::from(__mm_s) }).clone());
    }
    if isSome(inDiffData.independenentVars.clone()) {
        println!("{}", (literal!("independentVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.independenentVars.clone())?)?;
    }
    if isSome(inDiffData.dependenentVars.clone()) {
        println!("{}", (literal!("dependenentVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.dependenentVars.clone())?)?;
    }
    if isSome(inDiffData.knownVars.clone()) {
        println!("{}", (literal!("knownVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.knownVars.clone())?)?;
    }
    if isSome(inDiffData.allVars.clone()) {
        println!("{}", (literal!("allVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.allVars.clone())?)?;
    }
    if !(inDiffData.controlVars.clone().is_empty()) {
        println!("{}", (literal!("controlVars:\n")).clone());
        BackendDump::printVarList(inDiffData.controlVars.clone());
    }
    if !(inDiffData.diffCrefs.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("diffCrefs:\n")); __mm_s.push_str(&*ComponentReference::printComponentRefListStr(inDiffData.diffCrefs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn isParamOrConstant(mut cref: Arc<DAE::ComponentRef>, mut diffData: BackendDAE::DifferentiateInputData) -> Result<bool> {
    let mut b: bool = false;
    b = (match diffData.clone() {
        BackendDAE::DifferentiateInputData { knownVars: Some(mut knownVars), .. } => {
            let mut var_lst: Option<Arc<metamodelica::List<BackendDAE::Var>>> = None;
            let mut var: BackendDAE::Var;
            var_lst = BackendVariable::getVarTryHard(cref.clone(), knownVars.clone());
            if isSome(var_lst.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(Util::getOption(var_lst.clone())?) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                var = __pa0.clone();
                b = BackendVariable::isParamOrConstant(var.clone());
            } else {
                b = false;
            }
            b.clone()
        },
        _ => {
            false
        },
    });
    Ok(b)
}

