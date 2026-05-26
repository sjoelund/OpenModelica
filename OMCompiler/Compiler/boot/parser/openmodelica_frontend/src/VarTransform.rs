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
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::HashTable2;
use crate::HashTable3;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// VariableReplacements consists of a mapping between variables and expressions, the first binary tree of this type.
/// To eliminate a variable from an equation system a replacement rule varname->expression is added to this
/// datatype.
/// To be able to update these replacement rules incrementally a backward lookup mechanism is also required.
/// For instance, having a rule a->b and adding a rule b->c requires to find the first rule a->b and update it to
/// a->c. This is what the second binary tree is used for.
#[derive(Clone, Debug, PartialEq)]
pub struct VariableReplacements {
    /// src -> dst, used for replacing. src is variable, dst is expression.
    pub hashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)),
    /// dst -> list of sources. dst is a variable, sources are variables.
    pub invHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr)),
}

impl Default for VariableReplacements {
    fn default() -> Self {
        Self {
            hashTable: Default::default(),
            invHashTable: Default::default(),
        }
    }
}

pub type REPLACEMENTS = VariableReplacements;


//protected import Debug;
pub fn applyReplacementsDAE(mut dae: DAE::DAElist, mut repl: VariableReplacements, mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<DAE::DAElist> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outDae: DAE::DAElist;
    outDae = (match (dae.clone(), repl.clone(), condExpFunc.clone()) {
        (DAE::DAElist { elementLst: ref elts }, _, _) => {
            let mut elts = elts.clone();
            elts = applyReplacementsDAEElts(elts.clone(), repl.clone(), condExpFunc.clone())?;
            DAE::DAElist { elementLst: elts.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outDae)
}

pub fn applyReplacementsDAEElts(mut inDae: Arc<metamodelica::List<Arc<DAE::Element>>>, mut repl: VariableReplacements, mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outDae: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    if BaseHashTable::hashTableCurrentSize(repl.hashTable.clone()) == 0 {
        outDae = inDae.clone();
        return Ok(outDae);
    }
    outDae = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut elt in (inDae.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::VAR { componentRef: cr, kind, direction: dir, parallelism: prl, protection: prot, ty: tp, binding: Some(bindExp), dims, connectorType: ct, source, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: ie } => {
            let mut bindExp2: Arc<DAE::Exp>;
            let mut attr = (*attr).clone();
            (bindExp2, _) = replaceExp(bindExp.clone(), repl.clone(), condExpFunc.clone())?;
            attr = applyReplacementsVarAttr(attr.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::VAR { componentRef: cr.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: Some(bindExp2.clone()), dims: dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: ie.clone() })
        },
        Deref @ DAE::Element::VAR { componentRef: cr, kind, direction: dir, parallelism: prl, protection: prot, ty: tp, binding: None, dims, connectorType: ct, source, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: ie } => {
            let mut attr = (*attr).clone();
            attr = applyReplacementsVarAttr(attr.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::VAR { componentRef: cr.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: None, dims: dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: ie.clone() })
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source } => {
            let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e2: Arc<DAE::Exp>;
            (e2, _) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr.clone())?, repl.clone(), condExpFunc.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr2 = __pa0.clone();
            Arc::new(DAE::Element::DEFINE { componentRef: cr2.clone(), exp: e2.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::INITIALDEFINE { componentRef: cr, exp: e, source } => {
            let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e2: Arc<DAE::Exp>;
            (e2, _) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr.clone())?, repl.clone(), condExpFunc.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr2 = __pa0.clone();
            Arc::new(DAE::Element::INITIALDEFINE { componentRef: cr2.clone(), exp: e2.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::EQUEQUATION { cr1: cr, cr2: cr1, source } => {
            let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cr1_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr.clone())?, repl.clone(), condExpFunc.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr2 = __pa0.clone();
            let __pa1 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr1.clone())?, repl.clone(), condExpFunc.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr1_2 = __pa1.clone();
            Arc::new(DAE::Element::EQUEQUATION { cr1: cr2.clone(), cr2: cr1_2.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source } => {
            let mut e22: Arc<DAE::Exp>;
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            (e22, _) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::EQUATION { exp: e11.clone(), scalar: e22.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::ARRAY_EQUATION { dimension: idims, exp: e1, array: e2, source } => {
            let mut e22: Arc<DAE::Exp>;
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            (e22, _) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::ARRAY_EQUATION { dimension: idims.clone(), exp: e11.clone(), array: e22.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { dimension: idims, exp: e1, array: e2, source } => {
            let mut e22: Arc<DAE::Exp>;
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            (e22, _) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: idims.clone(), exp: e11.clone(), array: e22.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::WHEN_EQUATION { condition: e1, equations: elist, elsewhen_: Some(elt2), source } => {
            let mut elist2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e11: Arc<DAE::Exp>;
            let mut elt2 = (*elt2).clone();
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(applyReplacementsDAEElts(list![elt2.clone()], repl.clone(), condExpFunc.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elt2 = __pa0.clone();
            elist2 = applyReplacementsDAEElts(elist.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::WHEN_EQUATION { condition: e11.clone(), equations: elist2.clone(), elsewhen_: Some(elt2.clone()), source: source.clone() })
        },
        Deref @ DAE::Element::WHEN_EQUATION { condition: e1, equations: elist, elsewhen_: None, source } => {
            let mut elist2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            elist2 = applyReplacementsDAEElts(elist.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::WHEN_EQUATION { condition: e11.clone(), equations: elist2.clone(), elsewhen_: None, source: source.clone() })
        },
        Deref @ DAE::Element::IF_EQUATION { condition1: conds, equations2: tbs, equations3: elist2, source } => {
            let mut elist22: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut tbs_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut conds_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (conds_1, _) = replaceExpList(conds.clone(), repl.clone(), condExpFunc.clone())?;
            tbs_1 = List::map2(tbs.clone(), Arc::new(applyReplacementsDAEElts), repl.clone(), condExpFunc.clone());
            elist22 = applyReplacementsDAEElts(elist2.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::IF_EQUATION { condition1: conds_1.clone(), equations2: tbs_1.clone(), equations3: elist22.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: conds, equations2: tbs, equations3: elist2, source } => {
            let mut elist22: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut tbs_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut conds_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (conds_1, _) = replaceExpList(conds.clone(), repl.clone(), condExpFunc.clone())?;
            tbs_1 = List::map2(tbs.clone(), Arc::new(applyReplacementsDAEElts), repl.clone(), condExpFunc.clone());
            elist22 = applyReplacementsDAEElts(elist2.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::INITIAL_IF_EQUATION { condition1: conds_1.clone(), equations2: tbs_1.clone(), equations3: elist22.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, source } => {
            let mut e22: Arc<DAE::Exp>;
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            (e22, _) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::INITIALEQUATION { exp1: e11.clone(), exp2: e22.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, source } => {
            let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (stmts2, _) = replaceEquationsStmts(stmts.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts2.clone() }), source: source.clone() })
        },
        Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, source } => {
            let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (stmts2, _) = replaceEquationsStmts(stmts.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::INITIALALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts2.clone() }), source: source.clone() })
        },
        Deref @ DAE::Element::COMP { ident: id, dAElist: elist, source, comment: cmt } => {
            let mut elist = (*elist).clone();
            elist = applyReplacementsDAEElts(elist.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::COMP { ident: (id.clone()).clone(), dAElist: elist.clone(), source: source.clone(), comment: cmt.clone() })
        },
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
            elt.clone()
        },
        Deref @ DAE::Element::ASSERT { condition: e1, message: e2, level: e3, source } => {
            let mut e22: Arc<DAE::Exp>;
            let mut e11: Arc<DAE::Exp>;
            let mut e32: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            (e22, _) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
            (e32, _) = replaceExp(e3.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::ASSERT { condition: e11.clone(), message: e22.clone(), level: e32.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::INITIAL_ASSERT { condition: e1, message: e2, level: e3, source } => {
            let mut e22: Arc<DAE::Exp>;
            let mut e11: Arc<DAE::Exp>;
            let mut e32: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            (e22, _) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
            (e32, _) = replaceExp(e3.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::INITIAL_ASSERT { condition: e11.clone(), message: e22.clone(), level: e32.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::TERMINATE { message: e1, source } => {
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::TERMINATE { message: e11.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, source } => {
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::INITIAL_TERMINATE { message: e11.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::REINIT { componentRef: cr, exp: e1, source } => {
            let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr.clone())?, repl.clone(), condExpFunc.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr2 = __pa0.clone();
            Arc::new(DAE::Element::REINIT { componentRef: cr2.clone(), exp: e11.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source } => {
            let mut e22: Arc<DAE::Exp>;
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            (e22, _) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::COMPLEX_EQUATION { lhs: e11.clone(), rhs: e22.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, source } => {
            let mut e22: Arc<DAE::Exp>;
            let mut e11: Arc<DAE::Exp>;
            (e11, _) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
            (e22, _) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
            Arc::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e11.clone(), rhs: e22.clone(), source: source.clone() })
        },
        _ => {
            Error::addInternalError((literal!("applyReplacementsDAEElts should not fail")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outDae)
}

fn applyReplacementsVarAttr(mut attr: Option<Arc<DAE::VariableAttributes>>, mut repl: VariableReplacements, mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &((attr.clone(), repl.clone(), condExpFunc.clone())) {
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity, unit, displayUnit, min, max, start: initial_, fixed, nominal, stateSelectOption: stateSelect, uncertainOption: unc, distributionOption: dist, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin }), _, _) => {
            let mut quantity = (*quantity).clone();
            let mut unit = (*unit).clone();
            let mut displayUnit = (*displayUnit).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut initial_ = (*initial_).clone();
            let mut fixed = (*fixed).clone();
            let mut nominal = (*nominal).clone();
            quantity = replaceExpOpt(quantity.clone(), repl.clone(), condExpFunc.clone())?;
            unit = replaceExpOpt(unit.clone(), repl.clone(), condExpFunc.clone())?;
            displayUnit = replaceExpOpt(displayUnit.clone(), repl.clone(), condExpFunc.clone())?;
            min = replaceExpOpt(min.clone(), repl.clone(), condExpFunc.clone())?;
            max = replaceExpOpt(max.clone(), repl.clone(), condExpFunc.clone())?;
            initial_ = replaceExpOpt(initial_.clone(), repl.clone(), condExpFunc.clone())?;
            fixed = replaceExpOpt(fixed.clone(), repl.clone(), condExpFunc.clone())?;
            nominal = replaceExpOpt(nominal.clone(), repl.clone(), condExpFunc.clone())?;
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: initial_.clone(), fixed: fixed.clone(), nominal: nominal.clone(), stateSelectOption: stateSelect.clone(), uncertainOption: unc.clone(), distributionOption: dist.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: startOrigin.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity, min, max, start: initial_, fixed, uncertainOption: unc, distributionOption: dist, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin }), _, _) => {
            let mut quantity = (*quantity).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut initial_ = (*initial_).clone();
            let mut fixed = (*fixed).clone();
            quantity = replaceExpOpt(quantity.clone(), repl.clone(), condExpFunc.clone())?;
            min = replaceExpOpt(min.clone(), repl.clone(), condExpFunc.clone())?;
            max = replaceExpOpt(max.clone(), repl.clone(), condExpFunc.clone())?;
            initial_ = replaceExpOpt(initial_.clone(), repl.clone(), condExpFunc.clone())?;
            fixed = replaceExpOpt(fixed.clone(), repl.clone(), condExpFunc.clone())?;
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: initial_.clone(), fixed: fixed.clone(), uncertainOption: unc.clone(), distributionOption: dist.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: startOrigin.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity, start: initial_, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin }), _, _) => {
            let mut quantity = (*quantity).clone();
            let mut initial_ = (*initial_).clone();
            let mut fixed = (*fixed).clone();
            quantity = replaceExpOpt(quantity.clone(), repl.clone(), condExpFunc.clone())?;
            initial_ = replaceExpOpt(initial_.clone(), repl.clone(), condExpFunc.clone())?;
            fixed = replaceExpOpt(fixed.clone(), repl.clone(), condExpFunc.clone())?;
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: initial_.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: startOrigin.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity, start: initial_, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin }), _, _) => {
            let mut quantity = (*quantity).clone();
            let mut initial_ = (*initial_).clone();
            let mut fixed = (*fixed).clone();
            quantity = replaceExpOpt(quantity.clone(), repl.clone(), condExpFunc.clone())?;
            initial_ = replaceExpOpt(initial_.clone(), repl.clone(), condExpFunc.clone())?;
            fixed = replaceExpOpt(fixed.clone(), repl.clone(), condExpFunc.clone())?;
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: initial_.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: startOrigin.clone() }))
        },
        (None, _, _) => {
            None
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn applyReplacements(mut inVariableReplacements1: VariableReplacements, mut inComponentRef2: Arc<DAE::ComponentRef>, mut inComponentRef3: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)> {
    let mut outComponentRef1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outComponentRef2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outComponentRef1, outComponentRef2) = (::match_deref::match_deref! { match &((inVariableReplacements1.clone(), inComponentRef2.clone(), inComponentRef3.clone())) {
        (repl, cr1, cr2) => {
            let mut cr1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cr2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr1.clone())?, repl.clone(), None)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr1_1 = __pa0.clone();
            let __pa1 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr2.clone())?, repl.clone(), None)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr2_1 = __pa1.clone();
            (cr1_1.clone(), cr2_1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outComponentRef1, outComponentRef2))
}

pub fn applyReplacementList(mut repl: VariableReplacements, mut increfs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut ocrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    ocrefs = (::match_deref::match_deref! { match &((repl.clone(), increfs.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (_, Deref @ metamodelica::List::Cons { head: cr1, tail: ocrefs }) => {
            let mut cr1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut ocrefs = (*ocrefs).clone();
            let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr1.clone())?, repl.clone(), None)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr1_1 = __pa0.clone();
            ocrefs = applyReplacementList(repl.clone(), ocrefs.clone())?;
            cons(cr1_1.clone(), ocrefs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ocrefs)
}

pub fn applyReplacementsExp(mut repl: VariableReplacements, mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outExp1: Arc<DAE::Exp>;
    let mut outExp2: Arc<DAE::Exp>;
    (outExp1, outExp2) = (::match_deref::match_deref! { match &((repl.clone(), inExp1.clone(), inExp2.clone())) {
        (_, e1, e2) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, _) = replaceExp(e1.clone(), repl.clone(), None)?;
            (e2, _) = replaceExp(e2.clone(), repl.clone(), None)?;
            (e1, _) = ExpressionSimplify::simplify1(e1.clone())?;
            (e2, _) = ExpressionSimplify::simplify1(e2.clone())?;
            (e1.clone(), e2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp1, outExp2))
}

pub fn emptyReplacementsArray(mut n: i32) -> Result<metamodelica::Array<VariableReplacements>> {
    let mut repl: metamodelica::Array<VariableReplacements>;
    repl = metamodelica::arrayFromVec(emptyReplacementsArray2(n.clone())?.into_iter().cloned().collect());
    Ok(repl)
}

fn emptyReplacementsArray2(mut n: i32) -> Result<Arc<metamodelica::List<VariableReplacements>>> {
    let mut replLst: Arc<metamodelica::List<VariableReplacements>> = metamodelica::nil();
    replLst = 'mc: {
        let __mc_input = n.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::nil())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (n.clone() < 0) else { bail!("pattern mismatch") };
            println!("{}", (literal!("Internal error, emptyReplacementsArray2 called with negative n!")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r: VariableReplacements;
            let mut replLst: Arc<metamodelica::List<VariableReplacements>> = replLst.clone();
            let true = (n.clone() > 0) else { bail!("pattern mismatch") };
            r = emptyReplacements();
            replLst = emptyReplacementsArray2(n.clone() - 1)?;
            Ok(cons(r.clone(), replLst.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(replLst)
}

pub fn emptyReplacements() -> VariableReplacements {
    let mut outVariableReplacements: VariableReplacements;
    outVariableReplacements = (match () {
        () => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut invHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
            ht = HashTable2::emptyHashTable();
            invHt = HashTable3::emptyHashTable();
            VariableReplacements { hashTable: ht.clone(), invHashTable: invHt.clone() }
        },
    });
    outVariableReplacements
}

pub fn emptyReplacementsSized(mut size: i32) -> VariableReplacements {
    let mut outVariableReplacements: VariableReplacements;
    outVariableReplacements = (match size.clone() {
        _ => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut invHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
            ht = HashTable2::emptyHashTableSized(size.clone());
            invHt = HashTable3::emptyHashTableSized(size.clone());
            VariableReplacements { hashTable: ht.clone(), invHashTable: invHt.clone() }
        },
    });
    outVariableReplacements
}

pub fn replaceEquationsStmts(mut inAlgorithmStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut repl: VariableReplacements, mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, bool)> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outAlgorithmStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    (outAlgorithmStatementLst, replacementPerformed) = 'mc: {
        let __mc_input = (inAlgorithmStatementLst.clone(), repl.clone(), condExpFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((metamodelica::nil(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { source, exp: e, exp1: e2, type_: tp }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (e_1, b1) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_2, b2) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e_2.clone(), exp: e_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, exp: e, expExpLst: expl1, type_: tp }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (e_1, b1) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    (expl2, b2) = replaceExpList(expl1.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: expl2.clone(), exp: e_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, exp: e2, lhs: e1, type_: tp }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (e_1, b1) = replaceExp(e1.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_2, b2) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_1.clone(), exp: e_2.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { source, else_: el, statementLst: stmts, exp: e }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut b3: bool = false;
                    let mut el_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    (el_1, b1) = replaceEquationsElse(el.clone(), repl.clone(), condExpFunc.clone())?;
                    (stmts2, b2) = replaceEquationsStmts(stmts.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_1, b3) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_IF { exp: e_1.clone(), statementLst: stmts2.clone(), else_: el_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_FOR { source, statementLst: stmts, range: e, iter: id1, iterIsArray, type_: tp }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (stmts2, b1) = replaceEquationsStmts(stmts.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_1, b2) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: iterIsArray.clone(), iter: (id1.clone()).clone(), range: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHILE { source, statementLst: stmts, exp: e }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (stmts2, b1) = replaceEquationsStmts(stmts.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_1, b2) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_WHILE { exp: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: ew, statementLst: stmts, initialCall, conditions, exp: e }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut b3: bool = false;
                    let mut ew_1: Option<Arc<DAE::Statement>> = None;
                    (ew_1, b1) = replaceOptEquationsStmts(ew.clone(), repl.clone(), condExpFunc.clone())?;
                    (stmts2, b2) = replaceEquationsStmts(stmts.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_1, b3) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: ew_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { source, level: e3, msg: e2, cond: e }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut e_3: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut b3: bool = false;
                    (e_1, b1) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_2, b2) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_3, b3) = replaceExp(e3.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_ASSERT { cond: e_1.clone(), msg: e_2.clone(), level: e_3.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TERMINATE { source, msg: e }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e_1 = __pa0.clone();
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_TERMINATE { msg: e_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_REINIT { source, value: e2, var: e }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (e_1, b1) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_2, b2) = replaceExp(e2.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_REINIT { var: e_1.clone(), value: e_2.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_NORETCALL { source, exp: e }, tail: xs }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e_1 = __pa0.clone();
                    (xs_1, _) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: x, tail: xs }, _, _) => {
                    let mut xs_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    (xs_1, b1) = replaceEquationsStmts(xs.clone(), repl.clone(), condExpFunc.clone())?;
                    Ok((cons(x.clone(), xs_1.clone()), b1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAlgorithmStatementLst, replacementPerformed))
}

fn replaceEquationsElse(mut inElse: Arc<DAE::Else>, mut repl: VariableReplacements, mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<(Arc<DAE::Else>, bool)> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut replacementPerformed: bool = false;
    (outElse, replacementPerformed) = 'mc: {
        let __mc_input = (inElse.clone(), repl.clone(), condExpFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Else::ELSEIF { exp: e, statementLst: st, else_: el }, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut st_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut el_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut b3: bool = false;
                    (el_1, b1) = replaceEquationsElse(el.clone(), repl.clone(), condExpFunc.clone())?;
                    (st_1, b2) = replaceEquationsStmts(st.clone(), repl.clone(), condExpFunc.clone())?;
                    (e_1, b3) = replaceExp(e.clone(), repl.clone(), condExpFunc.clone())?;
                    let true = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Else::ELSEIF { exp: e_1.clone(), statementLst: st_1.clone(), else_: el_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Else::ELSE { statementLst: st }, _, _) => {
                    let mut st_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceEquationsStmts(st.clone(), repl.clone(), condExpFunc.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    st_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Else::ELSE { statementLst: st_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inElse.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElse, replacementPerformed))
}

fn replaceOptEquationsStmts(mut optStmt: Option<Arc<DAE::Statement>>, mut inVariableReplacements: VariableReplacements, mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<(Option<Arc<DAE::Statement>>, bool)> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outAlgorithmStatementLst: Option<Arc<DAE::Statement>> = None;
    let mut replacementPerformed: bool = false;
    (outAlgorithmStatementLst, replacementPerformed) = 'mc: {
        let __mc_input = (optStmt.clone(), inVariableReplacements.clone(), condExpFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(stmt), _, _) => {
                    let mut stmt2: Arc<DAE::Statement>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceEquationsStmts(list![stmt.clone()], inVariableReplacements.clone(), condExpFunc.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    stmt2 = __pa0.clone();
                    Ok((Some(stmt2.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((optStmt.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAlgorithmStatementLst, replacementPerformed))
}

pub fn dumpReplacements(mut inVariableReplacements: VariableReplacements) -> Result<()> {
    let _ = (match inVariableReplacements.clone() {
        VariableReplacements { hashTable: mut ht, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut len_str: ArcStr = arcstr::literal!("");
            let mut len: i32 = 0;
            let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
            tplLst = BaseHashTable::hashTableList(ht.clone());
            r#str = stringDelimitList(List::map(tplLst.clone(), Arc::new(printReplacementTupleStr)), (literal!("\n")).clone());
            println!("{}", (literal!("Replacements: (")).clone());
            len = (tplLst.clone().len() as i32);
            len_str = (intString(len.clone())).clone();
            println!("{}", (len_str.clone()).clone());
            println!("{}", (literal!(")\n")).clone());
            println!("{}", (literal!("=============\n")).clone());
            println!("{}", (r#str.clone()).clone());
            println!("{}", (literal!("\n")).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn dumpReplacementsStr(mut inVariableReplacements: VariableReplacements) -> Result<ArcStr> {
    let mut ostr: ArcStr = arcstr::literal!("");
    ostr = ((match inVariableReplacements.clone() {
        VariableReplacements { hashTable: mut ht, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
            tplLst = BaseHashTable::hashTableList(ht.clone());
            r#str = stringDelimitList(List::map(tplLst.clone(), Arc::new(printReplacementTupleStr)), (literal!("\n")).clone());
            s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Replacements: (")); __mm_s.push_str(&*intString((tplLst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n=============\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            s1.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(ostr)
}

pub fn getAllReplacements(mut inVariableReplacements: VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dsts: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    (crefs, dsts) = (match inVariableReplacements.clone() {
        VariableReplacements { hashTable: mut ht, .. } => {
            let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
            tplLst = BaseHashTable::hashTableList(ht.clone());
            crefs = List::map(tplLst.clone(), Arc::new(fnptr!(Util::tuple21, _)));
            dsts = List::map(tplLst.clone(), Arc::new(fnptr!(Util::tuple22, _)));
            (crefs.clone(), dsts.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((crefs, dsts))
}

fn printReplacementTupleStr(mut tpl: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple21(tpl.clone()))?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::tuple22(tpl.clone()))?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn replacementSources(mut repl: VariableReplacements) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut sources: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    sources = (match repl.clone() {
        VariableReplacements { hashTable: mut ht, invHashTable: _ } => {
            sources = BaseHashTable::hashTableKeyList(ht.clone());
            sources.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(sources)
}

pub fn replacementTargets(mut repl: VariableReplacements) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut sources: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    sources = (match repl.clone() {
        VariableReplacements { hashTable: mut ht, invHashTable: _ } => {
            let mut targets: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut targets2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            targets = BaseHashTable::hashTableValueList(ht.clone());
            targets2 = List::flatten(List::map(targets.clone(), Arc::new(Expression::extractCrefsFromExp)));
            targets2.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(sources)
}

pub fn addReplacementLst(mut inRepl: VariableReplacements, mut crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut dsts: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<VariableReplacements> {
    let mut repl: VariableReplacements;
    repl = (::match_deref::match_deref! { match &((inRepl.clone(), crs.clone(), dsts.clone())) {
        (repl, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            repl.clone()
        },
        (repl, Deref @ metamodelica::List::Cons { head: cr, tail: crrest }, Deref @ metamodelica::List::Cons { head: dst, tail: dstrest }) => {
            let mut repl = (*repl).clone();
            repl = addReplacement(repl.clone(), cr.clone(), dst.clone())?;
            repl = addReplacementLst(repl.clone(), crrest.clone(), dstrest.clone())?;
            repl.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(repl)
}

pub fn addReplacement(mut repl: VariableReplacements, mut inSrc: Arc<DAE::ComponentRef>, mut inDst: Arc<DAE::Exp>) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = (repl.clone(), inSrc.clone(), inDst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (VariableReplacements { .. }, src, dst) => {
                    let mut src_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut dst_1: Arc<DAE::Exp>;
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                    let mut ht_1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                    let mut invHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
                    let mut invHt_1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(makeTransitive(repl.clone(), src.clone(), dst.clone())?) {
                        (VariableReplacements { hashTable: __pa0, invHashTable: __pa1 }, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ht = __pa0.clone();
                    invHt = __pa1.clone();
                    src_1 = __pa2.clone();
                    dst_1 = __pa3.clone();
                    ht_1 = BaseHashTable::add((src_1.clone(), dst_1.clone()), ht.clone())?;
                    invHt_1 = addReplacementInv(invHt.clone(), src_1.clone(), dst_1.clone())?;
                    Ok(VariableReplacements { hashTable: ht_1.clone(), invHashTable: invHt_1.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-add_replacement failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRepl)
}

pub fn addReplacementNoTransitive(mut repl: VariableReplacements, mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements = repl.clone();
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut invHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    let VariableReplacements { hashTable: __pa0, invHashTable: __pa1 } = (outRepl.clone()) else { bail!("pattern mismatch") };
    ht = __pa0.clone();
    invHt = __pa1.clone();
    ht = BaseHashTable::add((src.clone(), dst.clone()), ht.clone())?;
    invHt = addReplacementInv(invHt.clone(), src.clone(), dst.clone())?;
    outRepl = VariableReplacements { hashTable: ht.clone(), invHashTable: invHt.clone() };
    Ok(outRepl)
}

fn addReplacementInv(mut invHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr)), mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr))> {
    let mut outInvHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    outInvHt = (::match_deref::match_deref! { match &((invHt.clone(), src.clone(), dst.clone())) {
        (_, _, _) => {
            let mut invHt_1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
            let mut dests: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            dests = Expression::extractCrefsFromExp(dst.clone())?;
            invHt_1 = List::fold1r(dests.clone(), Arc::new(addReplacementInv2), src.clone(), invHt.clone());
            invHt_1.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInvHt)
}

fn addReplacementInv2(mut invHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr)), mut dst: Arc<DAE::ComponentRef>, mut src: Arc<DAE::ComponentRef>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr))> {
    let mut outInvHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    let mut srcs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    if BaseHashTable::hasKey(dst.clone(), invHt.clone()) {
        srcs = BaseHashTable::get(dst.clone(), invHt.clone())?;
        srcs = amortizeUnion(cons(src.clone(), srcs.clone()));
        outInvHt = BaseHashTable::add((dst.clone(), srcs.clone()), invHt.clone())?;
    } else {
        outInvHt = BaseHashTable::add((dst.clone(), list![src.clone()]), invHt.clone())?;
    }
    Ok(outInvHt)
}

fn amortizeUnion(mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Arc<metamodelica::List<Arc<DAE::ComponentRef>>> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    crefs = (::match_deref::match_deref! { match &(inCrefs.clone()) {
        _ if (intMod((inCrefs.clone().len() as i32), 7) == 0) => List::union(metamodelica::nil(), inCrefs.clone()),
        _ => inCrefs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    crefs
}

pub fn addReplacementIfNot(mut condition: bool, mut repl: VariableReplacements, mut inSrc: Arc<DAE::ComponentRef>, mut inDst: Arc<DAE::Exp>) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = (::match_deref::match_deref! { match &((condition.clone(), repl.clone(), inSrc.clone(), inDst.clone())) {
        (false, _, src, dst) => {
            let mut repl_1: VariableReplacements;
            repl_1 = addReplacement(repl.clone(), src.clone(), dst.clone())?;
            repl_1.clone()
        },
        (true, _, _, _) => {
            repl.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRepl)
}

fn makeTransitive(mut repl: VariableReplacements, mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>) -> Result<(VariableReplacements, Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outRepl: VariableReplacements;
    let mut outSrc: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outDst: Arc<DAE::Exp>;
    (outRepl, outSrc, outDst) = (::match_deref::match_deref! { match &((repl.clone(), src.clone(), dst.clone())) {
        (_, _, _) => {
            let mut repl_1: VariableReplacements;
            let mut repl_2: VariableReplacements;
            let mut src_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut src_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dst_1: Arc<DAE::Exp>;
            let mut dst_2: Arc<DAE::Exp>;
            let mut dst_3: Arc<DAE::Exp>;
            (repl_1, src_1, dst_1) = makeTransitive1(repl.clone(), src.clone(), dst.clone())?;
            (repl_2, src_2, dst_2) = makeTransitive2(repl_1.clone(), src_1.clone(), dst_1.clone())?;
            (dst_3, _) = ExpressionSimplify::simplify1(dst_2.clone())?;
            (repl_2.clone(), src_2.clone(), dst_3.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outRepl, outSrc, outDst))
}

fn makeTransitive1(mut repl: VariableReplacements, mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>) -> Result<(VariableReplacements, Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outRepl: VariableReplacements;
    let mut outSrc: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outDst: Arc<DAE::Exp>;
    (outRepl, outSrc, outDst) = 'mc: {
        let __mc_input = (repl.clone(), src.clone(), dst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (VariableReplacements { hashTable: _, invHashTable: invHt }, _, _) => {
                    let mut lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut repl_1: VariableReplacements;
                    let mut singleRepl: VariableReplacements;
                    lst = BaseHashTable::get(src.clone(), invHt.clone())?;
                    singleRepl = addReplacementNoTransitive(emptyReplacementsSized(53), src.clone(), dst.clone())?;
                    repl_1 = makeTransitive12(lst.clone(), repl.clone(), singleRepl.clone())?;
                    Ok((repl_1.clone(), src.clone(), dst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((repl.clone(), src.clone(), dst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRepl, outSrc, outDst))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeTransitive12(mut lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut repl: VariableReplacements, mut singleRepl: VariableReplacements) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = (::match_deref::match_deref! { match &((lst.clone(), repl.clone(), singleRepl.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            repl.clone()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: crs }, VariableReplacements { hashTable: ht, .. }, _) => {
            let mut crDst: Arc<DAE::Exp>;
            let mut repl1: VariableReplacements;
            let mut repl2: VariableReplacements;
            crDst = BaseHashTable::get(cr.clone(), ht.clone())?;
            (crDst, _) = replaceExp(crDst.clone(), singleRepl.clone(), None)?;
            repl1 = addReplacementNoTransitive(repl.clone(), cr.clone(), crDst.clone())?;
            repl2 = makeTransitive12(crs.clone(), repl1.clone(), singleRepl.clone())?;
            repl2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRepl)
}

fn makeTransitive2(mut repl: VariableReplacements, mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>) -> Result<(VariableReplacements, Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outRepl: VariableReplacements;
    let mut outSrc: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outDst: Arc<DAE::Exp>;
    (outRepl, outSrc, outDst) = 'mc: {
        let __mc_input = (repl.clone(), src.clone(), dst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut dst_1: Arc<DAE::Exp>;
                    (dst_1, _) = replaceExp(dst.clone(), repl.clone(), None)?;
                    Ok((repl.clone(), src.clone(), dst_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((repl.clone(), src.clone(), dst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRepl, outSrc, outDst))
}

pub fn getReplacement(mut inVariableReplacements: VariableReplacements, mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut outComponentRef: Arc<DAE::Exp>;
    outComponentRef = (::match_deref::match_deref! { match &((inVariableReplacements.clone(), inComponentRef.clone())) {
        (VariableReplacements { hashTable: ht, .. }, src) => {
            let mut dst: Arc<DAE::Exp>;
            dst = BaseHashTable::get(src.clone(), ht.clone())?;
            dst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn replaceExpOpt(mut inExp: Option<Arc<DAE::Exp>>, mut repl: VariableReplacements, mut funcOpt: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<Option<Arc<DAE::Exp>>> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outExp: Option<Arc<DAE::Exp>> = None;
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), repl.clone(), funcOpt.clone())) {
        (Some(e), _, _) => {
            let mut e = (*e).clone();
            (e, _) = replaceExp(e.clone(), repl.clone(), funcOpt.clone())?;
            Some(e.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn avoidDoubleHashLookup(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_UNKNOWN }, _) => {
                    Ok(Expression::makeCrefExp(cr.clone(), inType.clone())?)
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

pub fn replaceExpRepeated(mut e: Arc<DAE::Exp>, mut repl: VariableReplacements, mut func: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>, mut maxIter: i32) -> Result<Arc<DAE::Exp>> {
    pub type VisitFunc = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outExp: Arc<DAE::Exp>;
    outExp = replaceExpRepeated2(e.clone(), repl.clone(), func.clone(), maxIter.clone(), 1, false)?;
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn replaceExpRepeated2(mut e: Arc<DAE::Exp>, mut repl: VariableReplacements, mut func: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>, mut maxIter: i32, mut i: i32, mut equal: bool) -> Result<Arc<DAE::Exp>> {
    pub type VisitFunc = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (e.clone(), repl.clone(), func.clone(), maxIter.clone(), i.clone(), equal.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let true = (i.clone() > maxIter.clone()) else { bail!("pattern mismatch") };
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, true) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    (e1, b) = replaceExp(e.clone(), repl.clone(), func.clone())?;
                    res = replaceExpRepeated2(e1.clone(), repl.clone(), func.clone(), maxIter.clone(), i.clone() + 1, !(b.clone()))?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn replaceExp(mut inExp: Arc<DAE::Exp>, mut inVarReplacements: VariableReplacements, mut inCondition: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<(Arc<DAE::Exp>, bool)> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outExp: Arc<DAE::Exp>;
    let mut replacementPerformed: bool = false;
    outExp = inExp.clone();
    if replaceExpCond(inCondition.clone(), inExp.clone()) {
        (outExp, _) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new({ let __pe_b1 = inVarReplacements.clone(); let __pe_b2 = inCondition.clone(); move |__pe_a0, __pe_a3| replaceExpCref(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), true)?;
    }
    replacementPerformed = !(referenceEq(&outExp.clone(),&inExp.clone()));
    Ok((outExp, replacementPerformed))
}

fn replaceExpCref(mut inExp: Arc<DAE::Exp>, mut inVarReplacements: VariableReplacements, mut inCondition: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>, mut inReplacementPerformed: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outExp: Arc<DAE::Exp>;
    let mut replacementPerformed: bool = false;
    if !(replaceExpCond(inCondition.clone(), inExp.clone())) {
        Error::addInternalError((literal!("Got exp to replace when condition is not allowing replacements. Check traversal.")).clone(), metamodelica::sourceInfo!())?;
    }
    replacementPerformed = false;
    outExp = inExp.clone();
    let _ = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            match '__try0: {
                outExp = unwrap_break_err!(getReplacement(inVarReplacements.clone(), cr.clone()), '__try0);
                outExp = unwrap_break_err!(avoidDoubleHashLookup(outExp.clone(), var_field!((*inExp).ty, DAE::Exp::CREF).clone()), '__try0);
                replacementPerformed = true;
                Ok::<_, anyhow::Error>((outExp.clone(), replacementPerformed.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    outExp = __try0_o0;
                    replacementPerformed = __try0_o1;
                }
                Err(_) => {
                    bail!("try/else: outputs not set in else branch");
                }
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, replacementPerformed))
}

pub fn replaceExpList(mut iexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut repl: VariableReplacements, mut cond: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, bool)> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    let mut acc1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut acc2: bool = false;
    let mut c: bool = false;
    for mut exp in &*iexpl.clone() {
        let mut exp = exp.clone();
        (exp, c) = replaceExp(exp.clone(), repl.clone(), cond.clone())?;
        acc2 = acc2.clone() || c.clone();
        acc1 = cons(exp.clone(), acc1.clone());
    }
    outExpl = acc1.clone().reverse();
    replacementPerformed = acc2.clone();
    Ok((outExpl, replacementPerformed))
}

fn replaceExpCond(mut inFuncTypeExpExpToBooleanOption: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>, mut inExp: Arc<DAE::Exp>) -> bool {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inFuncTypeExpExpToBooleanOption.clone(), inExp.clone())) {
        (Some(cond), e) => {
            let mut res: bool = false;
            res = cond(e.clone()).unwrap();
            res.clone()
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn replaceExpMatrix(mut inTplExpExpBooleanLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, bool)> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outTplExpExpBooleanLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    let mut acc1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut acc2: bool = false;
    let mut c: bool = false;
    for mut exp in &*inTplExpExpBooleanLstLst.clone() {
        let mut exp = exp.clone();
        (exp, c) = replaceExpList(exp.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
        acc2 = acc2.clone() || c.clone();
        acc1 = cons(exp.clone(), acc1.clone());
    }
    outTplExpExpBooleanLstLst = acc1.clone().reverse();
    replacementPerformed = acc2.clone();
    Ok((outTplExpExpBooleanLstLst, replacementPerformed))
}

