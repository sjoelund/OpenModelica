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
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util_datatypes_basic::List;

pub fn symSolver(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Option<BackendDAE::InlineData>> {
    let mut inlineData: Option<BackendDAE::InlineData> = None;
    if Flags::getConfigEnum(Flags::SYM_SOLVER.clone())? > 0 {
        inlineData = Some(symSolverWork(inDAE.clone())?);
    } else {
        inlineData = None;
    }
    Ok(inlineData)
}

fn symSolverWork(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<BackendDAE::InlineData> {
    let mut inlineData: BackendDAE::InlineData;
    let mut osystlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut syst_: Arc<BackendDAE::EqSystem>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut tmpv: BackendDAE::Var;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut sharedIn: Arc<BackendDAE::Shared>;
    let mut localInline: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut knownVariables: BackendDAE::Variables;
    let mut saveKnGlobalVars: BackendDAE::Variables;
    let mut inlineBDAE: Arc<BackendDAE::BackendDAE>;
    let mut execbool: bool = false;
    localInline = BackendDAEUtil::copyEqSystems(inDAE.eqs.clone())?;
    knownVariables = BackendVariable::emptyVars(BackendDAEUtil::daeSize(inDAE.clone()));
    inlineData = BackendDAE::InlineData { inlineSystems: localInline.clone(), knownVariables: knownVariables.clone() };
    cref = ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::symSolverDT)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
    tmpv = BackendVariable::makeVar(cref.clone());
    tmpv = BackendVariable::setBindExp(tmpv.clone(), Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })));
    inlineData.knownVariables = BackendVariable::addVars(list![tmpv.clone()], inlineData.knownVariables.clone());
    knownVariables = inlineData.knownVariables.clone();
    for mut syst in &*inlineData.inlineSystems.clone() {
        let mut syst = syst.clone();
        (syst_, knownVariables) = symSolverUpdateSyst(syst.clone(), knownVariables.clone())?;
        osystlst = cons(syst_.clone(), osystlst.clone());
    }
    inlineData.knownVariables = knownVariables.clone();
    shared = inDAE.shared.clone();
    saveKnGlobalVars = shared.globalKnownVars.clone();
    knownVariables = BackendVariable::addVariables(shared.globalKnownVars.clone(), knownVariables.clone())?;
    assign_field!(
        shared.globalKnownVars = knownVariables.clone(),
        shared.backendDAEType = crate::BackendDAE::BackendDAEType::INLINESYSTEM
    );
    inlineBDAE = BackendDAE::DAE(osystlst.clone(), shared.clone())?;
    execbool = FlagsUtil::disableDebug(Flags::EXEC_STAT.clone())?;
    if Flags::isSet(Flags::DUMP_INLINE_SOLVER.clone())? {
        BackendDump::bltdump((literal!("Generated inline system:")).clone(), inlineBDAE.clone())?;
    }
    inlineBDAE = BackendDAEUtil::getSolvedSystemforJacobians(inlineBDAE.clone(), list![(literal!("removeEqualRHS")).clone(), (literal!("removeSimpleEquations")).clone(), (literal!("evalFunc")).clone()], None, None, list![(literal!("inlineArrayEqn")).clone(), (literal!("constantLinearSystem")).clone(), (literal!("solveSimpleEquations")).clone(), (literal!("tearingSystem")).clone(), (literal!("calculateStrongComponentJacobians")).clone(), (literal!("removeConstants")).clone(), (literal!("simplifyTimeIndepFuncCalls")).clone()])?;
    let _ = FlagsUtil::set(Flags::EXEC_STAT.clone(), execbool.clone())?;
    if Flags::isSet(Flags::DUMP_INLINE_SOLVER.clone())? {
        BackendDump::bltdump((literal!("Final inline systems:")).clone(), inlineBDAE.clone())?;
    }
    if Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone())? || Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? || Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())? {
        BackendDump::dumpCompShort(inlineBDAE.clone())?;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inlineBDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: _, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    localInline = __pa0.clone();
    inlineData.inlineSystems = localInline.clone();
    assign_field!(shared.globalKnownVars = saveKnGlobalVars.clone());
    Ok(inlineData)
}

fn symSolverUpdateSyst(mut iSyst: Arc<BackendDAE::EqSystem>, mut inKnVars: BackendDAE::Variables) -> Result<(Arc<BackendDAE::EqSystem>, BackendDAE::Variables)> {
    let mut oSyst: Arc<BackendDAE::EqSystem>;
    let mut oKnVars: BackendDAE::Variables = inKnVars.clone();
    let mut equOptArr: metamodelica::Array<Option<Arc<BackendDAE::Equation>>>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    oSyst = (::match_deref::match_deref! { match &(iSyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. } => {
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            crlst = metamodelica::nil();
            for mut i in 1..=ExpandableArray::getLastUsedIndex(eqns.clone()) {
                if ExpandableArray::occupied(i.clone(), eqns.clone()) {
                    eqn = ExpandableArray::get(i.clone(), eqns.clone())?;
                    let (__pa0, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(symSolverUpdateEqn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables))> + 'static>), (crlst.clone(), syst.orderedVars.clone()))?;
                    eqn = __pa0.clone();
                    crlst = __pa1.clone();
                    ExpandableArray::update(i.clone(), eqn.clone(), eqns.clone())?;
                }
            }
            (vars, oKnVars) = symSolverState(vars.clone(), inKnVars.clone(), crlst.clone())?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = eqns.clone()
            );
            BackendDAEUtil::clearEqSyst(syst.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oSyst, oKnVars))
}

// function changes every state variable to algebraic variable
fn symSolverState(mut vars: BackendDAE::Variables, mut knvars: BackendDAE::Variables, mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables)> {
    let mut ovars: BackendDAE::Variables = vars.clone();
    let mut oknvars: BackendDAE::Variables = knvars.clone();
    let mut idx: i32 = 0;
    let mut oldCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: BackendDAE::Var;
    for mut cref in &*crlst.clone() {
        let mut cref = cref.clone();
        (var, idx) = BackendVariable::getVar2(cref.clone(), ovars.clone())?;
        ovars = BackendVariable::setVarKindForVar(idx.clone(), crate::BackendDAE::VarKind::ALG_STATE, ovars.clone())?;
        oldCref = ComponentReference::appendStringLastIdent((literal!("$Old")).clone(), cref.clone())?;
        var = BackendVariable::copyVarNewName(oldCref.clone(), var.clone());
        var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::ALG_STATE_OLD)?;
        oknvars = BackendVariable::addVars(list![var.clone()], oknvars.clone());
    }
    Ok((ovars, oknvars))
}

fn symSolverUpdateEqn(mut inExp: Arc<DAE::Exp>, mut inTl: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables);
    let mut orderedVars: BackendDAE::Variables;
    let mut inTpl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (inTpl, orderedVars) = inTl.clone();
    if Flags::getConfigEnum(Flags::SYM_SOLVER.clone())? > 1 {
        let (__pa0, (__pa1, __pa2)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(symSolverUpdateStates) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables))> + 'static>), (inTpl.clone(), orderedVars.clone()))?;
        outExp = __pa0.clone();
        inTpl = __pa1.clone();
        orderedVars = __pa2.clone();
    } else {
        (outExp, inTpl) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(symSolverUpdateDer) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), inTpl.clone())?;
    }
    outTpl = (inTpl.clone(), orderedVars.clone());
    Ok((outExp, outTpl))
}

fn symSolverUpdateStates(mut inExp: Arc<DAE::Exp>, mut inTl: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTl: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables);
    let mut inTpl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut orderedVars: BackendDAE::Variables;
    (inTpl, orderedVars) = inTl.clone();
    (outExp, outTl) = (::match_deref::match_deref! { match &((inTpl.clone(), inExp.clone())) {
        (cr_lst, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }) => {
            let mut e2: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            e2 = Expression::crefExp(ComponentReference::appendStringLastIdent((literal!("$Old")).clone(), cr.clone())?)?;
            e3 = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::symSolverDT)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            cont = false;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e3.clone() }), (List::unionElt(cr.clone(), cr_lst.clone()), orderedVars.clone()))
        },
        (cr_lst, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            let mut e: Arc<DAE::Exp>;
            let mut cr_lst = (*cr_lst).clone();
            (e, cr_lst) = symSolverAppendStringToStates(cr.clone(), cr_lst.clone(), orderedVars.clone())?;
            (e.clone(), (cr_lst.clone(), orderedVars.clone()))
        },
        _ => {
            (inExp.clone(), inTl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTl))
}

fn symSolverAppendStringToStates(mut inCr: Arc<DAE::ComponentRef>, mut incr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut orderedVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp> = Expression::crefExp(inCr.clone())?;
    let mut outcr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = incr_lst.clone();
    if BackendVariable::isState(inCr.clone(), orderedVars.clone())? {
        outExp = Expression::crefExp(ComponentReference::appendStringLastIdent((literal!("$Old")).clone(), inCr.clone())?)?;
        outcr_lst = List::unionElt(inCr.clone(), incr_lst.clone());
    }
    Ok((outExp, outcr_lst))
}

// function changes call "der" to difference quotient
fn symSolverUpdateDer(mut inExp: Arc<DAE::Exp>, mut inTpl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTpl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inTpl.clone(), inExp.clone())) {
        (cr_lst, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }) => {
            let mut e2: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            e2 = Expression::crefExp(ComponentReference::appendStringLastIdent((literal!("$Old")).clone(), cr.clone())?)?;
            e3 = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::symSolverDT)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e3.clone() }), List::unionElt(cr.clone(), cr_lst.clone()))
        },
        _ => {
            (inExp.clone(), inTpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

