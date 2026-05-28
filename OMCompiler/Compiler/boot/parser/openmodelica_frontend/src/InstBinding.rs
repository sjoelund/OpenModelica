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

use crate::Ceval;
use crate::ComponentReference;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionSimplify;
use crate::FCore;
use crate::FGraph;
use crate::InnerOuter;
use crate::InstSection;
use crate::InstUtil;
use crate::Mod;
use crate::PrefixUtil;
use crate::Types;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

pub type InstDims = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>;

thread_local! { static __stateSelectType_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("never")).clone(), (literal!("avoid")).clone(), (literal!("default")).clone(), (literal!("prefer")).clone(), (literal!("always")).clone()], literalVarLst: list![Arc::new(DAE::Var { name: (literal!("never")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(1), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("never")).clone(), (literal!("avoid")).clone(), (literal!("default")).clone(), (literal!("prefer")).clone(), (literal!("always")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("avoid")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(2), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("never")).clone(), (literal!("avoid")).clone(), (literal!("default")).clone(), (literal!("prefer")).clone(), (literal!("always")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("default")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(3), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("never")).clone(), (literal!("avoid")).clone(), (literal!("default")).clone(), (literal!("prefer")).clone(), (literal!("always")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("prefer")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(4), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("never")).clone(), (literal!("avoid")).clone(), (literal!("default")).clone(), (literal!("prefer")).clone(), (literal!("always")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("always")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(5), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("never")).clone(), (literal!("avoid")).clone(), (literal!("default")).clone(), (literal!("prefer")).clone(), (literal!("always")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], attributeLst: metamodelica::nil() }); }
pub fn stateSelectType() -> Arc<DAE::Type> { __stateSelectType_TLS.with(|__t| __t.clone()) }

thread_local! { static __uncertaintyType_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("given")).clone(), (literal!("sought")).clone(), (literal!("refine")).clone(), (literal!("propagate")).clone()], literalVarLst: list![Arc::new(DAE::Var { name: (literal!("given")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(1), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("given")).clone(), (literal!("sought")).clone(), (literal!("refine")).clone(), (literal!("propagate")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("sought")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(2), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("given")).clone(), (literal!("sought")).clone(), (literal!("refine")).clone(), (literal!("propagate")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("refine")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(3), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("given")).clone(), (literal!("sought")).clone(), (literal!("refine")).clone(), (literal!("propagate")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("propagate")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(4), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("given")).clone(), (literal!("sought")).clone(), (literal!("refine")).clone(), (literal!("propagate")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], attributeLst: metamodelica::nil() }); }
pub fn uncertaintyType() -> Arc<DAE::Type> { __uncertaintyType_TLS.with(|__t| __t.clone()) }

thread_local! { static __distributionType_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Distribution")).clone() }) }, varLst: list![Arc::new(DAE::Var { name: (literal!("name")).clone(), attributes: Arc::new(DAE::Attributes { connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC }), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("params")).clone(), attributes: Arc::new(DAE::Attributes { connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC }), ty: DAE::T_ARRAY_REAL_NODIM().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("paramNames")).clone(), attributes: Arc::new(DAE::Attributes { connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC }), ty: DAE::T_ARRAY_STRING_NODIM().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false }); }
pub fn distributionType() -> Arc<DAE::Type> { __distributionType_TLS.with(|__t| __t.clone()) }

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn instBinding(mut inMod: Arc<DAE::Mod>, mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inType: Arc<DAE::Type>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut inString: ArcStr, mut useConstValue: bool) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExpExpOption: Option<Arc<DAE::Exp>> = None;
    outExpExpOption = 'mc: {
        let __mc_input = (inMod.clone(), inVarLst.clone(), inType.clone(), inIntegerLst.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, _, expected_type, Deref @ metamodelica::List::Nil, bind_name) => {
                    let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut e: Arc<DAE::Exp>;
                    let mut e_1: Arc<DAE::Exp>;
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut optVal: Option<Arc<Values::Value>> = None;
                    mod2 = Mod::lookupCompModification(r#mod.clone(), (bind_name.clone()).clone())?;
                    let Some(DAE::TYPED { modifierAsExp: __pa0, modifierAsValue: __pa1, properties: DAE::PROP { type_: __pa2, constFlag: _ }, modifierAsAbsynExp: _, .. }) = (Mod::modEquation(mod2.clone())?) else { bail!("pattern mismatch") };
                    e = __pa0.clone();
                    optVal = __pa1.clone();
                    ty2 = __pa2.clone();
                    (e_1, _) = Types::matchType(e.clone(), ty2.clone(), expected_type.clone(), true)?;
                    e_1 = InstUtil::checkUseConstValue(useConstValue.clone(), e_1.clone(), optVal.clone())?;
                    Ok(Some(e_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, _, etype, index_list, bind_name) => {
                    let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut result: Option<Arc<DAE::Exp>> = None;
                    mod2 = Mod::lookupCompModification(r#mod.clone(), (bind_name.clone()).clone())?;
                    result = instBinding2(mod2.clone(), etype.clone(), index_list.clone(), (bind_name.clone()).clone(), useConstValue.clone())?;
                    Ok(result.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, _, _, Deref @ metamodelica::List::Nil, bind_name) => {
                    if '__try0: {
                        let _ = unwrap_break_err!(Mod::lookupCompModification(r#mod.clone(), (bind_name.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding, name, .. }, tail: _ }, _, _, bind_name) => {
                    let true = (stringEq((name.clone()).clone(), (bind_name.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(DAEUtil::bindingExp(binding.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, Deref @ metamodelica::List::Cons { head: _, tail: varLst }, etype, index_list, bind_name) => {
                    Ok(instBinding(r#mod.clone(), varLst.clone(), etype.clone(), index_list.clone(), (bind_name.clone()).clone(), useConstValue.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpExpOption)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn instBinding2(mut inMod: Arc<DAE::Mod>, mut inType: Arc<DAE::Type>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut inString: ArcStr, mut useConstValue: bool) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExpExpOption: Option<Arc<DAE::Exp>> = None;
    outExpExpOption = (::match_deref::match_deref! { match &((inMod.clone(), inType.clone(), inIntegerLst.clone(), inString.clone())) {
        (r#mod, etype, Deref @ metamodelica::List::Cons { head: index, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut e: Arc<DAE::Exp>;
            let mut e_1: Arc<DAE::Exp>;
            let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut optVal: Option<Arc<Values::Value>> = None;
            mod2 = Mod::lookupIdxModification(r#mod.clone(), Arc::new(DAE::Exp::ICONST { integer: index.clone() }))?;
            let Some(DAE::TYPED { modifierAsExp: __pa0, modifierAsValue: __pa1, properties: DAE::PROP { type_: __pa2, constFlag: _ }, modifierAsAbsynExp: _, .. }) = (Mod::modEquation(mod2.clone())?) else { bail!("pattern mismatch") };
            e = __pa0.clone();
            optVal = __pa1.clone();
            ty2 = __pa2.clone();
            (e_1, _) = Types::matchType(e.clone(), ty2.clone(), etype.clone(), true)?;
            e_1 = InstUtil::checkUseConstValue(useConstValue.clone(), e_1.clone(), optVal.clone())?;
            Some(e_1.clone())
        },
        (r#mod, etype, Deref @ metamodelica::List::Cons { head: index, tail: res }, bind_name) => {
            let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut result: Option<Arc<DAE::Exp>> = None;
            result = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut mod2: Arc<DAE::Mod> = mod2.clone();
            let mut result: Option<Arc<DAE::Exp>>;
            mod2 = Mod::lookupIdxModification(r#mod.clone(), Arc::new(DAE::Exp::ICONST { integer: index.clone() }))?;
            result = instBinding2(mod2.clone(), etype.clone(), res.clone(), (bind_name.clone()).clone(), useConstValue.clone())?;
            Ok(result.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(None)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
            result.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpExpOption)
}

pub fn instStartBindingExp(mut inMod: Arc<DAE::Mod>, mut inExpectedType: Arc<DAE::Type>, mut inVariability: SCode::Variability) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outStartValue: Option<Arc<DAE::Exp>> = None;
    if SCodeUtil::isConstant(inVariability.clone()) {
        outStartValue = None;
    } else {
        outStartValue = instBinding(inMod.clone(), metamodelica::nil(), Types::arrayElementType(inExpectedType.clone()), metamodelica::nil(), (literal!("start")).clone(), false)?;
    }
    Ok(outStartValue)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn instStartOrigin(mut inMod: Arc<DAE::Mod>, mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inString: ArcStr) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExpExpOption: Option<Arc<DAE::Exp>> = None;
    outExpExpOption = 'mc: {
        let __mc_input = (inMod.clone(), inVarLst.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, _, bind_name) => {
                    let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    mod2 = Mod::lookupCompModification(r#mod.clone(), (bind_name.clone()).clone())?;
                    let Some(_) = (Mod::modEquation(mod2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Some(Arc::new(DAE::Exp::SCONST { string: (literal!("binding")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name, .. }, tail: _ }, bind_name) => {
                    let true = (stringEq((name.clone()).clone(), (bind_name.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(Some(Arc::new(DAE::Exp::SCONST { string: (literal!("type")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, Deref @ metamodelica::List::Cons { head: _, tail: varLst }, bind_name) => {
                    Ok(instStartOrigin(r#mod.clone(), varLst.clone(), (bind_name.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpExpOption)
}

pub fn instDaeVariableAttributes(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inMod: Arc<DAE::Mod>, mut inType: Arc<DAE::Type>, mut inIntegerLst: Arc<metamodelica::List<i32>>) -> Result<(FCore::Cache, Option<Arc<DAE::VariableAttributes>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDAEVariableAttributesOption: Option<Arc<DAE::VariableAttributes>> = None;
    (outCache, outDAEVariableAttributesOption) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inMod.clone(), inType.clone(), inIntegerLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, r#mod, Deref @ DAE::Type::T_REAL { varLst }, index_list) => {
                    let mut quantity_str: Option<Arc<DAE::Exp>> = None;
                    let mut unit_str: Option<Arc<DAE::Exp>> = None;
                    let mut displayunit_str: Option<Arc<DAE::Exp>> = None;
                    let mut nominal_val: Option<Arc<DAE::Exp>> = None;
                    let mut fixed_val: Option<Arc<DAE::Exp>> = None;
                    let mut exp_bind_select: Option<Arc<DAE::Exp>> = None;
                    let mut exp_bind_uncertainty: Option<Arc<DAE::Exp>> = None;
                    let mut min_val: Option<Arc<DAE::Exp>> = None;
                    let mut max_val: Option<Arc<DAE::Exp>> = None;
                    let mut start_val: Option<Arc<DAE::Exp>> = None;
                    let mut startOrigin: Option<Arc<DAE::Exp>> = None;
                    let mut stateSelect_value: Option<DAE::StateSelect> = None;
                    let mut uncertainty_value: Option<DAE::Uncertainty> = None;
                    let mut distribution_value: Option<Arc<DAE::Distribution>> = None;
                    quantity_str = instBinding(r#mod.clone(), varLst.clone(), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), (literal!("quantity")).clone(), false)?;
                    unit_str = instBinding(r#mod.clone(), varLst.clone(), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), (literal!("unit")).clone(), false)?;
                    displayunit_str = instBinding(r#mod.clone(), varLst.clone(), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), (literal!("displayUnit")).clone(), false)?;
                    min_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_REAL_DEFAULT().clone(), index_list.clone(), (literal!("min")).clone(), false)?;
                    max_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_REAL_DEFAULT().clone(), index_list.clone(), (literal!("max")).clone(), false)?;
                    start_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_REAL_DEFAULT().clone(), index_list.clone(), (literal!("start")).clone(), false)?;
                    fixed_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_BOOL_DEFAULT().clone(), index_list.clone(), (literal!("fixed")).clone(), true)?;
                    nominal_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_REAL_DEFAULT().clone(), index_list.clone(), (literal!("nominal")).clone(), false)?;
                    exp_bind_select = instEnumerationBinding(r#mod.clone(), varLst.clone(), index_list.clone(), (literal!("stateSelect")).clone(), stateSelectType().clone(), true)?;
                    stateSelect_value = InstUtil::getStateSelectFromExpOption(exp_bind_select.clone());
                    exp_bind_uncertainty = instEnumerationBinding(r#mod.clone(), varLst.clone(), index_list.clone(), (literal!("uncertain")).clone(), uncertaintyType().clone(), true)?;
                    uncertainty_value = getUncertainFromExpOption(exp_bind_uncertainty.clone());
                    distribution_value = instDistributionBinding(r#mod.clone(), varLst.clone(), index_list.clone(), (literal!("distribution")).clone(), false)?;
                    startOrigin = instStartOrigin(r#mod.clone(), varLst.clone(), (literal!("start")).clone())?;
                    Ok((cache.clone(), Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity_str.clone(), unit: unit_str.clone(), displayUnit: displayunit_str.clone(), min: min_val.clone(), max: max_val.clone(), start: start_val.clone(), fixed: fixed_val.clone(), nominal: nominal_val.clone(), stateSelectOption: stateSelect_value.clone(), uncertainOption: uncertainty_value.clone(), distributionOption: distribution_value.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, r#mod, Deref @ DAE::Type::T_INTEGER { varLst }, index_list) => {
                    let mut quantity_str: Option<Arc<DAE::Exp>> = None;
                    let mut fixed_val: Option<Arc<DAE::Exp>> = None;
                    let mut exp_bind_uncertainty: Option<Arc<DAE::Exp>> = None;
                    let mut min_val: Option<Arc<DAE::Exp>> = None;
                    let mut max_val: Option<Arc<DAE::Exp>> = None;
                    let mut start_val: Option<Arc<DAE::Exp>> = None;
                    let mut startOrigin: Option<Arc<DAE::Exp>> = None;
                    let mut uncertainty_value: Option<DAE::Uncertainty> = None;
                    let mut distribution_value: Option<Arc<DAE::Distribution>> = None;
                    quantity_str = instBinding(r#mod.clone(), varLst.clone(), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), (literal!("quantity")).clone(), false)?;
                    min_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_INTEGER_DEFAULT().clone(), index_list.clone(), (literal!("min")).clone(), false)?;
                    max_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_INTEGER_DEFAULT().clone(), index_list.clone(), (literal!("max")).clone(), false)?;
                    start_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_INTEGER_DEFAULT().clone(), index_list.clone(), (literal!("start")).clone(), false)?;
                    fixed_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_BOOL_DEFAULT().clone(), index_list.clone(), (literal!("fixed")).clone(), true)?;
                    exp_bind_uncertainty = instEnumerationBinding(r#mod.clone(), varLst.clone(), index_list.clone(), (literal!("uncertain")).clone(), uncertaintyType().clone(), true)?;
                    uncertainty_value = getUncertainFromExpOption(exp_bind_uncertainty.clone());
                    distribution_value = instDistributionBinding(r#mod.clone(), varLst.clone(), index_list.clone(), (literal!("distribution")).clone(), false)?;
                    startOrigin = instStartOrigin(r#mod.clone(), varLst.clone(), (literal!("start")).clone())?;
                    Ok((cache.clone(), Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity_str.clone(), min: min_val.clone(), max: max_val.clone(), start: start_val.clone(), fixed: fixed_val.clone(), uncertainOption: uncertainty_value.clone(), distributionOption: distribution_value.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, r#mod, tp @ Deref @ DAE::Type::T_BOOL { varLst }, index_list) => {
                    let mut quantity_str: Option<Arc<DAE::Exp>> = None;
                    let mut fixed_val: Option<Arc<DAE::Exp>> = None;
                    let mut start_val: Option<Arc<DAE::Exp>> = None;
                    let mut startOrigin: Option<Arc<DAE::Exp>> = None;
                    quantity_str = instBinding(r#mod.clone(), varLst.clone(), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), (literal!("quantity")).clone(), false)?;
                    start_val = instBinding(r#mod.clone(), varLst.clone(), tp.clone(), index_list.clone(), (literal!("start")).clone(), false)?;
                    fixed_val = instBinding(r#mod.clone(), varLst.clone(), tp.clone(), index_list.clone(), (literal!("fixed")).clone(), true)?;
                    startOrigin = instStartOrigin(r#mod.clone(), varLst.clone(), (literal!("start")).clone())?;
                    Ok((cache.clone(), Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity_str.clone(), start: start_val.clone(), fixed: fixed_val.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, Deref @ DAE::Type::T_CLOCK { .. }, _) => {
                    Ok((cache.clone(), Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: None, finalPrefix: None }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, r#mod, tp @ Deref @ DAE::Type::T_STRING { varLst }, index_list) => {
                    let mut quantity_str: Option<Arc<DAE::Exp>> = None;
                    let mut fixed_val: Option<Arc<DAE::Exp>> = None;
                    let mut start_val: Option<Arc<DAE::Exp>> = None;
                    let mut startOrigin: Option<Arc<DAE::Exp>> = None;
                    quantity_str = instBinding(r#mod.clone(), varLst.clone(), tp.clone(), index_list.clone(), (literal!("quantity")).clone(), false)?;
                    start_val = instBinding(r#mod.clone(), varLst.clone(), tp.clone(), index_list.clone(), (literal!("start")).clone(), false)?;
                    fixed_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_BOOL_DEFAULT().clone(), index_list.clone(), (literal!("fixed")).clone(), true)?;
                    startOrigin = instStartOrigin(r#mod.clone(), varLst.clone(), (literal!("start")).clone())?;
                    Ok((cache.clone(), Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity_str.clone(), start: start_val.clone(), fixed: fixed_val.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, r#mod, enumtype @ Deref @ DAE::Type::T_ENUMERATION { attributeLst: varLst, .. }, index_list) => {
                    let mut quantity_str: Option<Arc<DAE::Exp>> = None;
                    let mut fixed_val: Option<Arc<DAE::Exp>> = None;
                    let mut exp_bind_min: Option<Arc<DAE::Exp>> = None;
                    let mut exp_bind_max: Option<Arc<DAE::Exp>> = None;
                    let mut exp_bind_start: Option<Arc<DAE::Exp>> = None;
                    let mut startOrigin: Option<Arc<DAE::Exp>> = None;
                    quantity_str = instBinding(r#mod.clone(), varLst.clone(), DAE::T_STRING_DEFAULT().clone(), index_list.clone(), (literal!("quantity")).clone(), false)?;
                    exp_bind_min = instBinding(r#mod.clone(), varLst.clone(), enumtype.clone(), index_list.clone(), (literal!("min")).clone(), false)?;
                    exp_bind_max = instBinding(r#mod.clone(), varLst.clone(), enumtype.clone(), index_list.clone(), (literal!("max")).clone(), false)?;
                    exp_bind_start = instBinding(r#mod.clone(), varLst.clone(), enumtype.clone(), index_list.clone(), (literal!("start")).clone(), false)?;
                    fixed_val = instBinding(r#mod.clone(), varLst.clone(), DAE::T_BOOL_DEFAULT().clone(), index_list.clone(), (literal!("fixed")).clone(), true)?;
                    startOrigin = instStartOrigin(r#mod.clone(), varLst.clone(), (literal!("start")).clone())?;
                    Ok((cache.clone(), Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity_str.clone(), min: exp_bind_min.clone(), max: exp_bind_max.clone(), start: exp_bind_start.clone(), fixed: fixed_val.clone(), equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, _) => {
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDAEVariableAttributesOption))
}

fn instEnumerationBinding(mut inMod: Arc<DAE::Mod>, mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inIndices: Arc<metamodelica::List<i32>>, mut inName: ArcStr, mut expected_type: Arc<DAE::Type>, mut useConstValue: bool) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outBinding: Option<Arc<DAE::Exp>> = None;
    match '__try0: {
        outBinding = unwrap_break_err!(instBinding(inMod.clone(), varLst.clone(), expected_type.clone(), inIndices.clone(), (inName.clone()).clone(), useConstValue.clone()), '__try0);
        Ok::<_, anyhow::Error>((outBinding.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outBinding = __try0_o0;
        }
        Err(_) => {
            Error::addMessage(Error::TYPE_ERROR.clone(), list![(inName.clone()).clone(), (literal!("enumeration type")).clone()])?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(outBinding)
}

fn instDistributionBinding(mut inMod: Arc<DAE::Mod>, mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut inString: ArcStr, mut useConstValue: bool) -> Result<Option<Arc<DAE::Distribution>>> {
    let mut out: Option<Arc<DAE::Distribution>> = None;
    out = 'mc: {
        let __mc_input = (inMod.clone(), inIntegerLst.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, index_list, bind_name) => {
                    let mut name: Arc<DAE::Exp>;
                    let mut params: Arc<DAE::Exp>;
                    let mut paramNames: Arc<DAE::Exp>;
                    let mut path: Arc<Absyn::Path>;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(instBinding(r#mod.clone(), varLst.clone(), distributionType().clone(), index_list.clone(), (bind_name.clone()).clone(), useConstValue.clone())?) {
                        Some(Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } } }, path: __pa3, .. }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    params = __pa1.clone();
                    paramNames = __pa2.clone();
                    path = __pa3.clone();
                    let true = (AbsynUtil::pathEqual(path.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("Distribution")).clone() }))) else { bail!("pattern mismatch") };
                    Ok(Some(Arc::new(DAE::Distribution { name: name.clone(), params: params.clone(), paramNames: paramNames.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, index_list, bind_name) => {
                    let mut name: Arc<DAE::Exp>;
                    let mut params: Arc<DAE::Exp>;
                    let mut paramNames: Arc<DAE::Exp>;
                    let mut path: Arc<Absyn::Path>;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(instBinding(r#mod.clone(), varLst.clone(), distributionType().clone(), index_list.clone(), (bind_name.clone()).clone(), useConstValue.clone())?) {
                        Some(Deref @ DAE::Exp::RECORD { exps: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } } }, path: __pa3, .. }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    params = __pa1.clone();
                    paramNames = __pa2.clone();
                    path = __pa3.clone();
                    let true = (AbsynUtil::pathEqual(path.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("Distribution")).clone() }))) else { bail!("pattern mismatch") };
                    Ok(Some(Arc::new(DAE::Distribution { name: name.clone(), params: params.clone(), paramNames: paramNames.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#mod, index_list, bind_name) => {
                    let mut name: Arc<DAE::Exp>;
                    let mut params: Arc<DAE::Exp>;
                    let mut paramNames: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut paramDim: i32 = 0;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crParams: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(instBinding(r#mod.clone(), varLst.clone(), distributionType().clone(), index_list.clone(), (bind_name.clone()).clone(), useConstValue.clone())?) {
                        Some(Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    ty = __pa1.clone();
                    let true = (Types::isRecord(ty.clone())) else { bail!("pattern mismatch") };
                    let __pa2 = ::match_deref::match_deref! { match &(ty.clone()) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: __pa2 }, tail: Deref @ metamodelica::List::Nil }, .. }, .. }, tail: _ } }, .. } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    paramDim = __pa2.clone();
                    crName = ComponentReference::crefPrependIdent(cr.clone(), (literal!("name")).clone(), metamodelica::nil(), DAE::T_STRING_DEFAULT().clone())?;
                    crParams = ComponentReference::crefPrependIdent(cr.clone(), (literal!("params")).clone(), metamodelica::nil(), Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: paramDim.clone() })] }))?;
                    name = Expression::makeCrefExp(crName.clone(), DAE::T_STRING_DEFAULT().clone())?;
                    params = Expression::makeCrefExp(crParams.clone(), Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: paramDim.clone() })] }))?;
                    paramNames = Expression::makeCrefExp(crParams.clone(), Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: paramDim.clone() })] }))?;
                    Ok(Some(Arc::new(DAE::Distribution { name: name.clone(), params: params.clone(), paramNames: paramNames.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

fn getUncertainFromExpOption(mut expOption: Option<Arc<DAE::Exp>>) -> Option<DAE::Uncertainty> {
    let mut out: Option<DAE::Uncertainty> = None;
    out = (::match_deref::match_deref! { match &(expOption.clone()) {
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "given" }, name: Deref @ "Uncertainty" }, .. }) => Some(openmodelica_frontend_types::DAE::Uncertainty::GIVEN),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sought" }, name: Deref @ "Uncertainty" }, .. }) => Some(openmodelica_frontend_types::DAE::Uncertainty::SOUGHT),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "refine" }, name: Deref @ "Uncertainty" }, .. }) => Some(openmodelica_frontend_types::DAE::Uncertainty::REFINE),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "propagate" }, name: Deref @ "Uncertainty" }, .. }) => Some(openmodelica_frontend_types::DAE::Uncertainty::PROPAGATE),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub fn instModEquation(mut inComponentRef: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inMod: Arc<DAE::Mod>, mut inSource: Arc<DAE::ElementSource>, mut inImpl: bool) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    outDae = 'mc: {
        let __mc_input = (inType.clone(), inMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: _, modifierAsValue: Some(_), properties: DAE::Properties::PROP { type_: _, constFlag: DAE::Const::C_CONST }, modifierAsAbsynExp: _, .. }), .. }) => {
                    Ok(DAE::emptyDae().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { properties: prop2, .. }), .. }) => {
                    ::match_deref::match_deref! { match &(Types::getPropType(prop2.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, tail: Deref @ metamodelica::List::Nil }, .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(DAE::emptyDae().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::MOD { info, binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: prop2, modifierAsAbsynExp: aexp2, .. }), .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
                    let mut lhs: Arc<DAE::Exp>;
                    let mut aexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut scode: Arc<SCode::Equation>;
                    let mut acr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    t = Types::simplifyType(inType.clone())?;
                    lhs = Expression::makeCrefExp(inComponentRef.clone(), t.clone())?;
                    acr = ComponentReference::unelabCref(inComponentRef.clone())?;
                    aexp1 = Arc::new(Absyn::Exp::CREF { componentRef: acr.clone() });
                    scode = Arc::new(SCode::Equation::EQ_EQUALS { expLeft: aexp1.clone(), expRight: aexp2.clone(), comment: SCode::noComment.clone(), info: info.clone() });
                    source = ElementSource::addSymbolicTransformation(inSource.clone(), Arc::new(DAE::SymbolicOperation::FLATTEN { scode: scode.clone(), dae: None }))?;
                    dae = InstSection::instEqEquation(lhs.clone(), DAE::Properties::PROP { type_: inType.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, e.clone(), prop2.clone(), source.clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, inImpl.clone(), info.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::MOD { binding: None, .. }) => {
                    Ok(DAE::emptyDae().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::NOMOD) => {
                    Ok(DAE::emptyDae().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Mod::REDECL { .. }) => {
                    Ok(DAE::emptyDae().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstBinding.instModEquation failed\n type: ")).clone())?;
                    Debug::trace((TypesDump::printTypeStr(inType.clone())?).clone())?;
                    Debug::trace((literal!("\n  cref: ")).clone())?;
                    Debug::trace((ComponentReferenceBasics::printComponentRefStr(inComponentRef.clone())?).clone())?;
                    Debug::trace((literal!("\n mod:")).clone())?;
                    Debug::traceln((Mod::printModStr(inMod.clone())?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDae)
}

pub fn makeBinding(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAttributes: SCode::Attributes, mut inMod: Arc<DAE::Mod>, mut inType: Arc<DAE::Type>, mut inPrefix: DAE::Prefix, mut componentName: ArcStr, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Binding>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    (outCache, outBinding) = 'mc: {
        let __mc_input = (inCache.clone(), inAttributes.clone(), inMod.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::NOMOD, _) => {
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut complex_vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut tpath: Arc<Absyn::Path>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::arrayElementType(inType.clone())) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, complexClassType: ClassInf::State::RECORD { path: __pa1 }, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    complex_vars = __pa0.clone();
                    tpath = __pa1.clone();
                    let true = (Types::allHaveBindings(complex_vars.clone())?) else { bail!("pattern mismatch") };
                    binding = makeRecordBinding(cache.clone(), inEnv.clone(), tpath.clone(), inType.clone(), complex_vars.clone(), metamodelica::nil(), inInfo.clone())?;
                    Ok((cache.clone(), binding.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::NOMOD, _) => {
                    Ok((cache.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Mod::REDECL { .. }, _) => {
                    Ok(makeBinding(inCache.clone(), inEnv.clone(), inAttributes.clone(), var_field!((*inMod).r#mod, DAE::Mod::REDECL).clone(), inType.clone(), inPrefix.clone(), (componentName.clone()).clone(), inInfo.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, SCode::Attributes { variability: SCode::Variability::PARAM, .. }, Deref @ DAE::Mod::MOD { binding: None, .. }, tp) => {
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut startValueModification: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let true = (Types::getFixedVarAttributeParameterOrConstant(tp.clone())) else { bail!("pattern mismatch") };
                    startValueModification = Mod::lookupCompModification(inMod.clone(), (literal!("start")).clone())?;
                    let false = (Mod::isEmptyMod(startValueModification.clone())) else { bail!("pattern mismatch") };
                    (cache, binding) = makeBinding(cache.clone(), inEnv.clone(), inAttributes.clone(), startValueModification.clone(), inType.clone(), inPrefix.clone(), (componentName.clone()).clone(), inInfo.clone())?;
                    binding = DAEUtil::setBindingSource(binding.clone(), openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_START_VALUE)?;
                    Ok((cache.clone(), binding.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::MOD { subModLst: sub_mods @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, _) => {
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut complex_vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut tpath: Arc<Absyn::Path>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::arrayElementType(inType.clone())) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, complexClassType: ClassInf::State::RECORD { path: __pa1 }, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    complex_vars = __pa0.clone();
                    tpath = __pa1.clone();
                    binding = makeRecordBinding(cache.clone(), inEnv.clone(), tpath.clone(), inType.clone(), complex_vars.clone(), sub_mods.clone(), inInfo.clone())?;
                    Ok((cache.clone(), binding.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::MOD { binding: None, .. }, _) => {
                    Ok((cache.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: Some(v), properties: prop, modifierAsAbsynExp: _, .. }), .. }, e_tp) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_val_exp: Arc<DAE::Exp>;
                    let mut e_val: Option<Arc<Values::Value>> = None;
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut v = (*v).clone();
                    c = Types::propAllConst(prop.clone())?;
                    tp = Types::getPropType(prop.clone())?;
                    let false = (Types::equivtypes(tp.clone(), e_tp.clone())?) else { bail!("pattern mismatch") };
                    e_val_exp = ValuesUtil::valueExp(v.clone(), Some(e.clone()))?;
                    (e_1, _) = Types::matchType(e.clone(), tp.clone(), e_tp.clone(), false)?;
                    (e_1, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    (e_val_exp, _) = Types::matchType(e_val_exp.clone(), tp.clone(), e_tp.clone(), false)?;
                    (e_val_exp, _) = ExpressionSimplify::simplify(e_val_exp.clone())?;
                    v = Ceval::cevalSimple(e_val_exp.clone())?;
                    e_val = Some(v.clone());
                    Ok((cache.clone(), Arc::new(DAE::Binding::EQBOUND { exp: e_1.clone(), evaluatedExp: e_val.clone(), constant_: c.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: e_val, properties: prop, modifierAsAbsynExp: _, .. }), .. }, e_tp) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    c = Types::propAllConst(prop.clone())?;
                    tp = Types::getPropType(prop.clone())?;
                    (e_1, _) = Types::matchType(e.clone(), tp.clone(), e_tp.clone(), false)?;
                    (e_1, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Binding::EQBOUND { exp: e_1.clone(), evaluatedExp: e_val.clone(), constant_: c.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Mod::MOD { info, binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: prop, modifierAsAbsynExp: _, .. }), .. }, tp) => {
                    let mut e_tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_tp_str: ArcStr = arcstr::literal!("");
                    let mut tp_str: ArcStr = arcstr::literal!("");
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut e_str_1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    e_tp = Types::getPropType(prop.clone())?;
                    let _ = Types::propAllConst(prop.clone())?;
                    if '__try0: {
                        (_, _) = unwrap_break_err!(Types::matchType(e.clone(), e_tp.clone(), tp.clone(), false), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    e_tp_str = (TypesDump::unparseTypeNoAttr(e_tp.clone())?).clone();
                    tp_str = (TypesDump::unparseTypeNoAttr(tp.clone())?).clone();
                    e_str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    e_str_1 = (stringAppend((literal!("=")).clone(), (e_str.clone()).clone())).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(inPrefix.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*componentName.clone()); ArcStr::from(__mm_s) }).clone();
                    Types::typeErrorSanityCheck((e_tp_str.clone()).clone(), (tp_str.clone()).clone(), info.clone())?;
                    Error::addSourceMessage(Error::MODIFIER_TYPE_MISMATCH_ERROR.clone(), list![(r#str.clone()).clone(), (tp_str.clone()).clone(), (e_str_1.clone()).clone(), (e_tp_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.makeBinding failed on component:")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(inPrefix.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*componentName.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outBinding))
}

pub fn makeRecordBinding(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inRecordName: Arc<Absyn::Path>, mut inRecordType: Arc<DAE::Type>, mut inRecordVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Binding>> {
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut accum_exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut accum_vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut accum_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut mods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = inMods.clone();
    let mut opt_mod: Option<Arc<DAE::SubMod>> = None;
    let mut name: ArcStr = literal!("");
    let mut scope: ArcStr = arcstr::literal!("");
    let mut ty_str: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut exp: Arc<DAE::Exp>;
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    dims = TypesDump::getDimensions(inRecordType.clone());
    match '__try0: {
        for mut var in &*inRecordVars.clone() {
            let mut var = var.clone();
            let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(var.clone()) {
                Deref @ DAE::Var { binding: __pa1, ty: __pa2, name: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            binding = __pa1.clone();
            ty = __pa2.clone();
            name = __pa3.clone();
            (mods, opt_mod) = unwrap_break_err!(List::deleteMemberOnTrue((name.clone()).clone(), mods.clone(), (std::sync::Arc::new(fnptr!(InstUtil::isSubModNamed, ArcStr, Arc<DAE::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::SubMod>) -> Result<bool> + 'static>)), '__try0);
            if isSome(opt_mod.clone()) {
                ty = Types::liftArrayListDims(ty.clone(), dims.clone());
                (exp, val) = unwrap_break_err!(makeRecordBinding3(opt_mod.clone(), ty.clone(), inInfo.clone()), '__try0);
            } else if DAEUtil::isBound(binding.clone()) {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(binding.clone()) {
                    Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(__pa4), exp: __pa5, .. } => (__pa4.clone(), __pa5.clone()),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                val = __pa4.clone();
                exp = __pa5.clone();
            } else {
                ety = unwrap_break_err!(Types::simplifyType(ty.clone()), '__try0);
                ty = Types::liftArrayListDims(ty.clone(), dims.clone());
                scope = (unwrap_break_err!(FGraph::printGraphPathStr(inEnv.clone()), '__try0)).clone();
                ty_str = (unwrap_break_err!(TypesDump::printTypeStr(ty.clone()), '__try0)).clone();
                exp = Arc::new(DAE::Exp::EMPTY { scope: (scope.clone()).clone(), name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ety.clone(), subscriptLst: metamodelica::nil() }), ty: ety.clone(), tyStr: (ty_str.clone()).clone() });
                val = Arc::new(Values::Value::EMPTY { scope: (scope.clone()).clone(), name: (name.clone()).clone(), ty: unwrap_break_err!(Types::typeToValue(ty.clone()), '__try0), tyStr: (ty_str.clone()).clone() });
            }
            accum_exps = cons(exp.clone(), accum_exps.clone());
            accum_vals = cons(val.clone(), accum_vals.clone());
            accum_names = cons((name.clone()).clone(), accum_names.clone());
        }
        ety = unwrap_break_err!(Types::simplifyType(Types::arrayElementType(inRecordType.clone())), '__try0);
        exp = Arc::new(DAE::Exp::CALL { path: inRecordName.clone(), expLst: accum_exps.clone().reverse(), attr: Arc::new(DAE::CallAttributes { ty: ety.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
        val = Arc::new(Values::Value::RECORD { record_: inRecordName.clone(), orderd: accum_vals.clone().reverse(), comp: accum_names.clone().reverse(), index: -1 });
        (exp, val) = unwrap_break_err!(InstUtil::liftRecordBinding(inRecordType.clone(), exp.clone(), val.clone()), '__try0);
        outBinding = Arc::new(DAE::Binding::EQBOUND { exp: exp.clone(), evaluatedExp: Some(val.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_RECORD_SUBMODS });
        Ok::<_, anyhow::Error>((ety.clone(), exp.clone(), outBinding.clone(), val.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            ety = __try0_o0;
            exp = __try0_o1;
            outBinding = __try0_o2;
            val = __try0_o3;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.makeRecordBinding2 failed for ")); __mm_s.push_str(&*AbsynUtil::pathString(inRecordName.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            bail!("fail");
        }
    }
    Ok(outBinding)
}

fn makeRecordBinding3(mut inSubMod: Option<Arc<DAE::SubMod>>, mut inType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<(Arc<DAE::Exp>, Arc<Values::Value>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outExp, outValue) = 'mc: {
        let __mc_input = inSubMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsValue: Some(val), modifierAsExp: exp, .. }), eachPrefix: SCode::Each::EACH, .. }, .. }) => {
                    Ok((exp.clone(), val.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { properties: DAE::Properties::PROP { type_: ty, .. }, modifierAsValue: Some(val), modifierAsExp: exp, .. }), eachPrefix: SCode::Each::NOT_EACH, .. }, .. }) => {
                    let mut ty = (*ty).clone();
                    let mut exp = (*exp).clone();
                    (exp, ty) = Types::matchType(exp.clone(), ty.clone(), inType.clone(), true)?;
                    Ok((exp.clone(), val.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { properties: DAE::Properties::PROP { type_: ty, .. }, modifierAsValue: None, modifierAsExp: exp, .. }), eachPrefix: SCode::Each::NOT_EACH, .. }, .. }) => {
                    let mut ty = (*ty).clone();
                    let mut exp = (*exp).clone();
                    (exp, ty) = Types::matchType(exp.clone(), ty.clone(), inType.clone(), true)?;
                    Ok((exp.clone(), Arc::new(Values::Value::OPTION { some: None })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { properties: DAE::Properties::PROP { type_: ty, .. }, modifierAsExp: exp, .. }), .. }, ident }) => {
                    let mut binding_str: ArcStr = arcstr::literal!("");
                    let mut expected_type_str: ArcStr = arcstr::literal!("");
                    let mut given_type_str: ArcStr = arcstr::literal!("");
                    binding_str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
                    expected_type_str = (TypesDump::unparseTypeNoAttr(inType.clone())?).clone();
                    given_type_str = (TypesDump::unparseTypeNoAttr(ty.clone())?).clone();
                    Types::typeErrorSanityCheck((given_type_str.clone()).clone(), (expected_type_str.clone()).clone(), inInfo.clone())?;
                    Error::addSourceMessage(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone(), list![(ident.clone()).clone(), (binding_str.clone()).clone(), (expected_type_str.clone()).clone(), (given_type_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outValue))
}

pub fn makeVariableBinding(mut inType: Arc<DAE::Type>, mut inMod: Arc<DAE::Mod>, mut inConst: DAE::Const, mut inPrefix: DAE::Prefix, mut inName: ArcStr) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outBinding: Option<Arc<DAE::Exp>> = None;
    let mut oeq_mod: Option<DAE::EqMod> = Mod::modEquation(inMod.clone())?;
    let mut e: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut p: DAE::Properties;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut e_str: ArcStr = arcstr::literal!("");
    let mut et_str: ArcStr = arcstr::literal!("");
    let mut bt_str: ArcStr = arcstr::literal!("");
    if isNone(oeq_mod.clone()) {
        outBinding = None;
        return Ok(outBinding);
    }
    let Some(DAE::TYPED { properties: __pa0, modifierAsExp: __pa1, .. }) = (oeq_mod.clone()) else { bail!("pattern mismatch") };
    p = __pa0.clone();
    e = __pa1.clone();
    if Types::isExternalObject(inType.clone()) {
        outBinding = Some(e.clone());
    } else if Types::isEmptyArray(Types::getPropType(p.clone())?) {
        outBinding = None;
    } else {
        info = Mod::getModInfo(inMod.clone());
        if let Ok((__pa2, DAE::PROP { constFlag: __pa3, .. })) = Types::matchProp(e.clone(), p.clone(), DAE::Properties::PROP { type_: inType.clone(), constFlag: inConst.clone() }, true) {
            e2 = __pa2.clone();
            c = __pa3.clone();
        } else {
            e_str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            et_str = (TypesDump::unparseTypeNoAttr(inType.clone())?).clone();
            bt_str = (TypesDump::unparseTypeNoAttr(Types::getPropType(p.clone())?)?).clone();
            Types::typeErrorSanityCheck((et_str.clone()).clone(), (bt_str.clone()).clone(), info.clone())?;
            Error::addSourceMessageAndFail(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone(), list![(inName.clone()).clone(), (e_str.clone()).clone(), (et_str.clone()).clone(), (bt_str.clone()).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        InstUtil::checkHigherVariability(inConst.clone(), c.clone(), inPrefix.clone(), (inName.clone()).clone(), e.clone(), info.clone())?;
        outBinding = Some(e2.clone());
    }
    Ok(outBinding)
}

