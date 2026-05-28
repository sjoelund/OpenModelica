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

use crate::AvlSetInt;
use crate::BackendDAE;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::EvaluateFunctions;
use crate::ExpressionSolve;
use crate::HashTableCrToCrEqLst;
use crate::SimCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::HashTableCrToExp;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

/// eqnAttributes(source,EquationAttributes)
pub type EquationSourceAndAttributes = (Arc<DAE::ElementSource>, BackendDAE::EquationAttributes);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimpleContainer {
    ALIAS {
        cr1: Arc<DAE::ComponentRef>,
        negatedCr1: bool,
        i1: i32,
        cr2: Arc<DAE::ComponentRef>,
        negatedCr2: bool,
        i2: i32,
        eqnAttributes: EquationSourceAndAttributes,
        visited: i32,
    },
    PARAMETERALIAS {
        unknowncr: Arc<DAE::ComponentRef>,
        negatedCr1: bool,
        i1: i32,
        paramcr: Arc<DAE::ComponentRef>,
        negatedCr2: bool,
        i2: i32,
        eqnAttributes: EquationSourceAndAttributes,
        visited: i32,
    },
    TIMEALIAS {
        cr1: Arc<DAE::ComponentRef>,
        negatedCr1: bool,
        i1: i32,
        cr2: Arc<DAE::ComponentRef>,
        negatedCr2: bool,
        i2: i32,
        eqnAttributes: EquationSourceAndAttributes,
        visited: i32,
    },
    TIMEINDEPENTVAR {
        cr: Arc<DAE::ComponentRef>,
        i: i32,
        exp: Arc<DAE::Exp>,
        eqnAttributes: EquationSourceAndAttributes,
        visited: i32,
    },
}
pub use self::SimpleContainer::{ALIAS,PARAMETERALIAS,TIMEALIAS,TIMEINDEPENTVAR};

pub type AccTuple = (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool);

/// fixed, list<startvalue, origin, cr>, nominal, (min, max)
pub type VarSetAttributes = (bool, (i32, Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>), Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>, (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>));

thread_local! { static __EMPTYVARSETATTRIBUTES_TLS: (bool, (i32, Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>), Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>, (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>)) = (false, (-1, metamodelica::nil()), metamodelica::nil(), (None, None)); }
pub fn EMPTYVARSETATTRIBUTES() -> (bool, (i32, Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>), Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>, (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>)) { __EMPTYVARSETATTRIBUTES_TLS.with(|__t| __t.clone()) }

// =============================================================================
// Starting point for preOpt and postOpt removeSimpleEquations module
//
// =============================================================================
pub fn removeSimpleEquations(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    if BackendDAEUtil::hasDAEMatching(inDAE.clone()) {
        outDAE = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())?) {
        Deref @ "default" => causal(inDAE.clone())?,
        Deref @ "causal" => causal(inDAE.clone())?,
        Deref @ "new" => performAliasEliminationBB(inDAE.clone(), true)?,
        _ => inDAE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outDAE = fixAliasVars(outDAE.clone())?;
        outDAE = fixAliasAndKnownVarsCausal(inDAE.clone(), outDAE.clone())?;
        outDAE = fixAliasVarsVariablity(outDAE.clone())?;
    } else {
        outDAE = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())?) {
        Deref @ "default" => fastAcausal(inDAE.clone())?,
        Deref @ "fastAcausal" => fastAcausal(inDAE.clone())?,
        Deref @ "allAcausal" => allAcausal(inDAE.clone())?,
        Deref @ "new" => performAliasEliminationBB(inDAE.clone(), true)?,
        _ => inDAE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outDAE = fixAliasVars(outDAE.clone())?;
        outDAE = fixKnownVars(outDAE.clone())?;
        outDAE = fixAliasVarsVariablity(outDAE.clone())?;
    }
    Ok(outDAE)
}

pub fn removeVerySimpleEquations(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    if BackendDAEUtil::hasDAEMatching(inDAE.clone()) {
        Error::addInternalError((literal!("Cannot run removeVerySimpleEquations on a matched system (continuing anyway)")).clone(), metamodelica::sourceInfo!())?;
        outDAE = inDAE.clone();
    } else {
        outDAE = performAliasEliminationBB(inDAE.clone(), true)?;
    }
    Ok(outDAE)
}

pub fn fixAliasVarsVariablity(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut aliasVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut systvars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut binding: Arc<DAE::Exp>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut paramOrConst: bool = false;
    let mut r#const: bool = false;
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut referencevar: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tempreferencevar: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut knownVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut aliasVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tempvar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    if !(Flags::getConfigBool(Flags::BUILDING_FMU.clone())?) {
        outDAE = inDAE.clone();
        return Ok(outDAE);
    }
    aliasVars = BackendDAEUtil::getAliasVars(inDAE.clone())?;
    systvars = BackendVariable::listVar(BackendVariable::equationSystemsVarsLst(inDAE.eqs.clone())?);
    for mut var in &*BackendVariable::varList(aliasVars.clone())? {
        let mut var = var.clone();
        binding = BackendVariable::varBindExp(var.clone())?;
        crefs = Expression::getAllCrefs(binding.clone())?;
        referencevar = metamodelica::nil();
        for mut cr in &*crefs.clone() {
            let mut cr = cr.clone();
            tempreferencevar = getVarsHelper(cr.clone(), systvars.clone());
            if tempreferencevar.clone().is_empty() {
                tempreferencevar = getVarsHelper(cr.clone(), inDAE.shared.globalKnownVars.clone());
            }
            referencevar = listAppend(tempreferencevar.clone(), referencevar.clone());
        }
        if referencevar.clone().is_empty() {
            paramOrConst = false;
            r#const = false;
        } else {
            paramOrConst = List::all(referencevar.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isParamOrConstant, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
            r#const = List::all(referencevar.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isConst, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
        }
        if r#const.clone() {
            tempvar = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::CONST)?;
            knownVarList = cons(BackendVariable::setVarFixed(tempvar.clone(), true)?, knownVarList.clone());
        } else if paramOrConst.clone() {
            tempvar = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::PARAM)?;
            knownVarList = cons(BackendVariable::setVarFixed(tempvar.clone(), true)?, knownVarList.clone());
        } else {
            aliasVarList = cons(var.clone(), aliasVarList.clone());
        }
    }
    globalKnownVars = BackendVariable::mergeVariables(inDAE.shared.globalKnownVars.clone(), BackendVariable::listVar(knownVarList.clone()), true)?;
    outDAE = BackendDAEUtil::setAliasVars(inDAE.clone(), BackendVariable::listVar(aliasVarList.clone()))?;
    outDAE = BackendDAEUtil::setDAEGlobalKnownVars(outDAE.clone(), globalKnownVars.clone())?;
    Ok(outDAE)
}

fn getVarsHelper(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables) -> Arc<metamodelica::List<BackendDAE::Var>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    match '__try0: {
        (outVars, _) = unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars.clone()), '__try0);
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

fn fixAliasVars(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut aliasVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut aliasVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut knownVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut binding: Arc<DAE::Exp>;
    aliasVars = BackendDAEUtil::getAliasVars(inDAE.clone())?;
    knownVarList = BackendVariable::varList(BackendDAEUtil::getGlobalKnownVarsFromDAE(inDAE.clone())?)?;
    for mut var in &*BackendVariable::varList(aliasVars.clone())? {
        let mut var = var.clone();
        binding = BackendVariable::varBindExp(var.clone())?;
        if Expression::isConst(binding.clone())? {
            knownVarList = cons(var.clone(), knownVarList.clone());
        } else {
            aliasVarList = cons(var.clone(), aliasVarList.clone());
        }
    }
    outDAE = BackendDAEUtil::setAliasVars(inDAE.clone(), BackendVariable::listVar(aliasVarList.clone()))?;
    outDAE = BackendDAEUtil::setDAEGlobalKnownVars(outDAE.clone(), BackendVariable::listVar(knownVarList.clone()))?;
    Ok(outDAE)
}

fn fixKnownVars(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    let mut eqs: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut binding: Arc<DAE::Exp>;
    let mut eqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut knownVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    globalKnownVars = dae.shared.globalKnownVars.clone();
    for mut var in &*BackendVariable::varList(globalKnownVars.clone())? {
        let mut var = var.clone();
        if BackendVariable::varHasBindExp(var.clone()) {
            binding = BackendVariable::varBindExp(var.clone())?;
            (_, crlst) = Expression::traverseExpTopDown(binding.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            if BackendDAEUtil::containAnyVar(crlst.clone(), dae.shared.localKnownVars.clone())? {
                varList = cons(BackendVariable::setBindExp(var.clone(), None), varList.clone());
                eqnList = cons(Arc::new(BackendDAE::Equation::EQUATION { exp: BackendVariable::varExp(var.clone())?, scalar: binding.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() }), eqnList.clone());
            } else {
                knownVarList = cons(var.clone(), knownVarList.clone());
            }
        } else {
            knownVarList = cons(var.clone(), knownVarList.clone());
        }
    }
    if !(varList.clone().is_empty()) {
        eqs = BackendDAEUtil::createEqSystem(BackendVariable::listVar(varList.clone()), BackendEquation::listEquation(eqnList.clone())?, metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION, BackendEquation::emptyEqns());
        assign_field!(dae.eqs = cons(eqs.clone(), dae.eqs.clone()));
    }
    dae = BackendDAEUtil::setDAEGlobalKnownVars(dae.clone(), BackendVariable::listVar(knownVarList.clone()))?;
    Ok(dae)
}

fn fixAliasAndKnownVarsCausal(mut inDAE1: Arc<BackendDAE::BackendDAE>, mut inDAE2: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE2.clone();
    let mut aliasVars1: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut knownVars1: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut aliasVars2: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut knownVars2: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut aliasVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut knownVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    aliasVars1 = BackendDAEUtil::getAliasVars(inDAE1.clone())?;
    aliasVars2 = BackendDAEUtil::getAliasVars(inDAE2.clone())?;
    knownVars1 = BackendDAEUtil::getGlobalKnownVarsFromDAE(inDAE1.clone())?;
    knownVars2 = BackendDAEUtil::getGlobalKnownVarsFromDAE(inDAE2.clone())?;
    for mut var in &*BackendVariable::varList(aliasVars2.clone())? {
        let mut var = var.clone();
        cref = BackendVariable::varCref(var.clone())?;
        if !(BackendVariable::existsVar(cref.clone(), aliasVars1.clone(), false)) {
            outDAE = fixAliasVarsCausal2(var.clone(), outDAE.clone())?;
        } else {
            aliasVarList = cons(var.clone(), aliasVarList.clone());
        }
    }
    outDAE = BackendDAEUtil::setAliasVars(outDAE.clone(), BackendVariable::listVar(aliasVarList.clone()))?;
    for mut var in &*BackendVariable::varList(knownVars2.clone())? {
        let mut var = var.clone();
        cref = BackendVariable::varCref(var.clone())?;
        if !(BackendVariable::existsVar(cref.clone(), knownVars1.clone(), false)) && !(BackendVariable::isInput(var.clone()) || BackendVariable::isAlgebraicOldState(var.clone())) {
            outDAE = fixKnownVarsCausal2(var.clone(), outDAE.clone())?;
        } else {
            knownVarList = cons(var.clone(), knownVarList.clone());
        }
    }
    outDAE = BackendDAEUtil::setDAEGlobalKnownVars(outDAE.clone(), BackendVariable::listVar(knownVarList.clone()))?;
    Ok(outDAE)
}

fn fixAliasVarsCausal2(mut inVar: BackendDAE::Var, mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut binding: Arc<DAE::Exp>;
    let mut rightCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut eqs1: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut done: bool = false;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    match '__try0: {
        binding = unwrap_break_err!(BackendVariable::varBindExp(inVar.clone()), '__try0);
        rightCrefs = unwrap_break_err!(Expression::getAllCrefs(binding.clone()), '__try0);
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inDAE.clone()) {
            Deref @ DAE { UNIQUEIO: __pa1, derivativeNamePrefix: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        eqs = __pa1.clone();
        shared = __pa2.clone();
        var = BackendVariable::setBindExp(inVar.clone(), None);
        var = unwrap_break_err!(BackendVariable::setVarFixed(var.clone(), false), '__try0);
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: unwrap_break_err!(BackendVariable::varExp(var.clone()), '__try0), scalar: binding.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() });
        for mut eq in &*eqs.clone() {
            let mut eq = eq.clone();
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(eq.clone()) {
                Deref @ BackendDAE::EqSystem { orderedEqs: __pa3, orderedVars: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            orderedEqs = __pa3.clone();
            orderedVars = __pa4.clone();
            if unwrap_break_err!(BackendVariable::existsAnyVar(rightCrefs.clone(), orderedVars.clone(), false), '__try0) {
                orderedVars = unwrap_break_err!(BackendVariable::addVar(var.clone(), orderedVars.clone()), '__try0);
                orderedEqs = unwrap_break_err!(BackendEquation::add(eqn.clone(), orderedEqs.clone()), '__try0);
                eqs1 = cons(BackendDAEUtil::setEqSystEqs(BackendDAEUtil::setEqSystVars(eq.clone(), orderedVars.clone())?, orderedEqs.clone()), eqs1.clone());
                let false = (done.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                done = true;
            } else {
                eqs1 = cons(eq.clone(), eqs1.clone());
            }
        }
        if !(done.clone()) {
            eqs1 = cons(BackendDAEUtil::createEqSystem(BackendVariable::listVar(list![var.clone()]), BackendEquation::listEquation(list![eqn.clone()])?, metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION, BackendEquation::emptyEqns()), eqs1.clone());
        }
        outDAE = unwrap_break_err!(BackendDAE::DAE(eqs1.clone().reverse(), shared.clone()), '__try0);
        Ok::<_, anyhow::Error>((binding.clone(), eqn.clone(), eqs.clone(), outDAE.clone(), rightCrefs.clone(), shared.clone(), var.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6)) => {
            binding = __try0_o0;
            eqn = __try0_o1;
            eqs = __try0_o2;
            outDAE = __try0_o3;
            rightCrefs = __try0_o4;
            shared = __try0_o5;
            var = __try0_o6;
        }
        Err(_) => {
            BackendDump::dumpVarList(list![inVar.clone()], (literal!("fixAliasVarsCausal2 failed for ...")).clone());
            Error::addCompilerError((literal!("fixAliasVarsCausal2 failed")).clone())?;
            bail!("fail");
        }
    }
    Ok(outDAE)
}

fn fixKnownVarsCausal2(mut inVar: BackendDAE::Var, mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut binding: Arc<DAE::Exp>;
    let mut rightCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut eqs1: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut done: bool = false;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    match '__try0: {
        binding = unwrap_break_err!(BackendVariable::varBindExp(inVar.clone()), '__try0);
        rightCrefs = unwrap_break_err!(Expression::getAllCrefs(binding.clone()), '__try0);
        var = BackendVariable::setBindExp(inVar.clone(), None);
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: unwrap_break_err!(BackendVariable::varExp(var.clone()), '__try0), scalar: binding.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() });
        for mut eq in &*inDAE.eqs.clone() {
            let mut eq = eq.clone();
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(eq.clone()) {
                Deref @ BackendDAE::EqSystem { orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            orderedEqs = __pa1.clone();
            orderedVars = __pa2.clone();
            if unwrap_break_err!(BackendVariable::existsAnyVar(rightCrefs.clone(), orderedVars.clone(), false), '__try0) {
                orderedVars = unwrap_break_err!(BackendVariable::addVar(var.clone(), orderedVars.clone()), '__try0);
                orderedEqs = unwrap_break_err!(BackendEquation::add(eqn.clone(), orderedEqs.clone()), '__try0);
                eqs1 = cons(BackendDAEUtil::setEqSystEqs(BackendDAEUtil::setEqSystVars(eq.clone(), orderedVars.clone())?, orderedEqs.clone()), eqs1.clone());
                let false = (done.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                done = true;
            } else {
                eqs1 = cons(eq.clone(), eqs1.clone());
            }
        }
        if !(done.clone()) {
            eqs1 = cons(BackendDAEUtil::createEqSystem(BackendVariable::listVar(list![var.clone()]), BackendEquation::listEquation(list![eqn.clone()])?, metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION, BackendEquation::emptyEqns()), eqs1.clone());
        }
        outDAE = unwrap_break_err!(BackendDAE::DAE(eqs1.clone().reverse(), inDAE.shared.clone()), '__try0);
        Ok::<_, anyhow::Error>((binding.clone(), eqn.clone(), outDAE.clone(), rightCrefs.clone(), var.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            binding = __try0_o0;
            eqn = __try0_o1;
            outDAE = __try0_o2;
            rightCrefs = __try0_o3;
            var = __try0_o4;
        }
        Err(_) => {
            BackendDump::dumpVarList(list![inVar.clone()], (literal!("fixKnownVarsCausal2 failed for ...")).clone());
            Error::addCompilerError((literal!("fixKnownVarsCausal2 failed")).clone())?;
            bail!("fail");
        }
    }
    Ok(outDAE)
}

// =============================================================================
// section for fastAcausal
//
// =============================================================================
pub fn fastAcausal(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut b: bool = false;
    let mut warnAliasConflicts: bool = false;
    let mut size: i32 = 0;
    let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    size = BackendDAEUtil::daeSize(inDAE.clone());
    size = intMax(BaseHashTable::defaultBucketSize.clone(), (((intReal(size.clone())) * (metamodelica::OrderedFloat(0.7_f64))).0 as i32));
    repl = BackendVarTransform::emptyReplacementsSized(size.clone());
    unReplaceable = HashSet::emptyHashSet();
    unReplaceable = BackendDAEUtil::foldEqSystem(inDAE.clone(), (std::sync::Arc::new(addUnreplaceable) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), unReplaceable.clone())?;
    (_, unReplaceable) = BackendDAEUtil::traverseBackendDAEExps(inDAE.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traverserExpUnreplaceable) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), unReplaceable.clone()))?;
    unReplaceable = addUnreplaceableFromWhens(inDAE.clone(), unReplaceable.clone())?;
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        BackendDump::dumpHashSet(unReplaceable.clone(), (literal!("Unreplaceable Crefs:")).clone())?;
    }
    let (__pa0, (__pa1, __pa2, _, _, __pa3)) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(fnptr!(fastAcausal1, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), i32, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), i32, bool)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), i32, bool))> + 'static>), (repl.clone(), false, unReplaceable.clone(), Flags::getConfigInt(Flags::MAXTRAVERSALS.clone())?, false))?;
    outDAE = __pa0.clone();
    repl = __pa1.clone();
    b = __pa2.clone();
    warnAliasConflicts = __pa3.clone();
    if warnAliasConflicts.clone() && BackendDAEUtil::isSimulationDAE(inDAE.shared.clone()) {
        Error::addMessage(Error::REDUNDANT_ALIAS_SET.clone(), metamodelica::nil())?;
    }
    outDAE = removeSimpleEquationsShared(b.clone(), outDAE.clone(), repl.clone())?;
    Ok(outDAE)
}

fn addUnreplaceable(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut inUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = inUnreplaceable.clone();
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orderedVars = __pa0.clone();
    for mut var in &*BackendVariable::varList(orderedVars.clone())? {
        let mut var = var.clone();
        if BackendVariable::varUnreplaceable(var.clone()) {
            outUnreplaceable = BaseHashSet::add(BackendVariable::varCref(var.clone())?, outUnreplaceable.clone())?;
        }
    }
    Ok(outUnreplaceable)
}

fn fastAcausal1(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inTpl: (BackendVarTransform::VariableReplacements, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), i32, bool)) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), i32, bool)) {
    let mut outSystem: Arc<BackendDAE::EqSystem> = BackendDAEUtil::copyEqSystem(inSystem.clone()).unwrap();
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outTpl: (BackendVarTransform::VariableReplacements, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), i32, bool);
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut simpleeqnslst: Arc<metamodelica::List<SimpleContainer>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut foundSimple: bool = false;
    let mut globalFoundSimple: bool = false;
    let mut warnAliasConflicts: bool = false;
    let mut maxTraversals: i32 = 0;
    if BackendDAEUtil::isClockedSyst(inSystem.clone()) {
        outSystem = inSystem.clone();
        outShared = inShared.clone();
        outTpl = inTpl.clone();
        return (outSystem, outShared, outTpl);
    }
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(outSystem.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        eqns = __pa1.clone();
        vars = __pa2.clone();
        (repl, globalFoundSimple, unReplaceable, maxTraversals, warnAliasConflicts) = inTpl.clone();
        eqnslst = BackendEquation::equationList(eqns.clone());
        mT = arrayCreate(BackendVariable::varsSize(vars.clone()).unwrap(), metamodelica::nil());
        (_, _, eqnslst, simpleeqnslst, _, _, foundSimple) = List::fold(eqnslst.clone(), (std::sync::Arc::new(simpleEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), (vars.clone(), inShared.clone(), metamodelica::nil(), metamodelica::nil(), 1, mT.clone(), false));
        (vars, outShared, repl, unReplaceable, eqnslst, globalFoundSimple, warnAliasConflicts) = unwrap_break_err!(causalFinder(foundSimple.clone(), simpleeqnslst.clone(), eqnslst.clone(), 1, maxTraversals.clone(), vars.clone(), inShared.clone(), repl.clone(), unReplaceable.clone(), mT.clone(), metamodelica::nil(), globalFoundSimple.clone(), warnAliasConflicts.clone()), '__try0);
        outSystem = unwrap_break_err!(updateSystem(globalFoundSimple.clone(), eqnslst.clone(), vars.clone(), repl.clone(), outSystem.clone()), '__try0);
        outTpl = (repl.clone(), globalFoundSimple.clone(), unReplaceable.clone(), maxTraversals.clone(), warnAliasConflicts.clone());
        GCExt::free(mT.clone());
        Ok::<_, anyhow::Error>((outShared.clone(), outSystem.clone(), outTpl.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            outShared = __try0_o0;
            outSystem = __try0_o1;
            outTpl = __try0_o2;
        }
        Err(_) => {
            outSystem = inSystem.clone();
            outShared = inShared.clone();
            outTpl = inTpl.clone();
        }
    }
    (outSystem, outShared, outTpl)
}

fn causalFinder(mut foundSimple: bool, mut simpleContainerIn: Arc<metamodelica::List<SimpleContainer>>, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut traversalIdx: i32, mut maxTraversals: i32, mut iVars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut iRepl: BackendVarTransform::VariableReplacements, mut iUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGlobalEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inGlobalFoundSimple: bool, mut warnAliasConflicts: bool) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool, bool)> {
    let mut outVars: BackendDAE::Variables = iVars.clone();
    let mut outShared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut outRepl: BackendVarTransform::VariableReplacements = iRepl.clone();
    let mut outUnReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = iUnreplaceable.clone();
    let mut outEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outGlobalFoundSimple: bool = inGlobalFoundSimple.clone();
    let mut warnAliasConflicts: bool = warnAliasConflicts;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut b: bool = false;
    let mut b1: bool = false;
    let mut simpleContainer: metamodelica::Array<SimpleContainer>;
    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    if foundSimple.clone() {
        simpleContainer = List::listArrayReverse(simpleContainerIn.clone())?;
        (vars, eqnslst, shared, repl, b) = handleSets((simpleContainer.clone().borrow().len() as i32), 1, simpleContainer.clone(), iMT.clone(), iUnreplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone())?;
        warnAliasConflicts = warnAliasConflicts.clone() || b.clone();
        (eqnslst, b1) = BackendVarTransform::replaceEquations(eqnslst.clone(), repl.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
        (outVars, outShared, outRepl, outUnReplaceable, outEqnslst, warnAliasConflicts) = causalFinder1(intGt(traversalIdx.clone(), maxTraversals.clone()), b1.clone(), eqnslst.clone(), traversalIdx.clone() + 1, maxTraversals.clone(), vars.clone(), shared.clone(), repl.clone(), iUnreplaceable.clone(), iMT.clone(), iGlobalEqnslst.clone(), inGlobalFoundSimple.clone(), warnAliasConflicts.clone())?;
        outGlobalFoundSimple = true;
    } else {
        outEqnslst = listAppend(iEqnslst.clone(), iGlobalEqnslst.clone());
    }
    Ok((outVars, outShared, outRepl, outUnReplaceable, outEqnslst, outGlobalFoundSimple, warnAliasConflicts))
}

fn causalFinder1(mut finished: bool, mut b: bool, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut index: i32, mut maxTraversals: i32, mut iVars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut iRepl: BackendVarTransform::VariableReplacements, mut iUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGlobalEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inGlobalFoundSimple: bool, mut warnAliasConflicts: bool) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outVars: BackendDAE::Variables = iVars.clone();
    let mut outShared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut outRepl: BackendVarTransform::VariableReplacements = iRepl.clone();
    let mut outUnReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = iUnreplaceable.clone();
    let mut outEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = listAppend(iEqnslst.clone(), iGlobalEqnslst.clone());
    let mut warnAliasConflicts: bool = warnAliasConflicts;
    (outVars, outShared, outRepl, outUnReplaceable, outEqnslst) = (::match_deref::match_deref! { match &((finished.clone(), b.clone(), iEqnslst.clone(), index.clone(), iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), iGlobalEqnslst.clone())) {
        (true, _, _, _, _, _, _, _, _) => {
            (iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), listAppend(iEqnslst.clone(), iGlobalEqnslst.clone()))
        },
        (_, false, Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
            (iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), iGlobalEqnslst.clone())
        },
        (_, false, _, _, _, _, _, _, _) => {
            (iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), listAppend(iEqnslst.clone(), iGlobalEqnslst.clone()))
        },
        (_, true, _, _, _, _, _, _, _) => {
            let mut b1: bool = false;
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut simpleeqnslst: Arc<metamodelica::List<SimpleContainer>> = metamodelica::nil();
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            (vars, shared, eqnslst, simpleeqnslst, _, _, b1) = List::fold(iEqnslst.clone(), (std::sync::Arc::new(simpleEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), (iVars.clone(), ishared.clone(), metamodelica::nil(), metamodelica::nil(), 1, iMT.clone(), false));
            (outVars, outShared, outRepl, outUnReplaceable, outEqnslst, _, warnAliasConflicts) = causalFinder(b1.clone(), simpleeqnslst.clone(), eqnslst.clone(), index.clone(), maxTraversals.clone(), vars.clone(), shared.clone(), iRepl.clone(), iUnreplaceable.clone(), iMT.clone(), iGlobalEqnslst.clone(), inGlobalFoundSimple.clone(), warnAliasConflicts.clone())?;
            (outVars.clone(), outShared.clone(), outRepl.clone(), outUnReplaceable.clone(), outEqnslst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outVars, outShared, outRepl, outUnReplaceable, outEqnslst, warnAliasConflicts))
}

// =============================================================================
// section for allAcausal
//
// =============================================================================
pub fn allAcausal(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut b: bool = false;
    let mut warnAliasConflicts: bool = false;
    let mut size: i32 = 0;
    let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    size = BackendDAEUtil::daeSize(inDAE.clone());
    size = intMax(BaseHashTable::defaultBucketSize.clone(), (((intReal(size.clone())) * (metamodelica::OrderedFloat(0.7_f64))).0 as i32));
    repl = BackendVarTransform::emptyReplacementsSized(size.clone());
    unReplaceable = HashSet::emptyHashSet();
    unReplaceable = BackendDAEUtil::foldEqSystem(inDAE.clone(), (std::sync::Arc::new(addUnreplaceable) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), unReplaceable.clone())?;
    (_, unReplaceable) = BackendDAEUtil::traverseBackendDAEExps(inDAE.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traverserExpUnreplaceable) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), unReplaceable.clone()))?;
    unReplaceable = addUnreplaceableFromWhens(inDAE.clone(), unReplaceable.clone())?;
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        BackendDump::dumpHashSet(unReplaceable.clone(), (literal!("Unreplaceable Crefs:")).clone())?;
    }
    let (__pa0, (__pa1, _, __pa2, __pa3)) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(allAcausal1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool))> + 'static>), (repl.clone(), unReplaceable.clone(), false, false))?;
    outDAE = __pa0.clone();
    repl = __pa1.clone();
    b = __pa2.clone();
    warnAliasConflicts = __pa3.clone();
    if warnAliasConflicts.clone() && BackendDAEUtil::isSimulationDAE(inDAE.shared.clone()) {
        Error::addMessage(Error::REDUNDANT_ALIAS_SET.clone(), metamodelica::nil())?;
    }
    outDAE = removeSimpleEquationsShared(b.clone(), outDAE.clone(), repl.clone())?;
    Ok(outDAE)
}

fn allAcausal1(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inTpl: (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool))> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outTpl: (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool);
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut b: bool = false;
    let mut b1: bool = false;
    let mut warnAliasConflicts: bool = false;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    if BackendDAEUtil::isClockedSyst(inSystem.clone()) {
        outSystem = inSystem.clone();
        outShared = inShared.clone();
        outTpl = inTpl.clone();
        return Ok((outSystem, outShared, outTpl));
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inSystem.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqns = __pa0.clone();
    vars = __pa1.clone();
    (repl, unReplaceable, b1, warnAliasConflicts) = inTpl.clone();
    eqnslst = BackendEquation::equationList(eqns.clone());
    mT = arrayCreate(BackendVariable::varsSize(vars.clone())?, metamodelica::nil());
    (vars, outShared, repl, unReplaceable, _, eqnslst, b, warnAliasConflicts) = allCausalFinder(eqnslst.clone(), (vars.clone(), inShared.clone(), repl.clone(), unReplaceable.clone(), mT.clone(), metamodelica::nil(), false, warnAliasConflicts.clone()))?;
    outSystem = updateSystem(b.clone(), eqnslst.clone(), vars.clone(), repl.clone(), inSystem.clone())?;
    outTpl = (repl.clone(), unReplaceable.clone(), b.clone() || b1.clone(), warnAliasConflicts.clone());
    Ok((outSystem, outShared, outTpl))
}

// =============================================================================
// section for causal
//
// =============================================================================
pub fn causal(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut b: bool = false;
    let mut warnAliasConflicts: bool = false;
    let mut size: i32 = 0;
    let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    size = BackendDAEUtil::daeSize(inDAE.clone());
    size = intMax(BaseHashTable::defaultBucketSize.clone(), (((intReal(size.clone())) * (metamodelica::OrderedFloat(0.7_f64))).0 as i32));
    repl = BackendVarTransform::emptyReplacementsSized(size.clone());
    unReplaceable = HashSet::emptyHashSet();
    unReplaceable = BackendDAEUtil::foldEqSystem(inDAE.clone(), (std::sync::Arc::new(addUnreplaceable) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), unReplaceable.clone())?;
    (_, unReplaceable) = BackendDAEUtil::traverseBackendDAEExps(inDAE.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traverserExpUnreplaceable) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), unReplaceable.clone()))?;
    unReplaceable = addUnreplaceableFromWhens(inDAE.clone(), unReplaceable.clone())?;
    unReplaceable = addUnreplaceableFromStateSets(inDAE.clone(), unReplaceable.clone())?;
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        BackendDump::dumpHashSet(unReplaceable.clone(), (literal!("Unreplaceable Crefs:")).clone())?;
    }
    let (__pa0, (__pa1, _, __pa2, __pa3)) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(causal1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool))> + 'static>), (repl.clone(), unReplaceable.clone(), false, false))?;
    outDAE = __pa0.clone();
    repl = __pa1.clone();
    b = __pa2.clone();
    warnAliasConflicts = __pa3.clone();
    if warnAliasConflicts.clone() && BackendDAEUtil::isSimulationDAE(inDAE.shared.clone()) {
        Error::addMessage(Error::REDUNDANT_ALIAS_SET.clone(), metamodelica::nil())?;
    }
    outDAE = removeSimpleEquationsShared(b.clone(), outDAE.clone(), repl.clone())?;
    Ok(outDAE)
}

fn causal1(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inTpl: (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool))> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outTpl: (BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool, bool);
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut b: bool = false;
    let mut b1: bool = false;
    let mut warnAliasConflicts: bool = false;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    if BackendDAEUtil::isClockedSyst(inSystem.clone()) {
        outSystem = inSystem.clone();
        outShared = inShared.clone();
        outTpl = inTpl.clone();
        return Ok((outSystem, outShared, outTpl));
    }
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inSystem.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    eqns = __pa1.clone();
    vars = __pa2.clone();
    (repl, unReplaceable, b1, warnAliasConflicts) = inTpl.clone();
    mT = arrayCreate(BackendVariable::varsSize(vars.clone())?, metamodelica::nil());
    (vars, outShared, repl, unReplaceable, _, eqnslst, b, warnAliasConflicts) = traverseComponents(comps.clone(), eqns.clone(), (std::sync::Arc::new(allCausalFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool, bool)> + 'static>), (vars.clone(), inShared.clone(), repl.clone(), unReplaceable.clone(), mT.clone(), metamodelica::nil(), false, warnAliasConflicts.clone()))?;
    outSystem = updateSystem(b.clone(), eqnslst.clone(), vars.clone(), repl.clone(), inSystem.clone())?;
    outTpl = (repl.clone(), unReplaceable.clone(), b.clone() || b1.clone(), warnAliasConflicts.clone());
    Ok((outSystem, outShared, outTpl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn traverseComponents<Type_a: Clone + 'static>(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Type_a) -> Result<Type_a> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Type_a) -> Result<Type_a> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = (::match_deref::match_deref! { match &((inComps.clone(), iEqns.clone(), inFunc.clone(), inTypeA.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            inTypeA.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, .. }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut arg: Type_a;
            eqn = BackendEquation::get(iEqns.clone(), e.clone());
            arg = inFunc(list![eqn.clone()], inTypeA.clone())?;
            traverseComponents(rest.clone(), iEqns.clone(), inFunc.clone(), arg.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: elst, .. }, tail: rest }, _, _, _) => {
            let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut arg: Type_a;
            eqnlst = BackendEquation::getList(elst.clone(), iEqns.clone());
            arg = inFunc(eqnlst.clone(), inTypeA.clone())?;
            traverseComponents(rest.clone(), iEqns.clone(), inFunc.clone(), arg.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: e, .. }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut arg: Type_a;
            eqn = BackendEquation::get(iEqns.clone(), e.clone());
            arg = inFunc(list![eqn.clone()], inTypeA.clone())?;
            traverseComponents(rest.clone(), iEqns.clone(), inFunc.clone(), arg.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: e, .. }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut arg: Type_a;
            eqn = BackendEquation::get(iEqns.clone(), e.clone());
            arg = inFunc(list![eqn.clone()], inTypeA.clone())?;
            traverseComponents(rest.clone(), iEqns.clone(), inFunc.clone(), arg.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: e, .. }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut arg: Type_a;
            eqn = BackendEquation::get(iEqns.clone(), e.clone());
            arg = inFunc(list![eqn.clone()], inTypeA.clone())?;
            traverseComponents(rest.clone(), iEqns.clone(), inFunc.clone(), arg.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: e, .. }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut arg: Type_a;
            eqn = BackendEquation::get(iEqns.clone(), e.clone());
            arg = inFunc(list![eqn.clone()], inTypeA.clone())?;
            traverseComponents(rest.clone(), iEqns.clone(), inFunc.clone(), arg.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: e, .. }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut arg: Type_a;
            eqn = BackendEquation::get(iEqns.clone(), e.clone());
            arg = inFunc(list![eqn.clone()], inTypeA.clone())?;
            traverseComponents(rest.clone(), iEqns.clone(), inFunc.clone(), arg.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: elst, .. }, .. }, tail: rest }, _, _, _) => {
            let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnlst1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut arg: Type_a;
            let mut elst = (*elst).clone();
            eqnlst = BackendEquation::getList(elst.clone(), iEqns.clone());
            (elst, _, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
            eqnlst1 = BackendEquation::getList(elst.clone(), iEqns.clone());
            arg = inFunc(listAppend(eqnlst.clone(), eqnlst1.clone()), inTypeA.clone())?;
            traverseComponents(rest.clone(), iEqns.clone(), inFunc.clone(), arg.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTypeA)
}

fn allCausalFinder(mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inTpl: (BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool, bool)> {
    let mut outTpl: (BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool, bool);
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut b: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    let mut globalFoundSimple: bool = false;
    let mut warnAliasConflicts: bool = false;
    let mut globaleqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut simpleeqnslst: Arc<metamodelica::List<SimpleContainer>> = metamodelica::nil();
    (vars, shared, repl, unReplaceable, mt, globaleqnslst, b, warnAliasConflicts) = inTpl.clone();
    (eqnslst, b2) = BackendVarTransform::replaceEquations(eqns.clone(), repl.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
    (_, _, eqnslst, simpleeqnslst, _, _, b1) = List::fold(eqnslst.clone(), (std::sync::Arc::new(simpleEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), (vars.clone(), shared.clone(), metamodelica::nil(), metamodelica::nil(), 1, mt.clone(), false));
    (vars, shared, repl, unReplaceable, eqnslst, globalFoundSimple, b3) = allCausalFinder1(b1.clone(), b2.clone(), simpleeqnslst.clone(), eqnslst.clone(), vars.clone(), shared.clone(), repl.clone(), unReplaceable.clone(), mt.clone(), globaleqnslst.clone(), b.clone(), warnAliasConflicts.clone())?;
    warnAliasConflicts = warnAliasConflicts.clone() || b3.clone();
    outTpl = (vars.clone(), shared.clone(), repl.clone(), unReplaceable.clone(), mt.clone(), eqnslst.clone(), globalFoundSimple.clone(), warnAliasConflicts.clone());
    Ok(outTpl)
}

fn allCausalFinder1(mut foundSimple: bool, mut didReplacement: bool, mut iSimpleeqnslst: Arc<metamodelica::List<SimpleContainer>>, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iVars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut iRepl: BackendVarTransform::VariableReplacements, mut iUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGlobalEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut globalFoundSimple: bool, mut warnAliasConflicts: bool) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool, bool)> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut outUnReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut outEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outGlobalFoundSimple: bool = false;
    let mut warnAliasConflicts: bool = warnAliasConflicts;
    (outVars, outShared, outRepl, outUnReplaceable, outEqnslst, outGlobalFoundSimple, warnAliasConflicts) = (::match_deref::match_deref! { match &((foundSimple.clone(), didReplacement.clone(), iSimpleeqnslst.clone(), iEqnslst.clone(), iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), iGlobalEqnslst.clone(), globalFoundSimple.clone())) {
        (false, _, _, Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
            (iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), iGlobalEqnslst.clone(), didReplacement.clone() || globalFoundSimple.clone(), warnAliasConflicts.clone())
        },
        (false, _, _, _, _, _, _, _, _, _) => {
            (iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), listAppend(iEqnslst.clone(), iGlobalEqnslst.clone()), didReplacement.clone() || globalFoundSimple.clone(), warnAliasConflicts.clone())
        },
        (true, _, _, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut simpleeqns: metamodelica::Array<SimpleContainer>;
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            simpleeqns = List::listArrayReverse(iSimpleeqnslst.clone())?;
            (vars, eqnslst, shared, repl, b) = handleSets((simpleeqns.clone().borrow().len() as i32), 1, simpleeqns.clone(), iMT.clone(), iUnreplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone())?;
            warnAliasConflicts = warnAliasConflicts.clone() || b.clone();
            (eqnslst, b1) = BackendVarTransform::replaceEquations(eqnslst.clone(), repl.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
            allCausalFinder2(b1.clone(), eqnslst.clone(), vars.clone(), shared.clone(), repl.clone(), iUnreplaceable.clone(), iMT.clone(), iGlobalEqnslst.clone(), true, warnAliasConflicts.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outVars, outShared, outRepl, outUnReplaceable, outEqnslst, outGlobalFoundSimple, warnAliasConflicts))
}

fn allCausalFinder2(mut b: bool, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iVars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut iRepl: BackendVarTransform::VariableReplacements, mut iUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGlobalEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut globalFoundSimple: bool, mut warnAliasConflicts: bool) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool, bool)> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut outUnReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut outEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outGlobalFoundSimple: bool = false;
    let mut warnAliasConflicts: bool = warnAliasConflicts;
    (outVars, outShared, outRepl, outUnReplaceable, outEqnslst, outGlobalFoundSimple, warnAliasConflicts) = (::match_deref::match_deref! { match &((b.clone(), iEqnslst.clone(), iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), iGlobalEqnslst.clone(), globalFoundSimple.clone())) {
        (false, Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
            (iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), iGlobalEqnslst.clone(), globalFoundSimple.clone(), warnAliasConflicts.clone())
        },
        (false, _, _, _, _, _, _, _) => {
            (iVars.clone(), ishared.clone(), iRepl.clone(), iUnreplaceable.clone(), listAppend(iEqnslst.clone(), iGlobalEqnslst.clone()), globalFoundSimple.clone(), warnAliasConflicts.clone())
        },
        (true, _, _, _, _, _, _, _) => {
            let mut b1: bool = false;
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut simpleeqnslst: Arc<metamodelica::List<SimpleContainer>> = metamodelica::nil();
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            (vars, shared, eqnslst, simpleeqnslst, _, _, b1) = List::fold(iEqnslst.clone(), (std::sync::Arc::new(simpleEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), (iVars.clone(), ishared.clone(), metamodelica::nil(), metamodelica::nil(), 1, iMT.clone(), false));
            allCausalFinder1(b1.clone(), false, simpleeqnslst.clone(), eqnslst.clone(), vars.clone(), shared.clone(), iRepl.clone(), iUnreplaceable.clone(), iMT.clone(), iGlobalEqnslst.clone(), globalFoundSimple.clone(), warnAliasConflicts.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outVars, outShared, outRepl, outUnReplaceable, outEqnslst, outGlobalFoundSimple, warnAliasConflicts))
}

// =============================================================================
// functions to find simple equations
//
// =============================================================================
fn simpleEquationsFinder(mut eqn: Arc<BackendDAE::Equation>, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: e2, exp: e1 }, _) => {
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr((literal!("Found Equation ")).clone(), e1.clone(), (literal!(" = ")).clone(), e2.clone(), (literal!(" to handle.\n")).clone())?;
                    }
                    Ok(simpleEquationAcausal(e1.clone(), e2.clone(), (source.clone(), eqAttr.clone()), false, inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: eqAttr, source, right: e2, left: e1, .. }, _) => {
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr((literal!("Found Array Equation ")).clone(), e1.clone(), (literal!(" = ")).clone(), e2.clone(), (literal!(" to handle.\n")).clone())?;
                    }
                    Ok(simpleArrayEquationAcausal(e1.clone(), e2.clone(), Expression::r#typeof(e1.clone())?, (source.clone(), eqAttr.clone()), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: eqAttr, source, exp: e2, componentRef: cr }, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    e1 = Expression::crefExp(cr.clone())?;
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr((literal!("Found Solved Equation ")).clone(), e1.clone(), (literal!(" = ")).clone(), e2.clone(), (literal!(" to handle.\n")).clone())?;
                    }
                    Ok(simpleEquationAcausal(e1.clone(), e2.clone(), (source.clone(), eqAttr.clone()), false, inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: eqAttr, source, exp: e1 }, _) => {
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStr((literal!("Found Residual Equation ")).clone(), e1.clone(), (literal!(" to handle.\n")).clone())?;
                    }
                    Ok(simpleExpressionAcausal(e1.clone(), (source.clone(), eqAttr.clone()), false, inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: eqAttr, source, right: e2, left: e1, .. }, _) => {
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr((literal!("Found Complex Equation ")).clone(), e1.clone(), (literal!(" = ")).clone(), e2.clone(), (literal!(" to handle.\n")).clone())?;
                    }
                    Ok(simpleEquationAcausal(e1.clone(), e2.clone(), (source.clone(), eqAttr.clone()), false, inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (v, s, eqns, seqns, index, mT, b)) => {
                    Ok((v.clone(), s.clone(), cons(eqn.clone(), eqns.clone()), seqns.clone(), index.clone(), mT.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn simpleEquationAcausal(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut selfCalled: bool, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), lhs.clone(), false, cr2.clone(), rhs.clone(), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: lhs.clone() }), false, cr2.clone(), rhs.clone(), true, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: lhs.clone() }), false, cr2.clone(), rhs.clone(), true, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), lhs.clone(), true, cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: rhs.clone() }), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), lhs.clone(), true, cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: rhs.clone() }), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: lhs.clone() }), false, cr2.clone(), rhs.clone(), true, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), lhs.clone(), true, cr2.clone(), Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: lhs.clone() }), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::ARRAY { array: elst1, .. }, Deref @ DAE::Exp::ARRAY { array: elst2, .. }, _, _, _) => {
            List::threadFold2(elst1.clone(), elst2.clone(), (std::sync::Arc::new(simpleEquationAcausal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, (Arc<DAE::ElementSource>, BackendDAE::EquationAttributes), bool, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), eqnAttributes.clone(), true, inTpl.clone())?
        },
        (Deref @ DAE::Exp::MATRIX { matrix: elstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: elstlst2, .. }, _, _, _) => {
            List::threadFold2(elstlst1.clone(), elstlst2.clone(), (std::sync::Arc::new(simpleEquationAcausalLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>, (Arc<DAE::ElementSource>, BackendDAE::EquationAttributes), bool, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), eqnAttributes.clone(), true, inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::ARRAY { ty, .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::MATRIX { ty, .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::ARRAY { ty, .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::MATRIX { ty, .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::ARRAY { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::MATRIX { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::ARRAY { ty, .. }, Deref @ DAE::Exp::CREF { .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::MATRIX { ty, .. }, Deref @ DAE::Exp::CREF { .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::ARRAY { ty, .. } }, Deref @ DAE::Exp::CREF { .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::MATRIX { ty, .. } }, Deref @ DAE::Exp::CREF { .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::ARRAY { ty, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::MATRIX { ty, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::ARRAY { ty, .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::MATRIX { ty, .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::ARRAY { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::MATRIX { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::ARRAY { ty, .. }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::MATRIX { ty, .. }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::ARRAY { ty, .. } }, Deref @ DAE::Exp::CREF { .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::MATRIX { ty, .. } }, Deref @ DAE::Exp::CREF { .. }, _, _, _) => {
            simpleArrayEquationAcausal(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        _ => {
            simpleEquationAcausal1(lhs.clone(), rhs.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn simpleArrayEquationAcausal(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>, mut eqnAttributes: EquationSourceAndAttributes, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut ds: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
    let mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut elst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut hasInlineAfterIndexReduction: bool = false;
    let mut expandLhs: bool = false;
    let mut expandRhs: bool = false;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut attr: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    dims = Expression::arrayDimension(ty.clone());
    ds = Expression::dimensionsSizes(dims.clone());
    subslst = List::map(ds.clone(), (std::sync::Arc::new(fnptr!(Expression::dimensionSizeSubscripts, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> + 'static>));
    subslst = Expression::rangesToSubscripts(subslst.clone())?;
    if subslst.clone().is_empty() {
        (source, attr) = eqnAttributes.clone();
        outTpl = inTpl.clone();
        return Ok(outTpl);
        for mut e in &*list![lhs.clone(), rhs.clone()] {
            let mut e = e.clone();
            if Expression::isEvaluatedConst(e.clone()) {
                continue;
            }
            eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: Arc::new(BackendDAE::WhenEquation { condition: Arc::new(DAE::Exp::BCONST { bool: false }), whenStmtLst: list![BackendDAE::WhenOperator::ASSERT { condition: Arc::new(DAE::Exp::BCONST { bool: false }), message: Arc::new(DAE::Exp::SCONST { string: (literal!("Failed assertion exp is 0")).clone() }), level: DAE::ASSERTIONLEVEL_ERROR().clone(), source: source.clone() }], elsewhenPart: None }), source: source.clone(), attr: attr.clone() });
            outTpl = simpleEquationsFinder(eq.clone(), outTpl.clone())?;
        }
        return Ok(outTpl);
    }
    (_, hasInlineAfterIndexReduction) = Expression::traverseExpTopDown(lhs.clone(), (std::sync::Arc::new(fnptr!(Expression::findCallIsInlineAfterIndexReduction, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
    (_, hasInlineAfterIndexReduction) = Expression::traverseExpTopDown(rhs.clone(), (std::sync::Arc::new(fnptr!(Expression::findCallIsInlineAfterIndexReduction, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), hasInlineAfterIndexReduction.clone())?;
    (elst1, expandLhs) = List::mapFold(subslst.clone(), Arc::new({ let __pe_b0 = lhs.clone(); move |__pe_a1, __pe_a2| Ok(Expression::applyExpSubscriptsFoldCheckSimplify(__pe_b0.clone(), __pe_a1, __pe_a2)) }), false);
    (elst2, expandRhs) = List::mapFold(subslst.clone(), Arc::new({ let __pe_b0 = rhs.clone(); move |__pe_a1, __pe_a2| Ok(Expression::applyExpSubscriptsFoldCheckSimplify(__pe_b0.clone(), __pe_a1, __pe_a2)) }), false);
    if !(hasInlineAfterIndexReduction.clone()) {
        if false && !(expandLhs.clone() && expandRhs.clone()) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("RemoveSimpleEquations.simpleArrayEquationAcausal")); __mm_s.push_str(&*literal!(" not expanding ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        let true = (expandLhs.clone() && expandRhs.clone()) else { bail!("pattern mismatch") };
    }
    outTpl = List::threadFold2(elst1.clone(), elst2.clone(), (std::sync::Arc::new(simpleEquationAcausal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, (Arc<DAE::ElementSource>, BackendDAE::EquationAttributes), bool, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), eqnAttributes.clone(), true, inTpl.clone())?;
    Ok(outTpl)
}

fn simpleEquationAcausalLst(mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut elst2: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut eqnAttributes: EquationSourceAndAttributes, mut selfCalled: bool, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = List::threadFold2(elst1.clone(), elst2.clone(), (std::sync::Arc::new(simpleEquationAcausal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, (Arc<DAE::ElementSource>, BackendDAE::EquationAttributes), bool, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?;
    Ok(outTpl)
}

fn simpleEquationAcausal1(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut selfCalled: bool, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (lhs.clone(), rhs.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut elst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    elst1 = Expression::splitRecord(lhs.clone(), Expression::r#typeof(lhs.clone())?)?;
                    elst2 = Expression::splitRecord(rhs.clone(), Expression::r#typeof(rhs.clone())?)?;
                    Ok(List::threadFold2(elst1.clone(), elst2.clone(), (std::sync::Arc::new(simpleEquationAcausal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, (Arc<DAE::ElementSource>, BackendDAE::EquationAttributes), bool, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), eqnAttributes.clone(), true, inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: elst1, .. }, _, _, _, _) => {
                    if !((Expression::isZero(rhs.clone()))) { bail!("guard") }
                    Ok(List::fold2(elst1.clone(), (std::sync::Arc::new(simpleExpressionAcausal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ElementSource>, BackendDAE::EquationAttributes), bool, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), eqnAttributes.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::ARRAY { array: elst2, .. }, _, _, _) => {
                    if !((Expression::isZero(lhs.clone()))) { bail!("guard") }
                    Ok(List::fold2(elst2.clone(), (std::sync::Arc::new(simpleExpressionAcausal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ElementSource>, BackendDAE::EquationAttributes), bool, (BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, bool)> + 'static>), eqnAttributes.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    if !((Expression::isZero(rhs.clone()))) { bail!("guard") }
                    Ok(simpleExpressionAcausal(lhs.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    if !((Expression::isZero(lhs.clone()))) { bail!("guard") }
                    Ok(simpleExpressionAcausal(rhs.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(timeIndependentEquationAcausal(lhs.clone(), rhs.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn generateEquation(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>, mut eqnAttributes: EquationSourceAndAttributes, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (source, eqAttr), (v, s, eqns, seqns, index, mT, b)) => {
                    if !((DAEUtil::expTypeComplex(ty.clone()))) { bail!("guard") }
                    let mut size: i32 = 0;
                    size = Expression::sizeOf(ty.clone())?;
                    Ok((v.clone(), s.clone(), cons(Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: eqAttr.clone() }), eqns.clone()), seqns.clone(), index.clone(), mT.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (source, eqAttr), (v, s, eqns, seqns, index, mT, b)) => {
                    if !((DAEUtil::expTypeArray(ty.clone()))) { bail!("guard") }
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ds: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut recordSize: Option<i32> = None;
                    dims = Expression::arrayDimension(ty.clone());
                    ds = Expression::dimensionsSizes(dims.clone());
                    tp = DAEUtil::expTypeElementType(ty.clone());
                    if DAEUtil::expTypeComplex(tp.clone()) {
                        recordSize = Some(Expression::sizeOf(tp.clone())?);
                    } else {
                        recordSize = None;
                    }
                    Ok((v.clone(), s.clone(), cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), eqns.clone()), seqns.clone(), index.clone(), mT.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (source, eqAttr), (v, s, eqns, seqns, index, mT, b)) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    b1 = DAEUtil::expTypeComplex(ty.clone());
                    b2 = DAEUtil::expTypeArray(ty.clone());
                    let false = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok((v.clone(), s.clone(), cons(Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: eqAttr.clone() }), eqns.clone()), seqns.clone(), index.clone(), mT.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAEOptimize.generateEquation failed on: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn simpleExpressionAcausal(mut exp: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut selfCalled: bool, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &((exp.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::ADD { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() }), false, cr2.clone(), e2.clone(), true, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::ADD_ARR { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e1.clone() }), false, cr2.clone(), e2.clone(), true, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::SUB { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::ADD { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::SUB { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), true, cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() }), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::SUB_ARR { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, _, _, _) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), true, cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), false, eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: e2.clone() }), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: e2.clone() }), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, _, _, _) => {
            simpleArrayEquationAcausal(e1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), ty.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        _ => {
            timeIndependentExpressionAcausal(exp.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn addSimpleEquationAcausal(mut cr1: Arc<DAE::ComponentRef>, mut inE1: Arc<DAE::Exp>, mut negatedCr1: bool, mut cr2: Arc<DAE::ComponentRef>, mut inE2: Arc<DAE::Exp>, mut negatedCr2: bool, mut eqnAttributes: EquationSourceAndAttributes, mut genEqn: bool, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (cr1.clone(), inE1.clone(), negatedCr1.clone(), cr2.clone(), inE2.clone(), negatedCr2.clone(), eqnAttributes.clone(), genEqn.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, (vars, shared, eqns, seqns, index, mT, _)) => {
                    let mut vars1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut vars2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut ilst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut ilst2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varskn1: bool = false;
                    let mut varskn2: bool = false;
                    let mut time1: bool = false;
                    let mut time2: bool = false;
                    let mut seqns = (*seqns).clone();
                    let mut index = (*index).clone();
                    let mut mT = (*mT).clone();
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrCrefStrCrefStr((literal!("Alias Equation ")).clone(), cr1.clone(), (literal!(" = ")).clone(), cr2.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" found. Negated lhs[")); __mm_s.push_str(&*boolString(negatedCr1.clone())); __mm_s.push_str(&*literal!("] = rhs[")); __mm_s.push_str(&*boolString(negatedCr2.clone())); __mm_s.push_str(&*literal!("].\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (vars1, ilst1, varskn1, time1) = getVars(cr1.clone(), vars.clone(), shared.clone())?;
                    (vars2, ilst2, varskn2, time2) = getVars(cr2.clone(), vars.clone(), shared.clone())?;
                    let true = (intEq((vars1.clone().len() as i32), (vars2.clone().len() as i32))) else { bail!("pattern mismatch") };
                    (seqns, index, mT) = generateSimpleContainters(vars1.clone(), negatedCr1.clone(), ilst1.clone(), varskn1.clone(), time1.clone(), vars2.clone(), negatedCr2.clone(), ilst2.clone(), varskn2.clone(), time2.clone(), eqnAttributes.clone(), seqns.clone(), index.clone(), mT.clone())?;
                    Ok((vars.clone(), shared.clone(), eqns.clone(), seqns.clone(), index.clone(), mT.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, true, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr((literal!("Non Alias Equation ")).clone(), inE1.clone(), (literal!(" = ")).clone(), inE2.clone(), (literal!(" to generate.\n")).clone())?;
                    }
                    e1 = Expression::crefExp(cr1.clone())?;
                    ty = Expression::r#typeof(e1.clone())?;
                    e2 = inE2.clone();
                    Ok(generateEquation(e1.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn getVars(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, bool, bool)> {
    let mut oVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oIndexs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varskn: bool = false;
    let mut time_: bool = false;
    (oVars, oIndexs, varskn, time_) = 'mc: {
        let __mc_input = (cr.clone(), vars.clone(), shared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: Deref @ "time", .. }, _, _) => {
                    Ok((metamodelica::nil(), metamodelica::nil(), true, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut oVars: Arc<metamodelica::List<BackendDAE::Var>> = oVars.clone();
                    let mut oIndexs: Arc<metamodelica::List<i32>> = oIndexs.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (__pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    oVars = __pa0.clone();
                    oIndexs = __pa1.clone();
                    Ok((oVars.clone(), oIndexs.clone(), false, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut oVars: Arc<metamodelica::List<BackendDAE::Var>> = oVars.clone();
                    let mut oIndexs: Arc<metamodelica::List<i32>> = oIndexs.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendVariable::getVarShared(cr.clone(), shared.clone())?) {
                        (__pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    oVars = __pa0.clone();
                    oIndexs = __pa1.clone();
                    if ComponentReference::crefIsScalarWithVariableSubs(cr.clone())? {
                        oVars = metamodelica::nil();
                        oIndexs = metamodelica::nil();
                    }
                    Ok((oVars.clone(), oIndexs.clone(), true, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oVars, oIndexs, varskn, time_))
}

fn generateSimpleContainters(mut vars1: Arc<metamodelica::List<BackendDAE::Var>>, mut negatedCr1: bool, mut ilst1: Arc<metamodelica::List<i32>>, mut varskn1: bool, mut time1: bool, mut vars2: Arc<metamodelica::List<BackendDAE::Var>>, mut negatedCr2: bool, mut ilst2: Arc<metamodelica::List<i32>>, mut varskn2: bool, mut time2: bool, mut eqnAttributes: EquationSourceAndAttributes, mut iSeqns: Arc<metamodelica::List<SimpleContainer>>, mut iIndex: i32, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSeqns: Arc<metamodelica::List<SimpleContainer>> = metamodelica::nil();
    let mut oIndex: i32 = 0;
    let mut oMT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (oSeqns, oIndex, oMT) = (::match_deref::match_deref! { match &((vars1.clone(), negatedCr1.clone(), ilst1.clone(), varskn1.clone(), time1.clone(), vars2.clone(), negatedCr2.clone(), ilst2.clone(), varskn2.clone(), time2.clone(), eqnAttributes.clone(), iSeqns.clone(), iIndex.clone(), iMT.clone())) {
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, _, Deref @ metamodelica::List::Cons { head: i1, tail: Deref @ metamodelica::List::Nil }, true, true, Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr2, .. }, tail: Deref @ metamodelica::List::Nil }, _, Deref @ metamodelica::List::Cons { head: i2, tail: Deref @ metamodelica::List::Nil }, false, false, _, _, _, _) => {
            let mut colum: Arc<metamodelica::List<i32>> = metamodelica::nil();
            colum = iMT.borrow()[(i2.clone()-1) as usize].clone();
            {let _arr = iMT.clone(); _arr.borrow_mut()[(i2.clone()-1) as usize] = cons(iIndex.clone(), colum.clone()); _arr};
            (cons(SimpleContainer::TIMEALIAS { cr1: cr2.clone(), negatedCr1: negatedCr2.clone(), i1: i2.clone(), cr2: cr1.clone(), negatedCr2: negatedCr1.clone(), i2: i1.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, iSeqns.clone()), iIndex.clone() + 1, iMT.clone())
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, _, Deref @ metamodelica::List::Cons { head: i1, tail: Deref @ metamodelica::List::Nil }, false, false, Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr2, .. }, tail: Deref @ metamodelica::List::Nil }, _, Deref @ metamodelica::List::Cons { head: i2, tail: Deref @ metamodelica::List::Nil }, true, true, _, _, _, _) => {
            let mut colum: Arc<metamodelica::List<i32>> = metamodelica::nil();
            colum = iMT.borrow()[(i1.clone()-1) as usize].clone();
            {let _arr = iMT.clone(); _arr.borrow_mut()[(i1.clone()-1) as usize] = cons(iIndex.clone(), colum.clone()); _arr};
            (cons(SimpleContainer::TIMEALIAS { cr1: cr1.clone(), negatedCr1: negatedCr1.clone(), i1: i1.clone(), cr2: cr2.clone(), negatedCr2: negatedCr2.clone(), i2: i2.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, iSeqns.clone()), iIndex.clone() + 1, iMT.clone())
        },
        (Deref @ metamodelica::List::Nil, _, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _) => {
            (iSeqns.clone(), iIndex.clone(), iMT.clone())
        },
        (Deref @ metamodelica::List::Cons { head: v1, tail: vlst1 }, _, Deref @ metamodelica::List::Cons { head: i1, tail: irest1 }, _, false, Deref @ metamodelica::List::Cons { head: v2, tail: vlst2 }, _, Deref @ metamodelica::List::Cons { head: i2, tail: irest2 }, _, false, _, _, _, _) => {
            let mut seqns: Arc<metamodelica::List<SimpleContainer>> = metamodelica::nil();
            let mut index: i32 = 0;
            let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            (seqns, index, mT) = generateSimpleContainter(v1.clone(), negatedCr1.clone(), i1.clone(), varskn1.clone(), v2.clone(), negatedCr2.clone(), i2.clone(), varskn2.clone(), eqnAttributes.clone(), iSeqns.clone(), iIndex.clone(), iMT.clone())?;
            (seqns, index, mT) = generateSimpleContainters(vlst1.clone(), negatedCr1.clone(), irest1.clone(), varskn1.clone(), time1.clone(), vlst2.clone(), negatedCr2.clone(), irest2.clone(), varskn2.clone(), time2.clone(), eqnAttributes.clone(), seqns.clone(), index.clone(), mT.clone())?;
            (seqns.clone(), index.clone(), mT.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oSeqns, oIndex, oMT))
}

fn generateSimpleContainter(mut v1: BackendDAE::Var, mut negatedCr1: bool, mut i1: i32, mut varskn1: bool, mut v2: BackendDAE::Var, mut negatedCr2: bool, mut i2: i32, mut varskn2: bool, mut eqnAttributes: EquationSourceAndAttributes, mut iSeqns: Arc<metamodelica::List<SimpleContainer>>, mut iIndex: i32, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<SimpleContainer>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSeqns: Arc<metamodelica::List<SimpleContainer>> = metamodelica::nil();
    let mut oIndex: i32 = 0;
    let mut oMT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (oSeqns, oIndex, oMT) = (::match_deref::match_deref! { match &((v1.clone(), negatedCr1.clone(), i1.clone(), varskn1.clone(), v2.clone(), negatedCr2.clone(), i2.clone(), varskn2.clone(), eqnAttributes.clone(), iSeqns.clone(), iIndex.clone(), iMT.clone())) {
        (BackendDAE::Var { varName: cr1, .. }, _, _, false, BackendDAE::Var { varName: cr2, .. }, _, _, false, _, _, _, _) => {
            let mut colum: Arc<metamodelica::List<i32>> = metamodelica::nil();
            checkEqualAlias(intEq(i1.clone(), i2.clone()), v1.clone(), negatedCr1.clone(), v2.clone(), negatedCr2.clone(), eqnAttributes.clone())?;
            colum = iMT.borrow()[(i1.clone()-1) as usize].clone();
            {let _arr = iMT.clone(); _arr.borrow_mut()[(i1.clone()-1) as usize] = cons(iIndex.clone(), colum.clone()); _arr};
            colum = iMT.borrow()[(i2.clone()-1) as usize].clone();
            {let _arr = iMT.clone(); _arr.borrow_mut()[(i2.clone()-1) as usize] = cons(iIndex.clone(), colum.clone()); _arr};
            (cons(SimpleContainer::ALIAS { cr1: cr1.clone(), negatedCr1: negatedCr1.clone(), i1: i1.clone(), cr2: cr2.clone(), negatedCr2: negatedCr2.clone(), i2: i2.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, iSeqns.clone()), iIndex.clone() + 1, iMT.clone())
        },
        (BackendDAE::Var { varName: cr1, .. }, _, _, true, BackendDAE::Var { varName: cr2, .. }, _, _, false, _, _, _, _) => {
            let mut colum: Arc<metamodelica::List<i32>> = metamodelica::nil();
            colum = iMT.borrow()[(i2.clone()-1) as usize].clone();
            {let _arr = iMT.clone(); _arr.borrow_mut()[(i2.clone()-1) as usize] = cons(iIndex.clone(), colum.clone()); _arr};
            (cons(SimpleContainer::PARAMETERALIAS { unknowncr: cr2.clone(), negatedCr1: negatedCr2.clone(), i1: i2.clone(), paramcr: cr1.clone(), negatedCr2: negatedCr1.clone(), i2: i1.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, iSeqns.clone()), iIndex.clone() + 1, iMT.clone())
        },
        (BackendDAE::Var { varName: cr1, .. }, _, _, false, BackendDAE::Var { varName: cr2, .. }, _, _, true, _, _, _, _) => {
            let mut colum: Arc<metamodelica::List<i32>> = metamodelica::nil();
            colum = iMT.borrow()[(i1.clone()-1) as usize].clone();
            {let _arr = iMT.clone(); _arr.borrow_mut()[(i1.clone()-1) as usize] = cons(iIndex.clone(), colum.clone()); _arr};
            (cons(SimpleContainer::PARAMETERALIAS { unknowncr: cr1.clone(), negatedCr1: negatedCr1.clone(), i1: i1.clone(), paramcr: cr2.clone(), negatedCr2: negatedCr2.clone(), i2: i2.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, iSeqns.clone()), iIndex.clone() + 1, iMT.clone())
        },
        (BackendDAE::Var { varName: cr1, .. }, _, _, true, BackendDAE::Var { varName: cr2, .. }, _, _, true, (source, _), _, _, _) => {
            let mut crexp1: Arc<DAE::Exp>;
            let mut crexp2: Arc<DAE::Exp>;
            let mut lhs: ArcStr = arcstr::literal!("");
            let mut rhs: ArcStr = arcstr::literal!("");
            crexp1 = Expression::crefExp(cr1.clone())?;
            crexp2 = Expression::crefExp(cr2.clone())?;
            crexp1 = negateExpression(negatedCr1.clone(), crexp1.clone(), crexp1.clone(), (literal!(" generateSimpleContainter ")).clone())?;
            crexp2 = negateExpression(negatedCr2.clone(), crexp2.clone(), crexp2.clone(), (literal!(" generateSimpleContainter ")).clone())?;
            lhs = (ExpressionBasics::printExpStr(crexp1.clone())?).clone();
            rhs = (ExpressionBasics::printExpStr(crexp2.clone())?).clone();
            Error::addSourceMessage(Error::EQ_WITHOUT_TIME_DEP_VARS.clone(), list![(lhs.clone()).clone(), (rhs.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oSeqns, oIndex, oMT))
}

fn checkEqualAlias(mut equal: bool, mut v1: BackendDAE::Var, mut negatedCr1: bool, mut v2: BackendDAE::Var, mut negatedCr2: bool, mut eqnAttributes: EquationSourceAndAttributes) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((equal.clone(), v1.clone(), negatedCr1.clone(), v2.clone(), negatedCr2.clone(), eqnAttributes.clone())) {
        (false, _, _, _, _, _) => {
            ()
        },
        (true, BackendDAE::Var { varName: cr1, .. }, _, BackendDAE::Var { varName: cr2, .. }, _, (source, _)) => {
            let mut crexp1: Arc<DAE::Exp>;
            let mut crexp2: Arc<DAE::Exp>;
            let mut eqn_str: ArcStr = arcstr::literal!("");
            let mut var_str: ArcStr = arcstr::literal!("");
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            var_str = (BackendDump::varString(v1.clone())?).clone();
            crexp1 = Expression::crefExp(cr1.clone())?;
            crexp2 = Expression::crefExp(cr2.clone())?;
            crexp1 = negateExpression(negatedCr1.clone(), crexp1.clone(), crexp1.clone(), (literal!(" checkEqualAlias ")).clone())?;
            crexp2 = negateExpression(negatedCr2.clone(), crexp2.clone(), crexp2.clone(), (literal!(" checkEqualAlias ")).clone())?;
            eqn_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(crexp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(crexp2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            Error::addSourceMessage(Error::STRUCT_SINGULAR_SYSTEM.clone(), list![(eqn_str.clone()).clone(), (var_str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn timeIndependentEquationAcausal(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut selfCalled: bool, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (lhs.clone(), rhs.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, (vars, Deref @ BackendDAE::Shared { globalKnownVars, .. }, _, _, _, _, _)) => {
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut tree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(lhs.clone(), (std::sync::Arc::new(traversingTimeVarsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>))> + 'static>), (false, vars.clone(), globalKnownVars.clone(), false, false, metamodelica::nil()))?) {
                        (_, (false, _, _, _, _, __pa0)) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ilst = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(rhs.clone(), (std::sync::Arc::new(traversingTimeVarsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>))> + 'static>), (false, vars.clone(), globalKnownVars.clone(), false, false, ilst.clone()))?) {
                        (_, (false, _, _, _, _, __pa1)) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ilst = __pa1.clone();
                    tree = AvlSetInt::new();
                    tree = AvlSetInt::addList(tree.clone(), ilst.clone())?;
                    ilst = AvlSetInt::listKeys(tree.clone(), metamodelica::nil());
                    vlst = List::map1r(ilst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    Ok(solveTimeIndependentAcausal(vlst.clone(), ilst.clone(), lhs.clone(), rhs.clone(), eqnAttributes.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, true, _) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = Expression::r#typeof(lhs.clone())?;
                    Ok(generateEquation(lhs.clone(), rhs.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn timeIndependentExpressionAcausal(mut exp: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut selfCalled: bool, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (exp.clone(), eqnAttributes.clone(), selfCalled.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (vars, Deref @ BackendDAE::Shared { globalKnownVars, .. }, _, _, _, _, _)) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut tree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(traversingTimeVarsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>))> + 'static>), (false, vars.clone(), globalKnownVars.clone(), false, false, metamodelica::nil()))?) {
                        (_, (false, _, _, _, _, __pa0)) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ilst = __pa0.clone();
                    tree = AvlSetInt::new();
                    tree = AvlSetInt::addList(tree.clone(), ilst.clone())?;
                    ilst = AvlSetInt::listKeys(tree.clone(), metamodelica::nil());
                    vlst = List::map1r(ilst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    ty = Expression::r#typeof(exp.clone())?;
                    e2 = Expression::makeConstZero(ty.clone());
                    Ok(solveTimeIndependentAcausal(vlst.clone(), ilst.clone(), exp.clone(), e2.clone(), eqnAttributes.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, true, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = Expression::r#typeof(exp.clone())?;
                    e2 = Expression::makeConstZero(ty.clone());
                    Ok(generateEquation(exp.clone(), e2.clone(), ty.clone(), eqnAttributes.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn toplevelInputOrUnfixed(mut inVar: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = BackendVariable::isVarOnTopLevelAndInput(inVar.clone()) || BackendVariable::varUnreplaceable(inVar.clone()) || BackendVariable::isParam(inVar.clone()) && !(BackendVariable::varFixed(inVar.clone()));
    b
}

fn traversingTimeVarsFinder(mut inExp: Arc<DAE::Exp>, mut inTuple: (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTuple: (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, Arc<metamodelica::List<i32>>);
    (outExp, cont, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: Deref @ "time", .. }, ty: _ }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?) {
                        (__pa0, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varlst = __pa0.clone();
                    let false = (List::none(varlst.clone(), (std::sync::Arc::new(fnptr!(toplevelInputOrUnfixed, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    vlst = __pa0.clone();
                    vlst = listAppend(ilst.clone(), vlst.clone());
                    Ok((inExp.clone(), true, (b.clone(), vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), vlst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (b, _, _, _, _, _)) => {
                    Ok((inExp.clone(), !(b.clone()), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTuple))
}

fn solveTimeIndependentAcausal(mut vlst: Arc<metamodelica::List<BackendDAE::Var>>, mut ilst: Arc<metamodelica::List<i32>>, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &((vlst.clone(), ilst.clone(), lhs.clone(), rhs.clone(), eqnAttributes.clone(), inTpl.clone())) {
        (Deref @ metamodelica::List::Cons { head: v @ BackendDAE::Var { varName: cr, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, _, _, _, _) => {
            let mut cre: Arc<DAE::Exp>;
            let mut es: Arc<DAE::Exp>;
            cre = Expression::crefExp(cr.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(ExpressionSolve::solve(lhs.clone(), rhs.clone(), cre.clone(), None)?) {
                (__pa0, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            es = __pa0.clone();
            constOrAliasAcausal(v.clone(), i.clone(), cr.clone(), es.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        (_, _, _, _, (source, eqAttr), (_, Deref @ BackendDAE::Shared { .. }, _, _, _, _, _)) => {
            let mut size: i32 = 0;
            size = Expression::sizeOf(Expression::r#typeof(lhs.clone())?)?;
            let true = (intEq(size.clone(), (vlst.clone().len() as i32))) else { bail!("pattern mismatch") };
            solveTimeIndependentAcausal1(vlst.clone(), ilst.clone(), lhs.clone(), rhs.clone(), (source.clone(), eqAttr.clone()), inTpl.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpl)
}

fn solveTimeIndependentAcausal1(mut vlst: Arc<metamodelica::List<BackendDAE::Var>>, mut ilst: Arc<metamodelica::List<i32>>, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &((vlst.clone(), ilst.clone(), lhs.clone(), rhs.clone(), eqnAttributes.clone(), inTpl.clone())) {
        (_, _, _, _, _, _) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cre: Arc<DAE::Exp>;
            let mut es: Arc<DAE::Exp>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::map(vlst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>))) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            crlst = __pa1.clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            let true = (List::all(crlst.clone(), Arc::new({ let __pe_b0 = cr.clone(); move |__pe_a1| ComponentReferenceBasics::crefPrefixOf(__pe_b0.clone(), __pe_a1) }))) else { bail!("pattern mismatch") };
            cre = Expression::crefExp(cr.clone())?;
            let __pa2 = ::match_deref::match_deref! { match &(ExpressionSolve::solve(lhs.clone(), rhs.clone(), cre.clone(), None)?) {
                (__pa2, Deref @ metamodelica::List::Nil) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            es = __pa2.clone();
            constOrAliasArrayAcausal(vlst.clone(), ilst.clone(), es.clone(), eqnAttributes.clone(), inTpl.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn constOrAliasArrayAcausal(mut vars: Arc<metamodelica::List<BackendDAE::Var>>, mut indxs: Arc<metamodelica::List<i32>>, mut exp: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &((vars.clone(), indxs.clone(), exp.clone(), eqnAttributes.clone(), inTpl.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            inTpl.clone()
        },
        (Deref @ metamodelica::List::Cons { head: v @ BackendDAE::Var { varName: cr, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: i, tail: ilst }, _, _, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut tpl: AccTuple;
            subs = ComponentReference::crefLastSubs(cr.clone())?;
            e = Expression::applyExpSubscripts(exp.clone(), subs.clone())?;
            tpl = constOrAliasAcausal(v.clone(), i.clone(), cr.clone(), e.clone(), eqnAttributes.clone(), inTpl.clone())?;
            constOrAliasArrayAcausal(vlst.clone(), ilst.clone(), exp.clone(), eqnAttributes.clone(), tpl.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpl)
}

fn constOrAliasAcausal(mut var: BackendDAE::Var, mut i: i32, mut cr: Arc<DAE::ComponentRef>, mut exp: Arc<DAE::Exp>, mut eqnAttributes: EquationSourceAndAttributes, mut inTpl: AccTuple) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (var.clone(), i.clone(), cr.clone(), exp.clone(), eqnAttributes.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (vars, shared, eqns, seqns, index, mT, _)) => {
                    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut cra: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut vars2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut ilst2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut negated: bool = false;
                    let mut seqns = (*seqns).clone();
                    let mut index = (*index).clone();
                    let mut mT = (*mT).clone();
                    (negated, cra) = aliasExp(exp.clone())?;
                    globalKnownVars = BackendVariable::daeGlobalKnownVars(shared.clone());
                    (vars2, ilst2) = BackendVariable::getVar(cra.clone(), globalKnownVars.clone())?;
                    (seqns, index, mT) = generateSimpleContainters(list![var.clone()], false, list![i.clone()], false, false, vars2.clone(), negated.clone(), ilst2.clone(), true, false, eqnAttributes.clone(), seqns.clone(), index.clone(), mT.clone())?;
                    Ok((vars.clone(), shared.clone(), eqns.clone(), seqns.clone(), index.clone(), mT.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (vars, shared, eqns, seqns, index, mT, _)) => {
                    if !((Expression::isConstValue(exp.clone())?)) { bail!("guard") }
                    let mut colum: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrCrefStrExpStr((literal!("Const Equation ")).clone(), cr.clone(), (literal!(" = ")).clone(), exp.clone(), (literal!(" found.\n")).clone())?;
                    }
                    colum = mT.borrow()[(i.clone()-1) as usize].clone();
                    {let _arr = mT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = cons(index.clone(), colum.clone()); _arr};
                    Ok((vars.clone(), shared.clone(), eqns.clone(), cons(SimpleContainer::TIMEINDEPENTVAR { cr: cr.clone(), i: i.clone(), exp: exp.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, seqns.clone()), index.clone() + 1, mT.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (vars, shared @ Deref @ BackendDAE::Shared { functionTree: functions, .. }, eqns, seqns, index, mT, _)) => {
                    if !((!(Expression::isImpure(exp.clone())?) && !(Expression::containsRecordType(exp.clone())?))) { bail!("guard") }
                    let mut colum: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut exp2: Arc<DAE::Exp>;
                    exp2 = EvaluateFunctions::evaluateConstantFunctionCallExp(exp.clone(), functions.clone(), false, Flags::getConfigInt(Flags::EVAL_RECURSION_LIMIT.clone())?)?;
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrCrefStrExpStr((literal!("Const Equation (through Ceval, case 1) ")).clone(), cr.clone(), (literal!(" = ")).clone(), exp.clone(), (literal!(" found.\n")).clone())?;
                    }
                    colum = mT.borrow()[(i.clone()-1) as usize].clone();
                    {let _arr = mT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = cons(index.clone(), colum.clone()); _arr};
                    Ok((vars.clone(), shared.clone(), eqns.clone(), cons(SimpleContainer::TIMEINDEPENTVAR { cr: cr.clone(), i: i.clone(), exp: exp2.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, seqns.clone()), index.clone() + 1, mT.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (vars, shared, eqns, seqns, index, mT, _)) => {
                    if !((!(Expression::isImpure(exp.clone())?) && !(Expression::containsRecordType(exp.clone())?))) { bail!("guard") }
                    let mut colum: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrCrefStrExpStr((literal!("Const Equation (through Ceval, case 2) ")).clone(), cr.clone(), (literal!(" = ")).clone(), exp.clone(), (literal!(" found.\n")).clone())?;
                    }
                    colum = mT.borrow()[(i.clone()-1) as usize].clone();
                    {let _arr = mT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = cons(index.clone(), colum.clone()); _arr};
                    Ok((vars.clone(), shared.clone(), eqns.clone(), cons(SimpleContainer::TIMEINDEPENTVAR { cr: cr.clone(), i: i.clone(), exp: exp.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, seqns.clone()), index.clone() + 1, mT.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn aliasExp(mut exp: Arc<DAE::Exp>) -> Result<(bool, Arc<DAE::ComponentRef>)> {
    let mut negate: bool = false;
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (negate, outCr) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            (false, cr.clone())
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            (true, cr.clone())
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            (true, cr.clone())
        },
        Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            (true, cr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((negate, outCr))
}

fn handleSets(mut containerIdx: i32, mut inMark: i32, mut containerArr: metamodelica::Array<SimpleContainer>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut vars: BackendDAE::Variables, mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>, mut repl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, bool)> {
    let mut vars: BackendDAE::Variables = vars;
    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = eqnslst;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut warnAliasConflicts: bool = false;
    let mut rmax: Option<(i32, i32)> = None;
    let mut smax: Option<(i32, i32)> = None;
    let mut unremovable: Option<i32> = None;
    let mut r#const: Option<i32> = None;
    let mut mark: i32 = inMark.clone();
    let mut b: bool = false;
    for mut idx in (1..=containerIdx.clone()).rev() {
        if !(intGt(getVisited(containerArr.borrow()[(idx.clone()-1) as usize].clone())?, 0)) {
            (rmax, smax, unremovable, r#const, _) = getAlias(list![idx.clone()], None, mark.clone(), containerArr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), false, metamodelica::nil(), None, None, None, None)?;
            (vars, eqnslst, shared, repl, b) = handleSet(rmax.clone(), smax.clone(), unremovable.clone(), r#const.clone(), mark.clone() + 1, containerArr.clone(), iMT.clone(), unReplaceable.clone(), vars.clone(), eqnslst.clone(), shared.clone(), repl.clone())?;
            mark = mark.clone() + 2;
            warnAliasConflicts = warnAliasConflicts.clone() || b.clone();
        }
    }
    Ok((vars, eqnslst, shared, repl, warnAliasConflicts))
}

fn getAlias(mut rows: Arc<metamodelica::List<i32>>, mut prevVar: Option<i32>, mut mark: i32, mut containerArr: metamodelica::Array<SimpleContainer>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut negate: bool, mut stack: Arc<metamodelica::List<i32>>, mut iRmax: Option<(i32, i32)>, mut iSmax: Option<(i32, i32)>, mut iUnremovable: Option<i32>, mut iConst: Option<i32>) -> Result<(Option<(i32, i32)>, Option<(i32, i32)>, Option<i32>, Option<i32>, bool)> {
    let mut oRmax: Option<(i32, i32)> = None;
    let mut oSmax: Option<(i32, i32)> = None;
    let mut oUnremovable: Option<i32> = None;
    let mut oConst: Option<i32> = None;
    let mut oContinue: bool = false;
    (oRmax, oSmax, oUnremovable, oConst, oContinue) = (::match_deref::match_deref! { match &((rows.clone(), prevVar.clone(), mark.clone(), containerArr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), negate.clone(), stack.clone(), iRmax.clone(), iSmax.clone(), iUnremovable.clone(), iConst.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _, _) => {
            (iRmax.clone(), iSmax.clone(), iUnremovable.clone(), iConst.clone(), true)
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut container: SimpleContainer;
            let mut rmax: Option<(i32, i32)> = None;
            let mut smax: Option<(i32, i32)> = None;
            let mut unremovable: Option<i32> = None;
            let mut r#const: Option<i32> = None;
            let mut visited: bool = false;
            let mut cont: bool = false;
            container = containerArr.borrow()[(r.clone()-1) as usize].clone();
            visited = isVisited(mark.clone(), container.clone())?;
            (rmax, smax, unremovable, r#const, cont) = getAlias1(visited.clone(), container.clone(), r.clone(), rest.clone(), prevVar.clone(), mark.clone(), containerArr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), negate.clone(), stack.clone(), iRmax.clone(), iSmax.clone(), iUnremovable.clone(), iConst.clone())?;
            (rmax.clone(), smax.clone(), unremovable.clone(), r#const.clone(), cont.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oRmax, oSmax, oUnremovable, oConst, oContinue))
}

fn getAlias1(mut visited: bool, mut containerIn: SimpleContainer, mut currIdx: i32, mut rows: Arc<metamodelica::List<i32>>, mut prevVar: Option<i32>, mut mark: i32, mut containerArr: metamodelica::Array<SimpleContainer>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut negate: bool, mut stack: Arc<metamodelica::List<i32>>, mut iRmax: Option<(i32, i32)>, mut iSmax: Option<(i32, i32)>, mut iUnremovable: Option<i32>, mut iConst: Option<i32>) -> Result<(Option<(i32, i32)>, Option<(i32, i32)>, Option<i32>, Option<i32>, bool)> {
    let mut oRmax: Option<(i32, i32)> = None;
    let mut oSmax: Option<(i32, i32)> = None;
    let mut oUnremovable: Option<i32> = None;
    let mut oConst: Option<i32> = None;
    let mut oContinue: bool = false;
    (oRmax, oSmax, oUnremovable, oConst, oContinue) = 'mc: {
        let __mc_input = (visited.clone(), containerIn.clone(), currIdx.clone(), rows.clone(), prevVar.clone(), mark.clone(), containerArr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), negate.clone(), stack.clone(), iRmax.clone(), iSmax.clone(), iUnremovable.clone(), iConst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut rmax: Option<(i32, i32)> = None;
                    let mut smax: Option<(i32, i32)> = None;
                    let mut unremovable: Option<i32> = None;
                    let mut r#const: Option<i32> = None;
                    let mut cont: bool = false;
                    {let _arr = containerArr.clone(); _arr.borrow_mut()[(currIdx.clone()-1) as usize] = setVisited(mark.clone(), containerIn.clone())?; _arr};
                    (rmax, smax, unremovable, r#const, cont) = getAlias2(containerIn.clone(), currIdx.clone(), prevVar.clone(), mark.clone(), containerArr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), negate.clone(), cons(currIdx.clone(), stack.clone()), iRmax.clone(), iSmax.clone(), iUnremovable.clone(), iConst.clone())?;
                    if cont.clone() {
                        (rmax, smax, unremovable, r#const, cont) = getAlias(rows.clone(), prevVar.clone(), mark.clone(), containerArr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), negate.clone(), stack.clone(), rmax.clone(), smax.clone(), unremovable.clone(), r#const.clone())?;
                    }
                    Ok((rmax.clone(), smax.clone(), unremovable.clone(), r#const.clone(), cont.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, _, _, _, _, _, _, _, _, true, _, _, _, Some(_), _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let SimpleContainer::ALIAS { cr1: __pa0, .. } = (containerArr.borrow()[(currIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    cr = __pa0.clone();
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(ComponentReference::crefLastType(cr.clone())?)?) else { bail!("pattern mismatch") };
                    Ok((None, None, None, iUnremovable.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, _, _, _, _, _, _, _, _, true, _, _, _, _, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let SimpleContainer::ALIAS { cr1: __pa0, .. } = (containerArr.borrow()[(currIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    cr = __pa0.clone();
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(ComponentReference::crefLastType(cr.clone())?)?) else { bail!("pattern mismatch") };
                    Ok((None, None, None, Some(currIdx.clone()), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    msg = (literal!("Circular Equalities Detected for Variables:\n")).clone();
                    msg = (circularEqualityMsg(stack.clone(), currIdx.clone(), containerArr.clone(), (msg.clone()).clone())?).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(msg.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oRmax, oSmax, oUnremovable, oConst, oContinue))
}

fn circularEqualityMsg(mut stack: Arc<metamodelica::List<i32>>, mut iR: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>, mut iMsg: ArcStr) -> Result<ArcStr> {
    let mut oMsg: ArcStr = arcstr::literal!("");
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut msg: ArcStr = arcstr::literal!("");
    lst = circularEqualityMsg_dispatch(stack.clone(), iR.clone(), simpleeqnsarr.clone())?;
    msg = stringDelimitList(lst.clone(), (literal!("\n")).clone());
    msg = stringAppendList(list![(iMsg.clone()).clone(), (msg.clone()).clone(), (literal!("\n")).clone()]);
    oMsg = (msg.clone()).clone();
    Ok(oMsg)
}

fn circularEqualityMsg_dispatch(mut stack: Arc<metamodelica::List<i32>>, mut iR: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut oMsg: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut slst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut r in &*stack.clone() {
        let mut r = r.clone();
        if r.clone() == iR.clone() {
            break;
        }
        let __range0 = &*getVarsNames(simpleeqnsarr.borrow()[(r.clone()-1) as usize].clone())?;
        for mut n in __range0 {
            let mut n = n.clone();
            oMsg = cons((ComponentReferenceBasics::printComponentRefStr(n.clone())?).clone(), oMsg.clone());
        }
        oMsg = cons((literal!("----------------------------------")).clone(), oMsg.clone());
    }
    oMsg.clone().reverse();
    Ok(oMsg)
}

fn getVarsNames(mut iS: SimpleContainer) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut names: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    names = (match iS.clone() {
        SimpleContainer::ALIAS { cr2: mut cr2, cr1: mut cr1, .. } => {
            list![cr1.clone(), cr2.clone()]
        },
        SimpleContainer::PARAMETERALIAS { paramcr: ref cr2, unknowncr: ref cr1, .. } => {
            list![cr1.clone(), cr2.clone()]
        },
        SimpleContainer::TIMEALIAS { cr2: mut cr2, cr1: mut cr1, .. } => {
            list![cr1.clone(), cr2.clone()]
        },
        SimpleContainer::TIMEINDEPENTVAR { cr: ref cr1, .. } => {
            list![cr1.clone()]
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(names)
}

fn getAlias2(mut containerIn: SimpleContainer, mut currIdx: i32, mut prevVar: Option<i32>, mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut negate: bool, mut stack: Arc<metamodelica::List<i32>>, mut iRmax: Option<(i32, i32)>, mut iSmax: Option<(i32, i32)>, mut iUnremovable: Option<i32>, mut iConst: Option<i32>) -> Result<(Option<(i32, i32)>, Option<(i32, i32)>, Option<i32>, Option<i32>, bool)> {
    let mut oRmax: Option<(i32, i32)> = None;
    let mut oSmax: Option<(i32, i32)> = None;
    let mut oUnremovable: Option<i32> = None;
    let mut oConst: Option<i32> = None;
    let mut oContinue: bool = false;
    (oRmax, oSmax, oUnremovable, oConst, oContinue) = (::match_deref::match_deref! { match &((containerIn.clone(), currIdx.clone(), prevVar.clone(), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), negate.clone(), stack.clone(), iRmax.clone(), iSmax.clone(), iUnremovable.clone(), iConst.clone())) {
        (SimpleContainer::ALIAS { negatedCr2, i2, negatedCr1, i1, .. }, _, None, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut adjEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rmax: Option<(i32, i32)> = None;
            let mut smax: Option<(i32, i32)> = None;
            let mut unremovable: Option<i32> = None;
            let mut r#const: Option<i32> = None;
            let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut state: bool = false;
            let mut replaceable_: bool = false;
            let mut cont: bool = false;
            let mut replaceble1: bool = false;
            let mut neg: bool = false;
            neg = boolOr(negatedCr1.clone(), negatedCr2.clone());
            adjEqs = List::removeOnTrue(currIdx.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i1.clone()-1) as usize].clone());
            v = BackendVariable::getVarAt(vars.clone(), i1.clone())?;
            (replaceable_, replaceble1) = replaceableAlias(v.clone(), unReplaceable.clone())?;
            state = BackendVariable::isStateVar(v.clone()) || BackendVariable::isClockedStateVar(v.clone());
            (rmax, smax, unremovable) = getAlias3(v.clone(), i1.clone(), state.clone(), replaceable_.clone() && replaceble1.clone(), currIdx.clone(), iRmax.clone(), iSmax.clone(), iUnremovable.clone())?;
            neg = if (neg.clone()) {!(negate.clone())} else {negate.clone()};
            (rmax, smax, unremovable, r#const, cont) = getAlias(adjEqs.clone(), Some(i1.clone()), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), neg.clone(), stack.clone(), rmax.clone(), smax.clone(), unremovable.clone(), iConst.clone())?;
            adjEqs = List::removeOnTrue(currIdx.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i2.clone()-1) as usize].clone());
            v = BackendVariable::getVarAt(vars.clone(), i2.clone())?;
            (replaceable_, replaceble1) = replaceableAlias(v.clone(), unReplaceable.clone())?;
            state = BackendVariable::isStateVar(v.clone()) || BackendVariable::isClockedStateVar(v.clone());
            (rmax, smax, unremovable) = getAlias3(v.clone(), i2.clone(), state.clone(), replaceable_.clone() && replaceble1.clone(), currIdx.clone(), rmax.clone(), smax.clone(), unremovable.clone())?;
            if cont.clone() {
                (rmax, smax, unremovable, r#const, cont) = getAlias(adjEqs.clone(), Some(i2.clone()), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), neg.clone(), stack.clone(), rmax.clone(), smax.clone(), unremovable.clone(), r#const.clone())?;
            }
            (rmax.clone(), smax.clone(), unremovable.clone(), r#const.clone(), cont.clone())
        },
        (SimpleContainer::ALIAS { negatedCr2, i2, negatedCr1, i1, .. }, _, Some(prevVarIdx), _, _, _, _, _, _, _, _, _, _, _) => {
            let mut adjEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rmax: Option<(i32, i32)> = None;
            let mut smax: Option<(i32, i32)> = None;
            let mut unremovable: Option<i32> = None;
            let mut r#const: Option<i32> = None;
            let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut i: i32 = 0;
            let mut state: bool = false;
            let mut replaceable_: bool = false;
            let mut cont: bool = false;
            let mut replaceble1: bool = false;
            let mut neg: bool = false;
            i = if (intEq(prevVarIdx.clone(), i1.clone())) {i2.clone()} else {i1.clone()};
            neg = boolOr(negatedCr1.clone(), negatedCr2.clone());
            adjEqs = List::removeOnTrue(currIdx.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i.clone()-1) as usize].clone());
            v = BackendVariable::getVarAt(vars.clone(), i.clone())?;
            (replaceable_, replaceble1) = replaceableAlias(v.clone(), unReplaceable.clone())?;
            state = BackendVariable::isStateVar(v.clone()) || BackendVariable::isClockedStateVar(v.clone());
            (rmax, smax, unremovable) = getAlias3(v.clone(), i.clone(), state.clone(), replaceable_.clone() && replaceble1.clone(), currIdx.clone(), iRmax.clone(), iSmax.clone(), iUnremovable.clone())?;
            neg = if (neg.clone()) {!(negate.clone())} else {negate.clone()};
            (rmax, smax, unremovable, r#const, cont) = getAlias(adjEqs.clone(), Some(i.clone()), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), vars.clone(), unReplaceable.clone(), neg.clone(), stack.clone(), rmax.clone(), smax.clone(), unremovable.clone(), iConst.clone())?;
            (rmax.clone(), smax.clone(), unremovable.clone(), r#const.clone(), cont.clone())
        },
        (SimpleContainer::PARAMETERALIAS { .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            (None, None, None, Some(currIdx.clone()), false)
        },
        (SimpleContainer::TIMEALIAS { .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            (None, None, None, Some(currIdx.clone()), false)
        },
        (SimpleContainer::TIMEINDEPENTVAR { .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            (None, None, None, Some(currIdx.clone()), false)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oRmax, oSmax, oUnremovable, oConst, oContinue))
}

fn getAlias3(mut var: BackendDAE::Var, mut i: i32, mut state: bool, mut replaceable_: bool, mut r: i32, mut iRmax: Option<(i32, i32)>, mut iSmax: Option<(i32, i32)>, mut iUnremovable: Option<i32>) -> Result<(Option<(i32, i32)>, Option<(i32, i32)>, Option<i32>)> {
    let mut oRmax: Option<(i32, i32)> = None;
    let mut oSmax: Option<(i32, i32)> = None;
    let mut oUnremovable: Option<i32> = None;
    (oRmax, oSmax, oUnremovable) = (match (var.clone(), i.clone(), state.clone(), replaceable_.clone(), r.clone(), iRmax.clone(), iSmax.clone(), iUnremovable.clone()) {
        (_, _, false, false, _, _, _, None) => {
            let mut w1: i32 = 0;
            w1 = BackendVariable::calcAliasKey(var.clone())?;
            (Some((i.clone(), w1.clone())), iSmax.clone(), Some(i.clone()))
        },
        (_, _, true, false, _, _, _, None) => {
            let mut w1: i32 = 0;
            w1 = BackendVariable::varStateSelectPrioAlias(var.clone())?;
            (iRmax.clone(), Some((i.clone(), w1.clone())), Some(i.clone()))
        },
        (_, _, true, _, _, _, None, _) => {
            let mut w1: i32 = 0;
            w1 = BackendVariable::varStateSelectPrioAlias(var.clone())?;
            (iRmax.clone(), Some((i.clone(), w1.clone())), iUnremovable.clone())
        },
        (_, _, true, _, _, _, Some((_, mut w2)), _) => {
            let mut w1: i32 = 0;
            let mut tpl: Option<(i32, i32)> = None;
            w1 = BackendVariable::varStateSelectPrioAlias(var.clone())?;
            tpl = if (intGt(w1.clone(), w2.clone())) {Some((i.clone(), w1.clone()))} else {iSmax.clone()};
            (iRmax.clone(), tpl.clone(), iUnremovable.clone())
        },
        (_, _, false, _, _, None, _, _) => {
            let mut w1: i32 = 0;
            w1 = BackendVariable::calcAliasKey(var.clone())?;
            (Some((i.clone(), w1.clone())), iSmax.clone(), iUnremovable.clone())
        },
        (_, _, false, _, _, Some((_, mut w2)), _, _) => {
            let mut w1: i32 = 0;
            let mut tpl: Option<(i32, i32)> = None;
            w1 = BackendVariable::calcAliasKey(var.clone())?;
            tpl = if (intLt(w1.clone(), w2.clone())) {Some((i.clone(), w1.clone()))} else {iRmax.clone()};
            (tpl.clone(), iSmax.clone(), iUnremovable.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((oRmax, oSmax, oUnremovable))
}

fn isVisited(mut mark: i32, mut iS: SimpleContainer) -> Result<bool> {
    let mut visited: bool = false;
    visited = intEq(mark.clone(), getVisited(iS.clone())?);
    Ok(visited)
}

fn getVisited(mut iS: SimpleContainer) -> Result<i32> {
    let mut visited: i32 = 0;
    visited = (match iS.clone() {
        SimpleContainer::ALIAS { visited: mut visited, .. } => visited.clone(),
        SimpleContainer::PARAMETERALIAS { visited: mut visited, .. } => visited.clone(),
        SimpleContainer::TIMEALIAS { visited: mut visited, .. } => visited.clone(),
        SimpleContainer::TIMEINDEPENTVAR { visited: mut visited, .. } => visited.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(visited)
}

fn setVisited(mut visited: i32, mut iS: SimpleContainer) -> Result<SimpleContainer> {
    let mut oS: SimpleContainer;
    oS = (match (visited.clone(), iS.clone()) {
        (_, SimpleContainer::ALIAS { cr1: mut cr1, negatedCr1: mut negatedCr1, i1: mut i1, cr2: mut cr2, negatedCr2: mut negatedCr2, i2: mut i2, eqnAttributes: mut eqnAttributes, visited: _ }) => {
            SimpleContainer::ALIAS { cr1: cr1.clone(), negatedCr1: negatedCr1.clone(), i1: i1.clone(), cr2: cr2.clone(), negatedCr2: negatedCr2.clone(), i2: i2.clone(), eqnAttributes: eqnAttributes.clone(), visited: visited.clone() }
        },
        (_, SimpleContainer::PARAMETERALIAS { unknowncr: ref cr1, negatedCr1: mut negatedCr1, i1: mut i1, paramcr: ref cr2, negatedCr2: mut negatedCr2, i2: mut i2, eqnAttributes: mut eqnAttributes, visited: _ }) => {
            SimpleContainer::PARAMETERALIAS { unknowncr: cr1.clone(), negatedCr1: negatedCr1.clone(), i1: i1.clone(), paramcr: cr2.clone(), negatedCr2: negatedCr2.clone(), i2: i2.clone(), eqnAttributes: eqnAttributes.clone(), visited: visited.clone() }
        },
        (_, SimpleContainer::TIMEALIAS { cr1: mut cr1, negatedCr1: mut negatedCr1, i1: mut i1, cr2: mut cr2, negatedCr2: mut negatedCr2, i2: mut i2, eqnAttributes: mut eqnAttributes, visited: _ }) => {
            SimpleContainer::TIMEALIAS { cr1: cr1.clone(), negatedCr1: negatedCr1.clone(), i1: i1.clone(), cr2: cr2.clone(), negatedCr2: negatedCr2.clone(), i2: i2.clone(), eqnAttributes: eqnAttributes.clone(), visited: visited.clone() }
        },
        (_, SimpleContainer::TIMEINDEPENTVAR { cr: ref cr1, i: mut i1, exp: mut exp, eqnAttributes: mut eqnAttributes, visited: _ }) => {
            SimpleContainer::TIMEINDEPENTVAR { cr: cr1.clone(), i: i1.clone(), exp: exp.clone(), eqnAttributes: eqnAttributes.clone(), visited: visited.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(oS)
}

fn replaceableAlias(mut var: BackendDAE::Var, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(bool, bool)> {
    let mut res: bool = false;
    let mut res1: bool = false;
    (res, res1) = 'mc: {
        let __mc_input = (var.clone(), unReplaceable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (BackendDAE::Var { varKind: mut kind, varName: ref cr, .. }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool = false;
            let mut cr = cr.clone();
            BackendVariable::isVarKindVariable(kind.clone())?;
            let false = (BackendVariable::isVarOnTopLevelAndOutput(var.clone())) else { bail!("pattern mismatch") };
            let false = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
            let false = (BackendVariable::varHasUncertainValueRefine(var.clone())) else { bail!("pattern mismatch") };
            cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            b = !(BaseHashSet::has(cr.clone(), unReplaceable.clone())?);
            Ok((true, b.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((false, false))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((res, res1))
}

fn handleSet(mut iRmax: Option<(i32, i32)>, mut iSmax: Option<(i32, i32)>, mut iUnremovable: Option<i32>, mut iConst: Option<i32>, mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iVars: BackendDAE::Variables, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut ishared: Arc<BackendDAE::Shared>, mut iRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, bool)> {
    let mut oVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut oRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut warnAliasConflicts: bool = false;
    (oVars, oEqnslst, oshared, oRepl) = 'mc: {
        let __mc_input = (iRmax.clone(), iSmax.clone(), iUnremovable.clone(), iConst.clone(), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(r), _, _, _, _, _, _, _, _) => {
                    let mut s: SimpleContainer;
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut pv: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut eqnAttributes: EquationSourceAndAttributes;
                    let mut negated: bool = false;
                    let mut replaceable_: bool = false;
                    let mut replaceble1: bool = false;
                    let mut negatedCr1: bool = false;
                    let mut negatedCr2: bool = false;
                    let mut exp2: Arc<DAE::Exp>;
                    let mut expcr: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut vsattr: VarSetAttributes;
                    let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    s = simpleeqnsarr.borrow()[(r.clone()-1) as usize].clone();
                    let SimpleContainer::PARAMETERALIAS { eqnAttributes: __pa0, paramcr: __pa1, i2: __pa2, negatedCr2: __pa3, i1: __pa4, negatedCr1: __pa5, unknowncr: __pa6, .. } = (s.clone()) else { bail!("pattern mismatch") };
                    eqnAttributes = __pa0.clone();
                    cr2 = __pa1.clone();
                    i2 = __pa2.clone();
                    negatedCr2 = __pa3.clone();
                    i1 = __pa4.clone();
                    negatedCr1 = __pa5.clone();
                    cr1 = __pa6.clone();
                    {let _arr = simpleeqnsarr.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = setVisited(mark.clone(), s.clone())?; _arr};
                    negated = boolOr(negatedCr1.clone(), negatedCr2.clone());
                    exp = Expression::crefExp(cr2.clone())?;
                    exp2 = negateExpression(negated.clone(), exp.clone(), exp.clone(), (literal!(" PARAMETERALIAS ")).clone())?;
                    v = BackendVariable::getVarAt(iVars.clone(), i1.clone())?;
                    (replaceable_, replaceble1) = replaceableAlias(v.clone(), unReplaceable.clone())?;
                    (vars, eqnslst, shared, repl) = handleSetVar(replaceable_.clone() && replaceble1.clone(), Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })), v.clone(), i1.clone(), eqnAttributes.clone(), exp2.clone(), iMT.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone())?;
                    expcr = Expression::crefExp(cr1.clone())?;
                    pv = BackendVariable::getVarSharedAt(i2.clone(), ishared.clone())?;
                    vsattr = addVarSetAttributes(pv.clone(), negated.clone(), mark.clone(), simpleeqnsarr.clone(), EMPTYVARSETATTRIBUTES().clone())?;
                    vsattr = if (replaceable_.clone() && replaceble1.clone()) {addVarSetAttributes(v.clone(), negated.clone(), mark.clone(), simpleeqnsarr.clone(), vsattr.clone())?} else {vsattr.clone()};
                    rows = List::removeOnTrue(r.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i1.clone()-1) as usize].clone());
                    {let _arr = iMT.clone(); _arr.borrow_mut()[(i1.clone()-1) as usize] = metamodelica::nil(); _arr};
                    (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(rows.clone(), i1.clone(), exp.clone(), Some(expcr.clone()), negated.clone(), Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), vars.clone(), eqnslst.clone(), shared.clone(), repl.clone(), vsattr.clone())?;
                    Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(r), _, _, _, _, _, _, _, _) => {
                    let mut s: SimpleContainer;
                    let mut i1: i32 = 0;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut eqnAttributes: EquationSourceAndAttributes;
                    let mut negated: bool = false;
                    let mut replaceable_: bool = false;
                    let mut replaceble1: bool = false;
                    let mut negatedCr1: bool = false;
                    let mut negatedCr2: bool = false;
                    let mut exp1: Arc<DAE::Exp>;
                    let mut expcr: Arc<DAE::Exp>;
                    let mut dexp: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut vsattr: VarSetAttributes;
                    let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    s = simpleeqnsarr.borrow()[(r.clone()-1) as usize].clone();
                    let SimpleContainer::TIMEALIAS { eqnAttributes: __pa0, negatedCr2: __pa1, negatedCr1: __pa2, i1: __pa3, cr1: __pa4, .. } = (s.clone()) else { bail!("pattern mismatch") };
                    eqnAttributes = __pa0.clone();
                    negatedCr2 = __pa1.clone();
                    negatedCr1 = __pa2.clone();
                    i1 = __pa3.clone();
                    cr1 = __pa4.clone();
                    {let _arr = simpleeqnsarr.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = setVisited(mark.clone(), s.clone())?; _arr};
                    negated = boolOr(negatedCr1.clone(), negatedCr2.clone());
                    exp = Expression::crefExp(DAE::crefTime().clone())?;
                    exp1 = negateExpression(negated.clone(), exp.clone(), exp.clone(), (literal!(" timealias ")).clone())?;
                    v = BackendVariable::getVarAt(iVars.clone(), i1.clone())?;
                    (replaceable_, replaceble1) = replaceableAlias(v.clone(), unReplaceable.clone())?;
                    dexp = negateExpression(negated.clone(), exp.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), (literal!(" timealias der ")).clone())?;
                    (vars, eqnslst, shared, repl) = handleSetVar(replaceable_.clone() && replaceble1.clone(), Some(dexp.clone()), v.clone(), i1.clone(), eqnAttributes.clone(), exp1.clone(), iMT.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone())?;
                    expcr = Expression::crefExp(cr1.clone())?;
                    vsattr = addVarSetAttributes(v.clone(), negated.clone(), mark.clone(), simpleeqnsarr.clone(), EMPTYVARSETATTRIBUTES().clone())?;
                    rows = List::removeOnTrue(r.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i1.clone()-1) as usize].clone());
                    {let _arr = iMT.clone(); _arr.borrow_mut()[(i1.clone()-1) as usize] = metamodelica::nil(); _arr};
                    (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(rows.clone(), i1.clone(), exp.clone(), Some(expcr.clone()), negated.clone(), Some(dexp.clone()), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), vars.clone(), eqnslst.clone(), shared.clone(), repl.clone(), vsattr.clone())?;
                    Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(r), _, _, _, _, _, _, _, _) => {
                    let mut s: SimpleContainer;
                    let mut i: i32 = 0;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut eqnAttributes: EquationSourceAndAttributes;
                    let mut replaceable_: bool = false;
                    let mut replaceble1: bool = false;
                    let mut constExp: bool = false;
                    let mut isState: bool = false;
                    let mut exp: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut vsattr: VarSetAttributes;
                    let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    s = simpleeqnsarr.borrow()[(r.clone()-1) as usize].clone();
                    let SimpleContainer::TIMEINDEPENTVAR { eqnAttributes: __pa0, exp: __pa1, i: __pa2, cr: __pa3, .. } = (s.clone()) else { bail!("pattern mismatch") };
                    eqnAttributes = __pa0.clone();
                    exp = __pa1.clone();
                    i = __pa2.clone();
                    cr = __pa3.clone();
                    {let _arr = simpleeqnsarr.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = setVisited(mark.clone(), s.clone())?; _arr};
                    let ref __pa5 @ BackendDAE::VAR { varName: ref __pa4, .. } = (BackendVariable::getVarAt(iVars.clone(), i.clone())?) else { bail!("pattern mismatch") };
                    cr = __pa4.clone();
                    v = __pa5.clone();
                    (replaceable_, replaceble1) = replaceableAlias(v.clone(), unReplaceable.clone())?;
                    (vars, shared, isState, eqnslst) = optMoveVarShared(replaceable_.clone(), v.clone(), i.clone(), eqnAttributes.clone(), exp.clone(), (std::sync::Arc::new(BackendVariable::addGlobalKnownVarDAE) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> + 'static>), iMT.clone(), iVars.clone(), ishared.clone(), iEqnslst.clone())?;
                    constExp = Expression::isConstValue(exp.clone())?;
                    repl = if (replaceable_.clone() && constExp.clone() && replaceble1.clone()) {BackendVarTransform::addReplacement(iRepl.clone(), cr.clone(), exp.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?} else {iRepl.clone()};
                    repl = if (isState.clone()) {BackendVarTransform::addDerConstRepl(cr.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), repl.clone())?} else {repl.clone()};
                    exp = Expression::crefExp(cr.clone())?;
                    vsattr = addVarSetAttributes(v.clone(), false, mark.clone(), simpleeqnsarr.clone(), EMPTYVARSETATTRIBUTES().clone())?;
                    rows = List::removeOnTrue(r.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i.clone()-1) as usize].clone());
                    {let _arr = iMT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
                    (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(rows.clone(), i.clone(), exp.clone(), None, false, Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), vars.clone(), eqnslst.clone(), shared.clone(), repl.clone(), vsattr.clone())?;
                    Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(r), _, _, _, _, _, _, _, _) => {
                    let mut s: SimpleContainer;
                    let mut i2: i32 = 0;
                    let mut i: i32 = 0;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut eqnAttributes: EquationSourceAndAttributes;
                    let mut replaceable_: bool = false;
                    let mut replaceble1: bool = false;
                    let mut constExp: bool = false;
                    let mut isState: bool = false;
                    let mut exp: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut vsattr: VarSetAttributes;
                    let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    s = simpleeqnsarr.borrow()[(r.clone()-1) as usize].clone();
                    let SimpleContainer::ALIAS { eqnAttributes: __pa0, i2: __pa1, i1: __pa2, .. } = (s.clone()) else { bail!("pattern mismatch") };
                    eqnAttributes = __pa0.clone();
                    i2 = __pa1.clone();
                    i = __pa2.clone();
                    {let _arr = simpleeqnsarr.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = setVisited(mark.clone(), s.clone())?; _arr};
                    let ref __pa4 @ BackendDAE::VAR { varName: ref __pa3, .. } = (BackendVariable::getVarAt(iVars.clone(), i.clone())?) else { bail!("pattern mismatch") };
                    cr = __pa3.clone();
                    v = __pa4.clone();
                    exp = if (Types::isRealOrSubTypeReal(ComponentReference::crefLastType(cr.clone())?)?) {Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })} else {Arc::new(DAE::Exp::ICONST { integer: 0 })};
                    (replaceable_, replaceble1) = replaceableAlias(v.clone(), unReplaceable.clone())?;
                    (vars, shared, isState, eqnslst) = optMoveVarShared(replaceable_.clone(), v.clone(), i.clone(), eqnAttributes.clone(), exp.clone(), (std::sync::Arc::new(BackendVariable::addGlobalKnownVarDAE) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> + 'static>), iMT.clone(), iVars.clone(), ishared.clone(), iEqnslst.clone())?;
                    constExp = Expression::isConstValue(exp.clone())?;
                    repl = if (replaceable_.clone() && constExp.clone() && replaceble1.clone()) {BackendVarTransform::addReplacement(iRepl.clone(), cr.clone(), exp.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?} else {iRepl.clone()};
                    repl = if (isState.clone()) {BackendVarTransform::addDerConstRepl(cr.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), repl.clone())?} else {repl.clone()};
                    exp = Expression::crefExp(cr.clone())?;
                    vsattr = addVarSetAttributes(v.clone(), false, mark.clone(), simpleeqnsarr.clone(), EMPTYVARSETATTRIBUTES().clone())?;
                    rows = List::removeOnTrue(r.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i2.clone()-1) as usize].clone());
                    {let _arr = iMT.clone(); _arr.borrow_mut()[(i2.clone()-1) as usize] = rows.clone(); _arr};
                    rows = List::removeOnTrue(r.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i.clone()-1) as usize].clone());
                    {let _arr = iMT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
                    (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(rows.clone(), i.clone(), exp.clone(), None, false, Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), vars.clone(), eqnslst.clone(), shared.clone(), repl.clone(), vsattr.clone())?;
                    Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some((i, _)), _, None, _, _, _, _, _, _, _, _) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut exp: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut vsattr: VarSetAttributes;
                    let mut oexp: Option<Arc<DAE::Exp>> = None;
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    let ref __pa1 @ BackendDAE::VAR { varName: ref __pa0, .. } = (BackendVariable::getVarAt(iVars.clone(), i.clone())?) else { bail!("pattern mismatch") };
                    cr = __pa0.clone();
                    v = __pa1.clone();
                    exp = Expression::crefExp(cr.clone())?;
                    vsattr = addVarSetAttributes(v.clone(), false, mark.clone(), simpleeqnsarr.clone(), EMPTYVARSETATTRIBUTES().clone())?;
                    oexp = varStateDerivative(v.clone())?;
                    (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(iMT.borrow()[(i.clone()-1) as usize].clone(), i.clone(), exp.clone(), None, false, oexp.clone(), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone(), vsattr.clone())?;
                    {let _arr = iMT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
                    (vars, warnAliasConflicts) = handleVarSetAttributes(vsattr.clone(), v.clone(), vars.clone(), shared.clone())?;
                    Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, Some(i), None, _, _, _, _, _, _, _, _) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut exp: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut vsattr: VarSetAttributes;
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    let ref __pa1 @ BackendDAE::VAR { varName: ref __pa0, .. } = (BackendVariable::getVarAt(iVars.clone(), i.clone())?) else { bail!("pattern mismatch") };
                    cr = __pa0.clone();
                    v = __pa1.clone();
                    exp = Expression::crefExp(cr.clone())?;
                    vsattr = addVarSetAttributes(v.clone(), false, mark.clone(), simpleeqnsarr.clone(), EMPTYVARSETATTRIBUTES().clone())?;
                    (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(iMT.borrow()[(i.clone()-1) as usize].clone(), i.clone(), exp.clone(), None, false, None, mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone(), vsattr.clone())?;
                    {let _arr = iMT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
                    (vars, warnAliasConflicts) = handleVarSetAttributes(vsattr.clone(), v.clone(), vars.clone(), shared.clone())?;
                    Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some((i, _)), None, _, None, _, _, _, _, _, _, _, _) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut exp: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut vsattr: VarSetAttributes;
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    let ref __pa1 @ BackendDAE::VAR { varName: ref __pa0, .. } = (BackendVariable::getVarAt(iVars.clone(), i.clone())?) else { bail!("pattern mismatch") };
                    cr = __pa0.clone();
                    v = __pa1.clone();
                    exp = Expression::crefExp(cr.clone())?;
                    vsattr = addVarSetAttributes(v.clone(), false, mark.clone(), simpleeqnsarr.clone(), EMPTYVARSETATTRIBUTES().clone())?;
                    (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(iMT.borrow()[(i.clone()-1) as usize].clone(), i.clone(), exp.clone(), None, false, None, mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone(), vsattr.clone())?;
                    {let _arr = iMT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
                    (vars, warnAliasConflicts) = handleVarSetAttributes(vsattr.clone(), v.clone(), vars.clone(), shared.clone())?;
                    Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oVars, oEqnslst, oshared, oRepl, warnAliasConflicts))
}

fn varStateDerivative(mut inVar: BackendDAE::Var) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExp: Option<Arc<DAE::Exp>> = None;
    outExp = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(dcr), .. }, .. } => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::crefExp(dcr.clone())?;
            Some(e.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn handleSetVar(mut replaceable_: bool, mut derReplaceState: Option<Arc<DAE::Exp>>, mut v: BackendDAE::Var, mut i: i32, mut eqnAttributes: EquationSourceAndAttributes, mut exp: Arc<DAE::Exp>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iVars: BackendDAE::Variables, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut ishared: Arc<BackendDAE::Shared>, mut iRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements)> {
    let mut oVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut oRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    (oVars, oEqnslst, oshared, oRepl) = (::match_deref::match_deref! { match &((replaceable_.clone(), derReplaceState.clone(), v.clone(), i.clone(), eqnAttributes.clone(), exp.clone(), iMT.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone())) {
        (true, _, BackendDAE::Var { varName: cr, .. }, _, (source, _), _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut bs: bool = false;
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            (vars, shared, bs) = moveVarShared(v.clone(), i.clone(), source.clone(), exp.clone(), (std::sync::Arc::new(BackendVariable::addAliasVarDAE) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> + 'static>), iVars.clone(), ishared.clone())?;
            repl = BackendVarTransform::addReplacement(iRepl.clone(), cr.clone(), exp.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
            repl = addDerConstRepl(bs.clone(), derReplaceState.clone(), cr.clone(), repl.clone())?;
            (vars.clone(), iEqnslst.clone(), shared.clone(), repl.clone())
        },
        (false, _, BackendDAE::Var { varName: cr, .. }, _, _, _, _, _, _, _, _) => {
            let mut crexp: Arc<DAE::Exp>;
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            crexp = Expression::crefExp(cr.clone())?;
            (vars, shared, eqnslst, _, _, _, _) = generateEquation(crexp.clone(), exp.clone(), Expression::r#typeof(exp.clone())?, eqnAttributes.clone(), (iVars.clone(), ishared.clone(), iEqnslst.clone(), metamodelica::nil(), -1, iMT.clone(), false))?;
            (vars.clone(), eqnslst.clone(), shared.clone(), iRepl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oVars, oEqnslst, oshared, oRepl))
}

fn addDerConstRepl(mut state: bool, mut derConstRepl: Option<Arc<DAE::Exp>>, mut cr: Arc<DAE::ComponentRef>, mut iRepl: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut oRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    oRepl = (::match_deref::match_deref! { match &((state.clone(), derConstRepl.clone(), cr.clone(), iRepl.clone())) {
        (true, Some(e), _, _) => {
            BackendVarTransform::addDerConstRepl(cr.clone(), e.clone(), iRepl.clone())?
        },
        _ => {
            iRepl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oRepl)
}

fn optMoveVarShared(mut replaceable_: bool, mut v: BackendDAE::Var, mut i: i32, mut eqnAttributes: EquationSourceAndAttributes, mut exp: Arc<DAE::Exp>, mut func: Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> + 'static>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iVars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, bool, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    pub type FuncMoveVarShared = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> + 'static>;

    let mut oVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut bs: bool = false;
    let mut oEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (oVars, oshared, bs, oEqnslst) = (::match_deref::match_deref! { match &((replaceable_.clone(), v.clone(), i.clone(), eqnAttributes.clone(), exp.clone(), func.clone(), iMT.clone(), iVars.clone(), ishared.clone(), iEqnslst.clone())) {
        (true, _, _, (source, _), _, _, _, _, _, _) => {
            (oVars, oshared, bs) = moveVarShared(v.clone(), i.clone(), source.clone(), exp.clone(), func.clone(), iVars.clone(), ishared.clone())?;
            (oVars.clone(), oshared.clone(), bs.clone(), iEqnslst.clone())
        },
        (false, BackendDAE::Var { varName: cr, .. }, _, _, _, _, _, _, _, _) => {
            let mut crexp: Arc<DAE::Exp>;
            crexp = Expression::crefExp(cr.clone())?;
            (oVars, oshared, oEqnslst, _, _, _, _) = generateEquation(crexp.clone(), exp.clone(), Expression::r#typeof(exp.clone())?, eqnAttributes.clone(), (iVars.clone(), ishared.clone(), iEqnslst.clone(), metamodelica::nil(), -1, iMT.clone(), false))?;
            (oVars.clone(), oshared.clone(), false, oEqnslst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oVars, oshared, bs, oEqnslst))
}

fn moveVarShared(mut v: BackendDAE::Var, mut i: i32, mut source: Arc<DAE::ElementSource>, mut exp: Arc<DAE::Exp>, mut func: Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> + 'static>, mut iVars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>) -> Result<(BackendDAE::Variables, Arc<BackendDAE::Shared>, bool)> {
    pub type FuncMoveVarShared = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> + 'static>;

    let mut oVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut bs: bool = false;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    let mut v1: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let BackendDAE::VAR { varName: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    cr = __pa0.clone();
    v1 = BackendVariable::setBindExp(v.clone(), Some(exp.clone()));
    ops = ElementSource::getSymbolicTransformations(source.clone());
    v1 = BackendVariable::mergeVariableOperations(v1.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr.clone(), exp: exp.clone() }), ops.clone()));
    bs = BackendVariable::isStateVar(v.clone());
    v1 = if (bs.clone()) {BackendVariable::setVarKind(v1.clone(), crate::BackendDAE::VarKind::DUMMY_STATE)?} else {v1.clone()};
    (oVars, _) = BackendVariable::removeVar(i.clone(), iVars.clone())?;
    oshared = func(v1.clone(), ishared.clone())?;
    Ok((oVars, oshared, bs))
}

fn traverseAliasTree(mut rows: Arc<metamodelica::List<i32>>, mut ilast: i32, mut exp: Arc<DAE::Exp>, mut optExp: Option<Arc<DAE::Exp>>, mut globalnegate: bool, mut derReplaceState: Option<Arc<DAE::Exp>>, mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iVars: BackendDAE::Variables, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut ishared: Arc<BackendDAE::Shared>, mut iRepl: BackendVarTransform::VariableReplacements, mut iAttributes: VarSetAttributes) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, VarSetAttributes)> {
    let mut oVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut oRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut oAttributes: VarSetAttributes;
    (oVars, oEqnslst, oshared, oRepl, oAttributes) = (::match_deref::match_deref! { match &((rows.clone(), ilast.clone(), exp.clone(), optExp.clone(), globalnegate.clone(), derReplaceState.clone(), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone(), iAttributes.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            (iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone(), iAttributes.clone())
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut s: SimpleContainer;
            let mut vsattr: VarSetAttributes;
            s = simpleeqnsarr.borrow()[(r.clone()-1) as usize].clone();
            {let _arr = simpleeqnsarr.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = setVisited(mark.clone(), s.clone())?; _arr};
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree1(s.clone(), r.clone(), ilast.clone(), exp.clone(), optExp.clone(), globalnegate.clone(), derReplaceState.clone(), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone(), iAttributes.clone())?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(rest.clone(), ilast.clone(), exp.clone(), optExp.clone(), globalnegate.clone(), derReplaceState.clone(), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), vars.clone(), eqnslst.clone(), shared.clone(), repl.clone(), vsattr.clone())?;
            (vars.clone(), eqnslst.clone(), shared.clone(), repl.clone(), vsattr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oVars, oEqnslst, oshared, oRepl, oAttributes))
}

fn traverseAliasTree1(mut sc: SimpleContainer, mut r: i32, mut ilast: i32, mut exp: Arc<DAE::Exp>, mut optExp: Option<Arc<DAE::Exp>>, mut globalnegated: bool, mut derReplaceState: Option<Arc<DAE::Exp>>, mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>, mut iMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iVars: BackendDAE::Variables, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut ishared: Arc<BackendDAE::Shared>, mut iRepl: BackendVarTransform::VariableReplacements, mut iAttributes: VarSetAttributes) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, BackendVarTransform::VariableReplacements, VarSetAttributes)> {
    let mut oVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut oRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut oAttributes: VarSetAttributes;
    (oVars, oEqnslst, oshared, oRepl, oAttributes) = (::match_deref::match_deref! { match &((sc.clone(), r.clone(), ilast.clone(), exp.clone(), optExp.clone(), globalnegated.clone(), derReplaceState.clone(), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone(), iAttributes.clone())) {
        (SimpleContainer::ALIAS { cr1: _, negatedCr1, i1, cr2: _, negatedCr2, i2, eqnAttributes: (source, eqAttr), visited: _ }, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut i: i32 = 0;
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut replaceable_: bool = false;
            let mut globalnegated1: bool = false;
            let mut replaceble1: bool = false;
            let mut negated: bool = false;
            let mut crexp: Arc<DAE::Exp>;
            let mut exp1: Arc<DAE::Exp>;
            let mut derReplacement: Option<Arc<DAE::Exp>> = None;
            let mut vsattr: VarSetAttributes;
            let mut source = (*source).clone();
            i = if (intEq(i1.clone(), ilast.clone())) {i2.clone()} else {i1.clone()};
            negated = boolOr(negatedCr2.clone(), negatedCr1.clone());
            let ref __pa1 @ BackendDAE::VAR { varName: ref __pa0, .. } = (BackendVariable::getVarAt(iVars.clone(), i.clone())?) else { bail!("pattern mismatch") };
            cr = __pa0.clone();
            v = __pa1.clone();
            (replaceable_, replaceble1) = replaceableAlias(v.clone(), unReplaceable.clone())?;
            crexp = Expression::crefExp(cr.clone())?;
            globalnegated1 = if (negated.clone()) {!(globalnegated.clone())} else {globalnegated.clone()};
            exp1 = negateExpression(globalnegated1.clone(), exp.clone(), exp.clone(), (literal!(" ALIAS_1 ")).clone())?;
            derReplacement = if (globalnegated1.clone()) {negateOptExp(derReplaceState.clone())?} else {derReplaceState.clone()};
            source = if (replaceable_.clone()) {addSubstitutionOption(optExp.clone(), crexp.clone(), source.clone())?} else {source.clone()};
            (vars, eqnslst, shared, repl) = handleSetVar(replaceable_.clone() && replaceble1.clone(), derReplacement.clone(), v.clone(), i.clone(), (source.clone(), eqAttr.clone()), exp1.clone(), iMT.clone(), iVars.clone(), iEqnslst.clone(), ishared.clone(), iRepl.clone())?;
            vsattr = if (replaceable_.clone() && replaceble1.clone()) {addVarSetAttributes(v.clone(), globalnegated1.clone(), mark.clone(), simpleeqnsarr.clone(), iAttributes.clone())?} else {iAttributes.clone()};
            crexp = negateExpression(negated.clone(), crexp.clone(), crexp.clone(), (literal!(" ALIAS_2 ")).clone())?;
            rows = List::removeOnTrue(r.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iMT.borrow()[(i.clone()-1) as usize].clone());
            {let _arr = iMT.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(rows.clone(), i.clone(), exp.clone(), Some(crexp.clone()), globalnegated1.clone(), derReplaceState.clone(), mark.clone(), simpleeqnsarr.clone(), iMT.clone(), unReplaceable.clone(), vars.clone(), eqnslst.clone(), shared.clone(), repl.clone(), vsattr.clone())?;
            (vars.clone(), eqnslst.clone(), shared.clone(), repl.clone(), vsattr.clone())
        },
        (SimpleContainer::PARAMETERALIAS { unknowncr: cr1, negatedCr1, i1, paramcr: cr2, negatedCr2, i2: _, eqnAttributes: (source, _), visited: _ }, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut negated: bool = false;
            let mut crexp: Arc<DAE::Exp>;
            let mut lhs: ArcStr = arcstr::literal!("");
            let mut rhs: ArcStr = arcstr::literal!("");
            cr = if (intEq(i1.clone(), ilast.clone())) {cr2.clone()} else {cr1.clone()};
            negated = boolOr(negatedCr1.clone(), negatedCr2.clone());
            crexp = Expression::crefExp(cr.clone())?;
            crexp = negateExpression(negated.clone(), crexp.clone(), crexp.clone(), (literal!(" PARAMETERLAIAS ")).clone())?;
            lhs = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            rhs = (ExpressionBasics::printExpStr(crexp.clone())?).clone();
            Error::addSourceMessage(Error::EQ_WITHOUT_TIME_DEP_VARS.clone(), list![(lhs.clone()).clone(), (rhs.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            bail!("fail")
        },
        (SimpleContainer::TIMEALIAS { eqnAttributes: (source, _), .. }, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut rhs: ArcStr = arcstr::literal!("");
            rhs = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            Error::addSourceMessage(Error::EQ_WITHOUT_TIME_DEP_VARS.clone(), list![(literal!("time")).clone(), (rhs.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            bail!("fail")
        },
        (SimpleContainer::TIMEINDEPENTVAR { eqnAttributes: (source, _), exp: exp1, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut lhs: ArcStr = arcstr::literal!("");
            let mut rhs: ArcStr = arcstr::literal!("");
            lhs = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            rhs = (ExpressionBasics::printExpStr(exp1.clone())?).clone();
            Error::addSourceMessage(Error::EQ_WITHOUT_TIME_DEP_VARS.clone(), list![(lhs.clone()).clone(), (rhs.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oVars, oEqnslst, oshared, oRepl, oAttributes))
}

fn negateOptExp(mut iExp: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut oExp: Option<Arc<DAE::Exp>> = None;
    oExp = (::match_deref::match_deref! { match &(iExp.clone()) {
        Some(e) => {
            let mut e = (*e).clone();
            e = negateExpression(true, e.clone(), e.clone(), (literal!(" in negateOptExp ")).clone())?;
            Some(e.clone())
        },
        _ => {
            iExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

fn addSubstitutionOption(mut optExp: Option<Arc<DAE::Exp>>, mut exp: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    let mut e: Arc<DAE::Exp>;
    if isSome(optExp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(optExp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), exp.clone(), e.clone())?;
    }
    Ok(source)
}

fn addVarSetAttributes(mut inVar: BackendDAE::Var, mut negate: bool, mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>, mut iAttributes: VarSetAttributes) -> Result<VarSetAttributes> {
    let mut oAttributes: VarSetAttributes;
    let mut fixed: bool = false;
    let mut fixedset: bool = false;
    let mut start: Option<Arc<DAE::Exp>> = None;
    let mut origin: Option<Arc<DAE::Exp>> = None;
    let mut nominalset: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
    let mut minmaxset: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>);
    let mut startvalues: (i32, Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>);
    (fixedset, startvalues, nominalset, minmaxset) = iAttributes.clone();
    fixed = BackendVariable::varFixed(inVar.clone());
    start = BackendVariable::varStartValueOption(inVar.clone())?;
    origin = BackendVariable::varStartOrigin(inVar.clone())?;
    (fixedset, startvalues) = addStartValue(fixed.clone(), fixedset.clone(), BackendVariable::varCref(inVar.clone())?, start.clone(), origin.clone(), negate.clone(), mark.clone(), simpleeqnsarr.clone(), startvalues.clone())?;
    nominalset = addNominalValue(inVar.clone(), nominalset.clone());
    minmaxset = addMinMaxAttribute(inVar.clone(), negate.clone(), mark.clone(), simpleeqnsarr.clone(), minmaxset.clone())?;
    oAttributes = (fixedset.clone(), startvalues.clone(), nominalset.clone(), minmaxset.clone());
    Ok(oAttributes)
}

fn addStartValue(mut fixed: bool, mut fixedset: bool, mut cr: Arc<DAE::ComponentRef>, mut start: Option<Arc<DAE::Exp>>, mut origin: Option<Arc<DAE::Exp>>, mut negate: bool, mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>, mut iStartvalues: (i32, Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>)) -> Result<(bool, (i32, Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>))> {
    let mut oFixed: bool = false;
    let mut oStartvalues: (i32, Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>);
    (oFixed, oStartvalues) = 'mc: {
        let __mc_input = (fixed.clone(), fixedset.clone(), cr.clone(), start.clone(), origin.clone(), negate.clone(), mark.clone(), simpleeqnsarr.clone(), iStartvalues.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, true, _, _, _, _, _, _, _) => {
                    Ok((fixedset.clone(), iStartvalues.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, false, _, None, _, _, _, _, _) => {
                    let mut originvalue: i32 = 0;
                    originvalue = BackendVariable::startOriginToValue(origin.clone())?;
                    Ok((true, (originvalue.clone(), list![(start.clone(), cr.clone())])))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, false, _, Some(startexp), _, _, _, _, _) => {
                    let mut originvalue: i32 = 0;
                    let mut startexp = (*startexp).clone();
                    startexp = negateExpression(negate.clone(), startexp.clone(), startexp.clone(), (literal!(" start_1 ")).clone())?;
                    originvalue = BackendVariable::startOriginToValue(origin.clone())?;
                    Ok((true, (originvalue.clone(), list![(Some(startexp.clone()), cr.clone())])))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, None, _, _, _, _, (setorigin, startvalues)) => {
                    let mut originvalue: i32 = 0;
                    let mut setorigin = (*setorigin).clone();
                    let mut startvalues = (*startvalues).clone();
                    originvalue = BackendVariable::startOriginToValue(origin.clone())?;
                    if originvalue.clone() > setorigin.clone() {
                        setorigin = originvalue.clone();
                        startvalues = if (fixed.clone()) {list![(start.clone(), cr.clone())]} else {metamodelica::nil()};
                    } else if originvalue.clone() == setorigin.clone() && fixed.clone() {
                        startvalues = cons((start.clone(), cr.clone()), startvalues.clone());
                    }
                    Ok((fixedset.clone(), (setorigin.clone(), startvalues.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(startexp), _, _, _, _, (setorigin, startvalues)) => {
                    let mut originvalue: i32 = 0;
                    let mut startexp = (*startexp).clone();
                    let mut setorigin = (*setorigin).clone();
                    let mut startvalues = (*startvalues).clone();
                    startexp = negateExpression(negate.clone(), startexp.clone(), startexp.clone(), (literal!(" start_2 ")).clone())?;
                    originvalue = BackendVariable::startOriginToValue(origin.clone())?;
                    if originvalue.clone() > setorigin.clone() {
                        setorigin = originvalue.clone();
                        startvalues = list![(Some(startexp.clone()), cr.clone())];
                    } else if originvalue.clone() == setorigin.clone() {
                        startvalues = cons((Some(startexp.clone()), cr.clone()), startvalues.clone());
                    }
                    Ok((fixedset.clone(), (setorigin.clone(), startvalues.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("RemoveSimpleEquations.addStartValue failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oFixed, oStartvalues))
}

fn mergeStartFixedAttributes(mut inVar: BackendDAE::Var, mut fixed: bool, mut startvalues: (i32, Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>), mut ishared: Arc<BackendDAE::Shared>) -> Result<(BackendDAE::Var, bool)> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut warnAliasConflicts: bool = false;
    outVar = 'mc: {
        let __mc_input = (inVar.clone(), fixed.clone(), startvalues.clone(), ishared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (_, Deref @ metamodelica::List::Nil), _) => {
                    Ok(inVar.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true, (_, Deref @ metamodelica::List::Cons { head: (start, _), tail: Deref @ metamodelica::List::Nil }), _) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    v = BackendVariable::setVarFixed(inVar.clone(), true)?;
                    Ok(BackendVariable::setVarStartValueOption(v.clone(), start.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true, (_, Deref @ metamodelica::List::Cons { head: (start, cr), tail: values }), Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut start1: Option<Arc<DAE::Exp>> = None;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut start = (*start).clone();
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    v = BackendVariable::setVarFixed(inVar.clone(), true)?;
                    start1 = optExpReplaceCrefWithBindExp(start.clone(), globalKnownVars.clone())?;
                    (_, start, _) = equalNonFreeStartValues(values.clone(), globalKnownVars.clone(), (start1.clone(), start.clone(), cr.clone()))?;
                    warnAliasConflicts = !(Flags::isSet(Flags::ALIAS_CONFLICTS.clone())?);
                    Ok(BackendVariable::setVarStartValueOption(v.clone(), start.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true, (_, values), Deref @ BackendDAE::Shared { .. }) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut startExp: Arc<DAE::Exp>;
                    let mut zerofreevalues: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut i: i32 = 0;
                    let mut hardcoded: bool = false;
                    if !(Flags::isSet(Flags::ALIAS_CONFLICTS.clone())?) {
                        Error::addMessage(Error::CONFLICTING_ALIAS_SET.clone(), metamodelica::nil())?;
                    } else {
                        let _ = BackendVariable::setVarFixed(inVar.clone(), true)?;
                        zerofreevalues = List::fold(values.clone(), (std::sync::Arc::new(fnptr!(getZeroFreeValues, (Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>), Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>)) as std::sync::Arc<dyn ::std::ops::Fn((Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>), Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>> + 'static>), metamodelica::nil());
                        r#str = (literal!("Conflicting start values for fixed states:\n")).clone();
                        for mut value in &*zerofreevalues.clone() {
                            let mut value = value.clone();
                            (startExp, cr) = value.clone();
                            let (_, (__pa0, __pa1)) = Expression::traverseExpTopDown(startExp.clone(), (std::sync::Arc::new(selectMinDepth) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (i32, bool))> + 'static>), (ComponentReference::crefDepth(cr.clone())?, true))?;
                            i = __pa0.clone();
                            hardcoded = __pa1.clone();
                            if hardcoded.clone() {
                                        i = i.clone() + 5;
                            }
                            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" * Candidate: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!("(start = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(startExp.clone())?); __mm_s.push_str(&*literal!(", confidence number = ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                        }
                        Error::addCompilerError((r#str.clone()).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false, (_, Deref @ metamodelica::List::Cons { head: (start, _), tail: Deref @ metamodelica::List::Nil }), _) => {
                    Ok(BackendVariable::setVarStartValueOption(inVar.clone(), start.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false, (_, Deref @ metamodelica::List::Cons { head: (start, cr), tail: values }), Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut start1: Option<Arc<DAE::Exp>> = None;
                    let mut start = (*start).clone();
                    start1 = optExpReplaceCrefWithBindExp(start.clone(), globalKnownVars.clone())?;
                    (_, start, _) = equalFreeStartValues(values.clone(), globalKnownVars.clone(), (start1.clone(), start.clone(), cr.clone()))?;
                    Ok(BackendVariable::setVarStartValueOption(inVar.clone(), start.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false, (_, values), Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut zerofreevalues: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    zerofreevalues = List::fold(values.clone(), (std::sync::Arc::new(fnptr!(getZeroFreeValues, (Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>), Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>)) as std::sync::Arc<dyn ::std::ops::Fn((Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>), Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>> + 'static>), metamodelica::nil());
                    (v, warnAliasConflicts) = selectFreeValue(zerofreevalues.clone(), inVar.clone(), globalKnownVars.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, warnAliasConflicts))
}

fn addNominalValue(mut inVar: BackendDAE::Var, mut iNominal: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>) -> Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>> {
    let mut oNominal: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
    let mut nominal: Arc<DAE::Exp>;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    match '__try0: {
        nominal = unwrap_break_err!(BackendVariable::varNominalValue(inVar.clone()), '__try0);
        cr = unwrap_break_err!(BackendVariable::varCref(inVar.clone()), '__try0);
        oNominal = cons((nominal.clone(), cr.clone()), iNominal.clone());
        Ok::<_, anyhow::Error>((oNominal.clone(),))
    } {
        Ok((__try0_o0,)) => {
            oNominal = __try0_o0;
        }
        Err(_) => {
            oNominal = iNominal.clone();
        }
    }
    oNominal
}

fn mergeNominalAttribute(mut nominalList: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>, mut inVar: BackendDAE::Var, mut globalKnownVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, bool)> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut warnAliasConflicts: bool = false;
    outVar = 'mc: {
        let __mc_input = (nominalList.clone(), inVar.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inVar.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut allExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    allExp = List::map(nominalList.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    let __pa0 = ::match_deref::match_deref! { match &(List::uniqueOnTrue(allExp.clone(), (std::sync::Arc::new(ExpressionBasics::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>))) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    Ok(BackendVariable::setVarNominalValue(inVar.clone(), e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    warnAliasConflicts = !(Flags::isSet(Flags::ALIAS_CONFLICTS.clone())?);
                    Ok(selectFreeValue1(nominalList.clone(), metamodelica::nil(), (literal!("Alias set with conflicting nominal values\n")).clone(), (literal!("nominal")).clone(), (std::sync::Arc::new(BackendVariable::setVarNominalValue) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<DAE::Exp>) -> Result<BackendDAE::Var> + 'static>), inVar.clone(), globalKnownVars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, warnAliasConflicts))
}

fn addMinMaxAttribute(mut inVar: BackendDAE::Var, mut negate: bool, mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>, mut iMinMax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>)) -> Result<(Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>)> {
    let mut oMinMax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>);
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let mut ominmax: Arc<metamodelica::List<Option<Arc<DAE::Exp>>>> = metamodelica::nil();
    let BackendDAE::VAR { values: __pa0, .. } = (inVar.clone()) else { bail!("pattern mismatch") };
    attr = __pa0.clone();
    ominmax = DAEUtil::getMinMax(attr.clone());
    oMinMax = mergeMinMax(negate.clone(), ominmax.clone(), iMinMax.clone(), mark.clone(), simpleeqnsarr.clone())?;
    Ok(oMinMax)
}

fn mergeMinMax(mut negate: bool, mut ominmax: Arc<metamodelica::List<Option<Arc<DAE::Exp>>>>, mut ominmax1: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>), mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>) -> Result<(Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>)> {
    let mut outMinMax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>);
    outMinMax = (::match_deref::match_deref! { match &((negate.clone(), ominmax.clone(), ominmax1.clone(), mark.clone(), simpleeqnsarr.clone())) {
        (_, Deref @ metamodelica::List::Nil, _, _, _) => {
            ominmax1.clone()
        },
        (false, Deref @ metamodelica::List::Cons { head: omin, tail: Deref @ metamodelica::List::Cons { head: omax, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
            let mut minMax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>);
            minMax = mergeMinMax1((omin.clone(), omax.clone()), ominmax1.clone())?;
            checkMinMax(minMax.clone(), mark.clone(), simpleeqnsarr.clone())?;
            minMax.clone()
        },
        (true, Deref @ metamodelica::List::Cons { head: None, tail: Deref @ metamodelica::List::Cons { head: None, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
            ominmax1.clone()
        },
        (true, Deref @ metamodelica::List::Cons { head: Some(min), tail: Deref @ metamodelica::List::Cons { head: Some(max), tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
            let mut min1: Arc<DAE::Exp>;
            let mut max1: Arc<DAE::Exp>;
            let mut minMax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>);
            min1 = negateExpression(true, min.clone(), min.clone(), (literal!(" min_1 ")).clone())?;
            max1 = negateExpression(true, max.clone(), max.clone(), (literal!(" max_1 ")).clone())?;
            minMax = mergeMinMax1((Some(max1.clone()), Some(min1.clone())), ominmax1.clone())?;
            checkMinMax(minMax.clone(), mark.clone(), simpleeqnsarr.clone())?;
            minMax.clone()
        },
        (true, Deref @ metamodelica::List::Cons { head: None, tail: Deref @ metamodelica::List::Cons { head: Some(max), tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
            let mut max1: Arc<DAE::Exp>;
            let mut minMax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>);
            max1 = negateExpression(true, max.clone(), max.clone(), (literal!(" max_2 ")).clone())?;
            minMax = mergeMinMax1((Some(max1.clone()), None), ominmax1.clone())?;
            checkMinMax(minMax.clone(), mark.clone(), simpleeqnsarr.clone())?;
            minMax.clone()
        },
        (true, Deref @ metamodelica::List::Cons { head: Some(min), tail: Deref @ metamodelica::List::Cons { head: None, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
            let mut min1: Arc<DAE::Exp>;
            let mut minMax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>);
            min1 = negateExpression(true, min.clone(), min.clone(), (literal!(" min_2 ")).clone())?;
            minMax = mergeMinMax1((None, Some(min1.clone())), ominmax1.clone())?;
            checkMinMax(minMax.clone(), mark.clone(), simpleeqnsarr.clone())?;
            minMax.clone()
        },
        _ => {
            println!("{}", (literal!("RemoveSimpleEquations.mergeMinMax failed!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMinMax)
}

fn mergeMinMax1(mut ominmax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>), mut ominmax1: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>)) -> Result<(Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>)> {
    let mut minMax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>);
    let mut omin: Option<Arc<DAE::Exp>> = None;
    let mut omin1: Option<Arc<DAE::Exp>> = None;
    let mut omin2: Option<Arc<DAE::Exp>> = None;
    let mut omax: Option<Arc<DAE::Exp>> = None;
    let mut omax1: Option<Arc<DAE::Exp>> = None;
    let mut omax2: Option<Arc<DAE::Exp>> = None;
    (omin, omax) = ominmax.clone();
    (omin1, omax1) = ominmax1.clone();
    omin2 = Expression::expOptMaxScalar(omin.clone(), omin1.clone())?;
    omax2 = Expression::expOptMinScalar(omax.clone(), omax1.clone())?;
    if referenceEq(&omin2.clone(),&omin.clone()) && referenceEq(&omax2.clone(),&omax.clone()) {
        minMax = ominmax.clone();
    } else if referenceEq(&omin2.clone(),&omin1.clone()) && referenceEq(&omax2.clone(),&omax1.clone()) {
        minMax = ominmax1.clone();
    } else {
        minMax = (omin2.clone(), omax2.clone());
    }
    Ok(minMax)
}

fn checkMinMax(mut minmax: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>), mut mark: i32, mut simpleeqnsarr: metamodelica::Array<SimpleContainer>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (minmax.clone(), mark.clone(), simpleeqnsarr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Some(min), Some(max)), _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut rmin: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rmax: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rmin = Expression::toReal(min.clone())?;
                    rmax = Expression::toReal(max.clone())?;
                    let true = (realGt(rmin.clone(), rmax.clone())) else { bail!("pattern mismatch") };
                    s4 = (ExpressionBasics::printExpStr(min.clone())?).clone();
                    s5 = (ExpressionBasics::printExpStr(max.clone())?).clone();
                    s = stringAppendList(list![(literal!("Alias variables with invalid limits min ")).clone(), (s4.clone()).clone(), (literal!(" > max ")).clone(), (s5.clone()).clone()]);
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

fn handleVarSetAttributes(mut inAttributes: VarSetAttributes, mut inVar: BackendDAE::Var, mut inVars: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>) -> Result<(BackendDAE::Variables, bool)> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut warnAliasConflicts: bool = false;
    outVars = ({
        let mut b1: bool = false;
        let mut b2: bool = false;
        let mut v: BackendDAE::Var = inVar.clone();
        'mc: {
        let __mc_input = (inAttributes.clone(), inVar.clone(), inVars.clone(), inShared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((fixedset, startvalues, nominalset, minmaxset), _, _, Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut isdiscrete: bool = false;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut min: Option<Arc<DAE::Exp>> = None;
                    let mut max: Option<Arc<DAE::Exp>> = None;
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    isdiscrete = BackendVariable::isVarDiscrete(inVar.clone());
                    if !(isdiscrete.clone()) {
                        (v, b1) = mergeStartFixedAttributes(inVar.clone(), fixedset.clone(), startvalues.clone(), inShared.clone())?;
                    }
                    (v, b2) = mergeNominalAttribute(nominalset.clone(), v.clone(), globalKnownVars.clone())?;
                    (min, max) = minmaxset.clone();
                    if isSome(min.clone()) {
                        min = Some(ExpressionSimplify::simplify(Util::getOption(min.clone())?)?);
                    }
                    if isSome(max.clone()) {
                        max = Some(ExpressionSimplify::simplify(Util::getOption(max.clone())?)?);
                    }
                    v = BackendVariable::setVarMinMax(v.clone(), min.clone(), max.clone())?;
                    vars = BackendVariable::addVar(v.clone(), inVars.clone())?;
                    warnAliasConflicts = b1.clone() || b2.clone();
                    Ok(vars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("RemoveSimpleEquations.handleVarSetAttributes failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok((outVars, warnAliasConflicts))
}

fn optExpReplaceCrefWithBindExp(mut iOExp: Option<Arc<DAE::Exp>>, mut globalKnownVars: BackendDAE::Variables) -> Result<Option<Arc<DAE::Exp>>> {
    let mut oOExp: Option<Arc<DAE::Exp>> = None;
    oOExp = (::match_deref::match_deref! { match &((iOExp.clone(), globalKnownVars.clone())) {
        (Some(e), _) => {
            let mut b: bool = false;
            let mut e = (*e).clone();
            (e, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars.clone())?;
            (e, _) = ExpressionSimplify::condsimplify(b.clone(), e.clone())?;
            Some(e.clone())
        },
        _ => {
            iOExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oOExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn equalNonFreeStartValues(mut iValues: Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>, mut globalKnownVars: BackendDAE::Variables, mut iValue: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)) -> Result<(Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)> {
    let mut oValue: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>);
    oValue = (::match_deref::match_deref! { match &((iValues.clone(), globalKnownVars.clone(), iValue.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            iValue.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (None, _), tail: values }, _, _) => {
            equalNonFreeStartValues(values.clone(), globalKnownVars.clone(), iValue.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: (None, _), tail: values }, _, (None, _, _)) => {
            equalNonFreeStartValues(values.clone(), globalKnownVars.clone(), iValue.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: (None, cr), tail: values }, _, (Some(e2), _, _)) if (Expression::isZero(e2.clone())) => {
            equalNonFreeStartValues(values.clone(), globalKnownVars.clone(), (None, None, cr.clone()))?
        },
        (Deref @ metamodelica::List::Cons { head: (Some(e), _), tail: values }, _, (Some(e2), _, _)) => {
            let mut e1: Arc<DAE::Exp>;
            let mut b: bool = false;
            (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars.clone())?;
            (e1, _) = ExpressionSimplify::condsimplify(b.clone(), e1.clone())?;
            let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
            equalNonFreeStartValues(values.clone(), globalKnownVars.clone(), iValue.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oValue)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn equalFreeStartValues(mut iValues: Arc<metamodelica::List<(Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)>>, mut globalKnownVars: BackendDAE::Variables, mut iValue: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)) -> Result<(Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>)> {
    let mut oValue: (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>);
    oValue = (::match_deref::match_deref! { match &((iValues.clone(), globalKnownVars.clone(), iValue.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            iValue.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (None, _), tail: values }, _, _) => {
            equalFreeStartValues(values.clone(), globalKnownVars.clone(), iValue.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: (Some(e), cr), tail: values }, _, (None, _, _)) => {
            let mut e1: Arc<DAE::Exp>;
            let mut b: bool = false;
            (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars.clone())?;
            (e1, _) = ExpressionSimplify::condsimplify(b.clone(), e1.clone())?;
            equalFreeStartValues(values.clone(), globalKnownVars.clone(), (Some(e1.clone()), Some(e.clone()), cr.clone()))?
        },
        (Deref @ metamodelica::List::Cons { head: (Some(e), _), tail: values }, _, (Some(e2), _, _)) => {
            let mut e1: Arc<DAE::Exp>;
            let mut b: bool = false;
            (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars.clone())?;
            (e1, _) = ExpressionSimplify::condsimplify(b.clone(), e1.clone())?;
            let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
            equalFreeStartValues(values.clone(), globalKnownVars.clone(), iValue.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oValue)
}

fn replaceCrefWithBindExp(mut exp: Arc<DAE::Exp>, mut vars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut replaced: bool = false;
    let mut replaced_crefs: Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(ComponentReference::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), 13);
    (outExp, replaced) = Expression::traverseExpBottomUp(exp.clone(), Arc::new({ let __pe_b2 = vars.clone(); let __pe_b3 = replaced_crefs.clone(); move |__pe_a0, __pe_a1| replaceCrefWithBindExp_traverser(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), false)?;
    Ok((outExp, replaced))
}

fn replaceCrefWithBindExp_traverser(mut exp: Arc<DAE::Exp>, mut replaced: bool, mut vars: BackendDAE::Variables, mut replacedCrefs: Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outReplaced: bool = false;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    (outExp, outReplaced) = 'mc: {
        let __mc_input = (exp.clone(), replaced.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
                    if !((!(UnorderedSet::contains(cr.clone(), replacedCrefs.clone())?))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let (BackendDAE::VAR { bindExp: Some(__pa0), .. }, _) = (BackendVariable::getVarSingle(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                    e = __pa0.clone();
                    UnorderedSet::add(cr.clone(), replacedCrefs.clone())?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), Arc::new({ let __pe_b2 = vars.clone(); let __pe_b3 = replacedCrefs.clone(); move |__pe_a0, __pe_a1| replaceCrefWithBindExp_traverser(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), false)?;
                    Ok((e.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok((exp.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), replaced.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outReplaced))
}

fn getZeroFreeValues(mut inTpl: (Option<Arc<DAE::Exp>>, Arc<DAE::ComponentRef>), mut iAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>) -> Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>> {
    let mut oAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
    oAcc = (::match_deref::match_deref! { match &((inTpl.clone(), iAcc.clone())) {
        ((Some(e), cr), _) => {
            cons((e.clone(), cr.clone()), iAcc.clone())
        },
        _ => {
            iAcc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oAcc
}

fn selectFreeValue(mut iZeroFreeValues: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>, mut inVar: BackendDAE::Var, mut globalKnownVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, bool)> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut warnAliasConflicts: bool = false;
    outVar = (::match_deref::match_deref! { match &((iZeroFreeValues.clone(), inVar.clone())) {
        (Deref @ metamodelica::List::Nil, _) => inVar.clone(),
        (_, _) => {
            warnAliasConflicts = !(Flags::isSet(Flags::ALIAS_CONFLICTS.clone())?);
            selectFreeValue1(iZeroFreeValues.clone(), metamodelica::nil(), (literal!("Alias set with conflicting start values\n")).clone(), (literal!("start")).clone(), (std::sync::Arc::new(BackendVariable::setVarStartValue) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<DAE::Exp>) -> Result<BackendDAE::Var> + 'static>), inVar.clone(), globalKnownVars.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, warnAliasConflicts))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn selectNonZeroExpression(mut iFavorit: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, i32)>>) -> Result<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, i32)> {
    let mut selected: (Arc<DAE::Exp>, Arc<DAE::ComponentRef>, i32);
    selected = (::match_deref::match_deref! { match &(iFavorit.clone()) {
        Deref @ metamodelica::List::Cons { head: tpl, tail: Deref @ metamodelica::List::Nil } => {
            tpl.clone()
        },
        Deref @ metamodelica::List::Cons { head: tpl @ (e, _, _), tail: _ } if (!(Expression::isZero(e.clone()))) => {
            tpl.clone()
        },
        Deref @ metamodelica::List::Cons { head: (_, _, _), tail: rest } => {
            selectNonZeroExpression(rest.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(selected)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn selectFreeValue1(mut iZeroFreeValues: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)>>, mut iFavorit: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, i32)>>, mut iStr: ArcStr, mut iAttributeName: ArcStr, mut inFunc: Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<DAE::Exp>) -> Result<BackendDAE::Var> + 'static>, mut inVar: BackendDAE::Var, mut globalKnownVars: BackendDAE::Variables) -> Result<BackendDAE::Var> {
    pub type FuncSetAttribute = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<DAE::Exp>) -> Result<BackendDAE::Var> + 'static>;

    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    outVar = ({
        let mut s: ArcStr = literal!("");
        'mc: {
        let __mc_input = (iZeroFreeValues.clone(), iFavorit.clone(), iStr.clone(), iAttributeName.clone(), inFunc.clone(), inVar.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(inVar.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, rest, _, _, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crVar: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    (e, cr, _) = selectNonZeroExpression(rest.clone())?;
                    crVar = BackendVariable::varCref(inVar.clone())?;
                    if Flags::isSet(Flags::ALIAS_CONFLICTS.clone())? {
                        (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars.clone())?;
                        (e1, _) = ExpressionSimplify::condsimplify(b.clone(), e1.clone())?;
                        s2 = (if (b.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iStr.clone()); __mm_s.push_str(&*literal!("=> Select value from ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*iAttributeName.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(") for variable: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(crVar.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                        Error::addMessage(Error::COMPILER_WARNING.clone(), list![(s.clone()).clone()])?;
                    }
                    v = inFunc(inVar.clone(), e.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (e, cr), tail: zerofreevalues }, Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut i: i32 = 0;
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut hardcoded: bool = false;
                    let (_, (__pa0, __pa1)) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(selectMinDepth) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (i32, bool))> + 'static>), (ComponentReference::crefDepth(cr.clone())?, true))?;
                    i = __pa0.clone();
                    hardcoded = __pa1.clone();
                    if hardcoded.clone() {
                        i = i.clone() + 5;
                    }
                    if Flags::isSet(Flags::ALIAS_CONFLICTS.clone())? {
                        (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars.clone())?;
                        (e1, _) = ExpressionSimplify::condsimplify(b.clone(), e1.clone())?;
                        s2 = (if (b.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iStr.clone()); __mm_s.push_str(&*literal!(" * Candidate: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*iAttributeName.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(", confidence number = ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    }
                    Ok(selectFreeValue1(zerofreevalues.clone(), list![(e.clone(), cr.clone(), i.clone())], (s.clone()).clone(), (iAttributeName.clone()).clone(), inFunc.clone(), inVar.clone(), globalKnownVars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (e, cr), tail: zerofreevalues }, Deref @ metamodelica::List::Cons { head: (es, crs, is), tail: rest }, _, _, _, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut crVar: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut i: i32 = 0;
                    let mut favorit: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut hardcoded: bool = false;
                    let (_, (__pa0, __pa1)) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(selectMinDepth) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (i32, bool))> + 'static>), (ComponentReference::crefDepth(cr.clone())?, true))?;
                    i = __pa0.clone();
                    hardcoded = __pa1.clone();
                    if hardcoded.clone() {
                        i = i.clone() + 5;
                    }
                    if Flags::isSet(Flags::ALIAS_CONFLICTS.clone())? {
                        (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars.clone())?;
                        (e1, _) = ExpressionSimplify::condsimplify(b.clone(), e1.clone())?;
                        s2 = (if (b.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iStr.clone()); __mm_s.push_str(&*literal!(" * Candidate: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*iAttributeName.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(", confidence number = ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    }
                    let true = (intEq(i.clone(), is.clone())) else { bail!("pattern mismatch") };
                    crVar = BackendVariable::varCref(inVar.clone())?;
                    favorit = if (ComponentReferenceBasics::crefEqual(crVar.clone(), crs.clone())?) {cons((es.clone(), crs.clone(), is.clone()), cons((e.clone(), cr.clone(), i.clone()), rest.clone()))} else {cons((e.clone(), cr.clone(), i.clone()), cons((es.clone(), crs.clone(), is.clone()), rest.clone()))};
                    Ok(selectFreeValue1(zerofreevalues.clone(), favorit.clone(), (s.clone()).clone(), (iAttributeName.clone()).clone(), inFunc.clone(), inVar.clone(), globalKnownVars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (e, cr), tail: zerofreevalues }, Deref @ metamodelica::List::Cons { head: (_, _, is), tail: _ }, _, _, _, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut i: i32 = 0;
                    let mut favorit: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut hardcoded: bool = false;
                    let (_, (__pa0, __pa1)) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(selectMinDepth) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (i32, bool))> + 'static>), (ComponentReference::crefDepth(cr.clone())?, true))?;
                    i = __pa0.clone();
                    hardcoded = __pa1.clone();
                    if hardcoded.clone() {
                        i = i.clone() + 5;
                    }
                    if Flags::isSet(Flags::ALIAS_CONFLICTS.clone())? {
                        (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars.clone())?;
                        (e1, _) = ExpressionSimplify::condsimplify(b.clone(), e1.clone())?;
                        s2 = (if (b.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iStr.clone()); __mm_s.push_str(&*literal!(" * Candidate: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*iAttributeName.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(", confidence number = ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    }
                    favorit = if (intLt(i.clone(), is.clone())) {list![(e.clone(), cr.clone(), i.clone())]} else {iFavorit.clone()};
                    Ok(selectFreeValue1(zerofreevalues.clone(), favorit.clone(), (s.clone()).clone(), (iAttributeName.clone()).clone(), inFunc.clone(), inVar.clone(), globalKnownVars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok(outVar)
}

fn selectMinDepth(mut e: Arc<DAE::Exp>, mut inMin: (i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (i32, bool))> {
    let mut eOut: Arc<DAE::Exp> = e.clone();
    let mut cont: bool = true;
    let mut outMin: (i32, bool);
    outMin = (::match_deref::match_deref! { match &((e.clone(), inMin.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (d, _)) => {
            let mut i: i32 = 0;
            i = ComponentReference::crefDepth(cr.clone())?;
            (intMin(i.clone(), d.clone()), false)
        },
        _ => {
            inMin.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eOut, cont, outMin))
}

// =============================================================================
// functions to update equation system and shared
//
// =============================================================================
fn updateSystem(mut foundSimple: bool, mut iEqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iVars: BackendDAE::Variables, mut repl: BackendVarTransform::VariableReplacements, mut isyst: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    osyst = (::match_deref::match_deref! { match &((foundSimple.clone(), isyst.clone())) {
        (false, _) => {
            isyst.clone()
        },
        (true, syst @ Deref @ BackendDAE::EqSystem { .. }) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut syst = (*syst).clone();
            (vars, _) = BackendVariable::traverseBackendDAEVars(iVars.clone(), (std::sync::Arc::new(updateVar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendVarTransform::VariableReplacements)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendVarTransform::VariableReplacements))> + 'static>), (BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), repl.clone()))?;
            eqns = BackendEquation::listEquation(iEqnslst.clone().reverse())?;
            assign_field!(
                syst.orderedEqs = eqns.clone(),
                syst.orderedVars = vars.clone()
            );
            BackendDAEUtil::clearEqSyst(syst.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(osyst)
}

fn updateVar(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, BackendVarTransform::VariableReplacements)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendVarTransform::VariableReplacements))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut oTpl: (BackendDAE::Variables, BackendVarTransform::VariableReplacements);
    (outVar, oTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(cr), .. }, .. }, (vars, repl)) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut v = (*v).clone();
                    let mut vars = (*vars).clone();
                    e = BackendVarTransform::getReplacement(repl.clone(), cr.clone())?;
                    v = updateStateOrder(e.clone(), v.clone())?;
                    vars = BackendVariable::addVar(v.clone(), vars.clone())?;
                    Ok((v.clone(), (vars.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (vars, repl)) => {
                    let mut vars = (*vars).clone();
                    vars = BackendVariable::addVar(v.clone(), vars.clone())?;
                    Ok((v.clone(), (vars.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, oTpl))
}

fn updateStateOrder(mut inExp: Arc<DAE::Exp>, mut inVar: BackendDAE::Var) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    outVar = (::match_deref::match_deref! { match &((inExp.clone(), inVar.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
            BackendVariable::setStateDerivative(inVar.clone(), Some(cr.clone()))?
        },
        _ => {
            BackendVariable::setStateDerivative(inVar.clone(), None)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVar)
}

fn removeSimpleEquationsShared(mut b: bool, mut inDAE: Arc<BackendDAE::BackendDAE>, mut repl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    outDAE = (::match_deref::match_deref! { match &((b.clone(), inDAE.clone())) {
        (false, _) => {
            inDAE.clone()
        },
        (true, Deref @ DAE { UNIQUEIO: systs, derivativeNamePrefix: shared @ BackendDAE::Shared { classAttrs: clsAttrsLst, constraints: constraintsLst, aliasVars, externalObjects, globalKnownVars, .. }, .. }) => {
            let mut systs1: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut b1: bool = false;
            let mut shared = (*shared).clone();
            let mut clsAttrsLst = (*clsAttrsLst).clone();
            let mut constraintsLst = (*constraintsLst).clone();
            let mut aliasVars = (*aliasVars).clone();
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                BackendVarTransform::dumpReplacements(repl.clone())?;
                BackendVarTransform::dumpExtendReplacements(repl.clone())?;
                BackendVarTransform::dumpDerConstReplacements(repl.clone())?;
            }
            let (_, (_, __pa0)) = BackendVariable::traverseBackendDAEVarsWithUpdate(aliasVars.clone(), (std::sync::Arc::new(replaceAliasVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<BackendDAE::Var>>))> + 'static>), (repl.clone(), metamodelica::nil()))?;
            varlst = __pa0.clone();
            aliasVars = List::fold(varlst.clone(), (std::sync::Arc::new(fixAliasConstBindings) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<BackendDAE::Variables> + 'static>), aliasVars.clone());
            todo!("unhandled field-assign shape: shared.aliasVars");
            (_, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), (std::sync::Arc::new(replaceVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
            (_, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(externalObjects.clone(), (std::sync::Arc::new(replaceVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
            (_, eqnslst, b1) = BackendEquation::traverseEquationArray(shared.initialEqs.clone(), (std::sync::Arc::new(replaceEquationTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool))> + 'static>), (repl.clone(), metamodelica::nil(), false))?;
            todo!("unhandled field-assign shape: shared.initialEqs");
            (_, eqnslst, _) = BackendEquation::traverseEquationArray(shared.removedEqs.clone(), (std::sync::Arc::new(replaceEquationTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool))> + 'static>), (repl.clone(), metamodelica::nil(), false))?;
            eqnslst = List::select(eqnslst.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::assertWithCondTrue, Arc<BackendDAE::Equation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>));
            todo!("unhandled field-assign shape: shared.removedEqs");
            (constraintsLst, clsAttrsLst) = replaceOptimicaExps(constraintsLst.clone(), clsAttrsLst.clone(), repl.clone())?;
            todo!("unhandled field-assign shape: shared.constraints");
            todo!("unhandled field-assign shape: shared.classAttrs");
            systs1 = removeSimpleEquationsShared1(systs.clone(), metamodelica::nil(), repl.clone(), None, aliasVars.clone())?;
            BackendDAE::DAE(systs1.clone(), shared.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

fn fixAliasConstBindings(mut iAVar: BackendDAE::Var, mut iAVars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut oAVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    let mut avar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    cr = BackendVariable::varCref(iAVar.clone())?;
    e = BackendVariable::varBindExp(iAVar.clone())?;
    e = fixAliasConstBindings1(cr.clone(), e.clone(), iAVars.clone())?;
    avar = BackendVariable::setBindExp(iAVar.clone(), Some(e.clone()));
    oAVars = BackendVariable::addVar(avar.clone(), iAVars.clone())?;
    Ok(oAVars)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fixAliasConstBindings1(mut iCr: Arc<DAE::ComponentRef>, mut iExp: Arc<DAE::Exp>, mut iAVars: BackendDAE::Variables) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = 'mc: {
        let __mc_input = (iCr.clone(), iExp.clone(), iAVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extractCrefsFromExp(iExp.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    let (BackendDAE::VAR { bindExp: Some(__pa1), .. }, _) = (BackendVariable::getVarSingle(cr.clone(), iAVars.clone())?) else { bail!("pattern mismatch") };
                    e = __pa1.clone();
                    Ok(fixAliasConstBindings1(cr.clone(), e.clone(), iAVars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oExp)
}

fn replaceAliasVarTraverser(mut inVar: BackendDAE::Var, mut inTpl: (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<BackendDAE::Var>>))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<BackendDAE::Var>>);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { bindExp: Some(e), .. }, (repl, varlst)) => {
                    let mut v1: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut e1: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut varlst = (*varlst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    b = Expression::isConstValue(e1.clone())?;
                    v1 = if (!(b.clone())) {BackendVariable::setBindExp(v.clone(), Some(e1.clone()))} else {v.clone()};
                    varlst = List::consOnTrue(b.clone(), v1.clone(), varlst.clone());
                    Ok((v1.clone(), (repl.clone(), varlst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn replaceVarTraverser(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    (outVar, repl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { bindExp: Some(e), .. }, repl) => {
                    let mut v1: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut e1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    v1 = BackendVariable::setBindExp(v.clone(), Some(e1.clone()));
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

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeSimpleEquationsShared1(mut inSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inSysts1: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut repl: BackendVarTransform::VariableReplacements, mut statesetrepl: Option<BackendVarTransform::VariableReplacements>, mut aliasVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    outSysts = (::match_deref::match_deref! { match &((inSysts.clone(), inSysts1.clone(), repl.clone(), statesetrepl.clone(), aliasVars.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            inSysts1.clone()
        },
        (Deref @ metamodelica::List::Cons { head: syst, tail: rest }, _, _, _, _) => {
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
            let mut statesetrepl1: Option<BackendVarTransform::VariableReplacements> = None;
            let mut syst = (*syst).clone();
            (_, eqnslst, b) = BackendEquation::traverseEquationArray(syst.orderedEqs.clone(), (std::sync::Arc::new(replaceEquationTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool))> + 'static>), (repl.clone(), metamodelica::nil(), false))?;
            (stateSets, b1, statesetrepl1) = removeAliasVarsStateSets(syst.stateSets.clone(), statesetrepl.clone(), syst.orderedVars.clone(), aliasVars.clone(), metamodelica::nil(), false)?;
            if b.clone() || b1.clone() {
                eqns = BackendEquation::listEquation(eqnslst.clone().reverse())?;
                assign_field!(
                    syst.stateSets = stateSets.clone(),
                    syst.orderedEqs = eqns.clone()
                );
                syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
            }
            (_, eqnslst, _) = BackendEquation::traverseEquationArray(syst.removedEqs.clone(), (std::sync::Arc::new(replaceEquationTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool))> + 'static>), (repl.clone(), metamodelica::nil(), false))?;
            eqnslst = List::select(eqnslst.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::assertWithCondTrue, Arc<BackendDAE::Equation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>));
            assign_field!(syst.removedEqs = BackendEquation::listEquation(eqnslst.clone())?);
            removeSimpleEquationsShared1(rest.clone(), cons(syst.clone(), inSysts1.clone()), repl.clone(), statesetrepl1.clone(), aliasVars.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSysts)
}

fn removeAliasVarsStateSets(mut iStateSets: Arc<metamodelica::List<BackendDAE::StateSet>>, mut iStatesetrepl: Option<BackendVarTransform::VariableReplacements>, mut vars: BackendDAE::Variables, mut aliasVars: BackendDAE::Variables, mut iAcc: Arc<metamodelica::List<BackendDAE::StateSet>>, mut inB: bool) -> Result<(Arc<metamodelica::List<BackendDAE::StateSet>>, bool, Option<BackendVarTransform::VariableReplacements>)> {
    let mut oStateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
    let mut outB: bool = false;
    let mut oStatesetrepl: Option<BackendVarTransform::VariableReplacements> = None;
    (oStateSets, outB, oStatesetrepl) = (::match_deref::match_deref! { match &((iStateSets.clone(), iStatesetrepl.clone(), vars.clone(), aliasVars.clone(), iAcc.clone(), inB.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            (iAcc.clone().reverse(), inB.clone(), iStatesetrepl.clone())
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::StateSet { index, rang, state: states, crA, varA, statescandidates, ovars, eqns, oeqns, crJ, varJ, jacobian: jac }, tail: stateSets }, _, _, _, _, _) => {
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut ovars = (*ovars).clone();
            let mut eqns = (*eqns).clone();
            let mut oeqns = (*oeqns).clone();
            let mut stateSets = (*stateSets).clone();
            repl = getAliasReplacements(iStatesetrepl.clone(), aliasVars.clone())?;
            hs = HashSet::emptyHashSet();
            hs = List::applyAndFold(statescandidates.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>), hs.clone());
            ovars = replaceOtherStateSetVars(ovars.clone(), vars.clone(), aliasVars.clone(), hs.clone(), metamodelica::nil())?;
            (eqns, b) = BackendVarTransform::replaceEquations(eqns.clone(), repl.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
            (oeqns, b1) = BackendVarTransform::replaceEquations(oeqns.clone(), repl.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
            oeqns = List::fold(oeqns.clone(), (std::sync::Arc::new(removeEqualLshRshEqns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> + 'static>), metamodelica::nil());
            oeqns = oeqns.clone().reverse();
            (stateSets, b, oStatesetrepl) = removeAliasVarsStateSets(stateSets.clone(), Some(repl.clone()), vars.clone(), aliasVars.clone(), cons(BackendDAE::StateSet { index: index.clone(), rang: rang.clone(), state: states.clone(), crA: crA.clone(), varA: varA.clone(), statescandidates: statescandidates.clone(), ovars: ovars.clone(), eqns: eqns.clone(), oeqns: oeqns.clone(), crJ: crJ.clone(), varJ: varJ.clone(), jacobian: jac.clone() }, iAcc.clone()), b.clone() || b1.clone())?;
            (stateSets.clone(), b.clone(), oStatesetrepl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oStateSets, outB, oStatesetrepl))
}

fn removeEqualLshRshEqns(mut iEqn: Arc<BackendDAE::Equation>, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    oEqns = 'mc: {
        let __mc_input = (iEqn.clone(), iEqns.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { scalar: rhs, exp: lhs, .. }, _) => {
                    let mut b: bool = false;
                    b = ExpressionBasics::expEqual(lhs.clone(), rhs.clone())?;
                    Ok(List::consOnTrue(!(b.clone()), iEqn.clone(), iEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: rhs, left: lhs, .. }, _) => {
                    let mut b: bool = false;
                    b = ExpressionBasics::expEqual(lhs.clone(), rhs.clone())?;
                    Ok(List::consOnTrue(!(b.clone()), iEqn.clone(), iEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: rhs, left: lhs, .. }, _) => {
                    let mut b: bool = false;
                    b = ExpressionBasics::expEqual(lhs.clone(), rhs.clone())?;
                    Ok(List::consOnTrue(!(b.clone()), iEqn.clone(), iEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cons(iEqn.clone(), iEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEqns)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn replaceOtherStateSetVars(mut iVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut vars: BackendDAE::Variables, mut aliasVars: BackendDAE::Variables, mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iAcc: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut oVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    oVarLst = 'mc: {
        let __mc_input = (iVarLst.clone(), vars.clone(), aliasVars.clone(), hs.clone(), iAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var, tail: varlst }, _, _, _, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut exp: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut var = (*var).clone();
                    let mut varlst = (*varlst).clone();
                    cr = BackendVariable::varCref(var.clone())?;
                    let false = (BaseHashSet::has(cr.clone(), hs.clone())?) else { bail!("pattern mismatch") };
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), aliasVars.clone())?;
                    exp = BackendVariable::varBindExp(var.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extractCrefsFromExp(exp.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    b = BaseHashSet::has(cr.clone(), hs.clone())?;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    varlst = List::consOnTrue(!(b.clone()), var.clone(), iAcc.clone());
                    Ok(replaceOtherStateSetVars(varlst.clone(), vars.clone(), aliasVars.clone(), hs.clone(), varlst.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var, tail: varlst }, _, _, _, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cr = BackendVariable::varCref(var.clone())?;
                    let true = (BaseHashSet::has(cr.clone(), hs.clone())?) else { bail!("pattern mismatch") };
                    Ok(replaceOtherStateSetVars(varlst.clone(), vars.clone(), aliasVars.clone(), hs.clone(), iAcc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var, tail: varlst }, _, _, _, _) => {
                    Ok(replaceOtherStateSetVars(varlst.clone(), vars.clone(), aliasVars.clone(), hs.clone(), cons(var.clone(), iAcc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oVarLst)
}

fn replaceOptimicaExps(mut icontraints: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut iclassAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>, mut irepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<DAE::Constraint>>>, Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>)> {
    let mut ocontraints: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    let mut oclassAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>> = metamodelica::nil();
    (ocontraints, oclassAttributes) = (::match_deref::match_deref! { match &((icontraints.clone(), iclassAttributes.clone(), irepl.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ DAE::ClassAttributes { objetiveE, objectiveIntegrandE, startTimeE, finalTimeE }, tail: restClassAtr }, _) => {
            let mut classAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>> = metamodelica::nil();
            let mut objetiveE = (*objetiveE).clone();
            let mut objectiveIntegrandE = (*objectiveIntegrandE).clone();
            let mut startTimeE = (*startTimeE).clone();
            let mut finalTimeE = (*finalTimeE).clone();
            let __pa0 = ::match_deref::match_deref! { match &(replaceOptExprTraverser((objetiveE.clone(), (irepl.clone(), metamodelica::nil(), false)))?) {
                (_, (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _)) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            objetiveE = __pa0.clone();
            let __pa2 = ::match_deref::match_deref! { match &(replaceOptExprTraverser((objectiveIntegrandE.clone(), (irepl.clone(), metamodelica::nil(), false)))?) {
                (_, (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }, _)) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            objectiveIntegrandE = __pa2.clone();
            let __pa4 = ::match_deref::match_deref! { match &(replaceOptExprTraverser((startTimeE.clone(), (irepl.clone(), metamodelica::nil(), false)))?) {
                (_, (_, Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil }, _)) => __pa4.clone(),
                _ => bail!("pattern mismatch"),
            } };
            startTimeE = __pa4.clone();
            let __pa6 = ::match_deref::match_deref! { match &(replaceOptExprTraverser((finalTimeE.clone(), (irepl.clone(), metamodelica::nil(), false)))?) {
                (_, (_, Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil }, _)) => __pa6.clone(),
                _ => bail!("pattern mismatch"),
            } };
            finalTimeE = __pa6.clone();
            (_, classAttributes) = replaceOptimicaExps(metamodelica::nil(), restClassAtr.clone(), irepl.clone())?;
            classAttributes = cons(Arc::new(DAE::ClassAttributes { objetiveE: objetiveE.clone(), objectiveIntegrandE: objectiveIntegrandE.clone(), startTimeE: startTimeE.clone(), finalTimeE: finalTimeE.clone() }), classAttributes.clone());
            (metamodelica::nil(), classAttributes.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: constraintLstExps }, tail: rest }, _, _) => {
            let mut constraintLst: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
            let mut constraintLstExps = (*constraintLstExps).clone();
            constraintLstExps = replaceOptimicaContraints(constraintLstExps.clone(), irepl.clone())?;
            (constraintLst, _) = replaceOptimicaExps(rest.clone(), iclassAttributes.clone(), irepl.clone())?;
            constraintLst = cons(Arc::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: constraintLstExps.clone() }), constraintLst.clone());
            (constraintLst.clone(), iclassAttributes.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((ocontraints, oclassAttributes))
}

fn replaceOptimicaContraints(mut icontraints: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut irepl: BackendVarTransform::VariableReplacements) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut ocontraints: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    ocontraints = (::match_deref::match_deref! { match &((icontraints.clone(), irepl.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _) => {
            let mut constraintLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut e = (*e).clone();
            let __pa0 = ::match_deref::match_deref! { match &(replaceExprTraverser((e.clone(), (irepl.clone(), metamodelica::nil(), false)))?) {
                (_, (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _)) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            constraintLst = replaceOptimicaContraints(rest.clone(), irepl.clone())?;
            constraintLst = cons(e.clone(), constraintLst.clone());
            constraintLst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ocontraints)
}

fn getAliasReplacements(mut iStatesetrepl: Option<BackendVarTransform::VariableReplacements>, mut aliasVars: BackendDAE::Variables) -> Result<BackendVarTransform::VariableReplacements> {
    let mut oStatesetrepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    oStatesetrepl = (match (iStatesetrepl.clone(), aliasVars.clone()) {
        (Some(mut repl), _) => {
            repl.clone()
        },
        _ => {
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            repl = BackendVarTransform::emptyReplacementsSized(BackendVariable::varsSize(aliasVars.clone())?);
            repl = BackendVariable::traverseBackendDAEVars(aliasVars.clone(), (std::sync::Arc::new(getAliasVarReplacements) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
            repl.clone()
        },
    });
    Ok(oStatesetrepl)
}

fn getAliasVarReplacements(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> {
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut exp: Arc<DAE::Exp>;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    v = inVar.clone();
    let BackendDAE::VAR { bindExp: Some(__pa0), varName: __pa1, .. } = (v.clone()) else { bail!("pattern mismatch") };
    exp = __pa0.clone();
    cr = __pa1.clone();
    repl = BackendVarTransform::addReplacement(inRepl.clone(), cr.clone(), exp.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
    Ok((v, repl))
}

fn replaceEquationTraverser(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTpl: (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool);
    (outEq, outTpl) = (::match_deref::match_deref! { match &((inEq.clone(), inTpl.clone())) {
        (e, (repl, eqns, b)) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut res: Arc<DAE::Exp>;
            let mut eqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut b1: bool = false;
            let mut eqns = (*eqns).clone();
            (eqns1, b1) = BackendVarTransform::replaceEquations(list![e.clone()], repl.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
            if BackendEquation::isInitialEquation(e.clone())? && BackendEquation::isEquation(e.clone()) {
                eqn = listHead(eqns1.clone())?;
                lhs = BackendEquation::getEquationLHS(eqn.clone())?;
                rhs = BackendEquation::getEquationRHS(eqn.clone())?;
                res = Expression::createResidualExp(lhs.clone(), rhs.clone())?;
                if Expression::isConst(res.clone())? {
                    if Expression::isZero(res.clone()) {
                        Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following initial equation is redundant and consistent due to simplifications in RemoveSimpleEquations and therefore removed from the initialization problem: ")); __mm_s.push_str(&*BackendDump::equationString(e.clone())?); __mm_s.push_str(&*if (b1.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}); ArcStr::from(__mm_s) }).clone())?;
                        eqns1 = metamodelica::nil();
                        b1 = true;
                    } else {
                        Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following initial equation is inconsistent due to simplifications in RemoveSimpleEquations and therefore removed from the initialization problem: ")); __mm_s.push_str(&*BackendDump::equationString(e.clone())?); __mm_s.push_str(&*if (b1.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}); ArcStr::from(__mm_s) }).clone())?;
                        eqns1 = metamodelica::nil();
                        b1 = true;
                    }
                }
            }
            eqns = listAppend(eqns1.clone(), eqns.clone());
            (e.clone(), (repl.clone(), eqns.clone(), b.clone() || b1.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

fn replaceExprTraverser(mut inTpl: (Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool))) -> Result<(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool))> {
    let mut outTpl: (Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool));
    outTpl = (::match_deref::match_deref! { match &(inTpl.clone()) {
        (exp, (repl, exps, b)) => {
            let mut exp1: Arc<DAE::Exp>;
            let mut b1: bool = false;
            let mut exps = (*exps).clone();
            (exp1, b1) = BackendVarTransform::replaceExp(exp.clone(), repl.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
            exps = cons(exp1.clone(), exps.clone());
            (exp.clone(), (repl.clone(), exps.clone(), b.clone() || b1.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn replaceOptExprTraverser(mut inTpl: (Option<Arc<DAE::Exp>>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Option<Arc<DAE::Exp>>>>, bool))) -> Result<(Option<Arc<DAE::Exp>>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Option<Arc<DAE::Exp>>>>, bool))> {
    let mut outTpl: (Option<Arc<DAE::Exp>>, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Option<Arc<DAE::Exp>>>>, bool));
    outTpl = (::match_deref::match_deref! { match &(inTpl.clone()) {
        (None, (repl, exps, b)) => {
            let mut exps = (*exps).clone();
            exps = cons(None, exps.clone());
            (None, (repl.clone(), exps.clone(), b.clone()))
        },
        (expOpt @ Some(exp), (repl, exps, b)) => {
            let mut exp1: Arc<DAE::Exp>;
            let mut b1: bool = false;
            let mut exps = (*exps).clone();
            (exp1, b1) = BackendVarTransform::replaceExp(exp.clone(), repl.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
            if referenceEq(&exp1.clone(),&exp.clone()) {
                exps = cons(expOpt.clone(), exps.clone());
            } else {
                exps = cons(Some(exp1.clone()), exps.clone());
            }
            (expOpt.clone(), (repl.clone(), exps.clone(), b.clone() || b1.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpl)
}

// =============================================================================
// functions to find unReplaceable variables
//
// unReplaceable:
//   - variables with variable subscribts
//   - variables set in when-clauses
//   - variables used in pre
//   - statescandidates of statesets
//   - lhs of array assign statement, because there is a cref used and this is not replaceable_ with array of crefs
// =============================================================================
fn addUnreplaceableFromStateSets(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    outUnreplaceable = List::fold(systs.clone(), (std::sync::Arc::new(addUnreplaceableFromStateSetSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), inUnreplaceable.clone());
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromStateSetSystem(mut isyst: Arc<BackendDAE::EqSystem>, mut inUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outUnreplaceable = (::match_deref::match_deref! { match &((isyst.clone(), inUnreplaceable.clone())) {
        (Deref @ BackendDAE::EqSystem { stateSets: Deref @ metamodelica::List::Nil, .. }, _) => {
            inUnreplaceable.clone()
        },
        (Deref @ BackendDAE::EqSystem { stateSets, .. }, _) => {
            let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            unReplaceable = List::fold(stateSets.clone(), (std::sync::Arc::new(addUnreplaceableFromStateSet) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::StateSet, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), inUnreplaceable.clone());
            unReplaceable.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromStateSet(mut iStateSet: BackendDAE::StateSet, mut inUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut statevars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let BackendDAE::STATESET { statescandidates: __pa0, .. } = (iStateSet.clone()) else { bail!("pattern mismatch") };
    statevars = __pa0.clone();
    crlst = List::map(statevars.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
    crlst = List::map(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefStripLastSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
    outUnreplaceable = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), inUnreplaceable.clone());
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromWhens(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { shared: BackendDAE::Shared { initialEqs: __pa0, .. }, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqns = __pa0.clone();
    systs = __pa1.clone();
    outUnreplaceable = List::fold(systs.clone(), (std::sync::Arc::new(addUnreplaceableFromWhensSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), inUnreplaceable.clone());
    (_, outUnreplaceable) = BackendDAEUtil::traverseBackendDAEExpsEqns(eqns.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(addUnreplaceableFromEqnsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), outUnreplaceable.clone()))?;
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromEqnsExp(mut e: Arc<DAE::Exp>, mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ohs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    (outExp, ohs) = (::match_deref::match_deref! { match &((e.clone(), hs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _) => {
            (e.clone(), hs.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            ohs = BaseHashSet::add(cr.clone(), hs.clone())?;
            (e.clone(), ohs.clone())
        },
        _ => {
            (e.clone(), hs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, ohs))
}

fn addUnreplaceableFromWhensSystem(mut isyst: Arc<BackendDAE::EqSystem>, mut inUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    eqns = BackendEquation::getEqnsFromEqSystem(isyst.clone());
    outUnreplaceable = BackendEquation::traverseEquationArray(eqns.clone(), (std::sync::Arc::new(addUnreplaceableFromWhenEqn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<BackendDAE::Equation>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), inUnreplaceable.clone())?;
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromWhenEqn(mut inEq: Arc<BackendDAE::Equation>, mut inHs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<BackendDAE::Equation>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> {
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    (eqn, hs) = (::match_deref::match_deref! { match &((inEq.clone(), inHs.clone())) {
        (eqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, .. }, hs) => {
            let mut hs = (*hs).clone();
            hs = addUnreplaceableFromWhen(weqn.clone(), hs.clone())?;
            (eqn.clone(), hs.clone())
        },
        (eqn @ Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, hs) => {
            let mut hs = (*hs).clone();
            hs = List::fold(stmts.clone(), (std::sync::Arc::new(addUnreplaceableFromWhenStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), hs.clone());
            (eqn.clone(), hs.clone())
        },
        _ => {
            (inEq.clone(), inHs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn, hs))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addUnreplaceableFromWhenStmt(mut inStmt: Arc<DAE::Statement>, mut inHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outHS = 'mc: {
        let __mc_input = (inStmt.clone(), inHS.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { elseWhen: None, statementLst: stmts, .. }, _) => {
                    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    hs = List::fold(stmts.clone(), (std::sync::Arc::new(addUnreplaceableFromStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), inHS.clone());
                    Ok(hs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { elseWhen: Some(stmt), statementLst: stmts, .. }, _) => {
                    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    hs = List::fold(stmts.clone(), (std::sync::Arc::new(addUnreplaceableFromStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), inHS.clone());
                    hs = addUnreplaceableFromWhenStmt(stmt.clone(), hs.clone())?;
                    Ok(hs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, _) => {
                    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr = (*cr).clone();
                    cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    hs = BaseHashSet::add(cr.clone(), inHS.clone())?;
                    Ok(hs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, _) => {
                    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr = (*cr).clone();
                    cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    hs = BaseHashSet::add(cr.clone(), inHS.clone())?;
                    Ok(hs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inHS.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHS)
}

fn addUnreplaceableFromStmt(mut inStmt: Arc<DAE::Statement>, mut inHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outHS = (::match_deref::match_deref! { match &((inStmt.clone(), inHS.clone())) {
        (Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, _) => {
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            hs = BaseHashSet::add(cr.clone(), inHS.clone())?;
            hs.clone()
        },
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst, .. }, _) => {
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crlst = List::flatten(List::map(expExpLst.clone(), (std::sync::Arc::new(Expression::extractCrefsFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>)));
            crlst = List::map(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefStripLastSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
            hs = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), inHS.clone());
            hs.clone()
        },
        (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, _) => {
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            hs = BaseHashSet::add(cr.clone(), inHS.clone())?;
            hs.clone()
        },
        _ => {
            inHS.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outHS)
}

fn addUnreplaceableFromWhen(mut inWEqn: Arc<BackendDAE::WhenEquation>, mut iHs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut oHs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    oHs = (::match_deref::match_deref! { match &((inWEqn.clone(), iHs.clone())) {
        (Deref @ BackendDAE::WhenEquation { elsewhenPart: oweqn, whenStmtLst, .. }, _) => {
            let mut weqn: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            hs = addUnreplaceableFromWhenOps(whenStmtLst.clone(), iHs.clone())?;
            if isSome(oweqn.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oweqn.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                weqn = __pa0.clone();
                hs = addUnreplaceableFromWhen(weqn.clone(), hs.clone())?;
            }
            hs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oHs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addUnreplaceableFromWhenOps(mut inWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut iHs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut oHs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    oHs = (::match_deref::match_deref! { match &(inWhenOps.clone()) {
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: left, .. }, .. }, tail: rest } => {
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut left = (*left).clone();
            left = ComponentReferenceBasics::crefStripLastSubs(left.clone())?;
            hs = BaseHashSet::add(left.clone(), iHs.clone())?;
            addUnreplaceableFromWhenOps(rest.clone(), hs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e, .. }, tail: rest } => {
            let mut left: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            crefLst = Expression::getAllCrefs(e.clone())?;
            hs = iHs.clone();
            for mut left in &*crefLst.clone() {
                let mut left = left.clone();
                left = ComponentReferenceBasics::crefStripLastSubs(left.clone())?;
                hs = BaseHashSet::add(left.clone(), hs.clone())?;
            }
            addUnreplaceableFromWhenOps(rest.clone(), hs.clone())?
        },
        _ => {
            iHs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oHs)
}

fn traverserExpUnreplaceable(mut e: Arc<DAE::Exp>, mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    (outExp, outHt) = 'mc: {
        let __mc_input = (e.clone(), unReplaceable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
                    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    outHt = traverseCrefUnreplaceable(cr.clone(), None, unReplaceable.clone())?;
                    Ok((e.clone(), outHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: explst, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    crlst = List::flatten(List::map(explst.clone(), (std::sync::Arc::new(Expression::extractCrefsFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>)));
                    crlst = List::map(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefStripLastSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    outHt = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), unReplaceable.clone());
                    Ok((e.clone(), outHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((e.clone(), unReplaceable.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outHt))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn traverseCrefUnreplaceable(mut inCref: Arc<DAE::ComponentRef>, mut preCref: Option<Arc<DAE::ComponentRef>>, mut iUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut oUnreplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    oUnreplaceable = (::match_deref::match_deref! { match &((inCref.clone(), preCref.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, subscriptLst: subs, identType: ty, ident: name }, Some(pcr)) => {
            let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut b: bool = false;
            let mut pcr = (*pcr).clone();
            (_, b) = Expression::traverseExpTopDownSubs(subs.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefPresent, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
            pcr = if (b.clone()) {ComponentReference::crefPrependIdent(pcr.clone(), (name.clone()).clone(), metamodelica::nil(), ty.clone())?} else {pcr.clone()};
            unReplaceable = if (b.clone()) {BaseHashSet::add(pcr.clone(), iUnreplaceable.clone())?} else {iUnreplaceable.clone()};
            pcr = ComponentReference::crefPrependIdent(pcr.clone(), (name.clone()).clone(), subs.clone(), ty.clone())?;
            traverseCrefUnreplaceable(cr.clone(), Some(pcr.clone()), unReplaceable.clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, subscriptLst: subs, identType: ty, ident: name }, None) => {
            let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut b: bool = false;
            (_, b) = Expression::traverseExpTopDownSubs(subs.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefPresent, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
            unReplaceable = if (b.clone()) {BaseHashSet::add(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), iUnreplaceable.clone())?} else {iUnreplaceable.clone()};
            traverseCrefUnreplaceable(cr.clone(), Some(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone() })), unReplaceable.clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, identType: ty, ident: name }, Some(pcr)) => {
            let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut b: bool = false;
            (_, b) = Expression::traverseExpTopDownSubs(subs.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefPresent, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
            unReplaceable = if (b.clone()) {BaseHashSet::add(ComponentReference::crefPrependIdent(pcr.clone(), (name.clone()).clone(), metamodelica::nil(), ty.clone())?, iUnreplaceable.clone())?} else {iUnreplaceable.clone()};
            unReplaceable.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty, ident: name, .. }, None) => {
            let mut unReplaceable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut b: bool = false;
            (_, b) = Expression::traverseExpTopDownCrefHelper(inCref.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefPresent, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
            unReplaceable = if (b.clone()) {BaseHashSet::add(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), iUnreplaceable.clone())?} else {iUnreplaceable.clone()};
            unReplaceable.clone()
        },
        (Deref @ DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { .. }, _) => {
            iUnreplaceable.clone()
        },
        (Deref @ DAE::ComponentRef::WILD, _) => {
            iUnreplaceable.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oUnreplaceable)
}

fn negateExpression(mut negationFlag: bool, mut inExp: Arc<DAE::Exp>, mut inAlternative: Arc<DAE::Exp>, mut message: ArcStr) -> Result<Arc<DAE::Exp>> {
    let mut outExpression: Arc<DAE::Exp>;
    outExpression = (::match_deref::match_deref! { match &((negationFlag.clone(), inExp.clone(), inAlternative.clone(), message.clone())) {
        (true, _, _, _) => {
            let mut negatedExp: Arc<DAE::Exp>;
            negatedExp = Expression::negate(inExp.clone())?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrExpStr((literal!("Negating: ")).clone(), inExp.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*message.clone()); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            negatedExp.clone()
        },
        (false, _, _, _) => {
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrExpStrExpStr((literal!("Not negating: ")).clone(), inExp.clone(), (literal!(" returning: ")).clone(), inAlternative.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*message.clone()); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            inAlternative.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpression)
}

fn performAliasEliminationBB(mut inDAE: Arc<BackendDAE::BackendDAE>, mut findAliases: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    outDAE = BackendDAEUtil::mapEqSystem(inDAE.clone(), Arc::new({ let __pe_b2 = findAliases.clone(); move |__pe_a0, __pe_a1| eliminateTrivialEquations(__pe_a0, __pe_a1, __pe_b2.clone()) }))?;
    outDAE = BackendDAEUtil::mapEqSystem(outDAE.clone(), (std::sync::Arc::new(getAliasAttributes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(outDAE)
}

fn eliminateTrivialEquations(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut findAliases: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    (outSystem, outShared) = 'mc: {
        let __mc_input = (inSystem.clone(), inShared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, .. }, shared @ Deref @ BackendDAE::Shared { eventInfo, initialEqs: inieqns, aliasVars, globalKnownVars, .. }) => {
                    let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut eqList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut initEqList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut remEqList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut simpleEqList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut HTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr));
                    let mut HTCrToCrEqLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr));
                    let mut tplExp: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut tplCrEqLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>> = metamodelica::nil();
                    let mut countAliasEquations: i32 = 0;
                    let mut countSimpleEquations: i32 = 0;
                    let mut size: i32 = 0;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut syst = (*syst).clone();
                    let mut orderedEqs = (*orderedEqs).clone();
                    let mut orderedVars = (*orderedVars).clone();
                    let mut shared = (*shared).clone();
                    let mut inieqns = (*inieqns).clone();
                    let mut aliasVars = (*aliasVars).clone();
                    let mut globalKnownVars = (*globalKnownVars).clone();
                    size = BackendVariable::varsSize(orderedVars.clone())?;
                    size = intMax(BaseHashTable::defaultBucketSize.clone(), (((intReal(size.clone())) * (metamodelica::OrderedFloat(0.7_f64))).0 as i32));
                    HTCrToExp = HashTableCrToExp::emptyHashTableSized(size.clone());
                    HTCrToCrEqLst = HashTableCrToCrEqLst::emptyHashTableSized(size.clone());
                    repl = BackendVarTransform::emptyReplacementsSized(size.clone());
                    (_, HTCrToExp, HTCrToCrEqLst, eqList, simpleEqList) = BackendEquation::traverseEquationArray(orderedEqs.clone(), Arc::new({ let __pe_b2 = findAliases.clone(); move |__pe_a0, __pe_a1| findSimpleEquations(__pe_a0, __pe_a1, __pe_b2.clone()) }), (orderedVars.clone(), HTCrToExp.clone(), HTCrToCrEqLst.clone(), metamodelica::nil(), metamodelica::nil()))?;
                    tplExp = BaseHashTable::hashTableList(HTCrToExp.clone());
                    tplCrEqLst = BaseHashTable::hashTableList(HTCrToCrEqLst.clone());
                    HTCrToExp = addRestCrefs(tplCrEqLst.clone(), HTCrToExp.clone(), HTCrToCrEqLst.clone())?;
                    tplExp = BaseHashTable::hashTableList(HTCrToExp.clone());
                    (aliasVars, orderedVars) = moveVars(tplExp.clone(), aliasVars.clone(), orderedVars.clone());
                    varList = BackendVariable::varList(orderedVars.clone())?;
                    varList = removeStateDerInfo(varList.clone())?;
                    orderedVars = BackendVariable::listVar1(varList.clone());
                    (eqList, _) = BackendEquation::traverseExpsOfEquationList(eqList.clone(), (std::sync::Arc::new(traverseExpTopDown) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)))> + 'static>), HTCrToExp.clone())?;
                    orderedEqs = BackendEquation::listEquation(eqList.clone())?;
                    initEqList = BackendEquation::equationList(inieqns.clone());
                    (initEqList, _) = BackendEquation::traverseExpsOfEquationList(initEqList.clone(), (std::sync::Arc::new(traverseExpTopDown) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)))> + 'static>), HTCrToExp.clone())?;
                    inieqns = BackendEquation::listEquation(initEqList.clone())?;
                    remEqList = BackendEquation::equationList(syst.removedEqs.clone());
                    (remEqList, _) = BackendEquation::traverseExpsOfEquationList(remEqList.clone(), (std::sync::Arc::new(traverseExpTopDown) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)))> + 'static>), HTCrToExp.clone())?;
                    assign_field!(syst.removedEqs = BackendEquation::listEquation(List::select(remEqList.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::assertWithCondTrue, Arc<BackendDAE::Equation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>)))?);
                    remEqList = BackendEquation::equationList(shared.removedEqs.clone());
                    (remEqList, _) = BackendEquation::traverseExpsOfEquationList(remEqList.clone(), (std::sync::Arc::new(traverseExpTopDown) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)))> + 'static>), HTCrToExp.clone())?;
                    assign_field!(shared.removedEqs = BackendEquation::listEquation(List::select(remEqList.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::assertWithCondTrue, Arc<BackendDAE::Equation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>)))?);
                    repl = addVarReplacements(tplExp.clone(), repl.clone())?;
                    let (__pa0, (_, __pa1)) = BackendVariable::traverseBackendDAEVarsWithUpdate(aliasVars.clone(), (std::sync::Arc::new(replaceAliasVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, Arc<metamodelica::List<BackendDAE::Var>>))> + 'static>), (repl.clone(), metamodelica::nil()))?;
                    aliasVars = __pa0.clone();
                    varlst = __pa1.clone();
                    aliasVars = List::fold(varlst.clone(), (std::sync::Arc::new(fixAliasConstBindings) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<BackendDAE::Variables> + 'static>), aliasVars.clone());
                    (globalKnownVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), (std::sync::Arc::new(replaceVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
                    if Flags::isSet(Flags::DUMP_REPL.clone())? {
                        tplExp = BaseHashTable::hashTableList(HTCrToExp.clone());
                        countAliasEquations = (tplExp.clone().len() as i32);
                        countSimpleEquations = (simpleEqList.clone().len() as i32);
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of Unknowns:    ")); __mm_s.push_str(&*intString(BackendVariable::varsSize(orderedVars.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of \"Complex\" Equations:   ")); __mm_s.push_str(&*intString(BackendEquation::equationLstSize(eqList.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of Alias Equations:   ")); __mm_s.push_str(&*intString(countAliasEquations.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of Simple Equations:   ")); __mm_s.push_str(&*intString(countSimpleEquations.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", (literal!("\nAliases:\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n")).clone());
                        BaseHashTable::dumpHashTable(HTCrToExp.clone());
                    }
                    assign_field!(
                        syst.orderedVars = orderedVars.clone(),
                        syst.orderedEqs = orderedEqs.clone()
                    );
                    assign_field!(
                        shared.eventInfo = eventInfo.clone(),
                        shared.globalKnownVars = globalKnownVars.clone(),
                        shared.aliasVars = aliasVars.clone(),
                        shared.initialEqs = inieqns.clone()
                    );
                    Ok((BackendDAEUtil::clearEqSyst(syst.clone())?, shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inSystem.clone(), inShared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSystem, outShared))
}

fn moveVars(mut cr_exp_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, mut inAliasVars: BackendDAE::Variables, mut inVars: BackendDAE::Variables) -> (BackendDAE::Variables, BackendDAE::Variables) {
    let mut outAliasVars: BackendDAE::Variables = inAliasVars.clone();
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut i: i32 = 0;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    let mut bs: bool = false;
    for mut cr_exp in &*cr_exp_lst.clone() {
        let mut cr_exp = cr_exp.clone();
        (cr, e) = cr_exp.clone();
        match '__try0: {
            (v, i) = unwrap_break_err!(BackendVariable::getVarSingle(cr.clone(), outVars.clone()), '__try0);
            v = BackendVariable::setBindExp(v.clone(), Some(e.clone()));
            ops = ElementSource::getSymbolicTransformations(DAE::emptyElementSource().clone());
            v = BackendVariable::mergeVariableOperations(v.clone(), cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr.clone(), exp: e.clone() }), ops.clone()));
            bs = BackendVariable::isStateVar(v.clone());
            v = if (bs.clone()) {unwrap_break_err!(BackendVariable::setVarKind(v.clone(), crate::BackendDAE::VarKind::DUMMY_STATE), '__try0)} else {v.clone()};
            (outVars, _) = unwrap_break_err!(BackendVariable::removeVar(i.clone(), outVars.clone()), '__try0);
            outAliasVars = unwrap_break_err!(BackendVariable::addVar(v.clone(), outAliasVars.clone()), '__try0);
            Ok::<_, anyhow::Error>((outAliasVars.clone(), outVars.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                outAliasVars = __try0_o0;
                outVars = __try0_o1;
            }
            Err(_) => {
                outAliasVars = outAliasVars.clone();
                outVars = outVars.clone();
            }
        }
    }
    (outAliasVars, outVars)
}

fn addVarReplacements(mut cr_exp_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut outRepl: BackendVarTransform::VariableReplacements = inRepl.clone();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    for mut cr_exp in &*cr_exp_lst.clone() {
        let mut cr_exp = cr_exp.clone();
        (cr, e) = cr_exp.clone();
        outRepl = BackendVarTransform::addReplacement(outRepl.clone(), cr.clone(), e.clone(), Some((std::sync::Arc::new(fnptr!(BackendVarTransform::skipPreChangeEdgeOperator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>)))?;
    }
    Ok(outRepl)
}

fn traverseExpTopDown(mut inExp: Arc<DAE::Exp>, mut inHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)) = inHTCrToExp.clone();
    (outExp, _) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(insertReplacementsInEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)))> + 'static>), inHTCrToExp.clone())?;
    (outExp, _) = ExpressionSimplify::simplify(outExp.clone())?;
    Ok((outExp, outHTCrToExp))
}

fn insertReplacementsInEquations(mut inE1: Arc<DAE::Exp>, mut inHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)))> {
    let mut outE1: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr));
    (outE1, cont, outHTCrToExp) = 'mc: {
        let __mc_input = inE1.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut value: Arc<DAE::Exp>;
                    if BaseHashTable::hasKey(cr.clone(), inHTCrToExp.clone()) {
                        value = BaseHashTable::get(cr.clone(), inHTCrToExp.clone())?;
                    } else {
                        value = inE1.clone();
                    }
                    Ok((value.clone(), true, inHTCrToExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inE1.clone(), true, inHTCrToExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outE1, cont, outHTCrToExp))
}

fn removeStateDerInfo(mut inVarList: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    vars = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (inVarList.clone()).into_iter().cloned() {
            let __x = if (BackendVariable::isStateVar(var.clone())) {BackendVariable::setStateDerivative(var.clone(), None)?} else {var.clone()};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(vars)
}

fn findSimpleEquations(mut inEq: Arc<BackendDAE::Equation>, mut inTuple: (BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>), mut findAliases: bool) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTuple: (BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    (outEq, outTuple) = 'mc: {
        let __mc_input = (inEq.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eq, (vars, HTCrToExp, HTCrToCrEqLst, eqList, simpleEqList)) => {
                    let mut eqSolved: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut count: i32 = 0;
                    let mut paramCount: i32 = 0;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut res: Arc<DAE::Exp>;
                    let mut value: Arc<DAE::Exp>;
                    let mut exp1: Arc<DAE::Exp>;
                    let mut exp2: Arc<DAE::Exp>;
                    let mut keepEquation: bool = false;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut eqAttr: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
                    let mut HTCrToExp = (*HTCrToExp).clone();
                    let mut HTCrToCrEqLst = (*HTCrToCrEqLst).clone();
                    let mut eqList = (*eqList).clone();
                    let mut simpleEqList = (*simpleEqList).clone();
                    res = BackendEquation::getEquationRHS(eq.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(res.clone(), (std::sync::Arc::new(findCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool))> + 'static>), (metamodelica::nil(), vars.clone(), 0, 0, true))?) {
                        (_, (__pa0, _, __pa1, __pa2, true)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr_lst = __pa0.clone();
                    count = __pa1.clone();
                    paramCount = __pa2.clone();
                    res = BackendEquation::getEquationLHS(eq.clone())?;
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(res.clone(), (std::sync::Arc::new(findCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool))> + 'static>), (cr_lst.clone(), vars.clone(), count.clone(), paramCount.clone(), true))?) {
                        (_, (__pa3, _, __pa4, _, true)) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr_lst = __pa3.clone();
                    count = __pa4.clone();
                    keepEquation = true;
                    if count.clone() == 1 {
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found Equation knw0: ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                        let __pa5 = ::match_deref::match_deref! { match &(cr_lst.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } => __pa5.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        cr = __pa5.clone();
                        let false = (BackendVariable::isState(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                        let false = (BackendVariable::isClockedState(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                        let false = (BackendVariable::isOutput(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                        let false = (BackendVariable::isDiscrete(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                        exp1 = Expression::crefExp(cr.clone())?;
                        let true = (Types::isSimpleType(Expression::r#typeof(exp1.clone())?)) else { bail!("pattern mismatch") };
                        let (__pa8, __pa7) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), exp1.clone(), None)?) {
                            __pa8 @ Deref @ BackendDAE::Equation::EQUATION { scalar: __pa7, .. } => (__pa8.clone(), __pa7.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        res = __pa7.clone();
                        eqSolved = __pa8.clone();
                        let true = (isSimple(res.clone())?) else { bail!("pattern mismatch") };
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found Equation knw1: ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                        HTCrToExp = addToCrToExp(cr.clone(), eqSolved.clone(), HTCrToExp.clone(), HTCrToCrEqLst.clone())?;
                        keepEquation = false;
                    } else if count.clone() == 2 && findAliases.clone() {
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found Equation al0: ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                        let (__pa9, __pa10) = ::match_deref::match_deref! { match &(cr_lst.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Nil } } => (__pa9.clone(), __pa10.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        cr2 = __pa9.clone();
                        cr1 = __pa10.clone();
                        let false = (BackendVariable::isState(cr1.clone(), vars.clone())? || BackendVariable::isState(cr2.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                        let false = (BackendVariable::isClockedState(cr1.clone(), vars.clone())? || BackendVariable::isClockedState(cr2.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                        let false = (BackendVariable::isOutput(cr1.clone(), vars.clone())? || BackendVariable::isOutput(cr2.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                        let false = (BackendVariable::isDiscrete(cr1.clone(), vars.clone())? || BackendVariable::isDiscrete(cr2.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                        exp1 = Expression::crefExp(cr1.clone())?;
                        let true = (Types::isSimpleType(Expression::r#typeof(exp1.clone())?)) else { bail!("pattern mismatch") };
                        exp2 = Expression::crefExp(cr2.clone())?;
                        let __pa12 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), exp2.clone(), None)?) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa12, .. } => __pa12.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        res = __pa12.clone();
                        let true = (isSimple(res.clone())?) else { bail!("pattern mismatch") };
                        let __pa13 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), exp1.clone(), None)?) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa13, .. } => __pa13.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        res = __pa13.clone();
                        let true = (isSimple(res.clone())?) else { bail!("pattern mismatch") };
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found Equation al1: ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                        HTCrToCrEqLst = addToCrAndEqLists(cr2.clone(), cr1.clone(), inEq.clone(), HTCrToCrEqLst.clone())?;
                        HTCrToCrEqLst = addToCrAndEqLists(cr1.clone(), cr2.clone(), inEq.clone(), HTCrToCrEqLst.clone())?;
                        if BaseHashTable::hasKey(cr2.clone(), HTCrToExp.clone()) {
                            value = BaseHashTable::get(cr2.clone(), HTCrToExp.clone())?;
                            let (__pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr1.clone())?, None)?) {
                                        Deref @ BackendDAE::Equation::EQUATION { attr: __pa14, source: __pa15, scalar: __pa16, .. } => (__pa14.clone(), __pa15.clone(), __pa16.clone()),
                                        _ => bail!("pattern mismatch"),
                            } };
                            eqAttr = __pa14.clone();
                            source = __pa15.clone();
                            res = __pa16.clone();
                            (res, _) = Expression::replaceExp(res.clone(), Expression::crefExp(cr2.clone())?, value.clone())?;
                            (res, _) = ExpressionSimplify::simplify(res.clone())?;
                            HTCrToExp = addToCrToExp(cr1.clone(), Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(cr1.clone())?, scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() }), HTCrToExp.clone(), HTCrToCrEqLst.clone())?;
                        } else {
                            if BaseHashTable::hasKey(cr1.clone(), HTCrToExp.clone()) {
                                        value = BaseHashTable::get(cr1.clone(), HTCrToExp.clone())?;
                                        let (__pa17, __pa18, __pa19) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr2.clone())?, None)?) {
                                            Deref @ BackendDAE::Equation::EQUATION { attr: __pa17, source: __pa18, scalar: __pa19, .. } => (__pa17.clone(), __pa18.clone(), __pa19.clone()),
                                            _ => bail!("pattern mismatch"),
                                        } };
                                        eqAttr = __pa17.clone();
                                        source = __pa18.clone();
                                        res = __pa19.clone();
                                        (res, _) = Expression::replaceExp(res.clone(), Expression::crefExp(cr1.clone())?, value.clone())?;
                                        (res, _) = ExpressionSimplify::simplify(res.clone())?;
                                        HTCrToExp = addToCrToExp(cr2.clone(), Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(cr2.clone())?, scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() }), HTCrToExp.clone(), HTCrToCrEqLst.clone())?;
                            }
                        }
                        keepEquation = false;
                    }
                    if keepEquation.clone() {
                        eqList = cons(inEq.clone(), eqList.clone());
                    } else {
                        simpleEqList = cons(inEq.clone(), simpleEqList.clone());
                    }
                    Ok((inEq.clone(), (vars.clone(), HTCrToExp.clone(), HTCrToCrEqLst.clone(), eqList.clone(), simpleEqList.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (vars, HTCrToExp, HTCrToCrEqLst, eqList, simpleEqList)) => {
                    let mut eqList = (*eqList).clone();
                    eqList = cons(inEq.clone(), eqList.clone());
                    Ok((inEq.clone(), (vars.clone(), HTCrToExp.clone(), HTCrToCrEqLst.clone(), eqList.clone(), simpleEqList.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.findSimpleEquations ++++++++++\n")).clone());
                    Ok((inEq.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEq, outTuple))
}

fn findCrefs(mut inE1: Arc<DAE::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool))> {
    let mut outE1: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTuple: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool);
    (outE1, cont, outTuple) = 'mc: {
        let __mc_input = (inE1.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, vars, count, _, _)) => {
                    if !((count.clone() < 0)) { bail!("guard") }
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (cr_lst, vars, count, paramCount, true)) => {
                    if !((count.clone() < 2 && !(ComponentReferenceBasics::crefEqual(cr.clone(), DAE::crefTime().clone())?))) { bail!("guard") }
                    (_, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok((inE1.clone(), true, (cons(cr.clone(), cr_lst.clone()), vars.clone(), count.clone() + 1, paramCount.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (cr_lst, vars, count, paramCount, true)) => {
                    if !((count.clone() < 2 && !(ComponentReferenceBasics::crefEqual(cr.clone(), DAE::crefTime().clone())?))) { bail!("guard") }
                    Ok((inE1.clone(), true, (cr_lst.clone(), vars.clone(), count.clone(), paramCount.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, (_, vars, _, _, true)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { .. }, (_, vars, _, _, _)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, (_, vars, _, _, _)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { .. }, (_, vars, _, _, _)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { .. }, (_, vars, _, _, _)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inE1.clone(), true, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outE1, cont, outTuple))
}

fn addToCrAndEqLists(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>, mut eq: Arc<BackendDAE::Equation>, mut inHTCrToCrEqLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))> {
    let mut outHTCrToCrEqLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr));
    let mut cr_eq_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut eqSolved: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outHTCrToCrEqLst = (match inHTCrToCrEqLst.clone() {
        mut HTCrToCrEqLst => {
            eqSolved = BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr2.clone())?, None)?;
            if BaseHashTable::hasKey(cr1.clone(), HTCrToCrEqLst.clone()) {
                cr_eq_lst = BaseHashTable::get(cr1.clone(), HTCrToCrEqLst.clone())?;
                cr_eq_lst = cons((cr2.clone(), eqSolved.clone()), cr_eq_lst.clone());
            } else {
                cr_eq_lst = list![(cr2.clone(), eqSolved.clone())];
            }
            HTCrToCrEqLst = BaseHashTable::add((cr1.clone(), cr_eq_lst.clone()), HTCrToCrEqLst.clone())?;
            HTCrToCrEqLst.clone()
        },
        _ => {
            println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.addToCrAndEqLists ++++++++++\n")).clone());
            BackendDump::printEquation(eq.clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Solve for:")); __mm_s.push_str(&*ComponentReference::debugPrintComponentRefTypeStr(cr1.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail")
        },
    });
    Ok(outHTCrToCrEqLst)
}

fn addToCrToExp(mut cr: Arc<DAE::ComponentRef>, mut eq: Arc<BackendDAE::Equation>, mut inHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)), mut inHTCrToCrEqLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))> {
    let mut outHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr));
    let mut value: Arc<DAE::Exp>;
    outHTCrToExp = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut value: Arc<DAE::Exp>;
            let mut outHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr));
            let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr.clone())?, None)?) {
                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            value = __pa0.clone();
            outHTCrToExp = BaseHashTable::add((cr.clone(), value.clone()), inHTCrToExp.clone())?;
            outHTCrToExp = solveAllCrefs(cr.clone(), value.clone(), outHTCrToExp.clone(), inHTCrToCrEqLst.clone())?;
            Ok(outHTCrToExp.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.addToCrToExp ++++++++++\n")).clone());
            BackendDump::printEquation(eq.clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReference::debugPrintComponentRefTypeStr(cr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHTCrToExp)
}

fn solveAllCrefs(mut cr: Arc<DAE::ComponentRef>, mut value: Arc<DAE::Exp>, mut inHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)), mut inHTCrToCrEqLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))> {
    let mut outHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr));
    outHTCrToExp = 'mc: {
        let __mc_input = inHTCrToExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut HTCrToExp = __mc_input.clone() else { bail!("nomatch") };
            let mut cr_eq_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
            if BaseHashTable::hasKey(cr.clone(), inHTCrToCrEqLst.clone()) {
                cr_eq_lst = BaseHashTable::get(cr.clone(), inHTCrToCrEqLst.clone())?;
                HTCrToExp = solveAllCrefs1(cr.clone(), value.clone(), cr_eq_lst.clone(), HTCrToExp.clone(), inHTCrToCrEqLst.clone())?;
            }
            Ok(HTCrToExp.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.solveAllCrefs ++++++++++\n")).clone());
            Ok(inHTCrToExp.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHTCrToExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn solveAllCrefs1(mut cr: Arc<DAE::ComponentRef>, mut value: Arc<DAE::Exp>, mut cr_eq_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>, mut inHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)), mut inHTCrToCrEqLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))> {
    let mut outHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr));
    outHTCrToExp = 'mc: {
        let __mc_input = (cr.clone(), value.clone(), cr_eq_lst.clone(), inHTCrToExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, HTCrToExp) => {
                    Ok(HTCrToExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: (cr1, eq), tail: cr_eq_rest }, HTCrToExp) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut eqAttr: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
                    let mut HTCrToExp = (*HTCrToExp).clone();
                    if !(BaseHashTable::hasKey(cr1.clone(), HTCrToExp.clone())) && !(isCrefInValue(cr1.clone(), value.clone())?) {
                        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr1.clone())?, None)?) {
                            Deref @ BackendDAE::Equation::EQUATION { attr: __pa0, source: __pa1, scalar: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        eqAttr = __pa0.clone();
                        source = __pa1.clone();
                        res = __pa2.clone();
                        (res, _) = Expression::replaceExp(res.clone(), Expression::crefExp(cr.clone())?, value.clone())?;
                        (res, _) = ExpressionSimplify::simplify(res.clone())?;
                        HTCrToExp = addToCrToExp(cr1.clone(), Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(cr1.clone())?, scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() }), inHTCrToExp.clone(), inHTCrToCrEqLst.clone())?;
                    }
                    HTCrToExp = solveAllCrefs1(cr.clone(), value.clone(), cr_eq_rest.clone(), HTCrToExp.clone(), inHTCrToCrEqLst.clone())?;
                    Ok(HTCrToExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.solveAllCrefs1 ++++++++++\n")).clone());
                    Ok(inHTCrToExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHTCrToExp)
}

fn isCrefInValue(mut cr: Arc<DAE::ComponentRef>, mut value: Arc<DAE::Exp>) -> Result<bool> {
    let mut isInValue: bool = false;
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    cr_lst = Expression::extractCrefsFromExp(value.clone())?;
    isInValue = listMember(cr.clone(), cr_lst.clone());
    Ok(isInValue)
}

fn addRestCrefs(mut tplCrEqLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>, mut inHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)), mut inHTCrToCrEqLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))> {
    let mut HTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)) = inHTCrToExp.clone();
    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr_eq_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    if '__try0: {
        for mut tpl in &*tplCrEqLst.clone() {
            let mut tpl = tpl.clone();
            (cr1, cr_eq_lst) = tpl.clone();
            if !(BaseHashTable::hasKey(cr1.clone(), HTCrToExp.clone())) {
                HTCrToExp = unwrap_break_err!(addThisCrefs(cr_eq_lst.clone(), HTCrToExp.clone(), inHTCrToCrEqLst.clone()), '__try0);
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.addRestCrefs ++++++++++\n")).clone());
        bail!("fail");
    }
    Ok(HTCrToExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addThisCrefs(mut cr_eq_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>, mut inHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr)), mut inHTCrToCrEqLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr))> {
    let mut outHTCrToExp: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr));
    outHTCrToExp = 'mc: {
        let __mc_input = (cr_eq_lst.clone(), inHTCrToExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, HTCrToExp) => {
                    Ok(HTCrToExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cr1, eq), tail: cr_eq_rest }, HTCrToExp) => {
                    let mut HTCrToExp = (*HTCrToExp).clone();
                    if !(BaseHashTable::hasKey(cr1.clone(), HTCrToExp.clone())) {
                        HTCrToExp = addToCrToExp(cr1.clone(), eq.clone(), HTCrToExp.clone(), inHTCrToCrEqLst.clone())?;
                    }
                    HTCrToExp = addThisCrefs(cr_eq_rest.clone(), HTCrToExp.clone(), inHTCrToCrEqLst.clone())?;
                    Ok(HTCrToExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.addThisCrefs ++++++++++\n")).clone());
                    Ok(inHTCrToExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHTCrToExp)
}

fn isSimple(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outIsSimple: bool = false;
    (_, outIsSimple) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(checkOperator) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), true)?;
    Ok(outIsSimple)
}

fn checkOperator(mut inExp: Arc<DAE::Exp>, mut inIsSimple: bool) -> Result<(Arc<DAE::Exp>, bool, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outIsSimple: bool = false;
    (outExp, cont, outIsSimple) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1, operator: op, exp2 } => {
                    let true = (checkOp(op.clone())) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(checkOperator(exp1.clone(), inIsSimple.clone())?) {
                        (_, true, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(checkOperator(exp2.clone(), inIsSimple.clone())?) {
                        (_, true, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((inExp.clone(), true, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: _, exp: exp1 } => {
                    Ok(checkOperator(exp1.clone(), inIsSimple.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: _, exp: exp1 } => {
                    Ok(checkOperator(exp1.clone(), inIsSimple.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), false, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outIsSimple))
}

fn checkOp(mut inOp: DAE::Operator) -> bool {
    let mut outB: bool = false;
    outB = (match inOp.clone() {
        DAE::Operator::ADD { .. } => true,
        DAE::Operator::SUB { .. } => true,
        DAE::Operator::UMINUS { .. } => true,
        DAE::Operator::MUL { .. } => false,
        DAE::Operator::EQUAL { .. } => false,
        DAE::Operator::DIV { .. } => false,
        DAE::Operator::POW { .. } => false,
        _ => false,
    });
    outB
}

fn determineAliasLst(mut inAliasVars: BackendDAE::Variables, mut inVars: BackendDAE::Variables, mut inHTAliasLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr))> {
    let mut outHTAliasLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr)) = inHTAliasLst.clone();
    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut e: Arc<DAE::Exp>;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut w: Option<BackendDAE::Var> = None;
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut i: i32 = 0;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    let mut bs: bool = false;
    let mut count: i32 = 0;
    let mut vars: metamodelica::Array<Option<BackendDAE::Var>>;
    let BackendDAE::VARIABLES { varArr: BackendDAE::VARIABLE_ARRAY { varOptArr: __pa0, .. }, .. } = (inAliasVars.clone()) else { bail!("pattern mismatch") };
    vars = __pa0.clone();
    let __range1 = vars.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut w in __range1 {
        match '__try2: {
            let Some(__pa3) = (w.clone()) else { break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            v = __pa3.clone();
            cr1 = unwrap_break_err!(BackendVariable::varCref(v.clone()), '__try2);
            e = unwrap_break_err!(BackendVariable::varBindExp(v.clone()), '__try2);
            let (_, (__pa4, _, __pa5, _, _)) = unwrap_break_err!(Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(findCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, BackendDAE::Variables, i32, i32, bool))> + 'static>), (metamodelica::nil(), inVars.clone(), 0, 0, true)), '__try2);
            cr_lst = __pa4.clone();
            count = __pa5.clone();
            let 1 = (count.clone()) else { break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            let __pa6 = ::match_deref::match_deref! { match &(cr_lst.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil } => __pa6.clone(),
                _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            cr2 = __pa6.clone();
            eq = Arc::new(BackendDAE::Equation::EQUATION { exp: unwrap_break_err!(Expression::crefExp(cr1.clone()), '__try2), scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() });
            outHTAliasLst = unwrap_break_err!(addToCrAndEqLists(cr2.clone(), cr1.clone(), eq.clone(), outHTAliasLst.clone()), '__try2);
            Ok::<_, anyhow::Error>((outHTAliasLst.clone(),))
        } {
            Ok((__try2_o0,)) => {
                outHTAliasLst = __try2_o0;
            }
            Err(_) => {
                outHTAliasLst = outHTAliasLst.clone();
            }
        }
    }
    Ok(outHTAliasLst)
}

fn getAliasAttributes(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut aliasVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut HTAliasLst: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr));
    let mut tplAliasLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>> = metamodelica::nil();
    let mut size: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(inSystem.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orderedVars = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(inShared.clone()) {
        Deref @ BackendDAE::Shared { aliasVars: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    aliasVars = __pa1.clone();
    size = BackendVariable::varsSize(orderedVars.clone())?;
    size = intMax(BaseHashTable::defaultBucketSize.clone(), (((intReal(size.clone())) * (metamodelica::OrderedFloat(0.7_f64))).0 as i32));
    HTAliasLst = HashTableCrToCrEqLst::emptyHashTableSized(size.clone());
    HTAliasLst = determineAliasLst(aliasVars.clone(), orderedVars.clone(), HTAliasLst.clone())?;
    tplAliasLst = BaseHashTable::hashTableList(HTAliasLst.clone());
    orderedVars = setAttributes(tplAliasLst.clone(), orderedVars.clone(), aliasVars.clone());
    outSystem = BackendDAEUtil::setEqSystVars(inSystem.clone(), orderedVars.clone())?;
    Ok((outSystem, outShared))
}

fn setAttributes(mut tplCrEqLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>>, mut inVars: BackendDAE::Variables, mut inAliasVars: BackendDAE::Variables) -> BackendDAE::Variables {
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut e: Arc<DAE::Exp>;
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut tplCrEqRest: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>)>> = metamodelica::nil();
    let mut cr_eq_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut HTStartExpToInt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut HTNominalExpToInt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut tplExpIndList: Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>> = metamodelica::nil();
    if tplCrEqLst.clone().is_empty() {
        return outVars;
    }
    if '__try0: {
        HTStartExpToInt = HashTableExpToIndex::emptyHashTableSized(100);
        HTNominalExpToInt = HashTableExpToIndex::emptyHashTableSized(100);
        for mut tpl in &*tplCrEqLst.clone() {
            let mut tpl = tpl.clone();
            (cr1, cr_eq_lst) = tpl.clone();
            unwrap_break_err!(BaseHashTable::clear(HTStartExpToInt.clone()), '__try0);
            unwrap_break_err!(BaseHashTable::clear(HTNominalExpToInt.clone()), '__try0);
            (v, i) = unwrap_break_err!(BackendVariable::getVarSingle(cr1.clone(), outVars.clone()), '__try0);
            if unwrap_break_err!(BackendVariable::varHasStartValue(v.clone()), '__try0) {
                e = unwrap_break_err!(BackendVariable::varStartValue(v.clone()), '__try0);
                if Expression::isZero(e.clone()) {
                    e = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
                }
                cr_lst = unwrap_break_err!(Expression::extractCrefsFromExp(e.clone()), '__try0);
                j = 2 - (cr_lst.clone().len() as i32);
                j = j.clone() * unwrap_break_err!(ComponentReference::crefDepth(cr1.clone()), '__try0);
                HTStartExpToInt = unwrap_break_err!(BaseHashTable::add((e.clone(), j.clone()), HTStartExpToInt.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALIAS.clone()), '__try0) {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("START: ")); __mm_s.push_str(&*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(cr1.clone()), '__try0)); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            if BackendVariable::varHasNominalValue(v.clone()) {
                e = unwrap_break_err!(BackendVariable::varNominalValue(v.clone()), '__try0);
                cr_lst = unwrap_break_err!(Expression::extractCrefsFromExp(e.clone()), '__try0);
                j = 2 - (cr_lst.clone().len() as i32);
                j = j.clone() * unwrap_break_err!(ComponentReference::crefDepth(cr1.clone()), '__try0);
                HTNominalExpToInt = unwrap_break_err!(BaseHashTable::add((e.clone(), j.clone()), HTNominalExpToInt.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALIAS.clone()), '__try0) {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NOMINAL: ")); __mm_s.push_str(&*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(cr1.clone()), '__try0)); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            (HTStartExpToInt, HTNominalExpToInt) = unwrap_break_err!(getThisAttributes(cr1.clone(), cr_eq_lst.clone(), inAliasVars.clone(), HTStartExpToInt.clone(), HTNominalExpToInt.clone()), '__try0);
            tplExpIndList = BaseHashTable::hashTableList(HTStartExpToInt.clone());
            if !(tplExpIndList.clone().is_empty()) {
                e = getDominantAttributeValue(tplExpIndList.clone());
                v = unwrap_break_err!(BackendVariable::setVarStartValue(v.clone(), e.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALIAS.clone()), '__try0) {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("START: ")); __mm_s.push_str(&*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(cr1.clone()), '__try0)); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    BaseHashTable::dumpHashTable(HTStartExpToInt.clone());
                }
            }
            tplExpIndList = BaseHashTable::hashTableList(HTNominalExpToInt.clone());
            if !(tplExpIndList.clone().is_empty()) {
                e = getDominantAttributeValue(tplExpIndList.clone());
                v = unwrap_break_err!(BackendVariable::setVarNominalValue(v.clone(), e.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALIAS.clone()), '__try0) {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NOMINAL: ")); __mm_s.push_str(&*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(cr1.clone()), '__try0)); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    BaseHashTable::dumpHashTable(HTNominalExpToInt.clone());
                }
            }
            outVars = unwrap_break_err!(BackendVariable::setVarAt(outVars.clone(), i.clone(), v.clone()), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.setAttributes ++++++++++\n")).clone());
    }
    outVars
}

fn getThisAttributes(mut cr: Arc<DAE::ComponentRef>, mut cr_eq_lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<BackendDAE::Equation>)>>, mut inAliasVars: BackendDAE::Variables, mut inHTStartExpToInt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), mut inHTNominalExpToInt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)))> {
    let mut outHTStartExpToInt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)) = inHTStartExpToInt.clone();
    let mut outHTNominalExpToInt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)) = inHTNominalExpToInt.clone();
    (outHTStartExpToInt, outHTNominalExpToInt) = 'mc: {
        let __mc_input = cr_eq_lst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((outHTStartExpToInt.clone(), outHTNominalExpToInt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (cr1, _), tail: cr_eq_rest } => {
                    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut res: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut j: i32 = 0;
                    let mut j1: i32 = 0;
                    let mut outHTNominalExpToInt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)) = outHTNominalExpToInt.clone();
                    let mut outHTStartExpToInt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)) = outHTStartExpToInt.clone();
                    (v, _) = BackendVariable::getVarSingle(cr1.clone(), inAliasVars.clone())?;
                    e = BackendVariable::varBindExp(v.clone())?;
                    if BackendVariable::varHasStartValue(v.clone())? {
                        res = BackendVariable::varStartValue(v.clone())?;
                        let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(cr1.clone())?, scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() }), Expression::crefExp(cr.clone())?, None)?) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, .. } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        e1 = __pa0.clone();
                        let __pa1 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(Arc::new(BackendDAE::Equation::EQUATION { exp: res.clone(), scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() }), Expression::crefExp(cr.clone())?, None)?) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa1, .. } => __pa1.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        e2 = __pa1.clone();
                        (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                        if Expression::isZero(e2.clone()) {
                            e2 = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
                        }
                        cr_lst = Expression::extractCrefsFromExp(e2.clone())?;
                        j = 2 - (cr_lst.clone().len() as i32);
                        j = j.clone() * ComponentReference::crefDepth(cr1.clone())?;
                        if BaseHashTable::hasKey(e2.clone(), outHTStartExpToInt.clone()) {
                            j1 = BaseHashTable::get(e2.clone(), outHTStartExpToInt.clone())?;
                            if j1.clone() < j.clone() {
                                        j = j1.clone();
                            }
                        }
                        outHTStartExpToInt = BaseHashTable::add((e2.clone(), j.clone()), outHTStartExpToInt.clone())?;
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("START: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                    }
                    if BackendVariable::varHasNominalValue(v.clone()) {
                        e2 = BackendVariable::varNominalValue(v.clone())?;
                        cr_lst = Expression::extractCrefsFromExp(e2.clone())?;
                        j = 2 - (cr_lst.clone().len() as i32);
                        j = j.clone() * ComponentReference::crefDepth(cr1.clone())?;
                        if BaseHashTable::hasKey(e2.clone(), outHTNominalExpToInt.clone()) {
                            j1 = BaseHashTable::get(e2.clone(), outHTNominalExpToInt.clone())?;
                            if j1.clone() < j.clone() {
                                        j = j1.clone();
                            }
                        }
                        outHTNominalExpToInt = BaseHashTable::add((e2.clone(), j.clone()), outHTNominalExpToInt.clone())?;
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NOMINAL: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                    }
                    (outHTStartExpToInt, outHTNominalExpToInt) = getThisAttributes(cr.clone(), cr_eq_rest.clone(), inAliasVars.clone(), outHTStartExpToInt.clone(), outHTNominalExpToInt.clone())?;
                    Ok((outHTStartExpToInt.clone(), outHTNominalExpToInt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("\n++++++++++ Error in RemoveSimpleEquations.getThisAttributes ++++++++++\n")).clone());
                    Ok((outHTStartExpToInt.clone(), outHTNominalExpToInt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outHTStartExpToInt, outHTNominalExpToInt))
}

fn getDominantAttributeValue(mut tplExpIndList: Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>) -> Arc<DAE::Exp> {
    let mut outE: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut tpl: (Arc<DAE::Exp>, i32);
    let mut i: i32 = 0;
    let mut j: i32 = 111111;
    for mut tpl in &*tplExpIndList.clone() {
        let mut tpl = tpl.clone();
        (e, i) = tpl.clone();
        if i.clone() < j.clone() {
            outE = e.clone();
            j = i.clone();
        }
    }
    outE
}

fn dumpSimpleContainer(mut container: SimpleContainer) -> Result<ArcStr> {
    let mut sOut: ArcStr = arcstr::literal!("");
    sOut = ('mc: {
        let __mc_input = container.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let SimpleContainer::ALIAS { cr1: mut cr1, negatedCr1: mut n1, i1: mut i1, cr2: mut cr2, negatedCr2: mut n2, i2: mut i2, eqnAttributes: _, visited: _ } = __mc_input.clone() else { bail!("nomatch") };
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (if (n1.clone()) {literal!("(-)")} else {literal!("")}).clone();
            s2 = (if (n2.clone()) {literal!("(-)")} else {literal!("")}).clone();
            s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr1.clone())?); ArcStr::from(__mm_s) }).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr2.clone())?); ArcStr::from(__mm_s) }).clone();
            Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ALIASE: \t\t")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("  (")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(i2.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SimpleContainer::PARAMETERALIAS { unknowncr: ref cr1, negatedCr1: mut n1, i1: mut i1, paramcr: ref cr2, negatedCr2: mut n2, i2: mut i2, eqnAttributes: _, visited: _ } = __mc_input.clone() else { bail!("nomatch") };
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (if (n1.clone()) {literal!("(-)")} else {literal!("")}).clone();
            s2 = (if (n2.clone()) {literal!("(-)")} else {literal!("")}).clone();
            s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr1.clone())?); ArcStr::from(__mm_s) }).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr2.clone())?); ArcStr::from(__mm_s) }).clone();
            Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PARAMETERALIASE: \t")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("  (")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(i2.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SimpleContainer::TIMEALIAS { cr1: mut cr1, negatedCr1: mut n1, i1: mut i1, cr2: mut cr2, negatedCr2: mut n2, i2: mut i2, eqnAttributes: _, visited: _ } = __mc_input.clone() else { bail!("nomatch") };
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (if (n1.clone()) {literal!("(-)")} else {literal!("")}).clone();
            s2 = (if (n2.clone()) {literal!("(-)")} else {literal!("")}).clone();
            s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr1.clone())?); ArcStr::from(__mm_s) }).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr2.clone())?); ArcStr::from(__mm_s) }).clone();
            Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TIMEALIASE: \t")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("  (")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(i2.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SimpleContainer::TIMEINDEPENTVAR { cr: ref cr1, i: _, exp: ref e, eqnAttributes: _, visited: _ } = __mc_input.clone() else { bail!("nomatch") };
            Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TIMEINDEPENT: \t")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!("----------"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(sOut)
}

