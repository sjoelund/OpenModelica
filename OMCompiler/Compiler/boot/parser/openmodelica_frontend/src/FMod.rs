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

use crate::FCore;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub type Name = ArcStr;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = FCore::Node;

pub type Data = FCore::Data;

pub type Kind = FCore::Kind;

pub type Ref = metamodelica::Array<FCore::Node>;

pub type Refs = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Children = Arc<FCore::RefTree::Tree>;

pub type Parents = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Scope = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type ImportTable = FCore::ImportTable;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type ModScope = FCore::ModScope;

pub fn merge(mut inParentRef: Ref, mut inOuterModRef: Ref, mut inInnerModRef: Ref, mut inGraph: Graph) -> (Graph, Ref) {
    let mut outGraph: Graph;
    let mut outMergedModRef: Ref;
    (outGraph, outMergedModRef) = (match (inParentRef.clone(), inOuterModRef.clone(), inInnerModRef.clone(), inGraph.clone()) {
        (mut r, _, _, mut g) => {
            (g.clone(), r.clone())
        },
    });
    (outGraph, outMergedModRef)
}

pub fn apply(mut inTargetRef: Ref, mut inModRef: Ref, mut inGraph: Graph) -> (Graph, Ref) {
    let mut outGraph: Graph;
    let mut outNodeRef: Ref;
    (outGraph, outNodeRef) = (match (inTargetRef.clone(), inModRef.clone(), inGraph.clone()) {
        (mut r, _, mut g) => {
            (g.clone(), r.clone())
        },
    });
    (outGraph, outNodeRef)
}

pub fn compactSubMods(mut inSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inModScope: ModScope) -> Arc<metamodelica::List<Arc<SCode::SubMod>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    submods = List::fold2(inSubMods.clone(), Arc::new(compactSubMod), inModScope.clone(), metamodelica::nil(), metamodelica::nil());
    outSubMods = submods.clone().reverse();
    outSubMods
}

fn compactSubMod(mut inSubMod: Arc<SCode::SubMod>, mut inModScope: ModScope, mut inName: Arc<metamodelica::List<ArcStr>>, mut inAccumMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut found: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::SubMod { ident: __pa0, r#mod: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    (submods, found) = List::findMap(inAccumMods.clone(), Arc::new({ let __pe_b1 = inSubMod.clone(); let __pe_b2 = inModScope.clone(); let __pe_b3 = inName.clone(); move |__pe_a0| compactSubMod2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    outSubMods = List::consOnTrue(!(found.clone()), inSubMod.clone(), submods.clone());
    Ok(outSubMods)
}

fn compactSubMod2(mut inExistingMod: Arc<SCode::SubMod>, mut inNewMod: Arc<SCode::SubMod>, mut inModScope: ModScope, mut inName: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<SCode::SubMod>, bool)> {
    let mut outMod: Arc<SCode::SubMod>;
    let mut outFound: bool = false;
    (outMod, outFound) = 'mc: {
        let __mc_input = (inExistingMod.clone(), inNewMod.clone(), inModScope.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::SubMod { ident: name1, .. }, Deref @ SCode::SubMod { ident: name2, .. }, _, _) => {
                    let false = (stringEqual((name1.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((inExistingMod.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::SubMod { ident: name1, .. }, _, _, _) => {
                    let mut submod: Arc<SCode::SubMod>;
                    submod = mergeSubModsInSameScope(inExistingMod.clone(), inNewMod.clone(), cons(name1.clone(), inName.clone()), inModScope.clone())?;
                    Ok((submod.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMod, outFound))
}

fn mergeSubModsInSameScope(mut inMod1: Arc<SCode::SubMod>, mut inMod2: Arc<SCode::SubMod>, mut inElementName: Arc<metamodelica::List<ArcStr>>, mut inModScope: ModScope) -> Result<Arc<SCode::SubMod>> {
    let mut outMod: Arc<SCode::SubMod>;
    let mut scope: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut info1: SourceInfo;
    let mut info2: SourceInfo;
    let mut mod1: Arc<SCode::Mod> = inMod1.r#mod.clone();
    let mut mod2: Arc<SCode::Mod> = inMod2.r#mod.clone();
    outMod = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ SCode::Mod::MOD { .. }, Deref @ SCode::Mod::MOD { binding: None, .. }) => {
            submods = List::fold2(var_field!((*mod1).subModLst, SCode::Mod::MOD).clone(), Arc::new(compactSubMod), inModScope.clone(), inElementName.clone(), var_field!((*mod2).subModLst, SCode::Mod::MOD).clone());
            Arc::new(SCode::SubMod { ident: (inMod1.ident.clone()).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: var_field!((*mod1).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*mod1).eachPrefix, SCode::Mod::MOD).clone(), subModLst: submods.clone(), binding: var_field!((*mod1).binding, SCode::Mod::MOD).clone(), comment: var_field!((*mod1).comment, SCode::Mod::MOD).clone(), info: var_field!((*mod1).info, SCode::Mod::MOD).clone() }) })
        },
        (Deref @ SCode::Mod::MOD { binding: None, .. }, Deref @ SCode::Mod::MOD { .. }) => {
            submods = List::fold2(var_field!((*mod1).subModLst, SCode::Mod::MOD).clone(), Arc::new(compactSubMod), inModScope.clone(), inElementName.clone(), var_field!((*mod2).subModLst, SCode::Mod::MOD).clone());
            Arc::new(SCode::SubMod { ident: (inMod2.ident.clone()).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: var_field!((*mod2).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*mod2).eachPrefix, SCode::Mod::MOD).clone(), subModLst: submods.clone(), binding: var_field!((*mod2).binding, SCode::Mod::MOD).clone(), comment: var_field!((*mod2).comment, SCode::Mod::MOD).clone(), info: var_field!((*mod2).info, SCode::Mod::MOD).clone() }) })
        },
        _ => {
            info1 = SCodeUtil::getModifierInfo(mod1.clone());
            info2 = SCodeUtil::getModifierInfo(mod2.clone());
            scope = (printModScope(inModScope.clone())?).clone();
            name = stringDelimitList(inElementName.clone().reverse(), (literal!(".")).clone());
            Error::addMultiSourceMessage(Error::DUPLICATE_MODIFICATIONS.clone(), list![(name.clone()).clone(), (scope.clone()).clone()], list![info2.clone(), info1.clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn printModScope(mut inModScope: ModScope) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inModScope.clone() {
        FCore::ModScope::MS_COMPONENT { name: mut name } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }
        },
        FCore::ModScope::MS_EXTENDS { path: mut path } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("extends ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }
        },
        FCore::ModScope::MS_DERIVED { path: mut path } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("inherited class ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }
        },
        FCore::ModScope::MS_CLASS_EXTENDS { name: mut name } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("class extends class ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }
        },
        FCore::ModScope::MS_CONSTRAINEDBY { path: mut path } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("constrainedby class ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

