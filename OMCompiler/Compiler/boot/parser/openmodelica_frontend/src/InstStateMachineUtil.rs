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
use crate::HashSet;
use crate::HashTable3;
use crate::HashTable;
use crate::HashTableCG;
use crate::HashTableSM1;
use crate::InnerOuter;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

/// Collecting information about a state/mode
#[derive(Clone, Debug, PartialEq)]
pub struct SMNode {
    pub componentRef: Arc<DAE::ComponentRef>,
    pub isInitial: bool,
    /// relations to other modes due to in- and out-going transitions
    pub edges: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)),
}

pub type SMNODE = SMNode;


/// Collecting information about a group of state components forming a flat state machine
#[derive(Clone, Debug, PartialEq)]
pub struct FlatSMGroup {
    pub initState: Arc<DAE::ComponentRef>,
    pub states: metamodelica::Array<Arc<DAE::ComponentRef>>,
}

pub type FLAT_SM_GROUP = FlatSMGroup;


#[derive(Clone, Debug, PartialEq)]
pub struct AdjacencyTable {
    /// Map cref to corresponding index in adjacency matrix
    pub cref2index: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)),
    /// Adjacency matrix showing which modes are connected by transitions
    pub adjacency: metamodelica::Array<metamodelica::Array<bool>>,
}

impl Default for AdjacencyTable {
    fn default() -> Self {
        Self {
            cref2index: Default::default(),
            adjacency: Default::default(),
        }
    }
}

pub type ADJACENCY_TABLE = AdjacencyTable;


// Table having crefs as keys and corresponding SMNODE as value
pub type SMNodeTable = (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SMNode)>>), i32, (HashTableSM1::FuncHashCref, HashTableSM1::FuncCrefEqual, HashTableSM1::FuncCrefStr, HashTableSM1::FuncExpStr));

// Table mapping crefs of SMNodes to corresponding crefs of FlatSMGroup
pub type SMNodeToFlatSMGroupTable = (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));

pub const SMS_PRE: &'static str = "smOf";

pub const DEBUG_SMDUMP: bool = false;

pub fn createSMNodeToFlatSMGroupTable(mut inDae: DAE::DAElist) -> Result<SMNodeToFlatSMGroupTable> {
    let mut smNodeToFlatSMGroup: SMNodeToFlatSMGroupTable;
    let mut elementLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smNodeTable: SMNodeTable;
    let mut nStates: i32 = 0;
    let mut iTable: AdjacencyTable;
    let mut transClosure: AdjacencyTable;
    let mut initialStates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut flatSMGroup: Arc<metamodelica::List<FlatSMGroup>> = metamodelica::nil();
    if intLt(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 33) {
        smNodeToFlatSMGroup = HashTableCG::emptyHashTableSized(1);
        return Ok(smNodeToFlatSMGroup);
    }
    let DAE::DAE { elementLst: __pa0 } = (inDae.clone()) else { bail!("pattern mismatch") };
    elementLst = __pa0.clone();
    smNodeTable = getSMNodeTable(elementLst.clone());
    nStates = BaseHashTable::hashTableCurrentSize(smNodeTable.clone());
    if nStates.clone() > 0 {
        smNodeToFlatSMGroup = HashTableCG::emptyHashTable();
        if DEBUG_SMDUMP.clone() {
            println!("{}", (literal!("***** InstStateMachineUtil.createSMNodeToFlatSMGroupTable: START ***** \n")).clone());
        }
        if DEBUG_SMDUMP.clone() {
            println!("{}", (literal!("***** State machine node table: ***** \n")).clone());
        }
        if DEBUG_SMDUMP.clone() {
            BaseHashTable::dumpHashTable(smNodeTable.clone());
        }
        if DEBUG_SMDUMP.clone() {
            println!("{}", (literal!("***** Adjacency Matrix: ***** \n")).clone());
        }
        iTable = createAdjacencyTable(smNodeTable.clone(), nStates.clone())?;
        if DEBUG_SMDUMP.clone() {
            printAdjacencyTable(iTable.clone(), nStates.clone())?;
        }
        if DEBUG_SMDUMP.clone() {
            println!("{}", (literal!("***** Transitive Closure: ***** \n")).clone());
        }
        transClosure = transitiveClosure(iTable.clone(), nStates.clone())?;
        if DEBUG_SMDUMP.clone() {
            printAdjacencyTable(transClosure.clone(), nStates.clone())?;
        }
        if DEBUG_SMDUMP.clone() {
            println!("{}", (literal!("***** Initial States: ***** \n")).clone());
        }
        initialStates = extractInitialStates(smNodeTable.clone())?;
        if DEBUG_SMDUMP.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(initialStates.clone(), Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        if DEBUG_SMDUMP.clone() {
            println!("{}", (literal!("***** Flat State Machine Groups: ***** \n")).clone());
        }
        flatSMGroup = extractFlatSMGroup(initialStates.clone(), transClosure.clone(), nStates.clone())?;
        if DEBUG_SMDUMP.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(flatSMGroup.clone(), Arc::new(dumpFlatSMGroupStr)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        if DEBUG_SMDUMP.clone() {
            println!("{}", (literal!("***** SM Node cref to SM Group cref mapping: ***** \n")).clone());
        }
        smNodeToFlatSMGroup = List::fold(flatSMGroup.clone(), Arc::new(relateNodesToGroup), smNodeToFlatSMGroup.clone());
        if DEBUG_SMDUMP.clone() {
            BaseHashTable::dumpHashTable(smNodeToFlatSMGroup.clone());
        }
        if DEBUG_SMDUMP.clone() {
            println!("{}", (literal!("***** InstStateMachineUtil.createSMNodeToFlatSMGroupTable: END ***** \n")).clone());
        }
    } else {
        smNodeToFlatSMGroup = HashTableCG::emptyHashTableSized(1);
    }
    Ok(smNodeToFlatSMGroup)
}

pub fn wrapSMCompsInFlatSMs(mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inDae1: DAE::DAElist, mut inDae2: DAE::DAElist, mut smNodeToFlatSMGroup: SMNodeToFlatSMGroupTable, mut smInitialCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(DAE::DAElist, DAE::DAElist)> {
    let mut outDae1: DAE::DAElist;
    let mut outDae2: DAE::DAElist;
    let mut elementLst1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elementLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smCompsLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut otherLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut smTransitionsLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut flatSmLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut flatSMsAndMergingEqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let DAE::DAE { elementLst: __pa0 } = (inDae1.clone()) else { bail!("pattern mismatch") };
    elementLst1 = __pa0.clone();
    (smCompsLst, otherLst1) = List::extractOnTrue(elementLst1.clone(), Arc::new(fnptr!(isSMComp, Arc<DAE::Element>)));
    let DAE::DAE { elementLst: __pa1 } = (inDae2.clone()) else { bail!("pattern mismatch") };
    elementLst2 = __pa1.clone();
    (smTransitionsLst, otherLst2) = List::extractOnTrue(elementLst2.clone(), Arc::new(fnptr!(isSMStatement2, Arc<DAE::Element>)));
    flatSmLst = List::map2(smInitialCrefs.clone(), Arc::new(createFlatSM), listAppend(smCompsLst.clone(), smTransitionsLst.clone()), smNodeToFlatSMGroup.clone());
    flatSMsAndMergingEqns = List::fold1(flatSmLst.clone(), Arc::new(mergeVariableDefinitions.clone()), inIH.clone(), metamodelica::nil());
    outDae1 = DAE::DAElist { elementLst: listAppend(flatSMsAndMergingEqns.clone(), otherLst1.clone()) };
    outDae2 = DAE::DAElist { elementLst: otherLst2.clone() };
    Ok((outDae1, outDae2))
}

fn freshAliasEqn_der(mut inInnerCrefToOuterOutputCrefs: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut innerCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outerCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (innerCref, outerCrefs) = inInnerCrefToOuterOutputCrefs.clone();
    ty = ComponentReference::crefLastType(innerCref.clone())?;
    outEqns = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut outerCref in (outerCrefs.clone()).into_iter().cloned() {
            let __x = Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: innerCref.clone(), ty: ty.clone() }), scalar: Arc::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() }), source: DAE::emptyElementSource.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outEqns)
}

fn freshMergingEqn_der(mut inInnerCrefToOuterOutputCrefs: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut innerCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outerCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outerCrefsStripped: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outerCrefDers: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut exp: Arc<DAE::Exp>;
    (innerCref, outerCrefs) = inInnerCrefToOuterOutputCrefs.clone();
    ty = ComponentReference::crefLastType(innerCref.clone())?;
    outerCrefsStripped = List::map(outerCrefs.clone(), Arc::new(ComponentReference::crefStripLastIdent));
    outerCrefDers = List::map(outerCrefs.clone(), Arc::new({ let __pe_b0 = (literal!("_der$")).clone(); move |__pe_a1| ComponentReference::appendStringLastIdent(__pe_b0.clone(), __pe_a1) }));
    exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: innerCref.clone(), ty: ty.clone() })], attr: DAE::callAttrBuiltinReal.clone() });
    outEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: mergingRhs_der(outerCrefDers.clone(), innerCref.clone(), ty.clone())?, source: DAE::emptyElementSource.clone() });
    Ok(outEqn)
}

fn mergingRhs_der(mut inOuterCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inInnerCref: Arc<DAE::ComponentRef>, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    let mut callAttributes: Arc<DAE::CallAttributes> = Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    res = (::match_deref::match_deref! { match &(inOuterCrefs.clone()) {
        Deref @ metamodelica::List::Cons { head: outerCref, tail: Deref @ metamodelica::List::Nil } => {
            let mut crefState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut outerCrefExp: Arc<DAE::Exp>;
            let mut crefStateExp: Arc<DAE::Exp>;
            let mut ifExp: Arc<DAE::Exp>;
            let mut expCond: Arc<DAE::Exp>;
            let mut expElse: Arc<DAE::Exp>;
            outerCrefExp = Arc::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() });
            crefState = ComponentReference::crefStripLastIdent(outerCref.clone())?;
            crefStateExp = Arc::new(DAE::Exp::CREF { componentRef: crefState.clone(), ty: ty.clone() });
            expCond = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("activeState")).clone() }), expLst: list![crefStateExp.clone()], attr: callAttributes.clone() });
            expElse = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) });
            ifExp = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: outerCrefExp.clone(), expElse: expElse.clone() });
            ifExp.clone()
        },
        Deref @ metamodelica::List::Cons { head: outerCref, tail: rest } => {
            let mut crefState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut outerCrefExp: Arc<DAE::Exp>;
            let mut crefStateExp: Arc<DAE::Exp>;
            let mut ifExp: Arc<DAE::Exp>;
            let mut expCond: Arc<DAE::Exp>;
            let mut expElse: Arc<DAE::Exp>;
            outerCrefExp = Arc::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() });
            crefState = ComponentReference::crefStripLastIdent(outerCref.clone())?;
            crefStateExp = Arc::new(DAE::Exp::CREF { componentRef: crefState.clone(), ty: ty.clone() });
            expCond = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("activeState")).clone() }), expLst: list![crefStateExp.clone()], attr: callAttributes.clone() });
            expElse = mergingRhs_der(rest.clone(), inInnerCref.clone(), ty.clone())?;
            ifExp = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: outerCrefExp.clone(), expElse: expElse.clone() });
            ifExp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

fn traversingCountDer(mut inExp: Arc<DAE::Exp>, mut inCref_HitCount: (Arc<DAE::ComponentRef>, i32)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outCref_HitCount: (Arc<DAE::ComponentRef>, i32);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut hitCount: i32 = 0;
    (cref, hitCount) = inCref_HitCount.clone();
    (outExp, outCref_HitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } if (ComponentReferenceBasics::crefEqual(componentRef.clone(), cref.clone())?) => {
            (inExp.clone(), (cref.clone(), hitCount.clone() + 1))
        },
        _ => {
            (inExp.clone(), inCref_HitCount.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outCref_HitCount))
}

fn freshMergingEqn(mut inInnerCrefToOuterOutputCrefs: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut innerCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outerCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outerCrefsStripped: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (innerCref, outerCrefs) = inInnerCrefToOuterOutputCrefs.clone();
    ty = ComponentReference::crefLastType(innerCref.clone())?;
    outerCrefsStripped = List::map(outerCrefs.clone(), Arc::new(ComponentReference::crefStripLastIdent));
    outEqn = Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: innerCref.clone(), ty: ty.clone() }), scalar: mergingRhs(outerCrefs.clone(), innerCref.clone(), ty.clone())?, source: DAE::emptyElementSource.clone() });
    Ok(outEqn)
}

fn mergingRhs(mut inOuterCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inInnerCref: Arc<DAE::ComponentRef>, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    let mut callAttributes: Arc<DAE::CallAttributes> = Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    res = (::match_deref::match_deref! { match &(inOuterCrefs.clone()) {
        Deref @ metamodelica::List::Cons { head: outerCref, tail: Deref @ metamodelica::List::Nil } => {
            let mut crefState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut outerCrefExp: Arc<DAE::Exp>;
            let mut innerCrefExp: Arc<DAE::Exp>;
            let mut crefStateExp: Arc<DAE::Exp>;
            let mut ifExp: Arc<DAE::Exp>;
            let mut expCond: Arc<DAE::Exp>;
            let mut expElse: Arc<DAE::Exp>;
            outerCrefExp = Arc::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() });
            innerCrefExp = Arc::new(DAE::Exp::CREF { componentRef: inInnerCref.clone(), ty: ty.clone() });
            crefState = ComponentReference::crefStripLastIdent(outerCref.clone())?;
            crefStateExp = Arc::new(DAE::Exp::CREF { componentRef: crefState.clone(), ty: ty.clone() });
            expCond = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("activeState")).clone() }), expLst: list![crefStateExp.clone()], attr: callAttributes.clone() });
            expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![innerCrefExp.clone()], attr: callAttributes.clone() });
            ifExp = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: outerCrefExp.clone(), expElse: expElse.clone() });
            ifExp.clone()
        },
        Deref @ metamodelica::List::Cons { head: outerCref, tail: rest } => {
            let mut crefState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut outerCrefExp: Arc<DAE::Exp>;
            let mut crefStateExp: Arc<DAE::Exp>;
            let mut ifExp: Arc<DAE::Exp>;
            let mut expCond: Arc<DAE::Exp>;
            let mut expElse: Arc<DAE::Exp>;
            outerCrefExp = Arc::new(DAE::Exp::CREF { componentRef: outerCref.clone(), ty: ty.clone() });
            crefState = ComponentReference::crefStripLastIdent(outerCref.clone())?;
            crefStateExp = Arc::new(DAE::Exp::CREF { componentRef: crefState.clone(), ty: ty.clone() });
            expCond = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("activeState")).clone() }), expLst: list![crefStateExp.clone()], attr: callAttributes.clone() });
            expElse = mergingRhs(rest.clone(), inInnerCref.clone(), ty.clone())?;
            ifExp = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: outerCrefExp.clone(), expElse: expElse.clone() });
            ifExp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

fn collectCorrespondingKeys(mut inInnerCref: Arc<DAE::ComponentRef>, mut inHashEntries: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>, mut inInnerCrefToOuterOutputCrefs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr))> {
    let mut outInnerCrefToOuterOutputCrefs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr)) = inInnerCrefToOuterOutputCrefs.clone();
    let mut outerRefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outerRefs = List::filterMap1(inHashEntries.clone(), Arc::new(crefEqualTuple22), inInnerCref.clone());
    outInnerCrefToOuterOutputCrefs = BaseHashTable::addUnique((inInnerCref.clone(), outerRefs.clone()), outInnerCrefToOuterOutputCrefs.clone())?;
    Ok(outInnerCrefToOuterOutputCrefs)
}

fn crefEqualTuple22(mut inHashEntry: (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>), mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut isEqual: bool = false;
    let mut tuple22: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    tuple22 = Util::tuple22(inHashEntry.clone());
    isEqual = ComponentReferenceBasics::crefEqual(tuple22.clone(), inCref.clone())?;
    if !(isEqual.clone()) {
        bail!("fail");
    }
    outCref = Util::tuple21(inHashEntry.clone());
    Ok(outCref)
}

fn traverserHelperSubsOuterByInnerExp(mut inExp: Arc<DAE::Exp>, mut inOuterToInner: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outOuterToInner: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    (outExp, outOuterToInner) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(traverserHelperSubsOuterByInner), inOuterToInner.clone())?;
    Ok((outExp, outOuterToInner))
}

fn traverserHelperSubsOuterByInner(mut inExp: Arc<DAE::Exp>, mut inOuterToInner: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outOuterToInner: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    (outExp, outOuterToInner) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef, ty }, tail: Deref @ metamodelica::List::Nil }, attr } if (BaseHashTable::hasKey(componentRef.clone(), inOuterToInner.clone())) => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: BaseHashTable::get(componentRef.clone(), inOuterToInner.clone())?, ty: ty.clone() })], attr: attr.clone() }), inOuterToInner.clone())
        },
        _ => {
            (inExp.clone(), inOuterToInner.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outOuterToInner))
}

fn matchOuterWithInner(mut inOuterCref: Arc<DAE::ComponentRef>, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inOuterCrefToInnerCref: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr))> {
    let mut outOuterCrefToInnerCref: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)) = inOuterCrefToInnerCref.clone();
    let mut crefIdent: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crefFound: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut strippedCref1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut strippedCref2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    crefIdent = ComponentReferenceBasics::crefLastCref(inOuterCref.clone())?;
    strippedCref1 = ComponentReference::crefStripLastIdent(inOuterCref.clone())?;
    strippedCref2 = if (ComponentReference::crefDepth(strippedCref1.clone())? >= 2) {ComponentReference::joinCrefs(ComponentReference::crefStripLastIdent(strippedCref1.clone())?, crefIdent.clone())?} else {crefIdent.clone()};
    crefFound = findInner(strippedCref2.clone(), crefIdent.clone(), inIH.clone())?;
    outOuterCrefToInnerCref = BaseHashTable::addUnique((inOuterCref.clone(), crefFound.clone()), outOuterCrefToInnerCref.clone())?;
    Ok(outOuterCrefToInnerCref)
}

fn findInner(mut inCrefTest: Arc<DAE::ComponentRef>, mut inCrefIdent: Arc<DAE::ComponentRef>, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCrefFound: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut testCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut strippedCref1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut strippedCref2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ht: InnerOuter::InstHierarchyHashTable;
    let InnerOuter::TOP_INSTANCE { ht: __pa0, .. } = (listHead(inIH.clone())?) else { bail!("pattern mismatch") };
    ht = __pa0.clone();
    match '__try1: {
        let _ = unwrap_break_err!(InnerOuter::get(inCrefTest.clone(), ht.clone()), '__try1);
        outCrefFound = inCrefTest.clone();
        Ok::<_, anyhow::Error>((outCrefFound.clone(),))
    } {
        Ok((__try1_o0,)) => {
            outCrefFound = __try1_o0;
        }
        Err(_) => {
            strippedCref1 = ComponentReference::crefStripLastIdent(inCrefTest.clone())?;
            strippedCref2 = if (ComponentReference::crefDepth(strippedCref1.clone())? >= 2) {ComponentReference::joinCrefs(ComponentReference::crefStripLastIdent(strippedCref1.clone())?, inCrefIdent.clone())?} else {inCrefIdent.clone()};
            outCrefFound = findInner(strippedCref2.clone(), inCrefIdent.clone(), inIH.clone())?;
        }
    }
    Ok(outCrefFound)
}

fn collectOuterOutputs(mut inElem: Arc<DAE::Element>, mut inOuterAcc: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr))) -> (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)) {
    let mut outOuterAcc: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)) = inOuterAcc.clone();
    let mut outerOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outerOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outerOutputCrefToSMCompCref: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outOuterAcc = (::match_deref::match_deref! { match &(inElem.clone()) {
        Deref @ DAE::Element::SM_COMP { dAElist, componentRef } => {
            outerOutputs = List::filterOnTrue(dAElist.clone(), Arc::new(fnptr!(isOuterOutput, Arc<DAE::Element>)));
            outerOutputCrefs = List::map(outerOutputs.clone(), Arc::new(DAEUtil::varCref));
            outerOutputCrefToSMCompCref = List::map(outerOutputCrefs.clone(), Arc::new({ let __pe_b1 = componentRef.clone(); move |__pe_a0| Ok(Util::makeTuple(__pe_a0, __pe_b1.clone())) }));
            List::fold(outerOutputCrefToSMCompCref.clone(), Arc::new(BaseHashTable::addUnique), outOuterAcc.clone())
        },
        _ => inOuterAcc.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outOuterAcc
}

fn isOuterOutput(mut inElem: Arc<DAE::Element>) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inElem.clone()) {
        Deref @ DAE::Element::VAR { innerOuter: Absyn::InnerOuter::OUTER, direction: DAE::VarDirection::OUTPUT, .. } => {
            true
        },
        Deref @ DAE::Element::VAR { innerOuter: Absyn::InnerOuter::INNER_OUTER, direction: DAE::VarDirection::OUTPUT, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn createFlatSM(mut smInitialCref: Arc<DAE::ComponentRef>, mut smElemsLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut smNodeToFlatSMGroup: SMNodeToFlatSMGroupTable) -> Result<Arc<DAE::Element>> {
    let mut flatSM: Arc<DAE::Element>;
    let mut smElemsInFlatSM: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    smElemsInFlatSM = List::filter2OnTrue(smElemsLst.clone(), Arc::new(isInFlatSM), smInitialCref.clone(), smNodeToFlatSMGroup.clone());
    flatSM = Arc::new(DAE::Element::FLAT_SM { ident: (ComponentReferenceBasics::printComponentRefStr(smInitialCref.clone())?).clone(), dAElist: smElemsInFlatSM.clone() });
    Ok(flatSM)
}

fn isInFlatSM(mut inElement: Arc<DAE::Element>, mut smInitialCref: Arc<DAE::ComponentRef>, mut smNodeToFlatSMGroup: SMNodeToFlatSMGroupTable) -> Result<bool> {
    let mut outResult: bool = false;
    let mut crefCorrespondingFlatSMGroup: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    crefCorrespondingFlatSMGroup = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: cref1, .. } if (BaseHashTable::hasKey(cref1.clone(), smNodeToFlatSMGroup.clone())) => {
            BaseHashTable::get(cref1.clone(), smNodeToFlatSMGroup.clone())?
        },
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: _ }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, .. }, .. } if (BaseHashTable::hasKey(cref1.clone(), smNodeToFlatSMGroup.clone())) => {
            BaseHashTable::get(cref1.clone(), smNodeToFlatSMGroup.clone())?
        },
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, .. }, .. } if (BaseHashTable::hasKey(cref1.clone(), smNodeToFlatSMGroup.clone())) => {
            BaseHashTable::get(cref1.clone(), smNodeToFlatSMGroup.clone())?
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstStateMachineUtil.isInFlatSM failed: Hash table lookup failed for ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![inElement.clone()])?); ArcStr::from(__mm_s) }).clone())?;
            BaseHashTable::dumpHashTableStatistics(smNodeToFlatSMGroup.clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outResult = ComponentReferenceBasics::crefEqual(crefCorrespondingFlatSMGroup.clone(), smInitialCref.clone())?;
    Ok(outResult)
}

fn isSMComp(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outResult: bool = false;
    outResult = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: _, dAElist: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outResult
}

fn relateNodesToGroup(mut flatSMGroup: FlatSMGroup, mut inNodeToGroup: SMNodeToFlatSMGroupTable) -> Result<SMNodeToFlatSMGroupTable> {
    let mut outNodeToGroup: SMNodeToFlatSMGroupTable = inNodeToGroup.clone();
    let mut nodeGroup: metamodelica::Array<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>;
    let mut initState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut states: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let FlatSMGroup { initState: __pa0, states: __pa1 } = (flatSMGroup.clone()) else { bail!("pattern mismatch") };
    initState = __pa0.clone();
    states = __pa1.clone();
    nodeGroup = Array::map(states.clone(), Arc::new({ let __pe_b1 = initState.clone(); move |__pe_a0| Ok(Util::makeTuple(__pe_a0, __pe_b1.clone())) }));
    outNodeToGroup = Array::fold(nodeGroup.clone(), Arc::new(BaseHashTable::add), outNodeToGroup.clone());
    Ok(outNodeToGroup)
}

fn extractFlatSMGroup(mut initialStates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut iTable: AdjacencyTable, mut nStates: i32) -> Result<Arc<metamodelica::List<FlatSMGroup>>> {
    let mut flatSMGroup: Arc<metamodelica::List<FlatSMGroup>> = metamodelica::nil();
    let mut cref2index: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut adjacency: metamodelica::Array<metamodelica::Array<bool>>;
    let mut entries: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut i2cref: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut members: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut membersArr: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut memberSet: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let AdjacencyTable { cref2index: __pa0, adjacency: __pa1 } = (iTable.clone()) else { bail!("pattern mismatch") };
    cref2index = __pa0.clone();
    adjacency = __pa1.clone();
    n = BaseHashTable::hashTableCurrentSize(cref2index.clone());
    assert!(n.clone() == nStates.clone(), "{}", &*(literal!("Value of nStates needs to be equal to number of modes within state table argument.")).clone());
    entries = BaseHashTable::hashTableList(cref2index.clone());
    entries = List::sort(entries.clone(), Arc::new(fnptr!(crefIndexCmp, (Arc<DAE::ComponentRef>, i32), (Arc<DAE::ComponentRef>, i32))))?;
    i2cref = metamodelica::arrayFromVec(List::map(entries.clone(), Arc::new(fnptr!(Util::tuple21, _))).into_iter().cloned().collect());
    flatSMGroup = metamodelica::nil();
    for mut cref in &*initialStates.clone() {
        let mut cref = cref.clone();
        i = BaseHashTable::get(cref.clone(), cref2index.clone())?;
        members = metamodelica::nil();
        for mut j in 1..=n.clone() {
            if adjacency.clone().borrow()[(i.clone()-1) as usize].clone().borrow()[(j.clone()-1) as usize].clone() {
                members = cons(i2cref.clone().borrow()[(j.clone()-1) as usize].clone(), members.clone());
            }
        }
        memberSet = HashSet::emptyHashSetSized((members.clone().len() as i32));
        memberSet = List::fold(members.clone(), Arc::new(BaseHashSet::add), memberSet.clone());
        memberSet = BaseHashSet::delete(cref.clone(), memberSet.clone())?;
        membersArr = metamodelica::arrayFromVec(cons(cref.clone(), BaseHashSet::hashSetList(memberSet.clone())?).into_iter().cloned().collect());
        flatSMGroup = cons(FlatSMGroup { initState: cref.clone(), states: membersArr.clone() }, flatSMGroup.clone());
    }
    Ok(flatSMGroup)
}

pub fn dumpFlatSMGroupStr(mut flatA: FlatSMGroup) -> Result<ArcStr> {
    let mut flatStr: ArcStr = arcstr::literal!("");
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut initialStateStr: ArcStr = arcstr::literal!("");
    let mut statesStr: ArcStr = arcstr::literal!("");
    let mut statesStrs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut initState: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut states: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let FlatSMGroup { states: __pa0, initState: __pa1 } = (flatA.clone()) else { bail!("pattern mismatch") };
    states = __pa0.clone();
    initState = __pa1.clone();
    initialStateStr = (ComponentReferenceBasics::printComponentRefStr(initState.clone())?).clone();
    crefs = Arc::new(states.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    statesStrs = List::map(crefs.clone(), Arc::new(ComponentReferenceBasics::printComponentRefStr));
    statesStr = stringDelimitList(statesStrs.clone(), (literal!(", ")).clone());
    flatStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*initialStateStr.clone()); __mm_s.push_str(&*literal!("( states(")); __mm_s.push_str(&*statesStr.clone()); __mm_s.push_str(&*literal!("))")); ArcStr::from(__mm_s) }).clone();
    Ok(flatStr)
}

fn extractInitialStates(mut smNodeTable: SMNodeTable) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut initialStates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut entries: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SMNode)>> = metamodelica::nil();
    let mut e: (Arc<DAE::ComponentRef>, SMNode);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut smNode: SMNode;
    let mut isInitial: bool = false;
    entries = BaseHashTable::hashTableList(smNodeTable.clone());
    initialStates = metamodelica::nil();
    for mut e in &*entries.clone() {
        let mut e = e.clone();
        (cref, smNode) = e.clone();
        let SMNode { isInitial: __pa0, .. } = (smNode.clone()) else { bail!("pattern mismatch") };
        isInitial = __pa0.clone();
        if isInitial.clone() {
            initialStates = cons(cref.clone(), initialStates.clone());
        }
    }
    Ok(initialStates)
}

fn transitiveClosure(mut iTable: AdjacencyTable, mut nStates: i32) -> Result<AdjacencyTable> {
    let mut transClosure: AdjacencyTable;
    let mut cref2index: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut adjacency: metamodelica::Array<metamodelica::Array<bool>>;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut c: bool = false;
    let AdjacencyTable { cref2index: __pa0, adjacency: __pa1 } = (iTable.clone()) else { bail!("pattern mismatch") };
    cref2index = __pa0.clone();
    adjacency = __pa1.clone();
    n = BaseHashTable::hashTableCurrentSize(cref2index.clone());
    assert!(n.clone() == nStates.clone(), "{}", &*(literal!("Value of nStates needs to be equal to number of states within state table argument.")).clone());
    for mut k in 1..=n.clone() {
        for mut i in 1..=n.clone() {
            if adjacency.clone().borrow()[(i.clone()-1) as usize].clone().borrow()[(k.clone()-1) as usize].clone() {
                for mut j in 1..=n.clone() {
                    if adjacency.clone().borrow()[(k.clone()-1) as usize].clone().borrow()[(j.clone()-1) as usize].clone() {
                        {let _arr = adjacency.clone().borrow()[(i.clone()-1) as usize].clone(); _arr.borrow_mut()[(j.clone()-1) as usize] = true; _arr};
                    }
                }
            }
        }
    }
    transClosure = AdjacencyTable { cref2index: cref2index.clone(), adjacency: adjacency.clone() };
    Ok(transClosure)
}

fn createAdjacencyTable(mut smNodes: SMNodeTable, mut nStates: i32) -> Result<AdjacencyTable> {
    let mut iTable: AdjacencyTable;
    let mut cref2index: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut adjacency: metamodelica::Array<metamodelica::Array<bool>>;
    let mut iRow: metamodelica::Array<bool>;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut edges: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut crefs1: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut crefs2: metamodelica::Array<Arc<DAE::ComponentRef>>;
    crefs1 = metamodelica::arrayFromVec(BaseHashTable::hashTableKeyList(smNodes.clone()).into_iter().cloned().collect());
    n = (crefs1.clone().borrow().len() as i32);
    cref2index = HashTable::emptyHashTableSized(n.clone());
    assert!(n.clone() == nStates.clone(), "{}", &*(literal!("Value of nStates needs to be equal to number of modes within mode table argument.")).clone());
    adjacency = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<metamodelica::Array<bool>>> = metamodelica::nil();
        for mut i in (1..=n.clone()).into_iter() {
            let __x = arrayCreate(n.clone(), false);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
    for mut i in 1..=n.clone() {
        cref2index = BaseHashTable::addNoUpdCheck((crefs1.borrow()[(i.clone()-1) as usize].clone(), i.clone()), cref2index.clone())?;
    }
    for mut i in 1..=n.clone() {
        let SMNode { edges: __pa0, .. } = (BaseHashTable::get(crefs1.borrow()[(i.clone()-1) as usize].clone(), smNodes.clone())?) else { bail!("pattern mismatch") };
        edges = __pa0.clone();
        crefs2 = metamodelica::arrayFromVec(BaseHashSet::hashSetList(edges.clone())?.into_iter().cloned().collect());
        m = (crefs2.clone().borrow().len() as i32);
        for mut j in 1..=m.clone() {
            cref = crefs2.borrow()[(j.clone()-1) as usize].clone();
            k = BaseHashTable::get(cref.clone(), cref2index.clone())?;
            {let _arr = adjacency.clone().borrow()[(i.clone()-1) as usize].clone(); _arr.borrow_mut()[(k.clone()-1) as usize] = true; _arr};
        }
    }
    iTable = AdjacencyTable { cref2index: cref2index.clone(), adjacency: adjacency.clone() };
    Ok(iTable)
}

fn printAdjacencyTable(mut iTable: AdjacencyTable, mut nStates: i32) -> Result<()> {
    let mut cref2index: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut adjacency: metamodelica::Array<metamodelica::Array<bool>>;
    let mut entries: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut entry: (Arc<DAE::ComponentRef>, i32);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut padn: i32 = 0;
    let mut row: metamodelica::Array<bool>;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut pads: ArcStr = arcstr::literal!("");
    let mut b: bool = false;
    let AdjacencyTable { cref2index: __pa0, adjacency: __pa1 } = (iTable.clone()) else { bail!("pattern mismatch") };
    cref2index = __pa0.clone();
    adjacency = __pa1.clone();
    entries = BaseHashTable::hashTableList(cref2index.clone());
    n = (entries.clone().len() as i32);
    assert!(n.clone() == nStates.clone(), "{}", &*(literal!("Value of nStates needs to be equal to number of modes within state table argument.")).clone());
    entries = List::sort(entries.clone(), Arc::new(fnptr!(crefIndexCmp, (Arc<DAE::ComponentRef>, i32), (Arc<DAE::ComponentRef>, i32))))?;
    for mut entry in &*entries.clone() {
        let mut entry = entry.clone();
        (cref, i) = entry.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref.clone())?); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    pads = (literal!(" ")).clone();
    padn = 8;
    r#str = (Util::stringPadRight((literal!("i")).clone(), padn.clone(), (pads.clone()).clone())).clone();
    for mut i in 1..=n.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Util::stringPadLeft(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(",")); ArcStr::from(__mm_s) }).clone(), padn.clone(), (pads.clone()).clone())); ArcStr::from(__mm_s) }).clone();
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    for mut i in 1..=n.clone() {
        r#str = (Util::stringPadRight((intString(i.clone())).clone(), padn.clone(), (pads.clone()).clone())).clone();
        for mut j in 1..=n.clone() {
            b = adjacency.clone().borrow()[(i.clone()-1) as usize].clone().borrow()[(j.clone()-1) as usize].clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Util::stringPadLeft(({ let mut __mm_s = String::new(); __mm_s.push_str(&*boolString(b.clone())); __mm_s.push_str(&*literal!(",")); ArcStr::from(__mm_s) }).clone(), padn.clone(), (pads.clone()).clone())); ArcStr::from(__mm_s) }).clone();
        }
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn crefIndexCmp(mut inElement1: (Arc<DAE::ComponentRef>, i32), mut inElement2: (Arc<DAE::ComponentRef>, i32)) -> bool {
    let mut inRes: bool = false;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    (_, i1) = inElement1.clone();
    (_, i2) = inElement2.clone();
    inRes = i1.clone() > i2.clone();
    inRes
}

pub fn getSMNodeTable(mut elementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> SMNodeTable {
    let mut smNodeTable: SMNodeTable;
    let mut elementLst2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    elementLst2 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut e in (elementLst.clone()).into_iter().cloned() {
            if !(isSMStatement2(e.clone())) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if !(elementLst2.clone().is_empty()) {
        smNodeTable = List::fold(elementLst2.clone(), Arc::new(extractSMStates2), HashTableSM1::emptyHashTable());
    } else {
        smNodeTable = HashTableSM1::emptyHashTableSized(1);
    }
    smNodeTable
}

fn isSMStatement(mut inElement: Arc<SCode::Equation>) -> bool {
    let mut outIsSMStatement: bool = false;
    outIsSMStatement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name, .. }, .. }, .. } => {
            (name.clone() == literal!("transition") || name.clone() == literal!("initialState")) && Config::synchronousFeaturesAllowed()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsSMStatement
}

fn isSMStatement2(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outIsSMStatement: bool = false;
    outIsSMStatement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, .. } => {
            (name.clone() == literal!("transition") || name.clone() == literal!("initialState")) && Config::synchronousFeaturesAllowed()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsSMStatement
}

fn extractSMStates2(mut inElement: Arc<DAE::Element>, mut inTable: SMNodeTable) -> Result<SMNodeTable> {
    let mut outTable: SMNodeTable = inTable.clone();
    outTable = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: _ } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, .. }, .. } => {
            let mut smnode1: SMNode;
            let mut smnode2: SMNode;
            let mut isInitial1: bool = false;
            let mut isInitial2: bool = false;
            let mut edges1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut edges2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            smnode1 = if (BaseHashTable::hasKey(cref1.clone(), outTable.clone())) {BaseHashTable::get(cref1.clone(), outTable.clone())?} else {SMNode { componentRef: cref1.clone(), isInitial: false, edges: HashSet::emptyHashSet() }};
            let SMNode { componentRef: _, isInitial: __pa0, edges: __pa1 } = (smnode1.clone()) else { bail!("pattern mismatch") };
            isInitial1 = __pa0.clone();
            edges1 = __pa1.clone();
            edges1 = BaseHashSet::add(cref1.clone(), edges1.clone())?;
            edges1 = BaseHashSet::add(cref2.clone(), edges1.clone())?;
            smnode1 = SMNode { componentRef: cref1.clone(), isInitial: isInitial1.clone(), edges: edges1.clone() };
            outTable = BaseHashTable::add((cref1.clone(), smnode1.clone()), outTable.clone())?;
            smnode2 = if (BaseHashTable::hasKey(cref2.clone(), outTable.clone())) {BaseHashTable::get(cref2.clone(), outTable.clone())?} else {SMNode { componentRef: cref2.clone(), isInitial: false, edges: HashSet::emptyHashSet() }};
            let SMNode { componentRef: _, isInitial: __pa2, edges: __pa3 } = (smnode2.clone()) else { bail!("pattern mismatch") };
            isInitial2 = __pa2.clone();
            edges2 = __pa3.clone();
            edges2 = BaseHashSet::add(cref1.clone(), edges2.clone())?;
            edges2 = BaseHashSet::add(cref2.clone(), edges2.clone())?;
            smnode2 = SMNode { componentRef: cref2.clone(), isInitial: isInitial2.clone(), edges: edges2.clone() };
            outTable = BaseHashTable::add((cref2.clone(), smnode2.clone()), outTable.clone())?;
            outTable.clone()
        },
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, .. }, .. } => {
            let mut smnode1: SMNode;
            let mut edges1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            smnode1 = if (BaseHashTable::hasKey(cref1.clone(), outTable.clone())) {BaseHashTable::get(cref1.clone(), outTable.clone())?} else {SMNode { componentRef: cref1.clone(), isInitial: true, edges: HashSet::emptyHashSet() }};
            let SMNode { componentRef: _, isInitial: _, edges: __pa0 } = (smnode1.clone()) else { bail!("pattern mismatch") };
            edges1 = __pa0.clone();
            edges1 = BaseHashSet::add(cref1.clone(), edges1.clone())?;
            smnode1 = SMNode { componentRef: cref1.clone(), isInitial: true, edges: edges1.clone() };
            outTable = BaseHashTable::add((cref1.clone(), smnode1.clone()), outTable.clone())?;
            outTable.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTable)
}

pub fn getSMStatesInContext(mut eqns: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inPrefix: DAE::Prefix) -> (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) {
    let mut states: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut initialStates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut eqns1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut statesLL: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>> = metamodelica::nil();
    let mut initialStatesCR: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    let mut statesCR: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    eqns1 = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut eq in (eqns.clone()).into_iter().cloned() {
            if !(isSMStatement(eq.clone())) { continue; }
            let __x = eq.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    initialStatesCR = List::filterMap(eqns1.clone(), Arc::new(extractInitialSMStates));
    initialStates = List::map(initialStatesCR.clone(), Arc::new(ComponentReference::toExpCref));
    initialStates = List::map1(initialStates.clone(), Arc::new(prefixCrefNoContext2), inPrefix.clone());
    statesLL = List::map(eqns1.clone(), Arc::new(fnptr!(extractSMStates, Arc<SCode::Equation>)));
    statesCR = List::flatten(statesLL.clone());
    states = List::map(statesCR.clone(), Arc::new(ComponentReference::toExpCref));
    states = List::map(states.clone(), Arc::new({ let __pe_b0 = inPrefix.clone(); move |__pe_a1| PrefixUtil::prefixCrefNoContext(__pe_b0.clone(), __pe_a1) }));
    (states, initialStates)
}

fn prefixCrefNoContext2(mut inCref: Arc<DAE::ComponentRef>, mut inPre: DAE::Prefix) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = PrefixUtil::prefixCrefNoContext(inPre.clone(), inCref.clone())?;
    Ok(outCref)
}

fn extractInitialSMStates(mut inElement: Arc<SCode::Equation>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outElement: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, tail: Deref @ metamodelica::List::Nil }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "initialState", .. }, .. }, .. } => {
            cref1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

fn extractSMStates(mut inElement: Arc<SCode::Equation>) -> Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> {
    let mut outElement: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cref2 }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "transition", .. }, .. }, .. } => {
            list![cref1.clone(), cref2.clone()]
        },
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, tail: Deref @ metamodelica::List::Nil }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "initialState", .. }, .. }, .. } => {
            list![cref1.clone()]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElement
}

