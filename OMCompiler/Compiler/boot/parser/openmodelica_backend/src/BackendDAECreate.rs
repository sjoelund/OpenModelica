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
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::Vectorization;
use crate::ZeroCrossings;
use openmodelica_ast::Absyn;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::HashTable;
use openmodelica_frontend::HashTableCrToExpSourceTpl;
use openmodelica_frontend::HashTableExpToExp;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend::Inline;
use openmodelica_frontend::Types;
use openmodelica_frontend::VarTransform;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::StackOverflow;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub type Functiontuple = /* ? */;

pub type ArrayBindingList = Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>;

fn printArrayBindingList(mut arrayBindingList: ArrayBindingList) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    let mut subscriptLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut bindingExp: Arc<DAE::Exp>;
    for mut tpl in &*arrayBindingList.clone() {
        let mut tpl = tpl.clone();
        (subscriptLst, bindingExp) = tpl.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[")); ArcStr::from(__mm_s) }).clone();
        for mut subscript in &*subscriptLst.clone() {
            let mut subscript = subscript.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*intString(subscript.clone())); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(bindingExp.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn patchRecordBindings(mut varlst: Arc<metamodelica::List<BackendDAE::Var>>, mut extvarlst: Arc<metamodelica::List<BackendDAE::Var>>, mut globalKnownVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut globalKnownVarLst: Arc<metamodelica::List<BackendDAE::Var>> = globalKnownVarLst;
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = eqns;
    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = reqns;
    let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = ieqns;
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>;
    let mut arrayMap: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>>>;
    let mut debug: bool = false;
    map = UnorderedMap::new((std::sync::Arc::new(ComponentReference::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), 1);
    collectRecordTypesVarLst(map.clone(), globalKnownVarLst.clone())?;
    eqns = List::map(eqns.clone(), Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone()) }));
    reqns = List::map(reqns.clone(), Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone()) }));
    ieqns = List::map(ieqns.clone(), Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone()) }));
    arrayMap = UnorderedMap::new((std::sync::Arc::new(ComponentReference::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), 1);
    let _ = List::map(varlst.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = arrayMap.clone(); move |__pe_a0| collectRecordElementBindings(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    let _ = List::map(globalKnownVarLst.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = arrayMap.clone(); move |__pe_a0| collectRecordElementBindings(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    let _ = List::map(extvarlst.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = arrayMap.clone(); move |__pe_a0| collectRecordElementBindings(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    map = collapseArrayBindings(arrayMap.clone(), map.clone())?;
    if debug.clone() {
        println!("{}", (literal!("patchRecordBindings arrayMap:\n")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*UnorderedMap::toString(arrayMap.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(printArrayBindingList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\npatchRecordBindings map\n")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(TypesDump::printTypeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    globalKnownVarLst = updateRecordTypesVarLst(map.clone(), globalKnownVarLst.clone())?;
    eqns = List::map(eqns.clone(), Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone()) }));
    reqns = List::map(reqns.clone(), Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone()) }));
    ieqns = List::map(ieqns.clone(), Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone()) }));
    Ok((globalKnownVarLst, eqns, reqns, ieqns))
}

fn collectRecordElementBindings(mut var: BackendDAE::Var, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>, mut arrayMap: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>>>) -> Result<()> {
    let mut rec_cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut is_rec: bool = false;
    (rec_cref, is_rec) = ComponentReference::crefGetFirstRec(var.varName.clone())?;
    let () = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(binding) if (is_rec.clone() && UnorderedMap::contains(rec_cref.clone(), map.clone()) && Expression::isConst(binding.clone())?) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut arrayCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut arrayBindingExpList: ArrayBindingList = metamodelica::nil();
            let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut intSubLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            if ComponentReference::isArrayElement(var.varName.clone()) {
                arrayCref = ComponentReference::crefStripSubsExceptModelSubs(var.varName.clone());
                arrayBindingExpList = UnorderedMap::getOrDefault(arrayCref.clone(), arrayMap.clone(), metamodelica::nil());
                subscriptLst = ComponentReferenceBasics::crefSubs(var.varName.clone())?;
                intSubLst = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut subscript in (subscriptLst.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            i.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collectRecordElementBindings")); __mm_s.push_str(&*literal!(" failed because index not integer.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                UnorderedMap::add(arrayCref.clone(), cons((intSubLst.clone(), binding.clone()), arrayBindingExpList.clone()), arrayMap.clone())?;
            } else {
                ty = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(rec_cref.clone(), map.clone(), metamodelica::sourceInfo!())?) {
        ty @ Deref @ DAE::Type::T_COMPLEX { .. } => {
            let mut ty = (*ty).clone();
            assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = updateConstantRecordElementBinding(v.clone(), binding.clone(), (ComponentReferenceBasics::crefLastIdent(var.varName.clone())?).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ty.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collectRecordElementBindings")); __mm_s.push_str(&*literal!(" failed because the type is not T_COMPLEX.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                UnorderedMap::add(rec_cref.clone(), ty.clone(), map.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn updateConstantRecordElementBinding(mut var: Arc<DAE::Var>, mut binding: Arc<DAE::Exp>, mut name: ArcStr) -> Result<Arc<DAE::Var>> {
    let mut var: Arc<DAE::Var> = var;
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    if DAEUtil::isConstVar(var.clone()) && var.name.clone() == name.clone() {
        r#const = if (Expression::isConst(binding.clone())?) {openmodelica_frontend_types::DAE::Const::C_CONST} else {openmodelica_frontend_types::DAE::Const::C_VAR};
        assign_field!(var.binding = Arc::new(DAE::Binding::EQBOUND { exp: binding.clone(), evaluatedExp: None, constant_: r#const.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }));
    }
    Ok(var)
}

fn collectRecordTypesVarLst(mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>, mut varLst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<()> {
    for mut var in &*varLst.clone() {
        let mut var = var.clone();
        collectRecordTypesVar(map.clone(), var.clone())?;
    }
    Ok(())
}

fn collectRecordTypesVar(mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>, mut var: BackendDAE::Var) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(exp) => {
            let _ = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(collectRecordTypesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> + 'static>), map.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn collectRecordTypesEqn(mut eqn: Arc<BackendDAE::Equation>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<Arc<BackendDAE::Equation>> {
    let mut eqn: Arc<BackendDAE::Equation> = eqn;
    (eqn, _) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(collectRecordTypesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> + 'static>); move |__pe_a0, __pe_a2| Expression::traverseExpTopDown(__pe_a0, __pe_b1.clone(), __pe_a2) }), map.clone())?;
    Ok(eqn)
}

fn collectRecordTypesExp(mut exp: Arc<DAE::Exp>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cont: bool = false;
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>> = map;
    cont = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } if (Types::isRecord(var_field!((*exp).ty, DAE::Exp::CREF).clone()) && Types::recordHasConstVar(var_field!((*exp).ty, DAE::Exp::CREF).clone())?) => {
            UnorderedMap::add(cref.clone(), var_field!((*exp).ty, DAE::Exp::CREF).clone(), map.clone())?;
            false
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, cont, map))
}

fn updateRecordTypesEqn(mut eqn: Arc<BackendDAE::Equation>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<Arc<BackendDAE::Equation>> {
    let mut eqn: Arc<BackendDAE::Equation> = eqn;
    (eqn, _) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(updateRecordTypesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> + 'static>); move |__pe_a0, __pe_a2| Expression::traverseExpTopDown(__pe_a0, __pe_b1.clone(), __pe_a2) }), map.clone())?;
    Ok(eqn)
}

fn updateRecordTypesExp(mut exp: Arc<DAE::Exp>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cont: bool = false;
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>> = map;
    (exp, cont) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } if (UnorderedMap::contains(cref.clone(), map.clone())) => {
            assign_variant_field!(exp => DAE::Exp::CREF; ty = UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!())?);
            (exp.clone(), false)
        },
        _ => {
            (exp.clone(), true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, cont, map))
}

fn compareArrayBindingExp(mut inElement1: (Arc<metamodelica::List<i32>>, Arc<DAE::Exp>), mut inElement2: (Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)) -> Result<bool> {
    let mut inRes: bool = false;
    let mut indiceLstElem1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut indiceLstElem2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rest_e2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e2: i32 = 0;
    (indiceLstElem1, _) = inElement1.clone();
    (indiceLstElem2, _) = inElement2.clone();
    if (indiceLstElem1.clone().len() as i32) != (indiceLstElem2.clone().len() as i32) {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.compareArrayBindingExp")); __mm_s.push_str(&*literal!(" failed because lists have different lengths.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    rest_e2 = indiceLstElem2.clone();
    for mut e1 in &*indiceLstElem1.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_e2 = __pa1.clone();
        if e1.clone() < e2.clone() {
            inRes = true;
            return Ok(inRes);
        } else if e1.clone() > e2.clone() {
            inRes = false;
            return Ok(inRes);
        }
    }
    inRes = true;
    return Ok(inRes);
    Ok(inRes)
}

fn collapseArrayBindings(mut arrayMap: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>> {
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>> = map;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut rec_cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut arrayBindingExpList: ArrayBindingList = metamodelica::nil();
    let mut subscriptLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut binding: Arc<DAE::Exp>;
    let mut scalarBinding: Arc<DAE::Exp>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut firstDim: i32 = 0;
    for mut pair in &*UnorderedMap::toList(arrayMap.clone()) {
        let mut pair = pair.clone();
        (cref, arrayBindingExpList) = pair.clone();
        arrayBindingExpList = List::sort(arrayBindingExpList.clone(), (std::sync::Arc::new(compareArrayBindingExp) as std::sync::Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, Arc<DAE::Exp>), (Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)) -> Result<bool> + 'static>))?;
        expLst = metamodelica::nil();
        for mut scalBind in &*arrayBindingExpList.clone() {
            let mut scalBind = scalBind.clone();
            (subscriptLst, scalarBinding) = scalBind.clone();
            expLst = cons(scalarBinding.clone(), expLst.clone());
        }
        binding = (match (subscriptLst.clone().len() as i32) {
        1 => {
            Arc::new(DAE::Exp::ARRAY { ty: ComponentReference::crefTypeFull(cref.clone())?, scalar: true, array: expLst.clone() })
        },
        2 => {
            let mut matLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            dims = TypesDump::getDimensions(ComponentReference::crefLastType(cref.clone())?);
            firstDim = (::match_deref::match_deref! { match &(listHead(dims.clone())?) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: firstDim } => firstDim.clone(),
        _ => bail!("match: no arm matched"),
    } });
            if let Ok(__iflet0) = List::splitEqualParts(expLst.clone(), firstDim.clone()) {
                matLst = __iflet0;
            } else {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings")); __mm_s.push_str(&*literal!(" failed to reshape matrix.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                bail!("fail");
            }
            Arc::new(DAE::Exp::MATRIX { ty: ComponentReference::crefTypeFull(cref.clone())?, integer: firstDim.clone(), matrix: matLst.clone() })
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings")); __mm_s.push_str(&*literal!("failed. Array of dimension greater 2 not yet supported. Open a ticket about it.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
        let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefGetFirstRec(cref.clone())?) {
            (__pa0, true) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        rec_cref = __pa0.clone();
        ty = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(rec_cref.clone(), map.clone(), metamodelica::sourceInfo!())?) {
        ty @ Deref @ DAE::Type::T_COMPLEX { .. } => {
            let mut ty = (*ty).clone();
            assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = updateConstantRecordElementBinding(v.clone(), binding.clone(), (ComponentReferenceBasics::crefLastIdent(cref.clone())?).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ty.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings")); __mm_s.push_str(&*literal!(" failed because the type is not T_COMPLEX.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        UnorderedMap::add(rec_cref.clone(), ty.clone(), map.clone())?;
    }
    Ok(map)
}

fn updateRecordTypesVarLst(mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>, mut varLst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = varLst;
    varLst = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (varLst.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(exp) => {
            let mut exp = (*exp).clone();
            (exp, _) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(updateRecordTypesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> + 'static>), map.clone())?;
            var.bindExp = Some(exp.clone());
            var.clone()
        },
        _ => {
            var.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(varLst)
}

fn getExternalObjectAlias(mut inInitEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut globalVarsIn: BackendDAE::Variables, mut extVars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables)> {
    let mut oInitEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut extAliasVars: BackendDAE::Variables;
    let mut globalVarsOut: BackendDAE::Variables;
    let mut extVarsOut: BackendDAE::Variables;
    let mut extCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut aliasEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut aliasVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut repl: BackendVarTransform::VariableReplacements;
    extCrefs = BackendVariable::getAllCrefFromVariables(extVars.clone())?;
    (oEqs, aliasEqs) = List::fold1(inEqs.clone(), (std::sync::Arc::new(getExternalObjectAlias2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), extCrefs.clone(), (metamodelica::nil(), metamodelica::nil()));
    (oInitEqs, aliasEqs) = List::fold1(inInitEqs.clone(), (std::sync::Arc::new(getExternalObjectAlias2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), extCrefs.clone(), (metamodelica::nil(), aliasEqs.clone()));
    (oRemEqs, aliasEqs) = List::fold1(inRemEqs.clone(), (std::sync::Arc::new(getExternalObjectAlias2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), extCrefs.clone(), (metamodelica::nil(), aliasEqs.clone()));
    if !(aliasEqs.clone().is_empty()) {
        Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Alias equations of external objects are not Modelica compliant as in:\n    ")); __mm_s.push_str(&*stringDelimitList(List::map(aliasEqs.clone(), (std::sync::Arc::new(BackendDump::equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>)), (literal!("\n    ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    }
    repl = BackendVarTransform::emptyReplacements();
    (aliasVarLst, repl) = List::fold1(aliasEqs.clone(), (std::sync::Arc::new(getExternalObjectAlias3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> + 'static>), extVars.clone(), (metamodelica::nil(), repl.clone()));
    extAliasVars = BackendVariable::listVar1(aliasVarLst.clone());
    extVarsOut = BackendVariable::deleteVars(extAliasVars.clone(), extVars.clone())?;
    extVarsOut = removeExtAliasBinding(extVarsOut.clone(), repl.clone())?;
    (oEqs, _) = BackendVarTransform::replaceEquations(oEqs.clone(), repl.clone(), None)?;
    (oInitEqs, _) = BackendVarTransform::replaceEquations(oInitEqs.clone(), repl.clone(), None)?;
    (oRemEqs, _) = BackendVarTransform::replaceEquations(oRemEqs.clone(), repl.clone(), None)?;
    (globalVarsOut, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalVarsIn.clone(), (std::sync::Arc::new(BackendVarTransform::replaceVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
    oEqs = oEqs.clone().reverse();
    oInitEqs = oInitEqs.clone().reverse();
    oRemEqs = oRemEqs.clone().reverse();
    Ok((oInitEqs, oEqs, oRemEqs, extAliasVars, globalVarsOut, extVarsOut))
}

fn removeExtAliasBinding(mut extVarsIn: BackendDAE::Variables, mut repl: BackendVarTransform::VariableReplacements) -> Result<BackendDAE::Variables> {
    let mut extVarsOut: BackendDAE::Variables;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut extVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    varLst = BackendVariable::varList(extVarsIn.clone())?;
    extVarLst = metamodelica::nil();
    for mut var in &*varLst.clone() {
        let mut var = var.clone();
        var = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { bindExp: Some(Deref @ DAE::Exp::CREF { componentRef: cref, .. }), .. } => {
            if BackendVarTransform::hasReplacement(repl.clone(), cref.clone()) {
                var.bindExp = None;
            }
            var.clone()
        },
        _ => var.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        extVarLst = cons(var.clone(), extVarLst.clone());
    }
    extVarsOut = BackendVariable::listVar(extVarLst.clone());
    Ok(extVarsOut)
}

fn getExternalObjectAlias3(mut eqIn: Arc<BackendDAE::Equation>, mut extVars: BackendDAE::Variables, mut tplIn: (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> {
    let mut tplOut: (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements);
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut v1: BackendDAE::Var;
    let mut v2: BackendDAE::Var;
    let mut simVar: BackendDAE::Var;
    let mut aliasVar: BackendDAE::Var;
    let mut crefs_lhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crefs_rhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut extAliasVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut repl: BackendVarTransform::VariableReplacements;
    (extAliasVars, repl) = tplIn.clone();
    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eqIn.clone()], repl.clone(), None)?) {
        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eq = __pa0.clone();
    match '__try2: {
        (crefs_lhs, crefs_rhs) = unwrap_break_err!(BackendEquation::equationCrefsSolved(eq.clone()), '__try2);
        (extAliasVars, repl) = (::match_deref::match_deref! { match &((crefs_lhs.clone(), crefs_rhs.clone())) {
        (Deref @ metamodelica::List::Cons { head: lhs, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: rhs, tail: Deref @ metamodelica::List::Nil }) => {
            crefs_lhs = unwrap_break_err!(ComponentReference::expandCref(lhs.clone(), true), '__try2);
            crefs_rhs = unwrap_break_err!(ComponentReference::expandCref(rhs.clone(), true), '__try2);
            (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars.clone(), extAliasVars.clone(), repl.clone()), '__try2);
            (extAliasVars.clone(), repl.clone())
        },
        (Deref @ metamodelica::List::Cons { head: lhs, tail: Deref @ metamodelica::List::Nil }, _) => {
            crefs_lhs = unwrap_break_err!(ComponentReference::expandCref(lhs.clone(), true), '__try2);
            (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars.clone(), extAliasVars.clone(), repl.clone()), '__try2);
            (extAliasVars.clone(), repl.clone())
        },
        (_, Deref @ metamodelica::List::Cons { head: rhs, tail: Deref @ metamodelica::List::Nil }) => {
            crefs_rhs = unwrap_break_err!(ComponentReference::expandCref(rhs.clone(), true), '__try2);
            (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars.clone(), extAliasVars.clone(), repl.clone()), '__try2);
            (extAliasVars.clone(), repl.clone())
        },
        _ => {
            (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars.clone(), extAliasVars.clone(), repl.clone()), '__try2);
            (extAliasVars.clone(), repl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        tplOut = (extAliasVars.clone(), repl.clone());
        Ok::<_, anyhow::Error>((tplOut.clone(),))
    } {
        Ok((__try2_o0,)) => {
            tplOut = __try2_o0;
        }
        Err(_) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.getExternalObjectAlias3")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*BackendDump::equationString(eqIn.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            tplOut = tplIn.clone();
        }
    }
    Ok(tplOut)
}

fn addExternalObjectReplacementRules(mut crefs_lhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut crefs_rhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut extVars: BackendDAE::Variables, mut extAliasVars: Arc<metamodelica::List<BackendDAE::Var>>, mut repl: BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> {
    let mut extAliasVars: Arc<metamodelica::List<BackendDAE::Var>> = extAliasVars;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut lhs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut rhs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut v1: BackendDAE::Var;
    let mut v2: BackendDAE::Var;
    let mut simVar: BackendDAE::Var;
    let mut aliasVar: BackendDAE::Var;
    if (crefs_lhs.clone().len() as i32) == (crefs_rhs.clone().len() as i32) {
        for mut tpl in &*List::zip(crefs_lhs.clone(), crefs_rhs.clone()) {
            let mut tpl = tpl.clone();
            (lhs, rhs) = tpl.clone();
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(lhs.clone(), extVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(rhs.clone(), extVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }, _) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            (simVar, aliasVar) = chooseExternalAlias(v1.clone(), v2.clone())?;
            extAliasVars = cons(aliasVar.clone(), extAliasVars.clone());
            repl = BackendVarTransform::addReplacement(repl.clone(), BackendVariable::varCref(aliasVar.clone())?, Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?, None)?;
        }
    } else {
        bail!("fail");
    }
    Ok((extAliasVars, repl))
}

fn chooseExternalAlias(mut var1: BackendDAE::Var, mut var2: BackendDAE::Var) -> Result<(BackendDAE::Var, BackendDAE::Var)> {
    let mut simVar: BackendDAE::Var;
    let mut aliasVar: BackendDAE::Var;
    if BackendVariable::varHasBindExp(var1.clone()) && !(BackendVariable::varHasBindExp(var2.clone())) {
        simVar = var1.clone();
        aliasVar = BackendVariable::setBindExp(var2.clone(), Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
    } else if BackendVariable::varHasBindExp(var2.clone()) && !(BackendVariable::varHasBindExp(var1.clone())) {
        simVar = var2.clone();
        aliasVar = BackendVariable::setBindExp(var1.clone(), Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
    } else if BackendVariable::varHasBindExp(var2.clone()) && BackendVariable::varHasBindExp(var1.clone()) {
        if Expression::isCall(BackendVariable::varBindExp(var1.clone())?) {
            simVar = var1.clone();
            aliasVar = BackendVariable::setBindExp(var2.clone(), Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
        } else {
            simVar = var2.clone();
            aliasVar = BackendVariable::setBindExp(var1.clone(), Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
        }
    } else {
        simVar = var1.clone();
        aliasVar = BackendVariable::setBindExp(var2.clone(), Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
    }
    Ok((simVar, aliasVar))
}

fn getExternalObjectAlias2(mut eqIn: Arc<BackendDAE::Equation>, mut extCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut eqTplIn: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut eqTplOut: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    eqTplOut = 'mc: {
        let __mc_input = (eqIn.clone(), extCrefs.clone(), eqTplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, left: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, .. }, _, (noAliasEqs, aliasEqs)) => {
                    let true = (List::exist1(extCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr1.clone()) && List::exist1(extCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr2.clone())) else { bail!("pattern mismatch") };
                    Ok((noAliasEqs.clone(), cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, componentRef: cr2 }, exp: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, componentRef: cr1 }, .. }, _, (noAliasEqs, aliasEqs)) => {
                    let true = (List::exist1(extCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr1.clone()) && List::exist1(extCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr2.clone())) else { bail!("pattern mismatch") };
                    Ok((noAliasEqs.clone(), cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, right: Deref @ DAE::Exp::ARRAY { .. }, .. }, _, (noAliasEqs, aliasEqs)) => {
                    Ok((noAliasEqs.clone(), cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, right: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, .. }, _, (_, _)) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::ARRAY { .. }, right: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, .. }, _, (_, _)) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut noAliasEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut aliasEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    (noAliasEqs, aliasEqs) = eqTplIn.clone();
                    Ok((cons(eqIn.clone(), noAliasEqs.clone()), aliasEqs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eqTplOut)
}

// =============================================================================
// section for processing builtin expressions
//
// Insert a unique index (starting with 1) before the first arguments of some
// builtin calls. Equal calls will get the same index.
//   - delay(expr, delayTime, delayMax)
//       => delay(index, expr, delayTime, delayMax)
//   - sample(start, interval)
//       => sample(index, start, interval)
// =============================================================================
fn transformBuiltinExpression(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32, i32, i32, Arc<metamodelica::List<BackendDAE::TimeEvent>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32, i32, i32, Arc<metamodelica::List<BackendDAE::TimeEvent>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32, i32, i32, Arc<metamodelica::List<BackendDAE::TimeEvent>>);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: es, attr }, (ht, _, _, _, _)) if (BaseHashTable::hasKey(inExp.clone(), ht.clone())) => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("delay")).clone() }), expLst: cons(Arc::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), ht.clone())? }), es.clone()), attr: attr.clone() }), inTuple.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: es, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) => {
            let mut ht = (*ht).clone();
            ht = BaseHashTable::add((inExp.clone(), iDelay.clone() + 1), ht.clone())?;
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("delay")).clone() }), expLst: cons(Arc::new(DAE::Exp::ICONST { integer: iDelay.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone() + 1, iSample.clone(), iSpatial.clone(), timeEvents.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, expLst: es, attr }, (ht, _, _, _, _)) if (BaseHashTable::hasKey(inExp.clone(), ht.clone())) => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("spatialDistribution")).clone() }), expLst: cons(Arc::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), ht.clone())? }), es.clone()), attr: attr.clone() }), inTuple.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, expLst: es, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) => {
            let mut ht = (*ht).clone();
            ht = BaseHashTable::add((inExp.clone(), iSpatial.clone() + 1), ht.clone())?;
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("spatialDistribution")).clone() }), expLst: cons(Arc::new(DAE::Exp::ICONST { integer: iSpatial.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone(), iSample.clone(), iSpatial.clone() + 1, timeEvents.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: es @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: interval, tail: Deref @ metamodelica::List::Nil } }, attr }, (ht, _, _, _, _)) if (!(Types::isClockOrSubTypeClock(Expression::r#typeof(interval.clone())?)?) && BaseHashTable::hasKey(inExp.clone(), ht.clone())) => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: cons(Arc::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), ht.clone())? }), es.clone()), attr: attr.clone() }), inTuple.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: es @ Deref @ metamodelica::List::Cons { head: start, tail: Deref @ metamodelica::List::Cons { head: interval, tail: Deref @ metamodelica::List::Nil } }, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) if (!(Types::isClockOrSubTypeClock(Expression::r#typeof(interval.clone())?)?)) => {
            let mut ht = (*ht).clone();
            let mut iSample = (*iSample).clone();
            let mut timeEvents = (*timeEvents).clone();
            iSample = iSample.clone() + 1;
            timeEvents = List::appendElt(BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { index: iSample.clone(), startExp: start.clone(), intervalExp: interval.clone() }, timeEvents.clone());
            ht = BaseHashTable::add((inExp.clone(), iSample.clone()), ht.clone())?;
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: cons(Arc::new(DAE::Exp::ICONST { integer: iSample.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone(), iSample.clone(), iSpatial.clone(), timeEvents.clone()))
        },
        _ => {
            (inExp.clone(), inTuple.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

/*
 *  lower all variables
 */
fn isStateOrAlgvar(mut e: Arc<DAE::Element>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::VARIABLE, .. } => true,
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::DISCRETE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub fn lowerKnownVarSingle(mut element: Arc<DAE::Element>) -> Result<Option<BackendDAE::Var>> {
    let mut var_opt: Option<BackendDAE::Var> = None;
    var_opt = (::match_deref::match_deref! { match &(element.clone()) {
        elem @ Deref @ DAE::Element::VAR { .. } if (DAEUtil::isParamOrConstVarKind(var_field!((**elem).kind, DAE::Element::VAR).clone())) => {
            let mut visibility: bool = false;
            let mut var: BackendDAE::Var;
            visibility = DAEUtil::boolVarVisibility(var_field!((**elem).protection, DAE::Element::VAR).clone())?;
            var = BackendDAE::Var { encrypted: var_field!((**elem).encrypted, DAE::Element::VAR).clone(), initNonlinear: false, unreplaceable: false, innerOuter: DAEUtil::toDAEInnerOuter(var_field!((*element).innerOuter, DAE::Element::VAR).clone())?, connectorType: var_field!((*element).connectorType, DAE::Element::VAR).clone(), comment: var_field!((*element).comment, DAE::Element::VAR).clone(), hideResult: BackendDAEUtil::setHideResultAttribute(var_field!((*element).comment, DAE::Element::VAR).clone(), var_field!((**elem).componentRef, DAE::Element::VAR).clone()), tearingSelectOption: None, values: setMinMaxFromEnumeration(var_field!((**elem).ty, DAE::Element::VAR).clone(), DAEUtil::setProtectedAttr(var_field!((**elem).variableAttributesOption, DAE::Element::VAR).clone(), visibility.clone())?)?, source: var_field!((*element).source, DAE::Element::VAR).clone(), arryDim: var_field!((*element).dims, DAE::Element::VAR).clone(), tplExp: None, bindExp: var_field!((**elem).binding, DAE::Element::VAR).clone(), varType: lowerType(var_field!((**elem).ty, DAE::Element::VAR).clone())?, varParallelism: var_field!((**elem).parallelism, DAE::Element::VAR).clone(), varDirection: var_field!((**elem).direction, DAE::Element::VAR).clone(), varKind: lowerKnownVarkind(var_field!((**elem).kind, DAE::Element::VAR).clone(), var_field!((**elem).componentRef, DAE::Element::VAR).clone(), var_field!((**elem).direction, DAE::Element::VAR).clone(), var_field!((**elem).connectorType, DAE::Element::VAR).clone(), var_field!((**elem).protection, DAE::Element::VAR).clone())?, varName: var_field!((**elem).componentRef, DAE::Element::VAR).clone() };
            Some(var.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var_opt)
}

fn replaceFillWithExpInAttributes(mut attr: Option<Arc<DAE::VariableAttributes>>) -> Option<Arc<DAE::VariableAttributes>> {
    let mut attr: Option<Arc<DAE::VariableAttributes>> = attr;
    attr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut u = (*u).clone();
            let mut du = (*du).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            let mut n = (*n).clone();
            q = replaceFillWithExp(q.clone());
            u = replaceFillWithExp(u.clone());
            du = replaceFillWithExp(du.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            n = replaceFillWithExp(n.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut u = (*u).clone();
            let mut du = (*du).clone();
            q = replaceFillWithExp(q.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            u = replaceFillWithExp(u.clone());
            du = replaceFillWithExp(du.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        _ => {
            attr.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attr
}

fn replaceFillWithExp(mut bind: Option<Arc<DAE::Exp>>) -> Option<Arc<DAE::Exp>> {
    let mut bind: Option<Arc<DAE::Exp>> = bind;
    let mut e1: Arc<DAE::Exp>;
    bind = (::match_deref::match_deref! { match &(bind.clone()) {
        Some(Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, .. }) => Some(e1.clone()),
        _ => bind.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    bind
}

fn buildAssertAlgorithms(mut assrtIn: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut source: Arc<DAE::ElementSource>, mut eqIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut eqOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = eqIn.clone();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut assrt in &*assrtIn.clone() {
        let mut assrt = assrt.clone();
        eqOut = cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: Arc::new(DAE::Algorithm { statementLst: list![assrt.clone()] }), source: source.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), eqOut.clone());
    }
    eqOut
}

fn setMinMaxFromEnumeration(mut inType: Arc<DAE::Type>, mut inVarAttr: Option<Arc<DAE::VariableAttributes>>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outVarAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outVarAttr = 'mc: {
        let __mc_input = (inType.clone(), inVarAttr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { names, path, .. }, _) => {
                    let mut min: Option<Arc<DAE::Exp>> = None;
                    let mut max: Option<Arc<DAE::Exp>> = None;
                    (min, max) = DAEUtil::getMinMaxValues(inVarAttr.clone());
                    Ok(setMinMaxFromEnumeration1(min.clone(), max.clone(), inVarAttr.clone(), path.clone(), names.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVarAttr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarAttr)
}

fn setMinMaxFromEnumeration1(mut inMin: Option<Arc<DAE::Exp>>, mut inMax: Option<Arc<DAE::Exp>>, mut inVarAttr: Option<Arc<DAE::VariableAttributes>>, mut inPath: Arc<Absyn::Path>, mut inNames: Arc<metamodelica::List<ArcStr>>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outVarAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outVarAttr = 'mc: {
        let __mc_input = (inMin.clone(), inMax.clone(), inVarAttr.clone(), inPath.clone(), inNames.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, None, _, _, _) => {
                    let mut i: i32 = 0;
                    let mut namee1: Arc<Absyn::Path>;
                    let mut nameen: Arc<Absyn::Path>;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut sn: ArcStr = arcstr::literal!("");
                    i = (inNames.clone().len() as i32);
                    s1 = (listHead(inNames.clone())?).clone();
                    namee1 = AbsynUtil::joinPaths(inPath.clone(), Arc::new(Absyn::Path::IDENT { name: (s1.clone()).clone() }))?;
                    sn = ((inNames.clone()).get(i.clone())?).clone();
                    nameen = AbsynUtil::joinPaths(inPath.clone(), Arc::new(Absyn::Path::IDENT { name: (sn.clone()).clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), Some(Arc::new(DAE::Exp::ENUM_LITERAL { name: namee1.clone(), index: 1 })), Some(Arc::new(DAE::Exp::ENUM_LITERAL { name: nameen.clone(), index: i.clone() })))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, Some(_), _, _, _) => {
                    let mut namee1: Arc<Absyn::Path>;
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = (listHead(inNames.clone())?).clone();
                    namee1 = AbsynUtil::joinPaths(inPath.clone(), Arc::new(Absyn::Path::IDENT { name: (s1.clone()).clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), Some(Arc::new(DAE::Exp::ENUM_LITERAL { name: namee1.clone(), index: 1 })), inMax.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(_), None, _, _, _) => {
                    let mut i: i32 = 0;
                    let mut nameen: Arc<Absyn::Path>;
                    let mut sn: ArcStr = arcstr::literal!("");
                    i = (inNames.clone().len() as i32);
                    sn = ((inNames.clone()).get(i.clone())?).clone();
                    nameen = AbsynUtil::joinPaths(inPath.clone(), Arc::new(Absyn::Path::IDENT { name: (sn.clone()).clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), inMin.clone(), Some(Arc::new(DAE::Exp::ENUM_LITERAL { name: nameen.clone(), index: i.clone() })))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVarAttr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarAttr)
}

// protected function fixParameterStartBinding
//   input Option<DAE.Exp> bind;
//   input DAE.Type ty;
//   input Option<DAE.VariableAttributes> attr;
//   input BackendDAE.VarKind kind;
//   output Option<DAE.Exp> outBind;
// algorithm
//   outBind := matchcontinue (bind, ty, attr, kind)
//     local
//       DAE.Exp exp;
//     case (NONE(), DAE.T_REAL(source=_), _, BackendDAE.PARAM())
//       equation
//         exp = DAEUtil.getStartAttr(attr);
//       then SOME(exp);
//     else bind;
//   end matchcontinue;
// end fixParameterStartBinding;
fn lowerVarkind(mut inVarKind: DAE::VarKind, mut inType: Arc<DAE::Type>, mut inComponentRef: Arc<DAE::ComponentRef>, mut inVarDirection: DAE::VarDirection, mut inConnectorType: Arc<DAE::ConnectorType>, mut daeAttr: Option<Arc<DAE::VariableAttributes>>, mut protection: DAE::VarVisibility) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
    outVarKind = (::match_deref::match_deref! { match &((inVarKind.clone(), daeAttr.clone())) {
        (DAE::VarKind::VARIABLE, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS), .. })) if (!(Types::isDiscreteType(inType.clone()))) => BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false },
        (DAE::VarKind::VARIABLE, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::PREFER), .. })) if (!(Types::isDiscreteType(inType.clone()))) => BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false },
        _ => {
            let false = (DAEUtil::topLevelInput(inComponentRef.clone(), inVarDirection.clone(), inConnectorType.clone(), protection.clone())?) else { bail!("pattern mismatch") };
            (::match_deref::match_deref! { match &((inVarKind.clone(), inType.clone())) {
        (DAE::VarKind::VARIABLE, Deref @ DAE::Type::T_BOOL { .. }) => crate::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE, Deref @ DAE::Type::T_INTEGER { .. }) => crate::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE, Deref @ DAE::Type::T_ENUMERATION { .. }) => crate::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE, _) => crate::BackendDAE::VarKind::VARIABLE,
        (DAE::VarKind::DISCRETE, _) => crate::BackendDAE::VarKind::DISCRETE,
        _ => bail!("match: no arm matched"),
    } })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarKind)
}

fn lowerKnownVarkind(mut varKind: DAE::VarKind, mut componentRef: Arc<DAE::ComponentRef>, mut varDirection: DAE::VarDirection, mut connectorType: Arc<DAE::ConnectorType>, mut visibility: DAE::VarVisibility) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
    outVarKind = 'mc: {
        let __mc_input = varKind.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::PARAM = __mc_input.clone() else { bail!("nomatch") };
            Ok(crate::BackendDAE::VarKind::PARAM)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::CONST = __mc_input.clone() else { bail!("nomatch") };
            Ok(crate::BackendDAE::VarKind::CONST)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::VARIABLE = __mc_input.clone() else { bail!("nomatch") };
            let true = (DAEUtil::topLevelInput(componentRef.clone(), varDirection.clone(), connectorType.clone(), visibility.clone())?) else { bail!("pattern mismatch") };
            Ok(crate::BackendDAE::VarKind::VARIABLE)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("function lowerKnownVarkind failed")).clone(), metamodelica::sourceInfo!())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarKind)
}

fn lowerType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { .. } => {
                    Ok(DAE::T_CLOCK_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { .. } => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { .. } => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { .. } => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lowerType: ")); __mm_s.push_str(&*TypesDump::printTypeStr(inType.clone())?); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn lowerExtObjVarkind(mut inType: Arc<DAE::Type>) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
    let mut path: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: __pa0 }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    outVarKind = BackendDAE::VarKind::EXTOBJ { fullClassName: path.clone() };
    Ok(outVarKind)
}

/*
 *  lower all equation types
 */
fn lowerForEquation(mut eq: Arc<BackendDAE::Equation>, mut iter: ArcStr, mut range: Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> {
    let mut forEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut iterExp: Arc<DAE::Exp>;
    let mut start: Arc<DAE::Exp>;
    let mut stop: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(range.clone()) {
        Deref @ DAE::Exp::RANGE { stop: __pa0, start: __pa1, ty: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stop = __pa0.clone();
    start = __pa1.clone();
    ty = __pa2.clone();
    ty = Types::unliftArray(ty.clone())?;
    iterExp = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iter.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty.clone() });
    forEq = Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: iterExp.clone(), start: start.clone(), stop: stop.clone(), body: eq.clone(), source: BackendEquation::equationSource(eq.clone())?, attr: BackendEquation::getEquationAttributes(eq.clone())? });
    Ok(forEq)
}

fn lowerIfEquationAsserts(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut inEqns: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut otheneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
    let mut oelseenqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outEqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    (otheneqns, oelseenqs, outEqns) = (::match_deref::match_deref! { match &((conditions.clone(), theneqns.clone(), elseenqs.clone(), conditions1.clone(), theneqns1.clone(), inEqns.clone())) {
        (_, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            (beqns, eqns) = lowerIfEquationAsserts1(elseenqs.clone(), None, conditions1.clone(), metamodelica::nil(), inEqns.clone())?;
            (theneqns1.clone().reverse(), beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }, _, _, _, _) => {
            let mut eqns1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqnslst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), Some(e.clone()), conditions1.clone(), metamodelica::nil(), inEqns.clone())?;
            (eqnslst1, eqns1, eqns) = lowerIfEquationAsserts(explst.clone(), eqnslst.clone(), elseenqs.clone(), cons(e.clone(), conditions1.clone()), cons(beqns.clone(), theneqns1.clone()), eqns.clone())?;
            (eqnslst1.clone(), eqns1.clone(), eqns.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((otheneqns, oelseenqs, outEqns))
}

fn lowerIfEquationAsserts1(mut brancheqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut condition: Option<Arc<DAE::Exp>>, mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut brancheqns1: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inEqns: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut obrancheqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outEqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    (obrancheqns, outEqns) = (::match_deref::match_deref! { match &((brancheqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), inEqns.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            (brancheqns1.clone().reverse(), inEqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ASSERT { source, level, message: msg, condition: cond }, tail: eqns }, None, _, _, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), cond.clone());
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(DAE::Element::ASSERT { condition: e.clone(), message: msg.clone(), level: level.clone(), source: source.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ASSERT { source, level, message: msg, condition: cond }, tail: eqns }, Some(e), _, _, _) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: cond.clone(), expElse: Arc::new(DAE::Exp::BCONST { bool: true }) });
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e.clone());
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(DAE::Element::ASSERT { condition: e.clone(), message: msg.clone(), level: level.clone(), source: source.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::TERMINATE { source, message: msg }, tail: eqns }, None, _, _, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), Arc::new(DAE::Exp::BCONST { bool: true }));
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })], else_: Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), source: source.clone() })] }), source: source.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::TERMINATE { source, message: msg }, tail: eqns }, Some(e), _, _, _) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e.clone());
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })], else_: Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), source: source.clone() })] }), source: source.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::NORETCALL { source, exp }, tail: eqns }, None, _, _, _) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: source.clone() })], else_: Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), source: source.clone() })] }), source: source.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::NORETCALL { source, exp }, tail: eqns }, Some(e), _, _, _) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e.clone());
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: source.clone() })], else_: Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), source: source.clone() })] }), source: source.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, _, _, _, _) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), cons(eqn.clone(), brancheqns1.clone()), inEqns.clone())?;
            (beqns.clone(), eqns.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((obrancheqns, outEqns))
}

fn makeIfExp(mut cond: Arc<DAE::Exp>, mut else_: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: Arc::new(DAE::Exp::BCONST { bool: true }), expElse: else_.clone() });
    oExp
}

fn lowerArrayEqn(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut inEqAttributes: BackendDAE::EquationAttributes, mut iAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut dimensions: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ea1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut ea2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut recordSize: i32 = 0;
    tp = Expression::r#typeof(e1.clone())?;
    tp = DAEUtil::expTypeElementType(tp.clone());
    if DAEUtil::expTypeComplex(tp.clone()) {
        recordSize = Expression::sizeOf(tp.clone())?;
        dimensions = Expression::dimensionsSizes(dims.clone());
        outEqsLst = cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimensions.clone(), left: e1.clone(), right: e2.clone(), source: source.clone(), attr: inEqAttributes.clone(), recordSize: Some(recordSize.clone()) }), iAcc.clone());
    } else if (Expression::isArray(e1.clone()) || Expression::isMatrix(e1.clone())) && (Expression::isArray(e2.clone()) || Expression::isMatrix(e2.clone())) {
        ea1 = Expression::flattenArrayExpToList(e1.clone())?;
        ea2 = Expression::flattenArrayExpToList(e2.clone())?;
        outEqsLst = generateEquations(ea1.clone(), ea2.clone(), source.clone(), inEqAttributes.clone(), iAcc.clone())?;
    } else {
        dimensions = Expression::dimensionsSizes(dims.clone());
        outEqsLst = cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimensions.clone(), left: e1.clone(), right: e2.clone(), source: source.clone(), attr: inEqAttributes.clone(), recordSize: None }), iAcc.clone());
    }
    Ok(outEqsLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn generateEquations(mut iE1lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iE2lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut source: Arc<DAE::ElementSource>, mut inEqAttributes: BackendDAE::EquationAttributes, mut iAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    oEqns = (::match_deref::match_deref! { match &((iE1lst.clone(), iE2lst.clone(), source.clone(), inEqAttributes.clone(), iAcc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _, _) => {
            iAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: e1lst }, Deref @ metamodelica::List::Cons { head: e2, tail: e2lst }, _, _, _) => {
            generateEquations(e1lst.clone(), e2lst.clone(), source.clone(), inEqAttributes.clone(), cons(Arc::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: inEqAttributes.clone() }), iAcc.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oEqns)
}

fn createWhenClock(mut whenClkCnt: i32, mut e: Arc<DAE::Exp>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, BackendDAE::EquationAttributes) {
    let mut outEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outEqAttrs: BackendDAE::EquationAttributes;
    let mut eqAttrs: BackendDAE::EquationAttributes;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut var: BackendDAE::Var;
    cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(whenClkCnt.clone())); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
    outVars = cons(BackendDAE::Var { encrypted: false, initNonlinear: false, unreplaceable: true, innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), comment: None, hideResult: None, tearingSelectOption: Some(crate::BackendDAE::TearingSelect::DEFAULT), values: None, source: DAE::emptyElementSource().clone(), arryDim: metamodelica::nil(), tplExp: None, bindExp: None, varType: DAE::T_CLOCK_DEFAULT().clone(), varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varKind: crate::BackendDAE::VarKind::VARIABLE, varName: cr.clone() }, inVars.clone());
    outEqs = cons(Arc::new(BackendDAE::Equation::EQUATION { attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), source: DAE::emptyElementSource().clone(), scalar: e.clone(), exp: Arc::new(DAE::Exp::CREF { ty: DAE::T_CLOCK_DEFAULT().clone(), componentRef: cr.clone() }) }), inEqs.clone());
    outEqAttrs = BackendEquation::defaultClockedEqAttr(whenClkCnt.clone());
    (outEqs, outVars, outEqAttrs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lowerWhenTupleEqn(mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCond: Arc<DAE::Exp>, mut e: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut i: i32, mut iEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEquationLst = (::match_deref::match_deref! { match &((explst.clone(), inCond.clone(), e.clone(), source.clone(), i.clone(), iEquationLst.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            iEquationLst.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty, componentRef: cr }, tail: rest }, _, _, _, _, _) => {
            let mut size: i32 = 0;
            let mut whenEq: Arc<BackendDAE::WhenEquation>;
            let mut whenOp: BackendDAE::WhenOperator;
            size = Expression::sizeOf(ty.clone())?;
            whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: Arc::new(DAE::Exp::TSUB { exp: e.clone(), ix: i.clone(), ty: ty.clone() }), source: source.clone() };
            whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
            lowerWhenTupleEqn(rest.clone(), inCond.clone(), e.clone(), source.clone(), i.clone() + 1, cons(Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), iEquationLst.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEquationLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lowerWhenIfEqns2(mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>, mut inCond: Arc<DAE::Exp>, mut iSource: Arc<DAE::ElementSource>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEqns = (::match_deref::match_deref! { match &((crexplst.clone(), inCond.clone(), iSource.clone(), inEqns.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            inEqns.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (cr, (e, source)), tail: rest }, _, _, _) => {
            let mut size: i32 = 0;
            let mut whenEq: Arc<BackendDAE::WhenEquation>;
            let mut whenOp: BackendDAE::WhenOperator;
            let mut source = (*source).clone();
            source = ElementSource::mergeSources(iSource.clone(), source.clone())?;
            size = Expression::sizeOf(Expression::r#typeof(e.clone())?)?;
            whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: e.clone(), source: source.clone() };
            whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
            lowerWhenIfEqns2(rest.clone(), inCond.clone(), iSource.clone(), cons(Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inEqns.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqns)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lowerWhenIfEqnsMergeNestedIf(mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>, mut inCond: Arc<DAE::Exp>, mut iSource: Arc<DAE::ElementSource>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr))> {
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
    oHt = (::match_deref::match_deref! { match &((crexplst.clone(), inCond.clone(), iSource.clone(), iHt.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            iHt.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (cr, (e, source)), tail: rest }, _, _, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let mut source = (*source).clone();
            (exp, _) = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: inCond.clone(), expThen: e.clone(), expElse: exp.clone() });
            source = ElementSource::mergeSources(iSource.clone(), source.clone())?;
            ht = BaseHashTable::add((cr.clone(), (exp.clone(), source.clone())), iHt.clone())?;
            lowerWhenIfEqnsMergeNestedIf(rest.clone(), inCond.clone(), iSource.clone(), ht.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oHt)
}

fn mergeWhenEqns(mut trueEqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut elseEqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEquationLst = 'mc: {
        let __mc_input = (trueEqnList.clone(), elseEqnList.clone(), inEquationLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    Ok(inEquationLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(trueEqnList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil) => {
                    Ok(elseEqnList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(listAppend(inEquationLst.clone(), elseEqnList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(listAppend(inEquationLst.clone(), trueEqnList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: inEqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { attr, source, whenEquation: whenEq @ Deref @ BackendDAE::WhenEquation { elsewhenPart: whenElsePart, whenStmtLst, condition: cond }, size }, tail: trueEqns }, _, _) => {
                    let mut res: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut elseEqnsRest: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut result: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut whenEqRes: Arc<BackendDAE::WhenEquation>;
                    let mut added: bool = false;
                    result = inEquationLst.clone();
                    elseEqnsRest = metamodelica::nil();
                    added = false;
                    for mut eqn in &*elseEqnList.clone() {
                        let mut eqn = eqn.clone();
                        let _ = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: eq @ Deref @ BackendDAE::WhenEquation { whenStmtLst: whenStmtLst2, .. }, .. } => {
                    for mut elem in &*whenStmtLst.clone() {
                        let mut elem = elem.clone();
                        let _ = (match elem.clone() {
        BackendDAE::WhenOperator::ASSIGN { left: ref eleft, .. } => {
                    for mut stmt in &*whenStmtLst2.clone() {
                        let mut stmt = stmt.clone();
                        let _ = 'mc: {
        let __mc_input = stmt.clone();
        if let Ok(__v) = (|| -> Result<_> {
                    let BackendDAE::WhenOperator::ASSIGN { left: ref eleft2, .. } = __mc_input.clone() else { bail!("nomatch") };
                    let mut added: bool = added.clone();
                    let mut result: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = result.clone();
                    let mut res: Arc<BackendDAE::Equation>;
                    let mut whenEqRes: Arc<BackendDAE::WhenEquation>;
                    let true = (ExpressionBasics::expEqual(eleft.clone(), eleft2.clone())?) else { bail!("pattern mismatch") };
                    whenEqRes = BackendEquation::setWhenElsePart(whenEq.clone(), eq.clone())?;
                    res = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                    result = cons(res.clone(), result.clone());
                    added = true;
                    Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    let mut elseEqnsRest: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = elseEqnsRest.clone();
                    elseEqnsRest = cons(eqn.clone(), elseEqnsRest.clone());
                    Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    }
                    ()
        },
        BackendDAE::WhenOperator::REINIT { stateVar: ref crleft, .. } => {
                    for mut stmt in &*whenStmtLst2.clone() {
                        let mut stmt = stmt.clone();
                        let _ = 'mc: {
        let __mc_input = stmt.clone();
        if let Ok(__v) = (|| -> Result<_> {
                    let BackendDAE::WhenOperator::REINIT { stateVar: ref crleft2, .. } = __mc_input.clone() else { bail!("nomatch") };
                    let mut whenEqRes: Arc<BackendDAE::WhenEquation>;
                    let mut added: bool = added.clone();
                    let mut result: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = result.clone();
                    let mut res: Arc<BackendDAE::Equation>;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(crleft.clone(), crleft2.clone())?) else { bail!("pattern mismatch") };
                    whenEqRes = BackendEquation::setWhenElsePart(whenEq.clone(), eq.clone())?;
                    res = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                    result = cons(res.clone(), result.clone());
                    added = true;
                    Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    let mut elseEqnsRest: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = elseEqnsRest.clone();
                    elseEqnsRest = cons(eqn.clone(), elseEqnsRest.clone());
                    Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    }
                    ()
        },
        _ => {
                    whenEqRes = BackendEquation::setWhenElsePart(whenEq.clone(), eq.clone())?;
                    res = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                    result = cons(res.clone(), result.clone());
                    added = true;
                    ()
        },
    });
                    }
                    ()
        },
        _ => {
                    res = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: whenElsePart.clone() }), source: source.clone(), attr: attr.clone() });
                    result = cons(res.clone(), result.clone());
                    ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    }
                    if !(added.clone()) {
                        result = cons(inEqn.clone(), result.clone());
                    }
                    result = mergeWhenEqns(trueEqns.clone(), elseEqnsRest.clone(), result.clone())?;
                    Ok(result.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAECreate.mergeWhenEqns: Error in mergeWhenEqns.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEquationLst)
}

/*
 *   lower algorithms
 */
/*
 *  alias Equations
 */
fn handleAliasEquations(mut iAliasEqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns) = (::match_deref::match_deref! { match &((iAliasEqns.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iEqns.clone(), iREqns.clone(), iIEqns.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
            (iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iEqns.clone(), iREqns.clone(), iIEqns.clone())
        },
        (_, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            (vars, globalKnownVars, extvars, avars, eqns, reqns, ieqns) = handleAliasEquations1(iAliasEqns.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iEqns.clone(), iREqns.clone(), iIEqns.clone())?;
            (vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), eqns.clone(), reqns.clone(), ieqns.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns))
}

fn handleAliasEquations1(mut iAliasEqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut repl: BackendVarTransform::VariableReplacements;
    repl = BackendVarTransform::emptyReplacements();
    (oVars, outGlobalKnownVars, oExtVars, oAVars, repl, oEqns) = handleAliasEquations2(iAliasEqns.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), repl.clone(), iEqns.clone())?;
    (oAVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(oAVars.clone(), (std::sync::Arc::new(replaceAliasVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
    oVars = BackendVariable::rehashVariables(oVars.clone())?;
    (oEqns, _) = BackendVarTransform::replaceEquations(oEqns.clone(), repl.clone(), None)?;
    (oREqns, _) = BackendVarTransform::replaceEquations(iREqns.clone(), repl.clone(), None)?;
    (oIEqns, _) = BackendVarTransform::replaceEquations(iIEqns.clone(), repl.clone(), None)?;
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns))
}

fn replaceAliasVarTraverser(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> {
    let mut outVar: BackendDAE::Var;
    let mut repl: BackendVarTransform::VariableReplacements;
    (outVar, repl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { bindExp: Some(e), .. }, repl) => {
                    let mut v1: BackendDAE::Var;
                    let mut e1: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    b = Expression::isConst(e1.clone())?;
                    v1 = if (!(b.clone())) {BackendVariable::setBindExp(v.clone(), Some(e1.clone()))} else {v.clone()};
                    Ok((v1.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, repl))
}

fn handleAliasEquations2(mut iAliasEqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iRepl: BackendVarTransform::VariableReplacements, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns) = (::match_deref::match_deref! { match &((iAliasEqns.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
            (iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { source, cr2, cr1 }, tail: aliaseqns }, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut ecr1: Arc<DAE::Exp>;
            let mut ecr2: Arc<DAE::Exp>;
            ecr1 = Expression::crefExp(cr1.clone())?;
            (ecr1, _) = BackendVarTransform::replaceExp(ecr1.clone(), iRepl.clone(), None)?;
            ecr2 = Expression::crefExp(cr2.clone())?;
            (ecr2, _) = BackendVarTransform::replaceExp(ecr2.clone(), iRepl.clone(), None)?;
            (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAlias(ecr1.clone(), ecr2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
            (vars, globalKnownVars, extvars, avars, repl, eqns) = handleAliasEquations2(aliaseqns.clone(), vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone())?;
            (vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns))
}

fn selectAlias(mut exp1: Arc<DAE::Exp>, mut exp2: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iRepl: BackendVarTransform::VariableReplacements, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns) = 'mc: {
        let __mc_input = (exp1.clone(), exp2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: explst1, .. }, Deref @ DAE::Exp::ARRAY { array: explst2, .. }, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { dims: dims1, .. }, componentRef: cr1 }, Deref @ DAE::Exp::ARRAY { array: explst2, .. }, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    crefs1 = ComponentReference::expandArrayCref(cr1.clone(), dims1.clone())?;
                    explst1 = List::map(crefs1.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: explst1, .. }, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { dims: dims2, .. }, componentRef: cr2 }, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    crefs2 = ComponentReference::expandArrayCref(cr2.clone(), dims2.clone())?;
                    explst2 = List::map(crefs2.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { dims: dims1, .. }, componentRef: cr1 }, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { dims: dims2, .. }, componentRef: cr2 }, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    crefs1 = ComponentReference::expandArrayCref(cr1.clone(), dims1.clone())?;
                    explst1 = List::map(crefs1.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                    crefs2 = ComponentReference::expandArrayCref(cr2.clone(), dims2.clone())?;
                    explst2 = List::map(crefs2.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: explstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: explstlst2, .. }, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(List::flatten(explstlst1.clone()), List::flatten(explstlst2.clone()), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut arrayTyp1: i32 = 0;
                    let mut arrayTyp2: i32 = 0;
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut v1: BackendDAE::Var;
                    let mut v2: BackendDAE::Var;
                    (v1, i1, arrayTyp1) = getVar(cr1.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone())?;
                    (v2, i2, arrayTyp2) = getVar(cr2.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone())?;
                    (vars, globalKnownVars, extvars, avars, repl) = selectAliasVar(v1.clone(), i1.clone(), arrayTyp1.clone(), exp1.clone(), v2.clone(), i2.clone(), arrayTyp2.clone(), exp2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), iEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    explst1 = Expression::splitRecord(exp1.clone(), Expression::r#typeof(exp1.clone())?)?;
                    explst2 = Expression::splitRecord(exp2.clone(), Expression::r#typeof(exp2.clone())?)?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _) => {
                    Ok((iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), cons(Arc::new(BackendDAE::Equation::EQUATION { exp: exp1.clone(), scalar: exp2.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), iEqns.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns))
}

fn getVar(mut cr: Arc<DAE::ComponentRef>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, i32, i32)> {
    let mut oVar: BackendDAE::Var;
    let mut index: i32 = 0;
    let mut varrArray: i32 = 0;
    (oVar, index, varrArray) = 'mc: {
        let __mc_input = (cr.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut v: BackendDAE::Var;
                    let mut i: i32 = 0;
                    (v, i) = BackendVariable::getVarSingle(cr.clone(), iVars.clone())?;
                    Ok((v.clone(), i.clone(), 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut v: BackendDAE::Var;
                    let mut i: i32 = 0;
                    (v, i) = BackendVariable::getVarSingle(cr.clone(), inGlobalKnownVars.clone())?;
                    Ok((v.clone(), i.clone(), 2))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut v: BackendDAE::Var;
                    let mut i: i32 = 0;
                    (v, i) = BackendVariable::getVarSingle(cr.clone(), iExtVars.clone())?;
                    Ok((v.clone(), i.clone(), 3))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oVar, index, varrArray))
}

fn selectAliasLst(mut iexplst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iexplst2: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut source: Arc<DAE::ElementSource>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iRepl: BackendVarTransform::VariableReplacements, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns) = (::match_deref::match_deref! { match &((iexplst1.clone(), iexplst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
            (iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: explst1 }, Deref @ metamodelica::List::Cons { head: e2, tail: explst2 }, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, _) = BackendVarTransform::replaceExp(e1.clone(), iRepl.clone(), None)?;
            (e2, _) = BackendVarTransform::replaceExp(e2.clone(), iRepl.clone(), None)?;
            (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAlias(e1.clone(), e2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
            (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone())?;
            (vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns))
}

fn selectAliasVar(mut v1: BackendDAE::Var, mut index1: i32, mut arrayIndx1: i32, mut e1: Arc<DAE::Exp>, mut v2: BackendDAE::Var, mut index2: i32, mut arrayIndx2: i32, mut e2: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendVarTransform::VariableReplacements)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl) = (::match_deref::match_deref! { match &((v1.clone(), index1.clone(), arrayIndx1.clone(), e1.clone(), v2.clone(), index2.clone(), arrayIndx2.clone(), e2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone())) {
        (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, _, 1, _, BackendDAE::Var { varName: cr2, .. }, _, 1, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            let false = (BackendVariable::isStateVar(v2.clone())) else { bail!("pattern mismatch") };
            replaceableAlias(v2.clone())?;
            var = BackendVariable::mergeAliasVars(v1.clone(), v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source.clone());
            avar = BackendVariable::mergeVariableOperations(v2.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops.clone()));
            avar = BackendVariable::setBindExp(avar.clone(), Some(e1.clone()));
            (vars, _) = BackendVariable::removeVar(index2.clone(), iVars.clone())?;
            avars = BackendVariable::addVar(avar.clone(), iAVars.clone())?;
            vars = BackendVariable::addVar(var.clone(), vars.clone())?;
            repl = BackendVarTransform::addReplacement(iRepl.clone(), cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr2.clone(), (literal!(" = ")).clone(), e1.clone(), (literal!(" found (4).\n")).clone())?;
            }
            (vars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), avars.clone(), repl.clone())
        },
        (BackendDAE::Var { varName: cr1, .. }, _, 1, _, BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, _, 1, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            let false = (BackendVariable::isStateVar(v1.clone())) else { bail!("pattern mismatch") };
            replaceableAlias(v1.clone())?;
            var = BackendVariable::mergeAliasVars(v2.clone(), v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source.clone());
            avar = BackendVariable::mergeVariableOperations(v1.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops.clone()));
            avar = BackendVariable::setBindExp(avar.clone(), Some(e2.clone()));
            (vars, _) = BackendVariable::removeVar(index1.clone(), iVars.clone())?;
            avars = BackendVariable::addVar(avar.clone(), iAVars.clone())?;
            vars = BackendVariable::addVar(var.clone(), vars.clone())?;
            repl = BackendVarTransform::addReplacement(iRepl.clone(), cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr1.clone(), (literal!(" = ")).clone(), e2.clone(), (literal!(" found (4).\n")).clone())?;
            }
            (vars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), avars.clone(), repl.clone())
        },
        (BackendDAE::Var { varName: cr1, .. }, _, 1, _, BackendDAE::Var { varName: cr2, .. }, _, 1, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            let mut acr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut w1: i32 = 0;
            let mut w2: i32 = 0;
            let mut aindx: i32 = 0;
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut e: Arc<DAE::Exp>;
            b1 = BackendVariable::isStateVar(v1.clone());
            b2 = BackendVariable::isStateVar(v2.clone());
            let true = (boolEq(b1.clone(), b2.clone())) else { bail!("pattern mismatch") };
            replaceableAlias(v1.clone())?;
            replaceableAlias(v2.clone())?;
            w1 = BackendVariable::calcAliasKey(v1.clone())?;
            w2 = BackendVariable::calcAliasKey(v2.clone())?;
            b = intGt(w2.clone(), w1.clone());
            (acr, avar, aindx, _, _, var, e) = if (b.clone()) {(cr2.clone(), v2.clone(), index2.clone(), e2.clone(), cr1.clone(), v1.clone(), e1.clone())} else {(cr1.clone(), v1.clone(), index1.clone(), e1.clone(), cr2.clone(), v2.clone(), e2.clone())};
            var = BackendVariable::mergeAliasVars(var.clone(), avar.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source.clone());
            avar = BackendVariable::mergeVariableOperations(avar.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: acr.clone(), exp: e.clone() }), ops.clone()));
            avar = BackendVariable::setBindExp(avar.clone(), Some(e.clone()));
            avar = if (b1.clone()) {BackendVariable::setVarKind(avar.clone(), crate::BackendDAE::VarKind::DUMMY_STATE)?} else {avar.clone()};
            (vars, _) = BackendVariable::removeVar(aindx.clone(), iVars.clone())?;
            avars = BackendVariable::addVar(avar.clone(), iAVars.clone())?;
            vars = BackendVariable::addVar(var.clone(), vars.clone())?;
            repl = BackendVarTransform::addReplacement(iRepl.clone(), acr.clone(), e.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), acr.clone(), (literal!(" = ")).clone(), e.clone(), (literal!(" found (4).\n")).clone())?;
            }
            (vars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), avars.clone(), repl.clone())
        },
        (BackendDAE::Var { varName: cr1, .. }, _, 1, _, BackendDAE::Var { .. }, _, 2, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            replaceableAlias(v1.clone())?;
            var = BackendVariable::mergeAliasVars(v2.clone(), v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source.clone());
            avar = BackendVariable::mergeVariableOperations(v1.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops.clone()));
            avar = BackendVariable::setBindExp(avar.clone(), Some(e2.clone()));
            avar = if (BackendVariable::isStateVar(v1.clone())) {BackendVariable::setVarKind(avar.clone(), crate::BackendDAE::VarKind::DUMMY_STATE)?} else {avar.clone()};
            (vars, _) = BackendVariable::removeVar(index1.clone(), iVars.clone())?;
            avars = BackendVariable::addVar(avar.clone(), iAVars.clone())?;
            globalKnownVars = BackendVariable::addVar(var.clone(), inGlobalKnownVars.clone())?;
            repl = BackendVarTransform::addReplacement(iRepl.clone(), cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr1.clone(), (literal!(" = ")).clone(), e2.clone(), (literal!(" found (4).\n")).clone())?;
            }
            (vars.clone(), globalKnownVars.clone(), iExtVars.clone(), avars.clone(), repl.clone())
        },
        (BackendDAE::Var { .. }, _, 2, _, BackendDAE::Var { varName: cr2, .. }, _, 1, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            replaceableAlias(v2.clone())?;
            var = BackendVariable::mergeAliasVars(v1.clone(), v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source.clone());
            avar = BackendVariable::mergeVariableOperations(v2.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops.clone()));
            avar = BackendVariable::setBindExp(avar.clone(), Some(e1.clone()));
            avar = if (BackendVariable::isStateVar(v2.clone())) {BackendVariable::setVarKind(avar.clone(), crate::BackendDAE::VarKind::DUMMY_STATE)?} else {avar.clone()};
            (vars, _) = BackendVariable::removeVar(index2.clone(), iVars.clone())?;
            avars = BackendVariable::addVar(avar.clone(), iAVars.clone())?;
            globalKnownVars = BackendVariable::addVar(var.clone(), inGlobalKnownVars.clone())?;
            repl = BackendVarTransform::addReplacement(iRepl.clone(), cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr2.clone(), (literal!(" = ")).clone(), e1.clone(), (literal!(" found (4).\n")).clone())?;
            }
            (vars.clone(), globalKnownVars.clone(), iExtVars.clone(), avars.clone(), repl.clone())
        },
        (BackendDAE::Var { varName: cr1, .. }, _, 1, _, BackendDAE::Var { .. }, _, 3, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            replaceableAlias(v1.clone())?;
            var = BackendVariable::mergeAliasVars(v2.clone(), v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source.clone());
            avar = BackendVariable::mergeVariableOperations(v1.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops.clone()));
            avar = BackendVariable::setBindExp(avar.clone(), Some(e2.clone()));
            avar = if (BackendVariable::isStateVar(v1.clone())) {BackendVariable::setVarKind(avar.clone(), crate::BackendDAE::VarKind::DUMMY_STATE)?} else {avar.clone()};
            (vars, _) = BackendVariable::removeVar(index1.clone(), iVars.clone())?;
            avars = BackendVariable::addVar(avar.clone(), iAVars.clone())?;
            extvars = BackendVariable::addVar(var.clone(), iExtVars.clone())?;
            repl = BackendVarTransform::addReplacement(iRepl.clone(), cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr1.clone(), (literal!(" = ")).clone(), e2.clone(), (literal!(" found (4).\n")).clone())?;
            }
            (vars.clone(), inGlobalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone())
        },
        (BackendDAE::Var { .. }, _, 3, _, BackendDAE::Var { varName: cr2, .. }, _, 1, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            replaceableAlias(v2.clone())?;
            var = BackendVariable::mergeAliasVars(v1.clone(), v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source.clone());
            avar = BackendVariable::mergeVariableOperations(v2.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops.clone()));
            avar = BackendVariable::setBindExp(avar.clone(), Some(e1.clone()));
            avar = if (BackendVariable::isStateVar(v2.clone())) {BackendVariable::setVarKind(avar.clone(), crate::BackendDAE::VarKind::DUMMY_STATE)?} else {avar.clone()};
            (vars, _) = BackendVariable::removeVar(index2.clone(), iVars.clone())?;
            avars = BackendVariable::addVar(avar.clone(), iAVars.clone())?;
            extvars = BackendVariable::addVar(var.clone(), iExtVars.clone())?;
            repl = BackendVarTransform::addReplacement(iRepl.clone(), cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr2.clone(), (literal!(" = ")).clone(), e1.clone(), (literal!(" found (4).\n")).clone())?;
            }
            (vars.clone(), inGlobalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl))
}

fn replaceableAlias(mut var: BackendDAE::Var) -> Result<()> {
    let _ = (match var.clone() {
        _ => {
            let false = (BackendVariable::isVarOnTopLevelAndOutput(var.clone())) else { bail!("pattern mismatch") };
            let false = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
            let false = (BackendVariable::varHasUncertainValueRefine(var.clone())) else { bail!("pattern mismatch") };
            ()
        },
    });
    Ok(())
}

/*
 *     other helping functions
 */
fn detectImplicitDiscrete(mut inVariables: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold1(inEquationLst.clone(), (std::sync::Arc::new(detectImplicitDiscreteFold) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, BackendDAE::Variables) -> Result<BackendDAE::Variables> + 'static>), inGlobalKnownVars.clone(), inVariables.clone());
    outVariables
}

fn detectImplicitDiscreteFold(mut inEquation: Arc<BackendDAE::Equation>, mut inGlobalKnownVars: BackendDAE::Variables, mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = (inEquation.clone(), inGlobalKnownVars.clone(), inVariables.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. }, _, _) => {
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (vars, _) = BackendVariable::getVar(cr.clone(), inVariables.clone())?;
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), crate::BackendDAE::VarKind::DISCRETE);
                    Ok(BackendVariable::addVars(vars.clone(), inVariables.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. }, _, _) => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    crefs = Expression::getAllCrefs(e.clone())?;
                    crefs = List::flatten(List::map1(crefs.clone(), (std::sync::Arc::new(ComponentReference::expandCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), true));
                    (vars, _) = BackendVariable::getVarLst(crefs.clone(), inVariables.clone());
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), crate::BackendDAE::VarKind::DISCRETE);
                    Ok(BackendVariable::addVars(vars.clone(), inVariables.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst }, .. }, _, _) => {
                    Ok(detectImplicitDiscreteAlgsStatemens(inVariables.clone(), inGlobalKnownVars.clone(), statementLst.clone(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVariables.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVariables)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getVarsFromExp(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = 'mc: {
        let __mc_input = (inExpLst.clone(), inVariables.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: expLst }, variables) => {
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (vars, _) = BackendVariable::getVar(cref.clone(), variables.clone())?;
                    varLst = getVarsFromExp(expLst.clone(), variables.clone())?;
                    Ok(listAppend(vars.clone(), varLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: expLst }, variables) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    varLst = getVarsFromExp(expLst.clone(), variables.clone())?;
                    Ok(varLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn detectImplicitDiscreteAlgsStatemens(mut inVariables: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut insideWhen: bool) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = (inVariables.clone(), inGlobalKnownVars.clone(), inStatementLst.clone(), insideWhen.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, _, Deref @ metamodelica::List::Nil, _) => {
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (vars, _) = BackendVariable::getVar(cr.clone(), v.clone())?;
                    vars = List::map(vars.clone(), Arc::new({ let __pe_b1 = crate::BackendDAE::VarKind::DISCRETE; move |__pe_a0| BackendVariable::setVarKind(__pe_a0, __pe_b1.clone()) }));
                    v_1 = BackendVariable::addVars(vars.clone(), v.clone());
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::ASUB { sub: subs, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::subscriptCref(cr.clone(), subs.clone())?;
                    (vars, _) = BackendVariable::getVar(cr.clone(), v.clone())?;
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), crate::BackendDAE::VarKind::DISCRETE);
                    v_1 = BackendVariable::addVars(vars.clone(), v.clone());
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    vars = getVarsFromExp(expExpLst.clone(), v.clone())?;
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), crate::BackendDAE::VarKind::DISCRETE);
                    v_1 = BackendVariable::addVars(vars.clone(), v.clone());
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (vars, _) = BackendVariable::getVar(cr.clone(), v.clone())?;
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), crate::BackendDAE::VarKind::DISCRETE);
                    v_1 = BackendVariable::addVars(vars.clone(), v.clone());
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { statementLst, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_FOR { statementLst, range: e, iter: iteratorName, type_: tp, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut iteratorExp: Arc<DAE::Exp>;
                    let mut iteratorexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    cr = ComponentReferenceBasics::makeCrefIdent((iteratorName.clone()).clone(), tp.clone(), metamodelica::nil());
                    iteratorExp = Expression::crefExp(cr.clone())?;
                    iteratorexps = BackendDAEUtil::extendRange(e.clone(), globalKnownVars.clone())?;
                    v_1 = detectImplicitDiscreteAlgsStatemensFor(iteratorExp.clone(), iteratorexps.clone(), v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { elseWhen: None, statementLst, .. }, tail: xs }, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), false)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { elseWhen: Some(statement), statementLst, .. }, tail: xs }, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut v_3: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), list![statement.clone()], true)?;
                    v_3 = detectImplicitDiscreteAlgsStatemens(v_2.clone(), globalKnownVars.clone(), xs.clone(), false)?;
                    Ok(v_3.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: _, tail: xs }, b) => {
                    let mut v_1: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), xs.clone(), b.clone())?;
                    Ok(v_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVariables)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn detectImplicitDiscreteAlgsStatemensFor(mut inIteratorExp: Arc<DAE::Exp>, mut inExplst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inVariables: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut insideWhen: bool) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = (inIteratorExp.clone(), inExplst.clone(), inVariables.clone(), inGlobalKnownVars.clone(), inStatementLst.clone(), insideWhen.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, v, globalKnownVars, _, _) => {
                    let mut v_1: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), inStatementLst.clone(), true)?;
                    Ok(v_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, v, globalKnownVars, statementLst, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut statementLst1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()));
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst1.clone(), true)?;
                    Ok(v_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::List::Cons { head: e, tail: rest }, v, globalKnownVars, statementLst, b) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut statementLst1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()));
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst1.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemensFor(ie.clone(), rest.clone(), v_1.clone(), globalKnownVars.clone(), statementLst.clone(), b.clone())?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::List::Cons { head: e, tail: rest }, v, globalKnownVars, statementLst, b) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut statementLst1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()));
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst1.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemensFor(ie.clone(), rest.clone(), v_1.clone(), globalKnownVars.clone(), statementLst.clone(), b.clone())?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    println!("{}", (literal!("BackendDAECreate.detectImplicitDiscreteAlgsStatemensFor failed \n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVariables)
}

fn renameFunctionParameter2(mut funcIn: DAE::FunctionDefinition, mut pathName: ArcStr) -> Result<DAE::FunctionDefinition> {
    let mut funcOut: DAE::FunctionDefinition;
    funcOut = 'mc: {
        let __mc_input = (funcIn.clone(), pathName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::FunctionDefinition::FUNCTION_DEF { body: mut body }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut params: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut crefs_new: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut params_new: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut repl: VarTransform::VariableReplacements;
            let mut body = body.clone();
            params = List::filterOnTrue(body.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isParameter, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
            let false = (params.clone().is_empty()) else { bail!("pattern mismatch") };
            crefs = List::map(params.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
            crefs_new = List::map1r(crefs.clone(), (std::sync::Arc::new(ComponentReference::prependStringCref) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), (pathName.clone()).clone());
            params_new = List::map(crefs_new.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
            repl = VarTransform::emptyReplacements();
            repl = VarTransform::addReplacementLst(repl.clone(), crefs.clone(), params_new.clone())?;
            (body, _) = DAEUtil::traverseDAEElementList(body.clone(), (std::sync::Arc::new(replaceParameters) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, VarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, VarTransform::VariableReplacements)> + 'static>), repl.clone());
            Ok(DAE::FunctionDefinition::FUNCTION_DEF { body: body.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(funcIn.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(funcOut)
}

fn replaceParameters(mut inExp: Arc<DAE::Exp>, mut replIn: VarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, VarTransform::VariableReplacements)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut replOut: VarTransform::VariableReplacements;
    replOut = replIn.clone();
    (outExp, _) = VarTransform::replaceExp(inExp.clone(), replIn.clone(), None)?;
    Ok((outExp, replOut))
}

