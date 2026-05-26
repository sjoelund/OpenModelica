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
use crate::DAEDump;
use crate::DAEUtil;
use crate::Expression;
use crate::FCore;
use crate::HashTableCrToExpOption;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

///
/// Properties of a transition
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transition {
    pub from: i32,
    pub to: i32,
    pub condition: Arc<DAE::Exp>,
    pub immediate: bool,
    pub reset: bool,
    pub synchronize: bool,
    pub priority: i32,
}

pub type TRANSITION = Transition;


///
/// Structure that combines states of flat state machine in
/// canonical order with governing semantic equations.
#[derive(Clone, Debug, PartialEq)]
pub struct FlatSmSemantics {
    pub ident: ArcStr,
    /// First element is the initial state
    pub smComps: metamodelica::Array<Arc<DAE::Element>>,
    /// List/Array of transition data sorted in priority
    pub t: Arc<metamodelica::List<Transition>>,
    /// Transition conditions sorted in priority
    pub c: Arc<metamodelica::List<Arc<DAE::Exp>>>,
    /// SMS veriables
    pub vars: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// SMS constants/parameters
    pub knowns: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// SMS equations
    pub eqs: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// Propagation related variables
    pub pvars: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// Propagation equations
    pub peqs: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// Cref to enclosing state if any
    pub enclosingState: Option<Arc<DAE::ComponentRef>>,
}

pub type FLAT_SM_SEMANTICS = FlatSmSemantics;


pub const SMS_PRE: &'static str = "smOf";

pub fn stateMachineToDataFlow(mut cache: FCore::Cache, mut env: FCore::Graph, mut inDAElist: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDAElist: DAE::DAElist;
    let mut elementLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elementLst1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut flatSmLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elementLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elementLst3: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut t: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut compElem: Arc<DAE::Element>;
    let mut nOfSubstitutions: i32 = 0;
    let mut ident: ArcStr = arcstr::literal!("");
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource>;
    let mut comment: Option<Arc<SCode::Comment>> = None;
    let DAE::DAE { elementLst: __pa0 } = (inDAElist.clone()) else { bail!("pattern mismatch") };
    elementLst = __pa0.clone();
    assert!((elementLst.clone().len() as i32) == 1, "{}", &*(literal!("Internal compiler error: Handling of elementLst != 1 not supported\n")).clone());
    let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(listHead(elementLst.clone())?) {
        Deref @ DAE::Element::COMP { ident: __pa1, dAElist: __pa2, source: __pa3, comment: __pa4 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ident = __pa1.clone();
    dAElist = __pa2.clone();
    source = __pa3.clone();
    comment = __pa4.clone();
    if !(List::any(dAElist.clone(), Arc::new(fnptr!(isFlatSm, Arc<DAE::Element>)))) {
        outDAElist = inDAElist.clone();
        return Ok(outDAElist);
    }
    (flatSmLst, otherLst) = List::extractOnTrue(dAElist.clone(), Arc::new(fnptr!(isFlatSm, Arc<DAE::Element>)));
    elementLst2 = List::fold2(flatSmLst.clone(), Arc::new(flatSmToDataFlow), None, None, metamodelica::nil());
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        elementLst2 = wrapHack(cache.clone(), elementLst2.clone())?;
    }
    elementLst3 = listAppend(otherLst.clone(), elementLst2.clone());
    outDAElist = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMP { ident: (ident.clone()).clone(), dAElist: elementLst3.clone(), source: source.clone(), comment: comment.clone() })] };
    let (__pa5, _, (_, __pa6)) = DAEUtil::traverseDAE(outDAElist.clone(), FCore::getFunctionTree(cache.clone()), Arc::new(Expression::traverseSubexpressionsHelper), (traversingSubsActiveState, 0));
    outDAElist = __pa5.clone();
    nOfSubstitutions = __pa6.clone();
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        let (__pa7, _, (_, __pa8)) = DAEUtil::traverseDAE(outDAElist.clone(), FCore::getFunctionTree(cache.clone()), Arc::new(Expression::traverseSubexpressionsHelper), (fnptr!(traversingSubsPreForPrevious, Arc<DAE::Exp>, i32), 0));
        outDAElist = __pa7.clone();
        nOfSubstitutions = __pa8.clone();
    }
    Ok(outDAElist)
}

fn traversingSubsActiveState(mut inExp: Arc<DAE::Exp>, mut inHitCount: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHitCount: i32 = 0;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "activeState" }, .. } => {
            (Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::crefPrependIdent(componentRef.clone(), (literal!("active")).clone(), metamodelica::nil(), DAE::T_BOOL_DEFAULT.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() }), inHitCount.clone() + 1)
        },
        _ => {
            (inExp.clone(), inHitCount.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHitCount))
}

fn flatSmToDataFlow(mut inFlatSm: Arc<DAE::Element>, mut inEnclosingStateCrefOption: Option<Arc<DAE::ComponentRef>>, mut inEnclosingFlatSmSemanticsOption: Option<FlatSmSemantics>, mut accElems: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElems: Arc<metamodelica::List<Arc<DAE::Element>>> = accElems.clone();
    let mut ident: ArcStr = arcstr::literal!("");
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smCompsLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut transitionLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst3: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut eqnLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst4: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smCompsLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut initialStateOp: Arc<DAE::Element>;
    let mut initialStateComp: Arc<DAE::Element>;
    let mut crefInitialState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut flatSmSemanticsBasics: FlatSmSemantics;
    let mut flatSmSemanticsWithPropagation: FlatSmSemantics;
    let mut flatSmSemantics: FlatSmSemantics;
    let mut transitions: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut knowns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut pvars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut peqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inFlatSm.clone()) {
        Deref @ DAE::Element::FLAT_SM { dAElist: __pa0, ident: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dAElist = __pa0.clone();
    ident = __pa1.clone();
    (smCompsLst, otherLst1) = List::extractOnTrue(dAElist.clone(), Arc::new(fnptr!(isSMComp, Arc<DAE::Element>)));
    (transitionLst, otherLst2) = List::extractOnTrue(otherLst1.clone(), Arc::new(fnptr!(isTransition, Arc<DAE::Element>)));
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(List::extractOnTrue(otherLst2.clone(), Arc::new(fnptr!(isInitialState, Arc<DAE::Element>)))) {
        (Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }, __pa3) => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    initialStateOp = __pa2.clone();
    otherLst3 = __pa3.clone();
    (eqnLst, otherLst4) = List::extractOnTrue(otherLst3.clone(), Arc::new(fnptr!(isEquation, Arc<DAE::Element>)));
    assert!(otherLst4.clone().is_empty(), "{}", &*(literal!("Internal compiler error. Unexpected elements in flat state machine.")).clone());
    let __pa5 = ::match_deref::match_deref! { match &(initialStateOp.clone()) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa5, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, .. }, .. } => __pa5.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crefInitialState = __pa5.clone();
    let (__pa7, __pa8) = ::match_deref::match_deref! { match &(List::extract1OnTrue(smCompsLst.clone(), Arc::new(sMCompEqualsRef), crefInitialState.clone())) {
        (Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil }, __pa8) => (__pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    initialStateComp = __pa7.clone();
    smCompsLst2 = __pa8.clone();
    flatSmSemanticsBasics = basicFlatSmSemantics((ident.clone()).clone(), cons(initialStateComp.clone(), smCompsLst2.clone()), transitionLst.clone())?;
    flatSmSemanticsWithPropagation = addPropagationEquations(flatSmSemanticsBasics.clone(), inEnclosingStateCrefOption.clone(), inEnclosingFlatSmSemanticsOption.clone())?;
    flatSmSemantics = elabXInStateOps(flatSmSemanticsWithPropagation.clone(), inEnclosingStateCrefOption.clone())?;
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        smCompsLst = List::map(smCompsLst.clone(), Arc::new(elabXInStateOps_CT.clone()));
    }
    let FlatSmSemantics { peqs: __pa10, pvars: __pa11, eqs: __pa12, knowns: __pa13, vars: __pa14, .. } = (flatSmSemantics.clone()) else { bail!("pattern mismatch") };
    peqs = __pa10.clone();
    pvars = __pa11.clone();
    eqs = __pa12.clone();
    knowns = __pa13.clone();
    vars = __pa14.clone();
    outElems = List::flatten(list![outElems.clone(), eqnLst.clone(), vars.clone(), knowns.clone(), eqs.clone(), pvars.clone(), peqs.clone()]);
    outElems = List::fold1(smCompsLst.clone(), Arc::new(smCompToDataFlow), flatSmSemantics.clone(), outElems.clone());
    Ok(outElems)
}

fn traversingSubsTicksInState(mut inExp: Arc<DAE::Exp>, mut inCref_HitCount: (Arc<DAE::ComponentRef>, i32)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outCref_HitCount: (Arc<DAE::ComponentRef>, i32);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut hitCount: i32 = 0;
    (cref, hitCount) = inCref_HitCount.clone();
    (outExp, outCref_HitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Nil, path: Deref @ Absyn::Path::IDENT { name: Deref @ "ticksInState" } } => {
            let mut crefTicksInState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            crefTicksInState = ComponentReference::joinCrefs(cref.clone(), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$ticksInState")).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }))?;
            (Arc::new(DAE::Exp::CREF { componentRef: crefTicksInState.clone(), ty: ty.clone() }), (cref.clone(), hitCount.clone() + 1))
        },
        _ => {
            (inExp.clone(), inCref_HitCount.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outCref_HitCount))
}

fn elabXInStateOps(mut inFlatSmSemantics: FlatSmSemantics, mut inEnclosingStateCrefOption: Option<Arc<DAE::ComponentRef>>) -> Result<FlatSmSemantics> {
    let mut outFlatSmSemantics: FlatSmSemantics;
    let mut i: i32 = 0;
    let mut found: bool = false;
    let mut c2: Arc<DAE::Exp>;
    let mut c3: Arc<DAE::Exp>;
    let mut c4: Arc<DAE::Exp>;
    let mut conditionNew: Arc<DAE::Exp>;
    let mut substTickExp: Arc<DAE::Exp>;
    let mut substTimeExp: Arc<DAE::Exp>;
    let mut stateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut t2: Transition;
    let mut tElab: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut cElab: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut smeqsElab: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut ident: ArcStr = arcstr::literal!("");
    let mut smComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut t: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut c: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut smvars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smknowns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smeqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut pvars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut peqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut enclosingStateOption: Option<Arc<DAE::ComponentRef>> = None;
    let mut from: i32 = 0;
    let mut to: i32 = 0;
    let mut condition: Arc<DAE::Exp>;
    let mut immediate: bool = false;
    let mut reset: bool = false;
    let mut synchronize: bool = false;
    let mut priority: i32 = 0;
    let FlatSmSemantics { ident: __pa0, smComps: __pa1, t: __pa2, c: __pa3, vars: __pa4, knowns: __pa5, eqs: __pa6, pvars: __pa7, peqs: __pa8, enclosingState: __pa9 } = (inFlatSmSemantics.clone()) else { bail!("pattern mismatch") };
    ident = __pa0.clone();
    smComps = __pa1.clone();
    t = __pa2.clone();
    c = __pa3.clone();
    smvars = __pa4.clone();
    smknowns = __pa5.clone();
    smeqs = __pa6.clone();
    pvars = __pa7.clone();
    peqs = __pa8.clone();
    enclosingStateOption = __pa9.clone();
    i = 0;
    for mut tc in &*List::zip(t.clone(), c.clone()) {
        let mut tc = tc.clone();
        i = i.clone() + 1;
        (t2, c2) = tc.clone();
        let Transition { from: __pa10, to: __pa11, condition: __pa12, immediate: __pa13, reset: __pa14, synchronize: __pa15, priority: __pa16 } = (t2.clone()) else { bail!("pattern mismatch") };
        from = __pa10.clone();
        to = __pa11.clone();
        condition = __pa12.clone();
        immediate = __pa13.clone();
        reset = __pa14.clone();
        synchronize = __pa15.clone();
        priority = __pa16.clone();
        let __pa17 = ::match_deref::match_deref! { match &(smComps.clone().borrow()[(from.clone()-1) as usize].clone()) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa17, .. } => __pa17.clone(),
            _ => bail!("pattern mismatch"),
        } };
        stateRef = __pa17.clone();
        substTickExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("$ticksInState")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), stateRef.clone())?, ty: DAE::T_INTEGER_DEFAULT.clone() });
        let (__pa18, (_, _, __pa19)) = Expression::traverseExpTopDown(c2.clone(), Arc::new(fnptr!(traversingSubsXInState, Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool))), (literal!("ticksInState"), substTickExp.clone(), false))?;
        c3 = __pa18.clone();
        found = __pa19.clone();
        if found.clone() && isSome(inEnclosingStateCrefOption.clone()) {
            Error::addCompilerError((literal!("Found 'ticksInState()' within a state of an hierarchical state machine.")).clone())?;
            bail!("fail");
        }
        smeqsElab = if (found.clone()) {List::map5(smeqs.clone(), Arc::new(smeqsSubsXInState), smComps.clone().borrow()[(1-1) as usize].clone(), i.clone(), (t.clone().len() as i32), substTickExp.clone(), (literal!("ticksInState")).clone())} else {smeqs.clone()};
        smeqs = smeqsElab.clone();
        substTimeExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("$timeInState")).clone(), DAE::T_REAL_DEFAULT.clone(), metamodelica::nil(), stateRef.clone())?, ty: DAE::T_REAL_DEFAULT.clone() });
        let (__pa20, (_, _, __pa21)) = Expression::traverseExpTopDown(c2.clone(), Arc::new(fnptr!(traversingSubsXInState, Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool))), (literal!("timeInState"), substTimeExp.clone(), false))?;
        c4 = __pa20.clone();
        found = __pa21.clone();
        if found.clone() && isSome(inEnclosingStateCrefOption.clone()) {
            Error::addCompilerError((literal!("Found 'timeInState()' within a state of an hierarchical state machine.")).clone())?;
            bail!("fail");
        }
        smeqsElab = if (found.clone()) {List::map5(smeqs.clone(), Arc::new(smeqsSubsXInState), smComps.clone().borrow()[(1-1) as usize].clone(), i.clone(), (t.clone().len() as i32), substTimeExp.clone(), (literal!("timeInState")).clone())} else {smeqs.clone()};
        smeqs = smeqsElab.clone();
        tElab = cons(Transition { from: from.clone(), to: to.clone(), condition: c4.clone(), immediate: immediate.clone(), reset: reset.clone(), synchronize: synchronize.clone(), priority: priority.clone() }, tElab.clone());
        cElab = cons(c4.clone(), cElab.clone());
    }
    outFlatSmSemantics = FlatSmSemantics { ident: (ident.clone()).clone(), smComps: smComps.clone(), t: tElab.clone().reverse(), c: cElab.clone().reverse(), vars: smvars.clone(), knowns: smknowns.clone(), eqs: smeqsElab.clone(), pvars: pvars.clone(), peqs: peqs.clone(), enclosingState: enclosingStateOption.clone() };
    Ok(outFlatSmSemantics)
}

fn smeqsSubsXInState(mut inSmeqs: Arc<DAE::Element>, mut initialStateComp: Arc<DAE::Element>, mut i: i32, mut nTransitions: i32, mut substExp: Arc<DAE::Exp>, mut xInState: ArcStr) -> Result<Arc<DAE::Element>> {
    let mut outSmeqs: Arc<DAE::Element>;
    let mut preRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut lhsRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crefInitialState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tArrayBool: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut elemSource: Arc<DAE::ElementSource>;
    let mut lhsExp: Arc<DAE::Exp>;
    let mut rhsExp: Arc<DAE::Exp>;
    let mut rhsExp2: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(initialStateComp.clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crefInitialState = __pa0.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), crefInitialState.clone());
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nTransitions.clone() })] });
    cref = qCref((literal!("cImmediate")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?;
    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inSmeqs.clone()) {
        Deref @ DAE::Element::EQUATION { exp: __pa1, scalar: __pa2, source: __pa3 } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhsExp = __pa1.clone();
    rhsExp = __pa2.clone();
    elemSource = __pa3.clone();
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(lhsExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: __pa5 } => (__pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhsRef = __pa4.clone();
    ty = __pa5.clone();
    if ComponentReferenceBasics::crefEqual(cref.clone(), lhsRef.clone())? {
        (rhsExp2, _) = Expression::traverseExpTopDown(rhsExp.clone(), Arc::new(fnptr!(traversingSubsXInState, Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool))), (xInState.clone(), substExp.clone(), false))?;
    } else {
        rhsExp2 = rhsExp.clone();
    }
    outSmeqs = Arc::new(DAE::Element::EQUATION { exp: lhsExp.clone(), scalar: rhsExp2.clone(), source: elemSource.clone() });
    Ok(outSmeqs)
}

fn traversingSubsXInState(mut inExp: Arc<DAE::Exp>, mut inXSubstHit: (ArcStr, Arc<DAE::Exp>, bool)) -> (Arc<DAE::Exp>, bool, (ArcStr, Arc<DAE::Exp>, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outXSubstHit: (ArcStr, Arc<DAE::Exp>, bool);
    (outExp, outXSubstHit) = (::match_deref::match_deref! { match &((inExp.clone(), inXSubstHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, (xInState, subsExp, _)) if (name.clone() == xInState.clone()) => {
            (subsExp.clone(), (xInState.clone(), subsExp.clone(), true))
        },
        _ => {
            (inExp.clone(), inXSubstHit.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outXSubstHit)
}

fn smCompToDataFlow(mut inSMComp: Arc<DAE::Element>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut accElems: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElems: Arc<metamodelica::List<Arc<DAE::Element>>> = accElems.clone();
    let mut varLst1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut varLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut assignedVarLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut stateVarLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut equationLst1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut equationLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut flatSmLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst3: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut stateVarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut startValuesOpt: Arc<metamodelica::List<Option<Arc<DAE::Exp>>>> = metamodelica::nil();
    let mut varCrefStartVal: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>> = metamodelica::nil();
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (HashTableCrToExpOption::FuncHashCref, HashTableCrToExpOption::FuncCrefEqual, HashTableCrToExpOption::FuncCrefStr, HashTableCrToExpOption::FuncExpStr));
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inSMComp.clone()) {
        Deref @ DAE::Element::SM_COMP { dAElist: __pa0, componentRef: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dAElist = __pa0.clone();
    componentRef = __pa1.clone();
    (varLst1, otherLst1) = List::extractOnTrue(dAElist.clone(), Arc::new(fnptr!(isVar, Arc<DAE::Element>)));
    (equationLst1, otherLst2) = List::extractOnTrue(otherLst1.clone(), Arc::new(fnptr!(isEquationOrWhenEquation, Arc<DAE::Element>)));
    assignedVarLst = List::filterOnTrue(varLst1.clone(), Arc::new({ let __pe_b0 = equationLst1.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = Arc::new(isVarAtLHS); move |__pe_a2| Ok(List::exist1(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)) }));
    stateVarLst = List::filterOnTrue(varLst1.clone(), Arc::new({ let __pe_b0 = equationLst1.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = Arc::new(isPreviousAppliedToVar); move |__pe_a2| Ok(List::exist1(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)) }));
    stateVarCrefs = List::map(stateVarLst.clone(), Arc::new(DAEUtil::varCref));
    startValuesOpt = List::map(stateVarLst.clone(), Arc::new(getStartAttrOption));
    varCrefStartVal = List::zip(stateVarCrefs.clone(), startValuesOpt.clone());
    crToExpOpt = HashTableCrToExpOption::emptyHashTableSized((varCrefStartVal.clone().len() as i32) + 1);
    crToExpOpt = List::fold(varCrefStartVal.clone(), Arc::new(BaseHashTable::add), crToExpOpt.clone());
    (equationLst2, varLst2) = List::fold3(equationLst1.clone(), Arc::new(addStateActivationAndReset), inSMComp.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt.clone(), (metamodelica::nil(), metamodelica::nil()));
    (flatSmLst, otherLst3) = List::extractOnTrue(otherLst2.clone(), Arc::new(fnptr!(isFlatSm, Arc<DAE::Element>)));
    outElems = List::flatten(list![outElems.clone(), varLst1.clone(), varLst2.clone(), equationLst2.clone(), otherLst3.clone()]);
    outElems = List::fold2(flatSmLst.clone(), Arc::new(flatSmToDataFlow), Some(componentRef.clone()), Some(inEnclosingFlatSmSemantics.clone()), outElems.clone());
    Ok(outElems)
}

fn addStateActivationAndReset(mut inEqn: Arc<DAE::Element>, mut inEnclosingSMComp: Arc<DAE::Element>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (HashTableCrToExpOption::FuncHashCref, HashTableCrToExpOption::FuncCrefEqual, HashTableCrToExpOption::FuncCrefStr, HashTableCrToExpOption::FuncExpStr)), mut accEqnsVars: (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outEqnsVars: (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>);
    let mut equations1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut vars1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut condition: Arc<DAE::Exp>;
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource>;
    outEqnsVars = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ DAE::Element::EQUATION { .. } => addStateActivationAndReset1(inEqn.clone(), inEnclosingSMComp.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt.clone(), accEqnsVars.clone())?,
        Deref @ DAE::Element::WHEN_EQUATION { condition, equations, elsewhen_: None, source } => {
            (equations1, vars1) = List::fold3(equations.clone(), Arc::new(addStateActivationAndReset), inEnclosingSMComp.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt.clone(), (metamodelica::nil(), metamodelica::nil()));
            (cons(Arc::new(DAE::Element::WHEN_EQUATION { condition: condition.clone(), equations: equations1.clone(), elsewhen_: None, source: source.clone() }), Util::tuple21(accEqnsVars.clone())), listAppend(vars1.clone(), Util::tuple22(accEqnsVars.clone())))
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError((literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n")).clone())?;
            bail!("fail")
        },
        _ => {
            Error::addCompilerError((literal!("Internal compiler error: StateMachineFlatten.addStateActivationAndReset(..) called with unexpected argument.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqnsVars)
}

fn addStateActivationAndReset1(mut inEqn: Arc<DAE::Element>, mut inEnclosingSMComp: Arc<DAE::Element>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (HashTableCrToExpOption::FuncHashCref, HashTableCrToExpOption::FuncCrefEqual, HashTableCrToExpOption::FuncCrefStr, HashTableCrToExpOption::FuncExpStr)), mut accEqnsVars: (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outEqnsVars: (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>);
    let mut stateVarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crefLHS: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut enclosingStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut substituteRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activeResetRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activeResetStatesRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cref2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut found: bool = false;
    let mut is: bool = false;
    let mut tyLHS: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut eqn: Arc<DAE::Element>;
    let mut eqn1: Arc<DAE::Element>;
    let mut eqn2: Arc<DAE::Element>;
    let mut var2: Arc<DAE::Element>;
    let mut varDecl: Arc<DAE::Element>;
    let mut attr: Arc<DAE::CallAttributes>;
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut isOuterVar: bool = false;
    let mut exp: Arc<DAE::Exp>;
    let mut scalar: Arc<DAE::Exp>;
    let mut scalarNew: Arc<DAE::Exp>;
    let mut source: Arc<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    scalar = __pa1.clone();
    source = __pa2.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inEnclosingSMComp.clone()) {
        Deref @ DAE::Element::SM_COMP { dAElist: __pa3, componentRef: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dAElist = __pa3.clone();
    enclosingStateRef = __pa4.clone();
    stateVarCrefs = BaseHashTable::hashTableKeyList(crToExpOpt.clone());
    match '__try5: {
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { ty: __pa6, componentRef: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => break '__try5 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        tyLHS = __pa6.clone();
        crefLHS = __pa7.clone();
        let (__pa8, (_, __pa9)) = unwrap_break_err!(Expression::traverseExpTopDown(scalar.clone(), Arc::new(traversingSubsPreviousCrefs), (stateVarCrefs.clone(), false)), '__try5);
        scalarNew = __pa8.clone();
        found = __pa9.clone();
        eqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: scalarNew.clone(), source: source.clone() });
        if List::any(stateVarCrefs.clone(), Arc::new({ let __pe_b0 = crefLHS.clone(); move |__pe_a1| ComponentReferenceBasics::crefEqual(__pe_b0.clone(), __pe_a1) })) {
            eqn1 = unwrap_break_err!(wrapInStateActivationConditional(eqn.clone(), enclosingStateRef.clone(), true), '__try5);
            var2 = createVarWithDefaults(ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), crefLHS.clone())?, openmodelica_frontend_types::DAE::VarKind::DISCRETE, tyLHS.clone(), metamodelica::nil());
            eqn2 = unwrap_break_err!(createResetEquation(crefLHS.clone(), tyLHS.clone(), enclosingStateRef.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt.clone()), '__try5);
            outEqnsVars = (cons(eqn1.clone(), cons(eqn2.clone(), Util::tuple21(accEqnsVars.clone()))), cons(var2.clone(), Util::tuple22(accEqnsVars.clone())));
        } else {
            outEqnsVars = (cons(unwrap_break_err!(wrapInStateActivationConditional(eqn.clone(), enclosingStateRef.clone(), false), '__try5), Util::tuple21(accEqnsVars.clone())), Util::tuple22(accEqnsVars.clone()));
        }
        Ok::<_, anyhow::Error>((crefLHS.clone(), outEqnsVars.clone(), tyLHS.clone()))
    } {
        Ok((__try5_o0, __try5_o1, __try5_o2)) => {
            crefLHS = __try5_o0;
            outEqnsVars = __try5_o1;
            tyLHS = __try5_o2;
        }
        Err(_) => {
            match '__try10: {
                if unwrap_break_err!(Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone()), '__try10) {
                    let (__pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(exp.clone()) {
                        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: __pa11, componentRef: __pa12 }, tail: Deref @ metamodelica::List::Nil }, attr: __pa13 } => (__pa11.clone(), __pa12.clone(), __pa13.clone()),
                        _ => break '__try10 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    tyLHS = __pa11.clone();
                    crefLHS = __pa12.clone();
                    attr = __pa13.clone();
                    if let Ok(__iflet16) = List::find1(dAElist.clone(), Arc::new(isCrefInVar), crefLHS.clone()) {
                        varDecl = __iflet16;
                    } else {
                        unwrap_break_err!(Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Couldn't find variable declaration matching to cref ")); __mm_s.push_str(&*ComponentReference::crefStr(crefLHS.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()), '__try10);
                        break '__try10 Err::<_, _>(anyhow::anyhow!("fail"));
                    }
                    isOuterVar = DAEUtil::isOuterVar(varDecl.clone());
                    if isOuterVar.clone() {
                        cref2 = unwrap_break_err!(ComponentReference::appendStringLastIdent((literal!("_der$")).clone(), crefLHS.clone()), '__try10);
                        var2 = createVarWithDefaults(cref2.clone(), openmodelica_frontend_types::DAE::VarKind::VARIABLE, tyLHS.clone(), metamodelica::nil());
                        eqn1 = Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cref2.clone(), ty: tyLHS.clone() }), scalar: scalar.clone(), source: source.clone() });
                        outEqnsVars = (cons(eqn1.clone(), Util::tuple21(accEqnsVars.clone())), cons(var2.clone(), Util::tuple22(accEqnsVars.clone())));
                    } else {
                        eqn1 = unwrap_break_err!(wrapInStateActivationConditionalCT(inEqn.clone(), enclosingStateRef.clone()), '__try10);
                        eqn2 = unwrap_break_err!(createResetEquationCT(crefLHS.clone(), tyLHS.clone(), enclosingStateRef.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt.clone()), '__try10);
                        outEqnsVars = (cons(eqn1.clone(), cons(eqn2.clone(), Util::tuple21(accEqnsVars.clone()))), Util::tuple22(accEqnsVars.clone()));
                    }
                } else {
                    break '__try10 Err::<_, _>(anyhow::anyhow!("fail"));
                }
                Ok::<_, anyhow::Error>((attr.clone(), crefLHS.clone(), eqn1.clone(), isOuterVar.clone(), outEqnsVars.clone(), tyLHS.clone(), varDecl.clone()))
            } {
                Ok((__try10_o0, __try10_o1, __try10_o2, __try10_o3, __try10_o4, __try10_o5, __try10_o6)) => {
                    attr = __try10_o0;
                    crefLHS = __try10_o1;
                    eqn1 = __try10_o2;
                    isOuterVar = __try10_o3;
                    outEqnsVars = __try10_o4;
                    tyLHS = __try10_o5;
                    varDecl = __try10_o6;
                }
                Err(_) => {
                    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
                        Error::addCompilerError((literal!("Currently, only equations in state machines with a LHS component reference, e.g., x=.., or its derivative, e.g., der(x)=.., are supported")).clone())?;
                    } else {
                        Error::addCompilerError((literal!("Currently, only equations in state machines with a LHS component reference, e.g., x=.., are supported")).clone())?;
                    }
                    bail!("fail");
                }
            }
        }
    }
    Ok(outEqnsVars)
}

fn isVarAtLHS(mut eqn: Arc<DAE::Element>, mut var: Arc<DAE::Element>) -> Result<bool> {
    let mut res: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crefLHS: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tyLHS: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut exp: Arc<DAE::Exp>;
    let mut scalar: Arc<DAE::Exp>;
    let mut scalarNew: Arc<DAE::Exp>;
    let mut source: Arc<DAE::ElementSource>;
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elsewhen_: Option<Arc<DAE::Element>> = None;
    res = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ DAE::Element::EQUATION { exp, scalar, source } => {
            cref = DAEUtil::varCref(var.clone())?;
            match '__try0: {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
                    Deref @ DAE::Exp::CREF { ty: __pa1, componentRef: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                tyLHS = __pa1.clone();
                crefLHS = __pa2.clone();
                res = unwrap_break_err!(ComponentReferenceBasics::crefEqual(crefLHS.clone(), cref.clone()), '__try0);
                Ok::<_, anyhow::Error>((res.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    res = __try0_o0;
                }
                Err(_) => {
                    res = false;
                }
            }
            res.clone()
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: None, equations, .. } => List::exist1(equations.clone(), Arc::new(isVarAtLHS), var.clone()),
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError((literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n")).clone())?;
            bail!("fail")
        },
        _ => {
            Error::addCompilerError((literal!("Internal compiler error: StateMachineFlatten.isVarAtLHS(..) called with unexpected argument.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isPreviousAppliedToVar(mut eqn: Arc<DAE::Element>, mut var: Arc<DAE::Element>) -> Result<bool> {
    let mut found: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut exp: Arc<DAE::Exp>;
    let mut scalar: Arc<DAE::Exp>;
    let mut scalarNew: Arc<DAE::Exp>;
    let mut source: Arc<DAE::ElementSource>;
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elsewhen_: Option<Arc<DAE::Element>> = None;
    found = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ DAE::Element::EQUATION { exp, scalar, source } => {
            cref = DAEUtil::varCref(var.clone())?;
            let (_, (_, __pa0)) = Expression::traverseExpTopDown(scalar.clone(), Arc::new(traversingFindPreviousCref), (cref.clone(), false))?;
            found = __pa0.clone();
            found.clone()
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: None, equations, .. } => List::exist1(equations.clone(), Arc::new(isPreviousAppliedToVar), var.clone()),
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError((literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n")).clone())?;
            bail!("fail")
        },
        _ => {
            Error::addCompilerError((literal!("Internal compiler error: StateMachineFlatten.isPreviousAppliedToVar(..) called with unexpected argument.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(found)
}

fn traversingFindPreviousCref(mut inExp: Arc<DAE::Exp>, mut inCrefHit: (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefHit: (Arc<DAE::ComponentRef>, bool);
    (outExp, outCrefHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, tail: Deref @ metamodelica::List::Nil }, attr: _ }, (cref, _)) if (ComponentReferenceBasics::crefEqual(cr.clone(), cref.clone())?) => {
            (inExp.clone(), (cref.clone(), true))
        },
        _ => {
            (inExp.clone(), inCrefHit.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefHit))
}

fn createResetEquationCT(mut inLHSCref: Arc<DAE::ComponentRef>, mut inLHSty: Arc<DAE::Type>, mut inStateCref: Arc<DAE::ComponentRef>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (HashTableCrToExpOption::FuncHashCref, HashTableCrToExpOption::FuncCrefEqual, HashTableCrToExpOption::FuncCrefStr, HashTableCrToExpOption::FuncExpStr))) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut activeExp: Arc<DAE::Exp>;
    let mut activeResetExp: Arc<DAE::Exp>;
    let mut activeResetStatesExp: Arc<DAE::Exp>;
    let mut orExp: Arc<DAE::Exp>;
    let mut andExp: Arc<DAE::Exp>;
    let mut startValueExp: Arc<DAE::Exp>;
    let mut preExp: Arc<DAE::Exp>;
    let mut reinitElem: Arc<DAE::Element>;
    let mut startValueOpt: Option<Arc<DAE::Exp>> = None;
    let mut initStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut preRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut i: i32 = 0;
    let mut nStates: i32 = 0;
    let mut enclosingFlatSMComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut tArrayBool: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut callAttributes: Arc<DAE::CallAttributes>;
    let FlatSmSemantics { smComps: __pa0, .. } = (inEnclosingFlatSmSemantics.clone()) else { bail!("pattern mismatch") };
    enclosingFlatSMComps = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(enclosingFlatSMComps.clone().borrow()[(1-1) as usize].clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    initStateRef = __pa1.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), initStateRef.clone());
    i = List::position1OnTrue(Arc::new(enclosingFlatSMComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), Arc::new(sMCompEqualsRef), inStateCref.clone());
    activeResetExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("activeReset")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    nStates = (enclosingFlatSMComps.clone().borrow().len() as i32);
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates.clone() })] });
    activeResetStatesExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("activeResetStates")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    orExp = Arc::new(DAE::Exp::LBINARY { exp1: activeResetExp.clone(), operator: DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: activeResetStatesExp.clone() });
    activeExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), inStateCref.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    andExp = Arc::new(DAE::Exp::LBINARY { exp1: activeExp.clone(), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: orExp.clone() });
    startValueOpt = BaseHashTable::get(inLHSCref.clone(), crToExpOpt.clone())?;
    if isSome(startValueOpt.clone()) {
        startValueExp = Util::getOption(startValueOpt.clone())?;
    } else {
        startValueExp = (::match_deref::match_deref! { match &(inLHSty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::ICONST { integer: 0 })
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=false.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() })
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n")); ArcStr::from(__mm_s) }).clone())?;
            Types::getNthEnumLiteral(inLHSty.clone(), 1)?
        },
        _ => {
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value.\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    reinitElem = Arc::new(DAE::Element::REINIT { componentRef: inLHSCref.clone(), exp: startValueExp.clone(), source: DAE::emptyElementSource.clone() });
    outEqn = Arc::new(DAE::Element::WHEN_EQUATION { condition: andExp.clone(), equations: list![reinitElem.clone()], elsewhen_: None, source: DAE::emptyElementSource.clone() });
    Ok(outEqn)
}

fn isCrefInVar(mut inElement: Arc<DAE::Element>, mut inCref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { componentRef: cref, .. } if (ComponentReferenceBasics::crefEqual(cref.clone(), inCref.clone())?) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn createResetEquation(mut inLHSCref: Arc<DAE::ComponentRef>, mut inLHSty: Arc<DAE::Type>, mut inStateCref: Arc<DAE::ComponentRef>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (HashTableCrToExpOption::FuncHashCref, HashTableCrToExpOption::FuncCrefEqual, HashTableCrToExpOption::FuncCrefStr, HashTableCrToExpOption::FuncExpStr))) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut activeExp: Arc<DAE::Exp>;
    let mut lhsExp: Arc<DAE::Exp>;
    let mut activeResetExp: Arc<DAE::Exp>;
    let mut activeResetStatesExp: Arc<DAE::Exp>;
    let mut orExp: Arc<DAE::Exp>;
    let mut andExp: Arc<DAE::Exp>;
    let mut previousExp: Arc<DAE::Exp>;
    let mut startValueExp: Arc<DAE::Exp>;
    let mut ifExp: Arc<DAE::Exp>;
    let mut startValueOpt: Option<Arc<DAE::Exp>> = None;
    let mut initStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut preRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut i: i32 = 0;
    let mut nStates: i32 = 0;
    let mut enclosingFlatSMComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut tArrayBool: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut callAttributes: Arc<DAE::CallAttributes>;
    let FlatSmSemantics { smComps: __pa0, .. } = (inEnclosingFlatSmSemantics.clone()) else { bail!("pattern mismatch") };
    enclosingFlatSMComps = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(enclosingFlatSMComps.clone().borrow()[(1-1) as usize].clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    initStateRef = __pa1.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), initStateRef.clone());
    i = List::position1OnTrue(Arc::new(enclosingFlatSMComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), Arc::new(sMCompEqualsRef), inStateCref.clone());
    activeResetExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("activeReset")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    nStates = (enclosingFlatSMComps.clone().borrow().len() as i32);
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates.clone() })] });
    activeResetStatesExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("activeResetStates")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    orExp = Arc::new(DAE::Exp::LBINARY { exp1: activeResetExp.clone(), operator: DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: activeResetStatesExp.clone() });
    activeExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), inStateCref.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    andExp = Arc::new(DAE::Exp::LBINARY { exp1: activeExp.clone(), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: orExp.clone() });
    callAttributes = Arc::new(DAE::CallAttributes { ty: inLHSty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    previousExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: inLHSCref.clone(), ty: inLHSty.clone() })], attr: callAttributes.clone() });
    startValueOpt = BaseHashTable::get(inLHSCref.clone(), crToExpOpt.clone())?;
    if isSome(startValueOpt.clone()) {
        startValueExp = Util::getOption(startValueOpt.clone())?;
    } else {
        startValueExp = (::match_deref::match_deref! { match &(inLHSty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::ICONST { integer: 0 })
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=false.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() })
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n")); ArcStr::from(__mm_s) }).clone())?;
            Types::getNthEnumLiteral(inLHSty.clone(), 1)?
        },
        _ => {
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value.\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    ifExp = Arc::new(DAE::Exp::IFEXP { expCond: andExp.clone(), expThen: startValueExp.clone(), expElse: previousExp.clone() });
    lhsExp = Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), inLHSCref.clone())?, ty: inLHSty.clone() });
    outEqn = Arc::new(DAE::Element::EQUATION { exp: lhsExp.clone(), scalar: ifExp.clone(), source: DAE::emptyElementSource.clone() });
    Ok(outEqn)
}

fn wrapInStateActivationConditional(mut inEqn: Arc<DAE::Element>, mut inStateCref: Arc<DAE::ComponentRef>, mut isResetEquation: bool) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut exp: Arc<DAE::Exp>;
    let mut scalar: Arc<DAE::Exp>;
    let mut scalar1: Arc<DAE::Exp>;
    let mut activeRef: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut callAttributes: Arc<DAE::CallAttributes>;
    let mut source: Arc<DAE::ElementSource>;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    scalar = __pa1.clone();
    source = __pa2.clone();
    match '__try3: {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        cref = __pa4.clone();
        ty = __pa5.clone();
        Ok::<_, anyhow::Error>((cref.clone(), ty.clone()))
    } {
        Ok((__try3_o0, __try3_o1)) => {
            cref = __try3_o0;
            ty = __try3_o1;
        }
        Err(_) => {
            Error::addCompilerError((literal!("The LHS of equations in state machines needs to be a component reference")).clone())?;
            bail!("fail");
        }
    }
    activeRef = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), inStateCref.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    callAttributes = Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    if isResetEquation.clone() {
        expElse = Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), cref.clone())?, ty: ty.clone() });
    } else {
        expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![exp.clone()], attr: callAttributes.clone() });
    }
    scalar1 = Arc::new(DAE::Exp::IFEXP { expCond: activeRef.clone(), expThen: scalar.clone(), expElse: expElse.clone() });
    outEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: scalar1.clone(), source: source.clone() });
    Ok(outEqn)
}

fn wrapInStateActivationConditionalCT(mut inEqn: Arc<DAE::Element>, mut inStateCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut exp: Arc<DAE::Exp>;
    let mut scalar: Arc<DAE::Exp>;
    let mut scalar1: Arc<DAE::Exp>;
    let mut activeRef: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut callAttributes: Arc<DAE::CallAttributes>;
    let mut source: Arc<DAE::ElementSource>;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    scalar = __pa1.clone();
    source = __pa2.clone();
    match '__try3: {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: __pa4, componentRef: __pa5 }, tail: Deref @ metamodelica::List::Nil }, attr: _ } => (__pa4.clone(), __pa5.clone()),
            _ => break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        ty = __pa4.clone();
        cref = __pa5.clone();
        Ok::<_, anyhow::Error>((cref.clone(), ty.clone()))
    } {
        Ok((__try3_o0, __try3_o1)) => {
            cref = __try3_o0;
            ty = __try3_o1;
        }
        Err(_) => {
            Error::addCompilerError((literal!("The LHS of equations in state machines needs to be a component reference, e.g., x = .., or its derivative, e.g., der(x) = ..")).clone())?;
            bail!("fail");
        }
    }
    activeRef = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), inStateCref.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    callAttributes = Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    expElse = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) });
    scalar1 = Arc::new(DAE::Exp::IFEXP { expCond: activeRef.clone(), expThen: scalar.clone(), expElse: expElse.clone() });
    outEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: scalar1.clone(), source: source.clone() });
    Ok(outEqn)
}

fn traversingSubsPreviousCref(mut inExp: Arc<DAE::Exp>, mut inCrefHit: (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefHit: (Arc<DAE::ComponentRef>, bool);
    (outExp, outCrefHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::List::Nil }, attr: _ }, (cref, _)) if (ComponentReferenceBasics::crefEqual(cr.clone(), cref.clone())?) => {
            let mut substituteRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("StateMachineFlatten.traversingSubsPreviousCref: cr: ")); __mm_s.push_str(&*ComponentReference::crefStr(cr.clone())?); __mm_s.push_str(&*literal!(", cref: ")); __mm_s.push_str(&*ComponentReference::crefStr(cref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            substituteRef = ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), cref.clone())?;
            (Arc::new(DAE::Exp::CREF { componentRef: substituteRef.clone(), ty: ty.clone() }), (cref.clone(), true))
        },
        _ => {
            (inExp.clone(), inCrefHit.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefHit))
}

fn traversingSubsPreviousCrefs(mut inExp: Arc<DAE::Exp>, mut inCrefsHit: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefsHit: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool);
    (outExp, outCrefsHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefsHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::List::Nil }, attr: _ }, (crefs, _)) if (List::any(crefs.clone(), Arc::new({ let __pe_b0 = cr.clone(); move |__pe_a1| ComponentReferenceBasics::crefEqual(__pe_b0.clone(), __pe_a1) }))) => {
            let mut substituteRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            substituteRef = ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), cr.clone())?;
            (Arc::new(DAE::Exp::CREF { componentRef: substituteRef.clone(), ty: ty.clone() }), (crefs.clone(), true))
        },
        _ => {
            (inExp.clone(), inCrefsHit.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefsHit))
}

fn getStartAttrOption(mut inElt: Arc<DAE::Element>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExpOpt: Option<Arc<DAE::Exp>> = None;
    let mut start: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut varAttrOpt: Option<Arc<DAE::VariableAttributes>> = None;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inElt.clone()) {
        Deref @ DAE::Element::VAR { ty: __pa0, variableAttributesOption: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    varAttrOpt = __pa1.clone();
    if isSome(varAttrOpt.clone()) {
        start = DAEUtil::getStartAttr(varAttrOpt.clone(), ty.clone())?;
        outExpOpt = Some(start.clone());
    } else {
        outExpOpt = None;
    }
    Ok(outExpOpt)
}

fn addPropagationEquations(mut inFlatSmSemantics: FlatSmSemantics, mut inEnclosingStateCrefOption: Option<Arc<DAE::ComponentRef>>, mut inEnclosingFlatSmSemanticsOption: Option<FlatSmSemantics>) -> Result<FlatSmSemantics> {
    let mut outFlatSmSemantics: FlatSmSemantics;
    let mut preRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut initStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut initRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut resetRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activeRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut stateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activePlotIndicatorRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut initVar: Arc<DAE::Element>;
    let mut activePlotIndicatorVar: Arc<DAE::Element>;
    let mut ticksInStateVar: Arc<DAE::Element>;
    let mut timeEnteredStateVar: Arc<DAE::Element>;
    let mut timeInStateVar: Arc<DAE::Element>;
    let mut activePlotIndicatorEqn: Arc<DAE::Element>;
    let mut ticksInStateEqn: Arc<DAE::Element>;
    let mut timeEnteredStateEqn: Arc<DAE::Element>;
    let mut timeInStateEqn: Arc<DAE::Element>;
    let mut rhs: Arc<DAE::Exp>;
    let mut andExp: Arc<DAE::Exp>;
    let mut eqExp: Arc<DAE::Exp>;
    let mut activeResetStateRefExp: Arc<DAE::Exp>;
    let mut activeStateRefExp: Arc<DAE::Exp>;
    let mut activeResetRefExp: Arc<DAE::Exp>;
    let mut tArrayBool: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tArrayInteger: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ident: ArcStr = arcstr::literal!("");
    let mut smComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut t: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut c: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut smvars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smknowns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smeqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut enclosingStateOption: Option<Arc<DAE::ComponentRef>> = None;
    let mut pvars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut peqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut enclosingStateCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut enclosingPreRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut enclosingActiveResetStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut enclosingActiveResetRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut enclosingActiveStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut enclosingFlatSMSemantics: FlatSmSemantics;
    let mut enclosingFlatSMComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut enclosingFlatSMInitStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut posOfEnclosingSMComp: i32 = 0;
    let mut nStates: i32 = 0;
    let FlatSmSemantics { eqs: __pa0, knowns: __pa1, vars: __pa2, c: __pa3, t: __pa4, smComps: __pa5, ident: __pa6, .. } = (inFlatSmSemantics.clone()) else { bail!("pattern mismatch") };
    smeqs = __pa0.clone();
    smknowns = __pa1.clone();
    smvars = __pa2.clone();
    c = __pa3.clone();
    t = __pa4.clone();
    smComps = __pa5.clone();
    ident = __pa6.clone();
    let __pa7 = ::match_deref::match_deref! { match &(smComps.clone().borrow()[(1-1) as usize].clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa7, .. } => __pa7.clone(),
        _ => bail!("pattern mismatch"),
    } };
    initStateRef = __pa7.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), initStateRef.clone());
    activeRef = qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    resetRef = qCref((literal!("reset")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    if isNone(inEnclosingFlatSmSemanticsOption.clone()) {
        initRef = qCref((literal!("init")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
        initVar = createVarWithDefaults(initRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil());
        initVar = setVarFixedStartValue(initVar.clone(), Arc::new(DAE::Exp::BCONST { bool: true }))?;
        pvars = cons(initVar.clone(), pvars.clone());
        peqs = cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: initRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), scalar: Arc::new(DAE::Exp::BCONST { bool: false }), source: DAE::emptyElementSource.clone() }), peqs.clone());
        rhs = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: initRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() })], attr: DAE::callAttrBuiltinImpureBool.clone() });
        peqs = cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() }), peqs.clone());
        peqs = cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), scalar: Arc::new(DAE::Exp::BCONST { bool: true }), source: DAE::emptyElementSource.clone() }), peqs.clone());
    } else {
        enclosingStateCref = Util::getOption(inEnclosingStateCrefOption.clone())?;
        enclosingFlatSMSemantics = Util::getOption(inEnclosingFlatSmSemanticsOption.clone())?;
        let FlatSmSemantics { smComps: __pa8, .. } = (enclosingFlatSMSemantics.clone()) else { bail!("pattern mismatch") };
        enclosingFlatSMComps = __pa8.clone();
        let __pa9 = ::match_deref::match_deref! { match &(enclosingFlatSMComps.clone().borrow()[(1-1) as usize].clone()) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa9, .. } => __pa9.clone(),
            _ => bail!("pattern mismatch"),
        } };
        enclosingFlatSMInitStateRef = __pa9.clone();
        enclosingPreRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), enclosingFlatSMInitStateRef.clone());
        posOfEnclosingSMComp = List::position1OnTrue(Arc::new(enclosingFlatSMComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), Arc::new(sMCompEqualsRef), enclosingStateCref.clone());
        nStates = (enclosingFlatSMComps.clone().borrow().len() as i32);
        tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates.clone() })] });
        tArrayInteger = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates.clone() })] });
        enclosingActiveResetStateRef = qCref((literal!("activeResetStates")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: posOfEnclosingSMComp.clone() }) })], enclosingPreRef.clone())?;
        enclosingActiveResetRef = qCref((literal!("activeReset")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), enclosingPreRef.clone())?;
        enclosingActiveStateRef = qCref((literal!("activeState")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), enclosingPreRef.clone())?;
        eqExp = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: enclosingActiveStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: posOfEnclosingSMComp.clone() }), index: -1, optionExpisASUB: None });
        andExp = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: enclosingActiveResetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: eqExp.clone() });
        rhs = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: enclosingActiveResetStateRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), operator: DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: andExp.clone() });
        peqs = cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() }), peqs.clone());
        rhs = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: enclosingActiveStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: posOfEnclosingSMComp.clone() }), index: -1, optionExpisASUB: None });
        peqs = cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() }), peqs.clone());
    }
    let __range10 = 1..=(smComps.clone().borrow().len() as i32);
    for mut i in __range10 {
        let __pa11 = ::match_deref::match_deref! { match &(smComps.clone().borrow()[(i.clone()-1) as usize].clone()) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa11, .. } => __pa11.clone(),
            _ => bail!("pattern mismatch"),
        } };
        stateRef = __pa11.clone();
        (activePlotIndicatorVar, activePlotIndicatorEqn) = createActiveIndicator(stateRef.clone(), preRef.clone(), i.clone())?;
        pvars = cons(activePlotIndicatorVar.clone(), pvars.clone());
        peqs = cons(activePlotIndicatorEqn.clone(), peqs.clone());
        let __pa12 = ::match_deref::match_deref! { match &(activePlotIndicatorVar.clone()) {
            Deref @ DAE::Element::VAR { componentRef: __pa12, .. } => __pa12.clone(),
            _ => bail!("pattern mismatch"),
        } };
        activePlotIndicatorRef = __pa12.clone();
        (ticksInStateVar, ticksInStateEqn) = createTicksInStateIndicator(stateRef.clone(), activePlotIndicatorRef.clone())?;
        pvars = cons(ticksInStateVar.clone(), pvars.clone());
        peqs = cons(ticksInStateEqn.clone(), peqs.clone());
        (timeEnteredStateVar, timeEnteredStateEqn) = createTimeEnteredStateIndicator(stateRef.clone(), activePlotIndicatorRef.clone())?;
        (timeInStateVar, timeInStateEqn) = createTimeInStateIndicator(stateRef.clone(), activePlotIndicatorRef.clone(), timeEnteredStateVar.clone())?;
        pvars = cons(timeEnteredStateVar.clone(), cons(timeInStateVar.clone(), pvars.clone()));
        peqs = cons(timeEnteredStateEqn.clone(), cons(timeInStateEqn.clone(), peqs.clone()));
    }
    outFlatSmSemantics = FlatSmSemantics { ident: (ident.clone()).clone(), smComps: smComps.clone(), t: t.clone(), c: c.clone(), vars: smvars.clone(), knowns: smknowns.clone(), eqs: smeqs.clone(), pvars: pvars.clone(), peqs: peqs.clone(), enclosingState: inEnclosingStateCrefOption.clone() };
    Ok(outFlatSmSemantics)
}

fn createTimeInStateIndicator(mut stateRef: Arc<DAE::ComponentRef>, mut stateActiveRef: Arc<DAE::ComponentRef>, mut timeEnteredStateVar: Arc<DAE::Element>) -> Result<(Arc<DAE::Element>, Arc<DAE::Element>)> {
    let mut timeInStateVar: Arc<DAE::Element>;
    let mut timeInStateEqn: Arc<DAE::Element>;
    let mut timeInStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut timeEnteredStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut timeInStateExp: Arc<DAE::Exp>;
    let mut timeEnteredStateExp: Arc<DAE::Exp>;
    let mut stateActiveExp: Arc<DAE::Exp>;
    let mut expCond: Arc<DAE::Exp>;
    let mut expSampleTime: Arc<DAE::Exp>;
    let mut expThen: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    timeInStateRef = qCref((literal!("$timeInState")).clone(), DAE::T_REAL_DEFAULT.clone(), metamodelica::nil(), stateRef.clone())?;
    timeInStateVar = createVarWithDefaults(timeInStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_REAL_DEFAULT.clone(), metamodelica::nil());
    timeInStateVar = setVarFixedStartValue(timeInStateVar.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) }))?;
    timeInStateExp = Arc::new(DAE::Exp::CREF { componentRef: timeInStateRef.clone(), ty: DAE::T_REAL_DEFAULT.clone() });
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(timeEnteredStateVar.clone()) {
        Deref @ DAE::Element::VAR { ty: __pa0, componentRef: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    timeEnteredStateRef = __pa1.clone();
    timeEnteredStateExp = Arc::new(DAE::Exp::CREF { componentRef: timeEnteredStateRef.clone(), ty: ty.clone() });
    stateActiveExp = Expression::crefExp(stateActiveRef.clone())?;
    expCond = Expression::crefExp(stateActiveRef.clone())?;
    expSampleTime = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("time")).clone(), identType: DAE::T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT.clone() }), Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK) })], attr: DAE::callAttrBuiltinImpureReal.clone() });
    expThen = Arc::new(DAE::Exp::BINARY { exp1: expSampleTime.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: timeEnteredStateExp.clone() });
    expElse = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) });
    timeInStateEqn = Arc::new(DAE::Element::EQUATION { exp: timeInStateExp.clone(), scalar: Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() }), source: DAE::emptyElementSource.clone() });
    Ok((timeInStateVar, timeInStateEqn))
}

fn createTimeEnteredStateIndicator(mut stateRef: Arc<DAE::ComponentRef>, mut stateActiveRef: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::Element>, Arc<DAE::Element>)> {
    let mut timeEnteredStateVar: Arc<DAE::Element>;
    let mut timeEnteredStateEqn: Arc<DAE::Element>;
    let mut timeEnteredStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut timeEnteredStateExp: Arc<DAE::Exp>;
    let mut stateActiveExp: Arc<DAE::Exp>;
    let mut expCond: Arc<DAE::Exp>;
    let mut expThen: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    timeEnteredStateRef = qCref((literal!("$timeEnteredState")).clone(), DAE::T_REAL_DEFAULT.clone(), metamodelica::nil(), stateRef.clone())?;
    timeEnteredStateVar = createVarWithDefaults(timeEnteredStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_REAL_DEFAULT.clone(), metamodelica::nil());
    timeEnteredStateVar = setVarFixedStartValue(timeEnteredStateVar.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) }))?;
    timeEnteredStateExp = Arc::new(DAE::Exp::CREF { componentRef: timeEnteredStateRef.clone(), ty: DAE::T_REAL_DEFAULT.clone() });
    stateActiveExp = Expression::crefExp(stateActiveRef.clone())?;
    expCond = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![stateActiveExp.clone()], attr: DAE::callAttrBuiltinImpureBool.clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::BCONST { bool: false }), index: -1, optionExpisASUB: None }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::RELATION { exp1: stateActiveExp.clone(), operator: DAE::Operator::EQUAL { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::BCONST { bool: true }), index: -1, optionExpisASUB: None }) });
    expThen = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("time")).clone(), identType: DAE::T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT.clone() }), Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK) })], attr: DAE::callAttrBuiltinImpureReal.clone() });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![timeEnteredStateExp.clone()], attr: DAE::callAttrBuiltinImpureReal.clone() });
    timeEnteredStateEqn = Arc::new(DAE::Element::EQUATION { exp: timeEnteredStateExp.clone(), scalar: Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() }), source: DAE::emptyElementSource.clone() });
    Ok((timeEnteredStateVar, timeEnteredStateEqn))
}

fn createTicksInStateIndicator(mut stateRef: Arc<DAE::ComponentRef>, mut stateActiveRef: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::Element>, Arc<DAE::Element>)> {
    let mut ticksInStateVar: Arc<DAE::Element>;
    let mut ticksInStateEqn: Arc<DAE::Element>;
    let mut ticksInStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ticksInStateExp: Arc<DAE::Exp>;
    let mut expCond: Arc<DAE::Exp>;
    let mut expThen: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    ticksInStateRef = qCref((literal!("$ticksInState")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), stateRef.clone())?;
    ticksInStateVar = createVarWithDefaults(ticksInStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil());
    ticksInStateVar = setVarFixedStartValue(ticksInStateVar.clone(), Arc::new(DAE::Exp::ICONST { integer: 0 }))?;
    ticksInStateExp = Arc::new(DAE::Exp::CREF { componentRef: ticksInStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() });
    expCond = Expression::crefExp(stateActiveRef.clone())?;
    expThen = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![ticksInStateExp.clone()], attr: DAE::callAttrBuiltinImpureInteger.clone() }), operator: DAE::Operator::ADD { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 1 }) });
    expElse = Arc::new(DAE::Exp::ICONST { integer: 0 });
    ticksInStateEqn = Arc::new(DAE::Element::EQUATION { exp: ticksInStateExp.clone(), scalar: Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() }), source: DAE::emptyElementSource.clone() });
    Ok((ticksInStateVar, ticksInStateEqn))
}

fn createActiveIndicator(mut stateRef: Arc<DAE::ComponentRef>, mut preRef: Arc<DAE::ComponentRef>, mut i: i32) -> Result<(Arc<DAE::Element>, Arc<DAE::Element>)> {
    let mut activePlotIndicatorVar: Arc<DAE::Element>;
    let mut eqn: Arc<DAE::Element>;
    let mut activeRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activePlotIndicatorRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activeStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut andExp: Arc<DAE::Exp>;
    let mut eqExp: Arc<DAE::Exp>;
    activePlotIndicatorRef = qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), stateRef.clone())?;
    activePlotIndicatorVar = createVarWithStartValue(activePlotIndicatorRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), Arc::new(DAE::Exp::BCONST { bool: false }), metamodelica::nil())?;
    activeRef = qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    activeStateRef = qCref((literal!("activeState")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    eqExp = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: activeStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: i.clone() }), index: -1, optionExpisASUB: None });
    andExp = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: eqExp.clone() });
    eqn = Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: activePlotIndicatorRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }), scalar: andExp.clone(), source: DAE::emptyElementSource.clone() });
    Ok((activePlotIndicatorVar, eqn))
}

fn setVarFixedStartValue(mut inVar: Arc<DAE::Element>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> {
    let mut outVar: Arc<DAE::Element>;
    let mut vao: Option<Arc<DAE::VariableAttributes>> = None;
    let __pa0 = ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Element::VAR { variableAttributesOption: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vao = __pa0.clone();
    vao = DAEUtil::setStartAttrOption(vao.clone(), Some(inExp.clone()))?;
    vao = DAEUtil::setFixedAttr(vao.clone(), Some(Arc::new(DAE::Exp::BCONST { bool: true })))?;
    outVar = DAEUtil::setVariableAttributes(inVar.clone(), vao.clone())?;
    Ok(outVar)
}

fn basicFlatSmSemantics(mut ident: ArcStr, mut q: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inTransitions: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<FlatSmSemantics> {
    let mut flatSmSemantics: FlatSmSemantics;
    let mut crefInitialState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut preRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut defaultIntVar: Arc<DAE::Element>;
    let mut defaultBoolVar: Arc<DAE::Element>;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut knowns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut preRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut nStatesRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activeRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut resetRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut selectedStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut selectedResetRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut firedRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activeStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut activeResetRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut nextStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut nextResetRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut stateMachineInFinalStateRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: Arc<DAE::Element>;
    let mut nStatesVar: Arc<DAE::Element>;
    let mut activeVar: Arc<DAE::Element>;
    let mut resetVar: Arc<DAE::Element>;
    let mut selectedStateVar: Arc<DAE::Element>;
    let mut selectedResetVar: Arc<DAE::Element>;
    let mut firedVar: Arc<DAE::Element>;
    let mut activeStateVar: Arc<DAE::Element>;
    let mut activeResetVar: Arc<DAE::Element>;
    let mut nextStateVar: Arc<DAE::Element>;
    let mut nextResetVar: Arc<DAE::Element>;
    let mut stateMachineInFinalStateVar: Arc<DAE::Element>;
    let mut nStates: i32 = 0;
    let mut nStatesDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut nStatesArrayBool: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut activeResetStatesRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut nextResetStatesRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut finalStatesRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut activeResetStatesVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut nextResetStatesVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut finalStatesVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut t: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut nTransitions: i32 = 0;
    let mut tDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut tArrayInteger: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tArrayBool: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tFromRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tToRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tImmediateRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tResetRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tSynchronizeRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tPriorityRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tFromVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tToVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tImmediateVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tResetVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tSynchronizeVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tPriorityVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut from: i32 = 0;
    let mut to: i32 = 0;
    let mut condition: Arc<DAE::Exp>;
    let mut immediate: bool = false;
    let mut reset: bool = false;
    let mut synchronize: bool = false;
    let mut priority: i32 = 0;
    let mut cExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut cRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut cImmediateRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut cVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut cImmediateVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut selectedStateEqn: Arc<DAE::Element>;
    let mut selectedResetEqn: Arc<DAE::Element>;
    let mut firedEqn: Arc<DAE::Element>;
    let mut activeStateEqn: Arc<DAE::Element>;
    let mut activeResetEqn: Arc<DAE::Element>;
    let mut nextStateEqn: Arc<DAE::Element>;
    let mut nextResetEqn: Arc<DAE::Element>;
    let mut exp: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut expCond: Arc<DAE::Exp>;
    let mut expThen: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let mut expIf: Arc<DAE::Exp>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut bindExp: Option<Arc<DAE::Exp>> = None;
    let __pa0 = ::match_deref::match_deref! { match &(listHead(q.clone())?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crefInitialState = __pa0.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), crefInitialState.clone());
    (t, cExps) = createTandC(q.clone(), inTransitions.clone())?;
    defaultIntVar = createVarWithDefaults(ComponentReference::makeDummyCref(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil());
    defaultBoolVar = createVarWithDefaults(ComponentReference::makeDummyCref(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil());
    knowns = metamodelica::nil();
    vars = metamodelica::nil();
    nStates = (q.clone().len() as i32);
    nStatesRef = qCref((literal!("nState")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    nStatesVar = createVarWithDefaults(nStatesRef.clone(), openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil());
    nStatesVar = DAEUtil::setElementVarBinding(nStatesVar.clone(), Some(Arc::new(DAE::Exp::ICONST { integer: nStates.clone() })));
    knowns = cons(nStatesVar.clone(), knowns.clone());
    nTransitions = (t.clone().len() as i32);
    tDims = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nTransitions.clone() })];
    tArrayInteger = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT.clone(), dims: tDims.clone() });
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT.clone(), dims: tDims.clone() });
    tFromRefs = arrayCreate(nTransitions.clone(), ComponentReference::makeDummyCref());
    tToRefs = arrayCreate(nTransitions.clone(), ComponentReference::makeDummyCref());
    tImmediateRefs = arrayCreate(nTransitions.clone(), ComponentReference::makeDummyCref());
    tResetRefs = arrayCreate(nTransitions.clone(), ComponentReference::makeDummyCref());
    tSynchronizeRefs = arrayCreate(nTransitions.clone(), ComponentReference::makeDummyCref());
    tPriorityRefs = arrayCreate(nTransitions.clone(), ComponentReference::makeDummyCref());
    tFromVars = arrayCreate(nTransitions.clone(), defaultIntVar.clone());
    tToVars = arrayCreate(nTransitions.clone(), defaultIntVar.clone());
    tImmediateVars = arrayCreate(nTransitions.clone(), defaultBoolVar.clone());
    tResetVars = arrayCreate(nTransitions.clone(), defaultBoolVar.clone());
    tSynchronizeVars = arrayCreate(nTransitions.clone(), defaultBoolVar.clone());
    tPriorityVars = arrayCreate(nTransitions.clone(), defaultIntVar.clone());
    i = 0;
    for mut t1 in &*t.clone() {
        let mut t1 = t1.clone();
        i = i.clone() + 1;
        let Transition { from: __pa1, to: __pa2, condition: _, immediate: __pa3, reset: __pa4, synchronize: __pa5, priority: __pa6 } = (t1.clone()) else { bail!("pattern mismatch") };
        from = __pa1.clone();
        to = __pa2.clone();
        immediate = __pa3.clone();
        reset = __pa4.clone();
        synchronize = __pa5.clone();
        priority = __pa6.clone();
        tFromRefs = {let _arr = tFromRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("tFrom")).clone(), tArrayInteger.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        tFromVars = {let _arr = tFromVars.clone(); let _val = createVarWithDefaults(tFromRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_INTEGER_DEFAULT.clone(), tDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        tFromVars = {let _arr = tFromVars.clone(); let _val = DAEUtil::setElementVarBinding(tFromVars.clone().borrow()[(i.clone()-1) as usize].clone(), Some(Arc::new(DAE::Exp::ICONST { integer: from.clone() }))); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        knowns = cons(tFromVars.clone().borrow()[(i.clone()-1) as usize].clone(), knowns.clone());
        tToRefs = {let _arr = tToRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("tTo")).clone(), tArrayInteger.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        tToVars = {let _arr = tToVars.clone(); let _val = createVarWithDefaults(tToRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_INTEGER_DEFAULT.clone(), tDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        tToVars = {let _arr = tToVars.clone(); let _val = DAEUtil::setElementVarBinding(tToVars.clone().borrow()[(i.clone()-1) as usize].clone(), Some(Arc::new(DAE::Exp::ICONST { integer: to.clone() }))); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        knowns = cons(tToVars.clone().borrow()[(i.clone()-1) as usize].clone(), knowns.clone());
        tImmediateRefs = {let _arr = tImmediateRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("tImmediate")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        tImmediateVars = {let _arr = tImmediateVars.clone(); let _val = createVarWithDefaults(tImmediateRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_BOOL_DEFAULT.clone(), tDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        tImmediateVars = {let _arr = tImmediateVars.clone(); let _val = DAEUtil::setElementVarBinding(tImmediateVars.clone().borrow()[(i.clone()-1) as usize].clone(), Some(Arc::new(DAE::Exp::BCONST { bool: immediate.clone() }))); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        knowns = cons(tImmediateVars.clone().borrow()[(i.clone()-1) as usize].clone(), knowns.clone());
        tResetRefs = {let _arr = tResetRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("tReset")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        tResetVars = {let _arr = tResetVars.clone(); let _val = createVarWithDefaults(tResetRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_BOOL_DEFAULT.clone(), tDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        tResetVars = {let _arr = tResetVars.clone(); let _val = DAEUtil::setElementVarBinding(tResetVars.clone().borrow()[(i.clone()-1) as usize].clone(), Some(Arc::new(DAE::Exp::BCONST { bool: reset.clone() }))); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        knowns = cons(tResetVars.clone().borrow()[(i.clone()-1) as usize].clone(), knowns.clone());
        tSynchronizeRefs = {let _arr = tSynchronizeRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("tSynchronize")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        tSynchronizeVars = {let _arr = tSynchronizeVars.clone(); let _val = createVarWithDefaults(tSynchronizeRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_BOOL_DEFAULT.clone(), tDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        tSynchronizeVars = {let _arr = tSynchronizeVars.clone(); let _val = DAEUtil::setElementVarBinding(tSynchronizeVars.clone().borrow()[(i.clone()-1) as usize].clone(), Some(Arc::new(DAE::Exp::BCONST { bool: synchronize.clone() }))); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        knowns = cons(tSynchronizeVars.clone().borrow()[(i.clone()-1) as usize].clone(), knowns.clone());
        tPriorityRefs = {let _arr = tPriorityRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("tPriority")).clone(), tArrayInteger.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        tPriorityVars = {let _arr = tPriorityVars.clone(); let _val = createVarWithDefaults(tPriorityRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_INTEGER_DEFAULT.clone(), tDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        tPriorityVars = {let _arr = tPriorityVars.clone(); let _val = DAEUtil::setElementVarBinding(tPriorityVars.clone().borrow()[(i.clone()-1) as usize].clone(), Some(Arc::new(DAE::Exp::ICONST { integer: priority.clone() }))); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        knowns = cons(tPriorityVars.clone().borrow()[(i.clone()-1) as usize].clone(), knowns.clone());
    }
    cRefs = arrayCreate(nTransitions.clone(), ComponentReference::makeDummyCref());
    cImmediateRefs = arrayCreate(nTransitions.clone(), ComponentReference::makeDummyCref());
    cVars = arrayCreate(nTransitions.clone(), defaultBoolVar.clone());
    cImmediateVars = arrayCreate(nTransitions.clone(), defaultBoolVar.clone());
    i = 0;
    for mut exp in &*cExps.clone() {
        let mut exp = exp.clone();
        i = i.clone() + 1;
        cRefs = {let _arr = cRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("c")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        cImmediateRefs = {let _arr = cImmediateRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("cImmediate")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        cVars = {let _arr = cVars.clone(); let _val = createVarWithDefaults(cRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), tDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        cImmediateVars = {let _arr = cImmediateVars.clone(); let _val = createVarWithStartValue(cImmediateRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), Arc::new(DAE::Exp::BCONST { bool: false }), tDims.clone())?; _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        vars = cons(cVars.clone().borrow()[(i.clone()-1) as usize].clone(), vars.clone());
        vars = cons(cImmediateVars.clone().borrow()[(i.clone()-1) as usize].clone(), vars.clone());
    }
    activeRef = qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    activeVar = createVarWithDefaults(activeRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil());
    vars = cons(activeVar.clone(), vars.clone());
    resetRef = qCref((literal!("reset")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    resetVar = createVarWithDefaults(resetRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil());
    vars = cons(resetVar.clone(), vars.clone());
    selectedStateRef = qCref((literal!("selectedState")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    selectedStateVar = createVarWithDefaults(selectedStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil());
    vars = cons(selectedStateVar.clone(), vars.clone());
    selectedResetRef = qCref((literal!("selectedReset")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    selectedResetVar = createVarWithDefaults(selectedResetRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil());
    vars = cons(selectedResetVar.clone(), vars.clone());
    firedRef = qCref((literal!("fired")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    firedVar = createVarWithDefaults(firedRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil());
    vars = cons(firedVar.clone(), vars.clone());
    activeStateRef = qCref((literal!("activeState")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    activeStateVar = createVarWithDefaults(activeStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil());
    vars = cons(activeStateVar.clone(), vars.clone());
    activeResetRef = qCref((literal!("activeReset")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    activeResetVar = createVarWithDefaults(activeResetRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil());
    vars = cons(activeResetVar.clone(), vars.clone());
    nextStateRef = qCref((literal!("nextState")).clone(), DAE::T_INTEGER_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    nextStateVar = createVarWithStartValue(nextStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT.clone(), Arc::new(DAE::Exp::ICONST { integer: 0 }), metamodelica::nil())?;
    vars = cons(nextStateVar.clone(), vars.clone());
    nextResetRef = qCref((literal!("nextReset")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    nextResetVar = createVarWithStartValue(nextResetRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), Arc::new(DAE::Exp::BCONST { bool: false }), metamodelica::nil())?;
    vars = cons(nextResetVar.clone(), vars.clone());
    nStatesDims = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates.clone() })];
    nStatesArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT.clone(), dims: nStatesDims.clone() });
    activeResetStatesRefs = arrayCreate(nStates.clone(), ComponentReference::makeDummyCref());
    activeResetStatesVars = arrayCreate(nStates.clone(), defaultBoolVar.clone());
    for mut i in 1..=nStates.clone() {
        activeResetStatesRefs = {let _arr = activeResetStatesRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("activeResetStates")).clone(), nStatesArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        activeResetStatesVars = {let _arr = activeResetStatesVars.clone(); let _val = createVarWithDefaults(activeResetStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), nStatesDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        vars = cons(activeResetStatesVars.clone().borrow()[(i.clone()-1) as usize].clone(), vars.clone());
    }
    nextResetStatesRefs = arrayCreate(nStates.clone(), ComponentReference::makeDummyCref());
    nextResetStatesVars = arrayCreate(nStates.clone(), defaultBoolVar.clone());
    for mut i in 1..=nStates.clone() {
        nextResetStatesRefs = {let _arr = nextResetStatesRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("nextResetStates")).clone(), nStatesArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        nextResetStatesVars = {let _arr = nextResetStatesVars.clone(); let _val = createVarWithStartValue(nextResetStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), Arc::new(DAE::Exp::BCONST { bool: false }), nStatesDims.clone())?; _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        vars = cons(nextResetStatesVars.clone().borrow()[(i.clone()-1) as usize].clone(), vars.clone());
    }
    finalStatesRefs = arrayCreate(nStates.clone(), ComponentReference::makeDummyCref());
    finalStatesVars = arrayCreate(nStates.clone(), defaultBoolVar.clone());
    for mut i in 1..=nStates.clone() {
        finalStatesRefs = {let _arr = finalStatesRefs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = qCref((literal!("finalStates")).clone(), nStatesArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })], preRef.clone())?; _arr};
        finalStatesVars = {let _arr = finalStatesVars.clone(); let _val = createVarWithDefaults(finalStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), nStatesDims.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        vars = cons(finalStatesVars.clone().borrow()[(i.clone()-1) as usize].clone(), vars.clone());
    }
    stateMachineInFinalStateRef = qCref((literal!("stateMachineInFinalState")).clone(), DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil(), preRef.clone())?;
    stateMachineInFinalStateVar = createVarWithDefaults(stateMachineInFinalStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT.clone(), metamodelica::nil());
    vars = cons(stateMachineInFinalStateVar.clone(), vars.clone());
    eqs = metamodelica::nil();
    i = 0;
    for mut cExp in &*cExps.clone() {
        let mut cExp = cExp.clone();
        i = i.clone() + 1;
        exp = Arc::new(DAE::Exp::CREF { componentRef: cImmediateRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
        eqs = cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: cExp.clone(), source: DAE::emptyElementSource.clone() }), eqs.clone());
        exp1 = Arc::new(DAE::Exp::CREF { componentRef: cRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
        let __pa7 = ::match_deref::match_deref! { match &(tImmediateVars.clone().borrow()[(i.clone()-1) as usize].clone()) {
            Deref @ DAE::Element::VAR { binding: __pa7, .. } => __pa7.clone(),
            _ => bail!("pattern mismatch"),
        } };
        bindExp = __pa7.clone();
        rhs = if (Util::applyOptionOrDefault(bindExp.clone(), Arc::new({ let __pe_b0 = Arc::new(DAE::Exp::BCONST { bool: true }); move |__pe_a1| ExpressionBasics::expEqual(__pe_b0.clone(), __pe_a1) }), false)) {exp.clone()} else {Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![exp.clone()], attr: DAE::callAttrBuiltinImpureBool.clone() })};
        eqs = cons(Arc::new(DAE::Element::EQUATION { exp: exp1.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() }), eqs.clone());
    }
    exp = Arc::new(DAE::Exp::CREF { componentRef: selectedStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expThen = Arc::new(DAE::Exp::ICONST { integer: 1 });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() })], attr: DAE::callAttrBuiltinImpureInteger.clone() });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
    selectedStateEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() });
    eqs = cons(selectedStateEqn.clone(), eqs.clone());
    exp = Arc::new(DAE::Exp::CREF { componentRef: selectedResetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expThen = Arc::new(DAE::Exp::BCONST { bool: true });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextResetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() })], attr: DAE::callAttrBuiltinImpureBool.clone() });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
    selectedResetEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() });
    eqs = cons(selectedResetEqn.clone(), eqs.clone());
    exp = Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() });
    expLst = metamodelica::nil();
    for mut i in 1..=nTransitions.clone() {
        expCond = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: tFromRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::CREF { componentRef: selectedStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), index: -1, optionExpisASUB: None });
        expThen = Arc::new(DAE::Exp::CREF { componentRef: cRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
        expElse = Arc::new(DAE::Exp::BCONST { bool: false });
        expIf = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
        expLst = cons(Arc::new(DAE::Exp::IFEXP { expCond: expIf.clone(), expThen: Arc::new(DAE::Exp::ICONST { integer: i.clone() }), expElse: Arc::new(DAE::Exp::ICONST { integer: 0 }) }), expLst.clone());
    }
    rhs = if ((expLst.clone().len() as i32) > 1) {Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }), expLst: list![Expression::makeScalarArray(expLst.clone(), DAE::T_INTEGER_DEFAULT.clone())], attr: DAE::callAttrBuiltinInteger.clone() })} else {listHead(expLst.clone())?};
    firedEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() });
    eqs = cons(firedEqn.clone(), eqs.clone());
    exp = Arc::new(DAE::Exp::CREF { componentRef: activeStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expThen = Arc::new(DAE::Exp::ICONST { integer: 1 });
    exp1 = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), operator: DAE::Operator::GREATER { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 0 }), index: -1, optionExpisASUB: None });
    exp2 = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("tTo")).clone(), tArrayInteger.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }) })], preRef.clone())?, ty: DAE::T_INTEGER_DEFAULT.clone() });
    expElse = Arc::new(DAE::Exp::IFEXP { expCond: exp1.clone(), expThen: exp2.clone(), expElse: Arc::new(DAE::Exp::CREF { componentRef: selectedStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }) });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
    activeStateEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() });
    eqs = cons(activeStateEqn.clone(), eqs.clone());
    exp = Arc::new(DAE::Exp::CREF { componentRef: activeResetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expThen = Arc::new(DAE::Exp::BCONST { bool: true });
    exp1 = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), operator: DAE::Operator::GREATER { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 0 }), index: -1, optionExpisASUB: None });
    exp2 = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("tReset")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }) })], preRef.clone())?, ty: DAE::T_INTEGER_DEFAULT.clone() });
    expElse = Arc::new(DAE::Exp::IFEXP { expCond: exp1.clone(), expThen: exp2.clone(), expElse: Arc::new(DAE::Exp::CREF { componentRef: selectedResetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() }) });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
    activeResetEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() });
    eqs = cons(activeResetEqn.clone(), eqs.clone());
    exp = Arc::new(DAE::Exp::CREF { componentRef: nextStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expThen = Arc::new(DAE::Exp::CREF { componentRef: activeStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() })], attr: DAE::callAttrBuiltinImpureInteger.clone() });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
    nextStateEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() });
    eqs = cons(nextStateEqn.clone(), eqs.clone());
    exp = Arc::new(DAE::Exp::CREF { componentRef: nextResetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    expThen = Arc::new(DAE::Exp::BCONST { bool: false });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextResetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() })], attr: DAE::callAttrBuiltinImpureBool.clone() });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
    nextResetEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() });
    eqs = cons(nextResetEqn.clone(), eqs.clone());
    for mut i in 1..=nStates.clone() {
        exp = Arc::new(DAE::Exp::CREF { componentRef: activeResetStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
        expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
        expThen = Arc::new(DAE::Exp::BCONST { bool: true });
        expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextResetStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() })], attr: DAE::callAttrBuiltinImpureBool.clone() });
        rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
        eqs = cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() }), eqs.clone());
    }
    for mut i in 1..=nStates.clone() {
        exp = Arc::new(DAE::Exp::CREF { componentRef: nextResetStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
        expCond = Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
        exp1 = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: activeStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: i.clone() }), index: -1, optionExpisASUB: None });
        expThen = Arc::new(DAE::Exp::IFEXP { expCond: exp1.clone(), expThen: Arc::new(DAE::Exp::BCONST { bool: false }), expElse: Arc::new(DAE::Exp::CREF { componentRef: activeResetStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() }) });
        expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextResetStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() })], attr: DAE::callAttrBuiltinImpureBool.clone() });
        rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
        eqs = cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() }), eqs.clone());
    }
    for mut i in 1..=nStates.clone() {
        exp = Arc::new(DAE::Exp::CREF { componentRef: finalStatesRefs.clone().borrow()[(i.clone()-1) as usize].clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
        expLst = metamodelica::nil();
        for mut j in 1..=nTransitions.clone() {
            expCond = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: tFromRefs.clone().borrow()[(j.clone()-1) as usize].clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: i.clone() }), index: -1, optionExpisASUB: None });
            expLst = cons(Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: Arc::new(DAE::Exp::ICONST { integer: 1 }), expElse: Arc::new(DAE::Exp::ICONST { integer: 0 }) }), expLst.clone());
        }
        exp1 = if ((expLst.clone().len() as i32) > 1) {Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }), expLst: list![Expression::makeScalarArray(expLst.clone(), DAE::T_INTEGER_DEFAULT.clone())], attr: DAE::callAttrBuiltinInteger.clone() })} else {listHead(expLst.clone())?};
        rhs = Arc::new(DAE::Exp::RELATION { exp1: exp1.clone(), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 0 }), index: -1, optionExpisASUB: None });
        eqs = cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() }), eqs.clone());
    }
    exp = Arc::new(DAE::Exp::CREF { componentRef: stateMachineInFinalStateRef.clone(), ty: DAE::T_BOOL_DEFAULT.clone() });
    rhs = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("finalStates")).clone(), nStatesArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::CREF { componentRef: activeStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT.clone() }) })], preRef.clone())?, ty: DAE::T_BOOL_DEFAULT.clone() });
    eqs = cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone() }), eqs.clone());
    flatSmSemantics = FlatSmSemantics { ident: (ident.clone()).clone(), smComps: metamodelica::arrayFromVec(q.clone().into_iter().cloned().collect()), t: t.clone(), c: cExps.clone(), vars: vars.clone(), knowns: knowns.clone(), eqs: eqs.clone(), pvars: metamodelica::nil(), peqs: metamodelica::nil(), enclosingState: None };
    Ok(flatSmSemantics)
}

fn qCref(mut ident: ArcStr, mut identType: Arc<DAE::Type>, mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut componentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outQual: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outQual = ComponentReference::joinCrefs(componentRef.clone(), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() }))?;
    Ok(outQual)
}

fn createVarWithDefaults(mut componentRef: Arc<DAE::ComponentRef>, mut kind: DAE::VarKind, mut ty: Arc<DAE::Type>, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<DAE::Element> {
    let mut var: Arc<DAE::Element>;
    var = Arc::new(DAE::Element::VAR { componentRef: componentRef.clone(), kind: kind.clone(), direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR, parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, protection: openmodelica_frontend_types::DAE::VarVisibility::PUBLIC, ty: ty.clone(), binding: None, dims: dims.clone(), connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), source: DAE::emptyElementSource.clone(), variableAttributesOption: None, comment: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: false });
    var
}

fn createVarWithStartValue(mut componentRef: Arc<DAE::ComponentRef>, mut kind: DAE::VarKind, mut ty: Arc<DAE::Type>, mut startExp: Arc<DAE::Exp>, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<DAE::Element>> {
    let mut outVar: Arc<DAE::Element>;
    let mut var: Arc<DAE::Element>;
    var = Arc::new(DAE::Element::VAR { componentRef: componentRef.clone(), kind: kind.clone(), direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR, parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, protection: openmodelica_frontend_types::DAE::VarVisibility::PUBLIC, ty: ty.clone(), binding: None, dims: dims.clone(), connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), source: DAE::emptyElementSource.clone(), variableAttributesOption: None, comment: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: false });
    outVar = setVarFixedStartValue(var.clone(), startExp.clone())?;
    Ok(outVar)
}

fn createTandC(mut inSMComps: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inTransitions: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Transition>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut t: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut c: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut transitions: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    transitions = List::map1(inTransitions.clone(), Arc::new(createTransition), inSMComps.clone());
    t = List::sort(transitions.clone(), Arc::new(priorityLt))?;
    c = List::map(t.clone(), Arc::new(extractCondtionFromTransition));
    Ok((t, c))
}

fn extractCondtionFromTransition(mut trans: Transition) -> Result<Arc<DAE::Exp>> {
    let mut condition: Arc<DAE::Exp>;
    let Transition { condition: __pa0, .. } = (trans.clone()) else { bail!("pattern mismatch") };
    condition = __pa0.clone();
    Ok(condition)
}

fn priorityLt(mut inTrans1: Transition, mut inTrans2: Transition) -> Result<bool> {
    let mut res: bool = false;
    let mut priority1: i32 = 0;
    let mut priority2: i32 = 0;
    let Transition { priority: __pa0, .. } = (inTrans1.clone()) else { bail!("pattern mismatch") };
    priority1 = __pa0.clone();
    let Transition { priority: __pa1, .. } = (inTrans2.clone()) else { bail!("pattern mismatch") };
    priority2 = __pa1.clone();
    res = intLt(priority1.clone(), priority2.clone());
    Ok(res)
}

fn createTransition(mut transitionElem: Arc<DAE::Element>, mut states: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Transition> {
    let mut trans: Transition;
    let mut crefFrom: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crefTo: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut from: i32 = 0;
    let mut to: i32 = 0;
    let mut condition: Arc<DAE::Exp>;
    let mut immediate: bool = true;
    let mut reset: bool = true;
    let mut synchronize: bool = false;
    let mut priority: i32 = 1;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(transitionElem.clone()) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa1, .. }, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa4 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa5 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: __pa6 }, tail: Deref @ metamodelica::List::Nil } } } } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    crefFrom = __pa0.clone();
    crefTo = __pa1.clone();
    condition = __pa2.clone();
    immediate = __pa3.clone();
    reset = __pa4.clone();
    synchronize = __pa5.clone();
    priority = __pa6.clone();
    from = List::position1OnTrue(states.clone(), Arc::new(sMCompEqualsRef), crefFrom.clone());
    to = List::position1OnTrue(states.clone(), Arc::new(sMCompEqualsRef), crefTo.clone());
    trans = Transition { from: from.clone(), to: to.clone(), condition: condition.clone(), immediate: immediate.clone(), reset: reset.clone(), synchronize: synchronize.clone(), priority: priority.clone() };
    Ok(trans)
}

fn isFlatSm(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outResult: bool = false;
    outResult = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::FLAT_SM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outResult
}

fn isSMComp(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outResult: bool = false;
    outResult = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::SM_COMP { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outResult
}

fn isTransition(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isInitialState(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isEquation(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isEquationOrWhenEquation(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::EQUATION { .. } => true,
        Deref @ DAE::Element::WHEN_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isPreOrPreviousEquation(mut inElement: Arc<DAE::Element>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::EQUATION { exp, scalar, source: _ } => {
            Expression::expHasPre(exp.clone())? || Expression::expHasPre(scalar.clone())? || Expression::expHasPrevious(exp.clone())? || Expression::expHasPrevious(scalar.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn isVar(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn sMCompEqualsRef(mut inElement: Arc<DAE::Element>, mut inCref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: cref, .. } if (ComponentReferenceBasics::crefEqual(cref.clone(), inCref.clone())?) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn dumpTransitionStr(mut transition: Transition) -> Result<ArcStr> {
    let mut transitionStr: ArcStr = arcstr::literal!("");
    let mut from: i32 = 0;
    let mut to: i32 = 0;
    let mut condition: Arc<DAE::Exp>;
    let mut immediate: bool = false;
    let mut reset: bool = false;
    let mut synchronize: bool = false;
    let mut priority: i32 = 0;
    let Transition { from: __pa0, to: __pa1, condition: __pa2, immediate: __pa3, reset: __pa4, synchronize: __pa5, priority: __pa6 } = (transition.clone()) else { bail!("pattern mismatch") };
    from = __pa0.clone();
    to = __pa1.clone();
    condition = __pa2.clone();
    immediate = __pa3.clone();
    reset = __pa4.clone();
    synchronize = __pa5.clone();
    priority = __pa6.clone();
    transitionStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TRANSITION(from=")); __mm_s.push_str(&*intString(from.clone())); __mm_s.push_str(&*literal!(", to=")); __mm_s.push_str(&*intString(to.clone())); __mm_s.push_str(&*literal!(", condition=")); __mm_s.push_str(&*ExpressionBasics::printExpStr(condition.clone())?); __mm_s.push_str(&*literal!(", immediate=")); __mm_s.push_str(&*boolString(immediate.clone())); __mm_s.push_str(&*literal!(", reset=")); __mm_s.push_str(&*boolString(reset.clone())); __mm_s.push_str(&*literal!(", synchronize=")); __mm_s.push_str(&*boolString(synchronize.clone())); __mm_s.push_str(&*literal!(", priority=")); __mm_s.push_str(&*intString(priority.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(transitionStr)
}

fn wrapHack(mut cache: FCore::Cache, mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut nOfSubstitutions: i32 = 0;
    let mut eqnLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut whenEq: Arc<DAE::Element>;
    let mut cond1: Arc<DAE::Exp>;
    let mut cond2: Arc<DAE::Exp>;
    let mut condition: Arc<DAE::Exp>;
    let mut condLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tArrayBool: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    cond1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("initial")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinImpureBool.clone() });
    cond2 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: list![Arc::new(DAE::Exp::RCONST { real: Flags::getConfigReal(Flags::DEFAULT_CLOCK_PERIOD.clone())? }), Arc::new(DAE::Exp::RCONST { real: Flags::getConfigReal(Flags::DEFAULT_CLOCK_PERIOD.clone())? })], attr: DAE::callAttrBuiltinImpureBool.clone() });
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 })] });
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        condLst = List::filterMap1(inElementLst.clone(), Arc::new(extractSmOfExps), (literal!("cImmediate")).clone());
        (eqnLst, otherLst) = List::extractOnTrue(inElementLst.clone(), Arc::new(isPreOrPreviousEquation));
        condition = Arc::new(DAE::Exp::ARRAY { ty: tArrayBool.clone(), scalar: true, array: cons(cond1.clone(), condLst.clone()) });
    } else {
        (eqnLst, otherLst) = List::extractOnTrue(inElementLst.clone(), Arc::new(fnptr!(isEquation, Arc<DAE::Element>)));
        condition = Arc::new(DAE::Exp::ARRAY { ty: tArrayBool.clone(), scalar: true, array: list![cond1.clone(), cond2.clone()] });
    }
    whenEq = Arc::new(DAE::Element::WHEN_EQUATION { condition: condition.clone(), equations: eqnLst.clone(), elsewhen_: None, source: DAE::emptyElementSource.clone() });
    outElementLst = listAppend(otherLst.clone(), list![whenEq.clone()]);
    Ok(outElementLst)
}

fn extractSmOfExps(mut inElem: Arc<DAE::Element>, mut inLastIdent: ArcStr) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inElem.clone()) {
        Deref @ DAE::Element::EQUATION { exp, .. } => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut firstIdent: ArcStr = arcstr::literal!("");
            let mut lastIdent: ArcStr = arcstr::literal!("");
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cref = __pa0.clone();
            firstIdent = (ComponentReferenceBasics::crefFirstIdent(cref.clone())?).clone();
            let true = (firstIdent.clone() == literal!("smOf")) else { bail!("pattern mismatch") };
            lastIdent = (ComponentReferenceBasics::crefLastIdent(cref.clone())?).clone();
            let true = (lastIdent.clone() == inLastIdent.clone()) else { bail!("pattern mismatch") };
            exp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn traversingSubsPreForPrevious(mut inExp: Arc<DAE::Exp>, mut inHitCount: i32) -> (Arc<DAE::Exp>, i32) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHitCount: i32 = 0;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst, attr } => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("pre")).clone() }), expLst: expLst.clone(), attr: attr.clone() }), inHitCount.clone() + 1)
        },
        _ => {
            (inExp.clone(), inHitCount.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHitCount)
}

fn traversingSubsXForSampleX(mut inExp: Arc<DAE::Exp>, mut inHitCount: i32) -> (Arc<DAE::Exp>, i32) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHitCount: i32 = 0;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::List::Cons { head: expX, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK }, tail: Deref @ metamodelica::List::Nil } }, attr: _ } => {
            (expX.clone(), inHitCount.clone() + 1)
        },
        _ => {
            (inExp.clone(), inHitCount.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHitCount)
}

