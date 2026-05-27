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
use crate::DAEUtil;
use crate::Expression;
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use crate::HashTable;
use crate::Lookup;
use crate::Types;
use crate::UnitAbsyn;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_script_util::UnitParserExt;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Flags;
use openmodelica_util::MMath;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn registerUnitWeights(mut cache: FCore::Cache, mut env: FCore::Graph, mut dae: DAE::DAElist) -> Result<()> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut du: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let _ = 'mc: {
        let __mc_input = (cache.clone(), env.clone(), dae.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, DAE::DAElist { elementLst: ref elts }) = __mc_input.clone() else { bail!("nomatch") };
            let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = paths.clone();
            let mut du: Arc<metamodelica::List<Arc<SCode::Element>>> = du.clone();
            paths = List::unionList(List::map(elts.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::getClassList, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> + 'static>)))?;
            du = List::unionList(List::map1(paths.clone(), (std::sync::Arc::new(retrieveUnitsFromEnv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, (FCore::Cache, FCore::Graph)) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> + 'static>), (cache.clone(), env.clone())))?;
            registerUnitWeightDefineunits(du.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn retrieveUnitsFromEnv(mut p: Arc<Absyn::Path>, mut tpl: (FCore::Cache, FCore::Graph)) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut du: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    du = 'mc: {
        let __mc_input = (p.clone(), tpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut env: FCore::Graph;
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut du: Arc<metamodelica::List<Arc<SCode::Element>>> = du.clone();
                    (_, _, env) = Lookup::lookupClass(Util::tuple21(tpl.clone()), Util::tuple22(tpl.clone()), p.clone(), None)?;
                    r = FGraph::lastScopeRef(env.clone())?;
                    r = FNode::child(r.clone(), (arcstr::literal!(FNode::duNodeName)).clone())?;
                    let FCore::N { data: FCore::DU { els: __pa0 }, .. } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    du = __pa0.clone();
                    Ok(du.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(du)
}

fn registerUnitWeightDefineunits(mut du: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = du.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    registerUnitWeightDefineunits2(list![Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("m")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("kg")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("s")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("A")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("k")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("mol")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("cd")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("rad")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("m/m")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("sr")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("m2/m2")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("Hz")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("s-1")).clone()), weight: Some(metamodelica::OrderedFloat(0.8_f64)), info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("N")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("m.kg.s-2")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("Pa")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("N/m2")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("W")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("J/s")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("J")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("N.m")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("C")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("s.A")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("V")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("W/A")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("F")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("C/V")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("Ohm")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("V/A")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("S")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("A/V")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("Wb")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("V.s")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("T")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("Wb/m2")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("H")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("Wb/A")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("lm")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("cd.sr")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("lx")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("lm/m2")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("Bq")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("s-1")).clone()), weight: Some(metamodelica::OrderedFloat(0.8_f64)), info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("Gy")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("J/kg")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("Sv")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("cd.sr")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() }), Arc::new(SCode::Element::DEFINEUNIT { name: (literal!("kat")).clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some((literal!("s-1.mol")).clone()), weight: None, info: SCodeUtil::dummyInfo.clone() })])?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    registerUnitWeightDefineunits2(du.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn registerUnitWeightDefineunits2(mut idu: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = idu.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::DEFINEUNIT { weight: Some(w), name: n, .. }, tail: du } => {
                    UnitParserExt::registerWeight((n.clone()).clone(), w.clone());
                    registerUnitWeightDefineunits2(du.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::DEFINEUNIT { weight: None, .. }, tail: du } => {
                    registerUnitWeightDefineunits2(du.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: du } => {
                    registerUnitWeightDefineunits2(du.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn registerUnits(mut prg: Absyn::Program) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = prg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (Flags::getConfigBool(Flags::UNIT_CHECKING.clone())?) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn registerUnitInClass(mut inTpl: (Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, i32)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, i32)> {
    let mut outTpl: (Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, i32);
    outTpl = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cl @ Deref @ Absyn::Class { .. }, pa, i) => {
                    let mut defunits: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
                    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    elts = AbsynUtil::getElementItemsInClass(cl.clone());
                    defunits = AbsynUtil::getDefineUnitsInElements(elts.clone());
                    registerDefineunits(defunits.clone())?;
                    Ok((cl.clone(), pa.clone(), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cl, pa, i) => {
                    Ok((cl.clone(), pa.clone(), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn registerDefineunits(mut elts: Arc<metamodelica::List<Arc<Absyn::Element>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = elts.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    registerDefineunits2(list![Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("m")).clone(), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("kg")).clone(), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("s")).clone(), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("A")).clone(), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("k")).clone(), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("mol")).clone(), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("cd")).clone(), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("rad")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("m/m")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("sr")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("m2/m2")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("Hz")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("s-1")).clone() }) }), Arc::new(Absyn::NamedArg { argName: (literal!("weight")).clone(), argValue: Arc::new(Absyn::Exp::REAL { value: (literal!("0.8")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("N")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("m.kg.s-2")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("Pa")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("N/m2")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("W")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("J/s")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("J")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("N.m")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("C")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("s.A")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("V")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("W/A")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("F")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("C/V")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("Ohm")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("V/A")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("S")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("A/V")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("Wb")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("V.s")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("T")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("Wb/m2")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("H")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("Wb/A")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("lm")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("cd.sr")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("lx")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("lm/m2")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("Bq")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("s-1")).clone() }) }), Arc::new(Absyn::NamedArg { argName: (literal!("weight")).clone(), argValue: Arc::new(Absyn::Exp::REAL { value: (literal!("0.8")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("Gy")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("J/kg")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("Sv")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("cd.sr")).clone() }) })], info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Element::DEFINEUNIT { name: (literal!("kat")).clone(), args: list![Arc::new(Absyn::NamedArg { argName: (literal!("exp")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (literal!("s-1.mol")).clone() }) })], info: Absyn::dummyInfo.clone() })])?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    registerDefineunits2(elts.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn registerDefineunits2(mut elts: Arc<metamodelica::List<Arc<Absyn::Element>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = elts.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: du @ Deref @ Absyn::Element::DEFINEUNIT { .. }, tail: rest } => {
                    let mut exp: ArcStr = arcstr::literal!("");
                    let mut name: ArcStr = arcstr::literal!("");
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynToSCode::translateElement(du.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC)?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::DEFINEUNIT { name: __pa0, visibility: _, exp: Some(__pa1), weight: _, .. }, tail: Deref @ metamodelica::List::Nil } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    exp = __pa1.clone();
                    UnitParserExt::addDerived((name.clone()).clone(), (exp.clone()).clone());
                    registerDefineunits2(rest.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: du @ Deref @ Absyn::Element::DEFINEUNIT { .. }, tail: rest } => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynToSCode::translateElement(du.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC)?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::DEFINEUNIT { name: __pa0, visibility: _, exp: None, weight: _, .. }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    UnitParserExt::addBase((name.clone()).clone());
                    registerDefineunits2(rest.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("registerDefineunits failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn add(mut unit: UnitAbsyn::Unit, mut ist: UnitAbsyn::Store) -> Result<(UnitAbsyn::Store, i32)> {
    let mut outSt: UnitAbsyn::Store;
    let mut index: i32 = 0;
    (outSt, index) = 'mc: {
        let __mc_input = (unit.clone(), ist.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, ref st @ UnitAbsyn::Store { numElts: ref numElts, storeVector: ref vector }) = __mc_input.clone() else { bail!("nomatch") };
            let mut st = st.clone();
            let mut index: i32 = index.clone();
            let true = (numElts.clone() == (vector.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
            st = expandStore(st.clone())?;
            (st, index) = add(unit.clone(), st.clone())?;
            Ok((st.clone(), index.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, UnitAbsyn::Store { numElts: mut numElts, storeVector: mut vector }) = __mc_input.clone() else { bail!("nomatch") };
            let mut newIndx: i32 = 0;
            newIndx = numElts.clone() + 1;
            vector = {let _arr = vector.clone(); _arr.borrow_mut()[(newIndx.clone()-1) as usize] = Some(unit.clone()); _arr};
            Ok((UnitAbsyn::Store { storeVector: vector.clone(), numElts: newIndx.clone() }, newIndx.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSt, index))
}

pub fn updateInstStore(mut store: UnitAbsyn::InstStore, mut st: UnitAbsyn::Store) -> Result<UnitAbsyn::InstStore> {
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    outStore = (match (store.clone(), st.clone()) {
        (UnitAbsyn::InstStore::INSTSTORE { store: _, ht: mut ht, checkResult: mut res }, _) => {
            UnitAbsyn::InstStore::INSTSTORE { store: st.clone(), ht: ht.clone(), checkResult: res.clone() }
        },
        (UnitAbsyn::InstStore::NOSTORE, _) => {
            crate::UnitAbsyn::InstStore::NOSTORE
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outStore)
}

fn expandStore(mut st: UnitAbsyn::Store) -> Result<UnitAbsyn::Store> {
    let mut outSt: UnitAbsyn::Store;
    outSt = (match st.clone() {
        UnitAbsyn::Store { storeVector: mut vector, numElts: mut indx } => {
            let mut incr: i32 = 0;
            incr = intMin(1, ((intReal(indx.clone()) * metamodelica::OrderedFloat(0.4_f64)).0 as i32));
            vector = Array::expand(incr.clone(), vector.clone(), None)?;
            UnitAbsyn::Store { storeVector: vector.clone(), numElts: indx.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outSt)
}

pub fn update(mut unit: UnitAbsyn::Unit, mut index: i32, mut st: UnitAbsyn::Store) -> Result<UnitAbsyn::Store> {
    let mut outSt: UnitAbsyn::Store;
    outSt = 'mc: {
        let __mc_input = (unit.clone(), index.clone(), st.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, UnitAbsyn::Store { storeVector: mut vector, numElts: mut indx }) = __mc_input.clone() else { bail!("nomatch") };
            vector = {let _arr = vector.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = Some(unit.clone()); _arr};
            Ok(UnitAbsyn::Store { storeVector: vector.clone(), numElts: indx.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("storing unit at index ")).clone());
            println!("{}", (intString(index.clone())).clone());
            println!("{}", (literal!(" failed\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSt)
}

pub fn find(mut index: i32, mut st: UnitAbsyn::Store) -> Result<UnitAbsyn::Unit> {
    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    unit = 'mc: {
        let __mc_input = (index.clone(), st.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, UnitAbsyn::Store { storeVector: mut vector, numElts: _ }) = __mc_input.clone() else { bail!("nomatch") };
            let mut unit: UnitAbsyn::Unit = unit.clone();
            let Some(__pa0) = (vector.borrow()[(index.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
            unit = __pa0.clone();
            Ok(unit.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!(" finding store at index ")).clone());
            println!("{}", (intString(index.clone())).clone());
            println!("{}", (literal!(" failed\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(unit)
}

pub fn instGetStore(mut store: UnitAbsyn::InstStore) -> Result<UnitAbsyn::Store> {
    let mut st: UnitAbsyn::Store;
    st = (match store.clone() {
        UnitAbsyn::InstStore::INSTSTORE { store: mut st, ht: _, checkResult: _ } => st.clone(),
        UnitAbsyn::InstStore::NOSTORE => emptyStore(),
        _ => bail!("match: no arm matched"),
    });
    Ok(st)
}

pub fn emptyInstStore() -> UnitAbsyn::InstStore {
    let mut st: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    st = emptyInstStore2(false);
    st
}

fn emptyInstStore2(mut wantInstStore: bool) -> UnitAbsyn::InstStore {
    let mut st: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    st = (match wantInstStore.clone() {
        true => {
            let mut s: UnitAbsyn::Store;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            s = emptyStore();
            ht = HashTable::emptyHashTable();
            UnitAbsyn::InstStore::INSTSTORE { store: s.clone(), ht: ht.clone(), checkResult: None }
        },
        _ => {
            UnitAbsyn::noStore().clone()
        },
    });
    st
}

pub fn emptyStore() -> UnitAbsyn::Store {
    let mut st: UnitAbsyn::Store;
    let mut vector: metamodelica::Array<Option<UnitAbsyn::Unit>>;
    vector = arrayCreate(10, None);
    st = UnitAbsyn::Store { storeVector: vector.clone(), numElts: 0 };
    st
}

pub fn printTerms(mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>) -> () {
    println!("{}", (printTermsStr(terms.clone())).clone());
    ()
}

pub fn printTermsStr(mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(terms.clone(), (std::sync::Arc::new(printTermStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<UnitAbsyn::UnitTerm>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    r#str
}

pub fn printTermStr(mut term: Arc<UnitAbsyn::UnitTerm>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(term.clone()) {
        Deref @ UnitAbsyn::UnitTerm::ADD { ut1: _, ut2: _, origExp: e } => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s1.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::SUB { ut1: _, ut2: _, origExp: e } => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s1.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::MUL { ut1: _, ut2: _, origExp: e } => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s1.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::DIV { ut1: _, ut2: _, origExp: e } => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s1.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::EQN { ut1: _, ut2: _, origExp: e } => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s1.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::LOC { loc: _, origExp: e } => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s1.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::POW { ut1: _, exponent: MMath::Rational { nom: _, denom: _ }, origExp: e } => {
            let mut s1: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s1.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn printInstStore(mut st: UnitAbsyn::InstStore) -> Result<()> {
    let _ = (match st.clone() {
        UnitAbsyn::InstStore::INSTSTORE { store: mut s, ht: mut h, checkResult: _ } => {
            println!("{}", (literal!("instStore, s:")).clone());
            printStore(s.clone())?;
            println!("{}", (literal!("\nht:")).clone());
            BaseHashTable::dumpHashTable(h.clone());
            ()
        },
        UnitAbsyn::InstStore::NOSTORE => {
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn printStore(mut st: UnitAbsyn::Store) -> Result<()> {
    let _ = (match st.clone() {
        UnitAbsyn::Store { storeVector: mut vector, numElts: _ } => {
            let mut lst: Arc<metamodelica::List<Option<UnitAbsyn::Unit>>> = metamodelica::nil();
            lst = Arc::new(vector.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            printStore2(lst.clone(), 1)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printStore2(mut lst: Arc<metamodelica::List<Option<UnitAbsyn::Unit>>>, mut indx: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((lst.clone(), indx.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Some(unit), tail: rest }, _) => {
            println!("{}", (intString(indx.clone())).clone());
            println!("{}", (literal!("->")).clone());
            printUnit(unit.clone())?;
            println!("{}", (literal!("\n")).clone());
            printStore2(rest.clone(), indx.clone() + 1)?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: None, tail: _ }, _) => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printUnit(mut unit: UnitAbsyn::Unit) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = unit.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: Deref @ metamodelica::List::Nil, units: baseunits } } => {
                    println!("{}", (printBaseUnitsStr(baseunits.clone())?).clone());
                    println!("{}", (literal!(" [")).clone());
                    println!("{}", (unit2str(unit.clone())?).clone());
                    println!("{}", (literal!("]")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: typeparams, units: baseunits } } => {
                    println!("{}", stringDelimitList(List::map(typeparams.clone(), (std::sync::Arc::new(printTypeParameterStr) as std::sync::Arc<dyn ::std::ops::Fn((MMath::Rational, UnitAbsyn::TypeParameter)) -> Result<ArcStr> + 'static>)), (literal!(",")).clone()));
                    println!("{}", (printBaseUnitsStr(baseunits.clone())?).clone());
                    println!("{}", (literal!(" [")).clone());
                    println!("{}", (unit2str(unit.clone())?).clone());
                    println!("{}", (literal!("]")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::Unit::UNSPECIFIED => {
                    println!("{}", (literal!("Unspecified")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printBaseUnitsStr(mut lst: Arc<metamodelica::List<MMath::Rational>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = lst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: MMath::Rational { nom: i1, denom: i2 }, tail: Deref @ metamodelica::List::Cons { head: MMath::Rational { nom: i3, denom: i4 }, tail: _ } } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("m^(")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(i2.clone())); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*literal!("s^(")); __mm_s.push_str(&*intString(i3.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(i4.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("printBaseUnitsStr failed len:")); __mm_s.push_str(&*intString((lst.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn printTypeParameterStr(mut typeParam: (MMath::Rational, UnitAbsyn::TypeParameter)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match typeParam.clone() {
        (MMath::Rational { nom: 0, denom: 0 }, UnitAbsyn::TypeParameter { name: mut name, indx: mut indx }) => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("[indx =")); __mm_s.push_str(&*intString(indx.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (MMath::Rational { nom: mut i1, denom: 1 }, UnitAbsyn::TypeParameter { name: mut name, indx: mut indx }) => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("^")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*literal!("[indx=")); __mm_s.push_str(&*intString(indx.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (MMath::Rational { nom: mut i1, denom: mut i2 }, UnitAbsyn::TypeParameter { name: mut name, indx: mut indx }) => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("^(")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(i2.clone())); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*literal!("[indx=")); __mm_s.push_str(&*intString(indx.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn splitRationals(mut inRationals: Arc<metamodelica::List<MMath::Rational>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut nums: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut denoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (nums, denoms) = (::match_deref::match_deref! { match &(inRationals.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: MMath::Rational { nom: i1, denom: i2 }, tail: rationals } => {
            (nums, denoms) = splitRationals(rationals.clone())?;
            (cons(i1.clone(), nums.clone()), cons(i2.clone(), denoms.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((nums, denoms))
}

pub fn joinRationals(mut inums: Arc<metamodelica::List<i32>>, mut idenoms: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<MMath::Rational>>> {
    let mut rationals: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
    rationals = (::match_deref::match_deref! { match &((inums.clone(), idenoms.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: i1, tail: nums }, Deref @ metamodelica::List::Cons { head: i2, tail: denoms }) => {
            rationals = joinRationals(nums.clone(), denoms.clone())?;
            cons(MMath::Rational { nom: i1.clone(), denom: i2.clone() }, rationals.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(rationals)
}

pub fn joinTypeParams(mut inums: Arc<metamodelica::List<i32>>, mut idenoms: Arc<metamodelica::List<i32>>, mut itpstrs: Arc<metamodelica::List<ArcStr>>, mut funcInstIdOpt: Option<i32>) -> Result<Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>> {
    let mut typeParams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
    typeParams = (::match_deref::match_deref! { match &((inums.clone(), idenoms.clone(), itpstrs.clone(), funcInstIdOpt.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: i1, tail: nums }, Deref @ metamodelica::List::Cons { head: i2, tail: denoms }, Deref @ metamodelica::List::Cons { head: tpParam, tail: tpstrs }, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut tpParam = (*tpParam).clone();
            typeParams = joinTypeParams(nums.clone(), denoms.clone(), tpstrs.clone(), funcInstIdOpt.clone())?;
            s = (Util::applyOptionOrDefault(funcInstIdOpt.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone())).clone();
            tpParam = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tpParam.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            cons((MMath::Rational { nom: i1.clone(), denom: i2.clone() }, UnitAbsyn::TypeParameter { name: (tpParam.clone()).clone(), indx: 0 }), typeParams.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(typeParams)
}

pub fn splitTypeParams(mut iTypeParams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut nums: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut denoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpstrs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (nums, denoms, tpstrs) = (::match_deref::match_deref! { match &(iTypeParams.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: (MMath::Rational { nom: i1, denom: i2 }, UnitAbsyn::TypeParameter { name: tpParam, indx: _ }), tail: typeParams } => {
            (nums, denoms, tpstrs) = splitTypeParams(typeParams.clone())?;
            (cons(i1.clone(), nums.clone()), cons(i2.clone(), denoms.clone()), cons((tpParam.clone()).clone(), tpstrs.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((nums, denoms, tpstrs))
}

pub fn instBuildUnitTerms(mut env: FCore::Graph, mut dae: DAE::DAElist, mut compDae: DAE::DAElist, mut store: UnitAbsyn::InstStore) -> Result<(UnitAbsyn::InstStore, Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>)> {
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    (outStore, terms) = 'mc: {
        let __mc_input = (env.clone(), dae.clone(), compDae.clone(), store.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, UnitAbsyn::InstStore::NOSTORE) = __mc_input.clone() else { bail!("nomatch") };
            Ok((crate::UnitAbsyn::InstStore::NOSTORE, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, UnitAbsyn::InstStore::INSTSTORE { store: mut st, ht: mut ht, checkResult: mut res }) = __mc_input.clone() else { bail!("nomatch") };
            let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
            (terms, st) = buildTerms(env.clone(), dae.clone(), ht.clone(), st.clone())?;
            (terms2, st) = buildTerms(env.clone(), compDae.clone(), ht.clone(), st.clone())?;
            terms = terms.clone().reverse();
            terms = List::append_reverse(terms2.clone(), terms.clone());
            st = createTypeParameterLocations(st.clone())?;
            Ok((UnitAbsyn::InstStore::INSTSTORE { store: st.clone(), ht: ht.clone(), checkResult: res.clone() }, terms.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("instBuildUnitTerms failed!!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStore, terms))
}

pub fn buildUnitTerms(mut env: FCore::Graph, mut dae: DAE::DAElist) -> Result<(Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, UnitAbsyn::Store, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))> {
    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut store: UnitAbsyn::Store;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    (store, ht) = buildStores(dae.clone())?;
    (terms, store) = buildTerms(env.clone(), dae.clone(), ht.clone(), store.clone())?;
    store = createTypeParameterLocations(store.clone())?;
    Ok((terms, store, ht))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn instAddStore(mut istore: UnitAbsyn::InstStore, mut itp: Arc<DAE::Type>, mut cr: Arc<DAE::ComponentRef>) -> Result<UnitAbsyn::InstStore> {
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    outStore = 'mc: {
        let __mc_input = (istore.clone(), itp.clone(), cr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (UnitAbsyn::InstStore::NOSTORE, _, _) => {
                    Ok(istore.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (UnitAbsyn::InstStore::INSTSTORE { store: st, ht, checkResult: res }, Deref @ DAE::Type::T_REAL { varLst }, _) => {
                    let mut unitStr: ArcStr = arcstr::literal!("");
                    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut indx: i32 = 0;
                    let mut st = (*st).clone();
                    let mut ht = (*ht).clone();
                    let mut outStore: UnitAbsyn::InstStore = outStore.clone();
                    for mut v in &*varLst.clone() {
                        let mut v = v.clone();
                        let _ = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::SCONST { string: unitStr }, .. }, name: Deref @ "unit", .. } => {
                    unit = str2unit((unitStr.clone()).clone(), None)?;
                    unit = if (0 == stringCompare((unitStr.clone()).clone(), (literal!("")).clone())) {crate::UnitAbsyn::Unit::UNSPECIFIED} else {unit.clone()};
                    (st, indx) = add(unit.clone(), st.clone())?;
                    ht = BaseHashTable::add((cr.clone(), indx.clone()), ht.clone())?;
                    outStore = UnitAbsyn::InstStore::INSTSTORE { store: st.clone(), ht: ht.clone(), checkResult: res.clone() };
                    return Ok(outStore);
                    ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    }
                    (st, indx) = add(crate::UnitAbsyn::Unit::UNSPECIFIED, st.clone())?;
                    (ht, _, _, _) = BaseHashTable::add((cr.clone(), indx.clone()), ht.clone())?;
                    Ok(UnitAbsyn::InstStore::INSTSTORE { store: st.clone(), ht: ht.clone(), checkResult: res.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (store, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. }, _) => {
                    Ok(instAddStore(store.clone(), tp.clone(), cr.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(istore.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStore)
}

pub fn storeSize(mut store: UnitAbsyn::Store) -> Result<i32> {
    let mut size: i32 = 0;
    size = (match store.clone() {
        UnitAbsyn::Store { storeVector: _, numElts: mut size } => size.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(size)
}

fn createTypeParameterLocations(mut store: UnitAbsyn::Store) -> Result<UnitAbsyn::Store> {
    let mut outStore: UnitAbsyn::Store;
    let mut nextElement: i32 = 0;
    let mut storeSz: i32 = 0;
    storeSz = storeSize(store.clone())?;
    (outStore, _, nextElement) = createTypeParameterLocations2(store.clone(), HashTable::emptyHashTable(), 1, storeSz.clone() + 1)?;
    outStore = addUnspecifiedStores(nextElement.clone() - storeSz.clone() - 1, outStore.clone())?;
    Ok(outStore)
}

fn addUnspecifiedStores(mut n: i32, mut istore: UnitAbsyn::Store) -> Result<UnitAbsyn::Store> {
    let mut outStore: UnitAbsyn::Store;
    outStore = 'mc: {
        let __mc_input = (n.clone(), istore.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (0, mut store) = __mc_input.clone() else { bail!("nomatch") };
            Ok(store.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (n.clone() < 0) else { bail!("pattern mismatch") };
            println!("{}", (literal!("addUnspecifiedStores n < 0!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut store) = __mc_input.clone() else { bail!("nomatch") };
            let true = (n.clone() > 0) else { bail!("pattern mismatch") };
            (store, _) = add(crate::UnitAbsyn::Unit::UNSPECIFIED, store.clone())?;
            store = addUnspecifiedStores(n.clone() - 1, store.clone())?;
            Ok(store.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStore)
}

fn createTypeParameterLocations2(mut istore: UnitAbsyn::Store, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut i: i32, mut inextElt: i32) -> Result<(UnitAbsyn::Store, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), i32)> {
    let mut outStore: UnitAbsyn::Store;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut outNextElt: i32 = 0;
    (outStore, outHt, outNextElt) = 'mc: {
        let __mc_input = (istore.clone(), iht.clone(), i.clone(), inextElt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (ref store @ UnitAbsyn::Store { storeVector: _, numElts: ref numElts }, mut ht, _, mut nextElt) = __mc_input.clone() else { bail!("nomatch") };
            let true = (i.clone() > numElts.clone()) else { bail!("pattern mismatch") };
            Ok((store.clone(), ht.clone(), nextElt.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (UnitAbsyn::Store { storeVector: mut vect, numElts: mut numElts }, mut ht, _, mut nextElt) = __mc_input.clone() else { bail!("nomatch") };
            let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
            let mut store: UnitAbsyn::Store;
            let Some(__pa0) = (vect.borrow()[(i.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
            unit = __pa0.clone();
            (unit, ht, nextElt) = createTypeParameterLocations3(unit.clone(), ht.clone(), nextElt.clone())?;
            vect = {let _arr = vect.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = Some(unit.clone()); _arr};
            (store, ht, nextElt) = createTypeParameterLocations2(UnitAbsyn::Store { storeVector: vect.clone(), numElts: numElts.clone() }, ht.clone(), i.clone() + 1, nextElt.clone())?;
            Ok((store.clone(), ht.clone(), nextElt.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (UnitAbsyn::Store { storeVector: mut vect, numElts: mut numElts }, mut ht, _, mut nextElt) = __mc_input.clone() else { bail!("nomatch") };
            let mut store: UnitAbsyn::Store;
            (store, ht, nextElt) = createTypeParameterLocations2(UnitAbsyn::Store { storeVector: vect.clone(), numElts: numElts.clone() }, ht.clone(), i.clone() + 1, nextElt.clone())?;
            Ok((store.clone(), ht.clone(), nextElt.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStore, outHt, outNextElt))
}

fn createTypeParameterLocations3(mut unit: UnitAbsyn::Unit, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut inextElt: i32) -> Result<(UnitAbsyn::Unit, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), i32)> {
    let mut outUnit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut outNextElt: i32 = 0;
    (outUnit, outHt, outNextElt) = (::match_deref::match_deref! { match &((unit.clone(), iht.clone(), inextElt.clone())) {
        (UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: params @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, units } }, ht, nextElt) => {
            let mut params = (*params).clone();
            let mut ht = (*ht).clone();
            let mut nextElt = (*nextElt).clone();
            (params, ht, nextElt) = createTypeParameterLocations4(params.clone(), ht.clone(), nextElt.clone())?;
            (UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: params.clone(), units: units.clone() } }, ht.clone(), nextElt.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outUnit, outHt, outNextElt))
}

fn createTypeParameterLocations4(mut iparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut inextElt: i32) -> Result<(Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), i32)> {
    let mut outParams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut outNextElt: i32 = 0;
    (outParams, outHt, outNextElt) = 'mc: {
        let __mc_input = (iparams.clone(), iht.clone(), inextElt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, ht, nextElt) => {
                    Ok((metamodelica::nil(), ht.clone(), nextElt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (r, UnitAbsyn::TypeParameter { name, indx: 0 }), tail: params }, ht, nextElt) => {
                    let mut indx: i32 = 0;
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut params = (*params).clone();
                    let mut ht = (*ht).clone();
                    let mut nextElt = (*nextElt).clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    indx = BaseHashTable::get(cref_.clone(), ht.clone())?;
                    (params, ht, nextElt) = createTypeParameterLocations4(params.clone(), ht.clone(), nextElt.clone())?;
                    Ok((cons((r.clone(), UnitAbsyn::TypeParameter { name: (name.clone()).clone(), indx: indx.clone() }), params.clone()), ht.clone(), nextElt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (r, UnitAbsyn::TypeParameter { name, indx: 0 }), tail: params }, ht, nextElt) => {
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut params = (*params).clone();
                    let mut ht = (*ht).clone();
                    let mut nextElt = (*nextElt).clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    (ht, _, _, _) = BaseHashTable::add((cref_.clone(), nextElt.clone()), ht.clone())?;
                    (params, ht, nextElt) = createTypeParameterLocations4(params.clone(), ht.clone(), nextElt.clone())?;
                    Ok((cons((r.clone(), UnitAbsyn::TypeParameter { name: (name.clone()).clone(), indx: nextElt.clone() }), params.clone()), ht.clone(), nextElt.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: param, tail: params }, ht, nextElt) => {
                    let mut params = (*params).clone();
                    let mut ht = (*ht).clone();
                    let mut nextElt = (*nextElt).clone();
                    (params, ht, nextElt) = createTypeParameterLocations4(params.clone(), ht.clone(), nextElt.clone())?;
                    Ok((cons(param.clone(), params.clone()), ht.clone(), nextElt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("createTypeParameterLocations4 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outParams, outHt, outNextElt))
}

fn buildStores(mut dae: DAE::DAElist) -> Result<(UnitAbsyn::Store, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))> {
    let mut store: UnitAbsyn::Store;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    (store, ht) = buildStores2(dae.clone(), emptyStore(), HashTable::emptyHashTable())?;
    (store, ht) = buildStores3(dae.clone(), store.clone(), ht.clone())?;
    Ok((store, ht))
}

fn buildTerms(mut env: FCore::Graph, mut dae: DAE::DAElist, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut istore: UnitAbsyn::Store) -> Result<(Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, UnitAbsyn::Store)> {
    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (terms, outStore) = 'mc: {
        let __mc_input = (env.clone(), dae.clone(), ht.clone(), istore.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Nil }, _, store) => {
                    Ok((metamodelica::nil(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: _ }, tail: elts } }, _, store) => {
                    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
                    let mut ut2: Arc<UnitAbsyn::UnitTerm>;
                    let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
                    (ut1, terms1, store) = buildTermExp(env.clone(), e1.clone(), false, ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env.clone(), e2.clone(), false, ht.clone(), store.clone())?;
                    (terms, store) = buildTerms(env.clone(), DAE::DAElist { elementLst: elts.clone() }, ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), listAppend(terms2.clone(), terms.clone()));
                    Ok((cons(Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }) }), terms.clone()), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source: _ }, tail: elts } }, _, store) => {
                    let mut crefExp1: Arc<DAE::Exp>;
                    let mut crefExp2: Arc<DAE::Exp>;
                    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
                    let mut ut2: Arc<UnitAbsyn::UnitTerm>;
                    let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
                    crefExp1 = Expression::crefExp(cr1.clone())?;
                    crefExp2 = Expression::crefExp(cr2.clone())?;
                    (ut1, terms1, store) = buildTermExp(env.clone(), crefExp1.clone(), false, ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env.clone(), crefExp2.clone(), false, ht.clone(), store.clone())?;
                    (terms, store) = buildTerms(env.clone(), DAE::DAElist { elementLst: elts.clone() }, ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), listAppend(terms2.clone(), terms.clone()));
                    Ok((cons(Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: Arc::new(DAE::Exp::BINARY { exp1: crefExp1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: crefExp2.clone() }) }), terms.clone()), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: Some(e1), componentRef: cr1 @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, .. }, tail: elts } }, _, store) => {
                    let mut crefExp1: Arc<DAE::Exp>;
                    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
                    let mut ut2: Arc<UnitAbsyn::UnitTerm>;
                    let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
                    crefExp1 = Expression::crefExp(cr1.clone())?;
                    (ut1, terms1, store) = buildTermExp(env.clone(), crefExp1.clone(), false, ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env.clone(), e1.clone(), false, ht.clone(), store.clone())?;
                    (terms, store) = buildTerms(env.clone(), DAE::DAElist { elementLst: elts.clone() }, ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), listAppend(terms2.clone(), terms.clone()));
                    Ok((cons(Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: Arc::new(DAE::Exp::BINARY { exp1: crefExp1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e1.clone() }) }), terms.clone()), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr1, exp: e1, source: _ }, tail: elts } }, _, store) => {
                    let mut crefExp1: Arc<DAE::Exp>;
                    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
                    let mut ut2: Arc<UnitAbsyn::UnitTerm>;
                    let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
                    crefExp1 = Expression::crefExp(cr1.clone())?;
                    (ut1, terms1, store) = buildTermExp(env.clone(), crefExp1.clone(), false, ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env.clone(), e1.clone(), false, ht.clone(), store.clone())?;
                    (terms, store) = buildTerms(env.clone(), DAE::DAElist { elementLst: elts.clone() }, ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), listAppend(terms2.clone(), terms.clone()));
                    Ok((cons(Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: Arc::new(DAE::Exp::BINARY { exp1: crefExp1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e1.clone() }) }), terms.clone()), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: _, tail: elts } }, _, store) => {
                    let mut store = (*store).clone();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
                    (terms, store) = buildTerms(env.clone(), DAE::DAElist { elementLst: elts.clone() }, ht.clone(), store.clone())?;
                    Ok((terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((terms, outStore))
}

fn buildTermExp(mut env: FCore::Graph, mut exp: Arc<DAE::Exp>, mut idivOrMul: bool, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut istore: UnitAbsyn::Store) -> Result<(Arc<UnitAbsyn::UnitTerm>, Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, UnitAbsyn::Store)> {
    let mut ut: Arc<UnitAbsyn::UnitTerm>;
    let mut extraTerms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (ut, extraTerms, outStore) = 'mc: {
        let __mc_input = (env.clone(), exp.clone(), idivOrMul.clone(), iht.clone(), istore.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::ICONST { integer: i }, divOrMul, ht, store) => {
                    let mut indx: i32 = 0;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut u: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut ht = (*ht).clone();
                    let mut store = (*store).clone();
                    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*intString(tick())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone();
                    u = if (divOrMul.clone()) {str2unit((literal!("1")).clone(), None)?} else {crate::UnitAbsyn::Unit::UNSPECIFIED};
                    (store, indx) = add(u.clone(), store.clone())?;
                    (ht, _, _, _) = BaseHashTable::add((ComponentReferenceBasics::makeCrefIdent((s1.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()), indx.clone()), ht.clone())?;
                    Ok((Arc::new(UnitAbsyn::UnitTerm::LOC { loc: indx.clone(), origExp: e.clone() }), metamodelica::nil(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::RCONST { real: r }, divOrMul, ht, store) => {
                    let mut indx: i32 = 0;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut u: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut ht = (*ht).clone();
                    let mut store = (*store).clone();
                    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*intString(tick())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*realString(r.clone())); ArcStr::from(__mm_s) }).clone();
                    u = if (divOrMul.clone()) {str2unit((literal!("1")).clone(), None)?} else {crate::UnitAbsyn::Unit::UNSPECIFIED};
                    (store, indx) = add(u.clone(), store.clone())?;
                    (ht, _, _, _) = BaseHashTable::add((ComponentReferenceBasics::makeCrefIdent((s1.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()), indx.clone()), ht.clone())?;
                    Ok((Arc::new(UnitAbsyn::UnitTerm::LOC { loc: indx.clone(), origExp: e.clone() }), metamodelica::nil(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CAST { ty: _, exp: e1 }, divOrMul, ht, store) => {
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    (ut, terms, store) = buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    Ok((ut.clone(), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, _, ht, store) => {
                    let mut indx: i32 = 0;
                    indx = BaseHashTable::get(cr.clone(), ht.clone())?;
                    Ok((Arc::new(UnitAbsyn::UnitTerm::LOC { loc: indx.clone(), origExp: e.clone() }), metamodelica::nil(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::ICONST { integer: i } }, divOrMul, ht, store) => {
                    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
                    let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    (ut1, terms1, store) = buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    (_, terms2, store) = buildTermExp(env.clone(), e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), terms2.clone());
                    ut = Arc::new(UnitAbsyn::UnitTerm::POW { ut1: ut1.clone(), exponent: MMath::Rational { nom: i.clone(), denom: 1 }, origExp: e.clone() });
                    Ok((ut.clone(), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::RCONST { real: r } }, divOrMul, ht, store) => {
                    let mut i: i32 = 0;
                    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
                    let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    (ut1, terms1, store) = buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    (_, terms2, store) = buildTermExp(env.clone(), e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), terms2.clone());
                    i = ((r.clone()).0 as i32);
                    let true = (intReal(i.clone()) - r.clone() == metamodelica::OrderedFloat(0.0_f64)) else { bail!("pattern mismatch") };
                    ut = Arc::new(UnitAbsyn::UnitTerm::POW { ut1: ut1.clone(), exponent: MMath::Rational { nom: i.clone(), denom: 1 }, origExp: e.clone() });
                    Ok((ut.clone(), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, divOrMul, ht, store) => {
                    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
                    let mut ut2: Arc<UnitAbsyn::UnitTerm>;
                    let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    divOrMul = Expression::operatorDivOrMul(op.clone());
                    (ut1, terms1, store) = buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env.clone(), e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), terms2.clone());
                    ut = buildTermOp(ut1.clone(), ut2.clone(), op.clone(), e.clone())?;
                    Ok((ut.clone(), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: _ }, divOrMul, ht, store) => {
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    divOrMul = Expression::operatorDivOrMul(op.clone());
                    (ut, terms, store) = buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    if '__try0: {
                        (_, _, _) = unwrap_break_err!(buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((ut.clone(), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, divOrMul, ht, store) => {
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    divOrMul = Expression::operatorDivOrMul(op.clone());
                    if '__try0: {
                        (_, _, _) = unwrap_break_err!(buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (ut, terms, store) = buildTermExp(env.clone(), e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    Ok((ut.clone(), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: _, exp: e1 }, divOrMul, ht, store) => {
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    (ut, terms, store) = buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    Ok((ut.clone(), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::IFEXP { expCond: _, expThen: e1, expElse: e2 }, divOrMul, ht, store) => {
                    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
                    let mut ut2: Arc<UnitAbsyn::UnitTerm>;
                    let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    divOrMul = false;
                    (ut1, terms1, store) = buildTermExp(env.clone(), e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env.clone(), e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), terms2.clone());
                    Ok((Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: e.clone() }), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::CALL { expLst: expl, path, .. }, divOrMul, ht, store) => {
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    divOrMul = false;
                    (ut, terms, store) = buildTermCall(env.clone(), path.clone(), e.clone(), expl.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    Ok((ut.clone(), terms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl }, _, ht, store) => {
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut uts: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("vector =")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    (uts, terms, store) = buildTermExpList(env.clone(), expl.clone(), ht.clone(), store.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(buildArrayElementTerms(uts.clone(), expl.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    uts = __pa1.clone();
                    uts = listAppend(terms.clone(), uts.clone());
                    Ok((ut.clone(), uts.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. }, _, ht, store) => {
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut uts: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut store = (*store).clone();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Matrix =")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    expl = List::flatten(mexpl.clone());
                    (uts, terms, store) = buildTermExpList(env.clone(), expl.clone(), ht.clone(), store.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(buildArrayElementTerms(uts.clone(), expl.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    uts = __pa1.clone();
                    uts = listAppend(terms.clone(), uts.clone());
                    Ok((ut.clone(), uts.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ DAE::Exp::CALL { .. }, _, _, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildTermDAE.CALL failed exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((ut, extraTerms, outStore))
}

fn buildArrayElementTerms(mut iuts: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, mut iexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>> {
    let mut outUts: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut rest_ut: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = iuts.clone();
    let mut ut1: Arc<UnitAbsyn::UnitTerm>;
    let mut ut2: Arc<UnitAbsyn::UnitTerm>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut rest_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = iexpl.clone();
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    while !(rest_ut.clone().is_empty()) {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(rest_ut.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ut1 = __pa0.clone();
        ut2 = __pa1.clone();
        rest_ut = __pa2.clone();
        let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(rest_expl.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: __pa6 } } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e1 = __pa4.clone();
        e2 = __pa5.clone();
        rest_expl = __pa6.clone();
        ty = Expression::r#typeof(e1.clone())?;
        outUts = cons(Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: list![e1.clone(), e2.clone()] }) }), outUts.clone());
    }
    outUts = outUts.clone().reverse();
    Ok(outUts)
}

fn buildTermCall(mut env: FCore::Graph, mut path: Arc<Absyn::Path>, mut funcCallExp: Arc<DAE::Exp>, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut divOrMul: bool, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut istore: UnitAbsyn::Store) -> Result<(Arc<UnitAbsyn::UnitTerm>, Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, UnitAbsyn::Store)> {
    let mut ut: Arc<UnitAbsyn::UnitTerm>;
    let mut extraTerms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (ut, extraTerms, outStore) = (::match_deref::match_deref! { match &((env.clone(), path.clone(), funcCallExp.clone(), expl.clone(), divOrMul.clone(), ht.clone(), istore.clone())) {
        (_, _, _, _, _, _, store) => {
            let mut formalParamIndxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut funcInstId: i32 = 0;
            let mut actTermLst: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut extraTerms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut functp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut store = (*store).clone();
            (_, functp, _) = Lookup::lookupType(FCore::noCache(), env.clone(), path.clone(), None)?;
            funcInstId = tick();
            (store, formalParamIndxs) = buildFuncTypeStores(functp.clone(), funcInstId.clone(), store.clone())?;
            (actTermLst, extraTerms, store) = buildTermExpList(env.clone(), expl.clone(), ht.clone(), store.clone())?;
            terms = buildFormal2ActualParamTerms(formalParamIndxs.clone(), actTermLst.clone())?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(buildResultTerms(functp.clone(), funcInstId.clone(), funcCallExp.clone(), store.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ut = __pa0.clone();
            extraTerms2 = __pa1.clone();
            store = __pa2.clone();
            extraTerms = List::flatten(list![extraTerms.clone(), extraTerms2.clone(), terms.clone()]);
            (ut.clone(), extraTerms.clone(), store.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ut, extraTerms, outStore))
}

fn buildResultTerms(mut ifunctp: Arc<DAE::Type>, mut funcInstId: i32, mut funcCallExp: Arc<DAE::Exp>, mut istore: UnitAbsyn::Store) -> Result<(Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, UnitAbsyn::Store)> {
    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut extraTerms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (terms, extraTerms, outStore) = 'mc: {
        let __mc_input = (ifunctp.clone(), funcInstId.clone(), funcCallExp.clone(), istore.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: _, funcResultType: functp, functionAttributes: _, path: _ }, _, _, store) => {
                    let mut unitStr: ArcStr = arcstr::literal!("");
                    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut indx: i32 = 0;
                    let mut indx2: i32 = 0;
                    let mut unspec: bool = false;
                    let mut store = (*store).clone();
                    unitStr = (getUnitStr(functp.clone())?).clone();
                    unspec = 0 == stringCompare((unitStr.clone()).clone(), (literal!("")).clone());
                    unit = str2unit((unitStr.clone()).clone(), Some(funcInstId.clone()))?;
                    unit = if (unspec.clone()) {crate::UnitAbsyn::Unit::UNSPECIFIED} else {unit.clone()};
                    (store, indx) = add(unit.clone(), store.clone())?;
                    (store, indx2) = add(crate::UnitAbsyn::Unit::UNSPECIFIED, store.clone())?;
                    Ok((list![Arc::new(UnitAbsyn::UnitTerm::LOC { loc: indx2.clone(), origExp: funcCallExp.clone() })], list![Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: Arc::new(UnitAbsyn::UnitTerm::LOC { loc: indx2.clone(), origExp: funcCallExp.clone() }), ut2: Arc::new(UnitAbsyn::UnitTerm::LOC { loc: indx.clone(), origExp: funcCallExp.clone() }), origExp: funcCallExp.clone() })], store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_TUPLE { types: typeLst, .. }, .. }, _, _, store) => {
                    let mut store = (*store).clone();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
                    let mut extraTerms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = extraTerms.clone();
                    (terms, extraTerms, store) = buildTupleResultTerms(typeLst.clone(), funcInstId.clone(), funcCallExp.clone(), store.clone())?;
                    Ok((terms.clone(), extraTerms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("buildResultTerms failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((terms, extraTerms, outStore))
}

fn buildTupleResultTerms(mut ifunctps: Arc<metamodelica::List<Arc<DAE::Type>>>, mut funcInstId: i32, mut funcCallExp: Arc<DAE::Exp>, mut istore: UnitAbsyn::Store) -> Result<(Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, UnitAbsyn::Store)> {
    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut extraTerms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (terms, extraTerms, outStore) = (::match_deref::match_deref! { match &((ifunctps.clone(), funcInstId.clone(), funcCallExp.clone(), istore.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, store) => {
            (metamodelica::nil(), metamodelica::nil(), store.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tp, tail: functps }, _, _, store) => {
            let mut terms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut terms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut extraTerms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut extraTerms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut store = (*store).clone();
            (terms1, extraTerms1, store) = buildResultTerms(tp.clone(), funcInstId.clone(), funcCallExp.clone(), store.clone())?;
            (terms2, extraTerms2, store) = buildTupleResultTerms(functps.clone(), funcInstId.clone(), funcCallExp.clone(), store.clone())?;
            terms = listAppend(terms1.clone(), terms2.clone());
            extraTerms = listAppend(extraTerms1.clone(), extraTerms2.clone());
            (terms.clone(), extraTerms.clone(), store.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((terms, extraTerms, outStore))
}

fn buildTermExpList(mut env: FCore::Graph, mut iexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut istore: UnitAbsyn::Store) -> Result<(Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, UnitAbsyn::Store)> {
    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut extraTerms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (terms, extraTerms, outStore) = 'mc: {
        let __mc_input = (env.clone(), iexpl.clone(), ht.clone(), istore.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, store) => {
                    Ok((metamodelica::nil(), metamodelica::nil(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: e, tail: expl }, _, store) => {
                    let mut eterms1: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut eterms2: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
                    let mut ut: Arc<UnitAbsyn::UnitTerm>;
                    let mut store = (*store).clone();
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
                    let mut extraTerms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = extraTerms.clone();
                    (ut, eterms1, store) = buildTermExp(env.clone(), e.clone(), false, ht.clone(), store.clone())?;
                    (terms, eterms2, store) = buildTermExpList(env.clone(), expl.clone(), ht.clone(), store.clone())?;
                    extraTerms = listAppend(eterms1.clone(), eterms2.clone());
                    Ok((cons(ut.clone(), terms.clone()), extraTerms.clone(), store.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: e, tail: _ }, _, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildTermExpList failed for exp")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((terms, extraTerms, outStore))
}

fn buildFuncTypeStores(mut funcType: Arc<DAE::Type>, mut funcInstId: i32, mut istore: UnitAbsyn::Store) -> Result<(UnitAbsyn::Store, Arc<metamodelica::List<i32>>)> {
    let mut outStore: UnitAbsyn::Store;
    let mut indxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outStore, indxs) = 'mc: {
        let __mc_input = (funcType.clone(), funcInstId.clone(), istore.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: args, .. }, _, store) => {
                    let mut store = (*store).clone();
                    let mut indxs: Arc<metamodelica::List<i32>> = indxs.clone();
                    (store, indxs) = buildFuncTypeStores2(args.clone(), funcInstId.clone(), store.clone())?;
                    Ok((store.clone(), indxs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tp, _, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildFuncTypeStores failed, tp")); __mm_s.push_str(&*TypesDump::unparseType(tp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStore, indxs))
}

fn buildFuncTypeStores2(mut ifargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut funcInstId: i32, mut istore: UnitAbsyn::Store) -> Result<(UnitAbsyn::Store, Arc<metamodelica::List<i32>>)> {
    let mut outStore: UnitAbsyn::Store;
    let mut indxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outStore, indxs) = (::match_deref::match_deref! { match &((ifargs.clone(), funcInstId.clone(), istore.clone())) {
        (Deref @ metamodelica::List::Nil, _, store) => {
            (store.clone(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { ty: tp, .. }, tail: fargs }, _, store) => {
            let mut unitStr: ArcStr = arcstr::literal!("");
            let mut indx: i32 = 0;
            let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
            let mut store = (*store).clone();
            unitStr = (getUnitStr(tp.clone())?).clone();
            unit = str2unit((unitStr.clone()).clone(), Some(funcInstId.clone()))?;
            unit = if (0 == stringCompare((unitStr.clone()).clone(), (literal!("")).clone())) {crate::UnitAbsyn::Unit::UNSPECIFIED} else {unit.clone()};
            (store, indx) = add(unit.clone(), store.clone())?;
            (store, indxs) = buildFuncTypeStores2(fargs.clone(), funcInstId.clone(), store.clone())?;
            (store.clone(), cons(indx.clone(), indxs.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outStore, indxs))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getUnitStr(mut itp: Arc<DAE::Type>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = itp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst } => {
                    for mut v in &*varLst.clone() {
                        let mut v = v.clone();
                        let _ = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::SCONST { string: r#str }, .. }, name: Deref @ "unit", .. } => {
                    return Ok(r#str);
                    ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    }
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: tp, .. } => {
                    Ok(getUnitStr(tp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                tp => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getUnitStr for type ")); __mm_s.push_str(&*TypesDump::unparseType(tp.clone())?); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn buildFormal2ActualParamTerms(mut iformalParamIndxs: Arc<metamodelica::List<i32>>, mut iactualParamIndxs: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>) -> Result<Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>> {
    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    terms = 'mc: {
        let __mc_input = (iformalParamIndxs.clone(), iactualParamIndxs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: loc1, tail: formalParamIndxs }, Deref @ metamodelica::List::Cons { head: ut, tail: actualParamIndxs }) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut terms: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = terms.clone();
                    terms = buildFormal2ActualParamTerms(formalParamIndxs.clone(), actualParamIndxs.clone())?;
                    e = origExpInTerm(ut.clone())?;
                    Ok(cons(Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: Arc::new(UnitAbsyn::UnitTerm::LOC { loc: loc1.clone(), origExp: e.clone() }), ut2: ut.clone(), origExp: e.clone() }), terms.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("buildFormal2ActualParamTerms failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(terms)
}

fn origExpInTerm(mut ut: Arc<UnitAbsyn::UnitTerm>) -> Result<Arc<DAE::Exp>> {
    let mut origExp: Arc<DAE::Exp>;
    origExp = (::match_deref::match_deref! { match &(ut.clone()) {
        Deref @ UnitAbsyn::UnitTerm::ADD { ut1: _, ut2: _, origExp: e } => {
            e.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::SUB { ut1: _, ut2: _, origExp: e } => {
            e.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::MUL { ut1: _, ut2: _, origExp: e } => {
            e.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::DIV { ut1: _, ut2: _, origExp: e } => {
            e.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::EQN { ut1: _, ut2: _, origExp: e } => {
            e.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::LOC { loc: _, origExp: e } => {
            e.clone()
        },
        Deref @ UnitAbsyn::UnitTerm::POW { ut1: _, exponent: _, origExp: e } => {
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(origExp)
}

fn buildTermOp(mut ut1: Arc<UnitAbsyn::UnitTerm>, mut ut2: Arc<UnitAbsyn::UnitTerm>, mut op: DAE::Operator, mut origExp: Arc<DAE::Exp>) -> Result<Arc<UnitAbsyn::UnitTerm>> {
    let mut ut: Arc<UnitAbsyn::UnitTerm>;
    ut = (::match_deref::match_deref! { match &((ut1.clone(), ut2.clone(), op.clone(), origExp.clone())) {
        (_, _, DAE::Operator::ADD { .. }, _) => Arc::new(UnitAbsyn::UnitTerm::ADD { ut1: ut1.clone(), ut2: ut2.clone(), origExp: origExp.clone() }),
        (_, _, DAE::Operator::SUB { .. }, _) => Arc::new(UnitAbsyn::UnitTerm::SUB { ut1: ut1.clone(), ut2: ut2.clone(), origExp: origExp.clone() }),
        (_, _, DAE::Operator::MUL { .. }, _) => Arc::new(UnitAbsyn::UnitTerm::MUL { ut1: ut1.clone(), ut2: ut2.clone(), origExp: origExp.clone() }),
        (_, _, DAE::Operator::DIV { .. }, _) => Arc::new(UnitAbsyn::UnitTerm::DIV { ut1: ut1.clone(), ut2: ut2.clone(), origExp: origExp.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(ut)
}

fn buildStores2(mut dae: DAE::DAElist, mut inStore: UnitAbsyn::Store, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(UnitAbsyn::Store, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))> {
    let mut outStore: UnitAbsyn::Store;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    (outStore, outHt) = 'mc: {
        let __mc_input = (dae.clone(), inStore.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::List::Nil }, _, _) => {
                    Ok((inStore.clone(), inHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { variableAttributesOption: attropt, componentRef: cr, .. }, tail: elts } }, _, _) => {
                    let mut indx: i32 = 0;
                    let mut unitStr: ArcStr = arcstr::literal!("");
                    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut store: UnitAbsyn::Store;
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let __pa0 = ::match_deref::match_deref! { match &(DAEUtil::getUnitAttr(attropt.clone())) {
                        Deref @ DAE::Exp::SCONST { string: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    unitStr = __pa0.clone();
                    unit = str2unit((unitStr.clone()).clone(), None)?;
                    (store, indx) = add(unit.clone(), inStore.clone())?;
                    ht = BaseHashTable::add((cr.clone(), indx.clone()), inHt.clone())?;
                    (store, ht) = buildStores2(DAE::DAElist { elementLst: elts.clone() }, store.clone(), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, .. }, tail: _ } }, _, _) => {
                    let mut indx: i32 = 0;
                    let mut store: UnitAbsyn::Store;
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    (store, indx) = add(crate::UnitAbsyn::Unit::UNSPECIFIED, inStore.clone())?;
                    ht = BaseHashTable::add((cr.clone(), indx.clone()), inHt.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: _, tail: elts } }, _, _) => {
                    let mut store: UnitAbsyn::Store;
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    (store, ht) = buildStores2(DAE::DAElist { elementLst: elts.clone() }, inStore.clone(), inHt.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStore, outHt))
}

fn buildStores3(mut dae: DAE::DAElist, mut inStore: UnitAbsyn::Store, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(UnitAbsyn::Store, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))> {
    let mut outStore: UnitAbsyn::Store;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    (outStore, outHt) = 'mc: {
        let __mc_input = (dae.clone(), inStore.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::List::Nil }, store, ht) => {
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: _ }, tail: elts } }, store, ht) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStoreExp(e1.clone(), store.clone(), ht.clone(), None)?;
                    (store, ht) = buildStoreExp(e2.clone(), store.clone(), ht.clone(), None)?;
                    (store, ht) = buildStores3(DAE::DAElist { elementLst: elts.clone() }, store.clone(), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: _, tail: elts } }, store, ht) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStores3(DAE::DAElist { elementLst: elts.clone() }, store.clone(), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStore, outHt))
}

fn buildStoreExp(mut exp: Arc<DAE::Exp>, mut inStore: UnitAbsyn::Store, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut parentOp: Option<DAE::Operator>) -> Result<(UnitAbsyn::Store, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))> {
    let mut outStore: UnitAbsyn::Store;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    (outStore, outHt) = 'mc: {
        let __mc_input = (exp.clone(), inStore.clone(), inHt.clone(), parentOp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r }, store, ht, _) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut indx: i32 = 0;
                    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    unit = selectConstantUnit(parentOp.clone())?;
                    (store, indx) = add(unit.clone(), store.clone())?;
                    s1 = (realString(r.clone())).clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent((s1.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    (ht, _, _, _) = BaseHashTable::add((cref_.clone(), indx.clone()), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: _, exp: Deref @ DAE::Exp::ICONST { integer: i } }, store, ht, _) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut indx: i32 = 0;
                    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    unit = selectConstantUnit(parentOp.clone())?;
                    (store, indx) = add(unit.clone(), store.clone())?;
                    s1 = (intString(i.clone())).clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent((s1.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    (ht, _, _, _) = BaseHashTable::add((cref_.clone(), indx.clone()), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, store, ht, _) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStoreExp(e1.clone(), store.clone(), ht.clone(), Some(op.clone()))?;
                    (store, ht) = buildStoreExp(e2.clone(), store.clone(), ht.clone(), Some(op.clone()))?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: _, exp: e1 }, store, ht, _) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStoreExp(e1.clone(), store.clone(), ht.clone(), parentOp.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: _, expThen: e1, expElse: e2 }, store, ht, _) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStoreExp(e1.clone(), store.clone(), ht.clone(), parentOp.clone())?;
                    (store, ht) = buildStoreExp(e2.clone(), store.clone(), ht.clone(), parentOp.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, store, ht, _) => {
                    Ok((store.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStore, outHt))
}

pub fn unitMultiply(mut u1: UnitAbsyn::Unit, mut u2: UnitAbsyn::Unit) -> Result<UnitAbsyn::Unit> {
    let mut u: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    u = (match (u1.clone(), u2.clone()) {
        (UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: ref tparams1, units: ref units1 } }, UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: ref tparams2, units: ref units2 } }) => {
            let mut tparams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
            let mut units: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
            tparams = listAppend(tparams1.clone(), tparams2.clone());
            units = List::threadMap(units1.clone(), units2.clone(), (std::sync::Arc::new(MMath::addRational) as std::sync::Arc<dyn ::std::ops::Fn(MMath::Rational, MMath::Rational) -> Result<MMath::Rational> + 'static>));
            UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: tparams.clone(), units: units.clone() } }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(u)
}

fn selectConstantUnit(mut op: Option<DAE::Operator>) -> Result<UnitAbsyn::Unit> {
    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    unit = (match op.clone() {
        None => crate::UnitAbsyn::Unit::UNSPECIFIED,
        Some(DAE::Operator::ADD { ty: _ }) => crate::UnitAbsyn::Unit::UNSPECIFIED,
        Some(DAE::Operator::SUB { ty: _ }) => crate::UnitAbsyn::Unit::UNSPECIFIED,
        Some(_) => str2unit((literal!("1")).clone(), None)?,
    });
    Ok(unit)
}

pub fn unit2str(mut unit: UnitAbsyn::Unit) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ((match unit.clone() {
        UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: ref typeParams, units: mut units } } => {
            let mut nums: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut denoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tpnoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tpdenoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tpstrs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (nums, denoms) = splitRationals(units.clone())?;
            (tpnoms, tpdenoms, tpstrs) = splitTypeParams(typeParams.clone())?;
            res = (UnitParserExt::unit2str(nums.clone(), denoms.clone(), tpnoms.clone(), tpdenoms.clone(), tpstrs.clone(), metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(0.0_f64))).clone();
            res.clone()
        },
        UnitAbsyn::Unit::UNSPECIFIED => {
            literal!("unspecified")
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(res)
}

pub fn str2unit(mut res: ArcStr, mut funcInstIdOpt: Option<i32>) -> Result<UnitAbsyn::Unit> {
    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut nums: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut denoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpnoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpdenoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpstrs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut typeParams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
    let mut units: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
    (unit, _, _) = str2unitWithScaleFactor((res.clone()).clone(), funcInstIdOpt.clone())?;
    Ok(unit)
}

pub fn str2unitWithScaleFactor(mut res: ArcStr, mut funcInstIdOpt: Option<i32>) -> Result<(UnitAbsyn::Unit, metamodelica::Real, metamodelica::Real)> {
    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut scaleFactor: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut offset: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nums: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut denoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpnoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpdenoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpstrs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut typeParams: Arc<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> = metamodelica::nil();
    let mut units: Arc<metamodelica::List<MMath::Rational>> = metamodelica::nil();
    (nums, denoms, tpnoms, tpdenoms, tpstrs, scaleFactor, offset) = UnitParserExt::str2unit((res.clone()).clone())?;
    units = joinRationals(nums.clone(), denoms.clone())?;
    typeParams = joinTypeParams(tpnoms.clone(), tpdenoms.clone(), tpstrs.clone(), funcInstIdOpt.clone())?;
    unit = UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: typeParams.clone(), units: units.clone() } };
    Ok((unit, scaleFactor, offset))
}

fn getDerivedUnitsHelper(mut baseUnit: UnitAbsyn::Unit, mut baseUnitStr: ArcStr, mut inUnits: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outUnits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut b: bool = false;
    for mut unitStr in &*inUnits.clone() {
        let mut unitStr = unitStr.clone();
        if boolNot(stringEq((baseUnitStr.clone()).clone(), (unitStr.clone()).clone())) {
            unit = str2unit((unitStr.clone()).clone(), None)?;
            b = baseUnit.clone() == unit.clone();
            if b.clone() {
                outUnits = cons((unitStr.clone()).clone(), outUnits.clone());
            }
        }
    }
    Ok(outUnits)
}

pub fn getDerivedUnits(mut baseUnit: UnitAbsyn::Unit, mut baseUnitStr: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut derivedUnits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut unitSymbols: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    unitSymbols = UnitParserExt::allUnitSymbols();
    derivedUnits = getDerivedUnitsHelper(baseUnit.clone(), (baseUnitStr.clone()).clone(), unitSymbols.clone())?;
    Ok(derivedUnits)
}

/* Tests  */
/* Test1:

model Test1 "CONSISTENT: All units defined. No inference"
  Position x;
  Velocity v;
  Acceleration a;
algorithm
  der(x) = v;
  der(v) = a;
end Test1;
*/
pub fn buildTest1() -> Result<(Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>, UnitAbsyn::Store)> {
    let mut ut: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
    let mut sigma: UnitAbsyn::Store;
    let mut r0: MMath::Rational;
    let mut r1: MMath::Rational;
    let mut nr1: MMath::Rational;
    let mut nr2: MMath::Rational;
    let mut unitderx: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut unitderv: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut unitx: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut unitv: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    let mut unita: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    r0 = MMath::Rational { nom: 0, denom: 0 };
    r1 = MMath::Rational { nom: 1, denom: 0 };
    nr1 = MMath::Rational { nom: -1, denom: 0 };
    nr2 = MMath::Rational { nom: -2, denom: 0 };
    ut = list![Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: Arc::new(UnitAbsyn::UnitTerm::LOC { loc: 1, origExp: Arc::new(DAE::Exp::SCONST { string: (literal!("1")).clone() }) }), ut2: Arc::new(UnitAbsyn::UnitTerm::LOC { loc: 4, origExp: Arc::new(DAE::Exp::SCONST { string: (literal!("4")).clone() }) }), origExp: Arc::new(DAE::Exp::SCONST { string: (literal!("1==4")).clone() }) }), Arc::new(UnitAbsyn::UnitTerm::EQN { ut1: Arc::new(UnitAbsyn::UnitTerm::LOC { loc: 2, origExp: Arc::new(DAE::Exp::SCONST { string: (literal!("2")).clone() }) }), ut2: Arc::new(UnitAbsyn::UnitTerm::LOC { loc: 5, origExp: Arc::new(DAE::Exp::SCONST { string: (literal!("5")).clone() }) }), origExp: Arc::new(DAE::Exp::SCONST { string: (literal!("2==5")).clone() }) })];
    unitderx = UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: list![r1.clone(), nr1.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone()] } };
    unitderv = UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: list![r1.clone(), nr2.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone()] } };
    unitx = UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: list![r1.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone()] } };
    unitv = UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: list![r1.clone(), nr1.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone()] } };
    unita = UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: list![r1.clone(), nr2.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone(), r0.clone()] } };
    sigma = emptyStore();
    (sigma, _) = add(unitderx.clone(), sigma.clone())?;
    (sigma, _) = add(unitderv.clone(), sigma.clone())?;
    (sigma, _) = add(unitx.clone(), sigma.clone())?;
    (sigma, _) = add(unitv.clone(), sigma.clone())?;
    (sigma, _) = add(unita.clone(), sigma.clone())?;
    printStore(sigma.clone())?;
    Ok((ut, sigma))
}

/* Test2:
model Test2 "CONSISTENT: Subtraction operator. All units defined. No inference"
Position x,y,z;
algorithm
z = x-y;
end Test2;
*/
/*public function buildTest2

  output UnitAbsyn.UnitTerms ut;
  output UnitAbsyn.Locations sigma;
protected
  MMath.Rational r0,r1;
  algorithm
    r0 := MMath.RATIONAL(0,0);
    r1 := MMath.RATIONAL(1,0);
    ut := {
    UnitAbsyn.EQN(UnitAbsyn.LOC("z"),UnitAbsyn.SUB(UnitAbsyn.LOC("x"),UnitAbsyn.LOC("y")))
    };
    sigma := {
    UnitAbsyn.LOCATION("x",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // x -> m
    UnitAbsyn.LOCATION("y",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // y -> m
    UnitAbsyn.LOCATION("z",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))) // z -> m
    };
 end buildTest2;
 */
/* Test3
 model Test3 "OVERDETERMINED: All units defined. No inference"
 Position x,y;
 Velocity z;
algorithm
 z = x-y;
end Test3;
 */
/*public function buildTest3
  output UnitAbsyn.UnitTerms ut;
  output UnitAbsyn.Locations sigma;
protected
  MMath.Rational r0,r1,nr1;
  algorithm
    r0 := MMath.RATIONAL(0,0);
    r1 := MMath.RATIONAL(1,0);
    nr1 := MMath.RATIONAL(-1,0);
    ut := {
    UnitAbsyn.EQN(UnitAbsyn.LOC("z"),UnitAbsyn.SUB(UnitAbsyn.LOC("x"),UnitAbsyn.LOC("y")))
    };
    sigma := {
    UnitAbsyn.LOCATION("x",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // x -> m
    UnitAbsyn.LOCATION("y",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // y -> m
    UnitAbsyn.LOCATION("z",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,nr1,r0,r0,r0,r0,r0}))) // z -> m/s
    };
 end buildTest3;
 */
/*
 Test5

 model Test5 "CONSTISTENT: Multiplication operator. Not all units defined. inference"
  Position x,y;
  Real z;
 algorithm
 z = x*y;
end test5;
*/
/*
 public function buildTest5
  output UnitAbsyn.UnitTerms ut;
  output UnitAbsyn.Locations sigma;
protected
  MMath.Rational r0,r1,nr1;
  algorithm
    r0 := MMath.RATIONAL(0,0);
    r1 := MMath.RATIONAL(1,0);
    nr1 := MMath.RATIONAL(-1,0);
    ut := {
    UnitAbsyn.EQN(UnitAbsyn.LOC("z"),UnitAbsyn.MUL(UnitAbsyn.LOC("x"),UnitAbsyn.LOC("y")))
    };
    sigma := {
    UnitAbsyn.LOCATION("x",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // x -> m
    UnitAbsyn.LOCATION("y",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // y -> m
    UnitAbsyn.LOCATION("z",UnitAbsyn.UNSPECIFIED())                                             // z -> unspecified
    };
 end buildTest5;
 */
/* Test 8


function Foo8
  input Real x;
  output Real y;
algorithm
  y := x+1; // 1 has unkown unit
end Foo8;

model Test8 "CONSISTENT. type inference in function call "
  Position x,y;
  Velocity v1,v2;

algorithm
  x = Foo8(y);
  v1 = Foo8(v2);
end Test8;
 */
/*public function buildTest8
  output UnitAbsyn.UnitTerms ut;
  output UnitAbsyn.Locations sigma;
protected
  MMath.Rational r0,r1,nr1;
  algorithm
    r0 := MMath.RATIONAL(0,0);
    r1 := MMath.RATIONAL(1,0);
    nr1 := MMath.RATIONAL(-1,0);
    ut := {
    UnitAbsyn.EQN(UnitAbsyn.LOC("x"),UnitAbsyn.LOC("Foo8(x)")),
    UnitAbsyn.EQN(UnitAbsyn.LOC("v1"),UnitAbsyn.LOC("Foo8(v2)"))
    };
    sigma := {
    UnitAbsyn.LOCATION("Foo8(y)",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // Foo8(x) -> m
    UnitAbsyn.LOCATION("Foo8(v2)",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,nr1,r0,r0,r0,r0,r0}))), // Foo8(v2) -> m/s
    UnitAbsyn.LOCATION("v1",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,nr1,r0,r0,r0,r0,r0}))), // Foo8(v2) -> m/s
    UnitAbsyn.LOCATION("x",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))) // Foo8(v2) -> m
    };
 end buildTest8;
 */
