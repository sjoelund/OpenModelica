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
use crate::DAEUtil;
use crate::Expression;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn algorithmEmpty(mut alg: Arc<DAE::Algorithm>) -> bool {
    let mut empty: bool = false;
    empty = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    empty
}

pub fn isReinitStatement(mut stmt: Arc<DAE::Statement>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_REINIT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isNotAssertStatement(mut stmt: Arc<DAE::Statement>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSERT { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn makeAssignmentNoTypeCheck(mut ty: Arc<DAE::Type>, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>) -> Arc<DAE::Statement> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. } => Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() }),
        Deref @ DAE::Exp::PATTERN { pattern: Deref @ DAE::Pattern::PAT_WILD } => Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() }),
        _ => Arc::new(DAE::Statement::STMT_ASSIGN { type_: ty.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: source.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStatement
}

pub fn makeArrayAssignmentNoTypeCheck(mut ty: Arc<DAE::Type>, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>) -> Arc<DAE::Statement> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _) => Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() }),
        _ => Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: ty.clone(), lhs: lhs.clone(), exp: rhs.clone(), source: source.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStatement
}

pub fn makeTupleAssignmentNoTypeCheck(mut ty: Arc<DAE::Type>, mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rhs: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    let mut b1: bool = false;
    let mut b2: bool = false;
    b1 = List::all(lhs.clone(), Arc::new(fnptr!(Expression::isWild, Arc<DAE::Exp>)));
    b2 = List::all(List::restOrEmpty(lhs.clone())?, Arc::new(fnptr!(Expression::isWild, Arc<DAE::Exp>)));
    outStatement = makeTupleAssignmentNoTypeCheck2(b1.clone(), b2.clone(), ty.clone(), lhs.clone(), rhs.clone(), source.clone());
    Ok(outStatement)
}

fn makeTupleAssignmentNoTypeCheck2(mut allWild: bool, mut singleAssign: bool, mut ty: Arc<DAE::Type>, mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rhs: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>) -> Arc<DAE::Statement> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &((allWild.clone(), singleAssign.clone(), ty.clone(), lhs.clone())) {
        (true, _, _, _) => {
            Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() })
        },
        (_, true, Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: ty1 @ Deref @ DAE::Type::T_ARRAY { .. }, tail: _ }, .. }, Deref @ metamodelica::List::Cons { head: lhs1, tail: _ }) => {
            Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: ty1.clone(), lhs: lhs1.clone(), exp: Arc::new(DAE::Exp::TSUB { exp: rhs.clone(), ix: 1, ty: ty1.clone() }), source: source.clone() })
        },
        (_, true, Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: ty1, tail: _ }, .. }, Deref @ metamodelica::List::Cons { head: lhs1, tail: _ }) => {
            Arc::new(DAE::Statement::STMT_ASSIGN { type_: ty1.clone(), exp1: lhs1.clone(), exp: Arc::new(DAE::Exp::TSUB { exp: rhs.clone(), ix: 1, ty: ty1.clone() }), source: source.clone() })
        },
        _ => {
            Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: ty.clone(), expExpLst: lhs.clone(), exp: rhs.clone(), source: source.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStatement
}

pub fn makeAssignment(mut inExp1: Arc<DAE::Exp>, mut inProperties2: DAE::Properties, mut inExp3: Arc<DAE::Exp>, mut inProperties4: DAE::Properties, mut inAttributes: Arc<DAE::Attributes>, mut initial_: SCode::Initial, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = 'mc: {
        let __mc_input = (inExp1.clone(), inProperties2.clone(), inExp3.clone(), inProperties4.clone(), inAttributes.clone(), initial_.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _, rhs, _, _, _) => {
                    Ok(Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, lhprop, rhs, rhprop, _, SCode::Initial::NON_INITIAL) => {
                    let mut outStatement: Arc<DAE::Statement>;
                    let DAE::C_PARAM { .. } = (Types::propAnyConst(lhprop.clone())?) else { bail!("pattern mismatch") };
                    let true = (ComponentReference::isRecord(cr.clone())) else { bail!("pattern mismatch") };
                    outStatement = makeAssignment2(lhs.clone(), lhprop.clone(), rhs.clone(), rhprop.clone(), source.clone())?;
                    Ok(outStatement.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, _, _, SCode::Initial::NON_INITIAL) => {
                    let mut lhs_str: ArcStr = arcstr::literal!("");
                    let mut rhs_str: ArcStr = arcstr::literal!("");
                    let DAE::C_PARAM { .. } = (Types::propAnyConst(lprop.clone())?) else { bail!("pattern mismatch") };
                    lhs_str = (ExpressionBasics::printExpStr(lhs.clone())?).clone();
                    rhs_str = (ExpressionBasics::printExpStr(rhs.clone())?).clone();
                    Error::addSourceMessage(Error::ASSIGN_PARAM_ERROR.clone(), list![(lhs_str.clone()).clone(), (rhs_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, _, _, _, Deref @ DAE::Attributes { variability: SCode::Variability::CONST, .. }, _) => {
                    let mut lhs_str: ArcStr = arcstr::literal!("");
                    lhs_str = (ExpressionBasics::printExpStr(lhs.clone())?).clone();
                    Error::addSourceMessage(Error::ASSIGN_READONLY_ERROR.clone(), list![(literal!("constant")).clone(), (lhs_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lhprop, rhs, rhprop, _, SCode::Initial::INITIAL) => {
                    let mut outStatement: Arc<DAE::Statement>;
                    let DAE::C_PARAM { .. } = (Types::propAnyConst(lhprop.clone())?) else { bail!("pattern mismatch") };
                    outStatement = makeAssignment2(lhs.clone(), lhprop.clone(), rhs.clone(), rhprop.clone(), source.clone())?;
                    Ok(outStatement.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lhprop, rhs, rhprop, _, _) => {
                    let mut outStatement: Arc<DAE::Statement>;
                    let DAE::C_VAR { .. } = (Types::propAnyConst(lhprop.clone())?) else { bail!("pattern mismatch") };
                    outStatement = makeAssignment2(lhs.clone(), lhprop.clone(), rhs.clone(), rhprop.clone(), source.clone())?;
                    Ok(outStatement.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, rprop, _, _) => {
                    let mut lhs_str: ArcStr = arcstr::literal!("");
                    let mut rhs_str: ArcStr = arcstr::literal!("");
                    let mut lt_str: ArcStr = arcstr::literal!("");
                    let mut rt_str: ArcStr = arcstr::literal!("");
                    let mut lt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut rt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut info: SourceInfo;
                    lt = Types::getPropType(lprop.clone())?;
                    rt = Types::getPropType(rprop.clone())?;
                    let false = (Types::equivtypes(lt.clone(), rt.clone())?) else { bail!("pattern mismatch") };
                    lhs_str = (ExpressionBasics::printExpStr(lhs.clone())?).clone();
                    rhs_str = (ExpressionBasics::printExpStr(rhs.clone())?).clone();
                    lt_str = (TypesDump::unparseTypeNoAttr(lt.clone())?).clone();
                    rt_str = (TypesDump::unparseTypeNoAttr(rt.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Types::typeErrorSanityCheck((lt_str.clone()).clone(), (rt_str.clone()).clone(), info.clone())?;
                    Error::addSourceMessage(Error::ASSIGN_TYPE_MISMATCH_ERROR.clone(), list![(lhs_str.clone()).clone(), (rhs_str.clone()).clone(), (lt_str.clone()).clone(), (rt_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, _, rhs, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- Algorithm.makeAssignment failed")).clone())?;
                    Debug::trace((literal!("    ")).clone())?;
                    Debug::trace((ExpressionBasics::printExpStr(lhs.clone())?).clone())?;
                    Debug::trace((literal!(" := ")).clone())?;
                    Debug::traceln((ExpressionBasics::printExpStr(rhs.clone())?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatement)
}

fn makeAssignment2(mut lhs: Arc<DAE::Exp>, mut lhprop: DAE::Properties, mut rhs: Arc<DAE::Exp>, mut rhprop: DAE::Properties, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ DAE::Exp::CREF { .. } if (!(Types::isPropArray(lhprop.clone())?)) => {
            let mut rhs_1: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            (rhs_1, _) = Types::matchProp(rhs.clone(), rhprop.clone(), lhprop.clone(), true)?;
            t = getPropExpType(lhprop.clone())?;
            let _ = (::match_deref::match_deref! { match &(rhs_1.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::CREF { .. }, tail: _ }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listAppend" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. } } if (ExpressionBasics::expEqual(lhs.clone(), e1.clone())?) => {
            if Flags::isSet(Flags::LIST_REVERSE_WRONG_ORDER.clone())? && !({
        let mut __acc: Option<bool> = None;
        for mut comment in (ElementSource::getComments(source.clone())?).into_iter().cloned() {
            let __x = SCodeUtil::commentHasBooleanNamedAnnotation(comment.clone(), (literal!("__OpenModelica_DisableListAppendWarning")).clone())?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    }) {
                Error::addSourceMessage(Error::LIST_REVERSE_WRONG_ORDER.clone(), list![(ExpressionBasics::printExpStr(e1.clone())?).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                bail!("fail");
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Arc::new(DAE::Statement::STMT_ASSIGN { type_: t.clone(), exp1: lhs.clone(), exp: rhs_1.clone(), source: source.clone() })
        },
        Deref @ DAE::Exp::CREF { .. } => {
            let mut rhs_1: Arc<DAE::Exp>;
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            (rhs_1, _) = Types::matchProp(rhs.clone(), rhprop.clone(), lhprop.clone(), false)?;
            ty = Types::getPropType(lhprop.clone())?;
            t = Types::simplifyType(ty.clone())?;
            Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: t.clone(), lhs: lhs.clone(), exp: rhs_1.clone(), source: source.clone() })
        },
        e3 @ Deref @ DAE::Exp::ASUB { exp: _, sub: _ } => {
            let mut rhs_1: Arc<DAE::Exp>;
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            (rhs_1, _) = Types::matchProp(rhs.clone(), rhprop.clone(), lhprop.clone(), true)?;
            t = getPropExpType(lhprop.clone())?;
            Arc::new(DAE::Statement::STMT_ASSIGN { type_: t.clone(), exp1: e3.clone(), exp: rhs_1.clone(), source: source.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub fn makeSimpleAssignment(mut inTpl: (Arc<DAE::Exp>, Arc<DAE::Exp>), mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStmt: Arc<DAE::Statement>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(inTpl.clone()) {
        (__pa1 @ Deref @ DAE::Exp::CREF { ty: __pa0, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    tp = __pa0.clone();
    e1 = __pa1.clone();
    e2 = __pa2.clone();
    outStmt = Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e1.clone(), exp: e2.clone(), source: source.clone() });
    Ok(outStmt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn makeAssignmentsList(mut lhsExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut lhsProps: Arc<metamodelica::List<DAE::Properties>>, mut rhsExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rhsProps: Arc<metamodelica::List<DAE::Properties>>, mut attributes: Arc<DAE::Attributes>, mut initial_: SCode::Initial, mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut assignments: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    assignments = (::match_deref::match_deref! { match &((lhsExps.clone(), lhsProps.clone(), rhsExps.clone(), rhsProps.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, tail: rest_lhs }, Deref @ metamodelica::List::Cons { head: _, tail: rest_lhs_prop }, Deref @ metamodelica::List::Cons { head: _, tail: rest_rhs }, Deref @ metamodelica::List::Cons { head: _, tail: rest_rhs_prop }) => {
            makeAssignmentsList(rest_lhs.clone(), rest_lhs_prop.clone(), rest_rhs.clone(), rest_rhs_prop.clone(), attributes.clone(), initial_.clone(), source.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: lhs, tail: rest_lhs }, Deref @ metamodelica::List::Cons { head: lhs_prop, tail: rest_lhs_prop }, Deref @ metamodelica::List::Cons { head: rhs, tail: rest_rhs }, Deref @ metamodelica::List::Cons { head: rhs_prop, tail: rest_rhs_prop }) => {
            let mut ass: Arc<DAE::Statement>;
            let mut rest_ass: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            ass = makeAssignment(lhs.clone(), lhs_prop.clone(), rhs.clone(), rhs_prop.clone(), attributes.clone(), initial_.clone(), source.clone())?;
            rest_ass = makeAssignmentsList(rest_lhs.clone(), rest_lhs_prop.clone(), rest_rhs.clone(), rest_rhs_prop.clone(), attributes.clone(), initial_.clone(), source.clone())?;
            cons(ass.clone(), rest_ass.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(assignments)
}

pub fn checkLHSWritable(mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut props: Arc<metamodelica::List<DAE::Properties>>, mut rhs: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>) -> Result<()> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut i: i32 = 1;
    let mut c: ArcStr = arcstr::literal!("");
    let mut l: ArcStr = arcstr::literal!("");
    let mut r: ArcStr = arcstr::literal!("");
    for mut p in &*props.clone() {
        let mut p = p.clone();
        let _ = (match p.clone() {
        DAE::Properties::PROP { constFlag: DAE::Const::C_VAR, .. } => (),
        DAE::Properties::PROP { type_: _, constFlag: DAE::Const::C_CONST } => {
            l = stringAppendList(list![(literal!("(")).clone(), stringDelimitList(List::map(lhs.clone(), Arc::new(ExpressionBasics::printExpStr)), (literal!(", ")).clone()), (literal!(")")).clone()]);
            r = (ExpressionBasics::printExpStr(rhs.clone())?).clone();
            Error::addSourceMessage(Error::ASSIGN_CONSTANT_ERROR.clone(), list![(l.clone()).clone(), (r.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            bail!("fail");
            ()
        },
        DAE::Properties::PROP { type_: ref ty, constFlag: DAE::Const::C_PARAM } => {
            if Types::getFixedVarAttributeParameterOrConstant(ty.clone()) {
                l = stringAppendList(list![(literal!("(")).clone(), stringDelimitList(List::map(lhs.clone(), Arc::new(ExpressionBasics::printExpStr)), (literal!(", ")).clone()), (literal!(")")).clone()]);
                r = (ExpressionBasics::printExpStr(rhs.clone())?).clone();
                c = (ExpressionBasics::printExpStr((lhs.clone()).get(i.clone())?)?).clone();
                Error::addSourceMessage(Error::ASSIGN_PARAM_FIXED_ERROR.clone(), list![(c.clone()).clone(), (l.clone()).clone(), (r.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                bail!("fail");
            }
            ()
        },
        DAE::Properties::PROP_TUPLE { type_: _, tupleConst: _ } => (),
        _ => bail!("match: no arm matched"),
    });
        i = i.clone() + 1;
    }
    Ok(())
}

pub fn makeTupleAssignment(mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTypesPropertiesLst: Arc<metamodelica::List<DAE::Properties>>, mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut initial_: SCode::Initial, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = 'mc: {
        let __mc_input = (inExpExpLst.clone(), inTypesPropertiesLst.clone(), inExp.clone(), inProperties.clone(), initial_.clone(), source.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, _, _, _) => {
                    let mut bvals: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
                    let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut lhs_str: ArcStr = arcstr::literal!("");
                    let mut rhs_str: ArcStr = arcstr::literal!("");
                    bvals = List::map(lprop.clone(), Arc::new(Types::propAnyConst));
                    let DAE::C_CONST { .. } = (List::reduce(bvals.clone(), Arc::new(fnptr!(Types::constOr, DAE::Const, DAE::Const)))?) else { bail!("pattern mismatch") };
                    sl = List::map(lhs.clone(), Arc::new(ExpressionBasics::printExpStr));
                    s = stringDelimitList(sl.clone(), (literal!(", ")).clone());
                    lhs_str = stringAppendList(list![(literal!("(")).clone(), (s.clone()).clone(), (literal!(")")).clone()]);
                    rhs_str = (ExpressionBasics::printExpStr(rhs.clone())?).clone();
                    Error::addSourceMessage(Error::ASSIGN_CONSTANT_ERROR.clone(), list![(lhs_str.clone()).clone(), (rhs_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, _, SCode::Initial::NON_INITIAL, _) => {
                    let mut bvals: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
                    let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut lhs_str: ArcStr = arcstr::literal!("");
                    let mut rhs_str: ArcStr = arcstr::literal!("");
                    bvals = List::map(lprop.clone(), Arc::new(Types::propAnyConst));
                    let DAE::C_PARAM { .. } = (List::reduce(bvals.clone(), Arc::new(fnptr!(Types::constOr, DAE::Const, DAE::Const)))?) else { bail!("pattern mismatch") };
                    sl = List::map(lhs.clone(), Arc::new(ExpressionBasics::printExpStr));
                    s = stringDelimitList(sl.clone(), (literal!(", ")).clone());
                    lhs_str = stringAppendList(list![(literal!("(")).clone(), (s.clone()).clone(), (literal!(")")).clone()]);
                    rhs_str = (ExpressionBasics::printExpStr(rhs.clone())?).clone();
                    Error::addSourceMessage(Error::ASSIGN_PARAM_ERROR.clone(), list![(lhs_str.clone()).clone(), (rhs_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expl, lhprops, rhs, DAE::Properties::PROP { type_: ty @ Deref @ DAE::Type::T_TUPLE { types: tpl, .. }, .. }, _, _) => {
                    let mut lhrtypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    checkLHSWritable(expl.clone(), lhprops.clone(), rhs.clone(), source.clone())?;
                    lhrtypes = List::map(lhprops.clone(), Arc::new(Types::getPropType));
                    Types::matchTypeTupleCall(rhs.clone(), tpl.clone(), lhrtypes.clone())?;
                    Ok(makeTupleAssignmentNoTypeCheck(ty.clone(), expl.clone(), rhs.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expl, lhprops, rhs, DAE::Properties::PROP_TUPLE { tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { .. }, type_: ty @ Deref @ DAE::Type::T_TUPLE { types: tpl, .. } }, _, _) => {
                    let mut lhrtypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    checkLHSWritable(expl.clone(), lhprops.clone(), rhs.clone(), source.clone())?;
                    lhrtypes = List::map(lhprops.clone(), Arc::new(Types::getPropType));
                    Types::matchTypeTupleCall(rhs.clone(), tpl.clone(), lhrtypes.clone())?;
                    Ok(makeTupleAssignmentNoTypeCheck(ty.clone(), expl.clone(), rhs.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, rprop, _, _) => {
                    let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut lhs_str: ArcStr = arcstr::literal!("");
                    let mut rhs_str: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut strInitial: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    sl = List::map(lhs.clone(), Arc::new(ExpressionBasics::printExpStr));
                    s = stringDelimitList(sl.clone(), (literal!(", ")).clone());
                    lhs_str = stringAppendList(list![(literal!("(")).clone(), (s.clone()).clone(), (literal!(")")).clone()]);
                    rhs_str = (ExpressionBasics::printExpStr(rhs.clone())?).clone();
                    str1 = stringDelimitList(List::map(lprop.clone(), Arc::new(Types::printPropStr)), (literal!(", ")).clone());
                    str2 = (Types::printPropStr(rprop.clone())?).clone();
                    strInitial = (SCodeDump::printInitialStr(initial_.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Algorithm.makeTupleAssignment failed on: \n\t")); __mm_s.push_str(&*lhs_str.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*rhs_str.clone()); __mm_s.push_str(&*literal!("\n\tprops lhs: (")); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*literal!(") =  props rhs: ")); __mm_s.push_str(&*str2.clone()); __mm_s.push_str(&*literal!("\n\tin ")); __mm_s.push_str(&*strInitial.clone()); __mm_s.push_str(&*literal!(" section")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatement)
}

fn getPropExpType(mut p: DAE::Properties) -> Result<Arc<DAE::Type>> {
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Types::getPropType(p.clone())?;
    t = Types::simplifyType(ty.clone())?;
    Ok(t)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn makeIf(mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut inTrueBranch: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inElseIfBranches: Arc<metamodelica::List<(Arc<DAE::Exp>, DAE::Properties, Arc<metamodelica::List<Arc<DAE::Statement>>>)>>, mut inElseBranch: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStatements = 'mc: {
        let __mc_input = (inExp.clone(), inProperties.clone(), inElseIfBranches.clone(), inElseBranch.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, _, _, _) => {
                    Ok(inTrueBranch.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ metamodelica::List::Nil, fb) => {
                    Ok(fb.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ metamodelica::List::Cons { head: (e, prop, tb), tail: eib }, fb) => {
                    Ok(makeIf(e.clone(), prop.clone(), tb.clone(), eib.clone(), fb.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP { type_: t, .. }, eib, fb) => {
                    let mut else_: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    let mut e = (*e).clone();
                    (e, _) = Types::matchType(e.clone(), t.clone(), DAE::T_BOOL_DEFAULT.clone(), true)?;
                    else_ = makeElse(eib.clone(), fb.clone(), source.clone())?;
                    Ok(list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: inTrueBranch.clone(), else_: else_.clone(), source: source.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP { type_: t, .. }, _, _) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut t_str: ArcStr = arcstr::literal!("");
                    e_str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    t_str = (TypesDump::unparseTypeNoAttr(t.clone())?).clone();
                    Error::addSourceMessage(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatements)
}

pub fn makeIfFromBranches(mut branches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Statement>>>)>>, mut source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<Arc<DAE::Statement>>> {
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStatements = (::match_deref::match_deref! { match &(branches.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: (e, br), tail: rest } => {
            let mut else_: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            else_ = makeElseFromBranches(rest.clone());
            list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: br.clone(), else_: else_.clone(), source: source.clone() })]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStatements
}

fn makeElseFromBranches(mut inTpl: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Statement>>>)>>) -> Arc<DAE::Else> {
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    outElse = (::match_deref::match_deref! { match &(inTpl.clone()) {
        Deref @ metamodelica::List::Nil => {
            Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE)
        },
        Deref @ metamodelica::List::Cons { head: (Deref @ DAE::Exp::BCONST { bool: true }, b), tail: Deref @ metamodelica::List::Nil } => {
            Arc::new(DAE::Else::ELSE { statementLst: b.clone() })
        },
        Deref @ metamodelica::List::Cons { head: (e, b), tail: xs } => {
            let mut else_: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            else_ = makeElseFromBranches(xs.clone());
            Arc::new(DAE::Else::ELSEIF { exp: e.clone(), statementLst: b.clone(), else_: else_.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElse
}

pub fn optimizeIf(mut icond: Arc<DAE::Exp>, mut istmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut iels: Arc<DAE::Else>, mut isource: Arc<DAE::ElementSource>) -> (Arc<metamodelica::List<Arc<DAE::Statement>>>, bool) {
    let mut ostmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut changed: bool = false;
    (ostmts, changed) = (::match_deref::match_deref! { match &((icond.clone(), istmts.clone(), iels.clone())) {
        (Deref @ DAE::Exp::BCONST { bool: true }, stmts, _) => {
            (stmts.clone(), true)
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::NOELSE) => {
            (metamodelica::nil(), true)
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::ELSE { statementLst: stmts }) => {
            (stmts.clone(), true)
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::ELSEIF { exp: cond, statementLst: stmts, else_: els }) => {
            (ostmts, _) = optimizeIf(cond.clone(), stmts.clone(), els.clone(), isource.clone());
            (ostmts.clone(), true)
        },
        _ => {
            (cons(Arc::new(DAE::Statement::STMT_IF { exp: icond.clone(), statementLst: istmts.clone(), else_: iels.clone(), source: isource.clone() }), metamodelica::nil()), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (ostmts, changed)
}

pub fn optimizeElseIf(mut cond: Arc<DAE::Exp>, mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut els: Arc<DAE::Else>) -> Arc<DAE::Else> {
    let mut oelse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    oelse = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ DAE::Exp::BCONST { bool: true } => Arc::new(DAE::Else::ELSE { statementLst: stmts.clone() }),
        Deref @ DAE::Exp::BCONST { bool: false } => els.clone(),
        _ => Arc::new(DAE::Else::ELSEIF { exp: cond.clone(), statementLst: stmts.clone(), else_: els.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oelse
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeElse(mut inTuple: Arc<metamodelica::List<(Arc<DAE::Exp>, DAE::Properties, Arc<metamodelica::List<Arc<DAE::Statement>>>)>>, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inSource: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Else>> {
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    outElse = 'mc: {
        let __mc_input = (inTuple.clone(), inStatementLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, fb) => {
                    Ok(Arc::new(DAE::Else::ELSE { statementLst: fb.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (Deref @ DAE::Exp::BCONST { bool: true }, DAE::Properties::PROP { .. }, b), tail: _ }, _) => {
                    Ok(Arc::new(DAE::Else::ELSE { statementLst: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (Deref @ DAE::Exp::BCONST { bool: false }, DAE::Properties::PROP { .. }, _), tail: xs }, fb) => {
                    Ok(makeElse(xs.clone(), fb.clone(), inSource.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (e, DAE::Properties::PROP { type_: t, .. }, b), tail: xs }, fb) => {
                    let mut else_: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    let mut e = (*e).clone();
                    (e, _) = Types::matchType(e.clone(), t.clone(), DAE::T_BOOL_DEFAULT.clone(), true)?;
                    else_ = makeElse(xs.clone(), fb.clone(), inSource.clone())?;
                    Ok(Arc::new(DAE::Else::ELSEIF { exp: e.clone(), statementLst: b.clone(), else_: else_.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (e, DAE::Properties::PROP { type_: t, .. }, _), tail: _ }, _) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut t_str: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo;
                    e_str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    t_str = (TypesDump::unparseTypeNoAttr(t.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(inSource.clone());
                    Error::addSourceMessage(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElse)
}

pub fn makeFor(mut inIdent: ArcStr, mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = 'mc: {
        let __mc_input = (inIdent.clone(), inExp.clone(), inProperties.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, e, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims, ty: t }, .. }) => {
                    let mut isArray: bool = false;
                    isArray = Types::isNonscalarArray(t.clone(), dims.clone())?;
                    Ok(Arc::new(DAE::Statement::STMT_FOR { type_: t.clone(), iterIsArray: isArray.clone(), iter: (i.clone()).clone(), range: e.clone(), statementLst: inStatementLst.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, e, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_METALIST { ty: t }, .. }) => {
                    let mut t = (*t).clone();
                    t = Types::simplifyType(t.clone())?;
                    Ok(Arc::new(DAE::Statement::STMT_FOR { type_: t.clone(), iterIsArray: false, iter: (i.clone()).clone(), range: e.clone(), statementLst: inStatementLst.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, e, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_METAARRAY { ty: t }, .. }) => {
                    let mut t = (*t).clone();
                    t = Types::simplifyType(t.clone())?;
                    Ok(Arc::new(DAE::Statement::STMT_FOR { type_: t.clone(), iterIsArray: false, iter: (i.clone()).clone(), range: e.clone(), statementLst: inStatementLst.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e, DAE::Properties::PROP { type_: t, .. }) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut t_str: ArcStr = arcstr::literal!("");
                    e_str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    t_str = (TypesDump::unparseTypeNoAttr(t.clone())?).clone();
                    Error::addSourceMessage(Error::FOR_EXPRESSION_TYPE_ERROR.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatement)
}

pub fn makeParFor(mut inIdent: ArcStr, mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inLoopPrlVars: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = 'mc: {
        let __mc_input = (inIdent.clone(), inProperties.clone(), inStatementLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims, ty: t }, .. }, stmts) => {
                    let mut isArray: bool = false;
                    isArray = Types::isNonscalarArray(t.clone(), dims.clone())?;
                    let _ = Types::simplifyType(t.clone())?;
                    Ok(Arc::new(DAE::Statement::STMT_PARFOR { type_: t.clone(), iterIsArray: isArray.clone(), iter: (i.clone()).clone(), range: inExp.clone(), statementLst: stmts.clone(), loopPrlVars: inLoopPrlVars.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Properties::PROP { type_: t, .. }, _) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut t_str: ArcStr = arcstr::literal!("");
                    e_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                    t_str = (TypesDump::unparseTypeNoAttr(t.clone())?).clone();
                    Error::addSourceMessage(Error::FOR_EXPRESSION_TYPE_ERROR.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatement)
}

pub fn makeWhile(mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = 'mc: {
        let __mc_input = (inProperties.clone(), inStatementLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. }, stmts) => {
                    Ok(Arc::new(DAE::Statement::STMT_WHILE { exp: inExp.clone(), statementLst: stmts.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t, .. }, _) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut t_str: ArcStr = arcstr::literal!("");
                    e_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                    t_str = (TypesDump::unparseTypeNoAttr(t.clone())?).clone();
                    Error::addSourceMessage(Error::WHILE_CONDITION_TYPE_ERROR.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatement)
}

pub fn makeWhenA(mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut elseWhenStmt: Option<Arc<DAE::Statement>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = 'mc: {
        let __mc_input = (inProperties.clone(), inStatementLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. }, stmts) => {
                    Ok(Arc::new(DAE::Statement::STMT_WHEN { exp: inExp.clone(), conditions: metamodelica::nil(), initialCall: false, statementLst: stmts.clone(), elseWhen: elseWhenStmt.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, .. }, stmts) => {
                    Ok(Arc::new(DAE::Statement::STMT_WHEN { exp: inExp.clone(), conditions: metamodelica::nil(), initialCall: false, statementLst: stmts.clone(), elseWhen: elseWhenStmt.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t, .. }, _) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut t_str: ArcStr = arcstr::literal!("");
                    e_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                    t_str = (TypesDump::unparseTypeNoAttr(t.clone())?).clone();
                    Error::addSourceMessage(Error::WHEN_CONDITION_TYPE_ERROR.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatement)
}

pub fn makeReinit(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inProperties3: DAE::Properties, mut inProperties4: DAE::Properties, mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outStatement: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStatement = 'mc: {
        let __mc_input = (inExp1.clone(), inExp2.clone(), inProperties3.clone(), inProperties4.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ DAE::Exp::CREF { .. }, val, DAE::Properties::PROP { type_: tp1, constFlag: _ }, DAE::Properties::PROP { type_: tp2, constFlag: _ }) => {
                    let mut var_1: Arc<DAE::Exp>;
                    let mut val_1: Arc<DAE::Exp>;
                    (val_1, _) = Types::matchType(val.clone(), tp2.clone(), DAE::T_REAL_DEFAULT.clone(), true)?;
                    (var_1, _) = Types::matchType(var.clone(), tp1.clone(), DAE::T_REAL_DEFAULT.clone(), true)?;
                    Ok(list![Arc::new(DAE::Statement::STMT_REINIT { var: var_1.clone(), value: val_1.clone(), source: source.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("reinit called with wrong args")).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatement)
}

pub fn makeAssert(mut cond: Arc<DAE::Exp>, mut msg: Arc<DAE::Exp>, mut level: Arc<DAE::Exp>, mut inProperties3: DAE::Properties, mut inProperties4: DAE::Properties, mut inProperties5: DAE::Properties, mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outStatement: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStatement = 'mc: {
        let __mc_input = (cond.clone(), inProperties3.clone(), inProperties4.clone(), inProperties5.clone(), source.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_STRING { .. }, .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ENUMERATION { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AssertionLevel" } }, .. }, .. }, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_STRING { .. }, .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ENUMERATION { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AssertionLevel" } }, .. }, .. }, _) => {
                    Ok(list![Arc::new(DAE::Statement::STMT_ASSERT { cond: cond.clone(), msg: msg.clone(), level: level.clone(), source: source.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Properties::PROP { type_: t1, .. }, DAE::Properties::PROP { type_: t2, .. }, DAE::Properties::PROP { type_: t3, .. }, _) => {
                    let mut info: SourceInfo;
                    let mut strTy: ArcStr = arcstr::literal!("");
                    let mut strExp: ArcStr = arcstr::literal!("");
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    strExp = (ExpressionBasics::printExpStr(cond.clone())?).clone();
                    strTy = (TypesDump::unparseType(t1.clone())?).clone();
                    Error::assertionOrAddSourceMessage(Types::isBooleanOrSubTypeBoolean(t1.clone())?, Error::EXP_TYPE_MISMATCH.clone(), list![(strExp.clone()).clone(), (literal!("Boolean")).clone(), (strTy.clone()).clone()], info.clone())?;
                    strExp = (ExpressionBasics::printExpStr(msg.clone())?).clone();
                    strTy = (TypesDump::unparseType(t2.clone())?).clone();
                    Error::assertionOrAddSourceMessage(Types::isString(t2.clone()), Error::EXP_TYPE_MISMATCH.clone(), list![(strExp.clone()).clone(), (literal!("String")).clone(), (strTy.clone()).clone()], info.clone())?;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(t3.clone()) {
                            Deref @ DAE::Type::T_ENUMERATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AssertionLevel" }, .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    strExp = (ExpressionBasics::printExpStr(level.clone())?).clone();
                    strTy = (TypesDump::unparseType(t3.clone())?).clone();
                    Error::assertionOrAddSourceMessage(Types::isString(t3.clone()), Error::EXP_TYPE_MISMATCH.clone(), list![(strExp.clone()).clone(), (literal!("AssertionLevel")).clone(), (strTy.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatement)
}

pub fn makeTerminate(mut msg: Arc<DAE::Exp>, mut props: DAE::Properties, mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outStatement: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStatement = (::match_deref::match_deref! { match &(props.clone()) {
        DAE::Properties::PROP { type_: Deref @ DAE::Type::T_STRING { .. }, .. } => list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })],
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub fn getCrefFromAlg(mut alg: Arc<DAE::Algorithm>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    crs = List::unionOnTrueList(List::map(getAllExps(alg.clone())?, Arc::new(Expression::extractCrefsFromExp)), Arc::new(ComponentReferenceBasics::crefEqual))?;
    Ok(crs)
}

pub fn getAllExps(mut inAlgorithm: Arc<DAE::Algorithm>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inAlgorithm.clone()) {
        Deref @ DAE::Algorithm { statementLst: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stmts = __pa0.clone();
    exps = getAllExpsStmts(stmts.clone());
    outExpExpLst = exps.clone();
    Ok(outExpExpLst)
}

pub fn getAllExpsStmts(mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (_, (_, __pa0)) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (fnptr!(Expression::expressionCollector, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>), metamodelica::nil()));
    exps = __pa0.clone();
    exps
}

pub fn getStatementSource(mut stmt: Arc<DAE::Statement>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource>;
    source = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_IF { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_FOR { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_PARFOR { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_WHILE { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_WHEN { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_ASSERT { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_TERMINATE { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_REINIT { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_NORETCALL { source, .. } => source.clone(),
        Deref @ DAE::Statement::STMT_RETURN { source } => source.clone(),
        Deref @ DAE::Statement::STMT_BREAK { source } => source.clone(),
        Deref @ DAE::Statement::STMT_CONTINUE { source } => source.clone(),
        Deref @ DAE::Statement::STMT_FAILURE { source, .. } => source.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Algorithm.getStatementSource")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub fn isNotDummyStatement(mut stmt: Arc<DAE::Statement>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_NORETCALL { exp, .. } => {
            (_, b) = Expression::traverseExpBottomUp(exp.clone(), Arc::new(fnptr!(Expression::hasNoSideEffects, Arc<DAE::Exp>, bool)), true)?;
            !(b.clone())
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

