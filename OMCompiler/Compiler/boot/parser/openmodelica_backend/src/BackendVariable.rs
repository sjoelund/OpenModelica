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
use crate::CommonSubExpression;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/* =======================================================
 *
 *  Section for functions that deals with Var
 *
 * =======================================================
 */
pub fn varEqual(mut inVar1: BackendDAE::Var, mut inVar2: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = ComponentReferenceBasics::crefEqualNoStringCompare(inVar1.varName.clone(), inVar2.varName.clone()).unwrap();
    outBoolean
}

pub fn setVarFixed(mut inVar: BackendDAE::Var, mut inBoolean: bool) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    oattr = if (isSome(inVar.values.clone())) {inVar.values.clone()} else {Some(getVariableAttributefromType(inVar.varType.clone())?)};
    outVar.values = DAEUtil::setFixedAttr(oattr.clone(), Some(Arc::new(DAE::Exp::BCONST { bool: inBoolean.clone() })))?;
    Ok(outVar)
}

pub fn removeFixedAttribute(mut var: BackendDAE::Var) -> Result<BackendDAE::Var> {
    let mut var: BackendDAE::Var = var;
    if isSome(var.values.clone()) {
        var.values = DAEUtil::setFixedAttr(var.values.clone(), None)?;
    }
    Ok(var)
}

pub fn removeStartAttribute(mut var: BackendDAE::Var) -> Result<BackendDAE::Var> {
    let mut var: BackendDAE::Var = var;
    if isSome(var.values.clone()) {
        var.values = DAEUtil::setStartAttrOption(var.values.clone(), None)?;
    }
    Ok(var)
}

pub fn varFixed(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM, .. } => {
            true
        },
        BackendDAE::Var { bindExp: Some(_), varKind: BackendDAE::VarKind::CONST, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn setVarStartValue(mut inVar: BackendDAE::Var, mut inExp: Arc<DAE::Exp>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    oattr = if (isSome(inVar.values.clone())) {inVar.values.clone()} else {Some(getVariableAttributefromType(inVar.varType.clone())?)};
    outVar.values = DAEUtil::setStartAttr(oattr.clone(), inExp.clone())?;
    Ok(outVar)
}

pub fn setVarStartValueOption(mut inVar: BackendDAE::Var, mut inExp: Option<Arc<DAE::Exp>>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    oattr = if (isSome(inVar.values.clone())) {inVar.values.clone()} else {Some(getVariableAttributefromType(inVar.varType.clone())?)};
    outVar.values = DAEUtil::setStartAttrOption(oattr.clone(), inExp.clone())?;
    Ok(outVar)
}

pub fn setVarStartOrigin(mut inVar: BackendDAE::Var, mut startOrigin: Option<Arc<DAE::Exp>>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    oattr = if (isSome(inVar.values.clone())) {inVar.values.clone()} else {Some(getVariableAttributefromType(inVar.varType.clone())?)};
    outVar.values = DAEUtil::setStartOrigin(oattr.clone(), startOrigin.clone())?;
    Ok(outVar)
}

pub fn setVarAttributes(mut inVar: BackendDAE::Var, mut inAttr: Option<Arc<DAE::VariableAttributes>>) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    outVar.values = inAttr.clone();
    outVar
}

pub fn varStartValue(mut inVar: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    sv = DAEUtil::getStartAttr(inVar.values.clone(), inVar.varType.clone())?;
    Ok(sv)
}

pub fn varUnreplaceable(mut inVar: BackendDAE::Var) -> bool {
    let mut outUnreplaceable: bool = inVar.unreplaceable.clone();
    outUnreplaceable
}

pub fn setVarUnreplaceable(mut inVar: BackendDAE::Var, mut value: bool) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    outVar.unreplaceable = value.clone();
    outVar
}

pub fn setVarInitNonlinear(mut var: BackendDAE::Var, mut value: bool) -> BackendDAE::Var {
    let mut var: BackendDAE::Var = var;
    var.initNonlinear = value.clone();
    var
}

pub fn varStartValueFail(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let BackendDAE::VAR { values: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    attr = __pa0.clone();
    sv = DAEUtil::getStartAttrFail(attr.clone())?;
    Ok(sv)
}

pub fn varNominalValueFail(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let BackendDAE::VAR { values: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    attr = __pa0.clone();
    sv = DAEUtil::getNominalAttrFail(attr.clone())?;
    Ok(sv)
}

pub fn varMinValueFail(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let BackendDAE::VAR { values: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    attr = __pa0.clone();
    sv = DAEUtil::getMinAttrFail(attr.clone())?;
    Ok(sv)
}

pub fn varMaxValueFail(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let BackendDAE::VAR { values: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    attr = __pa0.clone();
    sv = DAEUtil::getMaxAttrFail(attr.clone())?;
    Ok(sv)
}

pub fn varStartValueType(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let BackendDAE::VAR { varType: __pa0, values: __pa1, .. } = (v.clone()) else { bail!("pattern mismatch") };
    ty = __pa0.clone();
    attr = __pa1.clone();
    sv = DAEUtil::getStartAttr(attr.clone(), ty.clone())?;
    Ok(sv)
}

pub fn varStartValueOption(mut v: BackendDAE::Var) -> Result<Option<Arc<DAE::Exp>>> {
    let mut sv: Option<Arc<DAE::Exp>> = None;
    sv = 'mc: {
        let __mc_input = v.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Var { values: mut attr, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut exp: Arc<DAE::Exp>;
            exp = DAEUtil::getStartAttrFail(attr.clone())?;
            Ok(Some(exp.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(None)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(sv)
}

pub fn varHasStartValue(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut outHasStartValue: bool = false;
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let BackendDAE::VAR { values: __pa0, .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    attr = __pa0.clone();
    outHasStartValue = DAEUtil::hasStartAttr(attr.clone());
    Ok(outHasStartValue)
}

pub fn varHasNoStartValue(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut outHasNoStartValue: bool = false;
    outHasNoStartValue = !(varHasStartValue(inVar.clone())?);
    Ok(outHasNoStartValue)
}

pub fn varStartOrigin(mut v: BackendDAE::Var) -> Result<Option<Arc<DAE::Exp>>> {
    let mut so: Option<Arc<DAE::Exp>> = None;
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let BackendDAE::VAR { values: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    attr = __pa0.clone();
    so = DAEUtil::getStartOrigin(attr.clone())?;
    Ok(so)
}

pub fn varBindExp(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    let BackendDAE::VAR { bindExp: Some(__pa0), .. } = (v.clone()) else { bail!("pattern mismatch") };
    sv = __pa0.clone();
    Ok(sv)
}

pub fn varHasConstantBindExp(mut v: BackendDAE::Var) -> Result<bool> {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(v.clone()) {
        BackendDAE::Var { bindExp: Some(e), .. } => {
            Expression::isConst(e.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

pub fn varHasNonConstantBindExpOrStartValue(mut v: BackendDAE::Var) -> Result<bool> {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(v.clone()) {
        BackendDAE::Var { bindExp: Some(e), .. } => {
            !(Expression::isConstValue(e.clone())?)
        },
        _ => {
            !(varHasConstantStartExp(v.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

pub fn varHasConstantStartExp(mut v: BackendDAE::Var) -> bool {
    let mut out: bool = false;
    let mut e: Arc<DAE::Exp>;
    match '__try0: {
        e = unwrap_break_err!(varStartValueFail(v.clone()), '__try0);
        out = unwrap_break_err!(Expression::isConstValue(e.clone()), '__try0);
        Ok::<_, anyhow::Error>((out.clone(),))
    } {
        Ok((__try0_o0,)) => {
            out = __try0_o0;
        }
        Err(_) => {
            out = true;
        }
    }
    out
}

pub fn varHasBindExp(mut v: BackendDAE::Var) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(v.clone()) {
        BackendDAE::Var { bindExp: Some(_), .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub fn varBindExpStartValue(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    sv = (::match_deref::match_deref! { match &(v.clone()) {
        BackendDAE::Var { bindExp: Some(e), .. } => {
            e.clone()
        },
        _ => {
            varStartValueFail(v.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sv)
}

pub fn varBindExpStartValueNoFail(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut sv: Arc<DAE::Exp>;
    sv = (::match_deref::match_deref! { match &(v.clone()) {
        BackendDAE::Var { bindExp: Some(e), .. } => {
            e.clone()
        },
        _ => {
            varStartValue(v.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sv)
}

pub fn varStateSelect(mut inVar: BackendDAE::Var) -> DAE::StateSelect {
    let mut outStateSelect: DAE::StateSelect = DAE::StateSelect::ALWAYS;
    outStateSelect = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(stateselect), .. }), .. } => {
            stateselect.clone()
        },
        _ => {
            openmodelica_frontend_types::DAE::StateSelect::DEFAULT
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStateSelect
}

pub fn varHasStateSelect(mut inVar: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(_), .. }), .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn varStateSelectAlways(mut v: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(v.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS), .. }), varKind: BackendDAE::VarKind::STATE { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn notVarStateSelectAlways(mut v: BackendDAE::Var, mut level: i32) -> bool {
    let mut b: bool = false;
    b = (match v.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: mut diffcount, .. }, .. } => {
            !(varStateSelectAlways(v.clone()) && (diffcount.clone() == level.clone() || diffcount.clone() == 1))
        },
        _ => {
            true
        },
    });
    b
}

pub fn varStateSelectNever(mut inVar: BackendDAE::Var) -> bool {
    let mut isNever: bool = false;
    isNever = (match varStateSelect(inVar.clone()) {
        DAE::StateSelect::NEVER => true,
        _ => false,
    });
    isNever
}

pub fn varStateSelectAvoid(mut inVar: BackendDAE::Var) -> bool {
    let mut isAvoid: bool = false;
    isAvoid = (match varStateSelect(inVar.clone()) {
        DAE::StateSelect::AVOID => true,
        _ => false,
    });
    isAvoid
}

pub fn varStateSelectPrefer(mut inVar: BackendDAE::Var) -> bool {
    let mut isPrefer: bool = false;
    isPrefer = (match varStateSelect(inVar.clone()) {
        DAE::StateSelect::PREFER => true,
        _ => false,
    });
    isPrefer
}

pub fn setVarStateSelect(mut inVar: BackendDAE::Var, mut stateSelect: DAE::StateSelect) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    oattr = if (isSome(inVar.values.clone())) {inVar.values.clone()} else {Some(getVariableAttributefromType(inVar.varType.clone())?)};
    outVar.values = DAEUtil::setStateSelect(oattr.clone(), stateSelect.clone())?;
    Ok(outVar)
}

pub fn varStateSelectForced(mut inVar: BackendDAE::Var) -> bool {
    let mut isForced: bool = false;
    isForced = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS), .. }), .. } => true,
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::PREFER), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isForced
}

pub fn isNaturalState(mut var: BackendDAE::Var) -> bool {
    let mut natural: bool = false;
    natural = (match var.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { natural: mut n, .. }, .. } => {
            n.clone()
        },
        _ => {
            false
        },
    });
    natural
}

pub fn isArtificialState(mut var: BackendDAE::Var) -> bool {
    let mut artificial: bool = false;
    artificial = (match var.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { natural: mut n, .. }, .. } => {
            !(n.clone())
        },
        _ => {
            false
        },
    });
    artificial
}

pub fn varStateDerivative(mut inVar: BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> {
    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let BackendDAE::VAR { varKind: BackendDAE::STATE { derName: Some(__pa0), .. }, .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    dcr = __pa0.clone();
    Ok(dcr)
}

pub fn varHasStateDerivative(mut inVar: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn setStateDerivative(mut var: BackendDAE::Var, mut derName: Option<Arc<DAE::ComponentRef>>) -> Result<BackendDAE::Var> {
    let mut var: BackendDAE::Var = var;
    let mut index: i32 = 0;
    let mut natural: bool = false;
    let BackendDAE::STATE { natural: __pa0, index: __pa1, .. } = (var.varKind.clone()) else { bail!("pattern mismatch") };
    natural = __pa0.clone();
    index = __pa1.clone();
    var.varKind = BackendDAE::VarKind::STATE { index: index.clone(), derName: derName.clone(), natural: natural.clone() };
    Ok(var)
}

pub fn getVariableAttributefromType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::VariableAttributes>> {
    let mut attr: Arc<DAE::VariableAttributes>;
    attr = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }),
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: None, min: None, max: None, start: None, fixed: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }),
        Deref @ DAE::Type::T_BOOL { .. } => Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }),
        Deref @ DAE::Type::T_STRING { .. } => Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }),
        Deref @ DAE::Type::T_ENUMERATION { .. } => Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: None, min: None, max: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }),
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("getVariableAttributefromType called with unsopported Type!\n")).clone())?;
            }
            Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attr)
}

pub fn setVarFinal(mut inVar: BackendDAE::Var, mut finalPrefix: bool) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    oattr = if (isSome(inVar.values.clone())) {inVar.values.clone()} else {Some(getVariableAttributefromType(inVar.varType.clone())?)};
    outVar.values = DAEUtil::setFinalAttr(oattr.clone(), finalPrefix.clone())?;
    Ok(outVar)
}

pub fn setVarMinMax(mut inVar: BackendDAE::Var, mut inMin: Option<Arc<DAE::Exp>>, mut inMax: Option<Arc<DAE::Exp>>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    if isSome(inMin.clone()) || isSome(inMax.clone()) {
        oattr = if (isSome(inVar.values.clone())) {inVar.values.clone()} else {Some(getVariableAttributefromType(inVar.varType.clone())?)};
        outVar.values = DAEUtil::setMinMax(oattr.clone(), inMin.clone(), inMax.clone())?;
    }
    Ok(outVar)
}

pub fn varNominalValue(mut inVar: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let BackendDAE::VAR { values: Some(DAE::VAR_ATTR_REAL { nominal: Some(__pa0), .. }), .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    outExp = __pa0.clone();
    Ok(outExp)
}

pub fn setVarNominalValue(mut inVar: BackendDAE::Var, mut inExp: Arc<DAE::Exp>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    oattr = if (isSome(inVar.values.clone())) {inVar.values.clone()} else {Some(getVariableAttributefromType(inVar.varType.clone())?)};
    outVar.values = DAEUtil::setNominalAttr(oattr.clone(), inExp.clone())?;
    Ok(outVar)
}

pub fn varType(mut inVar: BackendDAE::Var) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let BackendDAE::VAR { varType: __pa0, .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    outType = __pa0.clone();
    Ok(outType)
}

pub fn varKind(mut inVar: BackendDAE::Var) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
    let BackendDAE::VAR { varKind: __pa0, .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    outVarKind = __pa0.clone();
    Ok(outVarKind)
}

pub fn varNominal(mut inVar: BackendDAE::Var) -> Result<metamodelica::Real> {
    let mut outReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let BackendDAE::VAR { values: Some(DAE::VAR_ATTR_REAL { nominal: Some(DAE::RCONST { real: __pa0 }), .. }), .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    outReal = __pa0.clone();
    Ok(outReal)
}

pub fn varHasNominalValue(mut inVar: BackendDAE::Var) -> bool {
    let mut outBool: bool = false;
    match '__try0: {
        let BackendDAE::VAR { values: Some(DAE::VAR_ATTR_REAL { nominal: Some(DAE::RCONST { .. }), .. }), .. } = (inVar.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        outBool = true;
        Ok::<_, anyhow::Error>((outBool.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outBool = __try0_o0;
        }
        Err(_) => {
            outBool = false;
        }
    }
    outBool
}

pub fn varCref(mut inVar: BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let BackendDAE::VAR { varName: __pa0, .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    outComponentRef = __pa0.clone();
    Ok(outComponentRef)
}

pub fn isStateVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isState(mut inCref: Arc<DAE::ComponentRef>, mut inVars: BackendDAE::Variables) -> Result<bool> {
    let mut outBool: bool = false;
    outBool = 'mc: {
        let __mc_input = inVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(getVar(inCref.clone(), inVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, _) => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBool)
}

pub fn isNonStateVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE_DER, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_CONSTR, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_FCONSTR, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_INPUT_WITH_DER, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_INPUT_DER, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_TGRID, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_LOOP_INPUT { .. }, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::ALG_STATE, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::LOOP_ITERATION, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::LOOP_SOLVED, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isClockedStateVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBool: bool = false;
    outBool = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::CLOCKED_STATE { .. }, .. } => true,
        _ => false,
    });
    outBool
}

pub fn isClockedState(mut inCref: Arc<DAE::ComponentRef>, mut inVars: BackendDAE::Variables) -> Result<bool> {
    let mut outBool: bool = false;
    outBool = 'mc: {
        let __mc_input = inVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(getVar(inCref.clone(), inVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::CLOCKED_STATE { .. }, .. }, tail: _ }, _) => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBool)
}

pub fn varHasUncertainValueRefine(mut var: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: Some(DAE::Uncertainty::REFINE), .. }), .. } => true,
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { uncertainOption: Some(DAE::Uncertainty::REFINE), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn varHasUncertainValuePropagate(mut var: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: Some(DAE::Uncertainty::PROPAGATE), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn varDistribution(mut var: BackendDAE::Var) -> Result<Arc<DAE::Distribution>> {
    let mut d: Arc<DAE::Distribution>;
    d = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { distributionOption: Some(d), .. }), .. } => d.clone(),
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { distributionOption: Some(d), .. }), .. } => d.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(d)
}

pub fn varTryGetDistribution(mut var: BackendDAE::Var) -> Option<Arc<DAE::Distribution>> {
    let mut dout: Option<Arc<DAE::Distribution>> = None;
    let mut d: Option<Arc<DAE::Distribution>> = None;
    dout = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { distributionOption: d @ Some(_), .. }), .. } => d.clone(),
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { distributionOption: d @ Some(_), .. }), .. } => d.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dout
}

pub fn varUncertainty(mut var: BackendDAE::Var) -> Result<DAE::Uncertainty> {
    let mut u: DAE::Uncertainty = DAE::Uncertainty::GIVEN;
    u = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: Some(u), .. }), .. } => u.clone(),
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { uncertainOption: Some(u), .. }), .. } => u.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(u)
}

pub fn varHasDistributionAttribute(mut var: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { distributionOption: Some(_), .. }), .. } => true,
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { distributionOption: Some(_), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn varHasUncertaintyAttribute(mut var: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: Some(_), .. }), .. } => true,
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { uncertainOption: Some(_), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isDummyStateVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isDummyDerVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isStateDerVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE_DER, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isStateorStateDerVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE_DER, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isVarDiscrete(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::CONST, .. } => true,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => true,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => true,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isVarNonDifferentiable(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, .. } => true,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => true,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => true,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isVarClockedState(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    let mut test: ArcStr = arcstr::literal!("");
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::CLOCKED_STATE { .. }, .. } => true,
        _ => false,
    });
    test = (literal!("")).clone();
    outBoolean
}

pub fn isDiscrete(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut outBoolean: bool = false;
    let mut v: BackendDAE::Var;
    (v, _) = getVarSingle(cr.clone(), vars.clone())?;
    outBoolean = isVarDiscrete(v.clone());
    Ok(outBoolean)
}

pub fn isVarNonDiscrete(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = !(isVarDiscrete(inVar.clone()));
    outBoolean
}

pub fn hasDiscreteVar(mut inBackendDAEVarLst: Arc<metamodelica::List<BackendDAE::Var>>) -> bool {
    let mut outBoolean: bool = false;
    for mut v in &*inBackendDAEVarLst.clone() {
        let mut v = v.clone();
        outBoolean = isVarDiscrete(v.clone());
        if outBoolean.clone() {
            break;
        }
    }
    outBoolean
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn hasContinuousVar(mut inBackendDAEVarLst: Arc<metamodelica::List<BackendDAE::Var>>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inBackendDAEVarLst.clone()) {
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, varKind: BackendDAE::VarKind::VARIABLE, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varType: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, varKind: BackendDAE::VarKind::VARIABLE, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE_DER, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_CONSTR, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_FCONSTR, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_INPUT_WITH_DER, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_INPUT_DER, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_TGRID, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_LOOP_INPUT { .. }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::ALG_STATE, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: vs } => {
            hasContinuousVar(vs.clone())
        },
        Deref @ metamodelica::List::Nil => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isVarNonDiscreteAlg(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, .. } => isVarAlg(var.clone()) && !(isVarDiscreteRealAlg(var.clone())) || isOptInputVar(var.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isOptInputVar(mut var: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (match var.varKind.clone() {
        BackendDAE::VarKind::OPT_LOOP_INPUT { .. } => true,
        BackendDAE::VarKind::OPT_INPUT_WITH_DER => true,
        BackendDAE::VarKind::OPT_INPUT_DER => true,
        _ => false,
    });
    b
}

pub fn isVarDiscreteRealAlg(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, varKind: BackendDAE::VarKind::DISCRETE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isVarAlg(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (match var.varKind.clone() {
        BackendDAE::VarKind::VARIABLE => true,
        BackendDAE::VarKind::DISCRETE => true,
        BackendDAE::VarKind::DUMMY_DER => true,
        BackendDAE::VarKind::DUMMY_STATE => true,
        BackendDAE::VarKind::CLOCKED_STATE { .. } => true,
        _ => false,
    });
    result
}

pub fn isVarConst(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => false,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => false,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => false,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, .. } => false,
        _ if (isConst(var.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isVarStringConst(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, .. } if (isConst(var.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isVarIntConst(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } if (isConst(var.clone())) => {
            true
        },
        BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } if (isConst(var.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isVarBoolConst(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } if (isConst(var.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

/* TODO: Is this correct? */
pub fn isVarParam(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => {
            false
        },
        BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => {
            false
        },
        BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, .. } => {
            false
        },
        BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => {
            false
        },
        _ if (isParam(var.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isVarStringParam(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, .. } if (isParam(var.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isVarIntParam(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } if (isParam(var.clone())) => true,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } if (isParam(var.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isVarBoolParam(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } if (isParam(var.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isVarConnector(mut var: BackendDAE::Var) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { connectorType: Deref @ DAE::ConnectorType::NON_CONNECTOR, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn isFlowVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { connectorType: Deref @ DAE::ConnectorType::FLOW, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isConst(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::CONST, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isParam(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_TGRID, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn makeParam(mut var: BackendDAE::Var) -> BackendDAE::Var {
    let mut var: BackendDAE::Var = var;
    var.varKind = crate::BackendDAE::VarKind::PARAM;
    var
}

pub fn makeParamOutputsOnly(mut var: BackendDAE::Var, mut fixed: bool) -> Result<(BackendDAE::Var, bool)> {
    let mut var: BackendDAE::Var = var;
    let mut fixed: bool = fixed;
    var.varKind = crate::BackendDAE::VarKind::PARAM;
    var = setHideResult(var.clone(), Some(Arc::new(DAE::Exp::BCONST { bool: true })));
    var.values = if (isSome(var.values.clone())) {var.values.clone()} else {Some(getVariableAttributefromType(var.varType.clone())?)};
    if isNone(DAEUtil::getFixedAttr(var.values.clone())) {
        var.values = DAEUtil::setFixedAttr(var.values.clone(), Some(Arc::new(DAE::Exp::BCONST { bool: fixed.clone() })))?;
    }
    Ok((var, fixed))
}

pub fn isParamOrConstant(mut invar: BackendDAE::Var) -> bool {
    let mut outbool: bool = false;
    outbool = isParam(invar.clone()) || isConst(invar.clone());
    outbool
}

pub fn isIntParam(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, varKind: BackendDAE::VarKind::PARAM, .. } => true,
        BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, varKind: BackendDAE::VarKind::PARAM, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isBoolParam(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, varKind: BackendDAE::VarKind::PARAM, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isStringParam(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, varKind: BackendDAE::VarKind::PARAM, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isExtObj(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::EXTOBJ { fullClassName: _ }, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isAlgState(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::ALG_STATE, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isRealParam(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, varKind: BackendDAE::VarKind::PARAM, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isRealOptimizeConstraintsVars(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_CONSTR, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isDAEmodeVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::DAE_RESIDUAL_VAR, .. } => true,
        BackendDAE::Var { varKind: BackendDAE::VarKind::DAE_AUX_VAR, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isDAEmodeResVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::DAE_RESIDUAL_VAR, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isDAEmodeAuxVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::DAE_AUX_VAR, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isRealOptimizeFinalConstraintsVars(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_FCONSTR, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isRealOptimizeDerInput(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_INPUT_DER, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isAlgebraicOldState(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::ALG_STATE_OLD, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isCSEVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        _ if (CommonSubExpression::isCSECref(inVar.varName.clone())) => true,
        _ => false,
    });
    outBoolean
}

pub fn isRESVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.varName.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, .. } => {
            StringUtil::startsWith((s.clone()).clone(), (literal!("$res")).clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: s, .. } => {
            StringUtil::startsWith((s.clone()).clone(), (literal!("$res")).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn hasMayerTermAnno(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(comm.clone(), (literal!("isMayer")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn hasOpenModelicaBoundaryConditionAnnotation(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(comm.clone(), (literal!("__OpenModelica_BoundaryCondition")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn hasLagrangeTermAnno(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(comm.clone(), (literal!("isLagrange")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn hasConTermAnno(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(comm.clone(), (literal!("isConstraint")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn hasFinalConTermAnno(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(comm.clone(), (literal!("isFinalConstraint")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn hasTimeGridAnno(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(comm.clone(), (literal!("isTimeGrid")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn isNonRealParam(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = !(isRealParam(inVar.clone()));
    outBoolean
}

pub fn isInput(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varDirection: DAE::VarDirection::INPUT, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isOutputVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVar.clone() {
        BackendDAE::Var { varDirection: DAE::VarDirection::OUTPUT, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isOutputAliasVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut s: ArcStr = arcstr::literal!("");
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varName: Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, .. }, .. } => if (StringUtil::startsWith((s.clone()).clone(), (literal!("$outputAlias")).clone())) {true} else {false},
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isRealVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isRealOutputVar(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, varDirection: DAE::VarDirection::OUTPUT, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isOutput(mut inCref: Arc<DAE::ComponentRef>, mut inVars: BackendDAE::Variables) -> Result<bool> {
    let mut outBool: bool = false;
    outBool = 'mc: {
        let __mc_input = inVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(getVar(inCref.clone(), inVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varDirection: DAE::VarDirection::OUTPUT, .. }, tail: _ }, _) => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBool)
}

pub fn isProtectedVar(mut v: BackendDAE::Var) -> bool {
    let mut hidden: bool = DAEUtil::getProtectedAttr(v.values.clone());
    hidden
}

pub fn isProtected(mut v: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(v.values.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { isProtected: Some(b), .. }) => b.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { isProtected: Some(b), .. }) => b.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { isProtected: Some(b), .. }) => b.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { isProtected: Some(b), .. }) => b.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { isProtected: Some(b), .. }) => b.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hasVarEvaluateAnnotationTrueOrFinal(mut inVar: BackendDAE::Var) -> bool {
    let mut select: bool = false;
    select = isFinalVar(inVar.clone()) || hasVarEvaluateAnnotationTrue(inVar.clone());
    select
}

pub fn hasVarEvaluateAnnotationTrueOrProtected(mut inVar: BackendDAE::Var) -> bool {
    let mut select: bool = false;
    select = isProtectedVar(inVar.clone()) || hasVarEvaluateAnnotationTrue(inVar.clone());
    select
}

pub fn hasVarEvaluateAnnotationTrueOrFinalOrProtected(mut inVar: BackendDAE::Var) -> bool {
    let mut select: bool = false;
    select = isFinalOrProtectedVar(inVar.clone()) || hasVarEvaluateAnnotationTrue(inVar.clone());
    select
}

pub fn hasVarEvaluateAnnotation(mut inVar: BackendDAE::Var) -> Result<bool> {
    let mut select: bool = false;
    select = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { comment: Some(Deref @ SCode::Comment { annotation_: Some(anno), .. }), .. } => {
            SCodeUtil::hasBooleanNamedAnnotation(anno.clone(), (literal!("Evaluate")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(select)
}

pub fn hasVarEvaluateAnnotationTrue(mut inVar: BackendDAE::Var) -> bool {
    let mut isTrue: bool = false;
    let mut ann: Arc<SCode::Annotation>;
    let mut val: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    match '__try0: {
        let BackendDAE::VAR { comment: Some(SCode::COMMENT { annotation_: Some(__pa1), .. }), .. } = (inVar.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        ann = __pa1.clone();
        let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(SCodeUtil::lookupAnnotationBinding(ann.clone(), (literal!("Evaluate")).clone()), '__try0)) {
            Some(__pa2) => __pa2.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        val = __pa2.clone();
        isTrue = stringEqual((Dump::printExpStr(val.clone()).unwrap()).clone(), (literal!("true")).clone());
        Ok::<_, anyhow::Error>((isTrue.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isTrue = __try0_o0;
        }
        Err(_) => {
            isTrue = false;
        }
    }
    isTrue
}

pub fn hasVarEvaluateAnnotationFalse(mut inVar: BackendDAE::Var) -> bool {
    let mut isFalse: bool = false;
    let mut ann: Arc<SCode::Annotation>;
    let mut val: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    match '__try0: {
        let BackendDAE::VAR { comment: Some(SCode::COMMENT { annotation_: Some(__pa1), .. }), .. } = (inVar.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        ann = __pa1.clone();
        let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(SCodeUtil::lookupAnnotationBinding(ann.clone(), (literal!("Evaluate")).clone()), '__try0)) {
            Some(__pa2) => __pa2.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        val = __pa2.clone();
        isFalse = stringEqual((Dump::printExpStr(val.clone()).unwrap()).clone(), (literal!("false")).clone());
        Ok::<_, anyhow::Error>((isFalse.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isFalse = __try0_o0;
        }
        Err(_) => {
            isFalse = false;
        }
    }
    isFalse
}

pub fn hasAnnotation(mut inVar: BackendDAE::Var) -> bool {
    let mut hasAnnot: bool = false;
    hasAnnot = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { comment: Some(Deref @ SCode::Comment { annotation_: Some(_), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasAnnot
}

pub fn getNamedAnnotation(mut inVar: BackendDAE::Var, mut inName: ArcStr) -> Result<Arc<Absyn::Exp>> {
    let mut outValue: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut ann: Arc<SCode::Annotation>;
    let BackendDAE::VAR { comment: Some(SCode::COMMENT { annotation_: Some(__pa0), .. }), .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    ann = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(SCodeUtil::lookupAnnotationBinding(ann.clone(), (inName.clone()).clone())?) {
        Some(__pa1) => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa1.clone();
    Ok(outValue)
}

pub fn getAnnotationComment(mut inVar: BackendDAE::Var) -> Result<Option<Arc<SCode::Comment>>> {
    let mut comment: Option<Arc<SCode::Comment>> = None;
    comment = (match inVar.clone() {
        BackendDAE::Var { comment: mut com, .. } => {
            com.clone()
        },
        _ => {
            bail!("fail")
        },
    });
    Ok(comment)
}

pub fn createpDerVar(mut inVar: BackendDAE::Var) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cr = varCref(inVar.clone())?;
    cr = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(BackendDAE::partialDerivativeNamePrefix)).clone(), DAE::T_REAL_DEFAULT.clone(), metamodelica::nil(), cr.clone());
    outVar = copyVarNewName(cr.clone(), inVar.clone());
    outVar = setVarKind(outVar.clone(), crate::BackendDAE::VarKind::JAC_TMP_VAR)?;
    Ok(outVar)
}

pub fn createClockedState(mut inVar: BackendDAE::Var) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cr = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::previousNamePrefix)).clone(), DAE::T_REAL_DEFAULT.clone(), metamodelica::nil(), inVar.varName.clone());
    outVar = copyVarNewName(cr.clone(), inVar.clone());
    outVar = setVarKind(outVar.clone(), crate::BackendDAE::VarKind::JAC_TMP_VAR)?;
    Ok(outVar)
}

pub fn createAliasDerVar(mut inCref: Arc<DAE::ComponentRef>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cr = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::derivativeNamePrefix)).clone(), inCref.clone())?;
    outVar = BackendDAE::Var { varName: cr.clone(), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource.clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    Ok(outVar)
}

pub fn createVar(mut inCref: Arc<DAE::ComponentRef>, mut prependStringCref: ArcStr) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cr = ComponentReference::appendStringLastIdent((prependStringCref.clone()).clone(), inCref.clone())?;
    outVar = makeVar(cr.clone());
    Ok(outVar)
}

pub fn createTmpVar(mut inCref: Arc<DAE::ComponentRef>, mut prependStringCref: ArcStr) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = createVar(inCref.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prependStringCref.clone()); __mm_s.push_str(&*intString(System::tmpTickIndex(Global::tmpVariableIndex.clone()))); ArcStr::from(__mm_s) }).clone())?;
    Ok(outVar)
}

pub fn createCSEVar(mut inCref: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = (match () {
        () if (ComponentReference::traverseCref(inCref.clone(), Arc::new(ComponentReference::crefIsRec), false)?) => {
            let mut source: Arc<DAE::ElementSource>;
            let mut path: Arc<Absyn::Path>;
            let mut varKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
            let __pa0 = ::match_deref::match_deref! { match &(inType.clone()) {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            path = __pa0.clone();
            source = Arc::new(DAE::ElementSource { info: Absyn::dummyInfo.clone(), partOfLst: metamodelica::nil(), instance: Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE), connectEquationOptLst: metamodelica::nil(), typeLst: list![path.clone()], operations: metamodelica::nil(), comment: metamodelica::nil() });
            varKind = if (Types::isDiscreteType(inType.clone())) {crate::BackendDAE::VarKind::DISCRETE} else {crate::BackendDAE::VarKind::VARIABLE};
            outVar = BackendDAE::Var { varName: inCref.clone(), varKind: varKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: inType.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: source.clone(), values: DAEUtil::setProtectedAttr(None, true)?, tearingSelectOption: Some(crate::BackendDAE::TearingSelect::NEVER), hideResult: Some(Arc::new(DAE::Exp::BCONST { bool: true })), comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
            outVar.clone()
        },
        _ => {
            let mut varKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
            varKind = if (Types::isDiscreteType(inType.clone())) {crate::BackendDAE::VarKind::DISCRETE} else {crate::BackendDAE::VarKind::VARIABLE};
            outVar = BackendDAE::Var { varName: inCref.clone(), varKind: varKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: inType.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource.clone(), values: DAEUtil::setProtectedAttr(None, true)?, tearingSelectOption: Some(crate::BackendDAE::TearingSelect::NEVER), hideResult: Some(Arc::new(DAE::Exp::BCONST { bool: true })), comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
            outVar.clone()
        },
    });
    Ok(outVar)
}

pub fn generateVar(mut cr: Arc<DAE::ComponentRef>, mut varKind: BackendDAE::VarKind, mut varType: Arc<DAE::Type>, mut subs: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut attr: Option<Arc<DAE::VariableAttributes>>) -> BackendDAE::Var {
    let mut var: BackendDAE::Var;
    var = BackendDAE::Var { varName: cr.clone(), varKind: varKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: varType.clone(), bindExp: None, tplExp: None, arryDim: subs.clone(), source: DAE::emptyElementSource.clone(), values: attr.clone(), tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    var
}

pub fn generateArrayVar(mut name: Arc<DAE::ComponentRef>, mut varKind: BackendDAE::VarKind, mut varType: Arc<DAE::Type>, mut attr: Option<Arc<DAE::VariableAttributes>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVars = (::match_deref::match_deref! { match &(varType.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims, ty: tp } => {
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            crlst = ComponentReference::expandCref(name.clone(), false)?;
            vars = List::map4(crlst.clone(), Arc::new(fnptr!(generateVar, Arc<DAE::ComponentRef>, BackendDAE::VarKind, Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Dimension>>>, Option<Arc<DAE::VariableAttributes>>)), varKind.clone(), tp.clone(), dims.clone(), None);
            vars.clone()
        },
        _ => {
            let mut var: BackendDAE::Var;
            var = BackendDAE::Var { varName: name.clone(), varKind: varKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: varType.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource.clone(), values: attr.clone(), tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            list![var.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVars)
}

pub fn createCSEArrayVar(mut inCref: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inArryDim: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = (::match_deref::match_deref! { match &(inCref.clone()) {
        _ if (ComponentReference::traverseCref(inCref.clone(), Arc::new(ComponentReference::crefIsRec), false)?) => {
            let mut source: Arc<DAE::ElementSource>;
            let mut path: Arc<Absyn::Path>;
            let mut varKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
            let __pa0 = ::match_deref::match_deref! { match &(inType.clone()) {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            path = __pa0.clone();
            source = Arc::new(DAE::ElementSource { info: Absyn::dummyInfo.clone(), partOfLst: metamodelica::nil(), instance: Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE), connectEquationOptLst: metamodelica::nil(), typeLst: list![path.clone()], operations: metamodelica::nil(), comment: metamodelica::nil() });
            varKind = if (Types::isDiscreteType(inType.clone())) {crate::BackendDAE::VarKind::DISCRETE} else {crate::BackendDAE::VarKind::VARIABLE};
            outVar = BackendDAE::Var { varName: inCref.clone(), varKind: varKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: inType.clone(), bindExp: None, tplExp: None, arryDim: inArryDim.clone(), source: source.clone(), values: DAEUtil::setProtectedAttr(None, true)?, tearingSelectOption: Some(crate::BackendDAE::TearingSelect::NEVER), hideResult: Some(Arc::new(DAE::Exp::BCONST { bool: true })), comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
            outVar.clone()
        },
        _ => {
            let mut varKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
            varKind = if (Types::isDiscreteType(inType.clone())) {crate::BackendDAE::VarKind::DISCRETE} else {crate::BackendDAE::VarKind::VARIABLE};
            outVar = BackendDAE::Var { varName: inCref.clone(), varKind: varKind.clone(), varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: inType.clone(), bindExp: None, tplExp: None, arryDim: inArryDim.clone(), source: DAE::emptyElementSource.clone(), values: DAEUtil::setProtectedAttr(None, true)?, tearingSelectOption: Some(crate::BackendDAE::TearingSelect::NEVER), hideResult: Some(Arc::new(DAE::Exp::BCONST { bool: true })), comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
            outVar.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVar)
}

pub fn copyVarNewName(mut cr: Arc<DAE::ComponentRef>, mut inVar: BackendDAE::Var) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    outVar.varName = cr.clone();
    outVar
}

pub fn setVarKindForVar(mut idx: i32, mut kind: BackendDAE::VarKind, mut varsIn: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut varsOut: BackendDAE::Variables;
    let mut var: BackendDAE::Var;
    var = getVarAt(varsIn.clone(), idx.clone())?;
    var = setVarKind(var.clone(), kind.clone())?;
    varsOut = setVarAt(varsIn.clone(), idx.clone(), var.clone())?;
    Ok(varsOut)
}

pub fn setVarsKind(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inVarKind: BackendDAE::VarKind) -> Arc<metamodelica::List<BackendDAE::Var>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVars = List::map1(inVars.clone(), Arc::new(setVarKind), inVarKind.clone());
    outVars
}

pub fn setVarKind(mut inVar: BackendDAE::Var, mut inVarKind: BackendDAE::VarKind) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    outVar.varKind = inVarKind.clone();
    if isDummyStateVar(outVar.clone()) && varStateSelectAlways(outVar.clone()) {
        Error::addMessage(Error::NON_STATE_STATESELECT_ALWAYS.clone(), list![(ComponentReference::crefStr(varCref(outVar.clone())?)?).clone()])?;
    }
    Ok(outVar)
}

pub fn setVarTS(mut inVar: BackendDAE::Var, mut inTS: Option<BackendDAE::TearingSelect>) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    outVar.tearingSelectOption = inTS.clone();
    outVar
}

pub fn setBindExp(mut inVar: BackendDAE::Var, mut inBindExp: Option<Arc<DAE::Exp>>) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    outVar.bindExp = inBindExp.clone();
    outVar
}

pub fn setHideResult(mut varIn: BackendDAE::Var, mut hideResultB: Option<Arc<DAE::Exp>>) -> BackendDAE::Var {
    let mut varOut: BackendDAE::Var = varIn.clone();
    varOut.hideResult = hideResultB.clone();
    varOut
}

pub fn setVarDirectionTpl(mut var: BackendDAE::Var, mut dir: DAE::VarDirection) -> (BackendDAE::Var, DAE::VarDirection) {
    let mut var: BackendDAE::Var = var;
    let mut dir: DAE::VarDirection = dir;
    var.varDirection = dir.clone();
    (var, dir)
}

pub fn setVarDirection(mut inVar: BackendDAE::Var, mut inVarDirection: DAE::VarDirection) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    outVar.varDirection = inVarDirection.clone();
    outVar
}

pub fn getVarDirection(mut inVar: BackendDAE::Var) -> DAE::VarDirection {
    let mut varDirection: DAE::VarDirection = inVar.varDirection.clone();
    varDirection
}

pub fn getVarNominalValue(mut InVar: BackendDAE::Var) -> Arc<DAE::Exp> {
    let mut nom: Arc<DAE::Exp> = DAEUtil::getNominalAttr(InVar.values.clone());
    nom
}

pub fn getVarKind(mut inVar: BackendDAE::Var) -> BackendDAE::VarKind {
    let mut varKind: BackendDAE::VarKind = inVar.varKind.clone();
    varKind
}

pub fn getVarKindForVar(mut idx: i32, mut varsIn: BackendDAE::Variables) -> Result<BackendDAE::VarKind> {
    let mut kind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
    let mut var: BackendDAE::Var;
    var = getVarAt(varsIn.clone(), idx.clone())?;
    kind = getVarKind(var.clone());
    Ok(kind)
}

pub fn isVarOnTopLevelAndOutput(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = isOutputVar(inVar.clone());
    outBoolean
}

pub fn isVarOnTopLevelAndInput(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = isInput(inVar.clone());
    outBoolean
}

pub fn isVarOnTopLevelAndInputNoDerInput(mut inVar: BackendDAE::Var) -> bool {
    let mut outBoolean: bool = isVarOnTopLevelAndInput(inVar.clone()) && !(isRealOptimizeDerInput(inVar.clone()));
    outBoolean
}

pub fn isFinalVar(mut inVar: BackendDAE::Var) -> bool {
    let mut b: bool = DAEUtil::getFinalAttr(inVar.values.clone());
    b
}

pub fn isFinalOrProtectedVar(mut inVar: BackendDAE::Var) -> bool {
    let mut b: bool = isFinalVar(inVar.clone()) || isProtectedVar(inVar.clone());
    b
}

pub fn isChangeable(mut v: BackendDAE::Var) -> bool {
    let mut isValueChangeable: bool = isVarOnTopLevelAndInput(v.clone()) || varFixed(v.clone()) && !(hasVarEvaluateAnnotationTrueOrFinalOrProtected(v.clone())) && if (isParam(v.clone())) {varHasConstantBindExp(v.clone()).unwrap() || !(varHasBindExp(v.clone())) && varHasConstantStartExp(v.clone())} else {varHasConstantStartExp(v.clone())};
    isValueChangeable
}

pub fn getVariableAttributes(mut inVar: BackendDAE::Var) -> Option<Arc<DAE::VariableAttributes>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = inVar.values.clone();
    outAttr
}

pub fn getVarSource(mut inVar: BackendDAE::Var) -> Arc<DAE::ElementSource> {
    let mut outSource: Arc<DAE::ElementSource> = inVar.source.clone();
    outSource
}

pub fn getVarType(mut inVar: BackendDAE::Var) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = inVar.varType.clone();
    outType
}

pub fn getMinMaxAsserts(mut inVar: BackendDAE::Var, mut inAsserts: Arc<metamodelica::List<Arc<DAE::Algorithm>>>) -> Result<(BackendDAE::Var, Arc<metamodelica::List<Arc<DAE::Algorithm>>>)> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outAsserts: Arc<metamodelica::List<Arc<DAE::Algorithm>>> = metamodelica::nil();
    outAsserts = 'mc: {
        let __mc_input = inVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Var { varKind: BackendDAE::VarKind::CONST, .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(inAsserts.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Var { source: mut source, varType: mut varType, values: mut attr, varName: ref name, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut e: Arc<DAE::Exp>;
            let mut cond: Arc<DAE::Exp>;
            let mut msg: Arc<DAE::Exp>;
            let mut level: Arc<DAE::Exp>;
            let mut min: Option<Arc<DAE::Exp>> = None;
            let mut max: Option<Arc<DAE::Exp>> = None;
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut format: ArcStr = arcstr::literal!("");
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            (min, max) = DAEUtil::getMinMaxValues(attr.clone());
            if isNone(min.clone()) && isNone(max.clone()) {
                bail!("fail");
            }
            e = Expression::crefExp(name.clone())?;
            tp = BackendDAEUtil::makeExpType(varType.clone());
            cond = getMinMaxAsserts1(min.clone(), max.clone(), e.clone(), tp.clone())?;
            (cond, _) = ExpressionSimplify::simplify(cond.clone())?;
            let false = (Expression::isConstTrue(cond.clone())) else { bail!("pattern mismatch") };
            r#str = (getMinMaxAsserts1Str(min.clone(), max.clone(), (ComponentReferenceBasics::printComponentRefStr(name.clone())?).clone())?).clone();
            if Flags::isSet(Flags::WARNING_MINMAX_ATTRIBUTES.clone())? {
                level = DAE::ASSERTIONLEVEL_WARNING.clone();
            } else {
                level = DAE::ASSERTIONLEVEL_ERROR.clone();
            }
            format = (if (Types::isRealOrSubTypeReal(tp.clone())?) {literal!("g")} else {literal!("d")}).clone();
            msg = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() }), operator: DAE::Operator::ADD { ty: DAE::T_STRING_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("String")).clone() }), expLst: list![e.clone(), Arc::new(DAE::Exp::SCONST { string: (format.clone()).clone() })], attr: DAE::callAttrBuiltinString.clone() }) });
            BackendDAEUtil::checkAssertCondition(cond.clone(), msg.clone(), level.clone(), ElementSource::getElementSourceFileInfo(source.clone()))?;
            Ok(cons(Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_ASSERT { cond: cond.clone(), msg: msg.clone(), level: level.clone(), source: source.clone() })] }), inAsserts.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inAsserts.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outAsserts))
}

fn getMinMaxAsserts1(mut omin: Option<Arc<DAE::Exp>>, mut omax: Option<Arc<DAE::Exp>>, mut e: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut cond: Arc<DAE::Exp>;
    cond = (::match_deref::match_deref! { match &((omin.clone(), omax.clone())) {
        (Some(min), Some(max)) => {
            Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: min.clone(), index: -1, optionExpisASUB: None }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: max.clone(), index: -1, optionExpisASUB: None }) })
        },
        (Some(min), None) => {
            Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: min.clone(), index: -1, optionExpisASUB: None })
        },
        (None, Some(max)) => {
            Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::LESSEQ { ty: tp.clone() }, exp2: max.clone(), index: -1, optionExpisASUB: None })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cond)
}

fn getMinMaxAsserts1Str(mut omin: Option<Arc<DAE::Exp>>, mut omax: Option<Arc<DAE::Exp>>, mut varStr: ArcStr) -> Result<ArcStr> {
    let mut msg: ArcStr = arcstr::literal!("");
    msg = ((::match_deref::match_deref! { match &((omin.clone(), omax.clone())) {
        (Some(min), Some(max)) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable violating min/max constraint: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(min.clone())?); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*varStr.clone()); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(max.clone())?); __mm_s.push_str(&*literal!(", has value: ")); ArcStr::from(__mm_s) }
        },
        (Some(min), None) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable violating min constraint: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(min.clone())?); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*varStr.clone()); __mm_s.push_str(&*literal!(", has value: ")); ArcStr::from(__mm_s) }
        },
        (None, Some(max)) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable violating max constraint: ")); __mm_s.push_str(&*varStr.clone()); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(max.clone())?); __mm_s.push_str(&*literal!(", has value: ")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(msg)
}

pub fn varSortFunc(mut v1: BackendDAE::Var, mut v2: BackendDAE::Var) -> Result<bool> {
    let mut greaterThan: bool = false;
    greaterThan = ComponentReferenceBasics::crefSortFunc(varCref(v1.clone())?, varCref(v2.clone())?)?;
    Ok(greaterThan)
}

pub fn sortInitialVars(mut vars: BackendDAE::Variables, mut fixableVars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut vars: BackendDAE::Variables = vars;
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut fixable_start: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut fixable: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut non_fixable: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    var_lst = varList(vars.clone())?;
    (fixable, non_fixable) = List::splitOnTrue(var_lst.clone(), Arc::new({ let __pe_b1 = fixableVars.clone(); move |__pe_a0| Ok(containsVar(__pe_a0, __pe_b1.clone())) }));
    (fixable_start, fixable) = List::splitOnTrue(fixable.clone(), Arc::new(varHasStartValue));
    var_lst = listAppend(listAppend(fixable_start.clone(), fixable.clone().reverse()), non_fixable.clone().reverse());
    vars = listVar(var_lst.clone());
    Ok(vars)
}

pub fn getAlias(mut inVar: BackendDAE::Var) -> Result<(Arc<DAE::ComponentRef>, bool)> {
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut negated: bool = false;
    let mut e: Arc<DAE::Exp>;
    e = varBindExp(inVar.clone())?;
    (outCr, negated) = getAlias1(e.clone())?;
    Ok((outCr, negated))
}

fn getAlias1(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::ComponentRef>, bool)> {
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut negated: bool = false;
    (outCr, negated) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: name, .. } => {
            (name.clone(), false)
        },
        Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: name, .. }, operator: DAE::Operator::UMINUS { ty: _ } } => {
            (name.clone(), true)
        },
        Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: name, .. }, operator: DAE::Operator::UMINUS_ARR { ty: _ } } => {
            (name.clone(), true)
        },
        Deref @ DAE::Exp::LUNARY { exp: Deref @ DAE::Exp::CREF { componentRef: name, .. }, operator: DAE::Operator::NOT { ty: _ } } => {
            (name.clone(), true)
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: name, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
            let mut name = (*name).clone();
            name = ComponentReference::crefPrefixDer(name.clone());
            (name.clone(), false)
        },
        Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: name, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, operator: DAE::Operator::UMINUS { ty: _ } } => {
            let mut name = (*name).clone();
            name = ComponentReference::crefPrefixDer(name.clone());
            (name.clone(), true)
        },
        Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: name, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, operator: DAE::Operator::UMINUS_ARR { ty: _ } } => {
            let mut name = (*name).clone();
            name = ComponentReference::crefPrefixDer(name.clone());
            (name.clone(), true)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCr, negated))
}

pub fn daenumVariables(mut syst: Arc<BackendDAE::EqSystem>) -> Result<i32> {
    let mut n: i32 = 0;
    let mut vars: BackendDAE::Variables;
    vars = daeVars(syst.clone());
    n = varsSize(vars.clone())?;
    Ok(n)
}

/* =======================================================
 *
 *  Section for functions that deals with VariablesArray
 *
 * =======================================================
 */
fn copyArray(mut inVariableArray: BackendDAE::VariableArray) -> BackendDAE::VariableArray {
    let mut outVariableArray: BackendDAE::VariableArray = inVariableArray.clone();
    outVariableArray.varOptArr = metamodelica::arrayFromVec(inVariableArray.varOptArr.clone().borrow().clone());
    outVariableArray
}

fn vararrayEmpty(mut inSize: i32) -> BackendDAE::VariableArray {
    let mut outArray: BackendDAE::VariableArray;
    let mut arr: metamodelica::Array<Option<BackendDAE::Var>>;
    arr = arrayCreate(inSize.clone(), None);
    outArray = BackendDAE::VariableArray { numberOfElements: 0, varOptArr: arr.clone() };
    outArray
}

fn vararrayAdd(mut inVariableArray: BackendDAE::VariableArray, mut inVar: BackendDAE::Var) -> Result<BackendDAE::VariableArray> {
    let mut outVariableArray: BackendDAE::VariableArray;
    let mut num_elems: i32 = 0;
    let mut arr: metamodelica::Array<Option<BackendDAE::Var>>;
    let BackendDAE::VARIABLE_ARRAY { numberOfElements: __pa0, varOptArr: __pa1 } = (inVariableArray.clone()) else { bail!("pattern mismatch") };
    num_elems = __pa0.clone();
    arr = __pa1.clone();
    num_elems = num_elems.clone() + 1;
    arr = Array::expandOnDemand(num_elems.clone(), arr.clone(), metamodelica::OrderedFloat(1.4_f64), None)?;
    {let _arr = arr.clone(); _arr.borrow_mut()[(num_elems.clone()-1) as usize] = Some(inVar.clone()); _arr};
    outVariableArray = BackendDAE::VariableArray { numberOfElements: num_elems.clone(), varOptArr: arr.clone() };
    Ok(outVariableArray)
}

fn vararraySetnth(mut inVariableArray: BackendDAE::VariableArray, mut inIndex: i32, mut inVar: BackendDAE::Var) -> Result<BackendDAE::VariableArray> {
    let mut outVariableArray: BackendDAE::VariableArray = inVariableArray.clone();
    let true = (inIndex.clone() <= inVariableArray.numberOfElements.clone()) else { bail!("pattern mismatch") };
    {let _arr = inVariableArray.varOptArr.clone(); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = Some(inVar.clone()); _arr};
    Ok(outVariableArray)
}

fn vararrayNth(mut inVariableArray: BackendDAE::VariableArray, mut inIndex: i32) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    let true = (inIndex.clone() <= inVariableArray.numberOfElements.clone()) else { bail!("pattern mismatch") };
    let Some(__pa0) = (inVariableArray.varOptArr.clone().borrow()[(inIndex.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
    outVar = __pa0.clone();
    Ok(outVar)
}

fn vararrayDelete(mut inVariableArray: BackendDAE::VariableArray, mut inIndex: i32) -> Result<(BackendDAE::VariableArray, BackendDAE::Var)> {
    let mut outVariableArray: BackendDAE::VariableArray = inVariableArray.clone();
    let mut outVar: BackendDAE::Var;
    let Some(__pa0) = (outVariableArray.varOptArr.clone().borrow()[(inIndex.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
    outVar = __pa0.clone();
    {let _arr = outVariableArray.varOptArr.clone(); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = None; _arr};
    Ok((outVariableArray, outVar))
}

fn vararrayList(mut inArray: BackendDAE::VariableArray) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varOptArr: metamodelica::Array<Option<BackendDAE::Var>>;
    let BackendDAE::VARIABLE_ARRAY { varOptArr: __pa0, .. } = (inArray.clone()) else { bail!("pattern mismatch") };
    varOptArr = __pa0.clone();
    outVars = metamodelica::nil();
    let __range1 = (1..=(varOptArr.clone().borrow().len() as i32)).rev();
    for mut i in __range1 {
        if isSome(varOptArr.borrow()[(i.clone()-1) as usize].clone()) {
            outVars = cons(Util::getOption(varOptArr.borrow()[(i.clone()-1) as usize].clone())?, outVars.clone());
        }
    }
    Ok(outVars)
}

/* =======================================================
 *
 *  Section for functions that deals with Variables
 *
 * =======================================================
 */
pub fn copyVariables(mut inVariables: BackendDAE::Variables) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = inVariables.clone();
    outVariables.crefIndices = metamodelica::arrayFromVec(inVariables.crefIndices.clone().borrow().clone());
    outVariables.varArr = copyArray(inVariables.varArr.clone());
    outVariables
}

pub fn emptyVars(mut inSize: i32) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    let mut indices: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>;
    let mut buckets: i32 = 0;
    let mut arr_size: i32 = 0;
    let mut arr: BackendDAE::VariableArray;
    arr_size = std::cmp::max(inSize.clone(), BaseHashTable::lowBucketSize.clone());
    buckets = ((intReal(arr_size.clone()) * metamodelica::OrderedFloat(1.4_f64)).0 as i32);
    indices = arrayCreate(buckets.clone(), metamodelica::nil());
    arr = vararrayEmpty(arr_size.clone());
    outVariables = BackendDAE::Variables { crefIndices: indices.clone(), varArr: arr.clone(), bucketSize: buckets.clone(), numberOfVars: 0 };
    outVariables
}

pub fn emptyVarsSized(mut size: i32) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables = emptyVars(size.clone());
    outVariables
}

pub fn isCrefInVarList(mut inCref: Arc<DAE::ComponentRef>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<bool> {
    let mut isInList: bool = false;
    for mut v in &*inVars.clone() {
        let mut v = v.clone();
        if ComponentReferenceBasics::crefEqual(varCref(v.clone())?, inCref.clone())? {
            isInList = true;
            return Ok(isInList);
        }
    }
    Ok(isInList)
}

pub fn areAllCrefsInVarList(mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<bool> {
    let mut isInList: bool = true;
    for mut cref in &*inCrefs.clone() {
        let mut cref = cref.clone();
        if !(isCrefInVarList(cref.clone(), inVars.clone())?) {
            isInList = false;
            return Ok(isInList);
        }
    }
    Ok(isInList)
}

pub fn areAllCrefsPrimaryParameters(mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inVars: BackendDAE::Variables) -> bool {
    let mut isPrimary: bool = true;
    let mut v: BackendDAE::Var;
    for mut cref in &*inCrefs.clone() {
        let mut cref = cref.clone();
        match '__try0: {
            (v, _) = unwrap_break_err!(getVar2(cref.clone(), inVars.clone()), '__try0);
            let true = (isParam(v.clone()) && varFixed(v.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            Ok::<_, anyhow::Error>((v.clone(),))
        } {
            Ok((__try0_o0,)) => {
                v = __try0_o0;
            }
            Err(_) => {
                isPrimary = false;
                return isPrimary;
            }
        }
    }
    isPrimary
}

pub fn varList(mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = vararrayList(inVariables.varArr.clone())?;
    Ok(outVarLst)
}

pub fn listVar(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    let mut size: i32 = 0;
    size = (inVarLst.clone().len() as i32);
    outVariables = emptyVarsSized(size.clone());
    outVariables = addVars(inVarLst.clone().reverse(), outVariables.clone());
    outVariables
}

pub fn listVarSized(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut size: i32) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold(inVarLst.clone(), Arc::new(addVar), emptyVarsSized(size.clone()));
    outVariables
}

pub fn listVar1(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    let mut size: i32 = 0;
    size = (inVarLst.clone().len() as i32);
    outVariables = List::fold(inVarLst.clone(), Arc::new(addVar), emptyVarsSized(size.clone()));
    outVariables
}

pub fn listVar2(mut inVarLst1: Arc<metamodelica::List<BackendDAE::Var>>, mut inVarLst2: Arc<metamodelica::List<BackendDAE::Var>>) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    let mut size: i32 = 0;
    size = (inVarLst1.clone().len() as i32) + (inVarLst2.clone().len() as i32);
    outVariables = List::fold(inVarLst2.clone(), Arc::new(addVar), List::fold(inVarLst1.clone(), Arc::new(addVar), emptyVarsSized(size.clone())));
    outVariables
}

pub fn equationSystemsVarsLst(mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut v: BackendDAE::Variables;
    for mut es in &*systs.clone() {
        let mut es = es.clone();
        let __pa0 = ::match_deref::match_deref! { match &(es.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        v = __pa0.clone();
        vars = varList(v.clone())?;
        outVars = List::append_reverse(vars.clone(), outVars.clone());
    }
    outVars = metamodelica::Dangerous::listReverseInPlace(outVars.clone());
    Ok(outVars)
}

pub fn daeVars(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> BackendDAE::Variables {
    let mut vars: BackendDAE::Variables = inEqSystem.orderedVars.clone();
    vars
}

pub fn daeGlobalKnownVars(mut inShared: Arc<BackendDAE::Shared>) -> BackendDAE::Variables {
    let mut outGlobalKnownVars: BackendDAE::Variables = inShared.globalKnownVars.clone();
    outGlobalKnownVars
}

pub fn daeAliasVars(mut inShared: Arc<BackendDAE::Shared>) -> BackendDAE::Variables {
    let mut outAliasVars: BackendDAE::Variables = inShared.aliasVars.clone();
    outAliasVars
}

pub fn varsSize(mut inVariables: BackendDAE::Variables) -> Result<i32> {
    let mut outNumVariables: i32 = 0;
    let BackendDAE::VARIABLES { varArr: BackendDAE::VARIABLE_ARRAY { numberOfElements: __pa0, .. }, .. } = (inVariables.clone()) else { bail!("pattern mismatch") };
    outNumVariables = __pa0.clone();
    Ok(outNumVariables)
}

/*
public function varDim
  "Returns the dimension of variables in the Variables structure.
  NOTE: function fail if dimension is not constant
  "
  input BackendDAE.Var inVar;
  output Integer outDimVariables = 1;
protected
  DAE.Dimensions dims;
  Integer n;
algorithm
  BackendDAE.VAR(arryDim=dims) := inVar;
  for dim in dims loop
    DAE.DIM_INTEGER(n) := dim;
    outDimVariables := n * outDimVariables;
  end for;
end varDim;
*/
fn varsLoadFactor(mut inVariables: BackendDAE::Variables, mut inIncrease: i32) -> metamodelica::Real {
    let mut outLoadFactor: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    outLoadFactor = intReal(inVariables.numberOfVars.clone() + inIncrease.clone()) / intReal(inVariables.bucketSize.clone());
    outLoadFactor
}

pub fn isVariable(mut inComponentRef1: Arc<DAE::ComponentRef>, mut inVariables2: BackendDAE::Variables, mut inVariables3: BackendDAE::Variables) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inComponentRef1.clone(), inVariables2.clone(), inVariables3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, vars, _) => {
                    let mut kind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
                    let __pa0 = ::match_deref::match_deref! { match &(getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: __pa0, .. }, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    kind = __pa0.clone();
                    isVarKindVariable(kind.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, _, globalKnownVars) => {
                    let mut kind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
                    let __pa0 = ::match_deref::match_deref! { match &(getVar(cr.clone(), globalKnownVars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: __pa0, .. }, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    kind = __pa0.clone();
                    isVarKindVariable(kind.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn isVarKindVariable(mut inVarKind: BackendDAE::VarKind) -> Result<()> {
    let _ = (match inVarKind.clone() {
        BackendDAE::VarKind::VARIABLE => (),
        BackendDAE::VarKind::STATE { .. } => (),
        BackendDAE::VarKind::DUMMY_STATE => (),
        BackendDAE::VarKind::DUMMY_DER => (),
        BackendDAE::VarKind::DISCRETE => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn isVarKindState(mut inVarKind: BackendDAE::VarKind) -> bool {
    let mut result: bool = false;
    result = (match inVarKind.clone() {
        BackendDAE::VarKind::STATE { .. } => true,
        _ => false,
    });
    result
}

pub fn isTopLevelInputOrOutput(mut inComponentRef: Arc<DAE::ComponentRef>, mut inVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = inComponentRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: BackendDAE::Var;
                    let __pa0 = ::match_deref::match_deref! { match &(getVar(inComponentRef.clone(), inVars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    Ok(isVarOnTopLevelAndOutput(v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: BackendDAE::Var;
                    let __pa0 = ::match_deref::match_deref! { match &(getVar(inComponentRef.clone(), inGlobalKnownVars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    Ok(isVarOnTopLevelAndInput(v.clone()))
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
    Ok(outBoolean)
}

pub fn deleteCrefs(mut varlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut vars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut vars_1: BackendDAE::Variables;
    vars_1 = List::fold(varlst.clone(), Arc::new(removeCref), vars.clone());
    vars_1 = listVar1(varList(vars_1.clone())?);
    Ok(vars_1)
}

pub fn deleteVars(mut inDelVars: BackendDAE::Variables, mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = inVariables.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut newvars: BackendDAE::Variables;
            let true = (intGt(varsSize(inDelVars.clone())?, 0)) else { bail!("pattern mismatch") };
            newvars = traverseBackendDAEVars(inDelVars.clone(), Arc::new(deleteVars1), inVariables.clone())?;
            newvars = listVar1(varList(newvars.clone())?);
            Ok(newvars.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inVariables.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVariables)
}

fn deleteVars1(mut inVar: BackendDAE::Var, mut inVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outVars: BackendDAE::Variables;
    outVars = removeCref(inVar.varName.clone(), inVars.clone())?;
    Ok((outVar, outVars))
}

pub fn deleteVar(mut inComponentRef: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut vars: BackendDAE::Variables;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (cr, _) = (inComponentRef.clone(), inVariables.clone());
    (_, ilst) = getVar(cr.clone(), inVariables.clone())?;
    (vars, _) = removeVars(ilst.clone(), inVariables.clone(), metamodelica::nil())?;
    vars = listVar1(varList(vars.clone())?);
    outVariables = vars.clone();
    Ok(outVariables)
}

pub fn deleteVarIfExistsAndReturn(mut inComponentRef: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> (Arc<metamodelica::List<BackendDAE::Var>>, BackendDAE::Variables) {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outVariables: BackendDAE::Variables = inVariables.clone();
    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if '__try0: {
        (outVarLst, ilst) = unwrap_break_err!(getVar(inComponentRef.clone(), inVariables.clone()), '__try0);
        (outVariables, _) = unwrap_break_err!(removeVars(ilst.clone(), inVariables.clone(), metamodelica::nil()), '__try0);
        outVariables = listVar1(varList(outVariables.clone()).unwrap());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    (outVarLst, outVariables)
}

pub fn removeCrefs(mut varlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut vars: BackendDAE::Variables) -> BackendDAE::Variables {
    let mut vars_1: BackendDAE::Variables;
    vars_1 = List::fold(varlst.clone(), Arc::new(removeCref), vars.clone());
    vars_1
}

pub fn removeCref(mut inComponentRef: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = inComponentRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cr => {
                    let mut vars: BackendDAE::Variables;
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (_, ilst) = getVar(cr.clone(), inVariables.clone())?;
                    (vars, _) = removeVars(ilst.clone(), inVariables.clone(), metamodelica::nil())?;
                    Ok(vars.clone())
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

pub fn removeVars(mut inVarPos: Arc<metamodelica::List<i32>>, mut inVariables: BackendDAE::Variables, mut iAcc: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outVariables: BackendDAE::Variables;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    (outVariables, outVars) = 'mc: {
        let __mc_input = inVarPos.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inVariables.clone(), iAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: i, tail: ilst } => {
                    let mut vars: BackendDAE::Variables;
                    let mut v: BackendDAE::Var;
                    let mut acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (vars, v) = removeVar(i.clone(), inVariables.clone())?;
                    (vars, acc) = removeVars(ilst.clone(), vars.clone(), cons(v.clone(), iAcc.clone()))?;
                    Ok((vars.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: ilst } => {
                    let mut vars: BackendDAE::Variables;
                    let mut acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (vars, acc) = removeVars(ilst.clone(), inVariables.clone(), iAcc.clone())?;
                    Ok((vars.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVariables, outVars))
}

pub fn removeVarDAE(mut inVarPos: i32, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<(Arc<BackendDAE::EqSystem>, BackendDAE::Var)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    let mut outVar: BackendDAE::Var;
    let mut vars: BackendDAE::Variables;
    (vars, outVar) = removeVar(inVarPos.clone(), inEqSystem.orderedVars.clone())?;
    outEqSystem = BackendDAEUtil::setEqSystVars(inEqSystem.clone(), vars.clone())?;
    Ok((outEqSystem, outVar))
}

pub fn removeAliasVars(mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedAliasVars(inShared.clone(), emptyVars(BaseHashTable::bigBucketSize.clone()))?;
    Ok(outShared)
}

pub fn removeVar(mut inIndex: i32, mut inVariables: BackendDAE::Variables) -> Result<(BackendDAE::Variables, BackendDAE::Var)> {
    let mut outVariables: BackendDAE::Variables;
    let mut outVar: BackendDAE::Var;
    let mut indices: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>;
    let mut cr_indices: Arc<metamodelica::List<BackendDAE::CrefIndex>> = metamodelica::nil();
    let mut arr: BackendDAE::VariableArray;
    let mut buckets: i32 = 0;
    let mut num_vars: i32 = 0;
    let mut hash_idx: i32 = 0;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let BackendDAE::VARIABLES { crefIndices: __pa0, varArr: __pa1, bucketSize: __pa2, numberOfVars: __pa3 } = (inVariables.clone()) else { bail!("pattern mismatch") };
    indices = __pa0.clone();
    arr = __pa1.clone();
    buckets = __pa2.clone();
    num_vars = __pa3.clone();
    let (__pa4, ref __pa6 @ BackendDAE::VAR { varName: ref __pa5, .. }) = (vararrayDelete(arr.clone(), inIndex.clone())?) else { bail!("pattern mismatch") };
    arr = __pa4.clone();
    cr = __pa5.clone();
    outVar = __pa6.clone();
    hash_idx = intMod(ComponentReference::hashComponentRef(cr.clone())?, buckets.clone()) + 1;
    cr_indices = indices.borrow()[(hash_idx.clone()-1) as usize].clone();
    (cr_indices, _) = List::deleteMemberOnTrue(BackendDAE::CrefIndex { cref: cr.clone(), index: inIndex.clone() - 1 }, cr_indices.clone(), Arc::new(fnptr!(removeVar2, BackendDAE::CrefIndex, BackendDAE::CrefIndex)))?;
    {let _arr = indices.clone(); _arr.borrow_mut()[(hash_idx.clone()-1) as usize] = cr_indices.clone(); _arr};
    outVariables = BackendDAE::Variables { crefIndices: indices.clone(), varArr: arr.clone(), bucketSize: buckets.clone(), numberOfVars: num_vars.clone() - 1 };
    Ok((outVariables, outVar))
}

fn removeVar2(mut inCrefIndex1: BackendDAE::CrefIndex, mut inCrefIndex2: BackendDAE::CrefIndex) -> bool {
    let mut outMatch: bool = false;
    outMatch = inCrefIndex1.index.clone() == inCrefIndex2.index.clone();
    outMatch
}

pub fn isKnownAndParam(mut inExp: Arc<DAE::Exp>, mut knownVars: BackendDAE::Variables) -> Result<bool> {
    let mut outBoolean: bool = false;
    let mut tpl: (bool, BackendDAE::Variables) = (true, knownVars.clone());
    let (_, (__pa0, _)) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(fnptr!(isKnownAndParamWork, Arc<DAE::Exp>, (bool, BackendDAE::Variables))), tpl.clone())?;
    outBoolean = __pa0.clone();
    Ok(outBoolean)
}

fn isKnownAndParamWork(mut inExp: Arc<DAE::Exp>, mut tpl: (bool, BackendDAE::Variables)) -> (Arc<DAE::Exp>, (bool, BackendDAE::Variables)) {
    let mut inExp: Arc<DAE::Exp> = inExp;
    let mut tpl: (bool, BackendDAE::Variables) = tpl;
    let mut outBoolean: bool = false;
    let mut knownVars: BackendDAE::Variables;
    (outBoolean, knownVars) = tpl.clone();
    tpl = (::match_deref::match_deref! { match &((inExp.clone(), outBoolean.clone())) {
        (_, false) => {
            (false, knownVars.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
            (crefIsParam(cr.clone(), knownVars.clone()), knownVars.clone())
        },
        _ => {
            (true, knownVars.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (inExp, tpl)
}

pub fn crefIsParam(mut inComponentRef: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> bool {
    let mut outBool: bool = true;
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    if '__try0: {
        (varlst, _) = unwrap_break_err!(getVar(inComponentRef.clone(), inVariables.clone()), '__try0);
        for mut var in &*varlst.clone() {
            let mut var = var.clone();
            outBool = isParam(var.clone());
            if !(outBool.clone()) {
                return outBool;
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        outBool = false;
    }
    outBool
}

pub fn existsVar(mut inComponentRef: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables, mut skipDiscrete: bool) -> bool {
    let mut outExists: bool = false;
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    match '__try0: {
        (varlst, _) = unwrap_break_err!(getVar(inComponentRef.clone(), inVariables.clone()), '__try0);
        varlst = if (skipDiscrete.clone()) {List::select(varlst.clone(), Arc::new(fnptr!(isVarNonDiscrete, BackendDAE::Var)))} else {varlst.clone()};
        outExists = !(varlst.clone().is_empty());
        Ok::<_, anyhow::Error>((outExists.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outExists = __try0_o0;
        }
        Err(_) => {
            outExists = false;
        }
    }
    outExists
}

pub fn existsAnyVar(mut inComponentRefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inVariables: BackendDAE::Variables, mut skipDiscrete: bool) -> Result<bool> {
    let mut outExists: bool = false;
    for mut cref in &*inComponentRefs.clone() {
        let mut cref = cref.clone();
        if existsVar(cref.clone(), inVariables.clone(), skipDiscrete.clone()) && !(isState(cref.clone(), inVariables.clone())?) {
            outExists = true;
            break;
        }
    }
    Ok(outExists)
}

pub fn makeVar(mut cr: Arc<DAE::ComponentRef>) -> BackendDAE::Var {
    let mut v: BackendDAE::Var;
    let mut tp: Arc<DAE::Type> = ComponentReference::crefLastType(cr.clone()).unwrap();
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = Expression::arrayDimension(tp.clone());
    v = BackendDAE::Var { varName: cr.clone(), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: Types::arrayElementType(tp.clone()), bindExp: None, tplExp: None, arryDim: dims.clone(), source: DAE::emptyElementSource.clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    v
}

pub fn addVarDAE(mut inVar: BackendDAE::Var, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    outEqSystem = BackendDAEUtil::setEqSystVars(inEqSystem.clone(), addVar(inVar.clone(), inEqSystem.orderedVars.clone())?)?;
    Ok(outEqSystem)
}

pub fn addVarsDAE(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Arc<BackendDAE::EqSystem> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = inEqSystem.clone();
    outEqSystem = List::fold(inVars.clone(), Arc::new(addVarDAE), outEqSystem.clone());
    outEqSystem
}

pub fn addGlobalKnownVarDAE(mut inGlobalKnownVar: BackendDAE::Var, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedGlobalKnownVars(inShared.clone(), addVar(inGlobalKnownVar.clone(), inShared.globalKnownVars.clone())?);
    Ok(outShared)
}

pub fn addNewGlobalKnownVarDAE(mut inGlobalKnownVar: BackendDAE::Var, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedGlobalKnownVars(inShared.clone(), addNewVar(inGlobalKnownVar.clone(), inShared.globalKnownVars.clone())?);
    Ok(outShared)
}

pub fn addAliasVarDAE(mut inVar: BackendDAE::Var, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedAliasVars(inShared.clone(), addVar(inVar.clone(), inShared.aliasVars.clone())?)?;
    Ok(outShared)
}

pub fn addNewAliasVarDAE(mut inVar: BackendDAE::Var, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedAliasVars(inShared.clone(), addNewVar(inVar.clone(), inShared.aliasVars.clone())?)?;
    Ok(outShared)
}

pub fn addVar(mut inVar: BackendDAE::Var, mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables = inVariables.clone();
    let mut hash_idx: i32 = 0;
    let mut arr_idx: i32 = 0;
    let mut indices: Arc<metamodelica::List<BackendDAE::CrefIndex>> = metamodelica::nil();
    hash_idx = intMod(ComponentReference::hashComponentRef(inVar.varName.clone())?, inVariables.bucketSize.clone()) + 1;
    indices = inVariables.crefIndices.clone().borrow()[(hash_idx.clone()-1) as usize].clone();
    match '__try0: {
        let BackendDAE::CREFINDEX { index: __pa1, .. } = (unwrap_break_err!(List::getMemberOnTrue(inVar.varName.clone(), indices.clone(), Arc::new(crefIndexEqualCref)), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        arr_idx = __pa1.clone();
        outVariables.varArr = unwrap_break_err!(vararraySetnth(inVariables.varArr.clone(), arr_idx.clone() + 1, inVar.clone()), '__try0);
        Ok::<_, anyhow::Error>((outVariables.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outVariables = __try0_o0;
        }
        Err(_) => {
            outVariables.varArr = vararrayAdd(outVariables.varArr.clone(), inVar.clone())?;
            {let _arr = outVariables.crefIndices.clone(); _arr.borrow_mut()[(hash_idx.clone()-1) as usize] = cons(BackendDAE::CrefIndex { cref: inVar.varName.clone(), index: outVariables.numberOfVars.clone() }, indices.clone()); _arr};
            outVariables.numberOfVars = outVariables.numberOfVars.clone() + 1;
        }
    }
    Ok(outVariables)
}

pub fn addVars(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inVariables: BackendDAE::Variables) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold(inVars.clone(), Arc::new(addVar), inVariables.clone());
    outVariables
}

pub fn addNewVars(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inVariables: BackendDAE::Variables) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold(inVars.clone(), Arc::new(addNewVar), inVariables.clone());
    outVariables
}

pub fn addNewVar(mut inVar: BackendDAE::Var, mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut hashvec: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>;
    let mut varr: BackendDAE::VariableArray;
    let mut bsize: i32 = 0;
    let mut num_vars: i32 = 0;
    let mut idx: i32 = 0;
    let mut indices: Arc<metamodelica::List<BackendDAE::CrefIndex>> = metamodelica::nil();
    let BackendDAE::VARIABLES { crefIndices: __pa0, varArr: __pa1, bucketSize: __pa2, numberOfVars: __pa3 } = (inVariables.clone()) else { bail!("pattern mismatch") };
    hashvec = __pa0.clone();
    varr = __pa1.clone();
    bsize = __pa2.clone();
    num_vars = __pa3.clone();
    idx = intMod(ComponentReference::hashComponentRef(inVar.varName.clone())?, bsize.clone()) + 1;
    varr = vararrayAdd(varr.clone(), inVar.clone())?;
    indices = hashvec.borrow()[(idx.clone()-1) as usize].clone();
    {let _arr = hashvec.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = cons(BackendDAE::CrefIndex { cref: inVar.varName.clone(), index: num_vars.clone() }, indices.clone()); _arr};
    outVariables = BackendDAE::Variables { crefIndices: hashvec.clone(), varArr: varr.clone(), bucketSize: bsize.clone(), numberOfVars: num_vars.clone() + 1 };
    Ok(outVariables)
}

pub fn addVariables(mut inSrcVars: BackendDAE::Variables, mut inDestVars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables = inDestVars.clone();
    let mut vars: metamodelica::Array<Option<BackendDAE::Var>>;
    let mut num_vars: i32 = 0;
    let mut var: BackendDAE::Var;
    let mut ovar: Option<BackendDAE::Var> = None;
    let BackendDAE::VARIABLES { varArr: BackendDAE::VARIABLE_ARRAY { varOptArr: __pa0, numberOfElements: __pa1 }, .. } = (inSrcVars.clone()) else { bail!("pattern mismatch") };
    vars = __pa0.clone();
    num_vars = __pa1.clone();
    for mut i in 1..=num_vars.clone() {
        ovar = vars.borrow()[(i.clone()-1) as usize].clone();
        if isSome(ovar.clone()) {
            let Some(__pa2) = (ovar.clone()) else { bail!("pattern mismatch") };
            var = __pa2.clone();
            outVars = addVar(var.clone(), outVars.clone())?;
        }
    }
    Ok(outVars)
}

pub fn getVarAt(mut inVariables: BackendDAE::Variables, mut inIndex: i32) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = vararrayNth(inVariables.varArr.clone(), inIndex.clone())?;
    Ok(outVar)
}

pub fn setVarAt(mut inVariables: BackendDAE::Variables, mut inIndex: i32, mut inVar: BackendDAE::Var) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables = inVariables.clone();
    vararraySetnth(inVariables.varArr.clone(), inIndex.clone(), inVar.clone())?;
    Ok(outVariables)
}

pub fn getVarAtIndexFirst(mut inIndex: i32, mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = getVarAt(inVariables.clone(), inIndex.clone())?;
    Ok(outVar)
}

pub fn getVarSharedAt(mut inInteger: i32, mut inShared: Arc<BackendDAE::Shared>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = getVarAt(inShared.globalKnownVars.clone(), inInteger.clone())?;
    Ok(outVar)
}

pub fn getVarDAE(mut inComponentRef: Arc<DAE::ComponentRef>, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>)> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outVarLst, outIntegerLst) = getVar(inComponentRef.clone(), inEqSystem.orderedVars.clone())?;
    Ok((outVarLst, outIntegerLst))
}

pub fn getVarShared(mut inComponentRef: Arc<DAE::ComponentRef>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>)> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outVarLst, outIntegerLst) = getVar(inComponentRef.clone(), inShared.globalKnownVars.clone())?;
    Ok((outVarLst, outIntegerLst))
}

pub fn containsVar(mut var: BackendDAE::Var, mut inVariables: BackendDAE::Variables) -> bool {
    let mut outB: bool = containsCref(var.varName.clone(), inVariables.clone());
    outB
}

pub fn containsCref(mut cr: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> bool {
    let mut outB: bool = false;
    match '__try0: {
        unwrap_break_err!(getVar(cr.clone(), inVariables.clone()), '__try0);
        outB = true;
        Ok::<_, anyhow::Error>((outB.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outB = __try0_o0;
        }
        Err(_) => {
            outB = false;
        }
    }
    outB
}

pub fn getVar(mut cr: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>)> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outVarLst, outIntegerLst) = 'mc: {
        let __mc_input = inVariables.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut v: BackendDAE::Var;
            let mut indx: i32 = 0;
            (v, indx) = getVar2(cr.clone(), inVariables.clone())?;
            Ok((list![v.clone()], if (true /* isPresent not implemented in Rust */) {list![indx.clone()]} else {metamodelica::nil()}))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut indxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crlst = ComponentReference::expandCref(cr.clone(), true)?;
            if true /* isPresent not implemented in Rust */ {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getVarLst(crlst.clone(), inVariables.clone())) {
                    (__pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                vLst = __pa0.clone();
                indxs = __pa1.clone();
            } else {
                let __pa2 = ::match_deref::match_deref! { match &(getVarLst(crlst.clone(), inVariables.clone())) {
                    (__pa2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                vLst = __pa2.clone();
                indxs = metamodelica::nil();
            }
            Ok((vLst.clone(), indxs.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut indxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let __pa0 = ::match_deref::match_deref! { match &(replaceVarWithWholeDim(cr.clone(), false)?) {
                (__pa0, true) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr1 = __pa0.clone();
            crlst = ComponentReference::expandCref(cr1.clone(), true)?;
            if true /* isPresent not implemented in Rust */ {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(getVarLst(crlst.clone(), inVariables.clone())) {
                    (__pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, __pa2) => (__pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                vLst = __pa1.clone();
                indxs = __pa2.clone();
            } else {
                let __pa3 = ::match_deref::match_deref! { match &(getVarLst(crlst.clone(), inVariables.clone())) {
                    (__pa3 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => __pa3.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                vLst = __pa3.clone();
                indxs = metamodelica::nil();
            }
            Ok((vLst.clone(), indxs.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVarLst, outIntegerLst))
}

pub fn getVarSingle(mut cr: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> Result<(BackendDAE::Var, i32)> {
    let mut outVar: BackendDAE::Var;
    let mut outInteger: i32 = 0;
    (outVar, outInteger) = 'mc: {
        let __mc_input = cr.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: BackendDAE::Var;
                    let mut indx: i32 = 0;
                    (v, indx) = getVar2(cr.clone(), inVariables.clone())?;
                    Ok((v.clone(), indx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: BackendDAE::Var;
                    let mut indx: i32 = 0;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    crlst = ComponentReference::expandCref(cr.clone(), true)?;
                    if true /* isPresent not implemented in Rust */ {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getVarLst(crlst.clone(), inVariables.clone())) {
                            (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }) => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        v = __pa0.clone();
                        indx = __pa1.clone();
                    } else {
                        let __pa4 = ::match_deref::match_deref! { match &(getVarLst(crlst.clone(), inVariables.clone())) {
                            (Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil }, _) => __pa4.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        v = __pa4.clone();
                        indx = 0;
                    }
                    Ok((v.clone(), indx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: BackendDAE::Var;
                    let mut indx: i32 = 0;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let __pa0 = ::match_deref::match_deref! { match &(replaceVarWithWholeDim(cr.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr1 = __pa0.clone();
                    crlst = ComponentReference::expandCref(cr1.clone(), true)?;
                    if true /* isPresent not implemented in Rust */ {
                        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(getVarLst(crlst.clone(), inVariables.clone())) {
                            (Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => (__pa1.clone(), __pa2.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        v = __pa1.clone();
                        indx = __pa2.clone();
                    } else {
                        let __pa5 = ::match_deref::match_deref! { match &(getVarLst(crlst.clone(), inVariables.clone())) {
                            (Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil }, _) => __pa5.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        v = __pa5.clone();
                        indx = 0;
                    }
                    Ok((v.clone(), indx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outInteger))
}

pub fn getVarTryHard(mut cref: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables) -> Option<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut var_lst_opt: Option<Arc<metamodelica::List<BackendDAE::Var>>> = None;
    let mut var: BackendDAE::Var;
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut strippedCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    match '__try0: {
        (var, _) = unwrap_break_err!(getVarSingle(cref.clone(), vars.clone()), '__try0);
        var_lst_opt = Some(list![var.clone()]);
        Ok::<_, anyhow::Error>((var_lst_opt.clone(),))
    } {
        Ok((__try0_o0,)) => {
            var_lst_opt = __try0_o0;
        }
        Err(_) => {
            match '__try1: {
                (var_lst, _) = unwrap_break_err!(getVar(cref.clone(), vars.clone()), '__try1);
                var_lst_opt = Some(var_lst.clone());
                Ok::<_, anyhow::Error>((var_lst_opt.clone(),))
            } {
                Ok((__try1_o0,)) => {
                    var_lst_opt = __try1_o0;
                }
                Err(_) => {
                    match '__try2: {
                        strippedCref = ComponentReference::crefStripSubsExceptModelSubs(cref.clone());
                        (var, _) = unwrap_break_err!(getVarSingle(strippedCref.clone(), vars.clone()), '__try2);
                        var_lst_opt = Some(list![var.clone()]);
                        Ok::<_, anyhow::Error>((var_lst_opt.clone(),))
                    } {
                        Ok((__try2_o0,)) => {
                            var_lst_opt = __try2_o0;
                        }
                        Err(_) => {
                            var_lst_opt = None;
                        }
                    }
                }
            }
        }
    }
    var_lst_opt
}

fn replaceVarWithWholeDim(mut inCref: Arc<DAE::ComponentRef>, mut iPerformed: bool) -> Result<(Arc<DAE::ComponentRef>, bool)> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut oPerformed: bool = false;
    (outCref, oPerformed) = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, subscriptLst: subs, identType: ty, ident: name } => {
            let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut b: bool = false;
            (subs_1, b) = replaceVarWithWholeDimSubs(subs.clone(), iPerformed.clone())?;
            (cr_1, b) = replaceVarWithWholeDim(cr.clone(), b.clone())?;
            (if (referenceEq(&subs_1.clone(),&subs.clone()) && referenceEq(&cr_1.clone(),&cr.clone())) {inCref.clone()} else {Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs_1.clone(), componentRef: cr_1.clone() })}, b.clone())
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, identType: ty, ident: name } => {
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut b: bool = false;
            (subs_1, b) = replaceVarWithWholeDimSubs(subs.clone(), iPerformed.clone())?;
            (if (referenceEq(&subs_1.clone(),&subs.clone())) {inCref.clone()} else {Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs_1.clone() })}, b.clone())
        },
        Deref @ DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { .. } => {
            (inCref.clone(), iPerformed.clone())
        },
        Deref @ DAE::ComponentRef::WILD => {
            (inCref.clone(), iPerformed.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendVariable.replaceVarWithWholeDim: Unknown cref")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCref, oPerformed))
}

fn replaceVarWithWholeDimSubs(mut inSubscript: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut iPerformed: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Subscript>>>, bool)> {
    let mut outSubscript: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut oPerformed: bool = false;
    (outSubscript, oPerformed) = (::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inSubscript.clone(), iPerformed.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: rest } => {
            let mut b: bool = false;
            (_, b) = replaceVarWithWholeDimSubs(rest.clone(), iPerformed.clone())?;
            (cons(Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM), rest.clone()), b.clone())
        },
        Deref @ metamodelica::List::Cons { head: sub @ Deref @ DAE::Subscript::SLICE { exp: sub_exp }, tail: rest } => {
            let mut res: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut r#const: bool = false;
            (res, b) = replaceVarWithWholeDimSubs(rest.clone(), iPerformed.clone())?;
            r#const = Expression::isConst(sub_exp.clone())?;
            res = if (r#const.clone()) {cons(sub.clone(), rest.clone())} else {cons(Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM), rest.clone())};
            (res.clone(), b.clone() || !(r#const.clone()))
        },
        Deref @ metamodelica::List::Cons { head: sub @ Deref @ DAE::Subscript::INDEX { exp: sub_exp }, tail: rest } => {
            let mut sub_exp_: Arc<DAE::Exp>;
            let mut res: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut r#const: bool = false;
            let mut calcRange: bool = false;
            (sub_exp_, calcRange) = computeRangeExps(sub_exp.clone());
            (res, b) = replaceVarWithWholeDimSubs(rest.clone(), iPerformed.clone())?;
            r#const = Expression::isConst(sub_exp_.clone())?;
            res = cons(if (r#const.clone()) {if (referenceEq(&sub_exp.clone(),&sub_exp_.clone())) {sub.clone()} else {Arc::new(DAE::Subscript::INDEX { exp: sub_exp_.clone() })}} else {Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM)}, rest.clone());
            (res.clone(), b.clone() || !(r#const.clone()) || calcRange.clone())
        },
        Deref @ metamodelica::List::Cons { head: sub @ Deref @ DAE::Subscript::WHOLE_NONEXP { exp: sub_exp }, tail: rest } => {
            let mut res: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut r#const: bool = false;
            (res, b) = replaceVarWithWholeDimSubs(rest.clone(), iPerformed.clone())?;
            r#const = Expression::isConst(sub_exp.clone())?;
            res = if (r#const.clone()) {cons(sub.clone(), rest.clone())} else {cons(Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM), rest.clone())};
            (res.clone(), b.clone() || !(r#const.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSubscript, oPerformed))
}

fn computeRangeExps(mut inExp: Arc<DAE::Exp>) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut isCalculated: bool = false;
    (outExp, isCalculated) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::ICONST { integer: stop2 }, start: Deref @ DAE::Exp::ICONST { integer: 1 }, .. }, operator: DAE::Operator::ADD { .. }, exp1: Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::ICONST { integer: stop1 }, start: Deref @ DAE::Exp::ICONST { integer: 1 }, ty, .. } } => {
            let mut exp: Arc<DAE::Exp>;
            let mut stop2 = (*stop2).clone();
            stop2 = stop1.clone() + stop2.clone();
            exp = Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: Arc::new(DAE::Exp::ICONST { integer: 1 }), step: None, stop: Arc::new(DAE::Exp::ICONST { integer: stop2.clone() }) });
            (exp.clone(), true)
        },
        _ => {
            (inExp.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, isCalculated)
}

pub fn getVarLst(mut inComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inVariables: BackendDAE::Variables) -> (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>) {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut v: BackendDAE::Var;
    let mut indx: i32 = 0;
    if true /* isPresent not implemented in Rust */ {
        for mut cr in &*inComponentRefLst.clone() {
            let mut cr = cr.clone();
            if '__try0: {
                (v, indx) = unwrap_break_err!(getVar2(cr.clone(), inVariables.clone()), '__try0);
                outVarLst = cons(v.clone(), outVarLst.clone());
                outIntegerLst = cons(indx.clone(), outIntegerLst.clone());
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
    } else {
        for mut cr in &*inComponentRefLst.clone() {
            let mut cr = cr.clone();
            if '__try1: {
                (v, indx) = unwrap_break_err!(getVar2(cr.clone(), inVariables.clone()), '__try1);
                outVarLst = cons(v.clone(), outVarLst.clone());
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
    }
    (outVarLst, outIntegerLst)
}

pub fn getVar2(mut inCref: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> Result<(BackendDAE::Var, i32)> {
    let mut outVar: BackendDAE::Var;
    let mut outIndex: i32 = 0;
    let mut indices: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>;
    let mut arr: BackendDAE::VariableArray;
    let mut buckets: i32 = 0;
    let mut hash_idx: i32 = 0;
    let mut cr_indices: Arc<metamodelica::List<BackendDAE::CrefIndex>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let BackendDAE::VARIABLES { bucketSize: __pa0, varArr: __pa1, crefIndices: __pa2, .. } = (inVariables.clone()) else { bail!("pattern mismatch") };
    buckets = __pa0.clone();
    arr = __pa1.clone();
    indices = __pa2.clone();
    hash_idx = intMod(ComponentReference::hashComponentRef(inCref.clone())?, buckets.clone()) + 1;
    cr_indices = indices.borrow()[(hash_idx.clone()-1) as usize].clone();
    let BackendDAE::CREFINDEX { index: __pa3, .. } = (List::getMemberOnTrue(inCref.clone(), cr_indices.clone(), Arc::new(crefIndexEqualCref))?) else { bail!("pattern mismatch") };
    outIndex = __pa3.clone();
    outIndex = outIndex.clone() + 1;
    let ref __pa5 @ BackendDAE::VAR { varName: ref __pa4, .. } = (vararrayNth(arr.clone(), outIndex.clone())?) else { bail!("pattern mismatch") };
    cr = __pa4.clone();
    outVar = __pa5.clone();
    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), inCref.clone())?) else { bail!("pattern mismatch") };
    Ok((outVar, outIndex))
}

fn crefIndexEqualCref(mut inCref: Arc<DAE::ComponentRef>, mut inIndex: BackendDAE::CrefIndex) -> Result<bool> {
    let mut outMatch: bool = false;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let BackendDAE::CREFINDEX { cref: __pa0, .. } = (inIndex.clone()) else { bail!("pattern mismatch") };
    cr = __pa0.clone();
    outMatch = ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), inCref.clone())?;
    Ok(outMatch)
}

pub fn getVarIndexFromVars(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inVariables: BackendDAE::Variables) -> Arc<metamodelica::List<i32>> {
    let mut outIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut var in &*inVars.clone() {
        let mut var = var.clone();
        (_, outIndices) = traversingVarIndexFinder(var.clone(), inVariables.clone(), outIndices.clone());
    }
    outIndices = outIndices.clone().reverse();
    outIndices
}

pub fn getVarIndexFromVariables(mut inVariables: BackendDAE::Variables, mut inVariables2: BackendDAE::Variables) -> Result<Arc<metamodelica::List<i32>>> {
    let mut v_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    v_lst = traverseBackendDAEVars(inVariables.clone(), Arc::new({ let __pe_b1 = inVariables2.clone(); move |__pe_a0, __pe_a2| Ok(traversingVarIndexFinder(__pe_a0, __pe_b1.clone(), __pe_a2)) }), metamodelica::nil())?.reverse();
    Ok(v_lst)
}

fn traversingVarIndexFinder(mut inVar: BackendDAE::Var, mut inVars: BackendDAE::Variables, mut inIndices: Arc<metamodelica::List<i32>>) -> (BackendDAE::Var, Arc<metamodelica::List<i32>>) {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    match '__try0: {
        cr = unwrap_break_err!(varCref(inVar.clone()), '__try0);
        (_, indices) = unwrap_break_err!(getVar(cr.clone(), inVars.clone()), '__try0);
        outIndices = List::append_reverse(indices.clone(), inIndices.clone());
        Ok::<_, anyhow::Error>((outIndices.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outIndices = __try0_o0;
        }
        Err(_) => {
            outIndices = inIndices.clone();
        }
    }
    (outVar, outIndices)
}

pub fn getVarIndexFromVariablesIndexInFirstSet(mut inVariables: BackendDAE::Variables, mut inVariables2: BackendDAE::Variables) -> Result<Arc<metamodelica::List<i32>>> {
    let mut v_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut a: Mutable::Mutable<Arc<metamodelica::List<i32>>>;
    (_, a, _) = traverseBackendDAEVars(inVariables.clone(), Arc::new(fnptr!(traversingVarIndexInFirstSetFinder, BackendDAE::Var, (BackendDAE::Variables, Mutable::Mutable<Arc<metamodelica::List<i32>>>, Mutable::Mutable<i32>))), (inVariables2.clone(), Mutable::create(metamodelica::nil()), Mutable::create(1)))?;
    v_lst = Mutable::access(a.clone()).reverse();
    Ok(v_lst)
}

fn traversingVarIndexInFirstSetFinder(mut var: BackendDAE::Var, mut data: (BackendDAE::Variables, Mutable::Mutable<Arc<metamodelica::List<i32>>>, Mutable::Mutable<i32>)) -> (BackendDAE::Var, (BackendDAE::Variables, Mutable::Mutable<Arc<metamodelica::List<i32>>>, Mutable::Mutable<i32>)) {
    let mut var: BackendDAE::Var = var;
    let mut data: (BackendDAE::Variables, Mutable::Mutable<Arc<metamodelica::List<i32>>>, Mutable::Mutable<i32>) = data;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut vars: BackendDAE::Variables;
    let mut l: Mutable::Mutable<Arc<metamodelica::List<i32>>>;
    let mut i: Mutable::Mutable<i32>;
    (vars, l, i) = data.clone();
    if '__try0: {
        cr = unwrap_break_err!(varCref(var.clone()), '__try0);
        unwrap_break_err!(getVar(cr.clone(), vars.clone()), '__try0);
        Mutable::update(l.clone(), cons(Mutable::access(i.clone()), Mutable::access(l.clone())));
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    Mutable::update(i.clone(), Mutable::access(i.clone()) + 1);
    (var, data)
}

pub fn mergeVariables(mut inVariables1: BackendDAE::Variables, mut inVariables2: BackendDAE::Variables, mut copy: bool) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut num_vars: i32 = 0;
    num_vars = varsSize(inVariables2.clone())?;
    if varsLoadFactor(inVariables1.clone(), num_vars.clone()) > metamodelica::OrderedFloat((1) as f64) {
        outVariables = emptyVarsSized(varsSize(inVariables1.clone())? + num_vars.clone());
        outVariables = addVariables(inVariables1.clone(), outVariables.clone())?;
    } else if copy.clone() {
        outVariables = copyVariables(inVariables1.clone());
    } else {
        outVariables = inVariables1.clone();
    }
    outVariables = addVariables(inVariables2.clone(), outVariables.clone())?;
    Ok(outVariables)
}

pub fn rehashVariables(mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut load: metamodelica::Real = varsLoadFactor(inVariables.clone(), 0);
    if load.clone() < metamodelica::OrderedFloat(0.5_f64) || load.clone() > metamodelica::OrderedFloat(1.0_f64) {
        outVariables = emptyVarsSized(varsSize(inVariables.clone())?);
        outVariables = addVariables(inVariables.clone(), outVariables.clone())?;
    } else {
        outVariables = inVariables.clone();
    }
    Ok(outVariables)
}

pub fn traverseBackendDAEVars<ArgT: Clone + 'static>(mut inVariables: BackendDAE::Variables, mut inFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)> + 'static>, mut inArg: ArgT) -> Result<ArgT> {
    pub type FuncType<ArgT: Clone> = fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)>;

    let mut outArg: ArgT;
    let mut num_vars: i32 = 0;
    let mut vars: metamodelica::Array<Option<BackendDAE::Var>>;
    let BackendDAE::VARIABLES { varArr: BackendDAE::VARIABLE_ARRAY { varOptArr: __pa0, numberOfElements: __pa1 }, .. } = (inVariables.clone()) else { bail!("pattern mismatch") };
    vars = __pa0.clone();
    num_vars = __pa1.clone();
    outArg = BackendDAEUtil::traverseArrayNoCopy(vars.clone(), inFunc.clone(), Arc::new(fnptr!(traverseBackendDAEVars2, Option<BackendDAE::Var>, _, _)), inArg.clone(), num_vars.clone())?;
    Ok(outArg)
}

pub type filterFunc = fn(BackendDAE::Var) -> Result<bool>;

pub fn filterCrefs(mut variables: BackendDAE::Variables, mut func: filterFunc, mut acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = acc;
    acc = traverseBackendDAEVars(variables.clone(), Arc::new({ let __pe_b1 = func; move |__pe_a0, __pe_a2| Ok(filterTraverse(__pe_a0, __pe_b1.clone(), __pe_a2)) }), acc.clone())?;
    Ok(acc)
}

fn filterTraverse(mut var: BackendDAE::Var, mut func: filterFunc, mut acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> (BackendDAE::Var, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) {
    let mut var: BackendDAE::Var = var;
    let mut acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = acc;
    if func(var.clone()).unwrap() {
        acc = cons(var.varName.clone(), acc.clone());
    }
    (var, acc)
}

fn traverseBackendDAEVars2<ArgT: Clone + 'static>(mut inVar: Option<BackendDAE::Var>, mut inFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)> + 'static>, mut inArg: ArgT) -> ArgT {
    pub type FuncType<ArgT: Clone> = fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)>;

    let mut outArg: ArgT;
    outArg = (match inVar.clone() {
        Some(mut v) => {
            let mut arg: ArgT;
            (_, arg) = inFunc(v.clone(), inArg.clone()).unwrap();
            arg.clone()
        },
        _ => {
            inArg.clone()
        },
    });
    outArg
}

pub fn traverseBackendDAEVarsWithStop<ArgT: Clone + 'static>(mut inVariables: BackendDAE::Variables, mut inFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, bool, ArgT)> + 'static>, mut inArg: ArgT) -> Result<ArgT> {
    pub type FuncType<ArgT: Clone> = fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, bool, ArgT)>;

    let mut outArg: ArgT;
    let mut num_vars: i32 = 0;
    let mut vars: metamodelica::Array<Option<BackendDAE::Var>>;
    let BackendDAE::VARIABLES { varArr: BackendDAE::VARIABLE_ARRAY { varOptArr: __pa0, numberOfElements: __pa1 }, .. } = (inVariables.clone()) else { bail!("pattern mismatch") };
    vars = __pa0.clone();
    num_vars = __pa1.clone();
    outArg = BackendDAEUtil::traverseArrayNoCopyWithStop(vars.clone(), inFunc.clone(), Arc::new(fnptr!(traverseBackendDAEVarsWithStop2, Option<BackendDAE::Var>, _, _)), inArg.clone(), num_vars.clone())?;
    Ok(outArg)
}

fn traverseBackendDAEVarsWithStop2<ArgT: Clone + 'static>(mut inVar: Option<BackendDAE::Var>, mut inFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, bool, ArgT)> + 'static>, mut inArg: ArgT) -> (bool, ArgT) {
    pub type FuncType<ArgT: Clone> = fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, bool, ArgT)>;

    let mut outContinue: bool = false;
    let mut outArg: ArgT;
    (outContinue, outArg) = (match inVar.clone() {
        None => {
            (true, inArg.clone())
        },
        Some(mut v) => {
            let mut arg: ArgT;
            let mut cont: bool = false;
            (_, cont, arg) = inFunc(v.clone(), inArg.clone()).unwrap();
            (cont.clone(), arg.clone())
        },
    });
    (outContinue, outArg)
}

pub fn traverseBackendDAE<ArgT: Clone + 'static>(mut dae: Arc<BackendDAE::BackendDAE>, mut inFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<BackendDAE::BackendDAE>, ArgT)> {
    pub type FuncType<ArgT: Clone> = fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)>;

    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    let mut arg: ArgT = arg;
    for mut syst in &*dae.eqs.clone() {
        let mut syst = syst.clone();
        (_, arg) = traverseBackendDAEVarsWithUpdate(syst.orderedVars.clone(), inFunc.clone(), arg.clone())?;
    }
    (_, arg) = traverseBackendDAEVarsWithUpdate(dae.shared.globalKnownVars.clone(), inFunc.clone(), arg.clone())?;
    (_, arg) = traverseBackendDAEVarsWithUpdate(dae.shared.localKnownVars.clone(), inFunc.clone(), arg.clone())?;
    (_, arg) = traverseBackendDAEVarsWithUpdate(dae.shared.externalObjects.clone(), inFunc.clone(), arg.clone())?;
    (_, arg) = traverseBackendDAEVarsWithUpdate(dae.shared.aliasVars.clone(), inFunc.clone(), arg.clone())?;
    Ok((dae, arg))
}

pub fn traverseBackendDAEVarsWithUpdate<ArgT: Clone + 'static>(mut inVariables: BackendDAE::Variables, mut inFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(BackendDAE::Variables, ArgT)> {
    pub type FuncType<ArgT: Clone> = fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)>;

    let mut outVariables: BackendDAE::Variables;
    let mut outArg: ArgT;
    let mut indices: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>;
    let mut buckets: i32 = 0;
    let mut num_vars1: i32 = 0;
    let mut num_vars2: i32 = 0;
    let mut vars: metamodelica::Array<Option<BackendDAE::Var>>;
    let BackendDAE::VARIABLES { crefIndices: __pa0, varArr: BackendDAE::VARIABLE_ARRAY { numberOfElements: __pa1, varOptArr: __pa2 }, bucketSize: __pa3, numberOfVars: __pa4 } = (inVariables.clone()) else { bail!("pattern mismatch") };
    indices = __pa0.clone();
    num_vars1 = __pa1.clone();
    vars = __pa2.clone();
    buckets = __pa3.clone();
    num_vars2 = __pa4.clone();
    if num_vars1.clone() != num_vars2.clone() {
        Error::addInternalError((literal!("function traverseBackendDAEVarsWithUpdate failed")).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    (vars, outArg) = BackendDAEUtil::traverseArrayNoCopyWithUpdate(vars.clone(), inFunc.clone(), Arc::new(fnptr!(traverseBackendDAEVarsWithUpdate2, Option<BackendDAE::Var>, _, _)), inArg.clone(), num_vars1.clone())?;
    outVariables = BackendDAE::Variables { crefIndices: indices.clone(), varArr: BackendDAE::VariableArray { numberOfElements: num_vars1.clone(), varOptArr: vars.clone() }, bucketSize: buckets.clone(), numberOfVars: num_vars2.clone() };
    Ok((outVariables, outArg))
}

fn traverseBackendDAEVarsWithUpdate2<ArgT: Clone + 'static>(mut inVar: Option<BackendDAE::Var>, mut inFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)> + 'static>, mut inArg: ArgT) -> (Option<BackendDAE::Var>, ArgT) {
    pub type FuncType<ArgT: Clone> = fn(BackendDAE::Var, ArgT) -> Result<(BackendDAE::Var, ArgT)>;

    let mut outVar: Option<BackendDAE::Var> = None;
    let mut outArg: ArgT;
    (outVar, outArg) = (match inVar.clone() {
        None => {
            (inVar.clone(), inArg.clone())
        },
        Some(mut v) => {
            let mut ov: Option<BackendDAE::Var> = None;
            let mut new_v: BackendDAE::Var;
            let mut arg: ArgT;
            (new_v, arg) = inFunc(v.clone(), inArg.clone()).unwrap();
            ov = if (referenceEq(&v.clone(),&new_v.clone())) {inVar.clone()} else {Some(new_v.clone())};
            (ov.clone(), arg.clone())
        },
    });
    (outVar, outArg)
}

pub fn getAllCrefFromVariables(mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    cr_lst = traverseBackendDAEVars(inVariables.clone(), Arc::new(traversingVarCrefFinder), metamodelica::nil())?;
    Ok(cr_lst)
}

fn traversingVarCrefFinder(mut inVar: BackendDAE::Var, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(BackendDAE::Var, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outVar: BackendDAE::Var;
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outVar, outCrefs) = 'mc: {
        let __mc_input = (inVar.clone(), inCrefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, cr_lst) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cr = varCref(v.clone())?;
                    Ok((v.clone(), cons(cr.clone(), cr_lst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outCrefs))
}

pub fn collectVarKindVarinVariables(mut inVar: BackendDAE::Var, mut inVarArrays: (Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, BackendDAE::Variables))> {
    pub type checkVarKindFunc = fn(BackendDAE::Var) -> Result<bool>;

    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outVarArrays: (Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, BackendDAE::Variables) = inVarArrays.clone();
    let mut vararray: BackendDAE::Variables;
    let mut checkVarKind: Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>;
    (checkVarKind, vararray) = inVarArrays.clone();
    outVarArrays = (match inVar.clone() {
        _ if (checkVarKind(inVar.clone())?) => {
            vararray = addVar(inVar.clone(), vararray.clone())?;
            (checkVarKind.clone(), vararray.clone())
        },
        _ => {
            outVarArrays.clone()
        },
    });
    Ok((outVar, outVarArrays))
}

pub fn getAllDiscreteVarFromVariables(mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut v_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    v_lst = traverseBackendDAEVars(inVariables.clone(), Arc::new(fnptr!(traversingisisVarDiscreteFinder, BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>)), metamodelica::nil())?;
    Ok(v_lst)
}

fn traversingisisVarDiscreteFinder(mut inVar: BackendDAE::Var, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> (BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>) {
    let mut v: BackendDAE::Var;
    let mut v_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    v = inVar.clone();
    v_lst = List::consOnTrue(isVarDiscrete(v.clone()), v.clone(), inVars.clone());
    (v, v_lst)
}

pub fn getAllStateVarFromVariables(mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut v_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    v_lst = traverseBackendDAEVars(inVariables.clone(), Arc::new(fnptr!(traversingisStateVarFinder, BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>)), metamodelica::nil())?;
    Ok(v_lst)
}

pub fn getAllClockedStatesFromVariables(mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut v_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    v_lst = traverseBackendDAEVars(inVariables.clone(), Arc::new(fnptr!(traversingisClockedStateVarFinder, BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>)), metamodelica::nil())?;
    Ok(v_lst)
}

pub fn getNumStateVarFromVariables(mut inVariables: BackendDAE::Variables) -> Result<i32> {
    let mut count: i32 = 0;
    count = traverseBackendDAEVars(inVariables.clone(), Arc::new(fnptr!(traversingisStateCount, BackendDAE::Var, i32)), 0)?;
    Ok(count)
}

fn traversingisStateVarFinder(mut inVar: BackendDAE::Var, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> (BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>) {
    let mut v: BackendDAE::Var;
    let mut v_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    v = inVar.clone();
    v_lst = List::consOnTrue(isStateVar(v.clone()), v.clone(), inVars.clone());
    (v, v_lst)
}

fn traversingisClockedStateVarFinder(mut inVar: BackendDAE::Var, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> (BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>) {
    let mut v: BackendDAE::Var;
    let mut v_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    v = inVar.clone();
    v_lst = List::consOnTrue(isClockedStateVar(v.clone()), v.clone(), inVars.clone());
    (v, v_lst)
}

fn traversingisStateCount(mut v: BackendDAE::Var, mut count: i32) -> (BackendDAE::Var, i32) {
    let mut v: BackendDAE::Var = v;
    let mut count: i32 = count;
    if isStateVar(v.clone()) {
        count = count.clone() + 1;
    }
    (v, count)
}

pub fn getAllVarIndicesFromVariables(mut inVariables: BackendDAE::Variables, mut isFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>)> {
    pub type FindFunc = fn(BackendDAE::Var) -> Result<bool>;

    let mut v_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut i_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut v_a: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
    let mut i_a: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    v_a = arrayCreate(1, metamodelica::nil());
    i_a = arrayCreate(1, metamodelica::nil());
    let _ = traverseBackendDAEVars(inVariables.clone(), Arc::new({ let __pe_b1 = v_a.clone(); let __pe_b2 = i_a.clone(); let __pe_b3: Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static> = isFunc.clone(); move |__pe_a0, __pe_a4| traversingisXXXFinder(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), arrayCreate(1, 1))?;
    v_lst = v_a.borrow()[(1-1) as usize].clone();
    i_lst = i_a.borrow()[(1-1) as usize].clone();
    Ok((v_lst, i_lst))
}

fn traversingisXXXFinder(mut inVar: BackendDAE::Var, mut v_lst: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>, mut i_lst: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut isFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, mut i: metamodelica::Array<i32>) -> Result<(BackendDAE::Var, metamodelica::Array<i32>)> {
    pub type FindFunc = fn(BackendDAE::Var) -> Result<bool>;

    let mut inVar: BackendDAE::Var = inVar;
    let mut i: metamodelica::Array<i32> = i;
    if isFunc(inVar.clone())? {
        {let _arr = v_lst.clone(); let _val = cons(inVar.clone(), v_lst.borrow()[(1-1) as usize].clone()); _arr.borrow_mut()[(1-1) as usize] = _val; _arr};
        {let _arr = i_lst.clone(); let _val = cons(i.borrow()[(1-1) as usize].clone(), i_lst.borrow()[(1-1) as usize].clone()); _arr.borrow_mut()[(1-1) as usize] = _val; _arr};
    }
    {
        let __cell0 = i.borrow()[(1-1) as usize].clone() + 1;
        i.clone().borrow_mut()[(1-1) as usize] = __cell0;
    }
    Ok((inVar, i))
}

pub fn mergeVariableOperations(mut inVar: BackendDAE::Var, mut inOps: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    ops = inOps.clone().reverse();
    outVar.source = List::foldr(ops.clone(), Arc::new(ElementSource::addSymbolicTransformation), inVar.source.clone());
    outVar
}

pub fn mergeAliasVars(mut inVar: BackendDAE::Var, mut inAVar: BackendDAE::Var, mut negate: bool, mut globalKnownVars: BackendDAE::Variables) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    let mut v1: BackendDAE::Var;
    let mut v2: BackendDAE::Var;
    let mut fixed: bool = false;
    let mut fixeda: bool = false;
    let mut sv: Option<Arc<DAE::Exp>> = None;
    let mut sva: Option<Arc<DAE::Exp>> = None;
    let mut so: Option<Arc<DAE::Exp>> = None;
    let mut soa: Option<Arc<DAE::Exp>> = None;
    let mut start: Arc<DAE::Exp>;
    fixed = varFixed(inVar.clone());
    fixeda = varFixed(inAVar.clone());
    sv = varStartValueOption(inVar.clone())?;
    sva = varStartValueOption(inAVar.clone())?;
    so = varStartOrigin(inVar.clone())?;
    soa = varStartOrigin(inAVar.clone())?;
    v1 = mergeStartFixed(inVar.clone(), fixed.clone(), sv.clone(), so.clone(), inAVar.clone(), fixeda.clone(), sva.clone(), soa.clone(), negate.clone(), globalKnownVars.clone())?;
    v2 = mergeNominalAttribute(inAVar.clone(), v1.clone(), negate.clone())?;
    outVar = mergeMinMaxAttribute(inAVar.clone(), v2.clone(), negate.clone())?;
    Ok(outVar)
}

fn mergeStartFixed(mut inVar: BackendDAE::Var, mut fixed: bool, mut sv: Option<Arc<DAE::Exp>>, mut so: Option<Arc<DAE::Exp>>, mut inAVar: BackendDAE::Var, mut fixeda: bool, mut sva: Option<Arc<DAE::Exp>>, mut soa: Option<Arc<DAE::Exp>>, mut negate: bool, mut globalKnownVars: BackendDAE::Variables) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = 'mc: {
        let __mc_input = (inVar.clone(), fixed.clone(), sv.clone(), so.clone(), inAVar.clone(), fixeda.clone(), sva.clone(), soa.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, true, _, _, _, false, _, _) => {
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, _, _, _, true, Some(sb), _) => {
                    let mut v1: BackendDAE::Var;
                    let mut v2: BackendDAE::Var;
                    let mut e: Arc<DAE::Exp>;
                    e = if (negate.clone()) {Expression::negate(sb.clone())?} else {sb.clone()};
                    v1 = setVarStartValue(v.clone(), e.clone())?;
                    v2 = setVarFixed(v1.clone(), true)?;
                    Ok(v2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, None, _, _, true, None, _) => {
                    let mut v1: BackendDAE::Var;
                    v1 = setVarFixed(v.clone(), true)?;
                    Ok(v1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, Some(_), _, _, true, None, _) => {
                    let mut v1: BackendDAE::Var;
                    let _ = setVarStartValueOption(v.clone(), None)?;
                    v1 = setVarFixed(v.clone(), true)?;
                    Ok(v1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, None, _, _, false, None, _) => {
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, Some(_), _, _, false, None, _) => {
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, None, _, _, false, Some(sb), _) => {
                    let mut v1: BackendDAE::Var;
                    let mut e: Arc<DAE::Exp>;
                    e = if (negate.clone()) {Expression::negate(sb.clone())?} else {sb.clone()};
                    v1 = setVarStartValue(v.clone(), e.clone())?;
                    Ok(v1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varType: ty, .. }, false, _, _, BackendDAE::Var { varType: tya, .. }, false, _, _) => {
                    let mut v1: BackendDAE::Var;
                    let mut sa: Arc<DAE::Exp>;
                    let mut sb: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut origin: Option<Arc<DAE::Exp>> = None;
                    sa = startValueType(sv.clone(), ty.clone())?;
                    sb = startValueType(sva.clone(), tya.clone())?;
                    e = if (negate.clone()) {Expression::negate(sb.clone())?} else {sb.clone()};
                    (e, origin) = getNonZeroStart(false, sa.clone(), so.clone(), e.clone(), soa.clone(), globalKnownVars.clone())?;
                    let _ = setVarStartValue(v.clone(), e.clone())?;
                    v1 = setVarStartOrigin(v.clone(), origin.clone())?;
                    Ok(v1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varType: ty, varName: cr, .. }, false, _, _, BackendDAE::Var { varType: tya, varName: cra, .. }, false, _, _) => {
                    let mut sa: Arc<DAE::Exp>;
                    let mut sb: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut i: i32 = 0;
                    let mut ia: i32 = 0;
                    sa = startValueType(sv.clone(), ty.clone())?;
                    sb = startValueType(sva.clone(), tya.clone())?;
                    e = if (negate.clone()) {Expression::negate(sb.clone())?} else {sb.clone()};
                    i = ComponentReference::crefDepth(cr.clone())?;
                    ia = ComponentReference::crefDepth(cra.clone())?;
                    Ok(mergeStartFixed1(intLt(ia.clone(), i.clone()), v.clone(), cr.clone(), sa.clone(), cra.clone(), e.clone(), soa.clone(), negate.clone(), (literal!(" have start values ")).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, true, None, _, _, true, None, _) => {
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varType: ty, .. }, true, _, _, BackendDAE::Var { varType: tya, .. }, true, _, _) => {
                    let mut v1: BackendDAE::Var;
                    let mut sa: Arc<DAE::Exp>;
                    let mut sb: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut origin: Option<Arc<DAE::Exp>> = None;
                    sa = startValueType(sv.clone(), ty.clone())?;
                    sb = startValueType(sva.clone(), tya.clone())?;
                    e = if (negate.clone()) {Expression::negate(sb.clone())?} else {sb.clone()};
                    (e, origin) = getNonZeroStart(true, sa.clone(), so.clone(), e.clone(), soa.clone(), globalKnownVars.clone())?;
                    let _ = setVarStartValue(v.clone(), e.clone())?;
                    v1 = setVarStartOrigin(v.clone(), origin.clone())?;
                    Ok(v1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varType: ty, varName: cr, .. }, true, _, _, BackendDAE::Var { varType: tya, varName: cra, .. }, true, _, _) => {
                    let mut sa: Arc<DAE::Exp>;
                    let mut sb: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut i: i32 = 0;
                    let mut ia: i32 = 0;
                    sa = startValueType(sv.clone(), ty.clone())?;
                    sb = startValueType(sva.clone(), tya.clone())?;
                    e = if (negate.clone()) {Expression::negate(sb.clone())?} else {sb.clone()};
                    i = ComponentReference::crefDepth(cr.clone())?;
                    ia = ComponentReference::crefDepth(cra.clone())?;
                    Ok(mergeStartFixed1(intLt(ia.clone(), i.clone()), v.clone(), cr.clone(), sa.clone(), cra.clone(), e.clone(), soa.clone(), negate.clone(), (literal!(" both fixed and have start values ")).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVar)
}

fn startValueType(mut iExp: Option<Arc<DAE::Exp>>, mut iTy: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = (::match_deref::match_deref! { match &(iExp.clone()) {
        Some(e) => {
            e.clone()
        },
        None if (Types::isRealOrSubTypeReal(iTy.clone())?) => {
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        None if (Types::isIntegerOrSubTypeInteger(iTy.clone())?) => {
            Arc::new(DAE::Exp::ICONST { integer: 0 })
        },
        None if (Types::isBooleanOrSubTypeBoolean(iTy.clone())?) => {
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        None if (Types::isStringOrSubTypeString(iTy.clone())?) => {
            Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() })
        },
        None if (Types::isEnumerationOrSubTypeEnumeration(iTy.clone())?) => {
            Types::getNthEnumLiteral(iTy.clone(), 1)?
        },
        _ => {
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

fn mergeStartFixed1(mut b: bool, mut inVar: BackendDAE::Var, mut cr: Arc<DAE::ComponentRef>, mut sv: Arc<DAE::Exp>, mut cra: Arc<DAE::ComponentRef>, mut sva: Arc<DAE::Exp>, mut soa: Option<Arc<DAE::Exp>>, mut negate: bool, mut s4: ArcStr) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = (::match_deref::match_deref! { match &((b.clone(), soa.clone())) {
        (false, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s5: ArcStr = arcstr::literal!("");
            let mut s6: ArcStr = arcstr::literal!("");
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (if (negate.clone()) {literal!(" = -")} else {literal!(" = ")}).clone();
            s3 = (ComponentReferenceBasics::printComponentRefStr(cra.clone())?).clone();
            s5 = (ExpressionBasics::printExpStr(sv.clone())?).clone();
            s6 = (ExpressionBasics::printExpStr(sva.clone())?).clone();
            s = stringAppendList(list![(literal!("Alias variables ")).clone(), (s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (s4.clone()).clone(), (s5.clone()).clone(), (literal!(" != ")).clone(), (s6.clone()).clone(), (literal!(". Use value from ")).clone(), (s1.clone()).clone(), (literal!(".")).clone()]);
            Error::addMessage(Error::COMPILER_WARNING.clone(), list![(s.clone()).clone()])?;
            inVar.clone()
        },
        (true, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s5: ArcStr = arcstr::literal!("");
            let mut s6: ArcStr = arcstr::literal!("");
            let mut v: BackendDAE::Var;
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (if (negate.clone()) {literal!(" = -")} else {literal!(" = ")}).clone();
            s3 = (ComponentReferenceBasics::printComponentRefStr(cra.clone())?).clone();
            s5 = (ExpressionBasics::printExpStr(sv.clone())?).clone();
            s6 = (ExpressionBasics::printExpStr(sva.clone())?).clone();
            s = stringAppendList(list![(literal!("Alias variables ")).clone(), (s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (s4.clone()).clone(), (s5.clone()).clone(), (literal!(" != ")).clone(), (s6.clone()).clone(), (literal!(". Use value from ")).clone(), (s3.clone()).clone(), (literal!(".")).clone()]);
            Error::addMessage(Error::COMPILER_WARNING.clone(), list![(s.clone()).clone()])?;
            v = setVarStartValue(inVar.clone(), sva.clone())?;
            v = setVarStartOrigin(v.clone(), soa.clone())?;
            v.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

fn replaceCrefWithBindExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _, hs)) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut hs = (*hs).clone();
                    let false = (BaseHashSet::has(cr.clone(), hs.clone())?) else { bail!("pattern mismatch") };
                    let (BackendDAE::VAR { bindExp: Some(__pa0), .. }, _) = (getVarSingle(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                    e = __pa0.clone();
                    (hs, _, _, _, _) = BaseHashSet::add(cr.clone(), hs.clone())?;
                    let (__pa1, (_, _, __pa2)) = Expression::traverseExpBottomUp(e.clone(), Arc::new(replaceCrefWithBindExp), (vars.clone(), false, hs.clone()))?;
                    e = __pa1.clone();
                    hs = __pa2.clone();
                    Ok((e.clone(), (vars.clone(), true, hs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { .. }, (vars, _, hs)) => {
                    Ok((e.clone(), (vars.clone(), true, hs.clone())))
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

fn getNonZeroStart(mut mustBeEqual: bool, mut exp1: Arc<DAE::Exp>, mut so: Option<Arc<DAE::Exp>>, mut exp2: Arc<DAE::Exp>, mut sao: Option<Arc<DAE::Exp>>, mut globalKnownVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, Option<Arc<DAE::Exp>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outStartOrigin: Option<Arc<DAE::Exp>> = None;
    (outExp, outStartOrigin) = 'mc: {
        let __mc_input = mustBeEqual.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut i: i32 = 0;
            let mut ia: i32 = 0;
            let mut origin: Option<Arc<DAE::Exp>> = None;
            let true = (ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?) else { bail!("pattern mismatch") };
            i = startOriginToValue(so.clone())?;
            ia = startOriginToValue(sao.clone())?;
            origin = if (intGt(ia.clone(), i.clone())) {sao.clone()} else {so.clone()};
            Ok((exp1.clone(), origin.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let false = __mc_input.clone() else { bail!("nomatch") };
            let mut exp1_1: Arc<DAE::Exp>;
            let mut i: i32 = 0;
            let mut ia: i32 = 0;
            let mut origin: Option<Arc<DAE::Exp>> = None;
            i = startOriginToValue(so.clone())?;
            ia = startOriginToValue(sao.clone())?;
            let false = (intEq(i.clone(), ia.clone())) else { bail!("pattern mismatch") };
            (exp1_1, origin) = if (intGt(ia.clone(), i.clone())) {(exp2.clone(), sao.clone())} else {(exp1.clone(), so.clone())};
            Ok((exp1_1.clone(), origin.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut exp2_1: Arc<DAE::Exp>;
            let mut exp1_1: Arc<DAE::Exp>;
            let mut i: i32 = 0;
            let mut ia: i32 = 0;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut origin: Option<Arc<DAE::Exp>> = None;
            let (__pa0, (_, __pa1, _)) = Expression::traverseExpBottomUp(exp1.clone(), Arc::new(replaceCrefWithBindExp), (globalKnownVars.clone(), false, HashSet::emptyHashSet()))?;
            exp1_1 = __pa0.clone();
            b1 = __pa1.clone();
            let (__pa2, (_, __pa3, _)) = Expression::traverseExpBottomUp(exp2.clone(), Arc::new(replaceCrefWithBindExp), (globalKnownVars.clone(), false, HashSet::emptyHashSet()))?;
            exp2_1 = __pa2.clone();
            b2 = __pa3.clone();
            (exp1_1, _) = ExpressionSimplify::condsimplify(b1.clone(), exp1_1.clone())?;
            (exp2_1, _) = ExpressionSimplify::condsimplify(b2.clone(), exp2_1.clone())?;
            let true = (ExpressionBasics::expEqual(exp1_1.clone(), exp2_1.clone())?) else { bail!("pattern mismatch") };
            exp1_1 = if (b1.clone()) {exp1.clone()} else {exp2.clone()};
            i = startOriginToValue(so.clone())?;
            ia = startOriginToValue(sao.clone())?;
            origin = if (intGt(ia.clone(), i.clone())) {sao.clone()} else {so.clone()};
            Ok((exp1_1.clone(), origin.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outStartOrigin))
}

pub fn startOriginToValue(mut startOrigin: Option<Arc<DAE::Exp>>) -> Result<i32> {
    let mut i: i32 = 0;
    i = (::match_deref::match_deref! { match &(startOrigin.clone()) {
        None => 0,
        Some(Deref @ DAE::Exp::SCONST { string: Deref @ "undefined" }) => 1,
        Some(Deref @ DAE::Exp::SCONST { string: Deref @ "type" }) => 2,
        Some(Deref @ DAE::Exp::SCONST { string: Deref @ "binding" }) => 3,
        _ => bail!("match: no arm matched"),
    } });
    Ok(i)
}

pub fn mergeNominalAttribute(mut inAVar: BackendDAE::Var, mut inVar: BackendDAE::Var, mut negate: bool) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = 'mc: {
        let __mc_input = (inAVar.clone(), inVar.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut v, mut var) = __mc_input.clone() else { bail!("nomatch") };
            let mut var1: BackendDAE::Var;
            let mut e: Arc<DAE::Exp>;
            let mut e_1: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut esum: Arc<DAE::Exp>;
            let mut eaverage: Arc<DAE::Exp>;
            e = varNominalValue(v.clone())?;
            e1 = varNominalValue(var.clone())?;
            e_1 = if (negate.clone()) {Expression::negate(e.clone())?} else {e.clone()};
            esum = Expression::makeSum(list![e_1.clone(), e1.clone()])?;
            eaverage = Expression::expDiv(esum.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            (eaverage, _) = ExpressionSimplify::simplify(eaverage.clone())?;
            var1 = setVarNominalValue(var.clone(), eaverage.clone())?;
            Ok(var1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut v, mut var) = __mc_input.clone() else { bail!("nomatch") };
            let mut var1: BackendDAE::Var;
            let mut e: Arc<DAE::Exp>;
            let mut e_1: Arc<DAE::Exp>;
            e = varNominalValue(v.clone())?;
            e_1 = if (negate.clone()) {Expression::negate(e.clone())?} else {e.clone()};
            var1 = setVarNominalValue(var.clone(), e_1.clone())?;
            Ok(var1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok(inVar.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVar)
}

pub fn mergeMinMaxAttribute(mut inAVar: BackendDAE::Var, mut inVar: BackendDAE::Var, mut negate: bool) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = 'mc: {
        let __mc_input = (inAVar.clone(), inVar.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { values: ref attr, .. }, ref var @ BackendDAE::Var { values: ref attr1, .. }) = __mc_input.clone() else { bail!("nomatch") };
            let mut var1: BackendDAE::Var;
            let mut min1: Option<Arc<DAE::Exp>> = None;
            let mut min2: Option<Arc<DAE::Exp>> = None;
            let mut max1: Option<Arc<DAE::Exp>> = None;
            let mut max2: Option<Arc<DAE::Exp>> = None;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (min1, max1) = DAEUtil::getMinMaxValues(attr.clone());
            (min2, max2) = DAEUtil::getMinMaxValues(attr1.clone());
            cr = varCref(v.clone())?;
            cr1 = varCref(var.clone())?;
            (min1, max1) = mergeMinMax(negate.clone(), min1.clone(), min2.clone(), max1.clone(), max2.clone(), cr.clone(), cr1.clone())?;
            var1 = setVarMinMax(var.clone(), min1.clone(), max1.clone())?;
            Ok(var1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inVar.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVar)
}

pub fn getMinMaxAttribute(mut inVar: BackendDAE::Var) -> (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>) {
    let mut outMin: Option<Arc<DAE::Exp>> = None;
    let mut outMax: Option<Arc<DAE::Exp>> = None;
    (outMin, outMax) = DAEUtil::getMinMaxValues(inVar.values.clone());
    (outMin, outMax)
}

fn mergeMinMax(mut negate: bool, mut inMin1: Option<Arc<DAE::Exp>>, mut inMin2: Option<Arc<DAE::Exp>>, mut inMax1: Option<Arc<DAE::Exp>>, mut inMax2: Option<Arc<DAE::Exp>>, mut cr: Arc<DAE::ComponentRef>, mut cr1: Arc<DAE::ComponentRef>) -> Result<(Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>)> {
    let mut outMin: Option<Arc<DAE::Exp>> = None;
    let mut outMax: Option<Arc<DAE::Exp>> = None;
    outMin = if (negate.clone()) {Util::applyOption(inMin1.clone(), Arc::new(Expression::negate))} else {inMin1.clone()};
    outMax = if (negate.clone()) {Util::applyOption(inMax1.clone(), Arc::new(Expression::negate))} else {inMax1.clone()};
    outMin = mergeMin(outMin.clone(), inMin2.clone())?;
    outMax = mergeMax(outMax.clone(), inMax2.clone())?;
    checkMinMax(outMin.clone(), outMax.clone(), cr.clone(), cr1.clone(), negate.clone())?;
    Ok((outMin, outMax))
}

fn checkMinMax(mut inMin: Option<Arc<DAE::Exp>>, mut inMax: Option<Arc<DAE::Exp>>, mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>, mut negate: bool) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inMin.clone(), inMax.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(min), Some(max)) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut rmin: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rmax: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rmin = Expression::toReal(min.clone())?;
                    rmax = Expression::toReal(max.clone())?;
                    let true = (realGt(rmin.clone(), rmax.clone())) else { bail!("pattern mismatch") };
                    s1 = (ComponentReferenceBasics::printComponentRefStr(cr1.clone())?).clone();
                    s2 = (if (negate.clone()) {literal!(" = -")} else {literal!(" = ")}).clone();
                    s3 = (ComponentReferenceBasics::printComponentRefStr(cr2.clone())?).clone();
                    s4 = (ExpressionBasics::printExpStr(min.clone())?).clone();
                    s5 = (ExpressionBasics::printExpStr(max.clone())?).clone();
                    s = stringAppendList(list![(literal!("Alias variables ")).clone(), (s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (literal!(" with invalid limits min ")).clone(), (s4.clone()).clone(), (literal!(" > max ")).clone(), (s5.clone()).clone()]);
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![(s.clone()).clone()])?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn mergeMin(mut inMin1: Option<Arc<DAE::Exp>>, mut inMin2: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outMin: Option<Arc<DAE::Exp>> = None;
    outMin = (::match_deref::match_deref! { match &((inMin1.clone(), inMin2.clone())) {
        (Some(min1), Some(min2)) => {
            let mut min: Arc<DAE::Exp>;
            min = Expression::expMaxScalar(min1.clone(), min2.clone())?;
            (min, _) = ExpressionSimplify::simplify(min.clone())?;
            if (referenceEq(&min.clone(),&min1.clone())) {inMin1.clone()} else if (referenceEq(&min.clone(),&min2.clone())) {inMin2.clone()} else {Some(min.clone())}
        },
        (None, _) => {
            inMin2.clone()
        },
        (_, None) => {
            inMin1.clone()
        },
        _ => {
            inMin1.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMin)
}

fn mergeMax(mut inMax1: Option<Arc<DAE::Exp>>, mut inMax2: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outMax: Option<Arc<DAE::Exp>> = None;
    outMax = (::match_deref::match_deref! { match &((inMax1.clone(), inMax2.clone())) {
        (Some(max1), Some(max2)) => {
            let mut max: Arc<DAE::Exp>;
            max = Expression::expMinScalar(max1.clone(), max2.clone())?;
            (max, _) = ExpressionSimplify::simplify(max.clone())?;
            if (referenceEq(&max.clone(),&max1.clone())) {inMax1.clone()} else if (referenceEq(&max.clone(),&max2.clone())) {inMax2.clone()} else {Some(max.clone())}
        },
        (None, _) => {
            inMax2.clone()
        },
        (_, None) => {
            inMax1.clone()
        },
        _ => {
            inMax1.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMax)
}

pub fn calcAliasKey(mut inVar: BackendDAE::Var) -> Result<i32> {
    let mut i: i32 = 0;
    let mut b: bool = false;
    let mut d: i32 = 0;
    b = ComponentReference::isRecord(inVar.varName.clone());
    i = if (b.clone()) {-1} else {0};
    b = ComponentReference::isArrayElement(inVar.varName.clone());
    i = intAdd(i.clone(), if (b.clone()) {-1} else {0});
    b = isProtectedVar(inVar.clone());
    i = intAdd(i.clone(), if (b.clone()) {5} else {0});
    b = isVarConnector(inVar.clone());
    i = intAdd(i.clone(), if (b.clone()) {1} else {0});
    b = isDummyDerVar(inVar.clone());
    i = intAdd(i.clone(), if (b.clone()) {10} else {0});
    b = selfGeneratedVar(inVar.varName.clone());
    i = intAdd(i.clone(), if (b.clone()) {100} else {0});
    d = ComponentReference::crefDepth(inVar.varName.clone())?;
    i = i.clone() + d.clone();
    Ok(i)
}

pub fn selfGeneratedVar(mut inCref: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool = StringUtil::startsWith((ComponentReference::crefStr(inCref.clone()).unwrap()).clone(), (literal!("$")).clone());
    b
}

pub fn varStateSelectPrioAlias(mut v: BackendDAE::Var) -> Result<i32> {
    let mut prio: i32 = 0;
    let mut ss: DAE::StateSelect = DAE::StateSelect::ALWAYS;
    let mut knownDer: bool = false;
    ss = varStateSelect(v.clone());
    prio = stateSelectToInteger(ss.clone())?;
    knownDer = varHasStateDerivative(v.clone());
    prio = prio.clone() * 2;
    prio = if (knownDer.clone()) {prio.clone() + 1} else {prio.clone()};
    Ok(prio)
}

pub fn stateSelectToInteger(mut inStateSelect: DAE::StateSelect) -> Result<i32> {
    let mut prio: i32 = 0;
    prio = (match inStateSelect.clone() {
        DAE::StateSelect::NEVER => -1,
        DAE::StateSelect::AVOID => 0,
        DAE::StateSelect::DEFAULT => 1,
        DAE::StateSelect::PREFER => 2,
        DAE::StateSelect::ALWAYS => 3,
        _ => bail!("match: no arm matched"),
    });
    Ok(prio)
}

pub fn transformXToXd(mut inVar: BackendDAE::Var) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var;
    outVar = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. } => {
            outVar = inVar.clone();
            outVar.varName = ComponentReference::crefPrefixDer(inVar.varName.clone());
            outVar.varKind = crate::BackendDAE::VarKind::STATE_DER;
            outVar.clone()
        },
        _ => inVar.clone(),
    });
    outVar
}

pub fn setStateIndex(mut v1: BackendDAE::Var, mut idx: i32) -> Result<BackendDAE::Var> {
    let mut v2: BackendDAE::Var = v1.clone();
    let mut derName: Option<Arc<DAE::ComponentRef>> = None;
    let mut natural: bool = false;
    if isStateVar(v1.clone()) {
        let BackendDAE::STATE { index: _, derName: __pa0, natural: __pa1 } = (getVarKind(v1.clone())) else { bail!("pattern mismatch") };
        derName = __pa0.clone();
        natural = __pa1.clone();
        v2 = setVarKind(v1.clone(), BackendDAE::VarKind::STATE { index: idx.clone(), derName: derName.clone(), natural: natural.clone() })?;
    }
    Ok(v2)
}

pub fn isRecordVar(mut inVar: BackendDAE::Var) -> bool {
    let mut isRec: bool = ComponentReference::traverseCref(inVar.varName.clone(), Arc::new(ComponentReference::crefIsRec), false).unwrap();
    isRec
}

pub fn varExp(mut inVar: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Expression::crefToExp(inVar.varName.clone())?;
    Ok(outExp)
}

pub fn varExp2(mut inVar: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: 1, .. }, .. } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Expression::crefExp(inVar.varName.clone())?;
            Expression::expDer(exp.clone())
        },
        _ => {
            Expression::crefExp(inVar.varName.clone())?
        },
    });
    Ok(outExp)
}

pub fn scalarizeVariables(mut vars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut vars: BackendDAE::Variables = vars;
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut new_var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    var_lst = varList(vars.clone())?;
    for mut var in &*var_lst.clone() {
        let mut var = var.clone();
        new_var_lst = scalarizeVar(var.clone(), new_var_lst.clone())?;
    }
    vars = listVar(new_var_lst.clone().reverse());
    Ok(vars)
}

pub fn scalarizeVar(mut var: BackendDAE::Var, mut scalar_vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut scalar_vars: Arc<metamodelica::List<BackendDAE::Var>> = scalar_vars;
    let mut scalar_crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut scalar_var: BackendDAE::Var;
    if Types::isArray(var.varType.clone()) {
        scalar_crefs = ComponentReference::expandCref(var.varName.clone(), false)?;
        for mut cref in &*scalar_crefs.clone() {
            let mut cref = cref.clone();
            scalar_var = copyVarNewName(cref.clone(), var.clone());
            scalar_var.varType = ComponentReference::crefTypeFull(cref.clone())?;
            scalar_vars = cons(scalar_var.clone(), scalar_vars.clone());
        }
    } else {
        scalar_vars = cons(var.clone(), scalar_vars.clone());
    }
    Ok(scalar_vars)
}

