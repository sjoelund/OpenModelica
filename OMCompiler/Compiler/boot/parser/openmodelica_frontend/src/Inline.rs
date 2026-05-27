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

use crate::AvlSetPath;
use crate::Ceval;
use crate::ComponentReference;
use crate::DAEDump;
use crate::Expression;
use crate::ExpressionSimplify;
use crate::HashTable2;
use crate::HashTable3;
use crate::HashTableCG;
use crate::Types;
use crate::VarTransform;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type Functiontuple = /* ? */;

pub fn checkExpsTypeEquiv(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<bool> {
    let mut bEquiv: bool = false;
    bEquiv = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (_, _) => {
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut b: bool = false;
            if Config::acceptMetaModelicaGrammar()? {
                b = true;
            } else {
                ty1 = Expression::r#typeof(inExp1.clone())?;
                ty2 = Expression::r#typeof(inExp2.clone())?;
                (ty2, _) = Types::traverseType(ty2.clone(), -1, (std::sync::Arc::new(fnptr!(Types::makeExpDimensionsUnknown, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?;
                b = Types::equivtypesOrRecordSubtypeOf(ty1.clone(), ty2.clone())?;
            }
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bEquiv)
}

pub fn hasGenerateEventsAnnotation(mut comment: Option<Arc<SCode::Comment>>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(comment.clone()) {
        Some(Deref @ SCode::Comment { annotation_: Some(anno), .. }) => {
            SCodeUtil::hasBooleanNamedAnnotation(anno.clone(), (literal!("GenerateEvents")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn dumpArgmap(mut inTpl: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<()> {
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut exp: Arc<DAE::Exp>;
    (cr, exp) = inTpl.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn mergeFunctionBody(mut iStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut iRepl: VarTransform::VariableReplacements, mut assertStmtsIn: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<(VarTransform::VariableReplacements, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut oRepl: VarTransform::VariableReplacements;
    let mut assertStmtsOut: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    (oRepl, assertStmtsOut) = (::match_deref::match_deref! { match &((iStmts.clone(), iRepl.clone(), assertStmtsIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            (iRepl.clone(), assertStmtsIn.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp, exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: stmts }, _, _) => {
            let mut repl: VarTransform::VariableReplacements;
            let mut assertStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut exp = (*exp).clone();
            (exp, _) = VarTransform::replaceExp(exp.clone(), iRepl.clone(), None)?;
            repl = VarTransform::addReplacementNoTransitive(iRepl.clone(), cr.clone(), exp.clone())?;
            (repl, assertStmts) = mergeFunctionBody(stmts.clone(), repl.clone(), assertStmtsIn.clone())?;
            (repl.clone(), assertStmts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp, lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: stmts }, _, _) => {
            let mut repl: VarTransform::VariableReplacements;
            let mut assertStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut exp = (*exp).clone();
            (exp, _) = VarTransform::replaceExp(exp.clone(), iRepl.clone(), None)?;
            repl = VarTransform::addReplacementNoTransitive(iRepl.clone(), cr.clone(), exp.clone())?;
            (repl, assertStmts) = mergeFunctionBody(stmts.clone(), repl.clone(), assertStmtsIn.clone())?;
            (repl.clone(), assertStmts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp, expExpLst: explst, .. }, tail: stmts }, _, _) => {
            let mut repl: VarTransform::VariableReplacements;
            let mut assertStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut exp = (*exp).clone();
            (exp, _) = VarTransform::replaceExp(exp.clone(), iRepl.clone(), None)?;
            repl = addTplAssignToRepl(explst.clone(), 1, exp.clone(), iRepl.clone())?;
            (repl, assertStmts) = mergeFunctionBody(stmts.clone(), repl.clone(), assertStmtsIn.clone())?;
            (repl.clone(), assertStmts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { source, level: exp2, msg: exp1, cond: exp }, tail: stmts }, _, _) => {
            let mut repl: VarTransform::VariableReplacements;
            let mut stmt: Arc<DAE::Statement>;
            let mut assertStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut exp2 = (*exp2).clone();
            let mut exp1 = (*exp1).clone();
            let mut exp = (*exp).clone();
            (exp, _) = VarTransform::replaceExp(exp.clone(), iRepl.clone(), None)?;
            (exp1, _) = VarTransform::replaceExp(exp1.clone(), iRepl.clone(), None)?;
            (exp2, _) = VarTransform::replaceExp(exp2.clone(), iRepl.clone(), None)?;
            stmt = Arc::new(DAE::Statement::STMT_ASSERT { cond: exp.clone(), msg: exp1.clone(), level: exp2.clone(), source: source.clone() });
            (repl, assertStmts) = mergeFunctionBody(stmts.clone(), iRepl.clone(), cons(stmt.clone(), assertStmtsIn.clone()))?;
            (repl.clone(), assertStmts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { else_: Deref @ DAE::Else::ELSE { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp: exp2, exp1: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, .. }, tail: Deref @ metamodelica::List::Nil } }, statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp: exp1, exp1: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, exp, .. }, tail: stmts }, _, _) if (ComponentReferenceBasics::crefEqual(cr1.clone(), cr2.clone())?) => {
            let mut repl: VarTransform::VariableReplacements;
            let mut assertStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut exp2 = (*exp2).clone();
            let mut exp1 = (*exp1).clone();
            let mut exp = (*exp).clone();
            (exp, _) = VarTransform::replaceExp(exp.clone(), iRepl.clone(), None)?;
            (exp1, _) = VarTransform::replaceExp(exp1.clone(), iRepl.clone(), None)?;
            (exp2, _) = VarTransform::replaceExp(exp2.clone(), iRepl.clone(), None)?;
            repl = VarTransform::addReplacementNoTransitive(iRepl.clone(), cr1.clone(), Arc::new(DAE::Exp::IFEXP { expCond: exp.clone(), expThen: exp1.clone(), expElse: exp2.clone() }))?;
            (repl, assertStmts) = mergeFunctionBody(stmts.clone(), repl.clone(), assertStmtsIn.clone())?;
            (repl.clone(), assertStmts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { else_: Deref @ DAE::Else::ELSE { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: exp2, lhs: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, .. }, tail: Deref @ metamodelica::List::Nil } }, statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: exp1, lhs: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, exp, .. }, tail: stmts }, _, _) if (ComponentReferenceBasics::crefEqual(cr1.clone(), cr2.clone())?) => {
            let mut repl: VarTransform::VariableReplacements;
            let mut assertStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut exp2 = (*exp2).clone();
            let mut exp1 = (*exp1).clone();
            let mut exp = (*exp).clone();
            (exp, _) = VarTransform::replaceExp(exp.clone(), iRepl.clone(), None)?;
            (exp1, _) = VarTransform::replaceExp(exp1.clone(), iRepl.clone(), None)?;
            (exp2, _) = VarTransform::replaceExp(exp2.clone(), iRepl.clone(), None)?;
            repl = VarTransform::addReplacementNoTransitive(iRepl.clone(), cr1.clone(), Arc::new(DAE::Exp::IFEXP { expCond: exp.clone(), expThen: exp1.clone(), expElse: exp2.clone() }))?;
            (repl, assertStmts) = mergeFunctionBody(stmts.clone(), repl.clone(), assertStmtsIn.clone())?;
            (repl.clone(), assertStmts.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oRepl, assertStmtsOut))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addTplAssignToRepl(mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut indx: i32, mut iExp: Arc<DAE::Exp>, mut iRepl: VarTransform::VariableReplacements) -> Result<VarTransform::VariableReplacements> {
    let mut oRepl: VarTransform::VariableReplacements;
    oRepl = (::match_deref::match_deref! { match &((explst.clone(), indx.clone(), iExp.clone(), iRepl.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            iRepl.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: rest }, _, _, _) => {
            let mut repl: VarTransform::VariableReplacements;
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::TSUB { exp: iExp.clone(), ix: indx.clone(), ty: tp.clone() });
            repl = VarTransform::addReplacementNoTransitive(iRepl.clone(), cr.clone(), exp.clone())?;
            addTplAssignToRepl(rest.clone(), indx.clone() + 1, iExp.clone(), repl.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oRepl)
}

fn getFunctionInputsOutputBody(mut r#fn: Arc<metamodelica::List<Arc<DAE::Element>>>, mut iRepl: VarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::Statement>>>, VarTransform::VariableReplacements)> {
    let mut oInputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut oOutputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut oBody: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut oRepl: VarTransform::VariableReplacements = iRepl.clone();
    let mut elt: Arc<DAE::Element>;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut binding: Option<Arc<DAE::Exp>> = None;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut st: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    for mut elt in &*r#fn.clone() {
        let mut elt = elt.clone();
        let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, componentRef: cr, .. } => {
            oInputs = cons(cr.clone(), oInputs.clone());
            ()
        },
        Deref @ DAE::Element::VAR { binding, direction: DAE::VarDirection::OUTPUT, componentRef: cr, .. } => {
            let mut binding = (*binding).clone();
            binding = makeComplexBinding(binding.clone(), var_field!((*elt).ty, DAE::Element::VAR).clone());
            oRepl = addOptBindingReplacements(cr.clone(), binding.clone(), oRepl.clone())?;
            oOutputs = cons(cr.clone(), oOutputs.clone());
            ()
        },
        Deref @ DAE::Element::VAR { binding, protection: DAE::VarVisibility::PROTECTED, componentRef: cr, .. } => {
            tp = ComponentReference::crefTypeFull(cr.clone())?;
            let false = (Expression::isArrayType(tp.clone())) else { bail!("pattern mismatch") };
            let false = (Expression::isRecordType(tp.clone())) else { bail!("pattern mismatch") };
            oRepl = addOptBindingReplacements(cr.clone(), binding.clone(), oRepl.clone())?;
            ()
        },
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: st }, .. } => {
            oBody = List::append_reverse(st.clone(), oBody.clone());
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown element: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![elt.clone()])?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    oInputs = oInputs.clone().reverse();
    oOutputs = oOutputs.clone().reverse();
    oBody = oBody.clone().reverse();
    Ok((oInputs, oOutputs, oBody, oRepl))
}

fn makeComplexBinding(mut binding: Option<Arc<DAE::Exp>>, mut ty: Arc<DAE::Type>) -> Option<Arc<DAE::Exp>> {
    let mut binding: Option<Arc<DAE::Exp>> = binding;
    binding = (::match_deref::match_deref! { match &((binding.clone(), ty.clone())) {
        (None, Deref @ DAE::Type::T_COMPLEX { .. }) => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut exp: Arc<DAE::Exp>;
            expl = metamodelica::nil();
            strl = metamodelica::nil();
            for mut var in &*var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone().reverse() {
                let mut var = var.clone();
                let () = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp, .. }, .. } => {
            expl = cons(exp.clone(), expl.clone());
            strl = cons((var.name.clone()).clone(), strl.clone());
            ()
        },
        _ => {
            return binding;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            Some(Arc::new(DAE::Exp::RECORD { path: ClassInfUtil::getStateName(var_field!((*ty).complexClassType, DAE::Type::T_COMPLEX).clone()), exps: expl.clone(), comp: strl.clone(), ty: ty.clone() }))
        },
        _ => {
            binding.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

fn addOptBindingReplacements(mut cr: Arc<DAE::ComponentRef>, mut binding: Option<Arc<DAE::Exp>>, mut iRepl: VarTransform::VariableReplacements) -> Result<VarTransform::VariableReplacements> {
    let mut oRepl: VarTransform::VariableReplacements;
    oRepl = (::match_deref::match_deref! { match &((cr.clone(), binding.clone(), iRepl.clone())) {
        (_, Some(e), _) => {
            addReplacement(cr.clone(), e.clone(), iRepl.clone())?
        },
        (_, None, _) => {
            iRepl.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oRepl)
}

fn addReplacement(mut iCr: Arc<DAE::ComponentRef>, mut iExp: Arc<DAE::Exp>, mut iRepl: VarTransform::VariableReplacements) -> Result<VarTransform::VariableReplacements> {
    let mut oRepl: VarTransform::VariableReplacements;
    oRepl = (::match_deref::match_deref! { match &((iCr.clone(), iExp.clone(), iRepl.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _, _) => {
            VarTransform::addReplacement(iRepl.clone(), iCr.clone(), iExp.clone())?
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oRepl)
}

// TODO: mahge: This needs to be rewritten completely.
pub fn extendCrefRecords(mut inArgmap: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, mut inCheckCr: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr))) -> Result<(Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)))> {
    let mut outArgmap: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
    let mut outCheckCr: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    (outArgmap, outCheckCr) = 'mc: {
        let __mc_input = (inArgmap.clone(), inCheckCr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, ht) => {
                    Ok((metamodelica::nil(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_COMPLEX { .. }, exp: e }), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut new1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    (new1, ht1) = extendCrefRecords(cons((c.clone(), e.clone()), res.clone()), ht.clone())?;
                    Ok((new1.clone(), ht1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { varLst, .. }, componentRef: cref }), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut res2: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut new: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut new1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    (res1, ht1) = extendCrefRecords(res.clone(), ht.clone())?;
                    new = List::map2(varLst.clone(), (std::sync::Arc::new(extendCrefRecords1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> + 'static>), c.clone(), cref.clone());
                    (new1, ht2) = extendCrefRecords(new.clone(), ht1.clone())?;
                    res2 = listAppend(new1.clone(), res1.clone());
                    Ok((cons((c.clone(), e.clone()), res2.clone()), ht2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, e @ Deref @ DAE::Exp::CREF { componentRef: cref, .. }), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut res2: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut new: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut new1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastType(cref.clone())?) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varLst = __pa0.clone();
                    (res1, ht1) = extendCrefRecords(res.clone(), ht.clone())?;
                    new = List::map2(varLst.clone(), (std::sync::Arc::new(extendCrefRecords1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> + 'static>), c.clone(), cref.clone());
                    (new1, ht2) = extendCrefRecords(new.clone(), ht1.clone())?;
                    res2 = listAppend(new1.clone(), res1.clone());
                    Ok((cons((c.clone(), e.clone()), res2.clone()), ht2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: rpath }, .. }, .. }, expLst: expl, .. }), tail: res }, ht) => {
                    if !((AbsynUtil::pathEqual(e.path.clone(), rpath.clone()))) { bail!("guard") }
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut res2: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut new: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut new1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    (res1, ht1) = extendCrefRecords(res.clone(), ht.clone())?;
                    crlst = List::map1(varLst.clone(), (std::sync::Arc::new(extendCrefRecords2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), c.clone());
                    new = List::zip(crlst.clone(), expl.clone());
                    (new1, ht2) = extendCrefRecords(new.clone(), ht1.clone())?;
                    res2 = listAppend(new1.clone(), res1.clone());
                    Ok((cons((c.clone(), e.clone()), res2.clone()), ht2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, e @ Deref @ DAE::Exp::RECORD { ty: Deref @ DAE::Type::T_COMPLEX { varLst, .. }, exps: expl, .. }), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut res2: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut new: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut new1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    (res1, ht1) = extendCrefRecords(res.clone(), ht.clone())?;
                    crlst = List::map1(varLst.clone(), (std::sync::Arc::new(extendCrefRecords2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), c.clone());
                    new = List::zip(crlst.clone(), expl.clone());
                    (new1, ht2) = extendCrefRecords(new.clone(), ht1.clone())?;
                    res2 = listAppend(new1.clone(), res1.clone());
                    Ok((cons((c.clone(), e.clone()), res2.clone()), ht2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, e), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht3: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut creftpllst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(e.clone())?) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varLst = __pa0.clone();
                    crlst = List::map1(varLst.clone(), (std::sync::Arc::new(extendCrefRecords2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), c.clone());
                    creftpllst = List::map1(crlst.clone(), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)), c.clone());
                    ht1 = List::fold(creftpllst.clone(), (std::sync::Arc::new(BaseHashTable::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ht.clone());
                    ht2 = getCheckCref(crlst.clone(), ht1.clone())?;
                    (res1, ht3) = extendCrefRecords(res.clone(), ht2.clone())?;
                    Ok((cons((c.clone(), e.clone()), res1.clone()), ht3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, e), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    (res1, ht1) = extendCrefRecords(res.clone(), ht.clone())?;
                    Ok((cons((c.clone(), e.clone()), res1.clone()), ht1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outArgmap, outCheckCr))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getCheckCref(mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inCheckCr: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr))> {
    let mut outCheckCr: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    outCheckCr = 'mc: {
        let __mc_input = (inCrefs.clone(), inCheckCr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, ht) => {
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, ht) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht3: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut creftpllst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastType(cr.clone())?) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varLst = __pa0.clone();
                    crlst = List::map1(varLst.clone(), (std::sync::Arc::new(extendCrefRecords2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cr.clone());
                    ht1 = getCheckCref(crlst.clone(), ht.clone())?;
                    creftpllst = List::map1(crlst.clone(), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)), cr.clone());
                    ht2 = List::fold(creftpllst.clone(), (std::sync::Arc::new(BaseHashTable::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ht1.clone());
                    ht3 = getCheckCref(rest.clone(), ht2.clone())?;
                    Ok(ht3.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, ht) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    ht1 = getCheckCref(rest.clone(), ht.clone())?;
                    Ok(ht1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCheckCr)
}

fn extendCrefRecords1(mut ev: Arc<DAE::Var>, mut c: Arc<DAE::ComponentRef>, mut e: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outArg: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>);
    outArg = 'mc: {
        let __mc_input = (ev.clone(), c.clone(), e.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Var { ty: tp, name, .. }, _, _) => {
                    let mut c1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut exp: Arc<DAE::Exp>;
                    c1 = ComponentReference::crefPrependIdent(c.clone(), (name.clone()).clone(), metamodelica::nil(), tp.clone())?;
                    e1 = ComponentReference::crefPrependIdent(e.clone(), (name.clone()).clone(), metamodelica::nil(), tp.clone())?;
                    exp = Expression::makeCrefExp(e1.clone(), tp.clone())?;
                    Ok((c1.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Inline.extendCrefRecords1 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArg)
}

fn extendCrefRecords2(mut ev: Arc<DAE::Var>, mut c: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outArg: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outArg = 'mc: {
        let __mc_input = (ev.clone(), c.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Var { ty: tp, name, .. }, _) => {
                    let mut c1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    c1 = ComponentReference::crefPrependIdent(c.clone(), (name.clone()).clone(), metamodelica::nil(), tp.clone())?;
                    Ok(c1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Inline.extendCrefRecords2 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArg)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getRhsExp(mut inElementList: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inElementList.clone()) {
        Deref @ metamodelica::List::Nil => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("Inline.getRhsExp failed - cannot inline such a function\n")).clone())?;
            bail!("fail")
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp: res, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: _ } => {
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: res, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: _ } => {
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: res, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: _ } => {
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: cdr } => {
            let mut res: Arc<DAE::Exp>;
            res = getRhsExp(cdr.clone())?;
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn replaceArgs(mut inExp: Arc<DAE::Exp>, mut inTuple: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (argmap, _, true)) => {
                    let mut e: Arc<DAE::Exp>;
                    e = getExpFromArgMap(argmap.clone(), cref.clone())?;
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (argmap, checkcr, true)) => {
                    if !((BaseHashTable::hasKey(ComponentReferenceBasics::crefFirstCref(cref.clone())?, checkcr.clone()))) { bail!("guard") }
                    Ok((inExp.clone(), (argmap.clone(), checkcr.clone(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (argmap, _, true)) => {
                    let mut firstCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut cref = (*cref).clone();
                    firstCref = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
                    ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(firstCref.clone())?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = getExpFromArgMap(argmap.clone(), firstCref.clone())?;
                    while !(ComponentReference::crefIsIdent(cref.clone())) {
                        cref = ComponentReference::crefRest(cref.clone())?;
                        ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(cref.clone())?) {
                            Deref @ metamodelica::List::Nil => (),
                            _ => bail!("pattern mismatch"),
                        } };
                        e = Arc::new(DAE::Exp::RSUB { exp: e.clone(), ix: -1, fieldName: (ComponentReferenceBasics::crefFirstIdent(cref.clone())?).clone(), ty: ComponentReference::crefType(cref.clone())? });
                    }
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (argmap, checkcr, true)) => {
                    getExpFromArgMap(argmap.clone(), ComponentReference::crefStripSubs(ComponentReferenceBasics::crefFirstCref(cref.clone())?)?)?;
                    Ok((inExp.clone(), (argmap.clone(), checkcr.clone(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { exp: Deref @ DAE::Exp::CALL { path, expLst, attr: Deref @ DAE::CallAttributes { ty: _, tuple_, builtin: false, isImpure, isFunctionPointerCall: _, inlineType, tailCall: tc } }, ty }, (argmap, _, true)) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut isFunctionPointerCall: bool = false;
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut path = (*path).clone();
                    let mut expLst = (*expLst).clone();
                    cref = ComponentReference::pathToCref(path.clone())?;
                    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(getExpFromArgMap(argmap.clone(), cref.clone())?) {
                        __pa2 @ Deref @ DAE::Exp::CREF { ty: __pa0, componentRef: __pa1 } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty2 = __pa0.clone();
                    cref = __pa1.clone();
                    e = __pa2.clone();
                    path = ComponentReference::crefToPath(cref.clone())?;
                    expLst = List::map(expLst.clone(), (std::sync::Arc::new(fnptr!(Expression::unboxExp, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    b = Expression::isBuiltinFunctionReference(e.clone());
                    isFunctionPointerCall = Types::isFunctionReferenceVar(ty2.clone());
                    e = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: tuple_.clone(), builtin: b.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: isFunctionPointerCall.clone(), inlineType: inlineType.clone(), tailCall: tc.clone() }) });
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNBOX { exp: Deref @ DAE::Exp::CALL { path, expLst: _, attr: Deref @ DAE::CallAttributes { builtin: false, .. } }, ty: _ }, (argmap, checkcr, true)) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cref = ComponentReference::pathToCref(path.clone())?;
                    let true = (BaseHashTable::hasKey(cref.clone(), checkcr.clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), (argmap.clone(), checkcr.clone(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, expLst, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_METATYPE { .. }, tuple_, builtin: false, isImpure, isFunctionPointerCall: _, inlineType: _, tailCall: tc } }, (argmap, _, true)) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut isFunctionPointerCall: bool = false;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
                    let mut path = (*path).clone();
                    let mut expLst = (*expLst).clone();
                    cref = ComponentReference::pathToCref(path.clone())?;
                    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(getExpFromArgMap(argmap.clone(), cref.clone())?) {
                        __pa2 @ Deref @ DAE::Exp::CREF { ty: __pa0, componentRef: __pa1 } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty = __pa0.clone();
                    cref = __pa1.clone();
                    e = __pa2.clone();
                    path = ComponentReference::crefToPath(cref.clone())?;
                    expLst = List::map(expLst.clone(), (std::sync::Arc::new(fnptr!(Expression::unboxExp, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    b = Expression::isBuiltinFunctionReference(e.clone());
                    (ty2, inlineType) = functionReferenceType(ty.clone())?;
                    isFunctionPointerCall = Types::isFunctionReferenceVar(ty2.clone());
                    e = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: Arc::new(DAE::CallAttributes { ty: ty2.clone(), tuple_: tuple_.clone(), builtin: b.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: isFunctionPointerCall.clone(), inlineType: inlineType.clone(), tailCall: tc.clone() }) });
                    e = boxIfUnboxedFunRef(e.clone(), ty.clone());
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, expLst: _, attr: Deref @ DAE::CallAttributes { builtin: false, ty: Deref @ DAE::Type::T_METATYPE { .. }, .. } }, (argmap, checkcr, true)) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cref = ComponentReference::pathToCref(path.clone())?;
                    let true = (BaseHashTable::hasKey(cref.clone(), checkcr.clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), (argmap.clone(), checkcr.clone(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

fn boxIfUnboxedFunRef(mut iexp: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((iexp.clone(), ty.clone())) {
        (exp, Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: Deref @ DAE::Type::T_FUNCTION { funcResultType: t, .. }, .. }) => {
            let mut exp = (*exp).clone();
            exp = if (Types::isBoxedType(t.clone())) {exp.clone()} else {Arc::new(DAE::Exp::BOX { exp: exp.clone() })};
            exp.clone()
        },
        _ => {
            iexp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn functionReferenceType(mut ty1: Arc<DAE::Type>) -> Result<(Arc<DAE::Type>, DAE::InlineType)> {
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    (ty2, inlineType) = (::match_deref::match_deref! { match &(ty1.clone()) {
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, functionAttributes: DAE::FunctionAttributes { inline: inlineType, .. }, .. }, .. } => {
            (Types::simplifyType(ty.clone())?, inlineType.clone())
        },
        _ => {
            (ty1.clone(), openmodelica_frontend_types::DAE::InlineType::NO_INLINE)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ty2, inlineType))
}

fn getExpFromArgMap(mut inArgMap: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut arg: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>);
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut key: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut exp: Arc<DAE::Exp>;
    subs = ComponentReferenceBasics::crefSubs(inComponentRef.clone())?;
    key = ComponentReference::crefStripSubs(inComponentRef.clone())?;
    for mut arg in &*inArgMap.clone() {
        let mut arg = arg.clone();
        (cref, exp) = arg.clone();
        if ComponentReferenceBasics::crefEqual(cref.clone(), key.clone())? {
            if let Ok(__iflet0) = Expression::applyExpSubscripts(exp.clone(), subs.clone()) {
                outExp = __iflet0;
            } else {
                continue;
            }
            return Ok(outExp);
        }
    }
    if Flags::isSet(Flags::FAILTRACE.clone())? {
        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Inline.getExpFromArgMap failed with empty argmap and cref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef.clone())?); ArcStr::from(__mm_s) }).clone())?;
    }
    bail!("fail");
    Ok(outExp)
}

fn getInputCrefs(mut inElement: Arc<DAE::Element>) -> Arc<DAE::ComponentRef> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outComponentRef = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, componentRef: cref, .. } => {
            cref.clone()
        },
        _ => {
            Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComponentRef
}

fn removeWilds(mut inComponentRef: Arc<DAE::ComponentRef>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::WILD => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn printInlineTypeStr(mut it: DAE::InlineType) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match it.clone() {
        DAE::InlineType::NO_INLINE => literal!("No inline"),
        DAE::InlineType::AFTER_INDEX_RED_INLINE => literal!("Inline after index reduction"),
        DAE::InlineType::EARLY_INLINE => literal!("Inline as soon as possible"),
        DAE::InlineType::BUILTIN_EARLY_INLINE => literal!("Inline as soon as possible, even if inlining is globally disabled"),
        DAE::InlineType::NORM_INLINE => literal!("Inline before index reduction"),
        DAE::InlineType::DEFAULT_INLINE => literal!("Inline if necessary"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn inlineEquationExp(mut inExp: Arc<DAE::EquationExp>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Statement>>>)> + 'static>, mut inSource: Arc<DAE::ElementSource>) -> Result<(Arc<DAE::EquationExp>, Arc<DAE::ElementSource>)> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Statement>>>)> + 'static>;

    let mut outExp: Arc<DAE::EquationExp>;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    (outExp, source) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: e } => {
            let mut changed: bool = false;
            let mut e_1: Arc<DAE::Exp>;
            let mut eq2: Arc<DAE::EquationExp>;
            (e_1, _) = Expression::traverseExpBottomUp(e.clone(), r#fn.clone(), metamodelica::nil())?;
            changed = !(referenceEq(&e.clone(),&e_1.clone()));
            eq2 = Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e_1.clone() });
            source = ElementSource::condAddSymbolicTransformation(changed.clone(), inSource.clone(), Arc::new(DAE::SymbolicOperation::OP_INLINE { before: inExp.clone(), after: eq2.clone() }))?;
            (eq2, source) = ExpressionSimplify::condSimplifyAddSymbolicOperation(changed.clone(), eq2.clone(), source.clone())?;
            (eq2.clone(), source.clone())
        },
        Deref @ DAE::EquationExp::RESIDUAL_EXP { exp: e } => {
            let mut changed: bool = false;
            let mut e_1: Arc<DAE::Exp>;
            let mut eq2: Arc<DAE::EquationExp>;
            (e_1, _) = Expression::traverseExpBottomUp(e.clone(), r#fn.clone(), metamodelica::nil())?;
            changed = !(referenceEq(&e.clone(),&e_1.clone()));
            eq2 = Arc::new(DAE::EquationExp::RESIDUAL_EXP { exp: e_1.clone() });
            source = ElementSource::condAddSymbolicTransformation(changed.clone(), inSource.clone(), Arc::new(DAE::SymbolicOperation::OP_INLINE { before: inExp.clone(), after: eq2.clone() }))?;
            (eq2, source) = ExpressionSimplify::condSimplifyAddSymbolicOperation(changed.clone(), eq2.clone(), source.clone())?;
            (eq2.clone(), source.clone())
        },
        Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: e1, rhs: e2 } => {
            let mut changed: bool = false;
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eq2: Arc<DAE::EquationExp>;
            (e1_1, _) = Expression::traverseExpBottomUp(e1.clone(), r#fn.clone(), metamodelica::nil())?;
            (e2_1, _) = Expression::traverseExpBottomUp(e2.clone(), r#fn.clone(), metamodelica::nil())?;
            changed = !(referenceEq(&e1.clone(),&e1_1.clone()) && referenceEq(&e2.clone(),&e2_1.clone()));
            eq2 = Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() });
            source = ElementSource::condAddSymbolicTransformation(changed.clone(), inSource.clone(), Arc::new(DAE::SymbolicOperation::OP_INLINE { before: inExp.clone(), after: eq2.clone() }))?;
            (eq2, source) = ExpressionSimplify::condSimplifyAddSymbolicOperation(changed.clone(), eq2.clone(), source.clone())?;
            (eq2.clone(), source.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Inline.inlineEquationExp failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, source))
}

fn getReplacementCheckComplex(mut repl: VarTransform::VariableReplacements, mut cr: Arc<DAE::ComponentRef>, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = 'mc: {
        let __mc_input = (repl.clone(), cr.clone(), ty.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(VarTransform::getReplacement(repl.clone(), cr.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_COMPLEX { varLst: vars, complexClassType: ClassInf::State::RECORD { path }, .. }) => {
                    let mut crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    crs = List::map1(List::map(vars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>)), (std::sync::Arc::new(ComponentReference::appendStringCref) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cr.clone());
                    exps = List::map1r(crs.clone(), (std::sync::Arc::new(VarTransform::getReplacement) as std::sync::Arc<dyn ::std::ops::Fn(VarTransform::VariableReplacements, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), repl.clone());
                    Ok(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: exps.clone(), attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(exp)
}

fn getInlineHashTableVarTransform() -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), VarTransform::VariableReplacements)> {
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut repl: VarTransform::VariableReplacements;
    let mut opt: Option<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), VarTransform::VariableReplacements)> = None;
    let mut regRepl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut invRepl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    opt = crate::Globals::inlineHashTable.with(|__root| __root.borrow().clone());
    (ht, repl) = (match opt.clone() {
        Some((mut ht, ref repl @ VarTransform::VariableReplacements { hashTable: ref regRepl, invHashTable: ref invRepl })) => {
            BaseHashTable::clearAssumeNoDelete(ht.clone())?;
            BaseHashTable::clearAssumeNoDelete(regRepl.clone())?;
            BaseHashTable::clearAssumeNoDelete(invRepl.clone())?;
            (ht.clone(), repl.clone())
        },
        _ => {
            ht = HashTableCG::emptyHashTable();
            repl = VarTransform::emptyReplacements();
            crate::Globals::inlineHashTable.with(|__root| *__root.borrow_mut() = Some((ht.clone(), repl.clone())));
            (ht.clone(), repl.clone())
        },
    });
    Ok((ht, repl))
}

