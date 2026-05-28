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
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Differentiate;
use crate::ExpressionSolve;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util_datatypes_basic::List;

pub fn createDynamicOptimization(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    shared = dae.shared.clone();
    let __pa0 = ::match_deref::match_deref! { match &(dae.eqs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    syst = __pa0.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa2, orderedVars: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqns = __pa2.clone();
    vars = __pa3.clone();
    (vars, eqns, shared) = addOptimizationVarsEqns(vars.clone(), eqns.clone(), shared.clone())?;
    assign_field!(
        syst.orderedVars = vars.clone(),
        syst.orderedEqs = eqns.clone()
    );
    assign_field!(
        dae.eqs = list![syst.clone()],
        dae.shared = shared.clone()
    );
    Ok(dae)
}

fn addOptimizationVarsEqns(mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>)> {
    let mut vars: BackendDAE::Variables = vars;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = eqns;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut mayer: Option<Arc<DAE::Exp>> = None;
    let mut lagrange: Option<Arc<DAE::Exp>> = None;
    let mut startTimeE: Option<Arc<DAE::Exp>> = None;
    let mut finalTimeE: Option<Arc<DAE::Exp>> = None;
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut classAttrs: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>> = metamodelica::nil();
    let mut constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut inOptimicaFlag: bool = Config::acceptOptimicaGrammar()?;
    let mut inDynOptimization: bool = Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())?;
    let debug: bool = false;
    classAttrs = shared.classAttrs.clone();
    constraints = shared.constraints.clone();
    globalKnownVars = shared.globalKnownVars.clone();
    eqnsLst = metamodelica::nil();
    if !(inOptimicaFlag.clone() || inDynOptimization.clone()) {
        println!("{}", (literal!("Something going wrong for postOptModul=createDynamicOptimization. Check your flags. You need -g=DynOpt or -g=Optimica!\n")).clone());
        bail!("fail");
    }
    FlagsUtil::setConfigEnum(Flags::GRAMMAR.clone(), Flags::OPTIMICA.clone())?;
    (mayer, lagrange, startTimeE, finalTimeE) = getOptimicaArgs(classAttrs.clone());
    varlst = BackendVariable::varList(globalKnownVars.clone())?;
    addTimeGrid(varlst.clone(), globalKnownVars.clone())?;
    varlst = listAppend(varlst.clone(), BackendVariable::varList(vars.clone())?);
    (vars, eqnsLst, mayer) = joinObjectFun(makeObject((arcstr::literal!(BackendDAE::optimizationMayerTermName)).clone(), (std::sync::Arc::new(fnptr!(findMayerTerm, Arc<metamodelica::List<BackendDAE::Var>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Option<Arc<DAE::Exp>>> + 'static>), varlst.clone(), mayer.clone())?, vars.clone(), eqnsLst.clone())?;
    (vars, eqnsLst, lagrange) = joinObjectFun(makeObject((arcstr::literal!(BackendDAE::optimizationLagrangeTermName)).clone(), (std::sync::Arc::new(fnptr!(findLagrangeTerm, Arc<metamodelica::List<BackendDAE::Var>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Option<Arc<DAE::Exp>>> + 'static>), varlst.clone(), lagrange.clone())?, vars.clone(), eqnsLst.clone())?;
    (vars, eqnsLst) = joinConstraints(constraints.clone(), (literal!("$con$")).clone(), crate::BackendDAE::VarKind::OPT_CONSTR, globalKnownVars.clone(), varlst.clone(), vars.clone(), eqnsLst.clone(), (std::sync::Arc::new(BackendVariable::hasConTermAnno) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
    (vars, eqnsLst) = joinConstraints(metamodelica::nil(), (literal!("$finalCon$")).clone(), crate::BackendDAE::VarKind::OPT_FCONSTR, globalKnownVars.clone(), varlst.clone(), vars.clone(), eqnsLst.clone(), (std::sync::Arc::new(BackendVariable::hasFinalConTermAnno) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
    FlagsUtil::setConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone(), true)?;
    assign_field!(shared.classAttrs = list![Arc::new(DAE::ClassAttributes { objetiveE: mayer.clone(), objectiveIntegrandE: lagrange.clone(), startTimeE: startTimeE.clone(), finalTimeE: finalTimeE.clone() })]);
    if debug.clone() {
        println!("{}", (literal!("\neqs")).clone());
        BackendDump::printEquationList(eqnsLst.clone());
    }
    eqns = BackendEquation::addList(eqnsLst.clone(), eqns.clone())?;
    Ok((vars, eqns, shared))
}

fn getOptimicaArgs(mut inClassAttr: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>) -> (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>) {
    let mut mayer: Option<Arc<DAE::Exp>> = None;
    let mut lagrange: Option<Arc<DAE::Exp>> = None;
    let mut startTimeE: Option<Arc<DAE::Exp>> = None;
    let mut finalTimeE: Option<Arc<DAE::Exp>> = None;
    (mayer, lagrange, startTimeE, finalTimeE) = (::match_deref::match_deref! { match &(inClassAttr.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::ClassAttributes { finalTimeE: finalTimeE_, startTimeE: startTimeE_, objectiveIntegrandE: lagrange_, objetiveE: mayer_ }, tail: Deref @ metamodelica::List::Nil } => {
            (mayer_.clone(), lagrange_.clone(), startTimeE_.clone(), finalTimeE_.clone())
        },
        _ => {
            (None, None, None, None)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (mayer, lagrange, startTimeE, finalTimeE)
}

fn addTimeGrid(mut varlst: Arc<metamodelica::List<BackendDAE::Var>>, mut iv: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut ov: BackendDAE::Variables = iv.clone();
    let mut tG: Arc<metamodelica::List<BackendDAE::Var>> = findTimeGrid(varlst.clone());
    let mut ind: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if !(tG.clone().is_empty()) {
        ind = BackendVariable::getVarIndexFromVars(tG.clone(), ov.clone());
        for mut i in &*ind.clone() {
            let mut i = i.clone();
            ov = BackendVariable::setVarKindForVar(i.clone(), crate::BackendDAE::VarKind::OPT_TGRID, ov.clone())?;
        }
    }
    Ok(ov)
}

fn joinConstraints(mut inConstraint: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut name: ArcStr, mut conKind: BackendDAE::VarKind, mut globalKnownVars: BackendDAE::Variables, mut varlst: Arc<metamodelica::List<BackendDAE::Var>>, mut vars: BackendDAE::Variables, mut e: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut findCon: Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>;

    let mut ovars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oe: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    constraints = addConstraints(varlst.clone(), inConstraint.clone(), findCon.clone())?;
    (ovars, oe) = addOptimizationVarsEqns2(constraints.clone(), 1, vars.clone(), e.clone(), globalKnownVars.clone(), (name.clone()).clone(), conKind.clone())?;
    Ok((ovars, oe))
}

fn joinObjectFun(mut obj: (BackendDAE::Var, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Option<Arc<DAE::Exp>>), mut vars: BackendDAE::Variables, mut e: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Option<Arc<DAE::Exp>>)> {
    let mut ovars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oe: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut objExp: Option<Arc<DAE::Exp>> = None;
    (ovars, oe, objExp) = (::match_deref::match_deref! { match &(obj.clone()) {
        (_, Deref @ metamodelica::List::Nil, _) => {
            (vars.clone(), e.clone(), None)
        },
        (v, e_, e1) => {
            (BackendVariable::addNewVar(v.clone(), vars.clone())?, listAppend(e_.clone(), e.clone()), e1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ovars, oe, objExp))
}

fn makeObject(mut name: ArcStr, mut findObj: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Option<Arc<DAE::Exp>>> + 'static>, mut varlst: Arc<metamodelica::List<BackendDAE::Var>>, mut optimicaExp: Option<Arc<DAE::Exp>>) -> Result<(BackendDAE::Var, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Option<Arc<DAE::Exp>>)> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Option<Arc<DAE::Exp>>> + 'static>;

    let mut outTpl: (BackendDAE::Var, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Option<Arc<DAE::Exp>>);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut annoObj: Option<Arc<DAE::Exp>> = None;
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut e: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (cr, v) = makeVar((name.clone()).clone());
    annoObj = findObj(varlst.clone())?;
    annoObj = mergeObjectVars(annoObj.clone(), optimicaExp.clone())?;
    e = BackendEquation::generateSolvedEqnsfromOption(cr.clone(), annoObj.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone());
    outTpl = (v.clone(), e.clone(), annoObj.clone());
    Ok(outTpl)
}

fn makeVar(mut name: ArcStr) -> (Arc<DAE::ComponentRef>, BackendDAE::Var) {
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    cr = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
    v = BackendDAE::Var { varName: cr.clone(), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: Some(crate::BackendDAE::TearingSelect::AVOID), hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    (cr, v)
}

fn addOptimizationVarsEqns1(mut constraintLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inI: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut globalKnownVars: BackendDAE::Variables, mut prefConCrefName: ArcStr, mut conKind: BackendDAE::VarKind) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut i: i32 = inI.clone();
    let mut dummyVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut conEqn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut conCrefName: ArcStr = arcstr::literal!("");
    for mut elem in &*constraintLst.clone() {
        let mut elem = elem.clone();
        match '__try0: {
            conCrefName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefConCrefName.clone()); __mm_s.push_str(&*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(Expression::expCref(elem.clone())?), '__try0)); ArcStr::from(__mm_s) }).clone();
            Ok::<_, anyhow::Error>((conCrefName.clone(),))
        } {
            Ok((__try0_o0,)) => {
                conCrefName = __try0_o0;
            }
            Err(_) => {
                conCrefName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefConCrefName.clone()); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone();
                i = i.clone() + 1;
            }
        }
        (conEqn, dummyVar) = BackendEquation::generateResidualFromRelation((conCrefName.clone()).clone(), elem.clone(), DAE::emptyElementSource().clone(), outVars.clone(), globalKnownVars.clone(), conKind.clone())?;
        outVars = BackendVariable::addNewVar(dummyVar.clone(), outVars.clone())?;
        outEqns = listAppend(conEqn.clone(), outEqns.clone());
    }
    Ok((outVars, outEqns))
}

fn addOptimizationVarsEqns2(mut inConstraint: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut inI: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut globalKnownVars: BackendDAE::Variables, mut prefConCrefName: ArcStr, mut conKind: BackendDAE::VarKind) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (outVars, outEqns) = (::match_deref::match_deref! { match &(inConstraint.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst }, tail: Deref @ metamodelica::List::Nil } => {
            let mut e: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut v: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            (v, e) = addOptimizationVarsEqns1(constraintLst.clone(), inI.clone(), inVars.clone(), inEqns.clone(), globalKnownVars.clone(), (prefConCrefName.clone()).clone(), conKind.clone())?;
            (v.clone(), e.clone())
        },
        _ => {
            (inVars.clone(), inEqns.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVars, outEqns))
}

fn findMayerTerm(mut varlst: Arc<metamodelica::List<BackendDAE::Var>>) -> Option<Arc<DAE::Exp>> {
    let mut mayer: Option<Arc<DAE::Exp>> = findObjTerm(varlst.clone(), (std::sync::Arc::new(BackendVariable::hasMayerTermAnno) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>)).unwrap();
    mayer
}

fn findLagrangeTerm(mut varlst: Arc<metamodelica::List<BackendDAE::Var>>) -> Option<Arc<DAE::Exp>> {
    let mut lagrange: Option<Arc<DAE::Exp>> = findObjTerm(varlst.clone(), (std::sync::Arc::new(BackendVariable::hasLagrangeTermAnno) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>)).unwrap();
    lagrange
}

fn findTimeGrid(mut varlst: Arc<metamodelica::List<BackendDAE::Var>>) -> Arc<metamodelica::List<BackendDAE::Var>> {
    let mut timeGrids: Arc<metamodelica::List<BackendDAE::Var>> = List::select(varlst.clone(), (std::sync::Arc::new(BackendVariable::hasTimeGridAnno) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    timeGrids
}

fn findObjTerm(mut InVarlst: Arc<metamodelica::List<BackendDAE::Var>>, mut findObjTermFun: Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>) -> Result<Option<Arc<DAE::Exp>>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>;

    let mut objeExp: Option<Arc<DAE::Exp>> = None;
    let mut e: Arc<DAE::Exp>;
    let mut nom: Arc<DAE::Exp>;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = List::select(InVarlst.clone(), findObjTermFun.clone());
    for mut v in &*varlst.clone() {
        let mut v = v.clone();
        nom = BackendVariable::getVarNominalValue(v.clone());
        cr = BackendVariable::varCref(v.clone())?;
        e = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_REAL_DEFAULT().clone() });
        e = Expression::expDiv(e.clone(), nom.clone())?;
        objeExp = mergeObjectVars(objeExp.clone(), Some(e.clone()))?;
    }
    Ok(objeExp)
}

fn mergeObjectVars(mut inmayer1: Option<Arc<DAE::Exp>>, mut inmayer2: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut mayer: Option<Arc<DAE::Exp>> = None;
    mayer = (::match_deref::match_deref! { match &((inmayer1.clone(), inmayer2.clone())) {
        (Some(e1), Some(e2)) => {
            let mut e3: Arc<DAE::Exp>;
            e3 = Expression::expAdd(e1.clone(), e2.clone())?;
            Some(e3.clone())
        },
        (None, Some(_)) => {
            inmayer2.clone()
        },
        (_, None) => {
            inmayer1.clone()
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(mayer)
}

fn addConstraints(mut InVarlst: Arc<metamodelica::List<BackendDAE::Var>>, mut inConstraint: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut findCon: Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Arc<DAE::Constraint>>>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>;

    let mut outConstraint: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut constraintLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    constraintLst = (::match_deref::match_deref! { match &(inConstraint.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: constraintLst_ }, tail: Deref @ metamodelica::List::Nil } => {
            constraintLst_.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    varlst = List::select(InVarlst.clone(), findCon.clone());
    constraintLst = addConstraints2(constraintLst.clone(), varlst.clone())?;
    outConstraint = list![Arc::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: constraintLst.clone() })];
    Ok(outConstraint)
}

fn addConstraints2(mut inConstraintLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inVarlst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outConstraintLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = inConstraintLst.clone();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    for mut v in &*inVarlst.clone() {
        let mut v = v.clone();
        cr = BackendVariable::varCref(v.clone())?;
        e = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_REAL_DEFAULT().clone() });
        outConstraintLst = cons(e.clone(), outConstraintLst.clone());
    }
    Ok(outConstraintLst)
}

// =============================================================================
// section for preOptModule >>inputDerivativesForDynOpt<<
//
// check for derivatives of inputs and replace (only for dyn. optimization)
// =============================================================================
pub fn inputDerivativesForDynOpt(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    if Config::acceptOptimicaGrammar()? || Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())? {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(inputDerivativesForDynOptWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

fn inputDerivativesForDynOptWork(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool = false;
    (osyst, outChanged) = ({
        let mut idercr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        let mut icr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut outShared: Arc<BackendDAE::Shared> = outShared.clone();
                    vars = BackendVariable::daeGlobalKnownVars(outShared.clone());
                    (_, idercr, icr, varLst) = BackendDAEUtil::traverseBackendDAEExpsEqns(orderedEqs.clone(), (std::sync::Arc::new(traverserinputDerivativesForDynOpt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>))> + 'static>), (vars.clone(), idercr.clone(), icr.clone(), varLst.clone()))?;
                    if idercr.clone().is_empty() {
                        bail!("fail");
                    }
                    varLst = BackendVariable::setVarsKind(varLst.clone(), crate::BackendDAE::VarKind::OPT_INPUT_WITH_DER);
                    for mut v in &*varLst.clone() {
                        let mut v = v.clone();
                        outShared = BackendVariable::addGlobalKnownVarDAE(v.clone(), outShared.clone())?;
                    }
                    varLst = List::map(idercr.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::makeVar, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<BackendDAE::Var> + 'static>));
                    varLst = List::map1(varLst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::setVarDirection, BackendDAE::Var, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, DAE::VarDirection) -> Result<BackendDAE::Var> + 'static>), openmodelica_frontend_types::DAE::VarDirection::INPUT);
                    for mut v in &*varLst.clone() {
                        let mut v = v.clone();
                        v = BackendVariable::setVarKind(v.clone(), crate::BackendDAE::VarKind::OPT_INPUT_DER)?;
                        outShared = BackendVariable::addGlobalKnownVarDAE(v.clone(), outShared.clone())?;
                    }
                    BackendVariable::daeGlobalKnownVars(outShared.clone());
                    Ok((isyst.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inChanged.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok((osyst, outShared, outChanged))
}

fn traverserinputDerivativesForDynOpt(mut inExp: Arc<DAE::Exp>, mut itpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>))> {
    let mut e: Arc<DAE::Exp>;
    let mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>);
    (e, tpl) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traverserExpinputDerivativesForDynOpt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>))> + 'static>), itpl.clone())?;
    Ok((e, tpl))
}

fn traverserExpinputDerivativesForDynOpt(mut inExp: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), tpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, lst, lst1, varLst)) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    let true = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setHideResult(var.clone(), Some(Arc::new(DAE::Exp::BCONST { bool: true })));
                    cr1 = ComponentReference::prependStringCref((literal!("$TMP$DER$P")).clone(), cr.clone())?;
                    e = Expression::crefExp(cr1.clone())?;
                    Ok((e.clone(), true, (vars.clone(), List::unionElt(cr1.clone(), lst.clone()), List::unionElt(cr.clone(), lst1.clone()), List::unionElt(var.clone(), varLst.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

// =============================================================================
// section for postOptModule >>extendDynamicOptimization<<
//
// transform loops from DAE in constraints for optimizer
// - bigger NLP
// - don't solve loop in each step
// - cheaper jacobians
// =============================================================================
pub fn removeLoops(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    if !(Flags::getConfigString(Flags::LOOP2CON.clone())? == literal!("none")) {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(findLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

fn findLoops(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outChanged: bool = false;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    (osyst, outShared, outChanged) = findLoops1(isyst.clone(), inShared.clone(), comps.clone(), inChanged.clone())?;
    Ok((osyst, outShared, outChanged))
}

fn findLoops1(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inchanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut changed: bool = inchanged.clone();
    let mut l2p_all: bool = Flags::getConfigString(Flags::LOOP2CON.clone())? == literal!("all");
    let mut l2p_nl: bool = false;
    let mut l2p_l: bool = false;
    if l2p_all.clone() {
        l2p_l = true;
    } else {
        l2p_nl = Flags::getConfigString(Flags::LOOP2CON.clone())? == literal!("noLin");
        l2p_l = !(l2p_nl.clone());
    }
    for mut comp in &*inComps.clone() {
        let mut comp = comp.clone();
        (osyst, oshared) = removeLoopsWork(osyst.clone(), oshared.clone(), comp.clone(), l2p_all.clone(), l2p_l.clone())?;
    }
    Ok((osyst, oshared, changed))
}

fn removeLoopsWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut icomp: Arc<BackendDAE::StrongComponent>, mut l2p_all: bool, mut l2p_l: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    (osyst, oshared) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone(), icomp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType, vars: vindx, eqns: eindex, .. }) => {
                    if !((l2p_all.clone() || if (l2p_l.clone()) {isConstOrlinear(jacType.clone())} else {!(isConstOrlinear(jacType.clone()))})) { bail!("guard") }
                    let mut syst = (*syst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut vars = (*vars).clone();
                    let mut shared = (*shared).clone();
                    (eqns, vars, shared) = res2Con(eqns.clone(), vars.clone(), eindex.clone(), vindx.clone(), shared.clone())?;
                    assign_field!(
                        syst.orderedEqs = eqns.clone(),
                        syst.orderedVars = vars.clone()
                    );
                    Ok((BackendDAEUtil::clearEqSyst(syst.clone())?, shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, shared, Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear, strictTearingSet: BackendDAE::TearingSet { tearingvars: vindx, residualequations: eindex, .. }, .. }) => {
                    if !((l2p_all.clone() || if (l2p_l.clone()) {linear.clone()} else {!(linear.clone())})) { bail!("guard") }
                    let mut syst = (*syst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut vars = (*vars).clone();
                    let mut shared = (*shared).clone();
                    (eqns, vars, shared) = res2Con(eqns.clone(), vars.clone(), eindex.clone(), vindx.clone(), shared.clone())?;
                    assign_field!(
                        syst.orderedEqs = eqns.clone(),
                        syst.orderedVars = vars.clone()
                    );
                    Ok((BackendDAEUtil::clearEqSyst(syst.clone())?, shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, shared @ Deref @ BackendDAE::Shared { functionTree: funcs, .. }, Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: vindx_, eqn: eindex_ }) => {
                    if !((l2p_all.clone() || !(l2p_l.clone()))) { bail!("guard") }
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut varexp: Arc<DAE::Exp>;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut syst = (*syst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut vars = (*vars).clone();
                    let mut shared = (*shared).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::get(eqns.clone(), eindex_.clone())) {
                        Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    e1 = __pa1.clone();
                    let ref __pa3 @ BackendDAE::VAR { varName: ref __pa2, .. } = (BackendVariable::getVarAt(vars.clone(), vindx_.clone())?) else { bail!("pattern mismatch") };
                    cr = __pa2.clone();
                    v = __pa3.clone();
                    varexp = Expression::crefExp(cr.clone())?;
                    varexp = if (BackendVariable::isStateVar(v.clone())) {Expression::expDer(varexp.clone())} else {varexp.clone()};
                    if '__try4: {
                        unwrap_break_err!(ExpressionSolve::solve2(e1.clone(), e2.clone(), varexp.clone(), Some(funcs.clone()), None, true, false), '__try4);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (eqns, vars, shared) = res2Con(eqns.clone(), vars.clone(), list![eindex_.clone()], list![vindx_.clone()], shared.clone())?;
                    assign_field!(
                        syst.orderedEqs = eqns.clone(),
                        syst.orderedVars = vars.clone()
                    );
                    Ok((BackendDAEUtil::clearEqSyst(syst.clone())?, shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared))
}

fn isConstOrlinear(mut jacType: BackendDAE::JacobianType) -> bool {
    let mut b: bool = false;
    b = (match jacType.clone() {
        BackendDAE::JacobianType::JAC_CONSTANT => true,
        BackendDAE::JacobianType::JAC_LINEAR => true,
        _ => false,
    });
    b
}

fn res2Con(mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut eindex: Arc<metamodelica::List<i32>>, mut vindx: Arc<metamodelica::List<i32>>, mut ishared: Arc<BackendDAE::Shared>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>)> {
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = BackendEquation::getList(eindex.clone(), ieqns.clone());
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), ivars.clone());
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut var_: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = List::map(var_lst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr_var: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    let mut res: Arc<DAE::Exp>;
    let mut ind_e: i32 = 0;
    let mut ind_v: i32 = 0;
    let mut ind_lst_v: Arc<metamodelica::List<i32>> = List::map(vindx.clone(), Arc::new(fnptr!(intAbs, i32)));
    let mut ind_lst_e: Arc<metamodelica::List<i32>> = eindex.clone();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(oshared.clone()) {
        Deref @ BackendDAE::Shared { globalKnownVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    globalKnownVars = __pa0.clone();
    for mut var_ in &*var_lst.clone() {
        let mut var_ = var_.clone();
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(cr_lst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cr_var = __pa1.clone();
        cr_lst = __pa2.clone();
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(eqn_lst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        eqn = __pa3.clone();
        eqn_lst = __pa4.clone();
        let (__pa5, __pa6) = ::match_deref::match_deref! { match &(ind_lst_e.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ind_e = __pa5.clone();
        ind_lst_e = __pa6.clone();
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(ind_lst_v.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa7, tail: __pa8 } => (__pa7.clone(), __pa8.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ind_v = __pa7.clone();
        ind_lst_v = __pa8.clone();
        cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$EqCon$")); __mm_s.push_str(&*ComponentReference::crefModelicaStr(cr_var.clone())); ArcStr::from(__mm_s) }).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
        e = Expression::crefExp(cr.clone())?;
        var = BackendVariable::makeVar(cr.clone());
        var = BackendVariable::setVarMinMax(var.clone(), Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })), Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))?;
        var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::OPT_CONSTR)?;
        var = BackendVariable::setVarDirection(var.clone(), openmodelica_frontend_types::DAE::VarDirection::OUTPUT);
        ovars = BackendVariable::addNewVar(var.clone(), ovars.clone())?;
        res = BackendDAEOptimize::makeEquationToResidualExp(eqn.clone())?;
        res = Expression::createResidualExp(res.clone(), Expression::makeConstZeroE(res.clone()))?;
        oeqns = BackendEquation::setAtIndex(oeqns.clone(), ind_e.clone(), Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: res.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }))?;
        (cr, var) = makeVar(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*ComponentReference::crefModelicaStr(cr_var.clone())); ArcStr::from(__mm_s) }).clone());
        var = BackendVariable::setVarDirection(var.clone(), openmodelica_frontend_types::DAE::VarDirection::INPUT);
        e = Expression::crefExp(cr_var.clone())?;
        if BackendVariable::isStateVar(var_.clone()) {
            e = Expression::expDer(e.clone());
            var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::OPT_LOOP_INPUT { replaceExp: ComponentReference::crefPrefixDer(cr_var.clone()) })?;
        } else {
            var = BackendVariable::mergeAliasVars(var.clone(), var_.clone(), false, globalKnownVars.clone())?;
            var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::OPT_LOOP_INPUT { replaceExp: cr_var.clone() })?;
        }
        oshared = BackendVariable::addGlobalKnownVarDAE(var.clone(), oshared.clone())?;
        oeqns = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: Expression::crefExp(cr.clone())?, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), oeqns.clone())?;
    }
    Ok((oeqns, ovars, oshared))
}

// =============================================================================
// section for postOptModule >>simplifyConstraints<<
//
// simplify nonlinear constraints if possible in  box constraints
// =============================================================================
pub fn simplifyConstraints(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut new_systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut eqn_: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut var_: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut var_con: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut eindex: i32 = 0;
    let mut vindx: i32 = 0;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut var_lst_opt: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut var_lst1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut c: Arc<DAE::Exp>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut oMax_con: Option<Arc<DAE::Exp>> = None;
    let mut oMin_con: Option<Arc<DAE::Exp>> = None;
    let mut max_con: Arc<DAE::Exp>;
    let mut min_con: Arc<DAE::Exp>;
    let mut zero: Arc<DAE::Exp>;
    let mut con2: Arc<DAE::Exp>;
    let mut z: Arc<DAE::Exp>;
    let mut der_e: Arc<DAE::Exp>;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b: bool = false;
    let mut b3: bool = false;
    let mut b4: bool = false;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())? {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
            Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        systlst = __pa0.clone();
        shared = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(shared.clone()) {
            Deref @ BackendDAE::Shared { globalKnownVars: __pa2, functionTree: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        globalKnownVars = __pa2.clone();
        funcs = __pa3.clone();
        for mut syst in &*systlst.clone() {
            let mut syst = syst.clone();
            let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(syst.clone()) {
                Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa4, .. }, orderedEqs: __pa5, orderedVars: __pa6, .. } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            comps = __pa4.clone();
            eqns = __pa5.clone();
            vars = __pa6.clone();
            b = false;
            for mut comp in &*comps.clone() {
                let mut comp = comp.clone();
                if (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
                    let (__pa8, __pa9) = ::match_deref::match_deref! { match &(comp.clone()) {
                        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: __pa8, eqn: __pa9 } => (__pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vindx = __pa8.clone();
                    eindex = __pa9.clone();
                    var_con = BackendVariable::getVarAt(vars.clone(), vindx.clone())?;
                    b3 = BackendVariable::isRealOptimizeConstraintsVars(var_con.clone());
                    if b3.clone() {
                        if '__try10: {
                            let (__pa13, __pa11, __pa12) = ::match_deref::match_deref! { match &(BackendEquation::get(eqns.clone(), eindex.clone())) {
                                __pa13 @ Deref @ BackendDAE::Equation::EQUATION { scalar: __pa11, exp: __pa12, .. } => (__pa13.clone(), __pa11.clone(), __pa12.clone()),
                                _ => break '__try10 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                            } };
                            e2 = __pa11.clone();
                            e1 = __pa12.clone();
                            eqn_ = __pa13.clone();
                            let true = (unwrap_break_err!(ExpressionBasics::expEqual(e1.clone(), BackendVariable::varExp(var_con.clone())?), '__try10)) else { break '__try10 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                            Ok::<(), anyhow::Error>(())
                        }.is_err() {
                            b3 = false;
                        }
                    }
                    if b3.clone() {
                        var_lst = BackendEquation::equationsLstVars(list![eqn_.clone()], vars.clone())?;
                        var_lst_opt = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut vv in (var_lst.clone()).into_iter().cloned() {
            if !(BackendVariable::isStateVar(vv.clone())) { continue; }
            let __x = vv.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                        b3 = (var_lst_opt.clone().len() as i32) == 1;
                        var_lst = BackendEquation::equationsLstVars(list![eqn_.clone()], globalKnownVars.clone())?;
                        var_lst_opt = listAppend(var_lst_opt.clone(), ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut vv in (var_lst.clone()).into_iter().cloned() {
            if !(BackendVariable::isInput(vv.clone())) { continue; }
            let __x = vv.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
                        if (var_lst_opt.clone().len() as i32) == 1 {
                            let __pa14 = ::match_deref::match_deref! { match &(var_lst_opt.clone()) {
                                Deref @ metamodelica::List::Cons { head: __pa14, tail: Deref @ metamodelica::List::Nil } => __pa14.clone(),
                                _ => bail!("pattern mismatch"),
                            } };
                            var_ = __pa14.clone();
                            let BackendDAE::VAR { varName: __pa16, .. } = (var_.clone()) else { bail!("pattern mismatch") };
                            cr = __pa16.clone();
                            e = Expression::crefExp(cr.clone())?;
                            tp = Expression::r#typeof(e.clone())?;
                            zero = Expression::makeConstZero(tp.clone());
                            if '__try17: {
                                der_e = unwrap_break_err!(Differentiate::differentiateExpSolve(e2.clone(), cr.clone(), Some(funcs.clone())), '__try17);
                                (der_e, _) = unwrap_break_err!(ExpressionSimplify::simplify(der_e.clone()), '__try17);
                                if Expression::isZero(e.clone()) {
                                    continue;
                                }
                                (z, _) = unwrap_break_err!(Expression::makeZeroExpression(Expression::arrayDimension(tp.clone())), '__try17);
                                (c, _) = unwrap_break_err!(Expression::replaceExp(e2.clone(), e.clone(), z.clone()), '__try17);
                                (c, _) = unwrap_break_err!(ExpressionSimplify::simplify(c.clone()), '__try17);
                                var_lst = unwrap_break_err!(BackendEquation::expressionVars(der_e.clone(), globalKnownVars.clone()), '__try17);
                                if b3.clone() {
                                    var_lst = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut vv in (var_lst.clone()).into_iter().cloned() {
            if !(!(BackendVariable::isParam(vv.clone()))) { continue; }
            let __x = vv.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                                }
                                var_lst = listAppend(BackendEquation::expressionVars(der_e.clone(), vars.clone())?, var_lst.clone());
                                var_lst1 = unwrap_break_err!(BackendEquation::expressionVars(c.clone(), globalKnownVars.clone()), '__try17);
                                if b3.clone() {
                                    var_lst1 = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut vv in (var_lst1.clone()).into_iter().cloned() {
            if !(!(BackendVariable::isParam(vv.clone()))) { continue; }
            let __x = vv.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                                }
                                var_lst1 = listAppend(BackendEquation::expressionVars(c.clone(), vars.clone())?, var_lst1.clone());
                                var_lst = listAppend(var_lst1.clone(), var_lst.clone());
                                b4 = unwrap_break_err!(Expression::expHasCref(der_e.clone(), DAE::crefTime().clone()), '__try17) || unwrap_break_err!(Expression::expHasCref(c.clone(), DAE::crefTime().clone()), '__try17);
                                if var_lst.clone().is_empty() && !(b4.clone()) {
                                    (oMin_con, oMax_con) = BackendVariable::getMinMaxAttribute(var_con.clone());
                                    b1 = isSome(oMin_con.clone());
                                    b2 = isSome(oMax_con.clone());
                                    con2 = Expression::makeNoEvent(Arc::new(DAE::Exp::RELATION { exp1: der_e.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: zero.clone(), index: -1, optionExpisASUB: None }));
                                    if b1.clone() {
                                        let __pa18 = ::match_deref::match_deref! { match &(oMin_con.clone()) {
                                            Some(__pa18) => __pa18.clone(),
                                            _ => break '__try17 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                                        } };
                                        min_con = __pa18.clone();
                                        min_con = unwrap_break_err!(Expression::makeDiv(Expression::expSub(min_con.clone(), c.clone())?, der_e.clone()), '__try17);
                                    } else {
                                        min_con = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e64_f64) });
                                    }
                                    if b2.clone() {
                                        let __pa19 = ::match_deref::match_deref! { match &(oMax_con.clone()) {
                                            Some(__pa19) => __pa19.clone(),
                                            _ => break '__try17 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                                        } };
                                        max_con = __pa19.clone();
                                        max_con = unwrap_break_err!(Expression::makeDiv(Expression::expSub(max_con.clone(), c.clone())?, der_e.clone()), '__try17);
                                    } else {
                                        max_con = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1e64_f64) });
                                    }
                                    oMin_con = Some(Arc::new(DAE::Exp::IFEXP { expCond: con2.clone(), expThen: max_con.clone(), expElse: min_con.clone() }));
                                    oMax_con = Some(Arc::new(DAE::Exp::IFEXP { expCond: con2.clone(), expThen: min_con.clone(), expElse: max_con.clone() }));
                                    oMin_con = unwrap_break_err!(ExpressionSimplify::simplify1o(oMin_con.clone()), '__try17);
                                    oMax_con = unwrap_break_err!(ExpressionSimplify::simplify1o(oMax_con.clone()), '__try17);
                                    var_con = unwrap_break_err!(BackendVariable::setVarMinMax(var_con.clone(), oMin_con.clone(), oMax_con.clone()), '__try17);
                                    var_ = unwrap_break_err!(BackendVariable::mergeMinMaxAttribute(var_con.clone(), var_.clone(), false), '__try17);
                                    var_con = unwrap_break_err!(BackendVariable::setVarKind(var_con.clone(), crate::BackendDAE::VarKind::VARIABLE), '__try17);
                                    vars = unwrap_break_err!(BackendVariable::setVarAt(vars.clone(), vindx.clone(), var_con.clone()), '__try17);
                                    match '__try20: {
                                        (_, vindx) = unwrap_break_err!(BackendVariable::getVarSingle(cr.clone(), vars.clone()), '__try20);
                                        vars = unwrap_break_err!(BackendVariable::setVarAt(vars.clone(), vindx.clone(), var_.clone()), '__try20);
                                        Ok::<_, anyhow::Error>((vindx.clone(),))
                                    } {
                                        Ok((__try20_o0,)) => {
                                            vindx = __try20_o0;
                                        }
                                        Err(_) => {
                                            (_, vindx) = unwrap_break_err!(BackendVariable::getVarSingle(cr.clone(), globalKnownVars.clone()), '__try17);
                                            globalKnownVars = unwrap_break_err!(BackendVariable::setVarAt(globalKnownVars.clone(), vindx.clone(), var_.clone()), '__try17);
                                        }
                                    }
                                    b = true;
                                }
                                Ok::<(), anyhow::Error>(())
                            }.is_err() {
                            }
                        }
                    }
                }
            }
            if b.clone() {
                new_systlst = cons(BackendDAEUtil::clearEqSyst(syst.clone())?, new_systlst.clone());
            } else {
                new_systlst = cons(syst.clone(), new_systlst.clone());
            }
        }
        shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), globalKnownVars.clone());
        outDAE = BackendDAE::DAE(new_systlst.clone(), shared.clone())?;
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

// =============================================================================
// section for postOptModule >>reduceDynamicOptimization<<
//
// remove eqs which not need for the calculations of cost and constraints
// =============================================================================
pub fn reduceDynamicOptimization(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut opt_varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut conVarsList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut fconVarsList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut objMayer: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut objLagrange: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut newsyst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut v: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systlst = __pa0.clone();
    shared = __pa1.clone();
    shared = BackendVariable::removeAliasVars(shared.clone())?;
    for mut syst in &*systlst.clone() {
        let mut syst = syst.clone();
        syst = BackendEquation::removeRemovedEqs(syst.clone());
        let __pa2 = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa2, .. } => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        v = __pa2.clone();
        varlst = BackendVariable::varList(v.clone())?;
        conVarsList = List::select(varlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isRealOptimizeConstraintsVars, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
        fconVarsList = List::select(varlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isRealOptimizeFinalConstraintsVars, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
        objMayer = checkObjectIsSet(v.clone(), (arcstr::literal!(BackendDAE::optimizationMayerTermName)).clone());
        objLagrange = checkObjectIsSet(v.clone(), (arcstr::literal!(BackendDAE::optimizationLagrangeTermName)).clone());
        opt_varlst = listAppend(conVarsList.clone(), listAppend(fconVarsList.clone(), listAppend(objMayer.clone(), objLagrange.clone())));
        if !(opt_varlst.clone().is_empty()) {
            newsyst = cons(BackendDAEUtil::tryReduceEqSystem(syst.clone(), shared.clone(), opt_varlst.clone(), false), newsyst.clone());
        }
    }
    outDAE = BackendDAE::DAE(newsyst.clone(), shared.clone())?;
    Ok(outDAE)
}

pub fn checkObjectIsSet(mut inVars: BackendDAE::Variables, mut CrefName: ArcStr) -> Arc<metamodelica::List<BackendDAE::Var>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut leftcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    leftcref = ComponentReferenceBasics::makeCrefIdent((CrefName.clone()).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
    match '__try0: {
        (outVars, _) = unwrap_break_err!(BackendVariable::getVar(leftcref.clone(), inVars.clone()), '__try0);
        Ok::<_, anyhow::Error>((outVars.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outVars = __try0_o0;
        }
        Err(_) => {
            outVars = metamodelica::nil();
        }
    }
    outVars
}

