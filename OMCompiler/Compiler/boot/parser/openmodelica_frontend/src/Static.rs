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

use crate::BackendCevalInterface;
use crate::Ceval;
use crate::ComponentReference;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use crate::Inline;
use crate::InnerOuter;
use crate::Inst;
use crate::InstFunction;
use crate::InstMeta;
use crate::Lookup;
use crate::OperatorOverloading;
use crate::Patternm;
use crate::PrefixUtil;
use crate::Types;
use crate::ValuesUtil;
use crate::VarTransform;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::BackendInterface;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::MetaUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub const SLOT_NOT_EVALUATED: i32 = 0;

pub const SLOT_EVALUATING: i32 = 1;

pub const SLOT_EVALUATED: i32 = 2;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot {
    /// The slots default argument.
    pub defaultArg: Arc<DAE::FuncArg>,
    /// True if the slot has been filled, otherwise false.
    pub slotFilled: bool,
    /// The argument for the slot given by the function call.
    pub arg: Option<Arc<DAE::Exp>>,
    /// The dimensions of the slot.
    pub dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>,
    /// The index of the slot, 1 = first slot etc.
    pub idx: i32,
    pub evalStatus: i32,
}

impl Default for Slot {
    fn default() -> Self {
        Self {
            defaultArg: Default::default(),
            slotFilled: Default::default(),
            arg: Default::default(),
            dims: Default::default(),
            idx: Default::default(),
            evalStatus: Default::default(),
        }
    }
}

pub type SLOT = Slot;


thread_local! { static __BUILTIN_TIME_TLS: Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> = Some((Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("time")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() }), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, DAE::dummyAttrInput().clone())); }
pub fn BUILTIN_TIME() -> Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> { __BUILTIN_TIME_TLS.with(|__t| __t.clone()) }

pub fn elabExpList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut inLastType: Arc<DAE::Type>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<DAE::Properties>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outProperties: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut exp: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut last_ty: Arc<DAE::Type> = inLastType.clone();
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        let () = 'mc: {
        let __mc_input = (e.clone(), last_ty.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr @ Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } }, Deref @ DAE::Type::T_ENUMERATION { names, path: path2, .. }) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut path1: Arc<Absyn::Path>;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut idx: i32 = 0;
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    path = AbsynUtil::crefToPath(cr.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynUtil::splitQualAndIdentPath(path.clone())?) {
                        (__pa0, Deref @ Absyn::Path::IDENT { name: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path1 = __pa0.clone();
                    name = __pa1.clone();
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    idx = List::position((name.clone()).clone(), names.clone())?;
                    exp = Arc::new(DAE::Exp::ENUM_LITERAL { name: path.clone(), index: idx.clone() });
                    prop = DAE::Properties::PROP { type_: last_ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut prop: DAE::Properties;
                    let mut last_ty: Arc<DAE::Type> = last_ty.clone();
                    let mut exp: Arc<DAE::Exp>;
                    (outCache, exp, prop) = elabExpInExpression(outCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
                    last_ty = Types::getPropType(prop.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        outExpl = cons(exp.clone(), outExpl.clone());
        outProperties = cons(prop.clone(), outProperties.clone());
    }
    outExpl = outExpl.clone().reverse();
    outProperties = outProperties.clone().reverse();
    Ok((outCache, outExpl, outProperties))
}

fn elabExpList_enum(mut inExp: Arc<Absyn::Exp>, mut inLastType: Arc<DAE::Type>) -> Result<i32> {
    let mut outIndex: i32 = 0;
    outIndex = 'mc: {
        let __mc_input = (inExp.clone(), inLastType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr @ Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } }, Deref @ DAE::Type::T_ENUMERATION { names, path: path2, .. }) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut path1: Arc<Absyn::Path>;
                    let mut name: ArcStr = arcstr::literal!("");
                    path = AbsynUtil::crefToPath(cr.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynUtil::splitQualAndIdentPath(path.clone())?) {
                        (__pa0, Deref @ Absyn::Path::IDENT { name: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path1 = __pa0.clone();
                    name = __pa1.clone();
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    Ok(List::position((name.clone()).clone(), names.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(-1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIndex)
}

pub fn elabExpListList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut inLastType: Arc<DAE::Type>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<DAE::Properties>>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut outProperties: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Properties>>>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut last_ty: Arc<DAE::Type> = inLastType.clone();
    for mut lst in &*inExpl.clone() {
        let mut lst = lst.clone();
        (outCache, expl, props) = elabExpList(outCache.clone(), inEnv.clone(), lst.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), last_ty.clone())?;
        outExpl = cons(expl.clone(), outExpl.clone());
        outProperties = cons(props.clone(), outProperties.clone());
        last_ty = Types::getPropType(listHead(props.clone())?)?;
    }
    outExpl = outExpl.clone().reverse();
    outProperties = outProperties.clone().reverse();
    Ok((outCache, outExpl, outProperties))
}

fn elabExpOptAndMatchType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Option<Arc<Absyn::Exp>>, mut inDefaultType: Arc<DAE::Type>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Option<Arc<DAE::Exp>>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Option<Arc<DAE::Exp>> = None;
    let mut outProperties: DAE::Properties;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut dexp: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    outProperties = DAE::Properties::PROP { type_: inDefaultType.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
    if isSome(inExp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(inExp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
        (outCache, dexp, prop) = elabExpInExpression(outCache.clone(), inEnv.clone(), exp.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
        (dexp, outProperties) = Types::matchProp(dexp.clone(), prop.clone(), outProperties.clone(), true)?;
        outExp = Some(dexp.clone());
    } else {
        outExp = None;
    }
    Ok((outCache, outExp, outProperties))
}

pub fn elabExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut num_errmsgs: i32 = 0;
    let mut elabfunc: PartialElabExpFunc;
    e = if (BackendInterface::noRewriteRulesFrontEnd()) {inExp.clone()} else {(BackendInterface::rewriteFrontEnd(inExp.clone())).0};
    num_errmsgs = Error::getNumErrorMessages();
    match '__try0: {
        elabfunc = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::Exp::END => {
            unwrap_break_err!(Error::addSourceMessage(Error::END_ILLEGAL_USE_ERROR.clone(), metamodelica::nil(), inInfo.clone()), '__try0);
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"))
        },
        Deref @ Absyn::Exp::CREF { .. } => (std::sync::Arc::new(elabExp_Cref) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::BINARY { .. } => (std::sync::Arc::new(elabExp_Binary) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::UNARY { .. } => (std::sync::Arc::new(elabExp_Unary) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::LBINARY { .. } => (std::sync::Arc::new(elabExp_Binary) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::LUNARY { .. } => (std::sync::Arc::new(elabExp_LUnary) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::RELATION { .. } => (std::sync::Arc::new(elabExp_Binary) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::IFEXP { .. } => (std::sync::Arc::new(elabExp_If) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::CALL { .. } => (std::sync::Arc::new(elabExp_Call) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::PARTEVALFUNCTION { .. } => (std::sync::Arc::new(elabExp_PartEvalFunction) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::TUPLE { .. } => (std::sync::Arc::new(elabExp_Tuple) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::RANGE { .. } => (std::sync::Arc::new(elabExp_Range) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::ARRAY { .. } => (std::sync::Arc::new(elabExp_Array) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::MATRIX { .. } => (std::sync::Arc::new(elabExp_Matrix) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::CODE { .. } => (std::sync::Arc::new(elabExp_Code) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::CONS { .. } => (std::sync::Arc::new(elabExp_Cons) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::LIST { .. } => (std::sync::Arc::new(elabExp_List) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::MATCHEXP { .. } => (std::sync::Arc::new(Patternm::elabMatchExpression) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::DOT { .. } => (std::sync::Arc::new(elabExp_Dot) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ Absyn::Exp::EXPRESSIONCOMMENT { .. } => (std::sync::Arc::new(elabExp_Comment) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        _ => (std::sync::Arc::new(elabExp_BuiltinType) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (outCache, outExp, outProperties) = unwrap_break_err!(elabfunc(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone()), '__try0);
        Ok::<_, anyhow::Error>((elabfunc.clone(), outCache.clone(), outExp.clone(), outProperties.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            elabfunc = __try0_o0;
            outCache = __try0_o1;
            outExp = __try0_o2;
            outProperties = __try0_o3;
        }
        Err(_) => {
            let true = (num_errmsgs.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
            Error::addSourceMessage(Error::GENERIC_ELAB_EXPRESSION.clone(), list![(Dump::printExpStr(e.clone())?).clone()], inInfo.clone())?;
            bail!("fail");
        }
    }
    Ok((outCache, outExp, outProperties))
}

pub type PartialElabExpFunc = std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::Exp>, bool, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>;

fn elabExp_BuiltinType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::INTEGER { .. } => (Arc::new(DAE::Exp::ICONST { integer: var_field!((*inExp).value, Absyn::Exp::INTEGER).clone() }), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }),
        Deref @ Absyn::Exp::REAL { .. } => (Arc::new(DAE::Exp::RCONST { real: stringReal((var_field!((*inExp).value, Absyn::Exp::REAL).clone()).clone())? }), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }),
        Deref @ Absyn::Exp::STRING { .. } => (Arc::new(DAE::Exp::SCONST { string: (System::unescapedString((var_field!((*inExp).value, Absyn::Exp::STRING).clone()).clone())).clone() }), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }),
        Deref @ Absyn::Exp::BOOL { .. } => (Arc::new(DAE::Exp::BCONST { bool: var_field!((*inExp).value, Absyn::Exp::BOOL).clone() }), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }),
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Cref(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let __pa0 = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr = __pa0.clone();
    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabCref(inCache.clone(), inEnv.clone(), cr.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
        (__pa1, Some((__pa2, __pa3, _))) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa1.clone();
    outExp = __pa2.clone();
    outProperties = __pa3.clone();
    if !(Flags::getConfigBool(Flags::CEVAL_EQUATION.clone())?) {
        let DAE::PROP { type_: __pa4, constFlag: __pa5 } = (outProperties.clone()) else { bail!("pattern mismatch") };
        ty = __pa4.clone();
        c = __pa5.clone();
        outProperties = if (Types::isParameter(c.clone())) {DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }} else {outProperties.clone()};
    }
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Binary(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut op: Absyn::Operator = Absyn::Operator::ADD;
    let mut prop1: DAE::Properties;
    let mut prop2: DAE::Properties;
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let () = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::BINARY { exp2: e2, op, exp1: e1 } => (),
        Deref @ Absyn::Exp::LBINARY { exp2: e2, op, exp1: e1 } => (),
        Deref @ Absyn::Exp::RELATION { exp2: e2, op, exp1: e1 } => (),
        _ => bail!("match: no arm matched"),
    } });
    (outCache, exp1, prop1) = elabExpInExpression(inCache.clone(), inEnv.clone(), e1.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
    (outCache, exp2, prop2) = elabExpInExpression(outCache.clone(), inEnv.clone(), e2.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
    (outCache, outExp, outProperties) = OperatorOverloading::binary(outCache.clone(), inEnv.clone(), op.clone(), prop1.clone(), exp1.clone(), prop2.clone(), exp2.clone(), inExp.clone(), e1.clone(), e2.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Unary(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut op: Absyn::Operator = Absyn::Operator::ADD;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::UNARY { exp: __pa0, op: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    op = __pa1.clone();
    let (__pa2, __pa3, __pa6, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
        (__pa2, __pa3, __pa6 @ DAE::Properties::PROP { type_: __pa4, constFlag: __pa5 }) => (__pa2.clone(), __pa3.clone(), __pa6.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa2.clone();
    outExp = __pa3.clone();
    ty = __pa4.clone();
    c = __pa5.clone();
    outProperties = __pa6.clone();
    if !(op.clone() == openmodelica_ast::Absyn::Operator::UPLUS && Types::isIntegerOrRealOrSubTypeOfEither(Types::arrayElementType(ty.clone()))?) {
        (outCache, outExp, outProperties) = OperatorOverloading::unary(outCache.clone(), inEnv.clone(), op.clone(), outProperties.clone(), outExp.clone(), inExp.clone(), e.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    Ok((outCache, outExp, outProperties))
}

fn elabExp_LUnary(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut op: Absyn::Operator = Absyn::Operator::ADD;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::LUNARY { exp: __pa0, op: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    op = __pa1.clone();
    (outCache, outExp, outProperties) = elabExpInExpression(outCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
    (outCache, outExp, outProperties) = OperatorOverloading::unary(outCache.clone(), inEnv.clone(), op.clone(), outProperties.clone(), outExp.clone(), inExp.clone(), e.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp_If(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut cond_e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut true_e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut false_e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut cond_exp: Arc<DAE::Exp>;
    let mut true_exp: Arc<DAE::Exp>;
    let mut false_exp: Arc<DAE::Exp>;
    let mut cond_prop: DAE::Properties;
    let mut true_prop: DAE::Properties;
    let mut false_prop: DAE::Properties;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut b: bool = false;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(AbsynUtil::canonIfExp(inExp.clone())?) {
        Deref @ Absyn::Exp::IFEXP { elseBranch: __pa0, trueBranch: __pa1, ifExp: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    false_e = __pa0.clone();
    true_e = __pa1.clone();
    cond_e = __pa2.clone();
    (cache, cond_exp, cond_prop) = elabExpInExpression(inCache.clone(), inEnv.clone(), cond_e.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut true_prop: DAE::Properties;
            let mut true_exp: Arc<DAE::Exp>;
            let mut false_exp: Arc<DAE::Exp>;
            let mut outCache: FCore::Cache = outCache.clone();
            let mut outExp: Arc<DAE::Exp>;
            let mut outProperties: DAE::Properties;
            let mut false_prop: DAE::Properties;
            ErrorExt::setCheckpoint((literal!("Static.elabExp:IFEXP")).clone());
            (outCache, true_exp, true_prop) = elabExpInExpression(cache.clone(), inEnv.clone(), true_e.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
            (outCache, false_exp, false_prop) = elabExpInExpression(outCache.clone(), inEnv.clone(), false_e.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
            (outCache, outExp, outProperties) = makeIfExp(outCache.clone(), inEnv.clone(), cond_exp.clone(), cond_prop.clone(), true_exp.clone(), true_prop.clone(), false_exp.clone(), false_prop.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
            ErrorExt::delCheckpoint((literal!("Static.elabExp:IFEXP")).clone());
            Ok((outCache.clone(), outExp.clone(), outProperties.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool = b.clone();
            let mut outCache: FCore::Cache = outCache.clone();
            let mut outExp: Arc<DAE::Exp>;
            let mut outProperties: DAE::Properties;
            ErrorExt::setCheckpoint((literal!("Static.elabExp:IFEXP:HACK")).clone());
            let true = (Types::isParameterOrConstant(Types::propAllConst(cond_prop.clone())?)) else { bail!("pattern mismatch") };
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(cache.clone(), inEnv.clone(), cond_exp.clone(), inImplicit.clone(), Absyn::Msg::MSG { info: inInfo.clone() }, 0)?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            outCache = __pa0.clone();
            b = __pa1.clone();
            (outCache, outExp, outProperties) = elabExpInExpression(outCache.clone(), inEnv.clone(), if (b.clone()) {true_e.clone()} else {false_e.clone()}, inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
            ErrorExt::delCheckpoint((literal!("Static.elabExp:IFEXP:HACK")).clone());
            ErrorExt::rollBack((literal!("Static.elabExp:IFEXP")).clone());
            Ok((outCache.clone(), outExp.clone(), outProperties.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            ErrorExt::rollBack((literal!("Static.elabExp:IFEXP:HACK")).clone());
            ErrorExt::delCheckpoint((literal!("Static.elabExp:IFEXP")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Call(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut func_name: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut args: Arc<Absyn::FunctionArgs>;
    let mut type_vars: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CALL { typeVars: __pa0, functionArgs: __pa1, function_: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    type_vars = __pa0.clone();
    args = __pa1.clone();
    func_name = __pa2.clone();
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. } => {
            (outCache, outExp, outProperties) = elabCall(inCache.clone(), inEnv.clone(), func_name.clone(), var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone(), var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone(), type_vars.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
            (outExp, _) = ExpressionSimplify::simplify1(outExp.clone())?;
            ()
        },
        Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. } => {
            (outCache, outExp, outProperties) = elabCallReduction(inCache.clone(), inEnv.clone(), func_name.clone(), var_field!((*args).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), var_field!((*args).iterType, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), var_field!((*args).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Dot(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::DOT { .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            s = ((::match_deref::match_deref! { match &(var_field!((*inExp).index, Absyn::Exp::DOT).clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: s, .. } } => s.clone(),
        _ => {
            Error::addSourceMessage(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dot operator is only allowed when indexing using a single simple name, got: ")); __mm_s.push_str(&*Dump::printExpStr(var_field!((*inExp).index, Absyn::Exp::DOT).clone())?); ArcStr::from(__mm_s) }).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
            (outCache, outExp, outProperties) = elabExp(inCache.clone(), inEnv.clone(), var_field!((*inExp).exp, Absyn::Exp::DOT).clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
            ty = Types::getPropType(outProperties.clone())?;
            let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_TUPLE { names: Some(names), .. } => {
            let mut i: i32 = 0;
            if !(listMember((s.clone()).clone(), names.clone())) {
                Error::addSourceMessage(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dot operator could not find ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*TypesDump::unparseType(ty.clone())?); ArcStr::from(__mm_s) }).clone()], inInfo.clone())?;
                bail!("fail");
            }
            i = List::position((s.clone()).clone(), names.clone())?;
            outExp = Arc::new(DAE::Exp::TSUB { exp: outExp.clone(), ix: i.clone(), ty: (var_field!((*ty).types, DAE::Type::T_TUPLE).clone()).get(i.clone())? });
            outProperties = DAE::Properties::PROP { type_: (var_field!((*ty).types, DAE::Type::T_TUPLE).clone()).get(i.clone())?, constFlag: Types::propAllConst(outProperties.clone())? };
            ()
        },
        _ => {
            Error::addSourceMessage(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dot operator is only allowed when the expression returns a named tuple. Got expression: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outExp.clone())?); __mm_s.push_str(&*literal!(" with type ")); __mm_s.push_str(&*TypesDump::unparseType(ty.clone())?); ArcStr::from(__mm_s) }).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (outExp.clone(), outProperties.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Comment(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let __pa0 = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    (outCache, outExp, outProperties) = elabExp(inCache.clone(), inEnv.clone(), exp.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp_PartEvalFunction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut pos_args: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    let mut path: Arc<Absyn::Path>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut consts: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: __pa0, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __pa1, argNames: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cref = __pa0.clone();
    pos_args = __pa1.clone();
    named_args = __pa2.clone();
    if pos_args.clone().is_empty() && named_args.clone().is_empty() {
        (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), Arc::new(Absyn::Exp::CREF { componentRef: cref.clone() }), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
    } else {
        path = AbsynUtil::crefToPath(cref.clone())?;
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(inCache.clone(), inEnv.clone(), path.clone(), inInfo.clone())?) {
            (__pa4, Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil }) => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        outCache = __pa4.clone();
        tty = __pa5.clone();
        tty = Types::makeFunctionPolymorphicReference(tty.clone())?;
        (outCache, args, consts, _, tty, _, slots) = elabTypes(outCache.clone(), inEnv.clone(), pos_args.clone(), named_args.clone(), metamodelica::nil(), list![tty.clone()], true, true, inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
        if !(Types::isFunctionPointer(tty.clone())) {
            (outCache, path) = Inst::makeFullyQualified(outCache.clone(), inEnv.clone(), path.clone())?;
            let (__pa7, Util::SUCCESS { .. }) = (instantiateDaeFunction(outCache.clone(), inEnv.clone(), path.clone(), false, None, true)?) else { bail!("pattern mismatch") };
            outCache = __pa7.clone();
        }
        tty2 = stripExtraArgsFromType(slots.clone(), tty.clone())?;
        tty2 = Types::makeFunctionPolymorphicReference(tty2.clone())?;
        ty = Types::simplifyType(tty2.clone())?;
        tty = Types::simplifyType(tty.clone())?;
        c = List::fold(consts.clone(), (std::sync::Arc::new(fnptr!(Types::constAnd, DAE::Const, DAE::Const)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Const, DAE::Const) -> Result<DAE::Const> + 'static>), openmodelica_frontend_types::DAE::Const::C_CONST);
        outExp = Arc::new(DAE::Exp::PARTEVALFUNCTION { path: path.clone(), expList: args.clone(), ty: ty.clone(), origType: tty.clone() });
        outProperties = DAE::Properties::PROP { type_: tty2.clone(), constFlag: c.clone() };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Tuple(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabExp_Tuple_LHS_RHS(inCache.clone(), inEnv.clone(), inExp.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), false)?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Tuple_LHS_RHS(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut isLhs: bool) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut el: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut consts: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::TUPLE { expressions: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    el = __pa0.clone();
    if (el.clone().len() as i32) == 1 {
        (outCache, outExp, outProperties) = elabExp(outCache.clone(), inEnv.clone(), (el.clone()).get(1)?, inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
        return Ok((outCache.clone(), outExp.clone(), outProperties.clone()));
    }
    (outCache, expl, props) = elabTuple(outCache.clone(), inEnv.clone(), el.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), isLhs.clone())?;
    (types, consts) = splitProps(props.clone())?;
    (outExp, outProperties) = fixTupleMetaModelica(expl.clone(), types.clone(), consts.clone())?;
    Ok((outCache, outExp, outProperties))
}

pub fn elabExpLHS(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::TUPLE { .. } => {
            (outCache, outExp, outProperties) = elabExp_Tuple_LHS_RHS(inCache.clone(), inEnv.clone(), inExp.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), true)?;
            (outCache.clone(), outExp.clone(), outProperties.clone())
        },
        _ => {
            (outCache, outExp, outProperties) = elabExp(inCache.clone(), inEnv.clone(), inExp.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
            (outCache.clone(), outExp.clone(), outProperties.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Range(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut start: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut step: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut stop: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut ostep: Option<Arc<Absyn::Exp>> = None;
    let mut start_exp: Arc<DAE::Exp>;
    let mut step_exp: Arc<DAE::Exp>;
    let mut stop_exp: Arc<DAE::Exp>;
    let mut ostep_exp: Option<Arc<DAE::Exp>> = None;
    let mut start_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut step_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut stop_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ostep_ty: Option<Arc<DAE::Type>> = None;
    let mut start_c: DAE::Const = DAE::Const::C_CONST;
    let mut step_c: DAE::Const = DAE::Const::C_CONST;
    let mut stop_c: DAE::Const = DAE::Const::C_CONST;
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::RANGE { stop: __pa0, step: __pa1, start: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stop = __pa0.clone();
    ostep = __pa1.clone();
    start = __pa2.clone();
    let (__pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), start.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
        (__pa3, __pa4, DAE::Properties::PROP { type_: __pa5, constFlag: __pa6 }) => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa3.clone();
    start_exp = __pa4.clone();
    start_ty = __pa5.clone();
    start_c = __pa6.clone();
    let (__pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache.clone(), inEnv.clone(), stop.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
        (__pa7, __pa8, DAE::Properties::PROP { type_: __pa9, constFlag: __pa10 }) => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa7.clone();
    stop_exp = __pa8.clone();
    stop_ty = __pa9.clone();
    stop_c = __pa10.clone();
    c = Types::constAnd(start_c.clone(), stop_c.clone());
    if isSome(ostep.clone()) {
        let __pa11 = ::match_deref::match_deref! { match &(ostep.clone()) {
            Some(__pa11) => __pa11.clone(),
            _ => bail!("pattern mismatch"),
        } };
        step = __pa11.clone();
        let (__pa12, __pa13, __pa14, __pa15) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache.clone(), inEnv.clone(), step.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
            (__pa12, __pa13, DAE::Properties::PROP { type_: __pa14, constFlag: __pa15 }) => (__pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone()),
            _ => bail!("pattern mismatch"),
        } };
        outCache = __pa12.clone();
        step_exp = __pa13.clone();
        step_ty = __pa14.clone();
        step_c = __pa15.clone();
        ostep_exp = Some(step_exp.clone());
        ostep_ty = Some(step_ty.clone());
        c = Types::constAnd(c.clone(), step_c.clone());
    }
    if Types::isBoxedType(start_ty.clone()) {
        (start_exp, start_ty) = Types::matchType(start_exp.clone(), start_ty.clone(), Types::unboxedType(start_ty.clone())?, true)?;
    }
    if Types::isBoxedType(stop_ty.clone()) {
        (stop_exp, stop_ty) = Types::matchType(stop_exp.clone(), stop_ty.clone(), Types::unboxedType(stop_ty.clone())?, true)?;
    }
    (start_exp, ostep_exp, stop_exp, ety) = deoverloadRange(start_exp.clone(), start_ty.clone(), ostep_exp.clone(), ostep_ty.clone(), stop_exp.clone(), stop_ty.clone(), inInfo.clone())?;
    (outCache, ty) = elabRangeType(outCache.clone(), inEnv.clone(), start_exp.clone(), ostep_exp.clone(), stop_exp.clone(), start_ty.clone(), ety.clone(), c.clone(), inImplicit.clone())?;
    outExp = Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: start_exp.clone(), step: ostep_exp.clone(), stop: stop_exp.clone() });
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Array(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut es: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut arr_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut exp: Arc<DAE::Exp>;
    (outExp, outProperties) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Nil } => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { bail!("guard") }
                    Ok((Arc::new(DAE::Exp::LIST { valList: metamodelica::nil() }), DAE::Properties::PROP { type_: DAE::T_METALIST_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::ARRAY { arrayExp: es } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    let mut arr_ty: Arc<DAE::Type> = arr_ty.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut props: Arc<metamodelica::List<DAE::Properties>> = props.clone();
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = expl.clone();
                    let mut c: DAE::Const = c.clone();
                    (outCache, expl, props) = elabExpList(inCache.clone(), inEnv.clone(), es.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabArray(expl.clone(), props.clone(), inPrefix.clone(), inInfo.clone())?) {
                        (__pa0, DAE::Properties::PROP { type_: __pa1, constFlag: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl = __pa0.clone();
                    ty = __pa1.clone();
                    c = __pa2.clone();
                    arr_ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: (expl.clone().len() as i32) })] });
                    exp = Arc::new(DAE::Exp::ARRAY { ty: Types::simplifyType(arr_ty.clone())?, scalar: !(Types::isArray(ty.clone())), array: expl.clone() });
                    InstMeta::checkArrayType(ty.clone())?;
                    exp = elabMatrixToMatrixExp(exp.clone())?;
                    Ok((exp.clone(), DAE::Properties::PROP { type_: arr_ty.clone(), constFlag: c.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::ARRAY { arrayExp: es } => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { bail!("guard") }
                    let mut outProperties: DAE::Properties;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outExp: Arc<DAE::Exp>;
                    (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), Arc::new(Absyn::Exp::LIST { exps: es.clone() }), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
                    Ok((outExp.clone(), outProperties.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Matrix(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ess: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
    let mut dess: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut props: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Properties>>>> = metamodelica::nil();
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut nmax: i32 = 0;
    let mut have_real: bool = false;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut dim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let __pa0 = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::MATRIX { matrix: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ess = __pa0.clone();
    (outCache, dess, props) = elabExpListList(inCache.clone(), inEnv.clone(), ess.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
    tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut pl in (props.clone()).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut p in (pl.clone()).into_iter().cloned() {
            let __x = Types::getPropType(p.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = __x.append(&__acc);
        }
        __acc
    });
    nmax = matrixConstrMaxDim(tys.clone())?;
    have_real = Types::containReal(tys.clone());
    if have_real.clone() {
        (dess, props) = List::threadMapList_2(dess.clone(), props.clone(), (std::sync::Arc::new(elabExp_Matrix_realCast) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, DAE::Properties) -> Result<(Arc<DAE::Exp>, DAE::Properties)> + 'static>))?;
    }
    let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(elabMatrixSemi(outCache.clone(), inEnv.clone(), dess.clone(), props.clone(), inImplicit.clone(), have_real.clone(), nmax.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
        (__pa1, __pa2, DAE::Properties::PROP { type_: __pa3, constFlag: __pa4 }, __pa5, __pa6) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa1.clone();
    outExp = __pa2.clone();
    ty = __pa3.clone();
    c = __pa4.clone();
    dim1 = __pa5.clone();
    dim2 = __pa6.clone();
    outExp = elabMatrixToMatrixExp(outExp.clone())?;
    ty = Types::unliftArray(Types::unliftArray(ty.clone())?)?;
    ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim2.clone()] });
    ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim1.clone()] });
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Matrix_realCast(mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Types::getPropType(inProperties.clone())?;
    if Types::isInteger(ty.clone()) {
        ty = Types::setArrayElementType(ty.clone(), DAE::T_REAL_DEFAULT().clone());
        outProperties = Types::setPropType(inProperties.clone(), ty.clone())?;
        ty = Types::simplifyType(ty.clone())?;
        (outExp, _) = ExpressionSimplify::simplify1(Arc::new(DAE::Exp::CAST { ty: ty.clone(), exp: inExp.clone() }))?;
    } else {
        outExp = inExp.clone();
        outProperties = inProperties.clone();
    }
    Ok((outExp, outProperties))
}

fn elabExp_Code(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut cn: Arc<Absyn::CodeNode>;
    let __pa0 = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CODE { code: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cn = __pa0.clone();
    ty = elabCodeType(cn.clone())?;
    ty2 = Types::simplifyType(ty.clone())?;
    outExp = Arc::new(DAE::Exp::CODE { code: cn.clone(), ty: ty2.clone() });
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Cons(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let mut prop1: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c1: DAE::Const = DAE::Const::C_CONST;
    let mut c2: DAE::Const = DAE::Const::C_CONST;
    let mut exp_str: ArcStr = arcstr::literal!("");
    let mut ty1_str: ArcStr = arcstr::literal!("");
    let mut ty2_str: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CONS { head: __pa0, rest: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(MetaUtil::transformArrayNodesToListNodes(list![e1.clone(), e2.clone()])) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa2.clone();
    e2 = __pa3.clone();
    (outCache, exp1, prop1) = elabExpInExpression(outCache.clone(), inEnv.clone(), e1.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
    let (__pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache.clone(), inEnv.clone(), e2.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
        (__pa5, __pa6, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_METALIST { ty: __pa7 }, constFlag: __pa8 }) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa5.clone();
    exp2 = __pa6.clone();
    ty2 = __pa7.clone();
    c2 = __pa8.clone();
    match '__try10: {
        ty1 = unwrap_break_err!(Types::getUniontypeIfMetarecordReplaceAllSubtypes(Types::getPropType(prop1.clone())?), '__try10);
        ty2 = unwrap_break_err!(Types::getUniontypeIfMetarecordReplaceAllSubtypes(ty2.clone()), '__try10);
        c1 = unwrap_break_err!(Types::propAllConst(prop1.clone()), '__try10);
        ty = unwrap_break_err!(Types::getUniontypeIfMetarecordReplaceAllSubtypes(Types::superType(Types::boxIfUnboxedType(ty1.clone())?, Types::boxIfUnboxedType(ty2.clone())?)?), '__try10);
        (exp1, _) = unwrap_break_err!(Types::matchType(exp1.clone(), ty1.clone(), ty.clone(), true), '__try10);
        ty = Arc::new(DAE::Type::T_METALIST { ty: ty.clone() });
        (exp2, _) = unwrap_break_err!(Types::matchType(exp2.clone(), ty.clone(), Arc::new(DAE::Type::T_METALIST { ty: ty2.clone() }), true), '__try10);
        outExp = Arc::new(DAE::Exp::CONS { car: exp1.clone(), cdr: exp2.clone() });
        outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: Types::constAnd(c1.clone(), c2.clone()) };
        Ok::<_, anyhow::Error>((c1.clone(), exp1.clone(), exp2.clone(), outExp.clone(), outProperties.clone(), ty.clone(), ty1.clone(), ty2.clone()))
    } {
        Ok((__try10_o0, __try10_o1, __try10_o2, __try10_o3, __try10_o4, __try10_o5, __try10_o6, __try10_o7)) => {
            c1 = __try10_o0;
            exp1 = __try10_o1;
            exp2 = __try10_o2;
            outExp = __try10_o3;
            outProperties = __try10_o4;
            ty = __try10_o5;
            ty1 = __try10_o6;
            ty2 = __try10_o7;
        }
        Err(_) => {
            exp_str = (Dump::printExpStr(inExp.clone())?).clone();
            ty1_str = (TypesDump::unparseType(Types::getPropType(prop1.clone())?)?).clone();
            ty2_str = (TypesDump::unparseType(ty2.clone())?).clone();
            Error::addSourceMessage(Error::META_CONS_TYPE_MATCH.clone(), list![(exp_str.clone()).clone(), (ty1_str.clone()).clone(), (ty2_str.clone()).clone()], inInfo.clone())?;
            bail!("fail");
        }
    }
    Ok((outCache, outExp, outProperties))
}

fn elabExp_List(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut es: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut consts: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::LIST { exps: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    es = __pa0.clone();
    if es.clone().is_empty() {
        outExp = Arc::new(DAE::Exp::LIST { valList: metamodelica::nil() });
        outProperties = DAE::Properties::PROP { type_: DAE::T_METALIST_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
    } else {
        (outCache, expl, props) = elabExpList(inCache.clone(), inEnv.clone(), es.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
        types = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut p in (props.clone()).into_iter().cloned() {
            let __x = Types::getPropType(p.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        consts = Types::getConstList(props.clone())?;
        c = List::fold(consts.clone(), (std::sync::Arc::new(fnptr!(Types::constAnd, DAE::Const, DAE::Const)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Const, DAE::Const) -> Result<DAE::Const> + 'static>), openmodelica_frontend_types::DAE::Const::C_CONST);
        ty = Types::boxIfUnboxedType(List::reduce(types.clone(), (std::sync::Arc::new(Types::superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>))?)?;
        (expl, _) = Types::matchTypes(expl.clone(), types.clone(), ty.clone(), true)?;
        outExp = Arc::new(DAE::Exp::LIST { valList: expl.clone() });
        outProperties = DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_METALIST { ty: ty.clone() }), constFlag: c.clone() };
    }
    Ok((outCache, outExp, outProperties))
}

pub fn elabExpInExpression(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabExp(inCache.clone(), inEnv.clone(), inExp.clone(), inImplicit.clone(), performVectorization.clone(), inPrefix.clone(), info.clone())?;
    (outExp, outProperties) = elabExpInExpression2(outExp.clone(), outProperties.clone());
    Ok((outCache, outExp, outProperties))
}

fn elabExpInExpression2(mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties) -> (Arc<DAE::Exp>, DAE::Properties) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (::match_deref::match_deref! { match &(inProperties.clone()) {
        DAE::Properties::PROP_TUPLE { tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c }, tail: _ } }, type_: Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: ty, tail: _ }, .. } } => {
            (Arc::new(DAE::Exp::TSUB { exp: inExp.clone(), ix: 1, ty: ty.clone() }), DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() })
        },
        _ => {
            (inExp.clone(), inProperties.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outProperties)
}

pub fn checkAssignmentToInput(mut inExp: Arc<Absyn::Exp>, mut inAttributes: Arc<DAE::Attributes>, mut inEnv: FCore::Graph, mut inAllowTopLevelInputs: bool, mut inInfo: SourceInfo) -> Result<()> {
    if !(inAllowTopLevelInputs.clone()) && FGraph::inFunctionScope(inEnv.clone())? && !(Config::acceptParModelicaGrammar()?) {
        checkAssignmentToInput2(inExp.clone(), inAttributes.clone(), inInfo.clone())?;
    }
    Ok(())
}

fn checkAssignmentToInput2(mut inExp: Arc<Absyn::Exp>, mut inAttributes: Arc<DAE::Attributes>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inExp.clone(), inAttributes.clone())) {
        (Deref @ Absyn::Exp::CREF { componentRef: cr }, Deref @ DAE::Attributes { direction: Absyn::Direction::INPUT, .. }) => {
            let mut cr_str: ArcStr = arcstr::literal!("");
            cr_str = (Dump::printComponentRefStr(cr.clone())?).clone();
            Error::addSourceMessage(Error::ASSIGN_READONLY_ERROR.clone(), list![(literal!("input")).clone(), (cr_str.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkAssignmentToInputs(mut inExpCrefs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inAttributes: Arc<metamodelica::List<Arc<DAE::Attributes>>>, mut inEnv: FCore::Graph, mut inInfo: SourceInfo) -> Result<()> {
    if FGraph::inFunctionScope(inEnv.clone())? {
        List::threadMap1_0(inExpCrefs.clone(), inAttributes.clone(), (std::sync::Arc::new(checkAssignmentToInput2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<DAE::Attributes>, SourceInfo) -> Result<()> + 'static>), inInfo.clone())?;
    }
    Ok(())
}

pub fn elabExpCrefNoEvalList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<DAE::Properties>>, Arc<metamodelica::List<Arc<DAE::Attributes>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outProperties: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut outAttributes: Arc<metamodelica::List<Arc<DAE::Attributes>>> = metamodelica::nil();
    let mut num_err: i32 = Error::getNumErrorMessages();
    let mut exp: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        if '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ Absyn::Exp::CREF { componentRef: __pa1 } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            cr = __pa1.clone();
            (outCache, exp, prop, attr) = unwrap_break_err!(elabCrefNoEval(outCache.clone(), inEnv.clone(), cr.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone()), '__try0);
            outExpl = cons(exp.clone(), outExpl.clone());
            outAttributes = cons(attr.clone(), outAttributes.clone());
            props = cons(prop.clone(), props.clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            let true = (num_err.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
            Error::addSourceMessage(Error::GENERIC_ELAB_EXPRESSION.clone(), list![(Dump::printExpStr(e.clone())?).clone()], inInfo.clone())?;
        }
    }
    if !(Flags::getConfigBool(Flags::CEVAL_EQUATION.clone())?) {
        for mut p in &*props.clone() {
            let mut p = p.clone();
            let DAE::PROP { type_: __pa2, constFlag: __pa3 } = (p.clone()) else { bail!("pattern mismatch") };
            ty = __pa2.clone();
            c = __pa3.clone();
            p = if (Types::isParameter(c.clone())) {DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }} else {p.clone()};
            outProperties = cons(p.clone(), outProperties.clone());
        }
    } else {
        outProperties = props.clone().reverse();
    }
    outExpl = outExpl.clone().reverse();
    outAttributes = outAttributes.clone().reverse();
    Ok((outCache, outExpl, outProperties, outAttributes))
}

// Part of MetaModelica extension
pub fn elabListExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpList: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inProp: DAE::Properties, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = inExpList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inCache.clone(), Arc::new(DAE::Exp::LIST { valList: metamodelica::nil() }), inProp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut outProperties: DAE::Properties;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let DAE::PROP { type_: __t1, constFlag: __pa0 } = (inProp.clone()) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(__t1.clone()) {
                        Deref @ DAE::Type::T_METALIST { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    c = __pa0.clone();
                    (outCache, expl, props) = elabExpList(inCache.clone(), inEnv.clone(), inExpList.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    types = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut p in (props.clone()).into_iter().cloned() {
                    let __x = Types::getPropType(p.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    (expl, ty) = Types::listMatchSuperType(expl.clone(), types.clone(), true)?;
                    outProperties = DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_METALIST { ty: ty.clone() }), constFlag: c.clone() };
                    Ok((outCache.clone(), Arc::new(DAE::Exp::LIST { valList: expl.clone() }), outProperties.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- Static.elabListExp failed, non-matching args in list constructor?")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

/* ------------------------------- */
pub fn fromEquationsToAlgAssignments(mut cp: Arc<Absyn::ClassPart>) -> Result<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>> {
    let mut algsOut: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
    algsOut = (::match_deref::match_deref! { match &(cp.clone()) {
        Deref @ Absyn::ClassPart::ALGORITHMS { contents: alg } => {
            alg.clone()
        },
        Deref @ Absyn::ClassPart::EQUATIONS { contents: rest } => {
            fromEquationsToAlgAssignmentsWork(rest.clone())?
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (Dump::unparseClassPart(cp.clone())?).clone();
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Static.fromEquationsToAlgAssignments: Unknown classPart in match expression:\n")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(algsOut)
}

fn fromEquationsToAlgAssignmentsWork(mut eqsIn: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>> {
    let mut algsOut: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
    for mut ei in &*eqsIn.clone() {
        let mut ei = ei.clone();
        let () = (::match_deref::match_deref! { match &(ei.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { info, comment, equation_: eq } => {
            let mut algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            algs = fromEquationToAlgAssignment(eq.clone(), comment.clone(), info.clone())?;
            algsOut = listAppend(algs.clone(), algsOut.clone());
            ()
        },
        Deref @ Absyn::EquationItem::EQUATIONITEMCOMMENT { .. } => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    algsOut = algsOut.clone().reverse();
    Ok(algsOut)
}

fn fromEquationBranchesToAlgBranches(mut eqsIn: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>> {
    let mut algsOut: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    let mut algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
    for mut branch in &*eqsIn.clone() {
        let mut branch = branch.clone();
        (e, eqs) = branch.clone();
        algs = fromEquationsToAlgAssignmentsWork(eqs.clone())?;
        algsOut = cons((e.clone(), algs.clone()), algsOut.clone());
    }
    algsOut = algsOut.clone().reverse();
    Ok(algsOut)
}

fn fromEquationToAlgAssignment(mut eq: Arc<Absyn::Equation>, mut comment: Option<Arc<Absyn::Comment>>, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>> {
    let mut algStatement: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
    algStatement = 'mc: {
        let __mc_input = eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: strLeft, subscripts: Deref @ metamodelica::List::Nil } }, rightSide: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: strRight, subscripts: Deref @ metamodelica::List::Nil } } } => {
                    let true = (strLeft.clone() == strRight.clone()) else { bail!("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: left, rightSide: right } => {
                    let mut algItem1: Arc<Absyn::AlgorithmItem>;
                    let mut algItem2: Arc<Absyn::AlgorithmItem>;
                    ::match_deref::match_deref! { match &(AbsynUtil::stripCommentExpressions(right.clone(), false)?) {
                        Deref @ Absyn::Exp::BOOL { value: true } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if '__try0: {
                        ::match_deref::match_deref! { match &(left.clone()) {
                            Deref @ Absyn::Exp::CREF { componentRef: _ } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    algItem1 = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("fail")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }) }), comment: comment.clone(), info: info.clone() });
                    algItem2 = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_IF { ifExp: Arc::new(Absyn::Exp::LUNARY { op: openmodelica_ast::Absyn::Operator::NOT, exp: left.clone() }), trueBranch: list![algItem1.clone()], elseIfAlgorithmBranch: metamodelica::nil(), elseBranch: metamodelica::nil() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem2.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: left, rightSide: Deref @ Absyn::Exp::BOOL { value: false } } => {
                    let mut algItem1: Arc<Absyn::AlgorithmItem>;
                    let mut algItem2: Arc<Absyn::AlgorithmItem>;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(left.clone()) {
                            Deref @ Absyn::Exp::CREF { componentRef: _ } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    algItem1 = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("fail")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }) }), comment: comment.clone(), info: info.clone() });
                    algItem2 = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_IF { ifExp: left.clone(), trueBranch: list![algItem1.clone()], elseIfAlgorithmBranch: metamodelica::nil(), elseBranch: metamodelica::nil() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem2.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_PDE { .. } => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "fail", subscripts: _ }, functionArgs: _ } => {
                    let mut algItem: Arc<Absyn::AlgorithmItem>;
                    algItem = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("fail")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }) }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_NORETCALL { functionName: cref, functionArgs: fargs } => {
                    let mut algItem: Arc<Absyn::AlgorithmItem>;
                    algItem = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: cref.clone(), functionArgs: fargs.clone() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: left, rightSide: right } => {
                    let mut algItem: Arc<Absyn::AlgorithmItem>;
                    algItem = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: left.clone(), value: right.clone() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_FAILURE { equ: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq2, comment: comment2, info: info2 } } => {
                    let mut res: Arc<Absyn::AlgorithmItem>;
                    let mut algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    algs = fromEquationToAlgAssignment(eq2.clone(), comment2.clone(), info2.clone())?;
                    res = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_FAILURE { equ: algs.clone() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![res.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_IF { equationElseItems: eqElseItems, elseIfBranches: eqBranches, equationTrueItems: eqTrueItems, ifExp: e } => {
                    let mut res: Arc<Absyn::AlgorithmItem>;
                    let mut algTrueItems: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    let mut algElseItems: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    let mut algBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
                    algTrueItems = fromEquationsToAlgAssignmentsWork(eqTrueItems.clone())?;
                    algElseItems = fromEquationsToAlgAssignmentsWork(eqElseItems.clone())?;
                    algBranches = fromEquationBranchesToAlgBranches(eqBranches.clone())?;
                    res = Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_IF { ifExp: e.clone(), trueBranch: algTrueItems.clone(), elseIfAlgorithmBranch: algBranches.clone(), elseBranch: algElseItems.clone() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![res.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Dump::equationName(eq.clone())?).clone();
                    Error::addSourceMessage(Error::META_MATCH_EQUATION_FORBIDDEN.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(algStatement)
}

fn elabMatrixToMatrixExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, ty: a @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
                    let mut mexpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut d1: i32 = 0;
                    mexpl = List::map(expl.clone(), (std::sync::Arc::new(Expression::arrayContent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>));
                    d1 = (mexpl.clone().len() as i32);
                    let true = (Expression::typeBuiltin(Expression::unliftArray(Expression::unliftArray(a.clone())?)?)) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::MATRIX { ty: a.clone(), integer: d1.clone(), matrix: mexpl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn matrixConstrMaxDim(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<i32> {
    let mut outMaxDim: i32 = 2;
    for mut ty in &*inTypes.clone() {
        let mut ty = ty.clone();
        outMaxDim = std::cmp::max(Types::numberOfDimensions(ty.clone())?, outMaxDim.clone());
    }
    Ok(outMaxDim)
}

fn elabCallReduction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inReductionFn: Arc<Absyn::ComponentRef>, mut inReductionExp: Arc<Absyn::Exp>, mut inIterType: Absyn::ReductionIterType, mut inIterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut env: FCore::Graph;
    let mut fold_env: FCore::Graph;
    let mut reduction_iters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut iter_const: DAE::Const = DAE::Const::C_CONST;
    let mut exp_const: DAE::Const = DAE::Const::C_CONST;
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut has_guard_exp: bool = false;
    let mut exp: Arc<DAE::Exp>;
    let mut afold_exp: Option<Arc<Absyn::Exp>> = None;
    let mut fold_exp: Option<Arc<DAE::Exp>> = None;
    let mut exp_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut res_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut r#fn: Arc<Absyn::Path>;
    let mut v: Option<Arc<Values::Value>> = None;
    let mut fold_id: ArcStr = arcstr::literal!("");
    let mut res_id: ArcStr = arcstr::literal!("");
    match '__try0: {
        env = unwrap_break_err!(FGraph::openScope(inEnv.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (arcstr::literal!(FCore::forIterScopeName)).clone(), None), '__try0);
        (outCache, env, reduction_iters, dims, iter_const, has_guard_exp) = unwrap_break_err!(elabCallReductionIterators(inCache.clone(), env.clone(), inIterators.clone(), inReductionExp.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone()), '__try0);
        dims = unwrap_break_err!(fixDimsIterType(inIterType.clone(), dims.clone()), '__try0);
        let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(elabExpInExpression(outCache.clone(), env.clone(), inReductionExp.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone()), '__try0)) {
            (__pa1, __pa2, DAE::Properties::PROP { type_: __pa3, constFlag: __pa4 }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        outCache = __pa1.clone();
        exp = __pa2.clone();
        exp_ty = __pa3.clone();
        exp_const = __pa4.clone();
        c = Types::constAnd(exp_const.clone(), iter_const.clone());
        r#fn = (::match_deref::match_deref! { match &(inReductionFn.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "$array", subscripts: Deref @ metamodelica::List::Nil } => Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }),
        _ => unwrap_break_err!(AbsynUtil::crefToPath(inReductionFn.clone()), '__try0),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (outCache, exp, exp_ty, res_ty, v, r#fn) = unwrap_break_err!(reductionType(outCache.clone(), inEnv.clone(), r#fn.clone(), exp.clone(), exp_ty.clone(), Types::unboxedType(exp_ty.clone())?, dims.clone(), has_guard_exp.clone(), inInfo.clone()), '__try0);
        outProperties = DAE::Properties::PROP { type_: exp_ty.clone(), constFlag: c.clone() };
        fold_id = (Util::getTempVariableIndex()).clone();
        res_id = (Util::getTempVariableIndex()).clone();
        (fold_env, afold_exp) = unwrap_break_err!(makeReductionFoldExp(env.clone(), r#fn.clone(), exp_ty.clone(), res_ty.clone(), (fold_id.clone()).clone(), (res_id.clone()).clone()), '__try0);
        (outCache, fold_exp, _) = unwrap_break_err!(elabExpOptAndMatchType(outCache.clone(), fold_env.clone(), afold_exp.clone(), res_ty.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone()), '__try0);
        outExp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: r#fn.clone(), iterType: inIterType.clone(), exprType: exp_ty.clone(), defaultValue: v.clone(), foldName: (fold_id.clone()).clone(), resultName: (res_id.clone()).clone(), foldExp: fold_exp.clone() }), expr: exp.clone(), iterators: reduction_iters.clone() });
        Ok::<_, anyhow::Error>((afold_exp.clone(), c.clone(), dims.clone(), env.clone(), exp.clone(), exp_const.clone(), exp_ty.clone(), r#fn.clone(), fold_env.clone(), fold_exp.clone(), fold_id.clone(), has_guard_exp.clone(), iter_const.clone(), outCache.clone(), outExp.clone(), outProperties.clone(), reduction_iters.clone(), res_id.clone(), res_ty.clone(), v.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10, __try0_o11, __try0_o12, __try0_o13, __try0_o14, __try0_o15, __try0_o16, __try0_o17, __try0_o18, __try0_o19)) => {
            afold_exp = __try0_o0;
            c = __try0_o1;
            dims = __try0_o2;
            env = __try0_o3;
            exp = __try0_o4;
            exp_const = __try0_o5;
            exp_ty = __try0_o6;
            r#fn = __try0_o7;
            fold_env = __try0_o8;
            fold_exp = __try0_o9;
            fold_id = __try0_o10;
            has_guard_exp = __try0_o11;
            iter_const = __try0_o12;
            outCache = __try0_o13;
            outExp = __try0_o14;
            outProperties = __try0_o15;
            reduction_iters = __try0_o16;
            res_id = __try0_o17;
            res_ty = __try0_o18;
            v = __try0_o19;
        }
        Err(_) => {
            if (inIterators.clone().len() as i32) > 1 {
                Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Reductions using multiple iterators is not yet implemented. Try rewriting the expression using nested reductions (e.g. array(i+j for i, j) => array(array(i+j for i) for j).")).clone()], inInfo.clone())?;
            } else {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                Debug::traceln((literal!("Static.elabCallReduction - failed!")).clone())?;
            }
            bail!("fail");
        }
    }
    Ok((outCache, outExp, outProperties))
}

fn fixDimsIterType(mut iterType: Absyn::ReductionIterType, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    outDims = (match iterType.clone() {
        Absyn::ReductionIterType::COMBINE => dims.clone(),
        _ => list![listHead(dims.clone())?],
    });
    Ok(outDims)
}

fn elabCallReductionIterators(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut inReductionExp: Arc<Absyn::Exp>, mut inImpl: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, Arc<metamodelica::List<Arc<DAE::Dimension>>>, DAE::Const, bool)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outIteratorsEnv: FCore::Graph = inEnv.clone();
    let mut outIterators: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut outConst: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    let mut outHasGuard: bool = false;
    let mut iter_name: ArcStr = arcstr::literal!("");
    let mut aiter_exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut oaguard_exp: Option<Arc<Absyn::Exp>> = None;
    let mut oaiter_exp: Option<Arc<Absyn::Exp>> = None;
    let mut iter_exp: Arc<DAE::Exp>;
    let mut guard_exp: Option<Arc<DAE::Exp>> = None;
    let mut full_iter_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut iter_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut iter_const: DAE::Const = DAE::Const::C_CONST;
    let mut guard_const: DAE::Const = DAE::Const::C_CONST;
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut env: FCore::Graph;
    for mut iter in &*inIterators.clone() {
        let mut iter = iter.clone();
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(iter.clone()) {
            Deref @ Absyn::ForIterator { name: __pa0, guardExp: __pa1, range: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        iter_name = __pa0.clone();
        oaguard_exp = __pa1.clone();
        oaiter_exp = __pa2.clone();
        if isSome(oaiter_exp.clone()) {
            let __pa3 = ::match_deref::match_deref! { match &(oaiter_exp.clone()) {
                Some(__pa3) => __pa3.clone(),
                _ => bail!("pattern mismatch"),
            } };
            aiter_exp = __pa3.clone();
            let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache.clone(), inEnv.clone(), aiter_exp.clone(), inImpl.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
                (__pa4, __pa5, DAE::Properties::PROP { type_: __pa6, constFlag: __pa7 }) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            outCache = __pa4.clone();
            iter_exp = __pa5.clone();
            full_iter_ty = __pa6.clone();
            iter_const = __pa7.clone();
        } else {
            let (__pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(deduceIterationRange((iter_name.clone()).clone(), AbsynUtil::findIteratorIndexedCrefs(inReductionExp.clone(), (iter_name.clone()).clone(), metamodelica::nil())?, inEnv.clone(), outCache.clone(), inInfo.clone())?) {
                (__pa8, DAE::Properties::PROP { type_: __pa9, constFlag: __pa10 }, __pa11) => (__pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                _ => bail!("pattern mismatch"),
            } };
            iter_exp = __pa8.clone();
            full_iter_ty = __pa9.clone();
            iter_const = __pa10.clone();
            outCache = __pa11.clone();
        }
        c = if (FGraph::inFunctionScope(inEnv.clone())?) {iter_const.clone()} else {openmodelica_frontend_types::DAE::Const::C_CONST};
        (outCache, iter_exp, _) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), iter_exp.clone(), DAE::Properties::PROP { type_: full_iter_ty.clone(), constFlag: c.clone() }, inImpl.clone(), inInfo.clone())?;
        (iter_ty, dim) = Types::unliftArrayOrList(full_iter_ty.clone())?;
        env = FGraph::addForIterator(inEnv.clone(), (iter_name.clone()).clone(), iter_ty.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::CONST, Some(iter_const.clone()))?;
        outIteratorsEnv = FGraph::addForIterator(outIteratorsEnv.clone(), (iter_name.clone()).clone(), iter_ty.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::CONST, Some(iter_const.clone()))?;
        let (__pa12, __pa13, __pa14) = ::match_deref::match_deref! { match &(elabExpOptAndMatchType(outCache.clone(), env.clone(), oaguard_exp.clone(), DAE::T_BOOL_DEFAULT().clone(), inImpl.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
            (__pa12, __pa13, DAE::Properties::PROP { type_: _, constFlag: __pa14 }) => (__pa12.clone(), __pa13.clone(), __pa14.clone()),
            _ => bail!("pattern mismatch"),
        } };
        outCache = __pa12.clone();
        guard_exp = __pa13.clone();
        guard_const = __pa14.clone();
        if isSome(guard_exp.clone()) {
            outHasGuard = true;
            dim = Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN);
        }
        outConst = Types::constAnd(outConst.clone(), Types::constAnd(guard_const.clone(), iter_const.clone()));
        outIterators = cons(Arc::new(DAE::ReductionIterator { id: (iter_name.clone()).clone(), exp: iter_exp.clone(), guardExp: guard_exp.clone(), ty: iter_ty.clone() }), outIterators.clone());
        outDims = cons(dim.clone(), outDims.clone());
    }
    outIterators = outIterators.clone().reverse();
    outDims = outDims.clone().reverse();
    Ok((outCache, outIteratorsEnv, outIterators, outDims, outConst, outHasGuard))
}

pub fn deduceIterationRange(mut inIterator: ArcStr, mut inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>, mut inEnv: FCore::Graph, mut inCache: FCore::Cache, mut inInfo: SourceInfo) -> Result<(Arc<DAE::Exp>, DAE::Properties, FCore::Cache)> {
    let mut outRange: Arc<DAE::Exp> = Arc::new(DAE::Exp::ICONST { integer: 0 });
    let mut outProperties: DAE::Properties = DAE::Properties::PROP { type_: DAE::T_UNKNOWN_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_UNKNOWN };
    let mut outCache: FCore::Cache = inCache.clone();
    let mut acref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut idx: i32 = 0;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut range: Arc<DAE::Exp>;
    let mut ranges: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut cr_str1: ArcStr = arcstr::literal!("");
    let mut cr_str2: ArcStr = arcstr::literal!("");
    if inCrefs.clone().is_empty() {
        Error::addSourceMessageAndFail(Error::IMPLICIT_ITERATOR_NOT_FOUND_IN_LOOP_BODY.clone(), list![(inIterator.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    for mut cr in &*inCrefs.clone() {
        let mut cr = cr.clone();
        (acref, idx) = cr.clone();
        cref = ComponentReference::toExpCref(acref.clone())?;
        if let Ok((__pa0, _, __pa1, _, _, _, _, _, _)) = Lookup::lookupVar(outCache.clone(), inEnv.clone(), cref.clone()) {
            outCache = __pa0.clone();
            ty = __pa1.clone();
        } else {
            Error::addSourceMessageAndFail(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(Dump::printComponentRefStr(acref.clone())?).clone(), (literal!("")).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        dims = TypesDump::getDimensions(ty.clone());
        if idx.clone() <= (dims.clone().len() as i32) {
            dim = (dims.clone()).get(idx.clone())?;
            (range, outProperties) = deduceReductionIterationRange2(dim.clone(), cref.clone(), ty.clone(), idx.clone())?;
        } else {
            range = Arc::new(DAE::Exp::ICONST { integer: 0 });
            outProperties = DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 0 })] }), constFlag: openmodelica_frontend_types::DAE::Const::C_UNKNOWN };
        }
        ranges = cons(range.clone(), ranges.clone());
    }
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ranges.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outRange = __pa2.clone();
    ranges = __pa3.clone();
    idx = 2;
    for mut r in &*ranges.clone() {
        let mut r = r.clone();
        if !(ExpressionBasics::expEqual(r.clone(), outRange.clone())?) {
            (acref, i1) = listHead(inCrefs.clone())?;
            cr_str1 = (Dump::printComponentRefStr(acref.clone())?).clone();
            (acref, i2) = (inCrefs.clone()).get(idx.clone())?;
            cr_str2 = (Dump::printComponentRefStr(acref.clone())?).clone();
            Error::addSourceMessageAndFail(Error::INCOMPATIBLE_IMPLICIT_RANGES.clone(), list![(intString(i2.clone())).clone(), (cr_str2.clone()).clone(), (intString(i1.clone())).clone(), (cr_str1.clone()).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        idx = idx.clone() + 1;
    }
    Ok((outRange, outProperties, outCache))
}

fn iteratorIndexedCrefsEqual(mut inCref1: (Arc<Absyn::ComponentRef>, i32), mut inCref2: (Arc<Absyn::ComponentRef>, i32)) -> bool {
    let mut outEqual: bool = false;
    let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut cr2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut idx1: i32 = 0;
    let mut idx2: i32 = 0;
    (cr1, idx1) = inCref1.clone();
    (cr2, idx2) = inCref2.clone();
    outEqual = idx1.clone() == idx2.clone() && AbsynUtil::crefEqual(cr1.clone(), cr2.clone());
    outEqual
}

fn deduceReductionIterationRange_traverser(mut inExp: Arc<Absyn::Exp>, mut inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>, mut inIterator: ArcStr) -> (Arc<Absyn::Exp>, Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) {
    let mut outExp: Arc<Absyn::Exp> = inExp.clone();
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
    outCrefs = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: cref } => {
            getIteratorIndexedCrefs(cref.clone(), (inIterator.clone()).clone(), inCrefs.clone())
        },
        _ => {
            inCrefs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outCrefs)
}

fn getIteratorIndexedCrefs(mut inCref: Arc<Absyn::ComponentRef>, mut inIterator: ArcStr, mut inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = inCrefs.clone();
    let mut crefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
    outCrefs = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: subs, name: id } => {
            let mut idx: i32 = 0;
            let mut name: ArcStr = arcstr::literal!("");
            idx = 1;
            for mut sub in &*subs.clone() {
                let mut sub = sub.clone();
                let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name } } } => {
            if name.clone() == inIterator.clone() {
                outCrefs = cons((Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() }), idx.clone()), outCrefs.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                idx = idx.clone() + 1;
            }
            outCrefs.clone()
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cref, subscripts: subs, name: id } => {
            let mut idx: i32 = 0;
            let mut cref = (*cref).clone();
            crefs = getIteratorIndexedCrefs(cref.clone(), (inIterator.clone()).clone(), metamodelica::nil());
            for mut cr in &*crefs.clone() {
                let mut cr = cr.clone();
                (cref, idx) = cr.clone();
                outCrefs = cons((Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id.clone()).clone(), subscripts: subs.clone(), componentRef: cref.clone() }), idx.clone()), outCrefs.clone());
            }
            getIteratorIndexedCrefs(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: subs.clone() }), (inIterator.clone()).clone(), outCrefs.clone())
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut idx: i32 = 0;
            let mut cref = (*cref).clone();
            crefs = getIteratorIndexedCrefs(cref.clone(), (inIterator.clone()).clone(), metamodelica::nil());
            for mut cr in &*crefs.clone() {
                let mut cr = cr.clone();
                (cref, idx) = cr.clone();
                outCrefs = cons((Arc::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref.clone() }), idx.clone()), outCrefs.clone());
            }
            outCrefs.clone()
        },
        _ => {
            inCrefs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCrefs
}

fn deduceReductionIterationRange2(mut inDimension: Arc<DAE::Dimension>, mut inCref: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inIndex: i32) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outRange: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut range_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut range_const: DAE::Const = DAE::Const::C_CONST;
    let mut enum_path: Arc<Absyn::Path>;
    let mut enum_start: Arc<Absyn::Path>;
    let mut enum_end: Arc<Absyn::Path>;
    let mut enum_lits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut sz: i32 = 0;
    let mut size_exp: Arc<DAE::Exp>;
    outRange = (::match_deref::match_deref! { match &(inDimension.clone()) {
        Deref @ DAE::Dimension::DIM_BOOLEAN => {
            range_ty = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![inDimension.clone()] });
            range_const = openmodelica_frontend_types::DAE::Const::C_CONST;
            Arc::new(DAE::Exp::RANGE { ty: range_ty.clone(), start: Arc::new(DAE::Exp::BCONST { bool: false }), step: None, stop: Arc::new(DAE::Exp::BCONST { bool: true }) })
        },
        Deref @ DAE::Dimension::DIM_ENUM { literals: enum_lits, enumTypeName: enum_path, .. } => {
            enum_start = AbsynUtil::suffixPath(enum_path.clone(), (listHead(enum_lits.clone())?).clone())?;
            enum_end = AbsynUtil::suffixPath(enum_path.clone(), (List::last(enum_lits.clone())?).clone())?;
            range_ty = Arc::new(DAE::Type::T_ENUMERATION { index: None, path: enum_path.clone(), names: enum_lits.clone(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() });
            range_ty = Arc::new(DAE::Type::T_ARRAY { ty: range_ty.clone(), dims: list![inDimension.clone()] });
            range_const = openmodelica_frontend_types::DAE::Const::C_CONST;
            Arc::new(DAE::Exp::RANGE { ty: range_ty.clone(), start: Arc::new(DAE::Exp::ENUM_LITERAL { name: enum_start.clone(), index: 1 }), step: None, stop: Arc::new(DAE::Exp::ENUM_LITERAL { name: enum_end.clone(), index: (enum_lits.clone().len() as i32) }) })
        },
        Deref @ DAE::Dimension::DIM_INTEGER { integer: sz } => {
            range_ty = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![inDimension.clone()] });
            range_const = openmodelica_frontend_types::DAE::Const::C_CONST;
            Arc::new(DAE::Exp::RANGE { ty: range_ty.clone(), start: Arc::new(DAE::Exp::ICONST { integer: 1 }), step: None, stop: Arc::new(DAE::Exp::ICONST { integer: sz.clone() }) })
        },
        _ => {
            size_exp = Arc::new(DAE::Exp::SIZE { exp: Arc::new(DAE::Exp::CREF { componentRef: inCref.clone(), ty: inType.clone() }), sz: Some(Arc::new(DAE::Exp::ICONST { integer: inIndex.clone() })) });
            range_ty = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![inDimension.clone()] });
            range_const = openmodelica_frontend_types::DAE::Const::C_PARAM;
            Arc::new(DAE::Exp::RANGE { ty: range_ty.clone(), start: Arc::new(DAE::Exp::ICONST { integer: 1 }), step: None, stop: size_exp.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outProperties = DAE::Properties::PROP { type_: range_ty.clone(), constFlag: range_const.clone() };
    Ok((outRange, outProperties))
}

fn makeReductionFoldExp(mut inEnv: FCore::Graph, mut path: Arc<Absyn::Path>, mut expty: Arc<DAE::Type>, mut resultTy: Arc<DAE::Type>, mut foldId: ArcStr, mut resultId: ArcStr) -> Result<(FCore::Graph, Option<Arc<Absyn::Exp>>)> {
    let mut outEnv: FCore::Graph;
    let mut afoldExp: Option<Arc<Absyn::Exp>> = None;
    (outEnv, afoldExp) = (::match_deref::match_deref! { match &(AbsynUtil::makeNotFullyQualified(path.clone())) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "$array" } => {
            (inEnv.clone(), None)
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "array" } => {
            (inEnv.clone(), None)
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "list" } => {
            (inEnv.clone(), None)
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" } => {
            (inEnv.clone(), None)
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut cr2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut env: FCore::Graph;
            env = FGraph::addForIterator(inEnv.clone(), (foldId.clone()).clone(), expty.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            env = FGraph::addForIterator(env.clone(), (resultId.clone()).clone(), expty.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            cr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (foldId.clone()).clone(), subscripts: metamodelica::nil() });
            cr2 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (resultId.clone()).clone(), subscripts: metamodelica::nil() });
            exp = Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::CREF { componentRef: cr2.clone() }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: Arc::new(Absyn::Exp::CREF { componentRef: cr1.clone() }) });
            (env.clone(), Some(exp.clone()))
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "product" } => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut cr2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut env: FCore::Graph;
            env = FGraph::addForIterator(inEnv.clone(), (foldId.clone()).clone(), expty.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            env = FGraph::addForIterator(env.clone(), (resultId.clone()).clone(), expty.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            cr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (foldId.clone()).clone(), subscripts: metamodelica::nil() });
            cr2 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (resultId.clone()).clone(), subscripts: metamodelica::nil() });
            exp = Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::CREF { componentRef: cr2.clone() }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: Arc::new(Absyn::Exp::CREF { componentRef: cr1.clone() }) });
            (env.clone(), Some(exp.clone()))
        },
        _ => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut cr2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut env: FCore::Graph;
            cr = AbsynUtil::pathToCref(path.clone())?;
            env = FGraph::addForIterator(inEnv.clone(), (foldId.clone()).clone(), expty.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            env = FGraph::addForIterator(env.clone(), (resultId.clone()).clone(), resultTy.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            cr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (foldId.clone()).clone(), subscripts: metamodelica::nil() });
            cr2 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (resultId.clone()).clone(), subscripts: metamodelica::nil() });
            exp = Arc::new(Absyn::Exp::CALL { function_: cr.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![Arc::new(Absyn::Exp::CREF { componentRef: cr1.clone() }), Arc::new(Absyn::Exp::CREF { componentRef: cr2.clone() })], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() });
            (env.clone(), Some(exp.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEnv, afoldExp))
}

fn reductionType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFn: Arc<Absyn::Path>, mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut unboxedType: Arc<DAE::Type>, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut hasGuardExp: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, Arc<DAE::Type>, Arc<DAE::Type>, Option<Arc<Values::Value>>, Arc<Absyn::Path>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut resultType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut defaultValue: Option<Arc<Values::Value>> = None;
    let mut outPath: Arc<Absyn::Path>;
    let mut r#fn: Arc<Absyn::Path> = AbsynUtil::makeNotFullyQualified(inFn.clone());
    (outExp, outType, resultType, defaultValue, outPath) = (::match_deref::match_deref! { match &((r#fn.clone(), unboxedType.clone())) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ty = List::foldr(dims.clone(), (std::sync::Arc::new(fnptr!(Types::liftArray, Arc<DAE::Type>, Arc<DAE::Dimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Type>> + 'static>), inType.clone());
            (inExp.clone(), ty.clone(), ty.clone(), Some(Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] })), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "$array" }, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ty = List::foldr(dims.clone(), (std::sync::Arc::new(fnptr!(Types::liftArray, Arc<DAE::Type>, Arc<DAE::Dimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Type>> + 'static>), inType.clone());
            (inExp.clone(), ty.clone(), ty.clone(), Some(Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] })), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut exp: Arc<DAE::Exp>;
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_METABOXED_DEFAULT().clone(), true)?;
            ty = List::foldr(dims.clone(), (std::sync::Arc::new(fnptr!(Types::liftList, Arc<DAE::Type>, Arc<DAE::Dimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Type>> + 'static>), ty.clone());
            (exp.clone(), ty.clone(), ty.clone(), Some(Arc::new(Values::Value::LIST { valueLst: metamodelica::nil() })), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut exp: Arc<DAE::Exp>;
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_METABOXED_DEFAULT().clone(), true)?;
            ty = List::foldr(dims.clone(), (std::sync::Arc::new(fnptr!(Types::liftList, Arc<DAE::Type>, Arc<DAE::Dimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Type>> + 'static>), ty.clone());
            (exp.clone(), ty.clone(), ty.clone(), Some(Arc::new(Values::Value::LIST { valueLst: metamodelica::nil() })), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            r = System::realMaxLit();
            v = Arc::new(Values::Value::REAL { real: r.clone() });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            let mut i: i32 = 0;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            i = System::intMaxLit();
            v = Arc::new(Values::Value::INTEGER { integer: i.clone() });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_BOOL { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::BOOL { boolean: true });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_STRING { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut exp: Arc<DAE::Exp>;
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_STRING_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), None, r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((*unboxedType).path, DAE::Type::T_ENUMERATION).clone(), (List::last(var_field!((*unboxedType).names, DAE::Type::T_ENUMERATION).clone())?).clone())?, index: (var_field!((*unboxedType).names, DAE::Type::T_ENUMERATION).clone().len() as i32) });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_ENUMERATION_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            r = -(System::realMaxLit());
            v = Arc::new(Values::Value::REAL { real: r.clone() });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            let mut i: i32 = 0;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            i = intNeg(System::intMaxLit());
            v = Arc::new(Values::Value::INTEGER { integer: i.clone() });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_BOOL { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::BOOL { boolean: false });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_STRING { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::STRING { string: (literal!("")).clone() });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_STRING_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((*unboxedType).path, DAE::Type::T_ENUMERATION).clone(), (listHead(var_field!((*unboxedType).names, DAE::Type::T_ENUMERATION).clone())?).clone())?, index: 1 });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_ENUMERATION_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::INTEGER { integer: 0 });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_BOOL { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::BOOL { boolean: false });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_STRING { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::STRING { string: (literal!("")).clone() });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_STRING_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_ARRAY { .. }) => {
            (inExp.clone(), inType.clone(), inType.clone(), None, r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(1.0_f64) });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::INTEGER { integer: 1 });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_BOOL { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp>;
            v = Arc::new(Values::Value::BOOL { boolean: true });
            (exp, ty) = Types::matchType(inExp.clone(), inType.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
            (exp.clone(), ty.clone(), ty.clone(), Some(v.clone()), r#fn.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_STRING { .. }) => {
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("product reduction not defined for String")).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_ARRAY { .. }) => {
            (inExp.clone(), inType.clone(), inType.clone(), None, r#fn.clone())
        },
        _ => {
            let mut fnTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut typeA: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut typeB: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut path: Arc<Absyn::Path>;
            let mut exp: Arc<DAE::Exp>;
            let mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
            let mut defaultBinding: Option<Arc<Values::Value>> = None;
            (outCache, fnTypes) = Lookup::lookupFunctionsInEnv(inCache.clone(), inEnv.clone(), inFn.clone(), info.clone())?;
            (typeA, typeB, resType, defaultBinding, path) = checkReductionType1(inEnv.clone(), inFn.clone(), fnTypes.clone(), info.clone())?;
            ty2 = if (isSome(defaultBinding.clone())) {typeB.clone()} else {inType.clone()};
            (exp, typeA, bindings) = Types::matchTypePolymorphicWithError(inExp.clone(), inType.clone(), typeA.clone(), Some(path.clone()), metamodelica::nil(), info.clone())?;
            (_, typeB, bindings) = Types::matchTypePolymorphicWithError(Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$result")).clone(), identType: DAE::T_ANYTYPE_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_ANYTYPE_DEFAULT().clone() }), ty2.clone(), typeB.clone(), Some(path.clone()), bindings.clone(), info.clone())?;
            bindings = Types::solvePolymorphicBindings(bindings.clone(), info.clone(), path.clone())?;
            typeA = Types::fixPolymorphicRestype(typeA.clone(), bindings.clone(), info.clone())?;
            typeB = Types::fixPolymorphicRestype(typeB.clone(), bindings.clone(), info.clone())?;
            resType = Types::fixPolymorphicRestype(resType.clone(), bindings.clone(), info.clone())?;
            (exp, ty) = checkReductionType2(exp.clone(), inType.clone(), typeA.clone(), typeB.clone(), resType.clone(), Types::equivtypes(typeA.clone(), typeB.clone())? || isSome(defaultBinding.clone()), Types::equivtypes(typeB.clone(), resType.clone())?, info.clone())?;
            let (__pa0, Util::SUCCESS { .. }) = (instantiateDaeFunction(outCache.clone(), inEnv.clone(), path.clone(), false, None, true)?) else { bail!("pattern mismatch") };
            outCache = __pa0.clone();
            Error::assertionOrAddSourceMessage(Config::acceptMetaModelicaGrammar()? || Flags::isSet(Flags::EXPERIMENTAL_REDUCTIONS.clone())?, Error::COMPILER_NOTIFICATION.clone(), list![(literal!("Custom reduction functions are an OpenModelica extension to the Modelica Specification. Do not use them if you need your model to compile using other tools or if you are concerned about using experimental features. Use -d=experimentalReductions to disable this message.")).clone()], info.clone())?;
            (exp.clone(), ty.clone(), typeB.clone(), defaultBinding.clone(), path.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExp, outType, resultType, defaultValue, outPath))
}

fn checkReductionType1(mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut fnTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut info: SourceInfo) -> Result<(Arc<DAE::Type>, Arc<DAE::Type>, Arc<DAE::Type>, Option<Arc<Values::Value>>, Arc<Absyn::Path>)> {
    let mut typeA: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut typeB: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut startValue: Option<Arc<Values::Value>> = None;
    let mut outPath: Arc<Absyn::Path>;
    (typeA, typeB, resType, startValue, outPath) = (::match_deref::match_deref! { match &(fnTypes.clone()) {
        Deref @ metamodelica::List::Nil => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            str1 = (AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?).clone();
            str2 = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
            Error::addSourceMessage(Error::LOOKUP_FUNCTION_ERROR.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { path, funcResultType: resType, funcArg: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { r#const: DAE::Const::C_VAR, ty: typeA, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { defaultBinding: Some(e), r#const: DAE::Const::C_VAR, ty: typeB, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Nil } => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            v = Ceval::cevalSimple(e.clone())?;
            (typeA.clone(), typeB.clone(), resType.clone(), Some(v.clone()), path.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { path, funcResultType: resType, funcArg: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { r#const: DAE::Const::C_VAR, ty: typeA, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, r#const: DAE::Const::C_VAR, ty: typeB, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Nil } => {
            (typeA.clone(), typeB.clone(), resType.clone(), None, path.clone())
        },
        _ => {
            let mut str1: ArcStr = arcstr::literal!("");
            str1 = stringDelimitList(List::map(fnTypes.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
            Error::addSourceMessage(Error::UNSUPPORTED_REDUCTION_TYPE.clone(), list![(str1.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((typeA, typeB, resType, startValue, outPath))
}

fn checkReductionType2(mut inExp: Arc<DAE::Exp>, mut expType: Arc<DAE::Type>, mut typeA: Arc<DAE::Type>, mut typeB: Arc<DAE::Type>, mut typeC: Arc<DAE::Type>, mut equivAB: bool, mut equivBC: bool, mut info: SourceInfo) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outExp, outTy) = (match (equivAB.clone(), equivBC.clone()) {
        (true, true) => {
            (inExp.clone(), typeA.clone())
        },
        (_, false) => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            str1 = (TypesDump::unparseType(typeB.clone())?).clone();
            str2 = (TypesDump::unparseType(typeC.clone())?).clone();
            Error::addSourceMessage(Error::REDUCTION_TYPE_ERROR.clone(), list![(literal!("second argument")).clone(), (literal!("result-type")).clone(), (literal!("identical")).clone(), (str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (false, true) => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            str1 = (TypesDump::unparseType(typeA.clone())?).clone();
            str2 = (TypesDump::unparseType(typeB.clone())?).clone();
            Error::addSourceMessage(Error::REDUCTION_TYPE_ERROR.clone(), list![(literal!("first")).clone(), (literal!("second arguments")).clone(), (literal!("identical")).clone(), (str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (true, true) => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            str1 = (TypesDump::unparseType(expType.clone())?).clone();
            str2 = (TypesDump::unparseType(typeA.clone())?).clone();
            Error::addSourceMessage(Error::REDUCTION_TYPE_ERROR.clone(), list![(literal!("reduction expression")).clone(), (literal!("first argument")).clone(), (literal!("compatible")).clone(), (str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outExp, outTy))
}

fn constToVariability(mut r#const: DAE::Const) -> Result<SCode::Variability> {
    let mut variability: SCode::Variability = SCode::Variability::CONST;
    variability = (match r#const.clone() {
        DAE::Const::C_VAR => openmodelica_frontend_types::SCode::Variability::VAR,
        DAE::Const::C_PARAM => openmodelica_frontend_types::SCode::Variability::PARAM,
        DAE::Const::C_CONST => openmodelica_frontend_types::SCode::Variability::CONST,
        DAE::Const::C_UNKNOWN => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- Static.constToVariability failed on DAE.C_UNKNOWN()\n")).clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(variability)
}

fn constructArrayType(mut arrayType: Arc<DAE::Type>, mut expType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    resType = (::match_deref::match_deref! { match &(arrayType.clone()) {
        Deref @ DAE::Type::T_UNKNOWN => {
            expType.clone()
        },
        Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } } => {
            let mut ty = (*ty).clone();
            ty = constructArrayType(ty.clone(), expType.clone())?;
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(resType)
}

fn elabCodeType(mut inCode: Arc<Absyn::CodeNode>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inCode.clone()) {
        Deref @ Absyn::CodeNode::C_TYPENAME { .. } => Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_TYPENAME }),
        Deref @ Absyn::CodeNode::C_VARIABLENAME { .. } => Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME }),
        Deref @ Absyn::CodeNode::C_EQUATIONSECTION { .. } => Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EquationSection")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }),
        Deref @ Absyn::CodeNode::C_ALGORITHMSECTION { .. } => Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("AlgorithmSection")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }),
        Deref @ Absyn::CodeNode::C_ELEMENT { .. } => Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Element")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }),
        Deref @ Absyn::CodeNode::C_EXPRESSION { .. } => Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION }),
        Deref @ Absyn::CodeNode::C_MODIFICATION { .. } => Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION_OR_MODIFICATION }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

pub fn elabGraphicsExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::INTEGER { value: i }, _, _) => {
                    Ok((cache.clone(), Arc::new(DAE::Exp::ICONST { integer: i.clone() }), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::REAL { value: s }, _, _) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = stringReal((s.clone()).clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Exp::RCONST { real: r.clone() }), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::STRING { value: s }, _, _) => {
                    let mut s = (*s).clone();
                    s = (System::unescapedString((s.clone()).clone())).clone();
                    Ok((cache.clone(), Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() }), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::BOOL { value: b }, _, _) => {
                    Ok((cache.clone(), Arc::new(DAE::Exp::BCONST { bool: b.clone() }), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CREF { componentRef: cr }, r#impl, pre) => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCref(cache.clone(), env.clone(), cr.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2, _))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    dexp = __pa1.clone();
                    prop = __pa2.clone();
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::BINARY { exp2: e2, op, exp1: e1 }, r#impl, pre) => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e1_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, e2_1, prop2) = elabGraphicsExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, dexp, prop) = OperatorOverloading::binary(cache.clone(), env.clone(), op.clone(), prop1.clone(), e1_1.clone(), prop2.clone(), e2_1.clone(), exp.clone(), e1.clone(), e2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UPLUS, .. }, r#impl, pre) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e_1 = __pa1.clone();
                    t = __pa2.clone();
                    c = __pa3.clone();
                    let true = (Types::isRealOrSubTypeReal(Types::arrayElementType(t.clone()))?) else { bail!("pattern mismatch") };
                    prop = DAE::Properties::PROP { type_: t.clone(), constFlag: c.clone() };
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::UNARY { exp: e, op }, r#impl, pre) => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, dexp, prop) = OperatorOverloading::unary(cache.clone(), env.clone(), op.clone(), prop1.clone(), e_1.clone(), exp.clone(), e.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::LBINARY { exp2: e2, op, exp1: e1 }, r#impl, pre) => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e1_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, e2_1, prop2) = elabGraphicsExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, dexp, prop) = OperatorOverloading::binary(cache.clone(), env.clone(), op.clone(), prop1.clone(), e1_1.clone(), prop2.clone(), e2_1.clone(), exp.clone(), e1.clone(), e2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::LUNARY { exp: e, op }, r#impl, pre) => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, dexp, prop) = OperatorOverloading::unary(cache.clone(), env.clone(), op.clone(), prop1.clone(), e_1.clone(), exp.clone(), e.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::RELATION { exp2: e2, op, exp1: e1 }, r#impl, pre) => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e1_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, e2_1, prop2) = elabGraphicsExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, dexp, prop) = OperatorOverloading::binary(cache.clone(), env.clone(), op.clone(), prop1.clone(), e1_1.clone(), prop2.clone(), e2_1.clone(), exp.clone(), e1.clone(), e2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ Absyn::Exp::IFEXP { .. }, r#impl, pre) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e3_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut prop3: DAE::Properties;
                    let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut e3: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(AbsynUtil::canonIfExp(e.clone())?) {
                        Deref @ Absyn::Exp::IFEXP { elseBranch: __pa0, trueBranch: __pa1, ifExp: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e3 = __pa0.clone();
                    e2 = __pa1.clone();
                    e1 = __pa2.clone();
                    (cache, e1_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, e2_1, prop2) = elabGraphicsExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, e3_1, prop3) = elabGraphicsExp(cache.clone(), env.clone(), e3.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, e_1, prop) = makeIfExp(cache.clone(), env.clone(), e1_1.clone(), prop1.clone(), e2_1.clone(), prop2.clone(), e3_1.clone(), prop3.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: nargs, args }, function_: r#fn, .. }, _, pre) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = elabCall(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), var_field!((*inExp).typeVars, Absyn::Exp::CALL).clone(), true, pre.clone(), info.clone())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::TUPLE { expressions: es @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, r#impl, pre) => {
                    let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut consts: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, es_1, props) = elabTuple(cache.clone(), env.clone(), es.clone(), r#impl.clone(), false, pre.clone(), info.clone(), false)?;
                    (types, consts) = splitProps(props.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Exp::TUPLE { PR: es_1.clone() }), DAE::Properties::PROP_TUPLE { type_: Arc::new(DAE::Type::T_TUPLE { types: types.clone(), names: None }), tupleConst: Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: consts.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RANGE { stop, step: None, start }, r#impl, pre) => {
                    let mut start_1: Arc<DAE::Exp>;
                    let mut stop_1: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut start_t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut stop_t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c_start: DAE::Const = DAE::Const::C_CONST;
                    let mut c_stop: DAE::Const = DAE::Const::C_CONST;
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut rt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), start.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    start_1 = __pa1.clone();
                    start_t = __pa2.clone();
                    c_start = __pa3.clone();
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), stop.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa4, __pa5, DAE::Properties::PROP { type_: __pa6, constFlag: __pa7 }) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa4.clone();
                    stop_1 = __pa5.clone();
                    stop_t = __pa6.clone();
                    c_stop = __pa7.clone();
                    let __pa8 = ::match_deref::match_deref! { match &(deoverloadRange(start_1.clone(), start_t.clone(), None, None, stop_1.clone(), stop_t.clone(), info.clone())?) {
                        (_, None, _, __pa8) => __pa8.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rt = __pa8.clone();
                    r#const = Types::constAnd(c_start.clone(), c_stop.clone());
                    (cache, t) = elabRangeType(cache.clone(), env.clone(), start_1.clone(), None, stop_1.clone(), start_t.clone(), rt.clone(), r#const.clone(), r#impl.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Exp::RANGE { ty: t.clone(), start: start_1.clone(), step: None, stop: stop_1.clone() }), DAE::Properties::PROP { type_: t.clone(), constFlag: r#const.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RANGE { stop, step: Some(step), start }, r#impl, pre) => {
                    let mut start_1: Arc<DAE::Exp>;
                    let mut stop_1: Arc<DAE::Exp>;
                    let mut start_2: Arc<DAE::Exp>;
                    let mut stop_2: Arc<DAE::Exp>;
                    let mut step_1: Arc<DAE::Exp>;
                    let mut step_2: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut start_t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut stop_t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut step_t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut c_start: DAE::Const = DAE::Const::C_CONST;
                    let mut c_stop: DAE::Const = DAE::Const::C_CONST;
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut c_step: DAE::Const = DAE::Const::C_CONST;
                    let mut rt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), start.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    start_1 = __pa1.clone();
                    start_t = __pa2.clone();
                    c_start = __pa3.clone();
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), step.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa4, __pa5, DAE::Properties::PROP { type_: __pa6, constFlag: __pa7 }) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa4.clone();
                    step_1 = __pa5.clone();
                    step_t = __pa6.clone();
                    c_step = __pa7.clone();
                    let (__pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), stop.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa8, __pa9, DAE::Properties::PROP { type_: __pa10, constFlag: __pa11 }) => (__pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa8.clone();
                    stop_1 = __pa9.clone();
                    stop_t = __pa10.clone();
                    c_stop = __pa11.clone();
                    let (__pa12, __pa13, __pa14, __pa15) = ::match_deref::match_deref! { match &(deoverloadRange(start_1.clone(), start_t.clone(), Some(step_1.clone()), Some(step_t.clone()), stop_1.clone(), stop_t.clone(), info.clone())?) {
                        (__pa12, Some(__pa13), __pa14, __pa15) => (__pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    start_2 = __pa12.clone();
                    step_2 = __pa13.clone();
                    stop_2 = __pa14.clone();
                    rt = __pa15.clone();
                    c1 = Types::constAnd(c_start.clone(), c_step.clone());
                    r#const = Types::constAnd(c1.clone(), c_stop.clone());
                    (cache, t) = elabRangeType(cache.clone(), env.clone(), start_1.clone(), Some(step_1.clone()), stop_1.clone(), start_t.clone(), rt.clone(), r#const.clone(), r#impl.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Exp::RANGE { ty: t.clone(), start: start_2.clone(), step: Some(step_2.clone()), stop: stop_2.clone() }), DAE::Properties::PROP { type_: t.clone(), constFlag: r#const.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::ARRAY { arrayExp: es }, r#impl, pre) => {
                    let mut l: i32 = 0;
                    let mut a: bool = false;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsArray(cache.clone(), env.clone(), es.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    es_1 = __pa1.clone();
                    t = __pa2.clone();
                    r#const = __pa3.clone();
                    l = (es_1.clone().len() as i32);
                    at = Types::simplifyType(t.clone())?;
                    a = Types::isArray(t.clone());
                    Ok((cache.clone(), Arc::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: a.clone(), array: es_1.clone() }), DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: l.clone() })] }), constFlag: r#const.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::MATRIX { matrix: ess }, r#impl, pre) => {
                    let mut nmax: i32 = 0;
                    let mut dim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut havereal: bool = false;
                    let mut mexp: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t_2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut tps_2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tps: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Properties>>>> = metamodelica::nil();
                    let mut tps_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Type>>>>> = metamodelica::nil();
                    let mut dess: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, dess, tps) = elabExpListList(cache.clone(), env.clone(), ess.clone(), r#impl.clone(), true, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    tps_1 = List::mapList(tps.clone(), (std::sync::Arc::new(Types::getPropType) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Properties) -> Result<Arc<DAE::Type>> + 'static>));
                    tps_2 = List::flatten(tps_1.clone());
                    nmax = matrixConstrMaxDim(tps_2.clone())?;
                    havereal = Types::containReal(tps_2.clone());
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabMatrixSemi(cache.clone(), env.clone(), dess.clone(), tps.clone(), r#impl.clone(), havereal.clone(), nmax.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }, __pa4, __pa5) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    mexp = __pa1.clone();
                    t = __pa2.clone();
                    c = __pa3.clone();
                    dim1 = __pa4.clone();
                    dim2 = __pa5.clone();
                    elabMatrixToMatrixExp(mexp.clone())?;
                    t_1 = Types::unliftArray(t.clone())?;
                    t_2 = Types::unliftArray(t_1.clone())?;
                    Ok((cache.clone(), mexp.clone(), DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: t_2.clone(), dims: list![dim2.clone()] }), dims: list![dim1.clone()] }), constFlag: c.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, e, _, pre) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut ps: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Print::printErrorBuf((literal!("- Inst.elabGraphicsExp failed: ")).clone())?;
                    ps = (PrefixUtil::printPrefixStr2(pre.clone())?).clone();
                    s = (Dump::printExpStr(e.clone())?).clone();
                    Print::printErrorBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ps.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Print::printErrorBuf((literal!("\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn deoverloadRange(mut inStartExp: Arc<DAE::Exp>, mut inStartType: Arc<DAE::Type>, mut inStepExp: Option<Arc<DAE::Exp>>, mut inStepType: Option<Arc<DAE::Type>>, mut inStopExp: Arc<DAE::Exp>, mut inStopType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<(Arc<DAE::Exp>, Option<Arc<DAE::Exp>>, Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut outStart: Arc<DAE::Exp>;
    let mut outStep: Option<Arc<DAE::Exp>> = None;
    let mut outStop: Arc<DAE::Exp>;
    let mut outRangeType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outStart, outStep, outStop, outRangeType) = (::match_deref::match_deref! { match &((inStartType.clone(), inStepType.clone(), inStopType.clone())) {
        (Deref @ DAE::Type::T_BOOL { .. }, None, Deref @ DAE::Type::T_BOOL { .. }) => {
            (inStartExp.clone(), None, inStopExp.clone(), DAE::T_BOOL_DEFAULT().clone())
        },
        (Deref @ DAE::Type::T_INTEGER { .. }, None, Deref @ DAE::Type::T_INTEGER { .. }) => {
            (inStartExp.clone(), inStepExp.clone(), inStopExp.clone(), DAE::T_INTEGER_DEFAULT().clone())
        },
        (Deref @ DAE::Type::T_INTEGER { .. }, Some(Deref @ DAE::Type::T_INTEGER { .. }), Deref @ DAE::Type::T_INTEGER { .. }) => {
            (inStartExp.clone(), inStepExp.clone(), inStopExp.clone(), DAE::T_INTEGER_DEFAULT().clone())
        },
        (Deref @ DAE::Type::T_ENUMERATION { names: ns, .. }, None, Deref @ DAE::Type::T_ENUMERATION { names: ne, .. }) => {
            let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut e1_str: ArcStr = arcstr::literal!("");
            let mut e2_str: ArcStr = arcstr::literal!("");
            let mut t1_str: ArcStr = arcstr::literal!("");
            if List::isEqual(ns.clone(), ne.clone(), true) {
                et = Types::simplifyType(inStartType.clone())?;
            } else {
                e1_str = (ExpressionBasics::printExpStr(inStartExp.clone())?).clone();
                e2_str = (ExpressionBasics::printExpStr(inStopExp.clone())?).clone();
                t1_str = (TypesDump::unparseTypeNoAttr(inStartType.clone())?).clone();
                TypesDump::unparseTypeNoAttr(inStopType.clone())?;
                Error::addSourceMessageAndFail(Error::UNRESOLVABLE_TYPE.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*e1_str.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*e2_str.clone()); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*t1_str.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*t1_str.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()], inInfo.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            (inStartExp.clone(), None, inStopExp.clone(), et.clone())
        },
        (_, None, _) => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(OperatorOverloading::elabArglist(list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], list![(inStartExp.clone(), inStartType.clone()), (inStopExp.clone(), inStopType.clone())])?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } }, _) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            outStart = __pa0.clone();
            outStop = __pa1.clone();
            (outStart.clone(), None, outStop.clone(), DAE::T_REAL_DEFAULT().clone())
        },
        (_, Some(step_ty), _) => {
            let mut step_exp: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inStepExp.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            step_exp = __pa0.clone();
            let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(OperatorOverloading::elabArglist(list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], list![(inStartExp.clone(), inStartType.clone()), (step_exp.clone(), step_ty.clone()), (inStopExp.clone(), inStopType.clone())])?) {
                (Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } } }, _) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            outStart = __pa1.clone();
            step_exp = __pa2.clone();
            outStop = __pa3.clone();
            (outStart.clone(), Some(step_exp.clone()), outStop.clone(), DAE::T_REAL_DEFAULT().clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outStart, outStep, outStop, outRangeType))
}

fn elabRangeType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inStart: Arc<DAE::Exp>, mut inStep: Option<Arc<DAE::Exp>>, mut inStop: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inExpType: Arc<DAE::Type>, mut co: DAE::Const, mut inImpl: bool) -> Result<(FCore::Cache, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outCache, outType) = 'mc: {
        let __mc_input = (inStep.clone(), co.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Const::C_VAR) => {
                    Ok((inCache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, _) => {
                    let mut start_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut stop_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dim: i32 = 0;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    (cache, start_val) = Ceval::ceval(inCache.clone(), inEnv.clone(), inStart.clone(), inImpl.clone(), Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    (cache, stop_val) = Ceval::ceval(cache.clone(), inEnv.clone(), inStop.clone(), inImpl.clone(), Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    dim = elabRangeSize(start_val.clone(), None, stop_val.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() })] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(step_exp), _) => {
                    let mut start_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut step_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut stop_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dim: i32 = 0;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    (cache, start_val) = Ceval::ceval(inCache.clone(), inEnv.clone(), inStart.clone(), inImpl.clone(), Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    (cache, step_val) = Ceval::ceval(cache.clone(), inEnv.clone(), step_exp.clone(), inImpl.clone(), Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    (cache, stop_val) = Ceval::ceval(cache.clone(), inEnv.clone(), inStop.clone(), inImpl.clone(), Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    dim = elabRangeSize(start_val.clone(), Some(step_val.clone()), stop_val.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() })] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outType))
}

fn elabRangeSize(mut inStartValue: Arc<Values::Value>, mut inStepValue: Option<Arc<Values::Value>>, mut inStopValue: Arc<Values::Value>) -> Result<i32> {
    let mut outSize: i32 = 0;
    outSize = 'mc: {
        let __mc_input = (inStartValue.clone(), inStepValue.clone(), inStopValue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _) => {
                    let false = (ValuesUtil::safeLessEq(inStartValue.clone(), inStopValue.clone())?) else { bail!("pattern mismatch") };
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: int_start }, None, Deref @ Values::Value::INTEGER { integer: int_stop }) => {
                    let mut dim: i32 = 0;
                    dim = int_stop.clone() - int_start.clone() + 1;
                    Ok(dim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: int_start }, Some(Deref @ Values::Value::INTEGER { integer: int_step }), Deref @ Values::Value::INTEGER { integer: int_stop }) => {
                    let mut dim: i32 = 0;
                    dim = int_stop.clone() - int_start.clone();
                    dim = intDiv(dim.clone(), int_step.clone()) + 1;
                    Ok(dim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: real_start }, None, Deref @ Values::Value::REAL { real: real_stop }) => {
                    Ok(Util::realRangeSize(real_start.clone(), metamodelica::OrderedFloat(1.0_f64), real_stop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: real_start }, Some(Deref @ Values::Value::REAL { real: real_step }), Deref @ Values::Value::REAL { real: real_stop }) => {
                    Ok(Util::realRangeSize(real_start.clone(), real_step.clone(), real_stop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ENUM_LITERAL { index: int_start, .. }, None, Deref @ Values::Value::ENUM_LITERAL { index: int_stop, .. }) => {
                    let mut dim: i32 = 0;
                    dim = int_stop.clone() - int_start.clone() + 1;
                    Ok(dim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: true }, None, Deref @ Values::Value::BOOL { boolean: false }) => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: false }, None, Deref @ Values::Value::BOOL { boolean: true }) => {
                    Ok(2)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: _ }, None, Deref @ Values::Value::BOOL { boolean: _ }) => {
                    Ok(1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSize)
}

fn elabTuple(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut isLhs: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<DAE::Properties>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outProperties: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut exp: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    if if (!(isLhs.clone())) {!(Config::acceptMetaModelicaGrammar()?)} else {false} {
        Error::addSourceMessage(Error::RHS_TUPLE_EXPRESSION.clone(), list![(Dump::printExpStr(Arc::new(Absyn::Exp::TUPLE { expressions: inExpl.clone() }))?).clone()], inInfo.clone())?;
        bail!("fail");
    }
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        (outCache, exp, prop) = elabExp(outCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
        if AbsynUtil::isTuple(e.clone()) {
            (exp, prop) = Types::matchProp(exp.clone(), prop.clone(), DAE::Properties::PROP { type_: DAE::T_METABOXED_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }, true)?;
        }
        outExpl = cons(exp.clone(), outExpl.clone());
        outProperties = cons(prop.clone(), outProperties.clone());
    }
    outExpl = outExpl.clone().reverse();
    outProperties = outProperties.clone().reverse();
    Ok((outCache, outExpl, outProperties))
}

fn stripExtraArgsFromType(mut slots: Arc<metamodelica::List<Slot>>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = inType.clone();
    outType = 'mc: {
        let __mc_input = outType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { .. } => {
                    let mut outType: Arc<DAE::Type> = outType.clone();
                    assign_variant_field!(outType => DAE::Type::T_FUNCTION; funcArg = stripExtraArgsFromType2(slots.clone(), var_field!((*outType).funcArg, DAE::Type::T_FUNCTION).clone(), metamodelica::nil())?);
                    Ok(outType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Static.stripExtraArgsFromType failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn stripExtraArgsFromType2(mut inSlots: Arc<metamodelica::List<Slot>>, mut inType: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut inAccumType: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Arc<metamodelica::List<Arc<DAE::FuncArg>>>> {
    let mut outType: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    outType = (::match_deref::match_deref! { match &((inSlots.clone(), inType.clone())) {
        (Deref @ metamodelica::List::Cons { head: Slot { slotFilled: true, .. }, tail: slotsRest }, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            stripExtraArgsFromType2(slotsRest.clone(), rest.clone(), inAccumType.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Slot { slotFilled: false, .. }, tail: slotsRest }, Deref @ metamodelica::List::Cons { head: arg, tail: rest }) => {
            stripExtraArgsFromType2(slotsRest.clone(), rest.clone(), cons(arg.clone(), inAccumType.clone()))?
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            inAccumType.clone().reverse()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

fn elabArray(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inProps: Arc<metamodelica::List<DAE::Properties>>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, DAE::Properties)> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outProperties: DAE::Properties;
    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    let mut c2: DAE::Const = DAE::Const::C_CONST;
    let mut mixed: bool = false;
    if inExpl.clone().is_empty() {
        Error::addSourceMessage(Error::EMPTY_ARRAY.clone(), metamodelica::nil(), inInfo.clone())?;
        bail!("fail");
    }
    for mut p in &*inProps.clone() {
        let mut p = p.clone();
        let DAE::PROP { constFlag: __pa0, type_: __pa1 } = (p.clone()) else { bail!("pattern mismatch") };
        c2 = __pa0.clone();
        ty = __pa1.clone();
        types = cons(ty.clone(), types.clone());
        c = Types::constAnd(c.clone(), c2.clone());
    }
    types = types.clone().reverse();
    (ty, mixed) = elabArrayHasMixedIntReals(types.clone())?;
    if mixed.clone() {
        outExpLst = elabArrayReal2(inExpl.clone(), types.clone(), ty.clone())?;
    } else {
        (outExpLst, ty) = elabArray2(inExpl.clone(), types.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    Ok((outExpLst, outProperties))
}

fn elabArrayHasMixedIntReals(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(Arc<DAE::Type>, bool)> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outIsMixed: bool = true;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut rest_tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inTypes.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outType = __pa0.clone();
    rest_tys = __pa1.clone();
    if Types::isReal(outType.clone()) {
        while !(rest_tys.clone().is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_tys.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa2.clone();
            rest_tys = __pa3.clone();
            if Types::isInteger(ty.clone()) {
                return Ok((outType.clone(), outIsMixed.clone()));
            }
        }
    } else if Types::isInteger(outType.clone()) {
        while !(rest_tys.clone().is_empty()) {
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_tys.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            outType = __pa4.clone();
            rest_tys = __pa5.clone();
            if Types::isReal(outType.clone()) {
                return Ok((outType.clone(), outIsMixed.clone()));
            }
        }
    }
    outIsMixed = false;
    Ok((outType, outIsMixed))
}

fn elabArrayConst(mut inProperties: Arc<metamodelica::List<DAE::Properties>>) -> Result<DAE::Const> {
    let mut outConst: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    for mut prop in &*inProperties.clone() {
        let mut prop = prop.clone();
        outConst = Types::constAnd(outConst.clone(), Types::getPropConst(prop.clone())?);
    }
    Ok(outConst)
}

fn elabArrayReal2(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inExpectedType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut exp: Arc<DAE::Exp>;
    let mut rest_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = inExpl.clone();
    for mut ty in &*inTypes.clone() {
        let mut ty = ty.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
        rest_expl = __pa1.clone();
        if !(Types::equivtypes(ty.clone(), inExpectedType.clone())?) {
            (exp, _) = Types::matchType(exp.clone(), ty.clone(), inExpectedType.clone(), true)?;
        }
        outExpl = cons(exp.clone(), outExpl.clone());
    }
    outExpl = outExpl.clone().reverse();
    Ok(outExpl)
}

fn elabArray2(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>)> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut rest_tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut exp1: Arc<DAE::Exp>;
    let mut rest_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut exp_str: ArcStr = arcstr::literal!("");
    let mut expl_str: ArcStr = arcstr::literal!("");
    let mut ty1_str: ArcStr = arcstr::literal!("");
    let mut ty2_str: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExpl.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp1 = __pa0.clone();
    rest_expl = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(inTypes.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outType = __pa2.clone();
    rest_tys = __pa3.clone();
    outExpl = list![exp1.clone()];
    outType = Types::getUniontypeIfMetarecordReplaceAllSubtypes(outType.clone())?;
    for mut exp2 in &*rest_expl.clone() {
        let mut exp2 = exp2.clone();
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_tys.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty2 = __pa4.clone();
        rest_tys = __pa5.clone();
        ty2 = Types::getUniontypeIfMetarecordReplaceAllSubtypes(ty2.clone())?;
        if !(Types::equivtypes(outType.clone(), ty2.clone())?) {
            if let Ok((__pa6, __pa7)) = Types::matchType(exp2.clone(), outType.clone(), ty2.clone(), false) {
                exp2 = __pa6.clone();
                outType = __pa7.clone();
            } else {
                ty1_str = (TypesDump::unparseTypeNoAttr(outType.clone())?).clone();
                ty2_str = (TypesDump::unparseTypeNoAttr(ty2.clone())?).clone();
                Types::typeErrorSanityCheck((ty1_str.clone()).clone(), (ty2_str.clone()).clone(), inInfo.clone())?;
                pre_str = (PrefixUtil::printPrefixStr(inPrefix.clone())?).clone();
                exp_str = (ExpressionBasics::printExpStr(exp2.clone())?).clone();
                expl_str = (List::toString(inExpl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("[")).clone(), (literal!(",")).clone(), (literal!("]")).clone(), true, 0)?).clone();
                Error::addSourceMessageAndFail(Error::TYPE_MISMATCH_ARRAY_EXP.clone(), list![(pre_str.clone()).clone(), (exp_str.clone()).clone(), (ty1_str.clone()).clone(), (expl_str.clone()).clone(), (ty2_str.clone()).clone()], inInfo.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
        }
        outExpl = cons(exp2.clone(), outExpl.clone());
    }
    outExpl = outExpl.clone().reverse();
    Ok((outExpl, outType))
}

fn elabGraphicsArray(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outProperties: DAE::Properties;
    let mut c: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    let mut c2: DAE::Const = DAE::Const::C_CONST;
    let mut exp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if inExpl.clone().is_empty() {
        Error::addSourceMessage(Error::EMPTY_ARRAY.clone(), metamodelica::nil(), inInfo.clone())?;
        bail!("fail");
    }
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsExp(outCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?) {
            (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        outCache = __pa0.clone();
        exp = __pa1.clone();
        ty = __pa2.clone();
        c2 = __pa3.clone();
        outExpl = cons(exp.clone(), outExpl.clone());
        c = Types::constAnd(c.clone(), c2.clone());
    }
    outExpl = outExpl.clone().reverse();
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    Ok((outCache, outExpl, outProperties))
}

fn elabMatrixComma(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inProps: Arc<metamodelica::List<DAE::Properties>>, mut inHaveReal: bool, mut inDims: i32, mut inInfo: SourceInfo) -> Result<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Dimension>, Arc<DAE::Dimension>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut outDim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut outDim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut exp: Arc<DAE::Exp>;
    let mut rest_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut accum_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut prop: DAE::Properties;
    let mut rest_props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut sty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inExpl.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        exp = __pa1.clone();
        rest_expl = __pa2.clone();
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inProps.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        prop = __pa3.clone();
        rest_props = __pa4.clone();
        let (__pa5, __pa7, __pa6) = ::match_deref::match_deref! { match &(unwrap_break_err!(promoteExp(exp.clone(), prop.clone(), inDims.clone()), '__try0)) {
            (__pa5, __pa7 @ DAE::Properties::PROP { type_: __pa6, .. }) => (__pa5.clone(), __pa7.clone(), __pa6.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        exp = __pa5.clone();
        ty = __pa6.clone();
        outProperties = __pa7.clone();
        accum_expl = cons(exp.clone(), accum_expl.clone());
        let (__pa8, __pa9) = ::match_deref::match_deref! { match &(TypesDump::getDimensions(ty.clone())) {
            Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: _ } } => (__pa8.clone(), __pa9.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        outDim1 = __pa8.clone();
        outDim2 = __pa9.clone();
        while !(rest_expl.clone().is_empty()) {
            let (__pa11, __pa12) = ::match_deref::match_deref! { match &(rest_expl.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa11, tail: __pa12 } => (__pa11.clone(), __pa12.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            exp = __pa11.clone();
            rest_expl = __pa12.clone();
            let (__pa13, __pa14) = ::match_deref::match_deref! { match &(rest_props.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa13, tail: __pa14 } => (__pa13.clone(), __pa14.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            prop = __pa13.clone();
            rest_props = __pa14.clone();
            let (__pa15, __pa17, __pa16) = ::match_deref::match_deref! { match &(unwrap_break_err!(promoteExp(exp.clone(), prop.clone(), inDims.clone()), '__try0)) {
                (__pa15, __pa17 @ DAE::Properties::PROP { type_: __pa16, .. }) => (__pa15.clone(), __pa17.clone(), __pa16.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            exp = __pa15.clone();
            ty = __pa16.clone();
            prop = __pa17.clone();
            accum_expl = cons(exp.clone(), accum_expl.clone());
            let (__pa18, __pa19) = ::match_deref::match_deref! { match &(TypesDump::getDimensions(ty.clone())) {
                Deref @ metamodelica::List::Cons { head: __pa18, tail: Deref @ metamodelica::List::Cons { head: __pa19, tail: _ } } => (__pa18.clone(), __pa19.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            dim1 = __pa18.clone();
            dim2 = __pa19.clone();
            if !(unwrap_break_err!(Expression::dimensionsEqual(dim1.clone(), outDim1.clone()), '__try0)) {
                unwrap_break_err!(Error::addSourceMessageAndFail(Error::COMMA_OPERATOR_DIFFERENT_SIZES.clone(), list![(ExpressionBasics::printExpStr(listHead(inExpl.clone())?)?).clone(), (ExpressionBasics::dimensionString(outDim1.clone())?).clone(), (ExpressionBasics::printExpStr(exp.clone())?).clone(), (ExpressionBasics::dimensionString(dim1.clone())?).clone()], inInfo.clone()), '__try0);
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            outDim2 = Expression::dimensionsAdd(dim2.clone(), outDim2.clone());
            outProperties = unwrap_break_err!(Types::matchWithPromote(prop.clone(), outProperties.clone(), inHaveReal.clone()), '__try0);
        }
        sty = Expression::liftArrayLeftList(Expression::unliftArrayX(ty.clone(), 2)?, list![outDim1.clone(), outDim2.clone()]);
        outExp = Arc::new(DAE::Exp::ARRAY { ty: sty.clone(), scalar: false, array: accum_expl.clone().reverse() });
        Ok::<_, anyhow::Error>((accum_expl.clone(), exp.clone(), outDim1.clone(), outDim2.clone(), outExp.clone(), outProperties.clone(), prop.clone(), rest_expl.clone(), rest_props.clone(), sty.clone(), ty.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10)) => {
            accum_expl = __try0_o0;
            exp = __try0_o1;
            outDim1 = __try0_o2;
            outDim2 = __try0_o3;
            outExp = __try0_o4;
            outProperties = __try0_o5;
            prop = __try0_o6;
            rest_expl = __try0_o7;
            rest_props = __try0_o8;
            sty = __try0_o9;
            ty = __try0_o10;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- Static.elabMatrixComma failed")).clone())?;
            bail!("fail");
        }
    }
    Ok((outExp, outProperties, outDim1, outDim2))
}

fn elabMatrixCatTwoExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::ARRAY { array: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        expl = __pa1.clone();
        expl = unwrap_break_err!(ExpressionSimplify::simplifyList(expl.clone()), '__try0);
        expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(Expression::matrixToArray(e.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        outExp = unwrap_break_err!(elabMatrixCatTwo(expl.clone()), '__try0);
        Ok::<_, anyhow::Error>((expl.clone(), outExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            expl = __try0_o0;
            outExp = __try0_o1;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- Static.elabMatrixCatTwoExp failed")).clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(outExp)
}

fn elabMatrixCatTwo(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    match '__try0: {
        outExp = todo!("reduction elabMatrixCatTwo2: cannot resolve default value");
        Ok::<_, anyhow::Error>((outExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outExp = __try0_o0;
        }
        Err(_) => {
            ty = Expression::r#typeof(listHead(inExpl.clone())?)?;
            outExp = Expression::makePureBuiltinCall((literal!("cat")).clone(), cons(Arc::new(DAE::Exp::ICONST { integer: 2 }), inExpl.clone()), ty.clone());
        }
    }
    Ok(outExp)
}

fn elabMatrixCatTwo2(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut sc: bool = false;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp1.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expl1 = __pa0.clone();
    sc = __pa1.clone();
    let __pa2 = ::match_deref::match_deref! { match &(inExp2.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __pa2, .. } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    expl2 = __pa2.clone();
    expl1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for (e1, e2) in (&(expl1.clone())).into_iter().zip((&(expl2.clone())).into_iter()) {
            let __x = elabMatrixCatTwo3(e1.clone(), e2.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    ty = Expression::r#typeof(listHead(expl1.clone())?)?;
    ty = Expression::liftArrayLeft(ty.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: (expl1.clone().len() as i32) }));
    outExp = Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: sc.clone(), array: expl1.clone() });
    Ok(outExp)
}

fn elabMatrixCatTwo3(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut sc: bool = false;
    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp1.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __pa0, scalar: __pa1, ty: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expl1 = __pa0.clone();
    sc = __pa1.clone();
    ty1 = __pa2.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inExp2.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __pa3, ty: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expl2 = __pa3.clone();
    ty2 = __pa4.clone();
    expl2 = listAppend(expl1.clone(), expl2.clone());
    ty1 = Expression::concatArrayType(ty1.clone(), ty2.clone())?;
    outExp = Arc::new(DAE::Exp::ARRAY { ty: ty1.clone(), scalar: sc.clone(), array: expl2.clone() });
    Ok(outExp)
}

fn elabMatrixCatOne(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    match '__try0: {
        outExp = unwrap_break_err!(List::reduce(inExpl.clone(), (std::sync::Arc::new(elabMatrixCatOne2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>)), '__try0);
        Ok::<_, anyhow::Error>((outExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outExp = __try0_o0;
        }
        Err(_) => {
            ty = Expression::r#typeof(listHead(inExpl.clone())?)?;
            outExp = Expression::makePureBuiltinCall((literal!("cat")).clone(), cons(Arc::new(DAE::Exp::ICONST { integer: 1 }), inExpl.clone()), ty.clone());
        }
    }
    Ok(outExp)
}

fn elabMatrixCatOne2(mut inArray1: Arc<DAE::Exp>, mut inArray2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut at: bool = false;
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dim_rest: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inArray1.clone()) {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } }, scalar: __pa3, array: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ety = __pa0.clone();
    dim1 = __pa1.clone();
    dim_rest = __pa2.clone();
    at = __pa3.clone();
    expl1 = __pa4.clone();
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(inArray2.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __pa6, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa7, tail: _ }, .. }, .. } => (__pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expl2 = __pa6.clone();
    dim2 = __pa7.clone();
    expl = listAppend(expl1.clone(), expl2.clone());
    dim = Expression::dimensionsAdd(dim1.clone(), dim2.clone());
    outExp = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: cons(dim.clone(), dim_rest.clone()) }), scalar: at.clone(), array: expl.clone() });
    Ok(outExp)
}

fn promoteExp(mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut inDims: i32) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    match '__try0: {
        let DAE::PROP { type_: __pa1, constFlag: __pa2 } = (inProperties.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        ty = __pa1.clone();
        c = __pa2.clone();
        (outExp, ty) = unwrap_break_err!(Expression::promoteExp(inExp.clone(), ty.clone(), inDims.clone()), '__try0);
        outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
        Ok::<_, anyhow::Error>((c.clone(), outExp.clone(), outProperties.clone(), ty.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            c = __try0_o0;
            outExp = __try0_o1;
            outProperties = __try0_o2;
            ty = __try0_o3;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- Static.promoteExp failed")).clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((outExp, outProperties))
}

fn elabMatrixSemi(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inProperties: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Properties>>>>, mut inImpl: bool, mut inHaveReal: bool, mut inDims: i32, mut inDoVectorization: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Dimension>, Arc<DAE::Dimension>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut outDim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut outDim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rest_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut rest_props: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Properties>>>> = metamodelica::nil();
    let mut exp: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut dim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dim1_str: ArcStr = arcstr::literal!("");
    let mut dim2_str: ArcStr = arcstr::literal!("");
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut el_str: ArcStr = arcstr::literal!("");
    let mut ty1_str: ArcStr = arcstr::literal!("");
    let mut ty2_str: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inMatrix.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expl = __pa0.clone();
    rest_expl = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(inProperties.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    props = __pa2.clone();
    rest_props = __pa3.clone();
    (outExp, outProperties, outDim1, outDim2) = elabMatrixComma(expl.clone(), props.clone(), inHaveReal.clone(), inDims.clone(), inInfo.clone())?;
    outExp = elabMatrixCatTwoExp(outExp.clone())?;
    while !(rest_expl.clone().is_empty()) {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_expl.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        expl = __pa4.clone();
        rest_expl = __pa5.clone();
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(rest_props.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => bail!("pattern mismatch"),
        } };
        props = __pa6.clone();
        rest_props = __pa7.clone();
        (exp, prop, dim1, dim2) = elabMatrixComma(expl.clone(), props.clone(), inHaveReal.clone(), inDims.clone(), inInfo.clone())?;
        if !(Expression::dimensionsEqual(dim2.clone(), outDim2.clone())?) {
            dim1_str = (ExpressionBasics::dimensionString(dim1.clone())?).clone();
            dim2_str = (ExpressionBasics::dimensionString(dim2.clone())?).clone();
            pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
            el_str = (List::toString(expl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?).clone();
            Error::addSourceMessageAndFail(Error::MATRIX_EXP_ROW_SIZE.clone(), list![(pre_str.clone()).clone(), (el_str.clone()).clone(), (dim1_str.clone()).clone(), (dim2_str.clone()).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        if let Ok(__iflet8) = Types::matchWithPromote(outProperties.clone(), prop.clone(), inHaveReal.clone()) {
            outProperties = __iflet8;
        } else {
            ty1_str = (TypesDump::unparsePropTypeNoAttr(outProperties.clone())?).clone();
            ty2_str = (TypesDump::unparsePropTypeNoAttr(prop.clone())?).clone();
            Types::typeErrorSanityCheck((ty1_str.clone()).clone(), (ty2_str.clone()).clone(), inInfo.clone())?;
            pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
            el_str = (List::toString(expl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?).clone();
            Error::addSourceMessageAndFail(Error::TYPE_MISMATCH_MATRIX_EXP.clone(), list![(pre_str.clone()).clone(), (el_str.clone()).clone(), (ty1_str.clone()).clone(), (ty2_str.clone()).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        exp = elabMatrixCatTwoExp(exp.clone())?;
        outExp = elabMatrixCatOne(list![outExp.clone(), exp.clone()])?;
        outDim1 = Expression::dimensionsAdd(dim1.clone(), outDim1.clone());
    }
    Ok((outCache, outExp, outProperties, outDim1, outDim2))
}

fn verifyBuiltInHandlerType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inImplicit: bool, mut inTypeChecker: Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>, mut inFnName: ArcStr, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    pub type extraFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>;

    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(inExpl.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    (outCache, _, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    ty = Types::getPropType(outProperties.clone())?;
    ty = Types::arrayElementType(ty.clone());
    let true = (inTypeChecker(ty.clone())?) else { bail!("pattern mismatch") };
    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(elabCallArgs(outCache.clone(), inEnv.clone(), Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (inFnName.clone()).clone() }) }), list![e.clone()], metamodelica::nil(), metamodelica::nil(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?) {
        (__pa2, __pa3, __pa4 @ DAE::Properties::PROP { .. }) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa2.clone();
    outExp = __pa3.clone();
    outProperties = __pa4.clone();
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinCardinality(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("cardinality")).clone(), inInfo.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(inPosArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    let DAE::PROP { type_: __pa2, .. } = (outProperties.clone()) else { bail!("pattern mismatch") };
    ty = __pa2.clone();
    ty = Types::liftArrayListDims(DAE::T_INTEGER_DEFAULT().clone(), TypesDump::getDimensions(ty.clone()));
    outExp = Expression::makePureBuiltinCall((literal!("cardinality")).clone(), list![outExp.clone()], ty.clone());
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSmooth(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut msg_str: ArcStr = arcstr::literal!("");
    let mut p: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut expr: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut dp: Arc<DAE::Exp>;
    let mut dexpr: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    if (inPosArgs.clone().len() as i32) != 2 || !(inNamedArgs.clone().is_empty()) {
        msg_str = (literal!(", expected smooth(p, expr)")).clone();
        printBuiltinFnArgError((literal!("smooth")).clone(), (msg_str.clone()).clone(), inPosArgs.clone(), inNamedArgs.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inPosArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    p = __pa0.clone();
    expr = __pa1.clone();
    let (__pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), p.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?) {
        (__pa3, __pa4, DAE::Properties::PROP { type_: __pa5, constFlag: __pa6 }) => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa3.clone();
    dp = __pa4.clone();
    ty = __pa5.clone();
    c = __pa6.clone();
    if !(Types::isParameterOrConstant(c.clone())) || !(Types::isInteger(ty.clone())) {
        msg_str = (literal!(", first argument must be a constant or parameter expression of type Integer")).clone();
        printBuiltinFnArgError((literal!("smooth")).clone(), (msg_str.clone()).clone(), inPosArgs.clone(), inNamedArgs.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    let (__pa7, __pa8, __pa11, __pa9, __pa10) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache.clone(), inEnv.clone(), expr.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?) {
        (__pa7, __pa8, __pa11 @ DAE::Properties::PROP { type_: __pa9, constFlag: __pa10 }) => (__pa7.clone(), __pa8.clone(), __pa11.clone(), __pa9.clone(), __pa10.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa7.clone();
    dexpr = __pa8.clone();
    ty = __pa9.clone();
    c = __pa10.clone();
    outProperties = __pa11.clone();
    if !(Types::isReal(ty.clone()) || Types::isRecordWithOnlyReals(ty.clone())) {
        msg_str = (literal!(", second argument must be a Real, array of Reals or record only containing Reals")).clone();
        printBuiltinFnArgError((literal!("smooth")).clone(), (msg_str.clone()).clone(), inPosArgs.clone(), inNamedArgs.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    ty = Types::simplifyType(ty.clone())?;
    outExp = Expression::makePureBuiltinCall((literal!("smooth")).clone(), list![dp.clone(), dexpr.clone()], ty.clone());
    Ok((outCache, outExp, outProperties))
}

fn printBuiltinFnArgError(mut inFnName: ArcStr, mut inMsg: ArcStr, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<()> {
    let mut args_str: ArcStr = arcstr::literal!("");
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut msg_str: ArcStr = arcstr::literal!("");
    let mut pos_args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    pos_args = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (inPosArgs.clone()).into_iter().cloned() {
            let __x = Dump::printExpStr(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    named_args = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (inNamedArgs.clone()).into_iter().cloned() {
            let __x = Dump::printNamedArgStr(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    args_str = stringDelimitList(listAppend(pos_args.clone(), named_args.clone()), (literal!(", ")).clone());
    pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
    msg_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inFnName.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*args_str.clone()); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*inMsg.clone()); ArcStr::from(__mm_s) }).clone();
    Error::addSourceMessageAndFail(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(msg_str.clone()).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    Ok(())
}

fn elabBuiltinSize(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: arraycr, tail: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, r#impl, pre) => {
            let mut dimp: Arc<DAE::Exp>;
            let mut arraycrefe: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut prop: DAE::Properties;
            let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut dims1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, dimp, _) = elabExpInExpression(cache.clone(), env.clone(), dim.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (cache, arraycrefe, prop) = elabExpInExpression(cache.clone(), env.clone(), arraycr.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
            ety = Expression::r#typeof(arraycrefe.clone())?;
            dims1 = Expression::arrayDimension(ety.clone());
            (_, dims2) = TypesDump::flattenArrayType(Types::getPropType(prop.clone())?);
            dims = if ((dims1.clone().len() as i32) >= (dims2.clone().len() as i32)) {dims1.clone()} else {dims2.clone()};
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elabBuiltinSizeIndex(arraycrefe.clone(), prop.clone(), ety.clone(), dimp.clone(), dims.clone(), env.clone(), info.clone())?) {
                (Some(__pa0), Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            exp = __pa0.clone();
            prop = __pa1.clone();
            (cache.clone(), exp.clone(), prop.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: arraycr, tail: Deref @ metamodelica::List::Nil }, r#impl, pre) => {
            let mut arraycrefe: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut arrtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop: DAE::Properties;
            let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arraycr.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: _ }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            arraycrefe = __pa1.clone();
            arrtp = __pa2.clone();
            ety = Expression::r#typeof(arraycrefe.clone())?;
            dims = Expression::arrayDimension(ety.clone());
            (exp, prop) = elabBuiltinSizeNoIndex(arraycrefe.clone(), ety.clone(), dims.clone(), arrtp.clone(), info.clone())?;
            (cache.clone(), exp.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSizeNoIndex(mut inArrayExp: Arc<DAE::Exp>, mut inArrayExpType: Arc<DAE::Type>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inArrayType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outSizeExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outSizeExp, outProperties) = 'mc: {
        let __mc_input = inDimensions.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut exp_str: ArcStr = arcstr::literal!("");
                    let mut size_str: ArcStr = arcstr::literal!("");
                    let false = (Types::isUnknownType(inArrayExpType.clone())) else { bail!("pattern mismatch") };
                    exp_str = (ExpressionBasics::printExpStr(inArrayExp.clone())?).clone();
                    size_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*exp_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone(), list![(size_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
                    let mut dim_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dim_int: i32 = 0;
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    dim_expl = List::map(inDimensions.clone(), (std::sync::Arc::new(Expression::dimensionSizeExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<Arc<DAE::Exp>> + 'static>));
                    dim_int = (dim_expl.clone().len() as i32);
                    ty = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() })] });
                    exp = Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: dim_expl.clone() });
                    prop = DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
                    Ok((exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut b: bool = false;
                    let mut cnst: DAE::Const = DAE::Const::C_CONST;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    b = Types::dimensionsKnown(inArrayType.clone())?;
                    cnst = Types::boolConstSize(b.clone());
                    exp = Arc::new(DAE::Exp::SIZE { exp: inArrayExp.clone(), sz: None });
                    ty = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] });
                    prop = DAE::Properties::PROP { type_: ty.clone(), constFlag: cnst.clone() };
                    Ok((exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSizeExp, outProperties))
}

fn elabBuiltinSizeIndex(mut inArrayExp: Arc<DAE::Exp>, mut inArrayProp: DAE::Properties, mut inArrayType: Arc<DAE::Type>, mut inIndexExp: Arc<DAE::Exp>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inEnv: FCore::Graph, mut inInfo: SourceInfo) -> Result<(Option<Arc<DAE::Exp>>, Option<DAE::Properties>)> {
    let mut outSizeExp: Option<Arc<DAE::Exp>> = None;
    let mut outProperties: Option<DAE::Properties> = None;
    (outSizeExp, outProperties) = 'mc: {
        let __mc_input = inDimensions.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut exp_str: ArcStr = arcstr::literal!("");
                    let mut index_str: ArcStr = arcstr::literal!("");
                    let mut size_str: ArcStr = arcstr::literal!("");
                    let false = (Types::isUnknownType(inArrayType.clone())) else { bail!("pattern mismatch") };
                    exp_str = (ExpressionBasics::printExpStr(inArrayExp.clone())?).clone();
                    index_str = (ExpressionBasics::printExpStr(inIndexExp.clone())?).clone();
                    size_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*exp_str.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*index_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone(), list![(size_str.clone()).clone()], inInfo.clone())?;
                    Ok((None, None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dim_int: i32 = 0;
                    let mut dim_count: i32 = 0;
                    let mut exp: Arc<DAE::Exp>;
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut prop: DAE::Properties;
                    dim_int = Expression::expInt(inIndexExp.clone())?;
                    dim_count = (inDimensions.clone().len() as i32);
                    let true = (dim_int.clone() > 0 && dim_int.clone() <= dim_count.clone()) else { bail!("pattern mismatch") };
                    dim = (inDimensions.clone()).get(dim_int.clone())?;
                    exp = Expression::dimensionSizeConstantExp(dim.clone())?;
                    prop = DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
                    Ok((Some(exp.clone()), Some(prop.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dim_int: i32 = 0;
                    let mut dim_count: i32 = 0;
                    let mut exp_str: ArcStr = arcstr::literal!("");
                    let mut index_str: ArcStr = arcstr::literal!("");
                    let mut dim_str: ArcStr = arcstr::literal!("");
                    let false = (Types::isUnknownType(inArrayType.clone())) else { bail!("pattern mismatch") };
                    dim_int = Expression::expInt(inIndexExp.clone())?;
                    dim_count = (inDimensions.clone().len() as i32);
                    let true = (dim_int.clone() <= 0 || dim_int.clone() > dim_count.clone()) else { bail!("pattern mismatch") };
                    index_str = (intString(dim_int.clone())).clone();
                    exp_str = (ExpressionBasics::printExpStr(inArrayExp.clone())?).clone();
                    dim_str = (intString(dim_count.clone())).clone();
                    Error::addSourceMessage(Error::INVALID_SIZE_INDEX.clone(), list![(index_str.clone()).clone(), (exp_str.clone()).clone(), (dim_str.clone()).clone()], inInfo.clone())?;
                    Ok((None, None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cnst: DAE::Const = DAE::Const::C_CONST;
                    exp = Arc::new(DAE::Exp::SIZE { exp: inArrayExp.clone(), sz: Some(inIndexExp.clone()) });
                    cnst = openmodelica_frontend_types::DAE::Const::C_PARAM;
                    cnst = if (FGraph::inFunctionScope(inEnv.clone())?) {openmodelica_frontend_types::DAE::Const::C_VAR} else {cnst.clone()};
                    prop = DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: cnst.clone() };
                    Ok((Some(exp.clone()), Some(prop.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSizeExp, outProperties))
}

fn elabBuiltinNDims(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inBoolean.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: arraycr, tail: Deref @ metamodelica::List::Nil }, r#impl, pre) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut arrtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut nd: i32 = 0;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arraycr.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, _, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    arrtp = __pa1.clone();
                    nd = Types::numberOfDimensions(arrtp.clone())?;
                    exp = Arc::new(DAE::Exp::ICONST { integer: nd.clone() });
                    Ok((cache.clone(), exp.clone(), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, expl, _, pre) => {
                    let mut sp: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    sp = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabBuiltinNdims failed for: ndims(")); __mm_s.push_str(&*Dump::printExpLstStr(expl.clone())); __mm_s.push_str(&*literal!(" in component: ")); __mm_s.push_str(&*sp.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinFill(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inBoolean.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: s, tail: dims }, r#impl, pre) => {
                    let mut s_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut dims_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dimprops: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut sty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dimvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut cache = (*cache).clone();
                    (cache, s_1, prop) = elabExpInExpression(cache.clone(), env.clone(), s.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, dims_1, dimprops) = elabExpList(cache.clone(), env.clone(), dims.clone(), r#impl.clone(), true, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    (dims_1, _) = Types::matchTypes(dims_1.clone(), List::map(dimprops.clone(), (std::sync::Arc::new(Types::getPropType) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Properties) -> Result<Arc<DAE::Type>> + 'static>)), DAE::T_INTEGER_DEFAULT().clone(), false)?;
                    c1 = Types::propertiesListToConst(dimprops.clone())?;
                    if '__try0: {
                        let DAE::C_VAR { .. } = (c1.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    c1 = Types::constAnd(c1.clone(), Types::propAllConst(prop.clone())?);
                    sty = Types::getPropType(prop.clone())?;
                    (cache, dimvals) = Ceval::cevalList(cache.clone(), env.clone(), dims_1.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    (cache, exp, prop) = elabBuiltinFill2(cache.clone(), env.clone(), s_1.clone(), sty.clone(), dimvals.clone(), c1.clone(), pre.clone(), dims.clone(), info.clone())?;
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: s, tail: dims }, r#impl, pre) => {
                    let mut s_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut dims_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dimprops: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut sty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut exp_type: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    c1 = unevaluatedFunctionVariability(env.clone())?;
                    (cache, s_1, prop) = elabExpInExpression(cache.clone(), env.clone(), s.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, dims_1, dimprops) = elabExpList(cache.clone(), env.clone(), dims.clone(), r#impl.clone(), true, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    (dims_1, _) = Types::matchTypes(dims_1.clone(), List::map(dimprops.clone(), (std::sync::Arc::new(Types::getPropType) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Properties) -> Result<Arc<DAE::Type>> + 'static>)), DAE::T_INTEGER_DEFAULT().clone(), false)?;
                    sty = Types::getPropType(prop.clone())?;
                    sty = Types::liftTypeWithDimExps(sty.clone(), dims_1.clone())?;
                    exp_type = Types::simplifyType(sty.clone())?;
                    prop = DAE::Properties::PROP { type_: sty.clone(), constFlag: c1.clone() };
                    exp = Expression::makePureBuiltinCall((literal!("fill")).clone(), cons(s_1.clone(), dims_1.clone()), exp_type.clone());
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: s, tail: dims }, r#impl, pre) => {
                    let mut s_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut dims_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut sty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut exp_type: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let false = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), s.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    s_1 = __pa1.clone();
                    sty = __pa2.clone();
                    c1 = __pa3.clone();
                    (cache, dims_1, _) = elabExpList(cache.clone(), env.clone(), dims.clone(), r#impl.clone(), true, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    sty = Types::liftTypeWithDimExps(sty.clone(), dims_1.clone())?;
                    exp_type = Types::simplifyType(sty.clone())?;
                    c1 = Types::constAnd(c1.clone(), openmodelica_frontend_types::DAE::Const::C_PARAM);
                    prop = DAE::Properties::PROP { type_: sty.clone(), constFlag: c1.clone() };
                    exp = Expression::makePureBuiltinCall((literal!("fill")).clone(), cons(s_1.clone(), dims_1.clone()), exp_type.clone());
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, dims, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Static.elabBuiltinFill failed in component")); __mm_s.push_str(&*PrefixUtil::printPrefixStr3(inPrefix.clone())?); __mm_s.push_str(&*literal!(" and scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); __mm_s.push_str(&*literal!(" for expression: fill(")); __mm_s.push_str(&*Dump::printExpLstStr(dims.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, dims, r#impl, pre) => {
                    let mut implstr: ArcStr = arcstr::literal!("");
                    let mut expstr: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sp: ArcStr = arcstr::literal!("");
                    let mut expstrs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Static.elabBuiltinFill: Couldn't elaborate fill(): ")).clone())?;
                    implstr = (boolString(r#impl.clone())).clone();
                    expstrs = List::map(dims.clone(), (std::sync::Arc::new(Dump::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<ArcStr> + 'static>));
                    expstr = stringDelimitList(expstrs.clone(), (literal!(", ")).clone());
                    sp = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    r#str = stringAppendList(list![(expstr.clone()).clone(), (literal!(" impl=")).clone(), (implstr.clone()).clone(), (literal!(", in component: ")).clone(), (sp.clone()).clone()]);
                    Debug::traceln((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

pub fn elabBuiltinFill2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut constVar: DAE::Const, mut inPrefix: DAE::Prefix, mut inDims: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inType.clone(), inValuesValueLst.clone(), constVar.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, s, sty, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v }, tail: Deref @ metamodelica::List::Nil }, c1, _) => {
                    let mut arraylist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut is_scalar: bool = false;
                    let mut sty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut v = (*v).clone();
                    let true = (intLt(v.clone(), 0)) else { bail!("pattern mismatch") };
                    v = 0;
                    arraylist = List::fill(s.clone(), v.clone());
                    sty2 = Arc::new(DAE::Type::T_ARRAY { ty: sty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: v.clone() })] });
                    at = Types::simplifyType(sty2.clone())?;
                    is_scalar = !(Types::isArray(sty.clone()));
                    Ok((cache.clone(), Arc::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: is_scalar.clone(), array: arraylist.clone() }), DAE::Properties::PROP { type_: sty2.clone(), constFlag: c1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, s, sty, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v }, tail: Deref @ metamodelica::List::Nil }, c1, _) => {
                    let mut arraylist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut is_scalar: bool = false;
                    let mut sty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    arraylist = List::fill(s.clone(), v.clone());
                    sty2 = Arc::new(DAE::Type::T_ARRAY { ty: sty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: v.clone() })] });
                    at = Types::simplifyType(sty2.clone())?;
                    is_scalar = !(Types::isArray(sty.clone()));
                    Ok((cache.clone(), Arc::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: is_scalar.clone(), array: arraylist.clone() }), DAE::Properties::PROP { type_: sty2.clone(), constFlag: c1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, s, sty, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v }, tail: rest }, c1, pre) => {
                    let mut arraylist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabBuiltinFill2(cache.clone(), env.clone(), s.clone(), sty.clone(), rest.clone(), c1.clone(), pre.clone(), inDims.clone(), inInfo.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: _ }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    exp = __pa1.clone();
                    ty = __pa2.clone();
                    arraylist = List::fill(exp.clone(), v.clone());
                    sty2 = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: v.clone() })] });
                    at = Types::simplifyType(sty2.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: false, array: arraylist.clone() }), DAE::Properties::PROP { type_: sty2.clone(), constFlag: c1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Static.elabBuiltinFill2 failed in component")); __mm_s.push_str(&*PrefixUtil::printPrefixStr3(inPrefix.clone())?); __mm_s.push_str(&*literal!(" and scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv.clone())?); __mm_s.push_str(&*literal!(" for expression: fill(")); __mm_s.push_str(&*Dump::printExpLstStr(inDims.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSymmetric(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: matexp, tail: Deref @ metamodelica::List::Nil }, r#impl, pre) => {
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut d1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut d2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut eltp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut newtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop: DAE::Properties;
            let mut c: DAE::Const = DAE::Const::C_CONST;
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), matexp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: __pa2, dims: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } }, dims: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } }, constFlag: __pa5 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            exp_1 = __pa1.clone();
            eltp = __pa2.clone();
            d2 = __pa3.clone();
            d1 = __pa4.clone();
            c = __pa5.clone();
            newtp = Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: eltp.clone(), dims: list![d1.clone()] }), dims: list![d2.clone()] });
            tp = Types::simplifyType(newtp.clone())?;
            exp = Expression::makePureBuiltinCall((literal!("symmetric")).clone(), list![exp_1.clone()], tp.clone());
            prop = DAE::Properties::PROP { type_: newtp.clone(), constFlag: c.clone() };
            (cache.clone(), exp.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinClassDirectory(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (match info.clone() {
        SourceInfo { fileName: mut fileName, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (stringAppend((System::dirname((fileName.clone()).clone())).clone(), (literal!("/")).clone())).clone();
            Error::addSourceMessage(Error::NON_STANDARD_OPERATOR_CLASS_DIRECTORY.clone(), metamodelica::nil(), info.clone())?;
            (inCache.clone(), Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() }), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSourceInfo(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    ::match_deref::match_deref! { match &(inAbsynExpLst.clone()) {
        Deref @ metamodelica::List::Nil => (),
        _ => bail!("pattern mismatch"),
    } };
    (outCache, outExp, outProperties) = (match info.clone() {
        SourceInfo { .. } => {
            let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            args = list![Arc::new(DAE::Exp::SCONST { string: info.fileName.clone() }), Arc::new(DAE::Exp::BCONST { bool: info.isReadOnly.clone() }), Arc::new(DAE::Exp::ICONST { integer: info.lineNumberStart.clone() }), Arc::new(DAE::Exp::ICONST { integer: info.columnNumberStart.clone() }), Arc::new(DAE::Exp::ICONST { integer: info.lineNumberEnd.clone() }), Arc::new(DAE::Exp::ICONST { integer: info.columnNumberEnd.clone() }), Arc::new(DAE::Exp::RCONST { real: info.lastModification.clone() })];
            outExp = Arc::new(DAE::Exp::METARECORDCALL { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("SourceInfo")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SOURCEINFO")).clone() }) }), args: args.clone(), fieldNames: list![(literal!("fileName")).clone(), (literal!("isReadOnly")).clone(), (literal!("lineNumberStart")).clone(), (literal!("columnNumberStart")).clone(), (literal!("lineNumberEnd")).clone(), (literal!("columnNumberEnd")).clone(), (literal!("lastEditTime")).clone()], index: 0, typeVars: metamodelica::nil() });
            (inCache.clone(), outExp.clone(), DAE::Properties::PROP { type_: DAE::T_SOURCEINFO_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSome(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut arg: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    if (inPosArgs.clone().len() as i32) != 1 || !(inNamedArgs.clone().is_empty()) {
        Error::addSourceMessageAndFail(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(literal!("SOME")).clone(), (literal!("")).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    } else {
        (outCache, arg, prop) = elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
        ty = Types::getPropType(prop.clone())?;
        (arg, ty) = Types::matchType(arg.clone(), ty.clone(), DAE::T_METABOXED_DEFAULT().clone(), true)?;
        c = Types::propAllConst(prop.clone())?;
        outExp = Arc::new(DAE::Exp::META_OPTION { exp: Some(arg.clone()) });
        outProperties = DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_METAOPTION { ty: ty.clone() }), constFlag: c.clone() };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinNone(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    if !(inPosArgs.clone().is_empty()) || !(inNamedArgs.clone().is_empty()) {
        Error::addSourceMessageAndFail(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(literal!("NONE")).clone(), (literal!("")).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    } else {
        outExp = Arc::new(DAE::Exp::META_OPTION { exp: None });
        outProperties = DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_METAOPTION { ty: DAE::T_UNKNOWN_DEFAULT().clone() }), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinHomotopy(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut replaceWith: ArcStr = arcstr::literal!("");
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    replaceWith = (Flags::getConfigString(Flags::REPLACE_HOMOTOPY.clone())?).clone();
    if replaceWith.clone() == literal!("actual") || replaceWith.clone() == literal!("simplified") {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getHomotopyArguments(inPosArgs.clone(), inNamedArgs.clone())?) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e1 = __pa0.clone();
        e2 = __pa1.clone();
        e = if (replaceWith.clone() == literal!("actual")) {e1.clone()} else {e2.clone()};
        (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    } else {
        (outCache, outExp, outProperties) = elabCallArgs(inCache.clone(), inEnv.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("homotopy")).clone() }), inPosArgs.clone(), inNamedArgs.clone(), metamodelica::nil(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    Ok((outCache, outExp, outProperties))
}

fn getHomotopyArguments(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut outPositionalArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    outPositionalArgs = (::match_deref::match_deref! { match &((args.clone(), nargs.clone())) {
        (Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, _) => {
            list![e1.clone(), e2.clone()]
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "actual", argValue: e1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "simplified", argValue: e2 }, tail: Deref @ metamodelica::List::Nil } }) => {
            list![e1.clone(), e2.clone()]
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "simplified", argValue: e2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "actual", argValue: e1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            list![e1.clone(), e2.clone()]
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "simplified", argValue: e2 }, tail: Deref @ metamodelica::List::Nil }) => {
            list![e1.clone(), e2.clone()]
        },
        _ => {
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("+replaceHomotopy: homotopy called with wrong arguments: ")); __mm_s.push_str(&*Dump::printFunctionArgsStr(Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: args.clone(), argNames: nargs.clone() }))?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPositionalArgs)
}

fn elabBuiltinDynamicSelect(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut msg_str: ArcStr = arcstr::literal!("");
    let mut astatic: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut adynamic: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut dstatic: Arc<DAE::Exp>;
    let mut ddynamic: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if (inPosArgs.clone().len() as i32) != 2 || !(inNamedArgs.clone().is_empty()) {
        msg_str = (literal!(", expected DynamicSelect(staticExp, dynamicExp)")).clone();
        printBuiltinFnArgError((literal!("DynamicSelect")).clone(), (msg_str.clone()).clone(), inPosArgs.clone(), inNamedArgs.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inPosArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    astatic = __pa0.clone();
    adynamic = __pa1.clone();
    let (__pa3, __pa4, __pa6, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), astatic.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?) {
        (__pa3, __pa4, __pa6 @ DAE::Properties::PROP { type_: __pa5, constFlag: _ }) => (__pa3.clone(), __pa4.clone(), __pa6.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa3.clone();
    dstatic = __pa4.clone();
    ty = __pa5.clone();
    outProperties = __pa6.clone();
    match '__try7: {
        (outCache, ddynamic, _) = unwrap_break_err!(elabExpInExpression(outCache.clone(), inEnv.clone(), adynamic.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone()), '__try7);
        outExp = Expression::makePureBuiltinCall((literal!("DynamicSelect")).clone(), list![dstatic.clone(), ddynamic.clone()], ty.clone());
        Ok::<_, anyhow::Error>((outExp.clone(),))
    } {
        Ok((__try7_o0,)) => {
            outExp = __try7_o0;
        }
        Err(_) => {
            outExp = dstatic.clone();
        }
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinTranspose(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImpl: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut exp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut el_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut d1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut d2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let __pa0 = ::match_deref::match_deref! { match &(inPosArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    aexp = __pa0.clone();
    let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), aexp.clone(), inImpl.clone(), true, inPrefix.clone(), inInfo.clone())?) {
        (__pa2, __pa3, DAE::Properties::PROP { type_: __pa4, constFlag: __pa5 }) => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa2.clone();
    exp = __pa3.clone();
    ty = __pa4.clone();
    c = __pa5.clone();
    let (__pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: __pa6, dims: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } }, dims: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Nil } } => (__pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    el_ty = __pa6.clone();
    d1 = __pa7.clone();
    d2 = __pa8.clone();
    ty = Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: el_ty.clone(), dims: list![d2.clone()] }), dims: list![d1.clone()] });
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    ty = Types::simplifyType(ty.clone())?;
    outExp = Expression::makePureBuiltinCall((literal!("transpose")).clone(), list![exp.clone()], ty.clone());
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSum(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: arrexp, tail: Deref @ metamodelica::List::Nil }, r#impl, pre) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut c: DAE::Const = DAE::Const::C_CONST;
            let mut b: bool = false;
            let mut estr: ArcStr = arcstr::literal!("");
            let mut tstr: ArcStr = arcstr::literal!("");
            let mut etp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arrexp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            exp_1 = __pa1.clone();
            t = __pa2.clone();
            c = __pa3.clone();
            tp = Types::arrayElementType(t.clone());
            etp = Types::simplifyType(tp.clone())?;
            b = Types::isArray(t.clone());
            b = b.clone() && Types::isSimpleType(tp.clone());
            estr = (Dump::printExpStr(arrexp.clone())?).clone();
            tstr = (TypesDump::unparseType(t.clone())?).clone();
            Error::assertionOrAddSourceMessage(b.clone(), Error::SUM_EXPECTED_ARRAY.clone(), list![(estr.clone()).clone(), (tstr.clone()).clone()], info.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sum")).clone(), list![exp_1.clone()], etp.clone());
            (cache.clone(), exp_2.clone(), DAE::Properties::PROP { type_: tp.clone(), constFlag: c.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinProduct(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inBoolean.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: arrexp, tail: Deref @ metamodelica::List::Nil }, r#impl, pre) => {
                    let mut exp_1: Arc<DAE::Exp>;
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut str_exp: ArcStr = arcstr::literal!("");
                    let mut str_pre: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arrexp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    exp_1 = __pa1.clone();
                    ty = __pa2.clone();
                    c = __pa3.clone();
                    (exp_1, _) = Types::matchType(exp_1.clone(), ty.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
                    str_exp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("product(")); __mm_s.push_str(&*Dump::printExpStr(arrexp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    str_pre = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::BUILTIN_FUNCTION_PRODUCT_HAS_SCALAR_PARAMETER.clone(), list![(str_exp.clone()).clone(), (str_pre.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), exp_1.clone(), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: c.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: arrexp, tail: Deref @ metamodelica::List::Nil }, r#impl, pre) => {
                    let mut exp_1: Arc<DAE::Exp>;
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut str_exp: ArcStr = arcstr::literal!("");
                    let mut str_pre: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arrexp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    exp_1 = __pa1.clone();
                    ty = __pa2.clone();
                    c = __pa3.clone();
                    (exp_1, _) = Types::matchType(exp_1.clone(), ty.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    str_exp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("product(")); __mm_s.push_str(&*Dump::printExpStr(arrexp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    str_pre = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::BUILTIN_FUNCTION_PRODUCT_HAS_SCALAR_PARAMETER.clone(), list![(str_exp.clone()).clone(), (str_pre.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), exp_1.clone(), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: c.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: arrexp, tail: Deref @ metamodelica::List::Nil }, r#impl, pre) => {
                    let mut exp_1: Arc<DAE::Exp>;
                    let mut exp_2: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut etp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa3, __pa2, __pa4) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arrexp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa3 @ Deref @ DAE::Type::T_ARRAY { ty: __pa2, dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, constFlag: __pa4 }) => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    exp_1 = __pa1.clone();
                    tp = __pa2.clone();
                    t = __pa3.clone();
                    c = __pa4.clone();
                    tp = Types::arrayElementType(t.clone());
                    etp = Types::simplifyType(tp.clone())?;
                    exp_2 = Expression::makePureBuiltinCall((literal!("product")).clone(), list![exp_1.clone()], etp.clone());
                    if !(Types::arrayHasUnknownDims(t.clone())) {
                        exp_2 = elabBuiltinProduct2(exp_2.clone())?;
                    }
                    Ok((cache.clone(), exp_2.clone(), DAE::Properties::PROP { type_: tp.clone(), constFlag: c.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinProduct2(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: array_exp, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    Ok(Expression::makeProductLst(Expression::arrayElements(array_exp.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn elabBuiltinPre(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut exp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut sc: bool = false;
    let mut exp_str: ArcStr = arcstr::literal!("");
    let mut pre_str: ArcStr = arcstr::literal!("");
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("pre")).clone(), inInfo.clone())?;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?) {
        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    exp = __pa1.clone();
    ty = __pa2.clone();
    c = __pa3.clone();
    if Expression::isMatrix(exp.clone()) {
        let __pa4 = ::match_deref::match_deref! { match &(ty.clone()) {
            Deref @ DAE::Type::T_ARRAY { ty: __pa4, .. } => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        ty2 = __pa4.clone();
        ty2 = Types::unliftArray(ty2.clone())?;
        outExp = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![exp.clone()], Types::simplifyType(ty2.clone())?);
        outExp = elabBuiltinPreMatrix(outExp.clone(), ty2.clone())?;
    } else if Types::isArray(ty.clone()) {
        ty2 = Types::unliftArray(ty.clone())?;
        outExp = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![exp.clone()], Types::simplifyType(ty2.clone())?);
        (expl, sc) = elabBuiltinPre2(outExp.clone(), ty2.clone())?;
        outExp = Arc::new(DAE::Exp::ARRAY { ty: Types::simplifyType(ty.clone())?, scalar: sc.clone(), array: expl.clone() });
    } else {
        ty = Types::arrayElementType(ty.clone());
        if Types::basicType(ty.clone()) {
            outExp = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![exp.clone()], Types::simplifyType(ty.clone())?);
        } else {
            exp_str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
            Error::addSourceMessageAndFail(Error::OPERAND_BUILTIN_TYPE.clone(), list![(literal!("pre")).clone(), (pre_str.clone()).clone(), (exp_str.clone()).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinPre2(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, bool)> {
    let mut outExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outScalar: bool = false;
    (outExp, outScalar) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl, scalar: sc, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    Ok((makePreLst(expl.clone(), inType.clone())?, sc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::MATRIX { matrix: mexpl, integer: i, ty }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut mexpl = (*mexpl).clone();
                    mexpl = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut e in (mexpl.clone()).into_iter().cloned() {
                    let __x = makePreLst(e.clone(), inType.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    Ok((list![Arc::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: i.clone(), matrix: mexpl.clone() })], false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((list![inExp.clone()], false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outScalar))
}

fn elabBuiltinInStream(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImpl: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut exp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(inArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    (outCache, exp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImpl.clone(), true, inPrefix.clone(), inInfo.clone())?;
    ty = Types::getPropType(outProperties.clone())?;
    outExp = elabBuiltinStreamOperator(outCache.clone(), inEnv.clone(), (literal!("inStream")).clone(), exp.clone(), ty.clone(), inInfo.clone())?;
    if Types::dimensionsKnown(ty.clone())? {
        (outCache, outExp, outProperties) = elabCallArgs(outCache.clone(), inEnv.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("inStream")).clone() }), list![e.clone()], metamodelica::nil(), metamodelica::nil(), inImpl.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinActualStream(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImpl: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut exp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(inArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    (outCache, exp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImpl.clone(), true, inPrefix.clone(), inInfo.clone())?;
    ty = Types::getPropType(outProperties.clone())?;
    outExp = elabBuiltinStreamOperator(outCache.clone(), inEnv.clone(), (literal!("actualStream")).clone(), exp.clone(), ty.clone(), inInfo.clone())?;
    if Types::dimensionsKnown(ty.clone())? {
        (outCache, outExp, outProperties) = elabCallArgs(outCache.clone(), inEnv.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("actualStream")).clone() }), list![e.clone()], metamodelica::nil(), metamodelica::nil(), inImpl.clone(), inPrefix.clone(), inInfo.clone())?;
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinStreamOperator(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inOperator: ArcStr, mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. } => {
            inExp.clone()
        },
        _ => {
            let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut exp: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(Expression::flattenArrayExpToList(inExp.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            exp = __pa0.clone();
            validateBuiltinStreamOperator(inCache.clone(), inEnv.clone(), exp.clone(), inType.clone(), (inOperator.clone()).clone(), inInfo.clone())?;
            et = Types::simplifyType(inType.clone())?;
            exp = Expression::makePureBuiltinCall((inOperator.clone()).clone(), list![exp.clone()], et.clone());
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn validateBuiltinStreamOperator(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inOperand: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inOperator: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inOperand.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
                    (_, attr, _, _, _, _, _, _, _) = Lookup::lookupVar(inCache.clone(), inEnv.clone(), cr.clone())?;
                    ::match_deref::match_deref! { match &(attr.clone()) {
                        Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut op_str: ArcStr = arcstr::literal!("");
                    op_str = (ExpressionBasics::printExpStr(inOperand.clone())?).clone();
                    Error::addSourceMessage(Error::NON_STREAM_OPERAND_IN_STREAM_OPERATOR.clone(), list![(op_str.clone()).clone(), (inOperator.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn makePreLst(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Types::simplifyType(inType.clone())?;
    outExpl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (inExpl.clone()).into_iter().cloned() {
            let __x = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![e.clone()], ty.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpl)
}

fn elabBuiltinPreMatrix(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp @ Deref @ DAE::Exp::MATRIX { .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut exp = (*exp).clone();
            assign_variant_field!(exp => DAE::Exp::MATRIX; matrix = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut row in (var_field!((*exp).matrix, DAE::Exp::MATRIX).clone()).into_iter().cloned() {
            let __x = makePreLst(row.clone(), inType.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp.clone()
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn elabBuiltinArray(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut arr_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut len: i32 = 0;
    (outCache, expl, props) = elabExpList(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elabBuiltinArray2(expl.clone(), props.clone(), inPrefix.clone(), inInfo.clone())?) {
        (_, DAE::Properties::PROP { type_: __pa0, constFlag: __pa1 }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    c = __pa1.clone();
    len = (expl.clone().len() as i32);
    arr_ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: len.clone() })] });
    outProperties = DAE::Properties::PROP { type_: arr_ty.clone(), constFlag: c.clone() };
    arr_ty = Types::simplifyType(arr_ty.clone())?;
    outExp = Arc::new(DAE::Exp::ARRAY { ty: arr_ty.clone(), scalar: Types::isArray(ty.clone()), array: expl.clone() });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinArray2(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inProperties: Arc<metamodelica::List<DAE::Properties>>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, DAE::Properties)> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outProperties: DAE::Properties;
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut prop: DAE::Properties;
    if !(sameDimensions(inProperties.clone())?) {
        pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
        Error::addSourceMessageAndFail(Error::DIFFERENT_DIM_SIZE_IN_ARGUMENTS.clone(), list![(literal!("array")).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    prop = if (Types::propsContainReal(inProperties.clone())?) {DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }} else {listHead(inProperties.clone())?};
    (outExpl, outProperties) = elabBuiltinArray3(inExpl.clone(), inProperties.clone(), prop.clone())?;
    Ok((outExpl, outProperties))
}

fn elabBuiltinArray3(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inPropertiesLst: Arc<metamodelica::List<DAE::Properties>>, mut inProperties: DAE::Properties) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, DAE::Properties)> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outProperties: DAE::Properties = listHead(inPropertiesLst.clone())?;
    let mut prop: DAE::Properties;
    let mut rest_props: Arc<metamodelica::List<DAE::Properties>> = inPropertiesLst.clone();
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_props.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        prop = __pa0.clone();
        rest_props = __pa1.clone();
        (e, _) = Types::matchProp(e.clone(), prop.clone(), inProperties.clone(), true)?;
        outExpl = cons(e.clone(), outExpl.clone());
    }
    outExpl = outExpl.clone().reverse();
    Ok((outExpl, outProperties))
}

fn elabBuiltinZeros(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabBuiltinFill(inCache.clone(), inEnv.clone(), cons(Arc::new(Absyn::Exp::INTEGER { value: 0 }), inPosArgs.clone()), metamodelica::nil(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn sameDimensions(mut inProps: Arc<metamodelica::List<DAE::Properties>>) -> Result<bool> {
    let mut res: bool = false;
    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>> = metamodelica::nil();
    types = List::map(inProps.clone(), (std::sync::Arc::new(Types::getPropType) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Properties) -> Result<Arc<DAE::Type>> + 'static>));
    dims = List::map(types.clone(), (std::sync::Arc::new(fnptr!(TypesDump::getDimensions, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> + 'static>));
    res = sameDimensions2(dims.clone())?;
    Ok(res)
}

fn sameDimensionsExceptionDimX(mut inProps: Arc<metamodelica::List<DAE::Properties>>, mut dimException: i32) -> Result<bool> {
    let mut res: bool = false;
    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>> = metamodelica::nil();
    types = List::map(inProps.clone(), (std::sync::Arc::new(Types::getPropType) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Properties) -> Result<Arc<DAE::Type>> + 'static>));
    dims = List::map(types.clone(), (std::sync::Arc::new(fnptr!(TypesDump::getDimensions, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> + 'static>));
    dims = List::map1(dims.clone(), Arc::new(listDelete.clone()), dimException.clone());
    res = sameDimensions2(dims.clone())?;
    Ok(res)
}

fn sameDimensions2(mut inDimensions: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<bool> {
    let mut outSame: bool = true;
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut rest_dims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>> = inDimensions.clone();
    if inDimensions.clone().is_empty() {
        return Ok(outSame.clone());
    }
    while !(listHead(rest_dims.clone())?.is_empty()) {
        dims = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        for mut d in (rest_dims.clone()).into_iter().cloned() {
            let __x = listHead(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if !(sameDimensions3(dims.clone())?) {
            outSame = false;
            return Ok(outSame.clone());
        }
        rest_dims = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>> = metamodelica::nil();
        for mut d in (rest_dims.clone()).into_iter().cloned() {
            let __x = listRest(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    }
    for mut d in &*rest_dims.clone() {
        let mut d = d.clone();
        let true = (d.clone().is_empty()) else { bail!("pattern mismatch") };
    }
    Ok(outSame)
}

fn sameDimensions3(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<bool> {
    let mut outSame: bool = true;
    let mut dim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    if inDims.clone().is_empty() {
        return Ok(outSame.clone());
    }
    dim1 = listHead(inDims.clone())?;
    for mut dim2 in &*listRest(inDims.clone())? {
        let mut dim2 = dim2.clone();
        if !(Expression::dimensionsEqual(dim1.clone(), dim2.clone())?) {
            outSame = false;
            return Ok(outSame.clone());
        }
    }
    Ok(outSame)
}

fn elabBuiltinOnes(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabBuiltinFill(inCache.clone(), inEnv.clone(), cons(Arc::new(Absyn::Exp::INTEGER { value: 1 }), inPosArgs.clone()), metamodelica::nil(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinMax(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFnArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImpl: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabBuiltinMinMaxCommon(inCache.clone(), inEnv.clone(), (literal!("max")).clone(), inFnArgs.clone(), inImpl.clone(), inPrefix.clone(), info.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinMin(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFnArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImpl: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabBuiltinMinMaxCommon(inCache.clone(), inEnv.clone(), (literal!("min")).clone(), inFnArgs.clone(), inImpl.clone(), inPrefix.clone(), info.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinMinMaxCommon(mut cache: FCore::Cache, mut env: FCore::Graph, mut inFnName: ArcStr, mut inFnArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut r#impl: bool, mut prefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut cache: FCore::Cache = cache;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (::match_deref::match_deref! { match &(inFnArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: arrexp, tail: Deref @ metamodelica::List::Nil } => {
            let mut arrexp_1: Arc<DAE::Exp>;
            let mut call: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut elt_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut c: DAE::Const = DAE::Const::C_CONST;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arrexp.clone(), r#impl.clone(), true, prefix.clone(), info.clone())?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            arrexp_1 = __pa1.clone();
            ty = __pa2.clone();
            c = __pa3.clone();
            let true = (Types::isArray(ty.clone())) else { bail!("pattern mismatch") };
            arrexp_1 = Expression::matrixToArray(arrexp_1.clone())?;
            elt_ty = Types::arrayElementType(ty.clone());
            tp = Types::simplifyType(elt_ty.clone())?;
            let false = (Types::isString(tp.clone())) else { bail!("pattern mismatch") };
            call = Expression::makePureBuiltinCall((inFnName.clone()).clone(), list![arrexp_1.clone()], tp.clone());
            (call.clone(), DAE::Properties::PROP { type_: elt_ty.clone(), constFlag: c.clone() })
        },
        Deref @ metamodelica::List::Cons { head: s1, tail: Deref @ metamodelica::List::Cons { head: s2, tail: Deref @ metamodelica::List::Nil } } => {
            let mut s1_1: Arc<DAE::Exp>;
            let mut s2_1: Arc<DAE::Exp>;
            let mut call: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut c: DAE::Const = DAE::Const::C_CONST;
            let mut c1: DAE::Const = DAE::Const::C_CONST;
            let mut c2: DAE::Const = DAE::Const::C_CONST;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), s1.clone(), r#impl.clone(), true, prefix.clone(), info.clone())?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            s1_1 = __pa1.clone();
            ty1 = __pa2.clone();
            c1 = __pa3.clone();
            let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), s2.clone(), r#impl.clone(), true, prefix.clone(), info.clone())?) {
                (__pa4, __pa5, DAE::Properties::PROP { type_: __pa6, constFlag: __pa7 }) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa4.clone();
            s2_1 = __pa5.clone();
            ty2 = __pa6.clone();
            c2 = __pa7.clone();
            let (__pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(Types::checkTypeCompat(s1_1.clone(), ty1.clone(), s2_1.clone(), ty2.clone(), false)?) {
                (__pa8, __pa9, __pa10, true) => (__pa8.clone(), __pa9.clone(), __pa10.clone()),
                _ => bail!("pattern mismatch"),
            } };
            s1_1 = __pa8.clone();
            s2_1 = __pa9.clone();
            ty = __pa10.clone();
            c = Types::constAnd(c1.clone(), c2.clone());
            tp = Types::simplifyType(ty.clone())?;
            let false = (Types::isString(tp.clone())) else { bail!("pattern mismatch") };
            call = Expression::makePureBuiltinCall((inFnName.clone()).clone(), list![s1_1.clone(), s2_1.clone()], tp.clone());
            (call.clone(), DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cache, outExp, outProperties))
}

fn elabBuiltinClock(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = ({
        let mut prop: DAE::Properties = DAE::Properties::PROP { type_: DAE::T_CLOCK_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR };
        'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
                    let mut call: Arc<DAE::Exp>;
                    call = Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK) });
                    Ok((cache.clone(), call.clone(), DAE::Properties::PROP { type_: DAE::T_CLOCK_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: aintervalCounter, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut intervalCounter: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, intervalCounter, prop1) = elabExpInExpression(cache.clone(), env.clone(), aintervalCounter.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    (intervalCounter, _) = Types::matchType(intervalCounter.clone(), ty1.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
                    call = Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: intervalCounter.clone(), resolution: Arc::new(DAE::Exp::ICONST { integer: 1 }) }) });
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: aintervalCounter, tail: Deref @ metamodelica::List::Cons { head: aresolution, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut intervalCounter: Arc<DAE::Exp>;
                    let mut resolution: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, intervalCounter, prop1) = elabExpInExpression(cache.clone(), env.clone(), aintervalCounter.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, resolution, prop2) = elabExpInExpression(cache.clone(), env.clone(), aresolution.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    ty2 = Types::arrayElementType(Types::getPropType(prop2.clone())?);
                    (intervalCounter, _) = Types::matchType(intervalCounter.clone(), ty1.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
                    (resolution, _) = Types::matchType(resolution.clone(), ty2.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
                    (cache, val) = Ceval::ceval(cache.clone(), env.clone(), resolution.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(val.clone())? >= 1, Error::WRONG_VALUE_OF_ARG.clone(), list![(literal!("Clock")).clone(), (literal!("resolution")).clone(), (ValuesDump::valString(val.clone())?).clone(), (literal!(">= 1")).clone()], info.clone())?;
                    resolution = ValuesUtil::valueExp(val.clone(), Some(resolution.clone()))?;
                    call = Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: intervalCounter.clone(), resolution: resolution.clone() }) });
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: ainterval, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut interval: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, interval, prop1) = elabExpInExpression(cache.clone(), env.clone(), ainterval.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    (interval, _) = Types::matchType(interval.clone(), ty1.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    call = Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::REAL_CLOCK { interval: interval.clone() }) });
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: acondition, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut condition: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, condition, prop1) = elabExpInExpression(cache.clone(), env.clone(), acondition.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    (condition, _) = Types::matchType(condition.clone(), ty1.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
                    call = Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: condition.clone(), startInterval: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }) }) });
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: acondition, tail: Deref @ metamodelica::List::Cons { head: astartInterval, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut condition: Arc<DAE::Exp>;
                    let mut startInterval: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, condition, prop1) = elabExpInExpression(cache.clone(), env.clone(), acondition.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, startInterval, prop2) = elabExpInExpression(cache.clone(), env.clone(), astartInterval.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    ty2 = Types::arrayElementType(Types::getPropType(prop2.clone())?);
                    (condition, _) = Types::matchType(condition.clone(), ty1.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
                    (startInterval, _) = Types::matchType(startInterval.clone(), ty2.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    call = Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: condition.clone(), startInterval: startInterval.clone() }) });
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: ac, tail: Deref @ metamodelica::List::Cons { head: asolverMethod, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut c: Arc<DAE::Exp>;
                    let mut solverMethod: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, c, prop1) = elabExpInExpression(cache.clone(), env.clone(), ac.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, solverMethod, prop2) = elabExpInExpression(cache.clone(), env.clone(), asolverMethod.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    ty2 = Types::arrayElementType(Types::getPropType(prop2.clone())?);
                    (c, _) = Types::matchType(c.clone(), ty1.clone(), DAE::T_CLOCK_DEFAULT().clone(), true)?;
                    (solverMethod, _) = Types::matchType(solverMethod.clone(), ty2.clone(), DAE::T_STRING_DEFAULT().clone(), true)?;
                    (cache, val) = Ceval::ceval(cache.clone(), env.clone(), solverMethod.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    solverMethod = ValuesUtil::valueExp(val.clone(), Some(solverMethod.clone()))?;
                    call = Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::SOLVER_CLOCK { c: c.clone(), solverMethod: solverMethod.clone() }) });
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: ac, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argValue: asolverMethod, argName: Deref @ "solverMethod" }, tail: Deref @ metamodelica::List::Nil }, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut c: Arc<DAE::Exp>;
                    let mut solverMethod: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, c, prop1) = elabExpInExpression(cache.clone(), env.clone(), ac.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, solverMethod, prop2) = elabExpInExpression(cache.clone(), env.clone(), asolverMethod.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    ty2 = Types::arrayElementType(Types::getPropType(prop2.clone())?);
                    (c, _) = Types::matchType(c.clone(), ty1.clone(), DAE::T_CLOCK_DEFAULT().clone(), true)?;
                    (solverMethod, _) = Types::matchType(solverMethod.clone(), ty2.clone(), DAE::T_STRING_DEFAULT().clone(), true)?;
                    (cache, val) = Ceval::ceval(cache.clone(), env.clone(), solverMethod.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    solverMethod = ValuesUtil::valueExp(val.clone(), Some(solverMethod.clone()))?;
                    call = Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::SOLVER_CLOCK { c: c.clone(), solverMethod: solverMethod.clone() }) });
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinHold(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("hold")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("hold")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSample(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: astart, tail: Deref @ metamodelica::List::Cons { head: ainterval, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut start: Arc<DAE::Exp>;
                    let mut interval: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, start, prop1) = elabExpInExpression(cache.clone(), env.clone(), astart.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, interval, prop2) = elabExpInExpression(cache.clone(), env.clone(), ainterval.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::getPropType(prop1.clone())?;
                    ty2 = Types::getPropType(prop2.clone())?;
                    (start, _) = Types::matchType(start.clone(), ty1.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    (interval, _) = Types::matchType(interval.clone(), ty2.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("start")).clone(), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("interval")).clone(), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }) });
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    call = __pa1.clone();
                    prop = __pa2.clone();
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Cons { head: ac, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut c: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut variability: DAE::Const = DAE::Const::C_CONST;
                    let mut cache = (*cache).clone();
                    (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, c, prop2) = elabExpInExpression(cache.clone(), env.clone(), ac.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    ty2 = Types::arrayElementType(Types::getPropType(prop2.clone())?);
                    variability = Types::getPropConst(prop1.clone())?;
                    (c, _) = Types::matchType(c.clone(), ty2.clone(), DAE::T_CLOCK_DEFAULT().clone(), true)?;
                    ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: variability.clone(), par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("c")).clone(), ty: ty2.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: Some(Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK) })) })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }) });
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    call = __pa1.clone();
                    prop = __pa2.clone();
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
                    let mut call: Arc<DAE::Exp>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop1: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut variability: DAE::Const = DAE::Const::C_CONST;
                    let mut cache = (*cache).clone();
                    (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
                    variability = Types::getPropConst(prop1.clone())?;
                    ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: variability.clone(), par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("c")).clone(), ty: DAE::T_CLOCK_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: Some(Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK) })) })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }) });
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    call = __pa1.clone();
                    prop = __pa2.clone();
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinShiftSample(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Cons { head: ashiftCounter, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut shiftCounter: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop2: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut aresolution: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let mut ashiftCounter = (*ashiftCounter).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (cache, shiftCounter, prop2) = elabExpInExpression(cache.clone(), env.clone(), ashiftCounter.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (shiftCounter, _) = Types::matchType(shiftCounter.clone(), Types::getPropType(prop2.clone())?, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, val) = Ceval::ceval(cache.clone(), env.clone(), shiftCounter.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(val.clone())? >= 0, Error::WRONG_VALUE_OF_ARG.clone(), list![(literal!("shiftSample")).clone(), (literal!("shiftCounter")).clone(), (ValuesDump::valString(val.clone())?).clone(), (literal!(">= 0")).clone()], info.clone())?;
            ashiftCounter = Arc::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(val.clone())? });
            aresolution = Arc::new(Absyn::Exp::INTEGER { value: 1 });
            ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("shiftCounter")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("resolution")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("shiftSample")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("shiftSample")).clone() }), list![au.clone(), ashiftCounter.clone(), aresolution.clone()], nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Cons { head: ashiftCounter, tail: Deref @ metamodelica::List::Cons { head: aresolution, tail: Deref @ metamodelica::List::Nil } } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut shiftCounter: Arc<DAE::Exp>;
            let mut resolution: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop2: DAE::Properties;
            let mut prop3: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut rval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let mut ashiftCounter = (*ashiftCounter).clone();
            let mut aresolution = (*aresolution).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (cache, shiftCounter, prop2) = elabExpInExpression(cache.clone(), env.clone(), ashiftCounter.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (shiftCounter, _) = Types::matchType(shiftCounter.clone(), Types::getPropType(prop2.clone())?, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, val) = Ceval::ceval(cache.clone(), env.clone(), shiftCounter.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(val.clone())? >= 0, Error::WRONG_VALUE_OF_ARG.clone(), list![(literal!("shiftSample")).clone(), (literal!("shiftCounter")).clone(), (ValuesDump::valString(val.clone())?).clone(), (literal!(">= 0")).clone()], info.clone())?;
            ashiftCounter = Arc::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(val.clone())? });
            (cache, resolution, prop3) = elabExpInExpression(cache.clone(), env.clone(), aresolution.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (resolution, _) = Types::matchType(resolution.clone(), Types::getPropType(prop3.clone())?, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, rval) = Ceval::ceval(cache.clone(), env.clone(), resolution.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(rval.clone())? >= 1, Error::WRONG_VALUE_OF_ARG.clone(), list![(literal!("shiftSample")).clone(), (literal!("resolution")).clone(), (ValuesDump::valString(rval.clone())?).clone(), (literal!(">= 1")).clone()], info.clone())?;
            aresolution = Arc::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(rval.clone())? });
            ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("shiftCounter")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("resolution")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("shiftSample")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("shiftSample")).clone() }), list![au.clone(), ashiftCounter.clone(), aresolution.clone()], nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinBackSample(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Cons { head: abackCounter, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut backCounter: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop2: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut aresolution: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let mut abackCounter = (*abackCounter).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (cache, backCounter, prop2) = elabExpInExpression(cache.clone(), env.clone(), abackCounter.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (backCounter, _) = Types::matchType(backCounter.clone(), Types::getPropType(prop2.clone())?, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, val) = Ceval::ceval(cache.clone(), env.clone(), backCounter.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(val.clone())? >= 0, Error::WRONG_VALUE_OF_ARG.clone(), list![(literal!("backSample")).clone(), (literal!("backCounter")).clone(), (ValuesDump::valString(val.clone())?).clone(), (literal!(">= 0")).clone()], info.clone())?;
            abackCounter = Arc::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(val.clone())? });
            aresolution = Arc::new(Absyn::Exp::INTEGER { value: 1 });
            ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("backCounter")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("resolution")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("backSample")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("backSample")).clone() }), list![au.clone(), abackCounter.clone(), aresolution.clone()], nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Cons { head: abackCounter, tail: Deref @ metamodelica::List::Cons { head: aresolution, tail: Deref @ metamodelica::List::Nil } } }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut backCounter: Arc<DAE::Exp>;
            let mut resolution: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop2: DAE::Properties;
            let mut prop3: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut rval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let mut abackCounter = (*abackCounter).clone();
            let mut aresolution = (*aresolution).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (cache, backCounter, prop2) = elabExpInExpression(cache.clone(), env.clone(), abackCounter.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (backCounter, _) = Types::matchType(backCounter.clone(), Types::getPropType(prop2.clone())?, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, val) = Ceval::ceval(cache.clone(), env.clone(), backCounter.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(val.clone())? >= 0, Error::WRONG_VALUE_OF_ARG.clone(), list![(literal!("backSample")).clone(), (literal!("backCounter")).clone(), (ValuesDump::valString(val.clone())?).clone(), (literal!(">= 0")).clone()], info.clone())?;
            abackCounter = Arc::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(val.clone())? });
            (cache, resolution, prop3) = elabExpInExpression(cache.clone(), env.clone(), aresolution.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            (resolution, _) = Types::matchType(resolution.clone(), Types::getPropType(prop3.clone())?, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, rval) = Ceval::ceval(cache.clone(), env.clone(), resolution.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(rval.clone())? >= 1, Error::WRONG_VALUE_OF_ARG.clone(), list![(literal!("backSample")).clone(), (literal!("resolution")).clone(), (ValuesDump::valString(rval.clone())?).clone(), (literal!(">= 1")).clone()], info.clone())?;
            aresolution = Arc::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(rval.clone())? });
            ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("backCounter")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("resolution")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("backSample")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("backSample")).clone() }), list![au.clone(), abackCounter.clone(), aresolution.clone()], nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinNoClock(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("noClock")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("noClock")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinFirstTick(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: metamodelica::nil(), funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("firstTick")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("firstTick")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("firstTick")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("firstTick")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinInterval(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: metamodelica::nil(), funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("interval")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("interval")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: au, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            ty1 = Types::arrayElementType(Types::getPropType(prop1.clone())?);
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("u")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("interval")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("interval")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isBlockTypeWorkaround(mut ity: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ity.clone()) {
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => isBlockTypeWorkaround(var_field!((*ity).complexType, DAE::Type::T_SUBTYPE_BASIC).clone()),
        Deref @ DAE::Type::T_COMPLEX { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn elabBuiltinTransition(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (match (inCache.clone(), inEnv.clone(), inBoolean.clone(), inPrefix.clone()) {
        (mut cache, mut env, mut r#impl, mut pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop: DAE::Properties;
            let mut n: i32 = 0;
            let mut strMsg0: ArcStr = arcstr::literal!("");
            let mut strPre: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut slist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            slist = List::map(nargs.clone(), (std::sync::Arc::new(Dump::printNamedArgStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::NamedArg>) -> Result<ArcStr> + 'static>));
            s1 = (Dump::printExpLstStr(args.clone())).clone();
            s2 = stringDelimitList(cons((s1.clone()).clone(), slist.clone()), (literal!(", ")).clone());
            strMsg0 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("transition(")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            strPre = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
            n = (args.clone().len() as i32);
            ty1 = elabBuiltinTransition2(cache.clone(), env.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info.clone(), (literal!("from")).clone(), n.clone(), (strMsg0.clone()).clone(), (strPre.clone()).clone())?;
            ty2 = elabBuiltinTransition2(cache.clone(), env.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info.clone(), (literal!("to")).clone(), n.clone(), (strMsg0.clone()).clone(), (strPre.clone()).clone())?;
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("from")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("to")).clone(), ty: ty2.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("condition")).clone(), ty: DAE::T_BOOL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("immediate")).clone(), ty: DAE::T_BOOL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: Some(Arc::new(DAE::Exp::BCONST { bool: true })) }), Arc::new(DAE::FuncArg { name: (literal!("reset")).clone(), ty: DAE::T_BOOL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: Some(Arc::new(DAE::Exp::BCONST { bool: true })) }), Arc::new(DAE::FuncArg { name: (literal!("synchronize")).clone(), ty: DAE::T_BOOL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: Some(Arc::new(DAE::Exp::BCONST { bool: false })) }), Arc::new(DAE::FuncArg { name: (literal!("priority")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: Some(Arc::new(DAE::Exp::ICONST { integer: 1 })) })], funcResultType: DAE::T_NORETCALL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("transition")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("transition")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinTransition2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut argName: ArcStr, mut n: i32, mut strMsg0: ArcStr, mut strPre: ArcStr) -> Result<Arc<DAE::Type>> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut arg1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut prop1: DAE::Properties;
    let mut nPos: i32 = 0;
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut strPos: ArcStr = arcstr::literal!("");
    let mut strMsg1: ArcStr = arcstr::literal!("");
    let mut b1: bool = false;
    strPos = (if (argName.clone() == literal!("from")) {literal!("first")} else {literal!("second")}).clone();
    nPos = if (argName.clone() == literal!("from")) {1} else {2};
    b1 = List::isMemberOnTrue((argName.clone()).clone(), nargs.clone(), (std::sync::Arc::new(fnptr!(elabBuiltinTransition3, ArcStr, Arc<Absyn::NamedArg>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::NamedArg>) -> Result<bool> + 'static>));
    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*strMsg0.clone()); __mm_s.push_str(&*literal!(", named argument \"")); __mm_s.push_str(&*argName.clone()); __mm_s.push_str(&*literal!("\" already has a value.")); ArcStr::from(__mm_s) }).clone();
    Error::assertionOrAddSourceMessage(!(b1.clone() && n.clone() >= nPos.clone()), Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(s1.clone()).clone(), (strPre.clone()).clone()], info.clone())?;
    s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*strMsg0.clone()); __mm_s.push_str(&*literal!(", missing value for ")); __mm_s.push_str(&*strPos.clone()); __mm_s.push_str(&*literal!(" argument \"")); __mm_s.push_str(&*argName.clone()); __mm_s.push_str(&*literal!("\".")); ArcStr::from(__mm_s) }).clone();
    Error::assertionOrAddSourceMessage(b1.clone() || n.clone() >= nPos.clone(), Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(s2.clone()).clone(), (strPre.clone()).clone()], info.clone())?;
    arg1 = elabBuiltinTransition5((argName.clone()).clone(), b1.clone(), args.clone(), nargs.clone())?;
    (_, _, prop1) = elabExpInExpression(inCache.clone(), inEnv.clone(), arg1.clone(), inBoolean.clone(), true, inPrefix.clone(), info.clone())?;
    ty = Types::getPropType(prop1.clone())?;
    strMsg1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*strMsg0.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*strPos.clone()); __mm_s.push_str(&*literal!("argument needs to be a block instance.")); ArcStr::from(__mm_s) }).clone();
    Error::assertionOrAddSourceMessage(isBlockTypeWorkaround(ty.clone()), Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(strMsg1.clone()).clone(), (strPre.clone()).clone()], info.clone())?;
    Ok(ty)
}

fn elabBuiltinTransition3(mut name: ArcStr, mut namedArg: Arc<Absyn::NamedArg>) -> bool {
    let mut outIsEqual: bool = false;
    outIsEqual = (::match_deref::match_deref! { match &(namedArg.clone()) {
        Deref @ Absyn::NamedArg { .. } => {
            stringEq((name.clone()).clone(), (namedArg.argName.clone()).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEqual
}

fn elabBuiltinTransition4(mut inElement: Arc<Absyn::NamedArg>) -> Result<Arc<Absyn::Exp>> {
    let mut argValue: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::NamedArg { argValue: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    argValue = __pa0.clone();
    Ok(argValue)
}

fn elabBuiltinTransition5(mut argName: ArcStr, mut getAsNamedArg: bool, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Arc<Absyn::Exp>> {
    let mut argValue: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    argValue = (::match_deref::match_deref! { match &((argName.clone(), getAsNamedArg.clone())) {
        (Deref @ "from", true) => {
            let mut namedArg: Arc<Absyn::NamedArg> = Arc::new(<Absyn::NamedArg as ::std::default::Default>::default());
            namedArg = List::getMemberOnTrue((literal!("from")).clone(), nargs.clone(), (std::sync::Arc::new(fnptr!(elabBuiltinTransition3, ArcStr, Arc<Absyn::NamedArg>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::NamedArg>) -> Result<bool> + 'static>))?;
            elabBuiltinTransition4(namedArg.clone())?
        },
        (Deref @ "from", false) => {
            listHead(args.clone())?
        },
        (Deref @ "to", true) => {
            let mut namedArg: Arc<Absyn::NamedArg> = Arc::new(<Absyn::NamedArg as ::std::default::Default>::default());
            namedArg = List::getMemberOnTrue((literal!("to")).clone(), nargs.clone(), (std::sync::Arc::new(fnptr!(elabBuiltinTransition3, ArcStr, Arc<Absyn::NamedArg>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::NamedArg>) -> Result<bool> + 'static>))?;
            elabBuiltinTransition4(namedArg.clone())?
        },
        (Deref @ "to", false) => {
            (args.clone()).get(2)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(argValue)
}

fn elabBuiltinInitialState(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: astate, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut strMsg: ArcStr = arcstr::literal!("");
            let mut strPre: ArcStr = arcstr::literal!("");
            let mut cache = (*cache).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), astate.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            ty1 = Types::getPropType(prop1.clone())?;
            strMsg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("initialState(")); __mm_s.push_str(&*Dump::printExpLstStr(args.clone())); __mm_s.push_str(&*literal!("), Argument needs to be a block instance.")); ArcStr::from(__mm_s) }).clone();
            strPre = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
            Error::assertionOrAddSourceMessage(isBlockTypeWorkaround(ty1.clone()), Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(strMsg.clone()).clone(), (strPre.clone()).clone()], info.clone())?;
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("state")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_NORETCALL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("initialState")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("initialState")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinActiveState(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: astate, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut strMsg: ArcStr = arcstr::literal!("");
            let mut strPre: ArcStr = arcstr::literal!("");
            let mut cache = (*cache).clone();
            (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), astate.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
            ty1 = Types::getPropType(prop1.clone())?;
            strMsg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("activeState(")); __mm_s.push_str(&*Dump::printExpLstStr(args.clone())); __mm_s.push_str(&*literal!("), Argument needs to be a block instance.")); ArcStr::from(__mm_s) }).clone();
            strPre = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
            Error::assertionOrAddSourceMessage(isBlockTypeWorkaround(ty1.clone()), Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(strMsg.clone()).clone(), (strPre.clone()).clone()], info.clone())?;
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("state")).clone(), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("activeState")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("activeState")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinTicksInState(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: metamodelica::nil(), funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("ticksInState")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("ticksInState")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinTimeInState(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, r#impl, pre) => {
            let mut call: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: metamodelica::nil(), funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("timeInState")).clone() }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], Arc::new(Absyn::Path::IDENT { name: (literal!("timeInState")).clone() }), args.clone(), nargs.clone(), metamodelica::nil(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            call = __pa1.clone();
            prop = __pa2.clone();
            (cache.clone(), call.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinBoolean(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = verifyBuiltInHandlerType(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inImplicit.clone(), (std::sync::Arc::new(Types::isIntegerOrRealOrBooleanOrSubTypeOfEither) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>), (literal!("boolean")).clone(), inPrefix.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinIntegerEnum(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = verifyBuiltInHandlerType(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inImplicit.clone(), (std::sync::Arc::new(fnptr!(Types::isEnumeration, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>), (literal!("Integer")).clone(), inPrefix.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinNoevent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("noEvent")).clone(), inInfo.clone())?;
    e = listHead(inPosArgs.clone())?;
    (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    outExp = Expression::makePureBuiltinCall((literal!("noEvent")).clone(), list![outExp.clone()], DAE::T_BOOL_DEFAULT().clone());
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinEdge(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut msg: ArcStr = arcstr::literal!("");
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("edge")).clone(), inInfo.clone())?;
    (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (outProperties.clone()) else { bail!("pattern mismatch") };
    ty = __pa0.clone();
    c = __pa1.clone();
    if !(Types::isScalarBoolean(ty.clone())) {
        msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("edge(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outExp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        Error::addSourceMessageAndFail(Error::TYPE_ERROR.clone(), list![(msg.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if Types::isVar(c.clone()) {
        outExp = Expression::makePureBuiltinCall((literal!("edge")).clone(), list![outExp.clone()], DAE::T_BOOL_DEFAULT().clone());
    } else {
        outExp = Arc::new(DAE::Exp::BCONST { bool: false });
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinDer(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut exp_str: ArcStr = arcstr::literal!("");
    let mut ty_str: ArcStr = arcstr::literal!("");
    if FGraph::inFunctionScope(inEnv.clone())? {
        Error::addSourceMessageAndFail(Error::DERIVATIVE_FUNCTION_CONTEXT.clone(), metamodelica::nil(), inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("der")).clone(), inInfo.clone())?;
    (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (outProperties.clone()) else { bail!("pattern mismatch") };
    ty = __pa0.clone();
    c = __pa1.clone();
    if !(Types::isRealOrSubTypeReal(Types::arrayElementType(ty.clone()))?) {
        exp_str = (Dump::printExpStr(listHead(inPosArgs.clone())?)?).clone();
        ty_str = (TypesDump::unparseTypeNoAttr(ty.clone())?).clone();
        Error::addSourceMessageAndFail(Error::DERIVATIVE_NON_REAL.clone(), list![(exp_str.clone()).clone(), (ty_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if Types::isVar(c.clone()) {
        if Types::dimensionsKnown(ty.clone())? {
            (outCache, outExp, outProperties) = elabCallArgs(inCache.clone(), inEnv.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), inPosArgs.clone(), metamodelica::nil(), metamodelica::nil(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
        } else {
            outExp = Expression::makePureBuiltinCall((literal!("der")).clone(), list![outExp.clone()], Types::simplifyType(ty.clone())?);
        }
    } else {
        dims = TypesDump::getDimensions(ty.clone());
        (outExp, ty) = Expression::makeZeroExpression(dims.clone())?;
        outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinChange(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: SCode::Variability = SCode::Variability::CONST;
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("change")).clone(), inInfo.clone())?;
    e = listHead(inPosArgs.clone())?;
    if !(AbsynUtil::isCref(e.clone())) {
        pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
        Error::addSourceMessageAndFail(Error::ARGUMENT_MUST_BE_VARIABLE.clone(), list![(literal!("First")).clone(), (literal!("change")).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (outProperties.clone()) else { bail!("pattern mismatch") };
    ty = __pa0.clone();
    c = __pa1.clone();
    if Types::isSimpleType(ty.clone()) {
        if Types::isParameterOrConstant(c.clone()) {
            outExp = Arc::new(DAE::Exp::BCONST { bool: false });
            outProperties = DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
        } else if Types::isDiscreteType(ty.clone()) {
            outExp = Expression::makePureBuiltinCall((literal!("change")).clone(), list![outExp.clone()], DAE::T_BOOL_DEFAULT().clone());
            outProperties = DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR };
        } else {
            cref = Expression::getCrefFromCrefOrAsub(outExp.clone())?;
            (outCache, attr, _, _, _, _, _, _, _) = Lookup::lookupVar(outCache.clone(), inEnv.clone(), cref.clone())?;
            let __pa2 = ::match_deref::match_deref! { match &(attr.clone()) {
                Deref @ DAE::Attributes { variability: __pa2, .. } => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            var = __pa2.clone();
            if var.clone() == openmodelica_frontend_types::SCode::Variability::DISCRETE {
                outExp = Expression::makePureBuiltinCall((literal!("change")).clone(), list![outExp.clone()], DAE::T_BOOL_DEFAULT().clone());
                outProperties = DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR };
            } else {
                pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
                Error::addSourceMessageAndFail(Error::ARGUMENT_MUST_BE_DISCRETE_VAR.clone(), list![(literal!("First")).clone(), (literal!("change")).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
        }
    } else {
        pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
        Error::addSourceMessageAndFail(Error::TYPE_MUST_BE_SIMPLE.clone(), list![(literal!("operand to change")).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinCat(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut dim_exp: Arc<DAE::Exp>;
    let mut dim_props: DAE::Properties;
    let mut dim_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut result_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dim_c: DAE::Const = DAE::Const::C_CONST;
    let mut arr_c: DAE::Const = DAE::Const::C_CONST;
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut exp_str: ArcStr = arcstr::literal!("");
    let mut dim_int: i32 = 0;
    let mut arr_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut arr_props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut arr_tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    if (inPosArgs.clone().len() as i32) < 2 || !(inNamedArgs.clone().is_empty()) {
        Error::addSourceMessageAndFail(Error::WRONG_NO_OF_ARGS.clone(), list![(literal!("cat")).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (outCache, dim_exp, dim_props) = elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (dim_props.clone()) else { bail!("pattern mismatch") };
    dim_ty = __pa0.clone();
    dim_c = __pa1.clone();
    if !(Types::isScalarInteger(dim_ty.clone())) {
        pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
        Error::addSourceMessageAndFail(Error::ARGUMENT_MUST_BE_INTEGER.clone(), list![(literal!("First")).clone(), (literal!("cat")).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(Ceval::ceval(inCache.clone(), inEnv.clone(), dim_exp.clone(), false, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?) {
        (__pa2, Deref @ Values::Value::INTEGER { integer: __pa3 }) => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa2.clone();
    dim_int = __pa3.clone();
    (outCache, arr_expl, arr_props) = elabExpList(outCache.clone(), inEnv.clone(), listRest(inPosArgs.clone())?, inImplicit.clone(), true, inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
    arr_tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut p in (arr_props.clone()).into_iter().cloned() {
            let __x = Types::getPropType(p.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut t in (arr_tys.clone()).into_iter().cloned() {
            let __x = Types::makeNthDimUnknown(t.clone(), dim_int.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })) {
        Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa4.clone();
    tys = __pa5.clone();
    result_ty = List::fold1(tys.clone(), (std::sync::Arc::new(Types::arraySuperType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, SourceInfo, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>), inInfo.clone(), ty.clone());
    if let Ok((__pa6, __pa7)) = Types::matchTypes(arr_expl.clone(), arr_tys.clone(), result_ty.clone(), false) {
        arr_expl = __pa6.clone();
        arr_tys = __pa7.clone();
    } else {
        exp_str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (inPosArgs.clone()).into_iter().cloned() {
            let __x = Dump::printExpStr(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone());
        exp_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cat(")); __mm_s.push_str(&*exp_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
        Error::addSourceMessageAndFail(Error::DIFFERENT_DIM_SIZE_IN_ARGUMENTS.clone(), list![(exp_str.clone()).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    dims = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        for mut t in (arr_tys.clone()).into_iter().cloned() {
            let __x = Types::getDimensionNth(t.clone(), dim_int.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    dim = todo!("reduction Expression.dimensionsAdd: cannot resolve default value");
    result_ty = Types::setDimensionNth(result_ty.clone(), dim.clone(), dim_int.clone())?;
    arr_c = elabArrayConst(arr_props.clone())?;
    c = Types::constAnd(dim_c.clone(), arr_c.clone());
    ty = Types::simplifyType(result_ty.clone())?;
    outExp = Expression::makePureBuiltinCall((literal!("cat")).clone(), cons(dim_exp.clone(), arr_expl.clone()), ty.clone());
    outProperties = DAE::Properties::PROP { type_: result_ty.clone(), constFlag: c.clone() };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinIdentity(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut exp_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut msg: Absyn::Msg = Absyn::Msg::NO_MSG;
    let mut sz: i32 = 0;
    let mut dim_size: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dim_exp: Arc<DAE::Exp>;
    let mut check_model: bool = false;
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("identity")).clone(), inInfo.clone())?;
    (outCache, dim_exp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?;
    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (outProperties.clone()) else { bail!("pattern mismatch") };
    ty = __pa0.clone();
    c = __pa1.clone();
    if !(Types::isScalarInteger(ty.clone())) {
        pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
        Error::addSourceMessageAndFail(Error::ARGUMENT_MUST_BE_INTEGER.clone(), list![(literal!("First")).clone(), (literal!("identity")).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if Types::isParameterOrConstant(c.clone()) {
        check_model = Flags::getConfigBool(Flags::CHECK_MODEL.clone())?;
        msg = if (check_model.clone()) {openmodelica_ast::Absyn::Msg::NO_MSG} else {Absyn::Msg::MSG { info: inInfo.clone() }};
        match '__try2: {
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(Ceval::ceval(outCache.clone(), inEnv.clone(), dim_exp.clone(), false, msg.clone(), 0), '__try2)) {
                (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            outCache = __pa3.clone();
            sz = __pa4.clone();
            dim_size = Arc::new(DAE::Dimension::DIM_INTEGER { integer: sz.clone() });
            dim_exp = Arc::new(DAE::Exp::ICONST { integer: sz.clone() });
            Ok::<_, anyhow::Error>((dim_exp.clone(), dim_size.clone(), outCache.clone(), sz.clone()))
        } {
            Ok((__try2_o0, __try2_o1, __try2_o2, __try2_o3)) => {
                dim_exp = __try2_o0;
                dim_size = __try2_o1;
                outCache = __try2_o2;
                sz = __try2_o3;
            }
            Err(_) => {
                if check_model.clone() {
                    dim_size = Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN);
                } else {
                    bail!("fail");
                }
                bail!("try/else: outputs not set in else branch");
            }
        }
    } else {
        dim_size = Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN);
    }
    ty = Types::liftArrayListDims(DAE::T_INTEGER_DEFAULT().clone(), list![dim_size.clone(), dim_size.clone()]);
    exp_ty = Types::simplifyType(ty.clone())?;
    outExp = Expression::makePureBuiltinCall((literal!("identity")).clone(), list![dim_exp.clone()], exp_ty.clone());
    outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    Ok((outCache, outExp, outProperties))
}

fn zeroSizeOverconstrainedOperator(mut inExp: Arc<DAE::Exp>, mut inFExp: Arc<DAE::Exp>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (ExpressionBasics::printExpStr(inFExp.clone())?).clone();
            Error::addSourceMessage(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO_RETURN_FALSE.clone(), list![(s.clone()).clone()], inInfo.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn elabBuiltinIsRoot(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut exp: Arc<DAE::Exp>;
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("Connections.isRoot")).clone(), inInfo.clone())?;
    (outCache, exp, _) = elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, false, false, inPrefix.clone(), inInfo.clone())?;
    outExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Connections")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("isRoot")).clone() }) }), expLst: list![exp.clone()], attr: DAE::callAttrBuiltinBool().clone() });
    outProperties = DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR };
    zeroSizeOverconstrainedOperator(exp.clone(), outExp.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinRooted(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut exp: Arc<DAE::Exp>;
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("rooted")).clone(), inInfo.clone())?;
    (outCache, exp, _) = elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, false, false, inPrefix.clone(), inInfo.clone())?;
    outExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("rooted")).clone() }), expLst: list![exp.clone()], attr: DAE::callAttrBuiltinBool().clone() });
    outProperties = DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR };
    zeroSizeOverconstrainedOperator(exp.clone(), outExp.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinUniqueRootIndices(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inNamedArg.clone(), inPrefix.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: aexp1, tail: Deref @ metamodelica::List::Cons { head: aexp2, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil, pre) => {
            let mut exp1: Arc<DAE::Exp>;
            let mut exp2: Arc<DAE::Exp>;
            let mut exp3: Arc<DAE::Exp>;
            let mut lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dim: i32 = 0;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), aexp1.clone(), false, false, pre.clone(), info.clone())?) {
                (__pa0, __pa2 @ Deref @ DAE::Exp::ARRAY { array: __pa1, .. }, _) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            lst = __pa1.clone();
            exp1 = __pa2.clone();
            dim = (lst.clone().len() as i32);
            (cache, exp2, _) = elabExpInExpression(cache.clone(), env.clone(), aexp2.clone(), false, false, pre.clone(), info.clone())?;
            exp3 = Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() });
            ty = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() })] });
            (cache.clone(), Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Connections")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("uniqueRootIndices")).clone() }) }), expLst: list![exp1.clone(), exp2.clone(), exp3.clone()], attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR })
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: aexp1, tail: Deref @ metamodelica::List::Cons { head: aexp2, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, Deref @ metamodelica::List::Nil, pre) => {
            let mut exp1: Arc<DAE::Exp>;
            let mut exp2: Arc<DAE::Exp>;
            let mut exp3: Arc<DAE::Exp>;
            let mut lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dim: i32 = 0;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), aexp1.clone(), false, false, pre.clone(), info.clone())?) {
                (__pa0, __pa2 @ Deref @ DAE::Exp::ARRAY { array: __pa1, .. }, _) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            lst = __pa1.clone();
            exp1 = __pa2.clone();
            dim = (lst.clone().len() as i32);
            (cache, exp2, _) = elabExpInExpression(cache.clone(), env.clone(), aexp2.clone(), false, false, pre.clone(), info.clone())?;
            (cache, exp3, _) = elabExpInExpression(cache.clone(), env.clone(), aexp2.clone(), false, false, pre.clone(), info.clone())?;
            ty = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() })] });
            (cache.clone(), Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Connections")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("uniqueRootIndices")).clone() }) }), expLst: list![exp1.clone(), exp2.clone(), exp3.clone()], attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR })
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: aexp1, tail: Deref @ metamodelica::List::Cons { head: aexp2, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "message", argValue: _ }, tail: Deref @ metamodelica::List::Nil }, pre) => {
            let mut exp1: Arc<DAE::Exp>;
            let mut exp2: Arc<DAE::Exp>;
            let mut exp3: Arc<DAE::Exp>;
            let mut lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dim: i32 = 0;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), aexp1.clone(), false, false, pre.clone(), info.clone())?) {
                (__pa0, __pa2 @ Deref @ DAE::Exp::ARRAY { array: __pa1, .. }, _) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            lst = __pa1.clone();
            exp1 = __pa2.clone();
            dim = (lst.clone().len() as i32);
            (cache, exp2, _) = elabExpInExpression(cache.clone(), env.clone(), aexp2.clone(), false, false, pre.clone(), info.clone())?;
            (cache, exp3, _) = elabExpInExpression(cache.clone(), env.clone(), aexp2.clone(), false, false, pre.clone(), info.clone())?;
            ty = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() })] });
            (cache.clone(), Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Connections")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("uniqueRootIndices")).clone() }) }), expLst: list![exp1.clone(), exp2.clone(), exp3.clone()], attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinScalar(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut scalar_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut ty_str: ArcStr = arcstr::literal!("");
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("scalar")).clone(), inInfo.clone())?;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?) {
        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    outExp = __pa1.clone();
    ty = __pa2.clone();
    c = __pa3.clone();
    (scalar_ty, dims) = TypesDump::flattenArrayType(ty.clone());
    for mut dim in &*dims.clone() {
        let mut dim = dim.clone();
        if Expression::dimensionKnown(dim.clone()) && Expression::dimensionSize(dim.clone())? != 1 {
            ty_str = (TypesDump::unparseTypeNoAttr(ty.clone())?).clone();
            Error::addSourceMessageAndFail(Error::INVALID_ARRAY_DIM_IN_CONVERSION_OP.clone(), list![(ty_str.clone()).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    if !(dims.clone().is_empty()) {
        outExp = Expression::makePureBuiltinCall((literal!("scalar")).clone(), list![outExp.clone()], scalar_ty.clone());
    }
    (outExp, _) = ExpressionSimplify::simplify1(outExp.clone())?;
    outProperties = DAE::Properties::PROP { type_: scalar_ty.clone(), constFlag: c.clone() };
    Ok((outCache, outExp, outProperties))
}

thread_local! { static __STRING_ARG_MINLENGTH_TLS: Slot = Slot { defaultArg: Arc::new(DAE::FuncArg { name: (literal!("minimumLength")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), slotFilled: false, arg: Some(Arc::new(DAE::Exp::ICONST { integer: 0 })), dims: metamodelica::nil(), idx: 2, evalStatus: SLOT_NOT_EVALUATED.clone() }; }
pub fn STRING_ARG_MINLENGTH() -> Slot { __STRING_ARG_MINLENGTH_TLS.with(|__t| __t.clone()) }

thread_local! { static __STRING_ARG_LEFTJUSTIFIED_TLS: Slot = Slot { defaultArg: Arc::new(DAE::FuncArg { name: (literal!("leftJustified")).clone(), ty: DAE::T_BOOL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), slotFilled: false, arg: Some(Arc::new(DAE::Exp::BCONST { bool: true })), dims: metamodelica::nil(), idx: 3, evalStatus: SLOT_NOT_EVALUATED.clone() }; }
pub fn STRING_ARG_LEFTJUSTIFIED() -> Slot { __STRING_ARG_LEFTJUSTIFIED_TLS.with(|__t| __t.clone()) }

thread_local! { static __STRING_ARG_SIGNIFICANT_DIGITS_TLS: Slot = Slot { defaultArg: Arc::new(DAE::FuncArg { name: (literal!("significantDigits")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), slotFilled: false, arg: Some(Arc::new(DAE::Exp::ICONST { integer: 6 })), dims: metamodelica::nil(), idx: 4, evalStatus: SLOT_NOT_EVALUATED.clone() }; }
pub fn STRING_ARG_SIGNIFICANT_DIGITS() -> Slot { __STRING_ARG_SIGNIFICANT_DIGITS_TLS.with(|__t| __t.clone()) }

fn elabBuiltinString(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut exp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut consts: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut val_slot: Slot = <Slot as ::std::default::Default>::default();
    let mut format_arg: Option<Arc<DAE::Exp>> = None;
    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    match '__try0: {
        e = Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("String")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: inPosArgs.clone(), argNames: inNamedArgs.clone() }), typeVars: metamodelica::nil() });
        (outCache, outExp, outProperties) = unwrap_break_err!(OperatorOverloading::string(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone()), '__try0);
        Ok::<_, anyhow::Error>((e.clone(), outCache.clone(), outExp.clone(), outProperties.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            e = __try0_o0;
            outCache = __try0_o1;
            outExp = __try0_o2;
            outProperties = __try0_o3;
        }
        Err(_) => {
            e = listHead(inPosArgs.clone())?;
            let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?) {
                (__pa1, __pa2, DAE::Properties::PROP { type_: __pa3, constFlag: __pa4 }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            outCache = __pa1.clone();
            exp = __pa2.clone();
            ty = __pa3.clone();
            c = __pa4.clone();
            if Types::isMetaBoxedType(ty.clone()) {
                ty = Types::unboxedType(ty.clone())?;
                exp = Arc::new(DAE::Exp::UNBOX { exp: exp.clone(), ty: ty.clone() });
            }
            val_slot = Slot { defaultArg: Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: ty.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), slotFilled: false, arg: None, dims: metamodelica::nil(), idx: 1, evalStatus: SLOT_NOT_EVALUATED.clone() };
            match '__try5: {
                slots = list![STRING_ARG_MINLENGTH().clone(), STRING_ARG_LEFTJUSTIFIED().clone()];
                if unwrap_break_err!(Types::isRealOrSubTypeReal(ty.clone()), '__try5) {
                    slots = cons(STRING_ARG_SIGNIFICANT_DIGITS().clone(), slots.clone());
                }
                slots = cons(val_slot.clone(), slots.clone());
                (outCache, args, _, consts, _) = unwrap_break_err!(elabInputArgs(outCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), slots.clone(), false, true, inImplicit.clone(), inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("String")).clone() }), false), '__try5);
                Ok::<_, anyhow::Error>((args.clone(), consts.clone(), outCache.clone(), slots.clone()))
            } {
                Ok((__try5_o0, __try5_o1, __try5_o2, __try5_o3)) => {
                    args = __try5_o0;
                    consts = __try5_o1;
                    outCache = __try5_o2;
                    slots = __try5_o3;
                }
                Err(_) => {
                    if Types::isRealOrSubTypeReal(ty.clone())? {
                        format_arg = Some(Arc::new(DAE::Exp::SCONST { string: (literal!("f")).clone() }));
                    } else if Types::isIntegerOrSubTypeInteger(ty.clone())? {
                        format_arg = Some(Arc::new(DAE::Exp::SCONST { string: (literal!("d")).clone() }));
                    } else if Types::isString(ty.clone()) {
                        format_arg = Some(Arc::new(DAE::Exp::SCONST { string: (literal!("s")).clone() }));
                    } else {
                        format_arg = None;
                    }
                    if isSome(format_arg.clone()) {
                        slots = list![val_slot.clone(), Slot { defaultArg: Arc::new(DAE::FuncArg { name: (literal!("format")).clone(), ty: DAE::T_STRING_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), slotFilled: false, arg: format_arg.clone(), dims: metamodelica::nil(), idx: 2, evalStatus: SLOT_NOT_EVALUATED.clone() }];
                    } else {
                        slots = list![val_slot.clone()];
                    }
                    (outCache, args, _, consts, _) = elabInputArgs(outCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), slots.clone(), false, true, inImplicit.clone(), inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("String")).clone() }), false)?;
                }
            }
            c = List::fold(consts.clone(), (std::sync::Arc::new(fnptr!(Types::constAnd, DAE::Const, DAE::Const)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Const, DAE::Const) -> Result<DAE::Const> + 'static>), openmodelica_frontend_types::DAE::Const::C_CONST);
            outExp = Expression::makePureBuiltinCall((literal!("String")).clone(), args.clone(), DAE::T_STRING_DEFAULT().clone());
            outProperties = DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: c.clone() };
        }
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinGetInstanceName(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut name: Arc<Absyn::Path>;
    let mut envName: Arc<Absyn::Path>;
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 0, (literal!("getInstanceName")).clone(), inInfo.clone())?;
    let FCore::CACHE { modelName: __pa0, .. } = (inCache.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    if PrefixUtil::isNoPrefix(inPrefix.clone()) {
        envName = FGraph::getGraphNameNoImplicitScopes(inEnv.clone())?;
        r#str = (if (AbsynUtil::pathEqual(envName.clone(), name.clone())) {AbsynUtil::pathLastIdent(name.clone())?} else {AbsynUtil::pathString(envName.clone(), (literal!(".")).clone(), true, false)?}).clone();
    } else {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathLastIdent(name.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(inPrefix.clone())?); ArcStr::from(__mm_s) }).clone();
    }
    outExp = Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() });
    outProperties = DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinIsPresent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut direction: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("isPresent")).clone(), info.clone())?;
    if !(FGraph::inFunctionScope(inEnv.clone())?) {
        Error::addSourceMessage(Error::IS_PRESENT_WRONG_SCOPE.clone(), list![(SCodeDump::restrString(FGraph::getScopeRestriction(FGraph::currentScope(inEnv.clone())?)?)?).clone()], info.clone())?;
    }
    outExp = (::match_deref::match_deref! { match &((inPosArgs.clone()).get(1)?) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: r#str, .. } } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupIdentLocal(outCache.clone(), inEnv.clone(), (r#str.clone()).clone())?) {
                (__pa0, Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { direction: __pa1, .. }, .. }, _, _, _, _) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            outCache = __pa0.clone();
            direction = __pa1.clone();
            let () = (match direction.clone() {
        Absyn::Direction::BIDIR => {
            Error::addSourceMessage(Error::IS_PRESENT_WRONG_DIRECTION.clone(), metamodelica::nil(), info.clone())?;
            bail!("fail")
        },
        _ => (),
    });
            Expression::makeImpureBuiltinCall((literal!("isPresent")).clone(), cons(Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() }), metamodelica::nil()), DAE::T_BOOL_DEFAULT().clone())
        },
        exp => {
            Error::addSourceMessage(Error::IS_PRESENT_INVALID_EXP.clone(), list![(Dump::printExpStr(exp.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outProperties = DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinVector(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut arr_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut exp_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut el_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("vector")).clone(), inInfo.clone())?;
    e = listHead(inPosArgs.clone())?;
    let (__pa0, __pa1, __pa4, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?) {
        (__pa0, __pa1, __pa4 @ DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa4.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    outExp = __pa1.clone();
    ty = __pa2.clone();
    c = __pa3.clone();
    outProperties = __pa4.clone();
    if Types::isSimpleType(ty.clone()) {
        arr_ty = Types::liftArray(ty.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 }));
        exp_ty = Types::simplifyType(arr_ty.clone())?;
        outExp = Arc::new(DAE::Exp::ARRAY { ty: exp_ty.clone(), scalar: true, array: list![outExp.clone()] });
        outProperties = DAE::Properties::PROP { type_: arr_ty.clone(), constFlag: c.clone() };
    } else if Expression::isArray(outExp.clone()) || Expression::isMatrix(outExp.clone()) {
        if Types::numberOfDimensions(ty.clone())? != 1 {
            checkBuiltinVectorDims(e.clone(), inEnv.clone(), ty.clone(), inPrefix.clone(), inInfo.clone())?;
            expl = Expression::getArrayOrMatrixContents(outExp.clone())?;
            expl = flattenArray(expl.clone());
            el_ty = Types::arrayElementType(ty.clone());
            arr_ty = Types::liftArray(el_ty.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: (expl.clone().len() as i32) }));
            outExp = Arc::new(DAE::Exp::ARRAY { ty: Types::simplifyType(arr_ty.clone())?, scalar: false, array: expl.clone() });
            outProperties = DAE::Properties::PROP { type_: arr_ty.clone(), constFlag: c.clone() };
        }
    } else {
        ty = Types::liftArray(Types::arrayElementType(ty.clone()), Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN));
        exp_ty = Types::simplifyType(ty.clone())?;
        outExp = Expression::makePureBuiltinCall((literal!("vector")).clone(), list![outExp.clone()], exp_ty.clone());
        outProperties = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    }
    Ok((outCache, outExp, outProperties))
}

fn checkBuiltinVectorDims(mut inExp: Arc<Absyn::Exp>, mut inEnv: FCore::Graph, mut inType: Arc<DAE::Type>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<()> {
    let mut found_dim_sz_one: bool = false;
    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut arg_str: ArcStr = arcstr::literal!("");
    let mut scope_str: ArcStr = arcstr::literal!("");
    let mut dim_str: ArcStr = arcstr::literal!("");
    let mut pre_str: ArcStr = arcstr::literal!("");
    dims = Types::getDimensionSizes(inType.clone())?;
    for mut dim in &*dims.clone() {
        let mut dim = dim.clone();
        if dim.clone() > 1 {
            if found_dim_sz_one.clone() {
                scope_str = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                arg_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("vector(")); __mm_s.push_str(&*Dump::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                dim_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = intString(d.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
                Error::addSourceMessageAndFail(Error::BUILTIN_VECTOR_INVALID_DIMENSIONS.clone(), list![(scope_str.clone()).clone(), (pre_str.clone()).clone(), (dim_str.clone()).clone(), (arg_str.clone()).clone()], inInfo.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            } else {
                found_dim_sz_one = true;
            }
        }
    }
    Ok(())
}

fn flattenArray(mut arr: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut flattenedExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    flattenedExpl = (::match_deref::match_deref! { match &(arr.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl, .. }, tail: rest_expl } => {
            let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut expl = (*expl).clone();
            expl = flattenArray(expl.clone());
            expl2 = flattenArray(rest_expl.clone());
            expl2 = listAppend(expl.clone(), expl2.clone());
            expl2.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, tail: Deref @ metamodelica::List::Nil }, .. }, tail: rest_expl } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expl = flattenArray(rest_expl.clone());
            cons(e.clone(), expl.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: expl } => {
            let mut expl = (*expl).clone();
            expl = flattenArray(expl.clone());
            cons(e.clone(), expl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    flattenedExpl
}

pub fn elabBuiltinMatrix(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImpl: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    checkBuiltinCallArgs(inPosArgs.clone(), inNamedArgs.clone(), 1, (literal!("matrix")).clone(), inInfo.clone())?;
    (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), listHead(inPosArgs.clone())?, inImpl.clone(), true, inPrefix.clone(), inInfo.clone())?;
    ty = Types::getPropType(outProperties.clone())?;
    (outExp, outProperties) = elabBuiltinMatrix2(inCache.clone(), inEnv.clone(), outExp.clone(), outProperties.clone(), ty.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinMatrix2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inArg: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut inType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (::match_deref::match_deref! { match &(inArg.clone()) {
        _ if (Types::isSimpleType(inType.clone())) => {
            let mut exp: Arc<DAE::Exp>;
            let mut props: DAE::Properties;
            (exp, props) = promoteExp(inArg.clone(), inProperties.clone(), 2)?;
            (exp.clone(), props.clone())
        },
        _ if (Types::numberOfDimensions(inType.clone())? == 1) => {
            let mut exp: Arc<DAE::Exp>;
            let mut props: DAE::Properties;
            (exp, props) = promoteExp(inArg.clone(), inProperties.clone(), 2)?;
            (exp.clone(), props.clone())
        },
        Deref @ DAE::Exp::MATRIX { .. } => {
            (inArg.clone(), inProperties.clone())
        },
        Deref @ DAE::Exp::ARRAY { array: expl, scalar, ty: Deref @ DAE::Type::T_ARRAY { ty: ety, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Cons { head: dim2, tail: _ } } } } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut props: DAE::Properties;
            let mut expl = (*expl).clone();
            expl = List::map1(expl.clone(), (std::sync::Arc::new(elabBuiltinMatrix3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, SourceInfo) -> Result<Arc<DAE::Exp>> + 'static>), inInfo.clone());
            ty = Types::arrayElementType(inType.clone());
            ty = Types::liftArrayListDims(ty.clone(), list![dim1.clone(), dim2.clone()]);
            props = Types::setPropType(inProperties.clone(), ty.clone())?;
            (Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: list![dim1.clone(), dim2.clone()] }), scalar: scalar.clone(), array: expl.clone() }), props.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExp, outProperties))
}

fn elabBuiltinMatrix3(mut inExp: Arc<DAE::Exp>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: expl, scalar, ty: Deref @ DAE::Type::T_ARRAY { ty: ety, dims: Deref @ metamodelica::List::Cons { head: dim, tail: _ } } } => {
            let mut expl = (*expl).clone();
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = arrayScalar(e.clone(), 3, (literal!("matrix")).clone(), inInfo.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: list![dim.clone()] }), scalar: scalar.clone(), array: expl.clone() })
        },
        Deref @ DAE::Exp::MATRIX { matrix: matrix_expl, ty: Deref @ DAE::Type::T_ARRAY { ty: ety, dims: Deref @ metamodelica::List::Cons { head: dim, tail: dims } }, .. } => {
            let mut ety2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            ety2 = Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: dims.clone() });
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (matrix_expl.clone()).into_iter().cloned() {
            let __x = Expression::makeArray(e.clone(), ety2.clone(), true);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = arrayScalar(e.clone(), 3, (literal!("matrix")).clone(), inInfo.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: list![dim.clone()] }), scalar: true, array: expl.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn arrayScalar(mut inExp: Arc<DAE::Exp>, mut inDim: i32, mut inOperator: ArcStr, mut inInfo: SourceInfo) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, .. } => {
            arrayScalar(exp.clone(), inDim.clone() + 1, (inOperator.clone()).clone(), inInfo.clone())?
        },
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            let mut dim_str: ArcStr = arcstr::literal!("");
            let mut size_str: ArcStr = arcstr::literal!("");
            dim_str = (intString(inDim.clone())).clone();
            size_str = (intString((expl.clone().len() as i32))).clone();
            Error::addSourceMessage(Error::INVALID_ARRAY_DIM_IN_CONVERSION_OP.clone(), list![(dim_str.clone()).clone(), (inOperator.clone()).clone(), (literal!("1")).clone(), (size_str.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: expl, tail: Deref @ metamodelica::List::Nil }, ty, .. } => {
            arrayScalar(Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: expl.clone() }), inDim.clone() + 1, (inOperator.clone()).clone(), inInfo.clone())?
        },
        Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. } => {
            let mut dim_str: ArcStr = arcstr::literal!("");
            let mut size_str: ArcStr = arcstr::literal!("");
            dim_str = (intString(inDim.clone())).clone();
            size_str = (intString((mexpl.clone().len() as i32))).clone();
            Error::addSourceMessage(Error::INVALID_ARRAY_DIM_IN_CONVERSION_OP.clone(), list![(dim_str.clone()).clone(), (inOperator.clone()).clone(), (literal!("1")).clone(), (size_str.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn elabBuiltinHandler(mut inIdent: ArcStr) -> Result<Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>> {
    pub type HandlerFunc = std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>;

    let mut outHandler: Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>;
    outHandler = (::match_deref::match_deref! { match &(inIdent.clone()) {
        Deref @ "smooth" => (std::sync::Arc::new(elabBuiltinSmooth) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "size" => (std::sync::Arc::new(elabBuiltinSize) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "ndims" => (std::sync::Arc::new(elabBuiltinNDims) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "zeros" => (std::sync::Arc::new(elabBuiltinZeros) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "ones" => (std::sync::Arc::new(elabBuiltinOnes) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "fill" => (std::sync::Arc::new(elabBuiltinFill) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "max" => (std::sync::Arc::new(elabBuiltinMax) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "min" => (std::sync::Arc::new(elabBuiltinMin) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "transpose" => (std::sync::Arc::new(elabBuiltinTranspose) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "symmetric" => (std::sync::Arc::new(elabBuiltinSymmetric) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "array" => (std::sync::Arc::new(elabBuiltinArray) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "sum" => (std::sync::Arc::new(elabBuiltinSum) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "product" => (std::sync::Arc::new(elabBuiltinProduct) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "pre" => (std::sync::Arc::new(elabBuiltinPre) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "firstTick" => (std::sync::Arc::new(elabBuiltinFirstTick) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "interval" => (std::sync::Arc::new(elabBuiltinInterval) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "boolean" => (std::sync::Arc::new(elabBuiltinBoolean) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "noEvent" => (std::sync::Arc::new(elabBuiltinNoevent) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "edge" => (std::sync::Arc::new(elabBuiltinEdge) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "der" => (std::sync::Arc::new(elabBuiltinDer) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "change" => (std::sync::Arc::new(elabBuiltinChange) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "cat" => (std::sync::Arc::new(elabBuiltinCat) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "identity" => (std::sync::Arc::new(elabBuiltinIdentity) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "vector" => (std::sync::Arc::new(elabBuiltinVector) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "matrix" => (std::sync::Arc::new(elabBuiltinMatrix) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "scalar" => (std::sync::Arc::new(elabBuiltinScalar) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "String" => (std::sync::Arc::new(elabBuiltinString) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "rooted" => (std::sync::Arc::new(elabBuiltinRooted) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "Integer" => (std::sync::Arc::new(elabBuiltinIntegerEnum) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "EnumToInteger" => (std::sync::Arc::new(elabBuiltinIntegerEnum) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "inStream" => (std::sync::Arc::new(elabBuiltinInStream) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "actualStream" => (std::sync::Arc::new(elabBuiltinActualStream) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "getInstanceName" => (std::sync::Arc::new(elabBuiltinGetInstanceName) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "classDirectory" => (std::sync::Arc::new(elabBuiltinClassDirectory) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "sample" => (std::sync::Arc::new(elabBuiltinSample) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "cardinality" => (std::sync::Arc::new(elabBuiltinCardinality) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "homotopy" => (std::sync::Arc::new(elabBuiltinHomotopy) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "DynamicSelect" => (std::sync::Arc::new(elabBuiltinDynamicSelect) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinClock) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "hold" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinHold) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "shiftSample" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinShiftSample) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "backSample" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinBackSample) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "noClock" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinNoClock) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "transition" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinTransition) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "initialState" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinInitialState) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "activeState" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinActiveState) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "ticksInState" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinTicksInState) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "timeInState" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinTimeInState) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "sourceInfo" => {
            let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinSourceInfo) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "SOME" => {
            let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinSome) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "NONE" => {
            let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinNone) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "isPresent" => {
            let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinIsPresent) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outHandler)
}

pub fn isBuiltinFunc(mut inPath: Arc<Absyn::Path>, mut ty: Arc<DAE::Type>) -> Result<(DAE::FunctionBuiltin, bool, Arc<Absyn::Path>)> {
    let mut isBuiltin: DAE::FunctionBuiltin = DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR;
    let mut b: bool = false;
    let mut outPath: Arc<Absyn::Path>;
    (isBuiltin, b, outPath) = 'mc: {
        let __mc_input = (inPath.clone(), ty.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: isBuiltin @ DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: _, .. }, .. }, .. }) => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeNotFullyQualified(path.clone());
                    Ok((isBuiltin.clone(), true, path.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: isBuiltin @ DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR, .. }, .. }) => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeNotFullyQualified(path.clone());
                    Ok((isBuiltin.clone(), false, path.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: id }, _) => {
                    elabBuiltinHandler((id.clone()).clone())?;
                    Ok((DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some((id.clone()).clone()), unboxArgs: false }, true, inPath.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _) => {
                    let mut path = (*path).clone();
                    let mut isBuiltin: DAE::FunctionBuiltin = isBuiltin.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(isBuiltinFunc(path.clone(), ty.clone())?) {
                        (__pa0 @ DAE::FunctionBuiltin::FUNCTION_BUILTIN { .. }, _, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    isBuiltin = __pa0.clone();
                    path = __pa1.clone();
                    Ok((isBuiltin.clone(), true, path.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, _) => {
                    Ok((DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: None, unboxArgs: false }, true, inPath.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, false, inPath.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((isBuiltin, b, outPath))
}

fn elabCallBuiltin(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFnName: Arc<Absyn::ComponentRef>, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    pub type HandlerFunc = std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>;

    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &(inFnName.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, .. } => {
            let mut handler: Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>;
            handler = elabBuiltinHandler((var_field!((*inFnName).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone())?;
            handler(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "isRoot", .. }, name: Deref @ "Connections", .. } => {
            elabBuiltinIsRoot(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "uniqueRootIndices", .. }, name: Deref @ "Connections", .. } => {
            Error::addSourceMessage(Error::NON_STANDARD_OPERATOR.clone(), list![(literal!("Connections.uniqueRootIndices")).clone()], inInfo.clone())?;
            elabBuiltinUniqueRootIndices(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rooted", .. }, name: Deref @ "Connections", .. } => {
            elabBuiltinRooted(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
            elabCallBuiltin(inCache.clone(), inEnv.clone(), cr.clone(), inPosArgs.clone(), inNamedArgs.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabCall(mut cache: FCore::Cache, mut env: FCore::Graph, mut r#fn: Arc<Absyn::ComponentRef>, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut r#impl: bool, mut pre: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut cache: FCore::Cache = cache;
    let mut e: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut numErrorMessages: i32 = Error::getNumErrorMessages();
    let mut handles: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut s: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    if hasBuiltInHandler(r#fn.clone())? {
        if '__try0: {
            (cache, e, prop) = unwrap_break_err!(elabCallBuiltin(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info.clone()), '__try0);
            return Ok((cache.clone(), e.clone(), prop.clone()));
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            let true = (numErrorMessages.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
            name = (Dump::printComponentRefStr(r#fn.clone())?).clone();
            s1 = stringDelimitList(List::map(args.clone(), (std::sync::Arc::new(Dump::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone());
            s2 = stringDelimitList(List::map(nargs.clone(), (std::sync::Arc::new(Dump::printNamedArgStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::NamedArg>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone());
            s = (if (s2.clone() == literal!("")) {s1.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }}).clone();
            s = stringAppendList(list![(name.clone()).clone(), (literal!("(")).clone(), (s.clone()).clone(), (literal!(").\n")).clone()]);
            Error::addSourceMessage(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(s.clone()).clone(), (PrefixUtil::printPrefixStr3(pre.clone())?).clone()], info.clone())?;
            bail!("fail");
        }
    }
    handles = metamodelica::nil();
    (cache, e, prop) = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut fn_1: Arc<Absyn::Path>;
            let mut prop: DAE::Properties = prop.clone();
            let mut e: Arc<DAE::Exp> = e.clone();
            let mut cache: FCore::Cache = cache.clone();
            ErrorExt::setCheckpoint((literal!("elabCall_InteractiveFunction")).clone());
            fn_1 = AbsynUtil::crefToPath(r#fn.clone())?;
            (cache, e, prop) = elabCallArgs(cache.clone(), env.clone(), fn_1.clone(), args.clone(), nargs.clone(), typeVars.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            ErrorExt::delCheckpoint((literal!("elabCall_InteractiveFunction")).clone());
            Ok((cache.clone(), e.clone(), prop.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut fnstr: ArcStr = arcstr::literal!("");
            let mut argstr: ArcStr = arcstr::literal!("");
            let mut prestr: ArcStr = arcstr::literal!("");
            let mut argstrs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- Static.elabCall failed\n")).clone())?;
            Debug::trace((literal!(" function: ")).clone())?;
            fnstr = (Dump::printComponentRefStr(r#fn.clone())?).clone();
            Debug::trace((fnstr.clone()).clone())?;
            Debug::trace((literal!("   posargs: ")).clone())?;
            argstrs = List::map(args.clone(), (std::sync::Arc::new(Dump::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<ArcStr> + 'static>));
            argstr = stringDelimitList(argstrs.clone(), (literal!(", ")).clone());
            Debug::traceln((argstr.clone()).clone())?;
            Debug::trace((literal!(" prefix: ")).clone())?;
            prestr = (PrefixUtil::printPrefixStr(pre.clone())?).clone();
            Debug::traceln((prestr.clone()).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut prop: DAE::Properties = prop.clone();
            let mut cache: FCore::Cache = cache.clone();
            let mut e: Arc<DAE::Exp> = e.clone();
            (cache, e, prop) = BackendCevalInterface::elabCallInteractive(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info.clone());
            Ok((cache.clone(), e.clone(), prop.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, e, prop))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasBuiltInHandler(mut r#fn: Arc<Absyn::ComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = r#fn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name } => {
                    elabBuiltinHandler((name.clone()).clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "isRoot", .. }, name: Deref @ "Connections", .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "uniqueRootIndices", .. }, name: Deref @ "Connections", .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rooted", .. }, name: Deref @ "Connections", .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
                    Ok(hasBuiltInHandler(cr.clone())?)
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
    Ok(b)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isValidDerVariableName(mut exp: Arc<Absyn::Exp>, mut nested: bool) -> bool {
    let mut isValid: bool = false;
    let mut arg: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    isValid = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => nested.clone(),
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "der", .. }, .. } => isValidDerVariableName(arg.clone(), true),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isValid
}

pub fn elabVariablenames(mut inExpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (inExpl.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: var_field!((*e).componentRef, Absyn::Exp::CREF).clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }),
        Deref @ Absyn::Exp::CALL { .. } if (isValidDerVariableName(e.clone(), false)) => Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_EXPRESSION { exp: e.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpl)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getOptionalNamedArgExpList(mut name: ArcStr, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut out: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    out = 'mc: {
        let __mc_input = nargs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argValue: Deref @ Absyn::Exp::ARRAY { arrayExp: absynExpList }, argName }, tail: _ } => {
                    let true = (stringEq((name.clone()).clone(), (argName.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(absynExpListToDaeExpList(absynExpList.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getOptionalNamedArgExpList((name.clone()).clone(), rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn absynExpListToDaeExpList(mut absynExpList: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut out: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &(absynExpList.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: absynCr }, tail: absynRest } => {
            let mut daeExpList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut absynPath: Arc<Absyn::Path>;
            let mut daeCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut crefExp: Arc<DAE::Exp>;
            absynPath = AbsynUtil::crefToPath(absynCr.clone())?;
            daeCr = ComponentReference::pathToCref(absynPath.clone())?;
            crefExp = Expression::crefExp(daeCr.clone())?;
            daeExpList = absynExpListToDaeExpList(absynRest.clone())?;
            cons(crefExp.clone(), daeExpList.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: absynRest } => {
            absynExpListToDaeExpList(absynRest.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

pub fn getOptionalNamedArg(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inImplicit: bool, mut inArgName: ArcStr, mut inType: Arc<DAE::Type>, mut inArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inDefaultExp: Arc<DAE::Exp>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp> = inDefaultExp.clone();
    let mut name: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    for mut arg in &*inArgs.clone() {
        let mut arg = arg.clone();
        let __pa0 = ::match_deref::match_deref! { match &(arg.clone()) {
            Deref @ Absyn::NamedArg { argName: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa0.clone();
        if name.clone() == inArgName.clone() {
            if '__try1: {
                let __pa2 = ::match_deref::match_deref! { match &(arg.clone()) {
                    Deref @ Absyn::NamedArg { argValue: __pa2, .. } => __pa2.clone(),
                    _ => break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                e = __pa2.clone();
                let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(unwrap_break_err!(elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone()), '__try1)) {
                    (__pa3, __pa4, DAE::Properties::PROP { type_: __pa5, .. }) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                    _ => break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                outCache = __pa3.clone();
                outExp = __pa4.clone();
                ty = __pa5.clone();
                (outExp, _) = unwrap_break_err!(Types::matchType(outExp.clone(), ty.clone(), inType.clone(), true), '__try1);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
            break;
        }
    }
    Ok((outCache, outExp))
}

pub fn elabUntypedCref(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCref: Arc<Absyn::ComponentRef>, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            (outCache, subs, _) = elabSubscripts(inCache.clone(), inEnv.clone(), var_field!((*inCref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
            ComponentReferenceBasics::makeCrefIdent((var_field!((*inCref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), subs.clone())
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (outCache, subs, _) = elabSubscripts(inCache.clone(), inEnv.clone(), var_field!((*inCref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
            (outCache, cr) = elabUntypedCref(outCache.clone(), inEnv.clone(), var_field!((*inCref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone())?;
            ComponentReferenceBasics::makeCrefQual((var_field!((*inCref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), subs.clone(), cr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outCref))
}

pub fn needToRebuild(mut newFile: ArcStr, mut oldFile: ArcStr, mut buildTime: metamodelica::Real) -> Result<bool> {
    let mut buildNeeded: bool = false;
    buildNeeded = 'mc: {
        let __mc_input = (newFile.clone(), oldFile.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", Deref @ "") => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (newf, oldf) => {
                    let mut nfmt: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let true = (stringEq((newf.clone()).clone(), (oldf.clone()).clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(System::getFileModificationTime((newf.clone()).clone())) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    nfmt = __pa0.clone();
                    let true = (realGt(buildTime.clone(), nfmt.clone())) else { bail!("pattern mismatch") };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(buildNeeded)
}

fn createDummyFarg(mut name: ArcStr) -> Arc<DAE::FuncArg> {
    let mut farg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    farg = Arc::new(DAE::FuncArg { name: (name.clone()).clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None });
    farg
}

pub fn elabCallArgs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs2(inCache.clone(), inEnv.clone(), inPath.clone(), inAbsynExpLst.clone(), inAbsynNamedArgLst.clone(), typeVars.clone(), inBoolean.clone(), Mutable::create(false), inPrefix.clone(), info.clone(), Error::getNumErrorMessages())?) {
        (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    outExp = __pa1.clone();
    outProperties = __pa2.clone();
    (outCache, outProperties) = elabCallArgsEvaluateArrayLength(outCache.clone(), inEnv.clone(), outProperties.clone(), inPrefix.clone(), info.clone());
    Ok((outCache, outExp, outProperties))
}

fn elabCallArgsEvaluateArrayLength(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inProperties: DAE::Properties, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> (FCore::Cache, DAE::Properties) {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outProperties: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    match '__try0: {
        let true = (unwrap_break_err!(FGraph::checkScopeType(list![FGraph::lastScopeRef(env.clone()).unwrap()], Some(crate::FCore::ScopeType::CLASS_SCOPE)), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        ty = unwrap_break_err!(Types::getPropType(inProperties.clone()), '__try0);
        let (__pa1, (__pa2, _)) = unwrap_break_err!(Types::traverseType(ty.clone(), (inCache.clone(), env.clone()), (std::sync::Arc::new(elabCallArgsEvaluateArrayLength2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, (FCore::Cache, FCore::Graph)) -> Result<(Arc<DAE::Type>, (FCore::Cache, FCore::Graph))> + 'static>)), '__try0);
        ty = __pa1.clone();
        outCache = __pa2.clone();
        outProperties = unwrap_break_err!(Types::setPropType(inProperties.clone(), ty.clone()), '__try0);
        Ok::<_, anyhow::Error>((outCache.clone(), outProperties.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outCache = __try0_o0;
            outProperties = __try0_o1;
        }
        Err(_) => {
            outCache = inCache.clone();
            outProperties = inProperties.clone();
        }
    }
    (outCache, outProperties)
}

fn elabCallArgsEvaluateArrayLength2(mut ty: Arc<DAE::Type>, mut inTpl: (FCore::Cache, FCore::Graph)) -> Result<(Arc<DAE::Type>, (FCore::Cache, FCore::Graph))> {
    let mut oty: Arc<DAE::Type> = ty.clone();
    let mut outTpl: (FCore::Cache, FCore::Graph);
    (oty, outTpl) = 'mc: {
        let __mc_input = (oty.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { .. }, tpl) => {
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut tpl = (*tpl).clone();
                    let mut oty: Arc<DAE::Type> = oty.clone();
                    (dims, tpl) = List::mapFold(var_field!((*oty).dims, DAE::Type::T_ARRAY).clone(), (std::sync::Arc::new(elabCallArgsEvaluateArrayLength3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, (FCore::Cache, FCore::Graph)) -> Result<(Arc<DAE::Dimension>, (FCore::Cache, FCore::Graph))> + 'static>), tpl.clone());
                    assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = dims.clone());
                    Ok((oty.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((oty.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oty, outTpl))
}

fn elabCallArgsEvaluateArrayLength3(mut inDim: Arc<DAE::Dimension>, mut inTpl: (FCore::Cache, FCore::Graph)) -> Result<(Arc<DAE::Dimension>, (FCore::Cache, FCore::Graph))> {
    let mut outDim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut outTpl: (FCore::Cache, FCore::Graph);
    (outDim, outTpl) = 'mc: {
        let __mc_input = (inDim.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_EXP { exp }, (cache, env)) => {
                    let mut i: i32 = 0;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(cache.clone(), env.clone(), exp.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    i = __pa1.clone();
                    Ok((Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() }), (cache.clone(), env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inDim.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDim, outTpl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn createInputVariableReplacements(mut inSlotLst: Arc<metamodelica::List<Slot>>, mut inVarsRepl: VarTransform::VariableReplacements) -> Result<VarTransform::VariableReplacements> {
    let mut outVarsRepl: VarTransform::VariableReplacements = <VarTransform::VariableReplacements as ::std::default::Default>::default();
    outVarsRepl = 'mc: {
        let __mc_input = inSlotLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inVarsRepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Slot { arg: Some(e), slotFilled: true, defaultArg: Deref @ DAE::FuncArg { name: id, .. }, .. }, tail: rest } => {
                    let mut o: VarTransform::VariableReplacements = <VarTransform::VariableReplacements as ::std::default::Default>::default();
                    o = VarTransform::addReplacement(inVarsRepl.clone(), ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()), e.clone())?;
                    Ok(createInputVariableReplacements(rest.clone(), o.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(createInputVariableReplacements(listRest(inSlotLst.clone())?, inVarsRepl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarsRepl)
}

fn elabCallArgs2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inBoolean: bool, mut stopElab: Mutable::Mutable<bool>, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut numErrors: i32) -> Result<(FCore::Cache, Option<(Arc<DAE::Exp>, DAE::Properties)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut expProps: Option<(Arc<DAE::Exp>, DAE::Properties)> = None;
    (outCache, expProps) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPath.clone(), inAbsynExpLst.clone(), inAbsynNamedArgLst.clone(), inBoolean.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut newslots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut newslots2: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut args_2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cl: Arc<SCode::Element>;
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupClassIdent(cache.clone(), env.clone(), (literal!("GraphicalAnnotationsProgram____")).clone(), None)?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    cl = __pa1.clone();
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), r#fn.clone(), None)?) {
                        (__pa2, __pa3 @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, __pa4) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    cl = __pa3.clone();
                    env_1 = __pa4.clone();
                    (cache, cl, env_2) = Lookup::lookupRecordConstructorClass(cache.clone(), env_1.clone(), r#fn.clone())?;
                    let __pa5 = ::match_deref::match_deref! { match &(SCodeUtil::getClassComponents(cl.clone())?) {
                        (_, Deref @ metamodelica::List::Cons { head: _, tail: __pa5 }) => __pa5.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    names = __pa5.clone();
                    fargs = List::map(names.clone(), (std::sync::Arc::new(fnptr!(createDummyFarg, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<DAE::FuncArg>> + 'static>));
                    slots = makeEmptySlots(fargs.clone());
                    (cache, _, newslots, _, _) = elabInputArgs(cache.clone(), env.clone(), args.clone(), nargs.clone(), slots.clone(), true, false, r#impl.clone(), pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), r#fn.clone(), true)?;
                    (cache, newslots2, _, _) = fillGraphicsDefaultSlots(cache.clone(), newslots.clone(), cl.clone(), env_2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    args_2 = slotListArgs(newslots2.clone());
                    tp = complexTypeFromSlots(newslots2.clone(), ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) })?;
                    Ok((cache.clone(), Some((Arc::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: args_2.clone(), attr: Arc::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), DAE::Properties::PROP { type_: DAE::T_UNKNOWN_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut outtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tp1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut newslots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut newslots2: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut args_2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut constlist: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
                    let mut constInputArgs: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut tyconst: Arc<DAE::TupleConst>;
                    let mut prop: DAE::Properties;
                    let mut prop_1: DAE::Properties;
                    let mut path: Arc<Absyn::Path>;
                    let mut vect_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut call_exp: Arc<DAE::Exp>;
                    let mut callExp: Arc<DAE::Exp>;
                    let mut func: DAE::Function;
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(Arc<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    ErrorExt::setCheckpoint((literal!("RecordConstructor")).clone());
                    (cache, func) = InstFunction::getRecordConstructorFunction(cache.clone(), env.clone(), r#fn.clone())?;
                    let DAE::RECORD_CONSTRUCTOR { path: __pa0, type_: __pa1, source: _ } = (func.clone()) else { bail!("pattern mismatch") };
                    path = __pa0.clone();
                    tp1 = __pa1.clone();
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(tp1.clone()) {
                        Deref @ DAE::Type::T_FUNCTION { funcArg: __pa2, funcResultType: __pa3, functionAttributes: _, path: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    fargs = __pa2.clone();
                    outtype = __pa3.clone();
                    path = __pa4.clone();
                    slots = makeEmptySlots(fargs.clone());
                    (cache, _, newslots, constInputArgs, _) = elabInputArgs(cache.clone(), env.clone(), args.clone(), nargs.clone(), slots.clone(), true, true, r#impl.clone(), pre.clone(), info.clone(), tp1.clone(), path.clone(), false)?;
                    (args_2, newslots2) = addDefaultArgs(newslots.clone(), info.clone());
                    vect_dims = slotsVectorizable(newslots2.clone(), info.clone())?;
                    constlist = constInputArgs.clone();
                    r#const = List::fold(constlist.clone(), (std::sync::Arc::new(fnptr!(Types::constAnd, DAE::Const, DAE::Const)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Const, DAE::Const) -> Result<DAE::Const> + 'static>), openmodelica_frontend_types::DAE::Const::C_CONST);
                    tyconst = elabConsts(outtype.clone(), r#const.clone())?;
                    prop = getProperties(outtype.clone(), tyconst.clone())?;
                    callExp = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: args_2.clone(), attr: Arc::new(DAE::CallAttributes { ty: outtype.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
                    (call_exp, prop_1) = vectorizeCall(callExp.clone(), vect_dims.clone(), newslots2.clone(), prop.clone(), info.clone())?;
                    expProps = Some((call_exp.clone(), prop_1.clone()));
                    Mutable::update(stopElab.clone(), true);
                    ErrorExt::rollBack((literal!("RecordConstructor")).clone());
                    Ok((cache.clone(), expProps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut recordEnv: FCore::Graph;
                    let mut recordCl: Arc<SCode::Element>;
                    let mut fn_1: Arc<Absyn::Path>;
                    let mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut operNames: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(Arc<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    let false = (Mutable::access(stopElab.clone())) else { bail!("pattern mismatch") };
                    (cache, recordCl, recordEnv) = Lookup::lookupClass(cache.clone(), env.clone(), r#fn.clone(), None)?;
                    let true = (SCodeUtil::isOperatorRecord(recordCl.clone())) else { bail!("pattern mismatch") };
                    fn_1 = AbsynUtil::joinPaths(r#fn.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("'constructor'")).clone() }))?;
                    (cache, recordCl, recordEnv) = Lookup::lookupClass(cache.clone(), recordEnv.clone(), fn_1.clone(), None)?;
                    let true = (SCodeUtil::isOperator(recordCl.clone())) else { bail!("pattern mismatch") };
                    operNames = AbsynToSCode::getListofQualOperatorFuncsfromOperator(recordCl.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsListInEnv(cache.clone(), recordEnv.clone(), operNames.clone(), info.clone(), metamodelica::nil())?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    typelist = __pa1.clone();
                    Mutable::update(stopElab.clone(), true);
                    (cache, expProps) = elabCallArgs3(cache.clone(), env.clone(), typelist.clone(), fn_1.clone(), args.clone(), nargs.clone(), typeVars.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    ErrorExt::rollBack((literal!("RecordConstructor")).clone());
                    Ok((cache.clone(), expProps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(Arc<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    ErrorExt::delCheckpoint((literal!("RecordConstructor")).clone());
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (Mutable::access(stopElab.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupType(cache.clone(), env.clone(), r#fn.clone(), None)?) {
                        (__pa0, __pa1 @ Deref @ DAE::Type::T_METARECORD { .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    t = __pa1.clone();
                    Mutable::update(stopElab.clone(), true);
                    (cache, expProps) = elabCallArgsMetarecord(cache.clone(), env.clone(), t.clone(), args.clone(), nargs.clone(), r#impl.clone(), stopElab.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), expProps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(Arc<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    ErrorExt::setCheckpoint((literal!("elabCallArgs2FunctionLookup")).clone());
                    let false = (Mutable::access(stopElab.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(cache.clone(), env.clone(), r#fn.clone(), info.clone())?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    typelist = __pa1.clone();
                    Mutable::update(stopElab.clone(), true);
                    (cache, expProps) = elabCallArgs3(cache.clone(), env.clone(), typelist.clone(), r#fn.clone(), args.clone(), nargs.clone(), typeVars.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    ErrorExt::delCheckpoint((literal!("elabCallArgs2FunctionLookup")).clone());
                    Ok((cache.clone(), expProps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut functype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tp1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut args_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    let mut types_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    let mut argStr: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(cache.clone(), env.clone(), r#fn.clone(), info.clone())?) {
                        (__pa0, __pa2 @ Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    tp1 = __pa1.clone();
                    typelist = __pa2.clone();
                    (cache, args_1, _, _, functype, _, _) = elabTypes(cache.clone(), env.clone(), args.clone(), nargs.clone(), metamodelica::nil(), typelist.clone(), true, false, r#impl.clone(), pre.clone(), info.clone())?;
                    argStr = (ExpressionDump::printExpListStr(args_1.clone())).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    fn_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(r#fn.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*argStr.clone()); __mm_s.push_str(&*literal!(")\nof type\n  ")); __mm_s.push_str(&*TypesDump::unparseType(functype.clone())?); ArcStr::from(__mm_s) }).clone();
                    types_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*TypesDump::unparseType(tp1.clone())?); ArcStr::from(__mm_s) }).clone();
                    Error::assertionOrAddSourceMessage(Error::getNumErrorMessages() != numErrors.clone(), Error::NO_MATCHING_FUNCTION_FOUND.clone(), list![(fn_str.clone()).clone(), (pre_str.clone()).clone(), (types_str.clone()).clone()], info.clone())?;
                    ErrorExt::delCheckpoint((literal!("elabCallArgs2FunctionLookup")).clone());
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, _, _, _, _) => {
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut re: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), r#fn.clone(), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { restriction: __pa1, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    re = __pa1.clone();
                    let false = (SCodeUtil::isFunctionRestriction(re.clone())) else { bail!("pattern mismatch") };
                    fn_str = (AbsynUtil::pathString(r#fn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    s = (SCodeDump::restrString(re.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_FUNCTION_GOT_CLASS.clone(), list![(fn_str.clone()).clone(), (s.clone()).clone()], info.clone())?;
                    ErrorExt::delCheckpoint((literal!("elabCallArgs2FunctionLookup")).clone());
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, _, _, _, pre) => {
                    let mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut t_lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    let mut types_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(cache.clone(), env.clone(), r#fn.clone(), info.clone())?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    typelist = __pa1.clone();
                    t_lst = List::map(typelist.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>));
                    fn_str = (AbsynUtil::pathString(r#fn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    types_str = stringDelimitList(t_lst.clone(), (literal!("\n -")).clone());
                    Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND.clone(), list![(fn_str.clone()).clone(), (pre_str.clone()).clone(), (types_str.clone()).clone()], info.clone())?;
                    ErrorExt::delCheckpoint((literal!("elabCallArgs2FunctionLookup")).clone());
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: _ } }, tail: Deref @ metamodelica::List::Nil }, _, r#impl, pre) => {
                    if !((Config::acceptOptimicaGrammar()?)) { bail!("guard") }
                    let mut prop: DAE::Properties;
                    let mut daeexp: Arc<DAE::Exp>;
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut daecref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(Arc<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    cref = AbsynUtil::pathToCref(r#fn.clone())?;
                    let (__pa0, __pa3, __pa1, __pa2, __pa4) = ::match_deref::match_deref! { match &(elabCref(cache.clone(), env.clone(), cref.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa3 @ Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: __pa2 }, __pa4, _))) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    daecref = __pa1.clone();
                    tp = __pa2.clone();
                    daeexp = __pa3.clone();
                    prop = __pa4.clone();
                    ErrorExt::rollBack((literal!("elabCallArgs2FunctionLookup")).clone());
                    daeexp = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { componentRef: daecref.clone(), instant: (name.clone()).clone() }), ty: tp.clone() });
                    expProps = Some((daeexp.clone(), prop.clone()));
                    Ok((cache.clone(), expProps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, _, _, _, _) => {
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    let mut scope: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupType(cache.clone(), env.clone(), r#fn.clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    scope = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); __mm_s.push_str(&*literal!(" (looking for a function or record)")); ArcStr::from(__mm_s) }).clone();
                    fn_str = (AbsynUtil::pathString(r#fn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::LOOKUP_ERROR.clone(), list![(fn_str.clone()).clone(), (scope.clone()).clone()], info.clone())?;
                    ErrorExt::delCheckpoint((literal!("elabCallArgs2FunctionLookup")).clone());
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, _, _, _, pre) => {
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(cache.clone(), env.clone(), r#fn.clone(), info.clone())?) {
                        (__pa0, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    fn_str = (AbsynUtil::pathString(r#fn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    fn_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fn_str.clone()); __mm_s.push_str(&*literal!(" in component ")); __mm_s.push_str(&*pre_str.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NO_CANDIDATE.clone(), list![(fn_str.clone()).clone()], info.clone())?;
                    ErrorExt::delCheckpoint((literal!("elabCallArgs2FunctionLookup")).clone());
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, r#fn, _, _, _, _) => {
                    ErrorExt::delCheckpoint((literal!("elabCallArgs2FunctionLookup")).clone());
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCallArgs failed on: ")); __mm_s.push_str(&*AbsynUtil::pathString(r#fn.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, expProps))
}

pub fn elabCallArgs3(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>>, mut r#fn: Arc<Absyn::Path>, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut r#impl: bool, mut pre: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Option<(Arc<DAE::Exp>, DAE::Properties)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut expProps: Option<(Arc<DAE::Exp>, DAE::Properties)> = None;
    let mut callExp: Arc<DAE::Exp>;
    let mut call_exp: Arc<DAE::Exp>;
    let mut args_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut args_2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut constlist: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    let mut restype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut functype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut isBuiltin: DAE::FunctionBuiltin = DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR;
    let mut funcParal: DAE::FunctionParallelism = DAE::FunctionParallelism::FP_KERNEL_FUNCTION;
    let mut tuple_: bool = false;
    let mut builtin: bool = false;
    let mut isImpure: bool = false;
    let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    let mut fn_1: Arc<Absyn::Path>;
    let mut prop: DAE::Properties;
    let mut prop_1: DAE::Properties;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tyconst: Arc<DAE::TupleConst>;
    let mut vect_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut slots2: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut status: Util::Status = Util::Status::FAILURE;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut didInline: bool = false;
    let mut onlyOneFunction: bool = false;
    let mut isFunctionPointer: bool = false;
    let mut purity: DAE::Purity = DAE::Purity::PURE;
    onlyOneFunction = (typelist.clone().len() as i32) == 1;
    let (__pa0, __pa1, __pa2, __pa3, __pa8, __pa4, __pa5, __pa6, __pa7, __pa9, __pa10) = ::match_deref::match_deref! { match &(elabTypes(inCache.clone(), inEnv.clone(), args.clone(), nargs.clone(), typeVars.clone(), typelist.clone(), onlyOneFunction.clone(), true, r#impl.clone(), pre.clone(), info.clone())?) {
        (__pa0, __pa1, __pa2, __pa3, __pa8 @ Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { functionParallelism: __pa4, isFunctionPointer: __pa5, inline: __pa6, purity: __pa7, .. }, .. }, __pa9, __pa10) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa8.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa9.clone(), __pa10.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa0.clone();
    args_1 = __pa1.clone();
    constlist = __pa2.clone();
    restype = __pa3.clone();
    funcParal = __pa4.clone();
    isFunctionPointer = __pa5.clone();
    inlineType = __pa6.clone();
    purity = __pa7.clone();
    functype = __pa8.clone();
    vect_dims = __pa9.clone();
    slots = __pa10.clone();
    isImpure = purity.clone() == DAE::Purity::IMPURE.clone();
    (fn_1, functype) = deoverloadFuncname(r#fn.clone(), functype.clone(), inEnv.clone());
    tuple_ = Types::isTuple(restype.clone());
    (isBuiltin, builtin, fn_1) = isBuiltinFunc(fn_1.clone(), functype.clone())?;
    inlineType = inlineBuiltin(isBuiltin.clone(), inlineType.clone());
    let true = (isValidWRTParallelScope(r#fn.clone(), builtin.clone(), funcParal.clone(), inEnv.clone(), info.clone())?) else { bail!("pattern mismatch") };
    r#const = List::fold(constlist.clone(), (std::sync::Arc::new(fnptr!(Types::constAnd, DAE::Const, DAE::Const)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Const, DAE::Const) -> Result<DAE::Const> + 'static>), openmodelica_frontend_types::DAE::Const::C_CONST);
    r#const = if (Flags::isSet(Flags::RML.clone())? && !(builtin.clone()) || purity.clone() == DAE::Purity::OM_IMPURE.clone()) {openmodelica_frontend_types::DAE::Const::C_VAR} else {r#const.clone()};
    (cache, r#const) = determineConstSpecialFunc(cache.clone(), inEnv.clone(), r#const.clone(), fn_1.clone())?;
    tyconst = elabConsts(restype.clone(), r#const.clone())?;
    prop = getProperties(restype.clone(), tyconst.clone())?;
    tp = Types::simplifyType(restype.clone())?;
    (args_2, slots2) = addDefaultArgs(slots.clone(), info.clone());
    let true = (List::fold(slots2.clone(), (std::sync::Arc::new(slotAnd) as std::sync::Arc<dyn ::std::ops::Fn(Slot, bool) -> Result<bool> + 'static>), true)) else { bail!("pattern mismatch") };
    callExp = Arc::new(DAE::Exp::CALL { path: fn_1.clone(), expLst: args_2.clone(), attr: Arc::new(DAE::CallAttributes { ty: tp.clone(), tuple_: tuple_.clone(), builtin: builtin.clone(), isImpure: isImpure.clone() || purity.clone() == DAE::Purity::OM_IMPURE.clone(), isFunctionPointerCall: isFunctionPointer.clone(), inlineType: inlineType.clone(), tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
    (call_exp, prop_1) = vectorizeCall(callExp.clone(), vect_dims.clone(), slots2.clone(), prop.clone(), info.clone())?;
    (cache, status) = instantiateDaeFunction(cache.clone(), inEnv.clone(), if (Lookup::isFunctionCallViaComponent(cache.clone(), inEnv.clone(), r#fn.clone())?) {r#fn.clone()} else {fn_1.clone()}, builtin.clone(), None, true)?;
    cache = instantiateImplicitRecordConstructors(cache.clone(), inEnv.clone(), args_1.clone())?;
    functionTree = FCore::getFunctionTree(cache.clone());
    (call_exp, _, didInline, _) = Inline::inlineExp(call_exp.clone(), (Some(functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE, openmodelica_frontend_types::DAE::InlineType::EARLY_INLINE]), DAE::emptyElementSource().clone())?;
    (call_exp, _) = ExpressionSimplify::condsimplify(didInline.clone(), call_exp.clone())?;
    didInline = didInline.clone() && !(Config::acceptMetaModelicaGrammar()?);
    prop_1 = if (didInline.clone()) {Types::setPropType(prop_1.clone(), restype.clone())?} else {prop_1.clone()};
    if !(isImpure.clone()) {
        (cache, call_exp, prop_1) = Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), call_exp.clone(), prop_1.clone(), r#impl.clone(), info.clone())?;
    }
    expProps = if (Util::isSuccess(status.clone())?) {Some((call_exp.clone(), prop_1.clone()))} else {None};
    outCache = cache.clone();
    Ok((outCache, expProps))
}

pub fn inlineBuiltin(mut isBuiltin: DAE::FunctionBuiltin, mut inlineType: DAE::InlineType) -> DAE::InlineType {
    let mut outInlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    outInlineType = (match isBuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR => openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE,
        _ => inlineType.clone(),
    });
    outInlineType
}

fn isValidWRTParallelScope(mut inFn: Arc<Absyn::Path>, mut isBuiltin: bool, mut inFuncParallelism: DAE::FunctionParallelism, mut inEnv: FCore::Graph, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = isValidWRTParallelScope_dispatch(inFn.clone(), isBuiltin.clone(), inFuncParallelism.clone(), FGraph::currentScope(inEnv.clone())?, inInfo.clone())?;
    Ok(isValid)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isValidWRTParallelScope_dispatch(mut inFn: Arc<Absyn::Path>, mut isBuiltin: bool, mut inFuncParallelism: DAE::FunctionParallelism, mut inScope: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = 'mc: {
        let __mc_input = (isBuiltin.clone(), inFuncParallelism.clone(), inScope.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, DAE::FunctionParallelism::FP_NON_PARALLEL, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: r#ref, tail: restScope }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let true = (listMember((scopeName.clone()).clone(), FCore::implicitScopeNames.clone())) else { bail!("pattern mismatch") };
                    let false = (stringEq((scopeName.clone()).clone(), (arcstr::literal!(FCore::parForScopeName)).clone())) else { bail!("pattern mismatch") };
                    Ok(isValidWRTParallelScope_dispatch(inFn.clone(), isBuiltin.clone(), inFuncParallelism.clone(), restScope.clone(), inInfo.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_NON_PARALLEL, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let true = (FGraph::checkScopeType(list![r#ref.clone()], Some(crate::FCore::ScopeType::CLASS_SCOPE))?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_NON_PARALLEL, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let true = (FGraph::checkScopeType(list![r#ref.clone()], Some(crate::FCore::ScopeType::FUNCTION_SCOPE))?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_NON_PARALLEL, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let mut errorString: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let true = (FGraph::checkScopeType(list![r#ref.clone()], Some(crate::FCore::ScopeType::PARALLEL_SCOPE))?) else { bail!("pattern mismatch") };
                    errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Non-Parallel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' can not be called from a parallel scope.")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Here called from :")); __mm_s.push_str(&*scopeName.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Please declare the function as parallel function.")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], inInfo.clone())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_PARALLEL_FUNCTION, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let true = (FGraph::checkScopeType(list![r#ref.clone()], Some(crate::FCore::ScopeType::PARALLEL_SCOPE))?) else { bail!("pattern mismatch") };
                    let false = (stringEqual((scopeName.clone()).clone(), (AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_PARALLEL_FUNCTION, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let mut errorString: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let true = (FGraph::checkScopeType(list![r#ref.clone()], Some(crate::FCore::ScopeType::PARALLEL_SCOPE))?) else { bail!("pattern mismatch") };
                    let true = (stringEqual((scopeName.clone()).clone(), (AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?).clone())) else { bail!("pattern mismatch") };
                    errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' can not call itself. Recurrsion is not allowed for parallel functions currently.")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel functions can only be called from: 'kernel' functions,")); __mm_s.push_str(&*literal!(" OTHER 'parallel' functions (no recurrsion) or from a body of a")); __mm_s.push_str(&*literal!(" 'parfor' loop")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], inInfo.clone())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_PARALLEL_FUNCTION, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let true = (stringEqual((scopeName.clone()).clone(), (arcstr::literal!(FCore::parForScopeName)).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_PARALLEL_FUNCTION, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let mut errorString: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' can not be called from a non parallel scope '")); __mm_s.push_str(&*scopeName.clone()); __mm_s.push_str(&*literal!("'.\n")); __mm_s.push_str(&*literal!("- Parallel functions can only be called from: 'kernel' functions,")); __mm_s.push_str(&*literal!(" other 'parallel' functions (no recurrsion) or from a body of a")); __mm_s.push_str(&*literal!(" 'parfor' loop")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], inInfo.clone())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_KERNEL_FUNCTION, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let mut errorString: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let true = (stringEqual((scopeName.clone()).clone(), (AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?).clone())) else { bail!("pattern mismatch") };
                    errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Kernel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' can not call itself. ")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Recurrsion is not allowed for Kernel functions. ")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], inInfo.clone())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_KERNEL_FUNCTION, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let mut errorString: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let true = (FGraph::checkScopeType(list![r#ref.clone()], Some(crate::FCore::ScopeType::PARALLEL_SCOPE))?) else { bail!("pattern mismatch") };
                    errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Kernel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' can not be called from a parallel scope '")); __mm_s.push_str(&*scopeName.clone()); __mm_s.push_str(&*literal!("'.\n")); __mm_s.push_str(&*literal!("- Kernel functions CAN NOT be called from: 'kernel' functions,")); __mm_s.push_str(&*literal!(" 'parallel' functions or from a body of a")); __mm_s.push_str(&*literal!(" 'parfor' loop")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], inInfo.clone())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_KERNEL_FUNCTION, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let mut errorString: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let true = (stringEqual((scopeName.clone()).clone(), (arcstr::literal!(FCore::parForScopeName)).clone())) else { bail!("pattern mismatch") };
                    errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Kernel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' can not be called from inside parallel for (parfor) loop body.")); __mm_s.push_str(&*literal!("'.\n")); __mm_s.push_str(&*literal!("- Kernel functions CAN NOT be called from: 'kernel' functions,")); __mm_s.push_str(&*literal!(" 'parallel' functions or from a body of a")); __mm_s.push_str(&*literal!(" 'parfor' loop")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], inInfo.clone())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_KERNEL_FUNCTION, Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    scopeName = (FNode::refName(r#ref.clone())?).clone();
                    let false = (stringEqual((scopeName.clone()).clone(), (AbsynUtil::pathString(inFn.clone(), (literal!(".")).clone(), true, false)?).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isValid)
}

fn elabCallArgsMetarecord(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inType: Arc<DAE::Type>, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut stopElab: Mutable::Mutable<bool>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Option<(Arc<DAE::Exp>, DAE::Properties)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut expProps: Option<(Arc<DAE::Exp>, DAE::Properties)> = None;
    (outCache, expProps) = { let mut ty = inType.clone(); 'mc: {
        let __mc_input = ty;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METARECORD { path: fq_path, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(List::find(var_field!((*inType).fields, DAE::Type::T_METARECORD).clone(), (std::sync::Arc::new(fnptr!(Types::varHasMetaRecordType, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>))?) {
                        Deref @ DAE::Var { name: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    fn_str = (AbsynUtil::pathString(fq_path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::METARECORD_CONTAINS_METARECORD_MEMBER.clone(), list![(fn_str.clone()).clone(), (r#str.clone()).clone()], inInfo.clone())?;
                    Ok((inCache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METARECORD { .. } => {
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    let false = ((var_field!((*inType).fields, DAE::Type::T_METARECORD).clone().len() as i32) == (inPosArgs.clone().len() as i32) + (inNamedArgs.clone().len() as i32)) else { bail!("pattern mismatch") };
                    fn_str = (TypesDump::unparseType(inType.clone())?).clone();
                    Error::addSourceMessage(Error::WRONG_NO_OF_ARGS.clone(), list![(fn_str.clone()).clone()], inInfo.clone())?;
                    Ok((inCache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METARECORD { path: fq_path, .. } => {
                    let mut field_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut typeVars: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut const_lst: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut ty_const: Arc<DAE::TupleConst>;
                    let mut prop: DAE::Properties;
                    let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
                    let mut ty: Arc<DAE::Type>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    field_names = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut var in (var_field!((*inType).fields, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
                    let __x = TypesDump::getVarName(var.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut var in (var_field!((*inType).fields, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
                    let __x = Types::getVarType(var.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    fargs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
        for (n, t) in (&(field_names.clone())).into_iter().zip((&(tys.clone())).into_iter()) {
                    let __x = Types::makeDefaultFuncArg((n.clone()).clone(), t.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    slots = makeEmptySlots(fargs.clone());
                    (outCache, _, slots, const_lst, bindings) = elabInputArgs(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), slots.clone(), true, true, inImplicit.clone(), inPrefix.clone(), inInfo.clone(), inType.clone(), var_field!((*inType).utPath, DAE::Type::T_METARECORD).clone(), false)?;
                    r#const = List::fold(const_lst.clone(), (std::sync::Arc::new(fnptr!(Types::constAnd, DAE::Const, DAE::Const)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Const, DAE::Const) -> Result<DAE::Const> + 'static>), openmodelica_frontend_types::DAE::Const::C_CONST);
                    ty_const = elabConsts(inType.clone(), r#const.clone())?;
                    let true = (List::fold(slots.clone(), (std::sync::Arc::new(slotAnd) as std::sync::Arc<dyn ::std::ops::Fn(Slot, bool) -> Result<bool> + 'static>), true)) else { bail!("pattern mismatch") };
                    args = slotListArgs(slots.clone());
                    if !(bindings.clone().is_empty()) {
                        bindings = Types::solvePolymorphicBindings(bindings.clone(), inInfo.clone(), var_field!((*inType).path, DAE::Type::T_METARECORD).clone())?;
                        typeVars = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut tv in (var_field!((*inType).typeVars, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
                    let __x = Types::fixPolymorphicRestype(tv.clone(), bindings.clone(), inInfo.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                        assign_variant_field!(ty => DAE::Type::T_METARECORD; typeVars = typeVars.clone());
                        prop = getProperties(ty.clone(), ty_const.clone())?;
                    } else {
                        prop = getProperties(ty.clone(), ty_const.clone())?;
                    }
                    Ok((outCache.clone(), Some((Arc::new(DAE::Exp::METARECORDCALL { path: fq_path.clone(), args: args.clone(), fieldNames: field_names.clone(), index: var_field!((*inType).index, DAE::Type::T_METARECORD).clone(), typeVars: var_field!((*inType).typeVars, DAE::Type::T_METARECORD).clone() }), prop.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METARECORD { path: fq_path, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut prop: DAE::Properties;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, _, prop) = elabExpInExpression(inCache.clone(), inEnv.clone(), Arc::new(Absyn::Exp::TUPLE { expressions: inPosArgs.clone() }), false, false, inPrefix.clone(), inInfo.clone())?;
                    tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut var in (var_field!((*inType).fields, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
                    let __x = Types::getVarType(var.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to match types:\n    actual:   ")); __mm_s.push_str(&*TypesDump::unparseType(Types::getPropType(prop.clone())?)?); __mm_s.push_str(&*literal!("\n    expected: ")); __mm_s.push_str(&*TypesDump::unparseType(Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }))?); ArcStr::from(__mm_s) }).clone();
                    fn_str = (AbsynUtil::pathString(fq_path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::META_RECORD_FOUND_FAILURE.clone(), list![(fn_str.clone()).clone(), (r#str.clone()).clone()], inInfo.clone())?;
                    Ok((outCache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METARECORD { path: fq_path, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut fn_str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to elaborate arguments ")); __mm_s.push_str(&*Dump::printExpStr(Arc::new(Absyn::Exp::TUPLE { expressions: inPosArgs.clone() }))?); ArcStr::from(__mm_s) }).clone();
                    fn_str = (AbsynUtil::pathString(fq_path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::META_RECORD_FOUND_FAILURE.clone(), list![(fn_str.clone()).clone(), (r#str.clone()).clone()], inInfo.clone())?;
                    Ok((inCache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    } };
    Ok((outCache, expProps))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ForceFunctionInst {
    /// Used when blocking function instantiation to instantiate the function anyway
    FORCE_FUNCTION_INST,
    /// Used when blocking function instantiation to instantiate the function anyway
    NORMAL_FUNCTION_INST,
}
pub use self::ForceFunctionInst::{FORCE_FUNCTION_INST,NORMAL_FUNCTION_INST};

pub fn instantiateDaeFunction(mut inCache: FCore::Cache, mut env: FCore::Graph, mut name: Arc<Absyn::Path>, mut builtin: bool, mut clOpt: Option<Arc<SCode::Element>>, mut printErrorMsg: bool) -> Result<(FCore::Cache, Util::Status)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut status: Util::Status = Util::Status::FAILURE;
    (outCache, status) = instantiateDaeFunction2(inCache.clone(), env.clone(), name.clone(), builtin.clone(), clOpt.clone(), printErrorMsg.clone(), crate::Static::ForceFunctionInst::NORMAL_FUNCTION_INST)?;
    Ok((outCache, status))
}

pub fn instantiateDaeFunctionFromTypes(mut inCache: FCore::Cache, mut env: FCore::Graph, mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut builtin: bool, mut clOpt: Option<Arc<SCode::Element>>, mut printErrorMsg: bool, mut acc: Util::Status) -> Result<(FCore::Cache, Util::Status)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut status: Util::Status = Util::Status::FAILURE;
    (outCache, status) = (::match_deref::match_deref! { match &((tys.clone(), acc.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { path: name, .. }, tail: rest }, Util::Status::SUCCESS) => {
            (outCache, status) = instantiateDaeFunction(inCache.clone(), env.clone(), name.clone(), builtin.clone(), clOpt.clone(), printErrorMsg.clone())?;
            instantiateDaeFunctionFromTypes(inCache.clone(), env.clone(), rest.clone(), builtin.clone(), clOpt.clone(), printErrorMsg.clone(), status.clone())?
        },
        _ => {
            (inCache.clone(), acc.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, status))
}

pub fn instantiateDaeFunctionForceInst(mut inCache: FCore::Cache, mut env: FCore::Graph, mut name: Arc<Absyn::Path>, mut builtin: bool, mut clOpt: Option<Arc<SCode::Element>>, mut printErrorMsg: bool) -> Result<(FCore::Cache, Util::Status)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut status: Util::Status = Util::Status::FAILURE;
    (outCache, status) = instantiateDaeFunction2(inCache.clone(), env.clone(), name.clone(), builtin.clone(), clOpt.clone(), printErrorMsg.clone(), crate::Static::ForceFunctionInst::FORCE_FUNCTION_INST)?;
    Ok((outCache, status))
}

fn instantiateDaeFunction2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inName: Arc<Absyn::Path>, mut builtin: bool, mut clOpt: Option<Arc<SCode::Element>>, mut printErrorMsg: bool, mut forceFunctionInst: ForceFunctionInst) -> Result<(FCore::Cache, Util::Status)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut status: Util::Status = Util::Status::FAILURE;
    let mut numError: i32 = Error::getNumErrorMessages();
    let mut instOnlyForcedFunctions: bool = isSome(openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| __root.borrow().clone()));
    (outCache, status) = 'mc: {
        let __mc_input = (builtin.clone(), clOpt.clone(), instOnlyForcedFunctions.clone(), forceFunctionInst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, true, ForceFunctionInst::NORMAL_FUNCTION_INST { .. }) => {
                    let false = (AbsynUtil::pathIsIdent(inName.clone())) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, _, _) => {
                    Ok((inCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, ForceFunctionInst::NORMAL_FUNCTION_INST { .. }) => {
                    let (_, true) = (isExternalObjectFunction(inCache.clone(), inEnv.clone(), inName.clone())?) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    let false = (FGraph::isTopScope(inEnv.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::pathSuffixOf(inName.clone(), FGraph::getGraphName(inEnv.clone())?)?) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut name: Arc<Absyn::Path>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, _, _, name) = lookupAndFullyQualify(inCache.clone(), inEnv.clone(), inName.clone())?;
                    FCore::checkCachedInstFuncGuard(outCache.clone(), name.clone())?;
                    Ok((outCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    let mut env: FCore::Graph;
                    let mut cl: Arc<SCode::Element>;
                    let mut name: Arc<Absyn::Path>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, env, cl, name) = lookupAndFullyQualify(inCache.clone(), inEnv.clone(), inName.clone())?;
                    outCache = FCore::addCachedInstFuncGuard(outCache.clone(), name.clone())?;
                    (outCache, _, _) = InstFunction::implicitFunctionInstantiation(outCache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil())?;
                    Ok((outCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(cl), _, _) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, _) = Inst::makeFullyQualified(inCache.clone(), inEnv.clone(), inName.clone())?;
                    (outCache, _, _) = InstFunction::implicitFunctionInstantiation(outCache.clone(), inEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil())?;
                    Ok((outCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    cref = ComponentReference::pathToCref(inName.clone())?;
                    (outCache, _, ty, _, _, _, _, _, _) = Lookup::lookupVar(inCache.clone(), inEnv.clone(), cref.clone())?;
                    ::match_deref::match_deref! { match &(ty.clone()) {
                        Deref @ DAE::Type::T_FUNCTION { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((outCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, true, _) => {
                    let mut pathStr: ArcStr = arcstr::literal!("");
                    let mut envStr: ArcStr = arcstr::literal!("");
                    let true = (Error::getNumErrorMessages() == numError.clone()) else { bail!("pattern mismatch") };
                    envStr = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                    pathStr = (AbsynUtil::pathString(inName.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::GENERIC_INST_FUNCTION.clone(), list![(pathStr.clone()).clone(), (envStr.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), openmodelica_util::Util::Status::FAILURE))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, status))
}

fn lookupAndFullyQualify(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFunctionName: Arc<Absyn::Path>) -> Result<(FCore::Cache, FCore::Graph, Arc<SCode::Element>, Arc<Absyn::Path>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outClass: Arc<SCode::Element>;
    let mut outFunctionName: Arc<Absyn::Path>;
    if Lookup::isFunctionCallViaComponent(inCache.clone(), inEnv.clone(), inFunctionName.clone())? {
        (_, outClass, outEnv) = Lookup::lookupClass(inCache.clone(), inEnv.clone(), inFunctionName.clone(), None)?;
        outFunctionName = FGraph::joinScopePath(outEnv.clone(), AbsynUtil::makeIdentPathFromString((SCodeUtil::elementName(outClass.clone())?).clone()))?;
        outCache = inCache.clone();
    } else {
        (outCache, outClass, outEnv) = Lookup::lookupClass(inCache.clone(), inEnv.clone(), inFunctionName.clone(), None)?;
        outFunctionName = AbsynUtil::makeFullyQualified(FGraph::joinScopePath(outEnv.clone(), AbsynUtil::makeIdentPathFromString((SCodeUtil::elementName(outClass.clone())?).clone()))?);
    }
    Ok((outCache, outEnv, outClass, outFunctionName))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn instantiateImplicitRecordConstructors(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    outCache = 'mc: {
        let __mc_input = args.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inCache.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: record_name }, .. }, .. }, tail: rest_args } => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let (__pa0, Util::SUCCESS { .. }) = (instantiateDaeFunction(inCache.clone(), inEnv.clone(), record_name.clone(), false, None, false)?) else { bail!("pattern mismatch") };
                    cache = __pa0.clone();
                    Ok(instantiateImplicitRecordConstructors(cache.clone(), inEnv.clone(), rest_args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_args } => {
                    Ok(instantiateImplicitRecordConstructors(inCache.clone(), inEnv.clone(), rest_args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCache)
}

fn addDefaultArgs(mut inSlots: Arc<metamodelica::List<Slot>>, mut inInfo: SourceInfo) -> (Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Slot>>) {
    let mut outArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outSlots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    (outArgs, outSlots) = List::map2_2(inSlots.clone(), (std::sync::Arc::new(fillDefaultSlot) as std::sync::Arc<dyn ::std::ops::Fn(Slot, metamodelica::Array<Slot>, SourceInfo) -> Result<(Arc<DAE::Exp>, Slot)> + 'static>), metamodelica::arrayFromVec(inSlots.clone().into_iter().cloned().collect()), inInfo.clone());
    (outArgs, outSlots)
}

fn fillDefaultSlot(mut inSlot: Slot, mut inSlotArray: metamodelica::Array<Slot>, mut inInfo: SourceInfo) -> Result<(Arc<DAE::Exp>, Slot)> {
    let mut outArg: Arc<DAE::Exp>;
    let mut outSlot: Slot = <Slot as ::std::default::Default>::default();
    (outArg, outSlot) = (::match_deref::match_deref! { match &(inSlot.clone()) {
        Slot { arg: Some(arg), slotFilled: true, .. } => {
            (arg.clone(), inSlot.clone())
        },
        Slot { idx, defaultArg: Deref @ DAE::FuncArg { defaultBinding: Some(_), .. }, slotFilled: false, .. } => {
            fillDefaultSlot2(inSlotArray.borrow()[(idx.clone()-1) as usize].clone(), inSlotArray.clone(), inInfo.clone())?
        },
        Slot { defaultArg: Deref @ DAE::FuncArg { name: id, .. }, .. } => {
            Error::addSourceMessage(Error::UNFILLED_SLOT.clone(), list![(id.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outArg, outSlot))
}

fn fillDefaultSlot2(mut inSlot: Slot, mut inSlotArray: metamodelica::Array<Slot>, mut inInfo: SourceInfo) -> Result<(Arc<DAE::Exp>, Slot)> {
    let mut outArg: Arc<DAE::Exp>;
    let mut outSlot: Slot = inSlot.clone();
    (outArg, outSlot) = (::match_deref::match_deref! { match &(outSlot.clone()) {
        Slot { evalStatus: 2, arg: Some(exp), .. } => {
            (exp.clone(), inSlot.clone())
        },
        Slot { evalStatus: 1, defaultArg: Deref @ DAE::FuncArg { name: id, .. }, .. } => {
            Error::addSourceMessage(Error::CYCLIC_DEFAULT_VALUE.clone(), list![(id.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        Slot { evalStatus: 0, idx, defaultArg: Deref @ DAE::FuncArg { defaultBinding: Some(exp), .. }, .. } => {
            let mut exp = (*exp).clone();
            outSlot.evalStatus = SLOT_EVALUATING.clone();
            {let _arr = inSlotArray.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = outSlot.clone(); _arr};
            exp = evaluateSlotExp(exp.clone(), inSlotArray.clone(), inInfo.clone())?;
            outSlot.arg = Some(exp.clone());
            outSlot.slotFilled = true;
            outSlot.evalStatus = SLOT_EVALUATED.clone();
            {let _arr = inSlotArray.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = outSlot.clone(); _arr};
            (exp.clone(), outSlot.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outArg, outSlot))
}

fn evaluateSlotExp(mut inExp: Arc<DAE::Exp>, mut inSlotArray: metamodelica::Array<Slot>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    (outExp, _) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(evaluateSlotExp_traverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Slot>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Slot>, SourceInfo))> + 'static>), (inSlotArray.clone(), inInfo.clone()))?;
    Ok(outExp)
}

fn evaluateSlotExp_traverser(mut inExp: Arc<DAE::Exp>, mut inTuple: (metamodelica::Array<Slot>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Slot>, SourceInfo))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (metamodelica::Array<Slot>, SourceInfo);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (orig_exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }, .. }, (slots, info)) => {
            let mut slot: Option<Slot> = None;
            let mut exp: Arc<DAE::Exp>;
            slot = lookupSlotInArray((id.clone()).clone(), slots.clone());
            exp = getOptSlotDefaultExp(slot.clone(), slots.clone(), info.clone(), orig_exp.clone())?;
            (exp.clone(), (slots.clone(), info.clone()))
        },
        _ => {
            (inExp.clone(), inTuple.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

fn lookupSlotInArray(mut inSlotName: ArcStr, mut inSlots: metamodelica::Array<Slot>) -> Option<Slot> {
    let mut outSlot: Option<Slot> = None;
    let mut slot: Slot = <Slot as ::std::default::Default>::default();
    match '__try0: {
        (slot, _) = unwrap_break_err!(Array::getMemberOnTrue((inSlotName.clone()).clone(), inSlots.clone(), (std::sync::Arc::new(isSlotNamed) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Slot) -> Result<bool> + 'static>)), '__try0);
        outSlot = Some(slot.clone());
        Ok::<_, anyhow::Error>((outSlot.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outSlot = __try0_o0;
        }
        Err(_) => {
            outSlot = None;
        }
    }
    outSlot
}

fn isSlotNamed(mut inName: ArcStr, mut inSlot: Slot) -> Result<bool> {
    let mut outIsNamed: bool = false;
    let mut id: ArcStr = arcstr::literal!("");
    let Slot { defaultArg: __t1, .. } = (inSlot.clone()) else { bail!("pattern mismatch") };
    let __pa0 = ::match_deref::match_deref! { match &(__t1.clone()) {
        Deref @ DAE::FuncArg { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    id = __pa0.clone();
    outIsNamed = stringEq((id.clone()).clone(), (inName.clone()).clone());
    Ok(outIsNamed)
}

fn getOptSlotDefaultExp(mut inSlot: Option<Slot>, mut inSlots: metamodelica::Array<Slot>, mut inInfo: SourceInfo, mut inOrigExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (match inSlot.clone() {
        Some(mut slot) => {
            let mut exp: Arc<DAE::Exp>;
            (exp, _) = fillDefaultSlot(slot.clone(), inSlots.clone(), inInfo.clone())?;
            exp.clone()
        },
        None => {
            inOrigExp.clone()
        },
    });
    Ok(outExp)
}

fn determineConstSpecialFunc(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inConst: DAE::Const, mut inFuncName: Arc<Absyn::Path>) -> Result<(FCore::Cache, DAE::Const)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    let mut is_ext: bool = false;
    (outCache, is_ext) = isExternalObjectFunction(inCache.clone(), inEnv.clone(), inFuncName.clone())?;
    outConst = if (is_ext.clone()) {openmodelica_frontend_types::DAE::Const::C_VAR} else {inConst.clone()};
    Ok((outCache, outConst))
}

pub fn isExternalObjectFunction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, bool)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outIsExt: bool = false;
    let mut els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut last_id: ArcStr = arcstr::literal!("");
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(Lookup::lookupClass(inCache.clone(), inEnv.clone(), inPath.clone(), None), '__try0)) {
            (__pa1, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa2, .. }, .. }, _) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        outCache = __pa1.clone();
        els = __pa2.clone();
        let true = (SCodeUtil::isExternalObject(els.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        outIsExt = true;
        Ok::<_, anyhow::Error>((outCache.clone(), outIsExt.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outCache = __try0_o0;
            outIsExt = __try0_o1;
        }
        Err(_) => {
            last_id = (AbsynUtil::pathLastIdent(inPath.clone())?).clone();
            outCache = inCache.clone();
            outIsExt = last_id.clone() == literal!("constructor") || last_id.clone() == literal!("destructor");
        }
    }
    Ok((outCache, outIsExt))
}

pub const vectorizeArg: &'static str = "$vectorizeArg";

fn vectorizeCall(mut inExp: Arc<DAE::Exp>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inSlots: Arc<metamodelica::List<Slot>>, mut inProperties: DAE::Properties, mut info: SourceInfo) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = 'mc: {
        let __mc_input = (inExp.clone(), inDims.clone(), inProperties.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ metamodelica::List::Nil, prop) => {
                    Ok((e.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: ad }, prop) => {
                    if !((Flags::getConfigBool(Flags::CHECK_MODEL.clone())?)) { bail!("guard") }
                    Ok(vectorizeCall(e.clone(), cons(Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 }), ad.clone()), inSlots.clone(), prop.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { .. }, Deref @ metamodelica::List::Cons { head: dim, tail: ad }, DAE::Properties::PROP { type_: tp, constFlag: c }) => {
                    let mut vect_exp: Arc<DAE::Exp>;
                    let mut exp_type: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut int_dim: i32 = 0;
                    let mut tp = (*tp).clone();
                    int_dim = Expression::dimensionSize(dim.clone())?;
                    exp_type = Types::simplifyType(Types::liftArray(tp.clone(), dim.clone()))?;
                    vect_exp = vectorizeCallScalar(e.clone(), exp_type.clone(), int_dim.clone(), inSlots.clone())?;
                    tp = Types::liftArray(tp.clone(), dim.clone());
                    Ok(vectorizeCall(vect_exp.clone(), ad.clone(), inSlots.clone(), DAE::Properties::PROP { type_: tp.clone(), constFlag: c.clone() }, info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { .. }, Deref @ metamodelica::List::Cons { head: dim, tail: ad }, DAE::Properties::PROP { type_: tp, constFlag: c }) => {
                    let mut vect_exp: Arc<DAE::Exp>;
                    let mut int_dim: i32 = 0;
                    let mut tp = (*tp).clone();
                    int_dim = Expression::dimensionSize(dim.clone())?;
                    vect_exp = vectorizeCallArray(inExp.clone(), int_dim.clone(), inSlots.clone())?;
                    tp = Types::liftArrayRight(tp.clone(), dim.clone())?;
                    Ok(vectorizeCall(vect_exp.clone(), ad.clone(), inSlots.clone(), DAE::Properties::PROP { type_: tp.clone(), constFlag: c.clone() }, info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: r#fn, expLst: es, attr }, Deref @ metamodelica::List::Cons { head: dim, tail: ad }, prop @ DAE::Properties::PROP { type_: tp, constFlag: c }) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut rinfo: Arc<DAE::ReductionInfo>;
                    let mut foldName: ArcStr = arcstr::literal!("");
                    let mut resultName: ArcStr = arcstr::literal!("");
                    let mut riters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
                    let mut iterType: Absyn::ReductionIterType = Absyn::ReductionIterType::COMBINE;
                    let mut es = (*es).clone();
                    let mut prop = (*prop).clone();
                    let mut tp = (*tp).clone();
                    (es, riters) = vectorizeCallUnknownDimension(es.clone(), inSlots.clone(), info.clone())?;
                    tp = Types::liftArrayRight(tp.clone(), dim.clone())?;
                    prop = DAE::Properties::PROP { type_: tp.clone(), constFlag: c.clone() };
                    e = Arc::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: es.clone(), attr: attr.clone() });
                    (e, prop) = vectorizeCall(e.clone(), ad.clone(), inSlots.clone(), prop.clone(), info.clone())?;
                    foldName = (Util::getTempVariableIndex()).clone();
                    resultName = (Util::getTempVariableIndex()).clone();
                    iterType = if ((riters.clone().len() as i32) > 1) {openmodelica_ast::Absyn::ReductionIterType::THREAD} else {openmodelica_ast::Absyn::ReductionIterType::COMBINE};
                    rinfo = Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), iterType: iterType.clone(), exprType: tp.clone(), defaultValue: Some(Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] })), foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: None });
                    Ok((Arc::new(DAE::Exp::REDUCTION { reductionInfo: rinfo.clone(), expr: e.clone(), iterators: riters.clone() }), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { .. }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: _ }, DAE::Properties::PROP { .. }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot vectorize call with dimensions [")); __mm_s.push_str(&*ExpressionBasics::dimensionsString(inDims.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = (ExpressionBasics::dimensionString(listHead(inDims.clone())?)?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.vectorizeCall failed: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outProperties))
}

fn vectorizeCallUnknownDimension(mut inEs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inSlots: Arc<metamodelica::List<Slot>>, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>)> {
    let mut oes: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut ofound: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
    let mut rest_slots: Arc<metamodelica::List<Slot>> = inSlots.clone();
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut name: ArcStr = arcstr::literal!("");
    for mut e in &*inEs.clone() {
        let mut e = e.clone();
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(rest_slots.clone()) {
            Deref @ metamodelica::List::Cons { head: Slot { defaultArg: Deref @ DAE::FuncArg { ty: __pa0, .. }, dims: __pa1, .. }, tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa0.clone();
        dims = __pa1.clone();
        rest_slots = __pa2.clone();
        if dims.clone().is_empty() {
            oes = cons(e.clone(), oes.clone());
        } else {
            name = (Util::getTempVariableIndex()).clone();
            tp = Types::expTypetoTypesType(Expression::r#typeof(e.clone())?)?;
            ofound = cons(Arc::new(DAE::ReductionIterator { id: (name.clone()).clone(), exp: e.clone(), guardExp: None, ty: tp.clone() }), ofound.clone());
            oes = cons(Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty.clone() }), oes.clone());
        }
    }
    if ofound.clone().is_empty() {
        Error::addSourceMessageAndFail(Error::INTERNAL_ERROR.clone(), list![(literal!("Static.vectorizeCallUnknownDimension could not find any slot to vectorize")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    oes = oes.clone().reverse();
    ofound = ofound.clone().reverse();
    Ok((oes, ofound))
}

fn vectorizeCallArray(mut inExp: Arc<DAE::Exp>, mut inDim: i32, mut inSlots: Arc<metamodelica::List<Slot>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut sc: bool = false;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expl = __pa0.clone();
    ty = __pa1.clone();
    expl = vectorizeCallArray2(expl.clone(), ty.clone(), inDim.clone(), inSlots.clone())?;
    sc = Expression::typeBuiltin(ty.clone());
    ty = Expression::liftArrayRight(ty.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: inDim.clone() }));
    outExp = Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: sc.clone(), array: expl.clone() });
    Ok(outExp)
}

fn vectorizeCallArray2(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inType: Arc<DAE::Type>, mut inDim: i32, mut inSlots: Arc<metamodelica::List<Slot>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (inExpl.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { .. } => vectorizeCallScalar(e.clone(), inType.clone(), inDim.clone(), inSlots.clone())?,
        Deref @ DAE::Exp::ARRAY { .. } => vectorizeCallArray(e.clone(), inDim.clone(), inSlots.clone())?,
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpl)
}

fn vectorizeCallScalar(mut exp: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>, mut dim: i32, mut slots: Arc<metamodelica::List<Slot>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { .. } => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut scalar: bool = false;
                    let mut new_exp: Arc<DAE::Exp>;
                    let mut e_type: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut arr_type: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    expl = vectorizeCallScalar2(var_field!((*exp).path, DAE::Exp::CALL).clone(), var_field!((*exp).expLst, DAE::Exp::CALL).clone(), var_field!((*exp).attr, DAE::Exp::CALL).clone(), slots.clone(), dim.clone())?;
                    e_type = Expression::unliftArray(ty.clone())?;
                    scalar = Expression::typeBuiltin(e_type.clone());
                    arr_type = Arc::new(DAE::Type::T_ARRAY { ty: e_type.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() })] });
                    new_exp = Arc::new(DAE::Exp::ARRAY { ty: arr_type.clone(), scalar: scalar.clone(), array: expl.clone() });
                    Ok(new_exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Static.vectorizeCallScalar failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn vectorizeCallScalar2(mut r#fn: Arc<Absyn::Path>, mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut attr: Arc<DAE::CallAttributes>, mut slots: Arc<metamodelica::List<Slot>>, mut dim: i32) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut callargs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    for mut cur_dim in (1..=dim.clone()).rev() {
        callargs = vectorizeCallScalar3(exps.clone(), slots.clone(), cur_dim.clone())?;
        res = cons(Arc::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: callargs.clone(), attr: attr.clone() }), res.clone());
    }
    Ok(res)
}

fn vectorizeCallScalar3(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inSlots: Arc<metamodelica::List<Slot>>, mut inIndex: i32) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rest_slots: Arc<metamodelica::List<Slot>> = inSlots.clone();
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_slots.clone()) {
            Deref @ metamodelica::List::Cons { head: Slot { dims: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dims = __pa0.clone();
        rest_slots = __pa1.clone();
        if !(dims.clone().is_empty()) {
            e = Expression::makeASUB(e.clone(), list![Arc::new(DAE::Exp::ICONST { integer: inIndex.clone() })])?;
            (e, _) = ExpressionSimplify::simplify1(e.clone())?;
        }
        outExpl = cons(e.clone(), outExpl.clone());
    }
    outExpl = outExpl.clone().reverse();
    Ok(outExpl)
}

fn deoverloadFuncname(mut inPath: Arc<Absyn::Path>, mut inType: Arc<DAE::Type>, mut inEnv: FCore::Graph) -> (Arc<Absyn::Path>, Arc<DAE::Type>) {
    let mut outPath: Arc<Absyn::Path>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outPath, outType) = (::match_deref::match_deref! { match &(inType.clone()) {
        tty @ Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some(name), .. }, .. }, .. } => {
            let mut tty = (*tty).clone();
            assign_variant_field!(tty => DAE::Type::T_FUNCTION; path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }));
            (var_field!((**tty).path, DAE::Type::T_FUNCTION).clone(), tty.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { path: r#fn, .. } => {
            (r#fn.clone(), inType.clone())
        },
        _ => {
            (inPath.clone(), inType.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outPath, outType)
}

fn elabTypes(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inOnlyOneFunction: bool, mut inCheckTypes: bool, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<DAE::Const>>, Arc<DAE::Type>, Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Dimension>>>, Arc<metamodelica::List<Slot>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outConsts: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut outResultType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outFunctionType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut outSlots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut params: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let mut res_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut func_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut func_attr: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut pb: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    let mut path: Arc<Absyn::Path>;
    let mut success: bool = false;
    let mut rest_tys: Arc<metamodelica::List<Arc<DAE::Type>>> = inTypes.clone();
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut arg: Arc<DAE::Exp>;
    let mut numArgs: i32 = 0;
    let mut funcarg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    let debug: bool = false;
    if (rest_tys.clone().len() as i32) > 1 {
        numArgs = (inPosArgs.clone().len() as i32) + (inNamedArgs.clone().len() as i32);
        tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut ty in (rest_tys.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { .. } => numArgs.clone() <= (var_field!(ty.funcArg, DAE::Type::T_FUNCTION).clone().len() as i32) && numArgs.clone() >= ({
        let mut __acc: i32 = 0;
        for mut argument in (var_field!(ty.funcArg, DAE::Type::T_FUNCTION).clone()).into_iter().cloned() {
            let __x = if (isNone(argument.defaultBinding.clone())) {1} else {0};
            __acc += __x;
        }
        __acc
    }),
        _ => bail!("match: no arm matched"),
    } })) { continue; }
            let __x = ty.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if !(tys.clone().is_empty()) {
            rest_tys = tys.clone();
        }
    }
    while !(success.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_tys.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        func_ty = __pa0.clone();
        rest_tys = __pa1.clone();
        let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(func_ty.clone()) {
            Deref @ DAE::Type::T_FUNCTION { path: __pa2, functionAttributes: __pa3, funcResultType: __pa4, funcArg: __pa5 } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        path = __pa2.clone();
        func_attr = __pa3.clone();
        res_ty = __pa4.clone();
        params = __pa5.clone();
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("elabTypes, try: ")); __mm_s.push_str(&*TypesDump::unparseType(func_ty.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        match '__try6: {
            slots = makeEmptySlots(params.clone());
            (outCache, outArgs, outSlots, outConsts, pb) = unwrap_break_err!(elabInputArgs(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), slots.clone(), inOnlyOneFunction.clone(), inCheckTypes.clone(), inImplicit.clone(), inPrefix.clone(), inInfo.clone(), func_ty.clone(), path.clone(), false), '__try6);
            (outCache, pb) = unwrap_break_err!(addPolymorphicTypeVars(outCache.clone(), inEnv.clone(), typeVars.clone(), func_ty.clone(), pb.clone(), path.clone(), inInfo.clone()), '__try6);
            pb = unwrap_break_err!(Types::solvePolymorphicBindings(pb.clone(), inInfo.clone(), path.clone()), '__try6);
            res_ty = unwrap_break_err!(Types::fixPolymorphicRestype(res_ty.clone(), pb.clone(), inInfo.clone()), '__try6);
            (outArgs, outSlots, params, res_ty) = (match func_attr.isBuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_BUILTIN { unboxArgs: true, .. } => (List::map(outArgs.clone(), (std::sync::Arc::new(fnptr!(Expression::unboxExp, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>)), ({
        let mut __acc: Arc<metamodelica::List<Slot>> = metamodelica::nil();
        for mut slot in (outSlots.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(slot.clone()) {
        Slot { arg: Some(arg), .. } => {
            slot.arg = Some(Expression::unboxExp(arg.clone()));
            slot.clone()
        },
        _ => slot.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
        for mut p in (params.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(p.clone()) {
        funcarg => {
            let mut funcarg = (*funcarg).clone();
            assign_field!(funcarg.ty = unwrap_break_err!(Types::unboxedType(Types::fixPolymorphicRestype(p.ty.clone(), pb.clone(), inInfo.clone())?), '__try6));
            funcarg.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), unwrap_break_err!(Types::unboxedType(res_ty.clone()), '__try6)),
        _ => (outArgs.clone(), outSlots.clone(), params.clone(), res_ty.clone()),
    });
            (params, res_ty) = unwrap_break_err!(applyArgTypesToFuncType(outSlots.clone(), params.clone(), res_ty.clone(), inEnv.clone(), inCheckTypes.clone(), inInfo.clone()), '__try6);
            outDimensions = unwrap_break_err!(slotsVectorizable(outSlots.clone(), inInfo.clone()), '__try6);
            outResultType = res_ty.clone();
            outFunctionType = Arc::new(DAE::Type::T_FUNCTION { funcArg: params.clone(), funcResultType: outResultType.clone(), functionAttributes: func_attr.clone(), path: path.clone() });
            outFunctionType = unwrap_break_err!(Types::fixPolymorphicRestype(outFunctionType.clone(), pb.clone(), inInfo.clone()), '__try6);
            outFunctionType = unwrap_break_err!(createActualFunctype(outFunctionType.clone(), outSlots.clone(), inCheckTypes.clone()), '__try6);
            success = true;
            if debug.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("elabTypes success for ")); __mm_s.push_str(&*unwrap_break_err!(TypesDump::unparseType(func_ty.clone()), '__try6)); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*unwrap_break_err!(TypesDump::unparseType(outFunctionType.clone()), '__try6)); __mm_s.push_str(&*literal!("=>")); __mm_s.push_str(&*unwrap_break_err!(TypesDump::unparseType(outResultType.clone()), '__try6)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            Ok::<_, anyhow::Error>((outArgs.clone(), outCache.clone(), outConsts.clone(), outDimensions.clone(), outFunctionType.clone(), outResultType.clone(), outSlots.clone(), params.clone(), pb.clone(), res_ty.clone(), slots.clone(), success.clone()))
        } {
            Ok((__try6_o0, __try6_o1, __try6_o2, __try6_o3, __try6_o4, __try6_o5, __try6_o6, __try6_o7, __try6_o8, __try6_o9, __try6_o10, __try6_o11)) => {
                outArgs = __try6_o0;
                outCache = __try6_o1;
                outConsts = __try6_o2;
                outDimensions = __try6_o3;
                outFunctionType = __try6_o4;
                outResultType = __try6_o5;
                outSlots = __try6_o6;
                params = __try6_o7;
                pb = __try6_o8;
                res_ty = __try6_o9;
                slots = __try6_o10;
                success = __try6_o11;
            }
            Err(_) => {
                bail!("try/else: outputs not set in else branch");
            }
        }
    }
    Ok((outCache, outArgs, outConsts, outResultType, outFunctionType, outDimensions, outSlots))
}

fn addPolymorphicTypeVars(mut cache: FCore::Cache, mut env: FCore::Graph, mut typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut funcTy: Arc<DAE::Type>, mut pb: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut fnPath: Arc<Absyn::Path>, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut cache: FCore::Cache = cache;
    let mut pb: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = pb;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut scope: FCore::Graph;
    let mut e: Arc<SCode::Element>;
    let mut poly_types: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut ty_name: ArcStr = arcstr::literal!("");
    if typeVars.clone().is_empty() {
        return Ok((cache.clone(), pb.clone()));
    }
    (cache, e, _) = Lookup::lookupClass(cache.clone(), env.clone(), fnPath.clone(), None)?;
    poly_types = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut c in (SCodeUtil::getClassElements(e.clone())).into_iter().cloned() {
            if !(SCodeUtil::isPolymorphicTypeVar(c.clone())) { continue; }
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*SCodeUtil::getElementName(c.clone())?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if (typeVars.clone().len() as i32) > (poly_types.clone().len() as i32) {
        Error::addSourceMessage(Error::TOO_MANY_TYPE_VARS_IN_CALL.clone(), list![(AbsynUtil::pathString(fnPath.clone(), (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
        bail!("fail");
    }
    for mut tv in &*typeVars.clone() {
        let mut tv = tv.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(poly_types.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty_name = __pa0.clone();
        poly_types = __pa1.clone();
        (cache, e, scope) = Lookup::lookupClass(cache.clone(), env.clone(), tv.clone(), Some(info.clone()))?;
        (cache, _, ty) = Inst::instClassType(cache.clone(), scope.clone(), e.clone())?;
        pb = Types::addPolymorphicBinding((ty_name.clone()).clone(), ty.clone(), pb.clone())?;
    }
    Ok((cache, pb))
}

fn applyArgTypesToFuncType(mut inSlots: Arc<metamodelica::List<Slot>>, mut inParameters: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut inResultType: Arc<DAE::Type>, mut inEnv: FCore::Graph, mut checkTypes: bool, mut inInfo: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::FuncArg>>>, Arc<DAE::Type>)> {
    let mut outParameters: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let mut outResultType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut used_args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut used_slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env: FCore::Graph;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut dummy_var: Arc<SCode::Element>;
    if !(checkTypes.clone()) || inParameters.clone().is_empty() {
        outParameters = inParameters.clone();
        outResultType = inResultType.clone();
        return Ok((outParameters.clone(), outResultType.clone()));
    }
    tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut param in (inParameters.clone()).into_iter().cloned() {
            let __x = Types::funcArgType(param.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    dims = getAllOutputDimensions(inResultType.clone());
    dims = listAppend(List::mapFlat(tys.clone(), (std::sync::Arc::new(fnptr!(TypesDump::getDimensions, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> + 'static>)), dims.clone());
    used_args = extractNamesFromDims(dims.clone(), metamodelica::nil())?;
    used_slots = ({
        let mut __acc: Arc<metamodelica::List<Slot>> = metamodelica::nil();
        for mut s in (inSlots.clone()).into_iter().cloned() {
            if !(isSlotUsed(s.clone(), used_args.clone())?) { continue; }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    cache = FCore::noCache();
    vars = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut s in (used_slots.clone()).into_iter().cloned() {
            let __x = makeVarFromSlot(s.clone(), inEnv.clone(), cache.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    dummy_var = Arc::new(SCode::Element::COMPONENT { name: (literal!("dummy")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultVarAttr.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
    env = FGraph::openScope(inEnv.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (arcstr::literal!(FCore::forScopeName)).clone(), None)?;
    env = makeDummyFuncEnv(env.clone(), vars.clone(), dummy_var.clone())?;
    outParameters = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
        for (s, p) in (&(inSlots.clone())).into_iter().zip((&(inParameters.clone())).into_iter()) {
            let __x = evaluateFuncParamDimAndMatchTypes(s.clone(), p.clone(), env.clone(), cache.clone(), inInfo.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outResultType = evaluateFuncArgTypeDims(inResultType.clone(), env.clone(), cache.clone())?;
    Ok((outParameters, outResultType))
}

fn getAllOutputDimensions(mut inOutputType: Arc<DAE::Type>) -> Arc<metamodelica::List<Arc<DAE::Dimension>>> {
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    outDimensions = (::match_deref::match_deref! { match &(inOutputType.clone()) {
        Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
            List::mapFlat(tys.clone(), (std::sync::Arc::new(fnptr!(TypesDump::getDimensions, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> + 'static>))
        },
        _ => {
            TypesDump::getDimensions(inOutputType.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDimensions
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn extractNamesFromDims(mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inAccumNames: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outNames = (::match_deref::match_deref! { match &(inDimensions.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp }, tail: rest_dims } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            crefs = Expression::extractCrefsFromExp(exp.clone())?;
            names = List::fold(crefs.clone(), (std::sync::Arc::new(fnptr!(extractNamesFromDims2, Arc<DAE::ComponentRef>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), inAccumNames.clone());
            extractNamesFromDims(rest_dims.clone(), names.clone())?
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest_dims } => {
            extractNamesFromDims(rest_dims.clone(), inAccumNames.clone())?
        },
        Deref @ metamodelica::List::Nil => {
            inAccumNames.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNames)
}

fn extractNamesFromDims2(mut inCref: Arc<DAE::ComponentRef>, mut inAccumNames: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outNames = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. } => {
            outNames = if (List::isMemberOnTrue((name.clone()).clone(), inAccumNames.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) {inAccumNames.clone()} else {cons((name.clone()).clone(), inAccumNames.clone())};
            outNames.clone()
        },
        _ => {
            inAccumNames.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outNames
}

fn isSlotUsed(mut inSlot: Slot, mut inUsedNames: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut outIsUsed: bool = false;
    let mut slot_name: ArcStr = arcstr::literal!("");
    let Slot { defaultArg: __t1, .. } = (inSlot.clone()) else { bail!("pattern mismatch") };
    let __pa0 = ::match_deref::match_deref! { match &(__t1.clone()) {
        Deref @ DAE::FuncArg { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    slot_name = __pa0.clone();
    outIsUsed = List::isMemberOnTrue((slot_name.clone()).clone(), inUsedNames.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>));
    Ok(outIsUsed)
}

fn makeVarFromSlot(mut inSlot: Slot, mut inEnv: FCore::Graph, mut inCache: FCore::Cache) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    outVar = 'mc: {
        let __mc_input = inSlot.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Slot { arg: Some(exp), defaultArg: Deref @ DAE::FuncArg { name, .. }, .. } => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let false = (Expression::expHasCref(exp.clone(), ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?) else { bail!("pattern mismatch") };
                    ty = Expression::r#typeof(exp.clone())?;
                    let true = (Types::dimensionsKnown(ty.clone())?) else { bail!("pattern mismatch") };
                    binding = Arc::new(DAE::Binding::EQBOUND { exp: exp.clone(), evaluatedExp: None, constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE });
                    Ok(Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: DAE::dummyAttrParam().clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Slot { arg: Some(exp), defaultArg: Deref @ DAE::FuncArg { name, .. }, .. } => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut exp = (*exp).clone();
                    (_, val) = Ceval::ceval(inCache.clone(), inEnv.clone(), exp.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    exp = ValuesUtil::valueExp(val.clone(), Some(exp.clone()))?;
                    ty = Expression::r#typeof(exp.clone())?;
                    binding = Arc::new(DAE::Binding::EQBOUND { exp: exp.clone(), evaluatedExp: Some(val.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE });
                    Ok(Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: DAE::dummyAttrParam().clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Slot { defaultArg: Deref @ DAE::FuncArg { ty, name, .. }, .. } => {
                    Ok(Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: DAE::dummyAttrParam().clone(), ty: ty.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVar)
}

fn evaluateStructuralSlots2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inSlots: Arc<metamodelica::List<Slot>>, mut usedSlots: Arc<metamodelica::List<ArcStr>>, mut acc: Arc<metamodelica::List<Slot>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Slot>>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    (cache, slots) = 'mc: {
        let __mc_input = inSlots.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inCache.clone(), acc.clone().reverse()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: slot, tail: rest } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut slots: Arc<metamodelica::List<Slot>> = slots.clone();
                    let false = (isSlotUsed(slot.clone(), usedSlots.clone())?) else { bail!("pattern mismatch") };
                    (cache, slots) = evaluateStructuralSlots2(inCache.clone(), inEnv.clone(), rest.clone(), usedSlots.clone(), cons(slot.clone(), acc.clone()))?;
                    Ok((cache.clone(), slots.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Slot { defaultArg: defaultArg @ Deref @ DAE::FuncArg { .. }, slotFilled: _, arg: Some(exp), dims, idx, evalStatus: ses }, tail: rest } => {
                    let mut slot: Slot = <Slot as ::std::default::Default>::default();
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut exp = (*exp).clone();
                    let mut slots: Arc<metamodelica::List<Slot>> = slots.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    (cache, val) = Ceval::ceval(inCache.clone(), inEnv.clone(), exp.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    exp = ValuesUtil::valueExp(val.clone(), Some(exp.clone()))?;
                    slot = Slot { defaultArg: defaultArg.clone(), slotFilled: true, arg: Some(exp.clone()), dims: dims.clone(), idx: idx.clone(), evalStatus: ses.clone() };
                    (cache, slots) = evaluateStructuralSlots2(cache.clone(), inEnv.clone(), rest.clone(), usedSlots.clone(), cons(slot.clone(), acc.clone()))?;
                    Ok((cache.clone(), slots.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: slot, tail: rest } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut slots: Arc<metamodelica::List<Slot>> = slots.clone();
                    (cache, slots) = evaluateStructuralSlots2(inCache.clone(), inEnv.clone(), rest.clone(), usedSlots.clone(), cons(slot.clone(), acc.clone()))?;
                    Ok((cache.clone(), slots.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, slots))
}

fn evaluateStructuralSlots(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inSlots: Arc<metamodelica::List<Slot>>, mut funcType: Arc<DAE::Type>) -> Result<(FCore::Cache, Arc<metamodelica::List<Slot>>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut slots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    (cache, slots) = (::match_deref::match_deref! { match &(funcType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcResultType, funcArg, .. } => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut used_args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut arg in (funcArg.clone()).into_iter().cloned() {
            let __x = Types::funcArgType(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            dims = getAllOutputDimensions(funcResultType.clone());
            dims = listAppend(List::mapFlat(tys.clone(), (std::sync::Arc::new(fnptr!(TypesDump::getDimensions, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> + 'static>)), dims.clone());
            used_args = extractNamesFromDims(dims.clone(), metamodelica::nil())?;
            (cache, slots) = evaluateStructuralSlots2(inCache.clone(), inEnv.clone(), inSlots.clone(), used_args.clone(), metamodelica::nil())?;
            (cache.clone(), slots.clone())
        },
        _ => {
            (inCache.clone(), inSlots.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, slots))
}

fn makeDummyFuncEnv(mut inEnv: FCore::Graph, mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inDummyVar: Arc<SCode::Element>) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut dummy_var: Arc<SCode::Element>;
    for mut var in &*inVars.clone() {
        let mut var = var.clone();
        dummy_var = SCodeUtil::setComponentName(inDummyVar.clone(), (DAEUtil::typeVarIdent(var.clone())?).clone())?;
        outEnv = FGraph::mkComponentNode(outEnv.clone(), var.clone(), dummy_var.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_TYPED, FGraph::empty())?;
    }
    Ok(outEnv)
}

fn evaluateFuncParamDimAndMatchTypes(mut inSlot: Slot, mut inParam: Arc<DAE::FuncArg>, mut inEnv: FCore::Graph, mut inCache: FCore::Cache, mut inInfo: SourceInfo) -> Result<Arc<DAE::FuncArg>> {
    let mut outParam: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    outParam = (::match_deref::match_deref! { match &((inSlot.clone(), inParam.clone())) {
        (_, Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_CODE { .. }, .. }) => {
            inParam.clone()
        },
        (Slot { dims: vdims, arg: Some(Deref @ DAE::Exp::ARRAY { ty: sty, .. }), .. }, _) => {
            let mut pty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inParam.clone()) {
                Deref @ DAE::FuncArg { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            pty = __pa0.clone();
            pty = evaluateFuncArgTypeDims(pty.clone(), inEnv.clone(), inCache.clone())?;
            dims1 = TypesDump::getDimensions(pty.clone());
            dims1 = listAppend(vdims.clone(), dims1.clone());
            dims2 = TypesDump::getDimensions(sty.clone());
            let true = (Expression::dimsEqual(dims1.clone(), dims2.clone())?) else { bail!("pattern mismatch") };
            outParam = Types::setFuncArgType(inParam.clone(), pty.clone())?;
            outParam.clone()
        },
        (Slot { dims: vdims, arg: Some(Deref @ DAE::Exp::MATRIX { ty: sty, .. }), .. }, _) => {
            let mut pty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut vdims = (*vdims).clone();
            let __pa0 = ::match_deref::match_deref! { match &(inParam.clone()) {
                Deref @ DAE::FuncArg { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            pty = __pa0.clone();
            pty = evaluateFuncArgTypeDims(pty.clone(), inEnv.clone(), inCache.clone())?;
            dims1 = TypesDump::getDimensions(pty.clone());
            vdims = listAppend(dims1.clone(), vdims.clone());
            dims2 = TypesDump::getDimensions(sty.clone());
            let true = (Expression::dimsEqual(vdims.clone(), dims2.clone())?) else { bail!("pattern mismatch") };
            outParam = Types::setFuncArgType(inParam.clone(), pty.clone())?;
            outParam.clone()
        },
        _ => {
            let mut pty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let __pa0 = ::match_deref::match_deref! { match &(inParam.clone()) {
                Deref @ DAE::FuncArg { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            pty = __pa0.clone();
            pty = evaluateFuncArgTypeDims(pty.clone(), inEnv.clone(), inCache.clone())?;
            outParam = Types::setFuncArgType(inParam.clone(), pty.clone())?;
            outParam.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outParam)
}

fn evaluateFuncArgTypeDims(mut inType: Arc<DAE::Type>, mut inEnv: FCore::Graph, mut inCache: FCore::Cache) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut n: i32 = 0;
                    let mut ty = (*ty).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Ceval::cevalDimension(inCache.clone(), inEnv.clone(), dim.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    n = __pa0.clone();
                    ty = evaluateFuncArgTypeDims(ty.clone(), inEnv.clone(), inCache.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut ty = (*ty).clone();
                    ty = evaluateFuncArgTypeDims(ty.clone(), inEnv.clone(), inCache.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ty @ Deref @ DAE::Type::T_TUPLE { .. } => {
                    let mut ty = (*ty).clone();
                    assign_variant_field!(ty => DAE::Type::T_TUPLE; types = List::map2(var_field!((*ty).types, DAE::Type::T_TUPLE).clone(), (std::sync::Arc::new(evaluateFuncArgTypeDims) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, FCore::Graph, FCore::Cache) -> Result<Arc<DAE::Type>> + 'static>), inEnv.clone(), inCache.clone()));
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn createActualFunctype(mut tp: Arc<DAE::Type>, mut slots: Arc<metamodelica::List<Slot>>, mut checkTypes: bool) -> Result<Arc<DAE::Type>> {
    let mut outTp: Arc<DAE::Type> = tp.clone();
    outTp = (::match_deref::match_deref! { match &((outTp.clone(), checkTypes.clone())) {
        (_, true) => tp.clone(),
        (Deref @ DAE::Type::T_FUNCTION { .. }, _) => {
            assign_variant_field!(outTp => DAE::Type::T_FUNCTION; funcArg = funcArgsFromSlots(slots.clone())?);
            outTp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn slotsVectorizable(mut inSlots: Arc<metamodelica::List<Slot>>, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    outDims = 'mc: {
        let __mc_input = inSlots.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Slot { dims: ad @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, arg: Some(exp), defaultArg: Deref @ DAE::FuncArg { name, .. }, .. }, tail: rest } => {
                    sameSlotsVectorizable(rest.clone(), ad.clone(), (name.clone()).clone(), exp.clone(), info.clone())?;
                    Ok(ad.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Slot { dims: Deref @ metamodelica::List::Nil, .. }, tail: rest } => {
                    Ok(slotsVectorizable(rest.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-slots_vectorizable failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDims)
}

fn sameSlotsVectorizable(mut inSlots: Arc<metamodelica::List<Slot>>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut name: ArcStr, mut exp: Arc<DAE::Exp>, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inSlots.clone()) {
        Deref @ metamodelica::List::Cons { head: Slot { dims: slot_ad @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, arg: Some(exp2), defaultArg: Deref @ DAE::FuncArg { name: name2, .. }, .. }, tail: rest } => {
            sameArraydimLst(inDims.clone(), (name.clone()).clone(), exp.clone(), slot_ad.clone(), (name2.clone()).clone(), exp2.clone(), info.clone())?;
            sameSlotsVectorizable(rest.clone(), inDims.clone(), (name.clone()).clone(), exp.clone(), info.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: Slot { dims: Deref @ metamodelica::List::Nil, .. }, tail: rest } => {
            sameSlotsVectorizable(rest.clone(), inDims.clone(), (name.clone()).clone(), exp.clone(), info.clone())?;
            ()
        },
        Deref @ metamodelica::List::Nil => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn sameArraydimLst(mut inDims1: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut name1: ArcStr, mut exp1: Arc<DAE::Exp>, mut inDims2: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut name2: ArcStr, mut exp2: Arc<DAE::Exp>, mut info: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inDims2.clone(), inDims2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: i1 }, tail: ads1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: i2 }, tail: ads2 }) => {
                    let true = (intEq(i1.clone(), i2.clone())) else { bail!("pattern mismatch") };
                    sameArraydimLst(ads1.clone(), (name1.clone()).clone(), exp1.clone(), ads2.clone(), (name2.clone()).clone(), exp2.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: ads1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: ads2 }) => {
                    sameArraydimLst(ads1.clone(), (name1.clone()).clone(), exp1.clone(), ads2.clone(), (name2.clone()).clone(), exp2.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: e1 }, tail: ads1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: e2 }, tail: ads2 }) => {
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    sameArraydimLst(ads1.clone(), (name1.clone()).clone(), exp1.clone(), ads2.clone(), (name2.clone()).clone(), exp2.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ad1, tail: _ }, Deref @ metamodelica::List::Cons { head: ad2, tail: _ }) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut str4: ArcStr = arcstr::literal!("");
                    str1 = (ExpressionBasics::printExpStr(exp1.clone())?).clone();
                    str2 = (ExpressionBasics::printExpStr(exp2.clone())?).clone();
                    str3 = (ExpressionBasics::dimensionString(ad1.clone())?).clone();
                    str4 = (ExpressionBasics::dimensionString(ad2.clone())?).clone();
                    Error::addSourceMessage(Error::VECTORIZE_CALL_DIM_MISMATCH.clone(), list![(name1.clone()).clone(), (str1.clone()).clone(), (name2.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone(), (str4.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getProperties(mut inType: Arc<DAE::Type>, mut inTupleConst: Arc<DAE::TupleConst>) -> Result<DAE::Properties> {
    let mut outProperties: DAE::Properties;
    outProperties = (::match_deref::match_deref! { match &((inType.clone(), inTupleConst.clone())) {
        (tt @ Deref @ DAE::Type::T_TUPLE { .. }, r#const) => {
            DAE::Properties::PROP_TUPLE { type_: tt.clone(), tupleConst: r#const.clone() }
        },
        (t, Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: b }, tail: Deref @ metamodelica::List::Nil } }) => {
            DAE::Properties::PROP { type_: t.clone(), constFlag: b.clone() }
        },
        (t, Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: b }, tail: Deref @ metamodelica::List::Nil } }) => {
            DAE::Properties::PROP { type_: t.clone(), constFlag: b.clone() }
        },
        (t, Deref @ DAE::TupleConst::SINGLE_CONST { r#const: b }) => {
            DAE::Properties::PROP { type_: t.clone(), constFlag: b.clone() }
        },
        (ty, r#const) => {
            let mut tystr: ArcStr = arcstr::literal!("");
            let mut conststr: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- Static.getProperties failed: ")).clone())?;
            tystr = (TypesDump::unparseType(ty.clone())?).clone();
            conststr = (TypesDump::printTupleConstStr(r#const.clone())?).clone();
            Debug::trace((tystr.clone()).clone())?;
            Debug::trace((literal!(", ")).clone())?;
            Debug::traceln((conststr.clone()).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outProperties)
}

fn elabConsts(mut inType: Arc<DAE::Type>, mut inConst: DAE::Const) -> Result<Arc<DAE::TupleConst>> {
    let mut outTupleConst: Arc<DAE::TupleConst>;
    outTupleConst = (::match_deref::match_deref! { match &((inType.clone(), inConst.clone())) {
        (Deref @ DAE::Type::T_TUPLE { types: tys, .. }, c) => {
            let mut consts: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
            consts = checkConsts(tys.clone(), c.clone())?;
            Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: consts.clone() })
        },
        (ty, c) => {
            let mut consts: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
            consts = checkConsts(list![ty.clone()], c.clone())?;
            Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: consts.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTupleConst)
}

fn checkConsts(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inConst: DAE::Const) -> Result<Arc<metamodelica::List<Arc<DAE::TupleConst>>>> {
    let mut outTupleConsts: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
    outTupleConsts = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
        for mut ty in (inTypes.clone()).into_iter().cloned() {
            let __x = checkConst(ty.clone(), inConst.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outTupleConsts)
}

fn checkConst(mut inType: Arc<DAE::Type>, mut c: DAE::Const) -> Result<Arc<DAE::TupleConst>> {
    let mut outTupleConst: Arc<DAE::TupleConst>;
    outTupleConst = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_TUPLE { .. } => {
            Error::addInternalError((literal!("No support for tuples built by tuples")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => Arc::new(DAE::TupleConst::SINGLE_CONST { r#const: c.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTupleConst)
}

fn splitProps(mut inProperties: Arc<metamodelica::List<DAE::Properties>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<Arc<DAE::TupleConst>>>)> {
    let mut outTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut outConsts: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut tc: Arc<DAE::TupleConst>;
    for mut prop in &*inProperties.clone().reverse() {
        let mut prop = prop.clone();
        tc = (match prop.clone() {
        DAE::Properties::PROP { constFlag: mut c, type_: ref ty } => Arc::new(DAE::TupleConst::SINGLE_CONST { r#const: c.clone() }),
        DAE::Properties::PROP_TUPLE { tupleConst: ref tc, type_: ref ty } => tc.clone(),
        _ => bail!("match: no arm matched"),
    });
        outTypes = cons(ty.clone(), outTypes.clone());
        outConsts = cons(tc.clone(), outConsts.clone());
    }
    Ok((outTypes, outConsts))
}

fn getTypes(mut farg: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> {
    let mut outTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    outTypes = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut arg in (farg.clone()).into_iter().cloned() {
            let __x = Types::funcArgType(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outTypes)
}

fn elabInputArgs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inSlots: Arc<metamodelica::List<Slot>>, mut inOnlyOneFunction: bool, mut inCheckTypes: bool, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut inFuncType: Arc<DAE::Type>, mut inPath: Arc<Absyn::Path>, mut isGraphicsExp: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Slot>>, Arc<metamodelica::List<DAE::Const>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outSlots: Arc<metamodelica::List<Slot>> = inSlots.clone();
    let mut outConsts: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut outPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let mut consts1: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut consts2: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    if inPosArgs.clone().is_empty() && inNamedArgs.clone().is_empty() {
        outConsts = list![openmodelica_frontend_types::DAE::Const::C_CONST];
    } else {
        fargs = funcArgsFromSlots(inSlots.clone())?;
        (outCache, outSlots, consts1, outPolymorphicBindings) = elabPositionalInputArgs(outCache.clone(), inEnv.clone(), inPosArgs.clone(), fargs.clone(), outSlots.clone(), inOnlyOneFunction.clone(), inCheckTypes.clone(), inImplicit.clone(), outPolymorphicBindings.clone(), inPrefix.clone(), inInfo.clone(), inPath.clone(), isGraphicsExp.clone())?;
        (outCache, outSlots, consts2, outPolymorphicBindings) = elabNamedInputArgs(outCache.clone(), inEnv.clone(), inNamedArgs.clone(), fargs.clone(), outSlots.clone(), inOnlyOneFunction.clone(), inCheckTypes.clone(), inImplicit.clone(), outPolymorphicBindings.clone(), inPrefix.clone(), inInfo.clone(), inPath.clone(), isGraphicsExp.clone())?;
        outConsts = listAppend(consts1.clone(), consts2.clone());
    }
    (outCache, outSlots) = evaluateStructuralSlots(outCache.clone(), inEnv.clone(), outSlots.clone(), inFuncType.clone())?;
    outExps = slotListArgs(outSlots.clone());
    Ok((outCache, outExps, outSlots, outConsts, outPolymorphicBindings))
}

fn makeEmptySlots(mut inArgs: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Arc<metamodelica::List<Slot>> {
    let mut outSlots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    (outSlots, _) = List::mapFold(inArgs.clone(), (std::sync::Arc::new(fnptr!(makeEmptySlot, Arc<DAE::FuncArg>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, i32) -> Result<(Slot, i32)> + 'static>), 1);
    outSlots
}

fn makeEmptySlot(mut inArg: Arc<DAE::FuncArg>, mut inIndex: i32) -> (Slot, i32) {
    let mut outSlot: Slot = <Slot as ::std::default::Default>::default();
    let mut outIndex: i32 = 0;
    outSlot = Slot { defaultArg: inArg.clone(), slotFilled: false, arg: None, dims: metamodelica::nil(), idx: inIndex.clone(), evalStatus: SLOT_NOT_EVALUATED.clone() };
    outIndex = inIndex.clone() + 1;
    (outSlot, outIndex)
}

fn funcArgsFromSlots(mut inSlots: Arc<metamodelica::List<Slot>>) -> Result<Arc<metamodelica::List<Arc<DAE::FuncArg>>>> {
    let mut outFuncArgs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    outFuncArgs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
        for mut slot in (inSlots.clone()).into_iter().cloned() {
            let __x = funcArgFromSlot(slot.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outFuncArgs)
}

fn funcArgFromSlot(mut inSlot: Slot) -> Result<Arc<DAE::FuncArg>> {
    let mut outFuncArg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    let Slot { defaultArg: __pa0, .. } = (inSlot.clone()) else { bail!("pattern mismatch") };
    outFuncArg = __pa0.clone();
    Ok(outFuncArg)
}

fn complexTypeFromSlots(mut inSlots: Arc<metamodelica::List<Slot>>, mut complexClassType: ClassInf::State) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut id: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    for mut slot in &*inSlots.clone() {
        let mut slot = slot.clone();
        let Slot { defaultArg: __t2, .. } = (slot.clone()) else { bail!("pattern mismatch") };
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(__t2.clone()) {
            Deref @ DAE::FuncArg { ty: __pa0, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa0.clone();
        id = __pa1.clone();
        vars = cons(Expression::makeVar((id.clone()).clone(), Types::simplifyType(ty.clone())?), vars.clone());
    }
    vars = vars.clone().reverse();
    outType = Arc::new(DAE::Type::T_COMPLEX { complexClassType: complexClassType.clone(), varLst: vars.clone(), equalityConstraint: None, usedExternally: false });
    Ok(outType)
}

fn slotListArgs(mut inSlots: Arc<metamodelica::List<Slot>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outArgs = List::filterMap(inSlots.clone(), (std::sync::Arc::new(slotArg) as std::sync::Arc<dyn ::std::ops::Fn(Slot) -> Result<Arc<DAE::Exp>> + 'static>));
    outArgs
}

fn slotArg(mut inSlot: Slot) -> Result<Arc<DAE::Exp>> {
    let mut outArg: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(inSlot.clone()) {
        Slot { arg: Some(__pa0), .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outArg = __pa0.clone();
    Ok(outArg)
}

fn fillGraphicsDefaultSlots(mut inCache: FCore::Cache, mut inSlots: Arc<metamodelica::List<Slot>>, mut inClass: Arc<SCode::Element>, mut inEnv: FCore::Graph, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Slot>>, Arc<metamodelica::List<DAE::Const>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outSlots: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut outConsts: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut outPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    let mut filled: bool = false;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut exp: Arc<DAE::Exp>;
    let mut defarg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    for mut slot in &*inSlots.clone() {
        let mut slot = slot.clone();
        let Slot { slotFilled: __pa0, .. } = (slot.clone()) else { bail!("pattern mismatch") };
        filled = __pa0.clone();
        if !(filled.clone()) {
            slot = 'mc: {
        let __mc_input = slot.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Slot { defaultArg: defarg @ Deref @ DAE::FuncArg { .. }, .. } => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut e: Arc<Absyn::Exp> = e.clone();
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    let mut c: DAE::Const = c.clone();
                    let mut exp: Arc<DAE::Exp>;
                    let mut outConsts: Arc<metamodelica::List<DAE::Const>> = outConsts.clone();
                    let mut outPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = outPolymorphicBindings.clone();
                    let mut slot: Slot;
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::getElementNamed((defarg.name.clone()).clone(), inClass.clone())?) {
                        Deref @ SCode::Element::COMPONENT { modifications: Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache.clone(), inEnv.clone(), e.clone(), inImplicit.clone(), true, inPrefix.clone(), inInfo.clone())?) {
                        (__pa2, __pa3, DAE::Properties::PROP { type_: __pa4, constFlag: __pa5 }) => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    outCache = __pa2.clone();
                    exp = __pa3.clone();
                    ty = __pa4.clone();
                    c = __pa5.clone();
                    (exp, _, outPolymorphicBindings) = Types::matchTypePolymorphic(exp.clone(), ty.clone(), defarg.ty.clone(), FGraph::getGraphPathNoImplicitScope(inEnv.clone())?, outPolymorphicBindings.clone(), false)?;
                    let true = (Types::constEqualOrHigher(c.clone(), defarg.r#const.clone())) else { bail!("pattern mismatch") };
                    outConsts = cons(c.clone(), outConsts.clone());
                    slot.slotFilled = true;
                    slot.arg = Some(exp.clone());
                    Ok(slot.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(slot.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        }
        outSlots = cons(slot.clone(), outSlots.clone());
    }
    outSlots = outSlots.clone().reverse();
    outConsts = outConsts.clone().reverse();
    Ok((outCache, outSlots, outConsts, outPolymorphicBindings))
}

fn printSlotsStr(mut inSlots: Arc<metamodelica::List<Slot>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inSlots.clone()) {
        Deref @ metamodelica::List::Cons { head: Slot { dims: ds, arg: exp, slotFilled: filled, defaultArg: farg, .. }, tail: xs } => {
            let mut farg_str: ArcStr = arcstr::literal!("");
            let mut filledStr: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            let mut str_lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            farg_str = (TypesDump::printFargStr(farg.clone())?).clone();
            filledStr = (if (filled.clone()) {literal!("filled")} else {literal!("not filled")}).clone();
            r#str = (Util::applyOptionOrDefault(exp.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>), (literal!("")).clone())).clone();
            str_lst = List::map(ds.clone(), (std::sync::Arc::new(ExpressionBasics::dimensionString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(str_lst.clone(), (literal!(", ")).clone());
            s1 = stringAppendList(list![(literal!("SLOT(")).clone(), (farg_str.clone()).clone(), (literal!(", ")).clone(), (filledStr.clone()).clone(), (literal!(", ")).clone(), (r#str.clone()).clone(), (literal!(", [")).clone(), (s.clone()).clone(), (literal!("])\n")).clone()]);
            s2 = (printSlotsStr(xs.clone())?).clone();
            res = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            res.clone()
        },
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn isFreeParameterExp(mut inExp: Arc<DAE::Exp>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(bool, FCore::Cache)> {
    let mut isFree: bool = false;
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    outCache = inCache.clone();
    isFree = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => {
            true
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let mut exp1: Arc<DAE::Exp>;
            (outCache, _, _, binding, _, _, _, _, _) = Lookup::lookupVar(inCache.clone(), inEnv.clone(), cr.clone())?;
            (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ DAE::Binding::VALBOUND { .. } => true,
        Deref @ DAE::Binding::EQBOUND { exp: exp1, .. } if (Expression::isConst(exp1.clone())?) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ DAE::Exp::BINARY { exp2, exp1, .. } => {
            let mut isFree2: bool = false;
            (isFree, outCache) = isFreeParameterExp(exp1.clone(), inCache.clone(), inEnv.clone())?;
            (isFree2, outCache) = isFreeParameterExp(exp2.clone(), outCache.clone(), inEnv.clone())?;
            isFree.clone() && isFree2.clone()
        },
        Deref @ DAE::Exp::UNARY { exp: exp1, .. } => {
            (isFree, outCache) = isFreeParameterExp(exp1.clone(), inCache.clone(), inEnv.clone())?;
            isFree.clone()
        },
        Deref @ DAE::Exp::LBINARY { exp2, exp1, .. } => {
            let mut isFree2: bool = false;
            (isFree, outCache) = isFreeParameterExp(exp1.clone(), inCache.clone(), inEnv.clone())?;
            (isFree2, outCache) = isFreeParameterExp(exp2.clone(), outCache.clone(), inEnv.clone())?;
            isFree.clone() && isFree2.clone()
        },
        Deref @ DAE::Exp::LUNARY { exp: exp1, .. } => {
            (isFree, outCache) = isFreeParameterExp(exp1.clone(), inCache.clone(), inEnv.clone())?;
            isFree.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: exps, .. } => {
            let mut isFree2: bool = false;
            outCache = inCache.clone();
            isFree = true;
            for mut exp in &*exps.clone() {
                let mut exp = exp.clone();
                (isFree2, outCache) = isFreeParameterExp(exp.clone(), outCache.clone(), inEnv.clone())?;
                isFree = isFree.clone() && isFree2.clone();
            }
            isFree.clone()
        },
        Deref @ DAE::Exp::ARRAY { array: exps, .. } => {
            let mut isFree2: bool = false;
            outCache = inCache.clone();
            isFree = true;
            for mut exp in &*exps.clone() {
                let mut exp = exp.clone();
                (isFree2, outCache) = isFreeParameterExp(exp.clone(), outCache.clone(), inEnv.clone())?;
                isFree = isFree.clone() && isFree2.clone();
            }
            isFree.clone()
        },
        Deref @ DAE::Exp::MATRIX { matrix: mat, .. } => {
            let mut isFree2: bool = false;
            outCache = inCache.clone();
            isFree = true;
            for mut row in &*mat.clone() {
                let mut row = row.clone();
                for mut exp in &*row.clone() {
                    let mut exp = exp.clone();
                    (isFree2, outCache) = isFreeParameterExp(exp.clone(), outCache.clone(), inEnv.clone())?;
                    isFree = isFree.clone() && isFree2.clone();
                }
            }
            isFree.clone()
        },
        Deref @ DAE::Exp::CAST { exp: exp1, .. } => {
            (isFree, outCache) = isFreeParameterExp(exp1.clone(), inCache.clone(), inEnv.clone())?;
            isFree.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((isFree, outCache))
}

fn elabPositionalInputArgs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inFuncArgs: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut inSlots: Arc<metamodelica::List<Slot>>, mut inOnlyOneFunction: bool, mut inCheckTypes: bool, mut inImplicit: bool, mut inPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut inPath: Arc<Absyn::Path>, mut isGraphicsExp: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Slot>>, Arc<metamodelica::List<DAE::Const>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outSlots: Arc<metamodelica::List<Slot>> = inSlots.clone();
    let mut outConsts: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut outPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = inPolymorphicBindings.clone();
    let mut farg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    let mut farg_rest: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = inFuncArgs.clone();
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut position: i32 = 1;
    for mut arg in &*inPosArgs.clone() {
        let mut arg = arg.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(farg_rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        farg = __pa0.clone();
        farg_rest = __pa1.clone();
        (outCache, outSlots, c, outPolymorphicBindings) = elabPositionalInputArg(outCache.clone(), inEnv.clone(), arg.clone(), farg.clone(), position.clone(), outSlots.clone(), inOnlyOneFunction.clone(), inCheckTypes.clone(), inImplicit.clone(), outPolymorphicBindings.clone(), inPrefix.clone(), inInfo.clone(), inPath.clone(), isGraphicsExp.clone())?;
        position = position.clone() + 1;
        outConsts = cons(c.clone(), outConsts.clone());
    }
    outConsts = outConsts.clone().reverse();
    Ok((outCache, outSlots, outConsts, outPolymorphicBindings))
}

fn elabPositionalInputArg(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut farg: Arc<DAE::FuncArg>, mut position: i32, mut inSlotLst: Arc<metamodelica::List<Slot>>, mut onlyOneFunction: bool, mut checkTypes: bool, mut r#impl: bool, mut inPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut path: Arc<Absyn::Path>, mut isGraphicsExp: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Slot>>, DAE::Const, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSlotLst: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    let mut outPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    let mut numErrors: i32 = Error::getNumErrorMessages();
    (outCache, outSlotLst, outConst, outPolymorphicBindings) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), farg.clone(), inSlotLst.clone(), onlyOneFunction.clone(), checkTypes.clone(), inPolymorphicBindings.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { par: pr, ty: vt @ Deref @ DAE::Type::T_CODE { ty: ct }, name: id, .. }, slots, _, true, polymorphicBindings, pre) => {
                    let mut slots_1: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    e_1 = elabCodeExp(e.clone(), cache.clone(), env.clone(), ct.clone(), info.clone())?;
                    slots_1 = fillSlot(Arc::new(DAE::FuncArg { name: (id.clone()).clone(), ty: vt.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: pr.clone(), defaultBinding: None }), e_1.clone(), metamodelica::nil(), slots.clone(), pre.clone(), info.clone(), path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), openmodelica_frontend_types::DAE::Const::C_VAR, polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { par: pr, ty: vt, name: id, .. }, slots, _, true, polymorphicBindings, pre) => {
                    let mut slots_1: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut props: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let mut vt = (*vt).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    (cache, e_1, props) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    t = Types::getPropType(props.clone())?;
                    (vt, _) = Types::traverseType(vt.clone(), -1, (std::sync::Arc::new(fnptr!(Types::makeExpDimensionsUnknown, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?;
                    c1 = Types::propAllConst(props.clone())?;
                    (e_2, _, polymorphicBindings) = Types::matchTypePolymorphic(e_1.clone(), t.clone(), vt.clone(), FGraph::getGraphPathNoImplicitScope(env.clone())?, polymorphicBindings.clone(), false)?;
                    slots_1 = fillSlot(Arc::new(DAE::FuncArg { name: (id.clone()).clone(), ty: vt.clone(), r#const: c1.clone(), par: pr.clone(), defaultBinding: None }), e_2.clone(), metamodelica::nil(), slots.clone(), pre.clone(), info.clone(), path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1.clone(), polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { par: pr, ty: vt, name: id, .. }, slots, _, true, polymorphicBindings, pre) => {
                    let mut slots_1: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut ds: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut props: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let mut vt = (*vt).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    (cache, e_1, props) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    t = Types::getPropType(props.clone())?;
                    (vt, _) = Types::traverseType(vt.clone(), -1, (std::sync::Arc::new(fnptr!(Types::makeExpDimensionsUnknown, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?;
                    c1 = Types::propAllConst(props.clone())?;
                    (e_2, _, ds, polymorphicBindings) = Types::vectorizableType(e_1.clone(), t.clone(), vt.clone(), FGraph::getGraphPathNoImplicitScope(env.clone())?)?;
                    slots_1 = fillSlot(Arc::new(DAE::FuncArg { name: (id.clone()).clone(), ty: vt.clone(), r#const: c1.clone(), par: pr.clone(), defaultBinding: None }), e_2.clone(), ds.clone(), slots.clone(), pre.clone(), info.clone(), path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1.clone(), polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { par: pr, name: id, .. }, slots, _, false, polymorphicBindings, pre) => {
                    let mut slots_1: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut props: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, props) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    t = Types::getPropType(props.clone())?;
                    c1 = Types::propAllConst(props.clone())?;
                    slots_1 = fillSlot(Arc::new(DAE::FuncArg { name: (id.clone()).clone(), ty: t.clone(), r#const: c1.clone(), par: pr.clone(), defaultBinding: None }), e_1.clone(), metamodelica::nil(), slots.clone(), pre.clone(), info.clone(), path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1.clone(), polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { ty: vt, name: id, .. }, _, true, true, _, pre) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let true = (Error::getNumErrorMessages() == numErrors.clone()) else { bail!("pattern mismatch") };
                    (cache, e_1, prop) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    s1 = (intString(position.clone())).clone();
                    s2 = AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?;
                    s3 = (ExpressionBasics::printExpStr(e_1.clone())?).clone();
                    s4 = (TypesDump::unparseTypeNoAttr(Types::getPropType(prop.clone())?)?).clone();
                    s5 = (TypesDump::unparseTypeNoAttr(vt.clone())?).clone();
                    Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (id.clone()).clone(), (s3.clone()).clone(), (s4.clone()).clone(), (s5.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outSlotLst, outConst, outPolymorphicBindings))
}

fn elabNamedInputArgs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inTypesFuncArgLst: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut inSlotLst: Arc<metamodelica::List<Slot>>, mut onlyOneFunction: bool, mut checkTypes: bool, mut r#impl: bool, mut inPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut path: Arc<Absyn::Path>, mut isGraphicsExp: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Slot>>, Arc<metamodelica::List<DAE::Const>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSlotLst: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut outTypesConstLst: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    let mut outPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    (outCache, outSlotLst, outTypesConstLst, outPolymorphicBindings) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynNamedArgLst.clone(), inTypesFuncArgLst.clone(), inSlotLst.clone(), inPolymorphicBindings.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, _, slots, _) => {
            (cache.clone(), slots.clone(), metamodelica::nil(), inPolymorphicBindings.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: na, tail: nas }, farg, slots, polymorphicBindings) => {
            let mut c1: DAE::Const = DAE::Const::C_CONST;
            let mut clist: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut slots = (*slots).clone();
            let mut polymorphicBindings = (*polymorphicBindings).clone();
            (cache, slots, c1, polymorphicBindings) = elabNamedInputArg(cache.clone(), env.clone(), na.clone(), farg.clone(), slots.clone(), onlyOneFunction.clone(), checkTypes.clone(), r#impl.clone(), polymorphicBindings.clone(), inPrefix.clone(), info.clone(), path.clone(), Error::getNumErrorMessages(), isGraphicsExp.clone())?;
            (cache, slots, clist, polymorphicBindings) = elabNamedInputArgs(cache.clone(), env.clone(), nas.clone(), farg.clone(), slots.clone(), onlyOneFunction.clone(), checkTypes.clone(), r#impl.clone(), polymorphicBindings.clone(), inPrefix.clone(), info.clone(), path.clone(), isGraphicsExp.clone())?;
            (cache.clone(), slots.clone(), cons(c1.clone(), clist.clone()), polymorphicBindings.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outSlotLst, outTypesConstLst, outPolymorphicBindings))
}

fn elabNamedInputArg(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inNamedArg: Arc<Absyn::NamedArg>, mut inTypesFuncArgLst: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut inSlotLst: Arc<metamodelica::List<Slot>>, mut onlyOneFunction: bool, mut checkTypes: bool, mut r#impl: bool, mut inPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut path: Arc<Absyn::Path>, mut numErrors: i32, mut isGraphicsExp: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Slot>>, DAE::Const, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSlotLst: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut outTypesConstLst: DAE::Const = DAE::Const::C_CONST;
    let mut outPolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    (outCache, outSlotLst, outTypesConstLst, outPolymorphicBindings) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inNamedArg.clone(), inTypesFuncArgLst.clone(), inSlotLst.clone(), onlyOneFunction.clone(), checkTypes.clone(), inPolymorphicBindings.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argValue: e, argName: id }, farg, slots, _, true, polymorphicBindings, pre) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut pr: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
                    let mut slots_1: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut ct: DAE::CodeType = DAE::CodeType::C_EXPRESSION;
                    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(findNamedArgType((id.clone()).clone(), farg.clone())?) {
                        __pa1 @ Deref @ DAE::Type::T_CODE { ty: __pa0 } => (__pa1.clone(), __pa0.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ct = __pa0.clone();
                    vt = __pa1.clone();
                    pr = findNamedArgParallelism((id.clone()).clone(), farg.clone())?;
                    e_1 = elabCodeExp(e.clone(), cache.clone(), env.clone(), ct.clone(), info.clone())?;
                    slots_1 = fillSlot(Arc::new(DAE::FuncArg { name: (id.clone()).clone(), ty: vt.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: pr.clone(), defaultBinding: None }), e_1.clone(), metamodelica::nil(), slots.clone(), pre.clone(), info.clone(), path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), openmodelica_frontend_types::DAE::Const::C_VAR, polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argValue: e, argName: id }, farg, slots, _, true, polymorphicBindings, pre) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut pr: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
                    let mut slots_1: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    vt = findNamedArgType((id.clone()).clone(), farg.clone())?;
                    pr = findNamedArgParallelism((id.clone()).clone(), farg.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e_1 = __pa1.clone();
                    t = __pa2.clone();
                    c1 = __pa3.clone();
                    (e_2, _, polymorphicBindings) = Types::matchTypePolymorphic(e_1.clone(), t.clone(), vt.clone(), FGraph::getGraphPathNoImplicitScope(env.clone())?, polymorphicBindings.clone(), false)?;
                    slots_1 = fillSlot(Arc::new(DAE::FuncArg { name: (id.clone()).clone(), ty: vt.clone(), r#const: c1.clone(), par: pr.clone(), defaultBinding: None }), e_2.clone(), metamodelica::nil(), slots.clone(), pre.clone(), info.clone(), path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1.clone(), polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argValue: e, argName: id }, farg, slots, _, true, polymorphicBindings, pre) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut pr: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
                    let mut slots_1: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut ds: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    vt = findNamedArgType((id.clone()).clone(), farg.clone())?;
                    pr = findNamedArgParallelism((id.clone()).clone(), farg.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e_1 = __pa1.clone();
                    t = __pa2.clone();
                    c1 = __pa3.clone();
                    (e_2, _, ds, polymorphicBindings) = Types::vectorizableType(e_1.clone(), t.clone(), vt.clone(), FGraph::getGraphPathNoImplicitScope(env.clone())?)?;
                    slots_1 = fillSlot(Arc::new(DAE::FuncArg { name: (id.clone()).clone(), ty: vt.clone(), r#const: c1.clone(), par: pr.clone(), defaultBinding: None }), e_2.clone(), ds.clone(), slots.clone(), pre.clone(), info.clone(), path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1.clone(), polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argValue: e, argName: id }, farg, slots, _, false, polymorphicBindings, pre) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c1: DAE::Const = DAE::Const::C_CONST;
                    let mut pr: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
                    let mut slots_1: Arc<metamodelica::List<Slot>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    vt = findNamedArgType((id.clone()).clone(), farg.clone())?;
                    pr = findNamedArgParallelism((id.clone()).clone(), farg.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: _, constFlag: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e_1 = __pa1.clone();
                    c1 = __pa2.clone();
                    slots_1 = fillSlot(Arc::new(DAE::FuncArg { name: (id.clone()).clone(), ty: vt.clone(), r#const: c1.clone(), par: pr.clone(), defaultBinding: None }), e_1.clone(), metamodelica::nil(), slots.clone(), pre.clone(), info.clone(), path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1.clone(), polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::NamedArg { argName: id, .. }, farg, slots, true, _, polymorphicBindings, _) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        unwrap_break_err!(findNamedArgType((id.clone()).clone(), farg.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s1 = AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?;
                    Error::addSourceMessage(Error::NO_SUCH_PARAMETER.clone(), list![(s1.clone()).clone(), (id.clone()).clone()], info.clone())?;
                    let true = (isGraphicsExp.clone()) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), slots.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argValue: e, argName: id }, farg, _, true, true, _, pre) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop: DAE::Properties;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let true = (Error::getNumErrorMessages() == numErrors.clone()) else { bail!("pattern mismatch") };
                    vt = findNamedArgType((id.clone()).clone(), farg.clone())?;
                    (cache, e_1, prop) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    s1 = AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?;
                    s2 = (ExpressionBasics::printExpStr(e_1.clone())?).clone();
                    s3 = (TypesDump::unparseTypeNoAttr(Types::getPropType(prop.clone())?)?).clone();
                    s4 = (TypesDump::unparseTypeNoAttr(vt.clone())?).clone();
                    Error::addSourceMessage(Error::NAMED_ARG_TYPE_MISMATCH.clone(), list![(s1.clone()).clone(), (id.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (s4.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outSlotLst, outTypesConstLst, outPolymorphicBindings))
}

fn findNamedArg(mut inIdent: ArcStr, mut inArgs: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Arc<DAE::FuncArg>> {
    let mut outArg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    let mut id: ArcStr = arcstr::literal!("");
    let mut haveMM: bool = Config::acceptMetaModelicaGrammar()?;
    let mut inIdent2: ArcStr = if (haveMM.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$in_")); __mm_s.push_str(&*inIdent.clone()); ArcStr::from(__mm_s) }} else {literal!("")};
    for mut arg in &*inArgs.clone() {
        let mut arg = arg.clone();
        let __pa0 = ::match_deref::match_deref! { match &(arg.clone()) {
            Deref @ DAE::FuncArg { name: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        id = __pa0.clone();
        if id.clone() == inIdent.clone() || haveMM.clone() && id.clone() == inIdent2.clone() {
            outArg = arg.clone();
            return Ok(outArg.clone());
        }
    }
    bail!("fail");
    Ok(outArg)
}

fn findNamedArgType(mut inIdent: ArcStr, mut inArgs: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(findNamedArg((inIdent.clone()).clone(), inArgs.clone())?) {
        Deref @ DAE::FuncArg { ty: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outType = __pa0.clone();
    Ok(outType)
}

fn findNamedArgParallelism(mut inIdent: ArcStr, mut inArgs: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<DAE::VarParallelism> {
    let mut outParallelism: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    let __pa0 = ::match_deref::match_deref! { match &(findNamedArg((inIdent.clone()).clone(), inArgs.clone())?) {
        Deref @ DAE::FuncArg { par: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outParallelism = __pa0.clone();
    Ok(outParallelism)
}

fn fillSlot(mut inFuncArg: Arc<DAE::FuncArg>, mut inExp: Arc<DAE::Exp>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inSlotLst: Arc<metamodelica::List<Slot>>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut r#fn: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Slot>>> {
    let mut outSlotLst: Arc<metamodelica::List<Slot>> = metamodelica::nil();
    let mut fa1: ArcStr = arcstr::literal!("");
    let mut fa2: ArcStr = arcstr::literal!("");
    let mut exp_str: ArcStr = arcstr::literal!("");
    let mut c_str: ArcStr = arcstr::literal!("");
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c1: DAE::Const = DAE::Const::C_CONST;
    let mut c2: DAE::Const = DAE::Const::C_CONST;
    let mut prl: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    let mut binding: Option<Arc<DAE::Exp>> = None;
    let mut filled: bool = false;
    let mut idx: i32 = 0;
    let mut ses: i32 = 0;
    let mut slot: Slot = <Slot as ::std::default::Default>::default();
    let mut rest_slots: Arc<metamodelica::List<Slot>> = inSlotLst.clone();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inFuncArg.clone()) {
        Deref @ DAE::FuncArg { r#const: __pa0, ty: __pa1, name: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    c1 = __pa0.clone();
    ty1 = __pa1.clone();
    fa1 = __pa2.clone();
    while !(rest_slots.clone().is_empty()) {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(rest_slots.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        slot = __pa3.clone();
        rest_slots = __pa4.clone();
        let Slot { defaultArg: __t6, .. } = (slot.clone()) else { bail!("pattern mismatch") };
        let __pa5 = ::match_deref::match_deref! { match &(__t6.clone()) {
            Deref @ DAE::FuncArg { name: __pa5, .. } => __pa5.clone(),
            _ => bail!("pattern mismatch"),
        } };
        fa2 = __pa5.clone();
        if stringEq((fa1.clone()).clone(), (fa2.clone()).clone()) || stringEq(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$in_")); __mm_s.push_str(&*fa1.clone()); ArcStr::from(__mm_s) }).clone(), (fa2.clone()).clone()) {
            let Slot { evalStatus: __pa7, idx: __pa8, slotFilled: __pa9, defaultArg: __t13, .. } = (slot.clone()) else { bail!("pattern mismatch") };
            let (__pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(__t13.clone()) {
                Deref @ DAE::FuncArg { defaultBinding: __pa10, par: __pa11, r#const: __pa12, .. } => (__pa10.clone(), __pa11.clone(), __pa12.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ses = __pa7.clone();
            idx = __pa8.clone();
            filled = __pa9.clone();
            binding = __pa10.clone();
            prl = __pa11.clone();
            c2 = __pa12.clone();
            if filled.clone() {
                pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
                Error::addSourceMessageAndFail(Error::FUNCTION_SLOT_ALREADY_FILLED.clone(), list![(fa2.clone()).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if !(Types::constEqualOrHigher(c1.clone(), c2.clone())) {
                exp_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                c_str = (TypesDump::unparseConst(c2.clone())?).clone();
                Error::addSourceMessageAndFail(Error::FUNCTION_SLOT_VARIABILITY.clone(), list![(fa1.clone()).clone(), (exp_str.clone()).clone(), AbsynUtil::pathStringNoQual(r#fn.clone(), (literal!(".")).clone(), true, false)?, (TypesDump::unparseConst(c1.clone())?).clone(), (c_str.clone()).clone()], inInfo.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            slot = Slot { defaultArg: Arc::new(DAE::FuncArg { name: (fa2.clone()).clone(), ty: ty1.clone(), r#const: c2.clone(), par: prl.clone(), defaultBinding: binding.clone() }), slotFilled: true, arg: Some(inExp.clone()), dims: inDims.clone(), idx: idx.clone(), evalStatus: ses.clone() };
            outSlotLst = List::append_reverse(outSlotLst.clone(), cons(slot.clone(), rest_slots.clone()));
            return Ok(outSlotLst.clone());
        }
        outSlotLst = cons(slot.clone(), outSlotLst.clone());
    }
    Error::addSourceMessageAndFail(Error::NO_SUCH_PARAMETER.clone(), list![(literal!("")).clone(), (fa1.clone()).clone()], inInfo.clone())?;
    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    Ok(outSlotLst)
}

pub fn elabCref(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut inImplicit: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut res: Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> = None;
    (outCache, res) = elabCref1(inCache.clone(), inEnv.clone(), inComponentRef.clone(), inImplicit.clone(), performVectorization.clone(), inPrefix.clone(), true, info.clone())?;
    Ok((outCache, res))
}

pub fn elabCrefNoEval(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut inImplicit: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut outAttributes: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabCref1(inCache.clone(), inEnv.clone(), inComponentRef.clone(), inImplicit.clone(), performVectorization.clone(), inPrefix.clone(), false, info.clone())?) {
        (__pa0, Some((__pa1, __pa2, __pa3))) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    outExp = __pa1.clone();
    outProperties = __pa2.clone();
    outAttributes = __pa3.clone();
    Ok((outCache, outExp, outProperties, outAttributes))
}

fn elabCref1(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut inImplicit: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut evalCref: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut res: Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> = None;
    (outCache, res) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), inImplicit.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::WILD, _, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = DAE::T_ANYTYPE_DEFAULT().clone();
                    et = Types::simplifyType(t.clone())?;
                    crefExp = Expression::makeCrefExp(Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD), et.clone())?;
                    Ok((cache.clone(), Some((crefExp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, DAE::dummyAttrVar().clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "Boolean", .. }, _, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    exp = Expression::makeScalarArray(list![Arc::new(DAE::Exp::BCONST { bool: false }), Arc::new(DAE::Exp::BCONST { bool: true })], DAE::T_BOOL_DEFAULT().clone());
                    t = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 })] });
                    Ok((cache.clone(), Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }, DAE::dummyAttrConst().clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "time", .. }, _, _) => {
                    let mut res: Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> = res.clone();
                    res = if (isValidTimeScope(inEnv.clone(), info.clone())?) {BUILTIN_TIME().clone()} else {None};
                    Ok((inCache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, r#impl, pre) => {
                    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut stripped_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut cache = (*cache).clone();
                    let mut res: Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> = res.clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::crefHasSubscripts(inComponentRef.clone())) else { bail!("pattern mismatch") };
                    subscripts = AbsynUtil::crefGetLastSubs(inComponentRef.clone())?;
                    stripped_cref = AbsynUtil::crefStripLastSubs(inComponentRef.clone())?;
                    let true = (!(AbsynUtil::crefHasSubscripts(stripped_cref.clone())) && (subscripts.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(subscripts.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: __pa0 }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    (cache, res) = elabCrefArraySubscripts(stripped_cref.clone(), e.clone(), cache.clone(), env.clone(), pre.clone(), evalCref.clone(), r#impl.clone(), info.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e }, tail: Deref @ metamodelica::List::Nil }, name: id }, r#impl, pre) => {
                    let mut cache = (*cache).clone();
                    let mut res: Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> = res.clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    (cache, res) = elabCrefArraySubscripts(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() }), e.clone(), cache.clone(), env.clone(), pre.clone(), evalCref.clone(), r#impl.clone(), info.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, r#impl, pre) => {
                    let mut c_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut constSubs: DAE::Const = DAE::Const::C_CONST;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    let mut hasZeroSizeDim: bool = false;
                    let mut splicedExpData: InstTypes::SplicedExpData = <InstTypes::SplicedExpData as ::std::default::Default>::default();
                    let mut forIteratorConstOpt: Option<DAE::Const> = None;
                    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut c = (*c).clone();
                    c = replaceEnd(c.clone())?;
                    env = if (AbsynUtil::crefIsFullyQualified(inComponentRef.clone())) {FGraph::topScope(inEnv.clone())?} else {inEnv.clone()};
                    (cache, c_1, constSubs, hasZeroSizeDim) = elabCrefSubs(cache.clone(), env.clone(), inEnv.clone(), c.clone(), pre.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#impl.clone(), false, info.clone())?;
                    (cache, attr, t, binding, forIteratorConstOpt, splicedExpData, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), c_1.clone())?;
                    (cache, exp, r#const, attr) = elabCref2(cache.clone(), env.clone(), c_1.clone(), attr.clone(), constSubs.clone(), forIteratorConstOpt.clone(), t.clone(), binding.clone(), performVectorization.clone(), splicedExpData.clone(), pre.clone(), evalCref.clone(), info.clone())?;
                    t = fixEnumerationType(t.clone())?;
                    (exp, r#const) = evaluateEmptyVariable(hasZeroSizeDim.clone() && evalCref.clone(), exp.clone(), t.clone(), r#const.clone())?;
                    Ok((cache.clone(), Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: r#const.clone() }, attr.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, _, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    let mut path: Arc<Absyn::Path>;
                    let mut enum_lit_strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut typeStr: ArcStr = arcstr::literal!("");
                    let mut cl: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut c = (*c).clone();
                    c = replaceEnd(c.clone())?;
                    path = AbsynUtil::crefToPath(c.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), None)?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    cl = __pa1.clone();
                    env = __pa2.clone();
                    typeStr = (AbsynUtil::pathLastIdent(path.clone())?).clone();
                    path = FGraph::joinScopePath(env.clone(), Arc::new(Absyn::Path::IDENT { name: (typeStr.clone()).clone() }))?;
                    enum_lit_strs = SCodeUtil::componentNames(cl.clone());
                    (exp, t) = makeEnumerationArray(path.clone(), enum_lit_strs.clone());
                    Ok((cache.clone(), Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }, DAE::dummyAttrConst().clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, _, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut origt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    let mut isBuiltinFn: bool = false;
                    let mut isBuiltinFnOrInlineBuiltin: bool = false;
                    let mut path: Arc<Absyn::Path>;
                    let mut fpath: Arc<Absyn::Path>;
                    let mut expCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut isBuiltin: DAE::FunctionBuiltin = DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR;
                    let mut cache = (*cache).clone();
                    let mut c = (*c).clone();
                    path = AbsynUtil::crefToPath(c.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupFunctionsInEnvNoError(cache.clone(), env.clone(), path.clone(), info.clone())?) {
                        (__pa0, Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    t = __pa1.clone();
                    (isBuiltin, isBuiltinFn, path) = isBuiltinFunc(path.clone(), t.clone())?;
                    isBuiltinFnOrInlineBuiltin = !(openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN == isBuiltin.clone());
                    fpath = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ DAE::Type::T_FUNCTION { .. } => var_field!((*t).path, DAE::Type::T_FUNCTION).clone(),
        _ => bail!("match: no arm matched"),
    } });
                    origt = t.clone();
                    t = Types::makeFunctionPolymorphicReference(t.clone())?;
                    c = AbsynUtil::pathToCref(fpath.clone())?;
                    expCref = ComponentReference::toExpCref(c.clone())?;
                    exp = Expression::makeCrefExp(expCref.clone(), Arc::new(DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: isBuiltinFnOrInlineBuiltin.clone(), functionType: origt.clone() }))?;
                    let (__pa3, Util::SUCCESS { .. }) = (instantiateDaeFunction(cache.clone(), env.clone(), path.clone(), isBuiltinFn.clone(), None, true)?) else { bail!("pattern mismatch") };
                    cache = __pa3.clone();
                    Ok((cache.clone(), Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, DAE::dummyAttrConst().clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "NONE", subscripts: Deref @ metamodelica::List::Nil }, _, _) => {
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::META_NONE_CREF.clone(), metamodelica::nil(), info.clone())?;
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, c, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCref failed: ")); __mm_s.push_str(&*Dump::printComponentRefStr(c.clone())?); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, r#impl, pre) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut scope: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        unwrap_break_err!(elabCrefSubs(cache.clone(), env.clone(), env.clone(), c.clone(), pre.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#impl.clone(), false, info.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s = (Dump::printComponentRefStr(c.clone())?).clone();
                    scope = (FGraph::printGraphPathStr(env.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(s.clone()).clone(), (scope.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, res))
}

fn elabCrefArraySubscripts(mut cref: Arc<Absyn::ComponentRef>, mut e: Arc<Absyn::Exp>, mut cache: FCore::Cache, mut env: FCore::Graph, mut pre: DAE::Prefix, mut evalCref: bool, mut r#impl: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)>)> {
    let mut cache: FCore::Cache = cache;
    let mut res: Option<(Arc<DAE::Exp>, DAE::Properties, Arc<DAE::Attributes>)> = None;
    let mut exp: Arc<DAE::Exp>;
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    let mut const1: DAE::Const = DAE::Const::C_CONST;
    let mut const2: DAE::Const = DAE::Const::C_CONST;
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut sub_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(elabCref1(cache.clone(), env.clone(), cref.clone(), false, false, pre.clone(), evalCref.clone(), info.clone())?) {
        (__pa0, Some((__pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }, __pa4))) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa0.clone();
    exp1 = __pa1.clone();
    t = __pa2.clone();
    const1 = __pa3.clone();
    attr = __pa4.clone();
    t = Types::metaArrayElementType(t.clone())?;
    let (__pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
        (__pa5, __pa6, DAE::Properties::PROP { type_: __pa7, constFlag: __pa8 }) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa5.clone();
    exp2 = __pa6.clone();
    sub_ty = __pa7.clone();
    const2 = __pa8.clone();
    if Types::isMetaBoxedType(sub_ty.clone()) {
        sub_ty = Types::unboxedType(sub_ty.clone())?;
        exp2 = Arc::new(DAE::Exp::UNBOX { exp: exp2.clone(), ty: sub_ty.clone() });
    }
    let true = (Types::isScalarInteger(sub_ty.clone())) else { bail!("pattern mismatch") };
    r#const = Types::constAnd(const1.clone(), const2.clone());
    exp = Expression::makeASUB(exp1.clone(), list![exp2.clone()])?;
    res = Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: r#const.clone() }, attr.clone()));
    Ok((cache, res))
}

fn isValidTimeScope(mut inEnv: FCore::Graph, mut inInfo: SourceInfo) -> Result<bool> {
    let mut outIsValid: bool = false;
    let mut res: SCode::Restriction = SCode::Restriction::R_BLOCK;
    if let Ok(__iflet0) = FGraph::lastScopeRestriction(inEnv.clone()) {
        res = __iflet0;
    } else {
        outIsValid = true;
        return Ok(outIsValid.clone());
    }
    outIsValid = (match res.clone() {
        SCode::Restriction::R_CLASS => true,
        SCode::Restriction::R_OPTIMIZATION => true,
        SCode::Restriction::R_MODEL => true,
        SCode::Restriction::R_BLOCK => true,
        _ => {
            Error::addSourceMessage(Error::INVALID_TIME_SCOPE.clone(), metamodelica::nil(), inInfo.clone())?;
            false
        },
    });
    Ok(outIsValid)
}

fn lookupFunctionsInEnvNoError(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outTypesTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outCache, outTypesTypeLst) = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut outCache: FCore::Cache = outCache.clone();
            let mut outTypesTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = outTypesTypeLst.clone();
            ErrorExt::setCheckpoint((literal!("Static.lookupFunctionsInEnvNoError")).clone());
            (outCache, outTypesTypeLst) = Lookup::lookupFunctionsInEnv(inCache.clone(), inEnv.clone(), inPath.clone(), inInfo.clone())?;
            ErrorExt::rollBack((literal!("Static.lookupFunctionsInEnvNoError")).clone());
            Ok((outCache.clone(), outTypesTypeLst.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            ErrorExt::rollBack((literal!("Static.lookupFunctionsInEnvNoError")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outTypesTypeLst))
}

fn evaluateEmptyVariable(mut hasZeroSizeDim: bool, mut inExp: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>, mut c: DAE::Const) -> Result<(Arc<DAE::Exp>, DAE::Const)> {
    let mut oexp: Arc<DAE::Exp>;
    let mut oc: DAE::Const = DAE::Const::C_CONST;
    (oexp, oc) = 'mc: {
        let __mc_input = (hasZeroSizeDim.clone(), inExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ DAE::Exp::ASUB { sub: ss, .. }) => {
                    let mut sc: bool = false;
                    let mut a: bool = false;
                    let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exp: Arc<DAE::Exp>;
                    expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (ss.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    a = Types::isArray(ty.clone());
                    sc = boolNot(a.clone());
                    et = Types::simplifyType(ty.clone())?;
                    exp = Arc::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: sc.clone(), array: metamodelica::nil() });
                    exp = Expression::makeASUB(exp.clone(), expl.clone())?;
                    Ok((exp.clone(), c.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
                    let mut sc: bool = false;
                    let mut a: bool = false;
                    let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    a = Types::isArray(ty.clone());
                    sc = boolNot(a.clone());
                    et = Types::simplifyType(ty.clone())?;
                    ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(cr.clone())?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = Arc::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: sc.clone(), array: metamodelica::nil() });
                    Ok((exp.clone(), c.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
                    let mut sc: bool = false;
                    let mut a: bool = false;
                    let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut exp: Arc<DAE::Exp>;
                    a = Types::isArray(ty.clone());
                    sc = boolNot(a.clone());
                    et = Types::simplifyType(ty.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(cr.clone())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ss = __pa0.clone();
                    exp = Arc::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: sc.clone(), array: metamodelica::nil() });
                    exp = Expression::makeASUB(exp.clone(), List::map(ss.clone(), (std::sync::Arc::new(Expression::getSubscriptExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> + 'static>)))?;
                    Ok((exp.clone(), c.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), c.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oexp, oc))
}

pub fn fixEnumerationType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { attributeLst: al, literalVarLst: v, names: n, path: p, index: Some(_) } => {
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: n.clone(), literalVarLst: v.clone(), attributeLst: al.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub fn applySubscriptsVariability(mut inVariability: SCode::Variability, mut inSubsConst: DAE::Const) -> SCode::Variability {
    let mut outVariability: SCode::Variability = SCode::Variability::CONST;
    outVariability = (match (inVariability.clone(), inSubsConst.clone()) {
        (SCode::Variability::PARAM, DAE::Const::C_VAR) => openmodelica_frontend_types::SCode::Variability::VAR,
        (SCode::Variability::CONST, DAE::Const::C_VAR) => openmodelica_frontend_types::SCode::Variability::VAR,
        (SCode::Variability::CONST, DAE::Const::C_PARAM) => openmodelica_frontend_types::SCode::Variability::PARAM,
        _ => inVariability.clone(),
    });
    outVariability
}

pub fn makeEnumerationArray(mut enumTypeName: Arc<Absyn::Path>, mut enumLiterals: Arc<metamodelica::List<ArcStr>>) -> (Arc<DAE::Exp>, Arc<DAE::Type>) {
    let mut enumArray: Arc<DAE::Exp>;
    let mut enumArrayType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut enum_lit_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut sz: i32 = 0;
    let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    enum_lit_expl = Expression::makeEnumLiterals(enumTypeName.clone(), enumLiterals.clone());
    sz = (enumLiterals.clone().len() as i32);
    ety = Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ENUMERATION { index: None, path: enumTypeName.clone(), names: enumLiterals.clone(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), dims: list![Arc::new(DAE::Dimension::DIM_ENUM { enumTypeName: enumTypeName.clone(), literals: enumLiterals.clone(), size: sz.clone() })] });
    enumArray = Arc::new(DAE::Exp::ARRAY { ty: ety.clone(), scalar: true, array: enum_lit_expl.clone() });
    enumArrayType = ety.clone();
    (enumArray, enumArrayType)
}

fn fillCrefSubscripts(mut inComponentRef: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outComponentRef = 'mc: {
        let __mc_input = (inComponentRef.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, .. }, _) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, identType: ty2, ident: id }, t) => {
                    let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    subs_1 = fillSubscripts(subs.clone(), t.clone())?;
                    Ok(ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty2.clone(), subs_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { identType: ty2, componentRef: cref, subscriptLst: subs, ident: id }, t) => {
                    let mut cref_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut subs = (*subs).clone();
                    let mut t = (*t).clone();
                    subs = fillSubscripts(subs.clone(), ty2.clone())?;
                    t = stripPrefixType(t.clone(), ty2.clone());
                    cref_1 = fillCrefSubscripts(cref.clone(), t.clone())?;
                    Ok(ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty2.clone(), subs.clone(), cref_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComponentRef)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn stripPrefixType(mut inType: Arc<DAE::Type>, mut inPrefixType: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inType.clone(), inPrefixType.clone())) {
        (Deref @ DAE::Type::T_ARRAY { ty: t, .. }, Deref @ DAE::Type::T_ARRAY { ty: pt, .. }) => {
            stripPrefixType(t.clone(), pt.clone())
        },
        _ => {
            inType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

fn fillSubscripts(mut inExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    outExpSubscriptLst = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { .. } => {
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    subs = List::fill(Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM), (TypesDump::getDimensions(inType.clone()).len() as i32));
                    subs = List::stripN(subs.clone(), (inExpSubscriptLst.clone().len() as i32))?;
                    subs = listAppend(inExpSubscriptLst.clone(), subs.clone());
                    Ok(subs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExpSubscriptLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpSubscriptLst)
}

fn elabCref2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCref: Arc<DAE::ComponentRef>, mut inAttributes: Arc<DAE::Attributes>, mut constSubs: DAE::Const, mut inIteratorConst: Option<DAE::Const>, mut inType: Arc<DAE::Type>, mut inBinding: Arc<DAE::Binding>, mut inVectorize: bool, mut splicedExpData: InstTypes::SplicedExpData, mut inPrefix: DAE::Prefix, mut evalCref: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Const, Arc<DAE::Attributes>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    let mut outAttributes: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    let mut var: SCode::Variability = DAEUtil::getAttrVariability(inAttributes.clone());
    (outExp, outConst, outAttributes) = 'mc: {
        let __mc_input = (var.clone(), inType.clone(), inBinding.clone(), splicedExpData.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_UNKNOWN, _, _) => {
                    let mut expTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    expTy = Types::simplifyType(inType.clone())?;
                    r#const = Types::variabilityToConst(var.clone())?;
                    Ok((Arc::new(DAE::Exp::CREF { componentRef: inCref.clone(), ty: expTy.clone() }), r#const.clone(), inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::PARAM, _, Deref @ DAE::Binding::EQBOUND { source: DAE::BindingSource::BINDING_FROM_START_VALUE, .. }, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
                    let mut outCache: FCore::Cache = outCache.clone();
                    let true = (Types::getFixedVarAttributeParameterOrConstant(inType.clone())) else { bail!("pattern mismatch") };
                    binding = DAEUtil::setBindingSource(inBinding.clone(), openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE)?;
                    (outCache, e, r#const, attr) = elabCref2(outCache.clone(), inEnv.clone(), inCref.clone(), inAttributes.clone(), constSubs.clone(), inIteratorConst.clone(), inType.clone(), binding.clone(), inVectorize.clone(), splicedExpData.clone(), inPrefix.clone(), evalCref.clone(), info.clone())?;
                    Ok((e.clone(), r#const.clone(), attr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST, Deref @ DAE::Type::T_ENUMERATION { path: p, index: Some(i), .. }, _, _) => {
                    if !((evalCref.clone())) { bail!("guard") }
                    let mut p = (*p).clone();
                    p = AbsynUtil::joinPaths(p.clone(), ComponentReference::crefLastPath(inCref.clone())?)?;
                    Ok((Arc::new(DAE::Exp::ENUM_LITERAL { name: p.clone(), index: i.clone() }), openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST, _, _, _) => {
                    if !((!(evalCref.clone()))) { bail!("guard") }
                    let mut expTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    expTy = Types::simplifyType(inType.clone())?;
                    Ok((Expression::makeCrefExp(inCref.clone(), expTy.clone())?, openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST, _, _, InstTypes::SplicedExpData { .. }) => {
                    if !((Types::isVar(constSubs.clone()))) { bail!("guard") }
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut subsc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut outCache: FCore::Cache = outCache.clone();
                    cr = ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?;
                    subsc = ComponentReference::crefLastSubs(inCref.clone())?;
                    (outCache, v) = Ceval::cevalCref(outCache.clone(), inEnv.clone(), cr.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    e = ValuesUtil::valueExp(v.clone(), None)?;
                    e = Expression::makeASUB(e.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subsc.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                    Ok((e.clone(), openmodelica_frontend_types::DAE::Const::C_VAR, inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST, _, binding, InstTypes::SplicedExpData { splicedExp: _, identType: idTy }) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let true = (Types::equivtypes(inType.clone(), idTy.clone())?) else { bail!("pattern mismatch") };
                    match '__try0: {
                        (outCache, v) = unwrap_break_err!(Ceval::cevalCrefBinding(outCache.clone(), inEnv.clone(), inCref.clone(), binding.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0), '__try0);
                        e = unwrap_break_err!(ValuesUtil::valueExp(v.clone(), None), '__try0);
                        Ok::<_, anyhow::Error>((e.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            e = __try0_o0;
                        }
                        Err(_) => {
                            let __pa1 = ::match_deref::match_deref! { match &(DAEUtil::bindingExp(binding.clone())?) {
                                        Some(__pa1) => __pa1.clone(),
                                        _ => bail!("pattern mismatch"),
                            } };
                            e = __pa1.clone();
                            e = Expression::makeASUB(e.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (ComponentReference::crefLastSubs(inCref.clone())?).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                        }
                    }
                    r#const = openmodelica_frontend_types::DAE::Const::C_CONST;
                    Ok((e.clone(), r#const.clone(), inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST, _, _, _) => {
                    if !((isSome(inIteratorConst.clone()))) { bail!("guard") }
                    let mut expTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    expTy = Types::simplifyType(inType.clone())?;
                    Ok((Expression::makeCrefExp(inCref.clone(), expTy.clone())?, openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST, _, Deref @ DAE::Binding::EQBOUND { constant_: DAE::Const::C_CONST, .. }, InstTypes::SplicedExpData { splicedExp: sexp, identType: idTy }) => {
                    let mut expTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut expIdTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    expTy = Types::simplifyType(inType.clone())?;
                    expIdTy = Types::simplifyType(idTy.clone())?;
                    cr = fillCrefSubscripts(inCref.clone(), inType.clone())?;
                    e = Expression::makeCrefExp(cr.clone(), expTy.clone())?;
                    e = crefVectorize(inVectorize.clone(), e.clone(), inType.clone(), sexp.clone(), expIdTy.clone())?;
                    (outCache, v) = Ceval::ceval(outCache.clone(), inEnv.clone(), e.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    e = ValuesUtil::valueExp(v.clone(), Some(e.clone()))?;
                    Ok((e.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::PARAM, _, _, InstTypes::SplicedExpData { splicedExp: sexp, identType: idTy }) => {
                    if !((DAEUtil::isBound(inBinding.clone()))) { bail!("guard") }
                    let mut expTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut expIdTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
                    let mut outCache: FCore::Cache = outCache.clone();
                    let true = (Flags::isSet(Flags::EVAL_PARAM.clone())? || Config::getEvaluateParametersInAnnotations()?) else { bail!("pattern mismatch") };
                    attr = DAEUtil::setAttrVariability(inAttributes.clone(), openmodelica_frontend_types::SCode::Variability::CONST);
                    expTy = Types::simplifyType(inType.clone())?;
                    expIdTy = Types::simplifyType(idTy.clone())?;
                    cr = fillCrefSubscripts(inCref.clone(), inType.clone())?;
                    e = crefVectorize(inVectorize.clone(), Expression::makeCrefExp(cr.clone(), expTy.clone())?, inType.clone(), sexp.clone(), expIdTy.clone())?;
                    (outCache, v) = Ceval::ceval(outCache.clone(), inEnv.clone(), e.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    e = ValuesUtil::valueExp(v.clone(), Some(e.clone()))?;
                    Ok((e.clone(), openmodelica_frontend_types::DAE::Const::C_PARAM, attr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST, _, Deref @ DAE::Binding::EQBOUND { constant_: DAE::Const::C_CONST, evaluatedExp: Some(v), .. }, InstTypes::SplicedExpData { splicedExp: Some(Deref @ DAE::Exp::CREF { componentRef: cr, .. }), identType: _ }) => {
                    let mut subCr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut subCr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut index: Arc<DAE::Exp>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(cr.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: __pa1 }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    subCr2 = __pa0.clone();
                    e = __pa1.clone();
                    let (__pa5, __pa4) = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(inCref.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: __pa5 @ Deref @ DAE::Exp::CREF { componentRef: __pa4, .. } }, tail: Deref @ metamodelica::List::Nil } => (__pa5.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    subCr1 = __pa4.clone();
                    index = __pa5.clone();
                    let true = (ComponentReferenceBasics::crefEqual(subCr1.clone(), subCr2.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::isArray(e.clone()) || Expression::isRange(e.clone())) else { bail!("pattern mismatch") };
                    e = ValuesUtil::valueExp(v.clone(), Some(e.clone()))?;
                    e = Arc::new(DAE::Exp::ASUB { exp: e.clone(), sub: list![Arc::new(DAE::Subscript::INDEX { exp: index.clone() })] });
                    Ok((e.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST, _, Deref @ DAE::Binding::UNBOUND, _) => {
                    if !((isNone(inIteratorConst.clone()))) { bail!("guard") }
                    let mut expTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut scope: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    if Flags::isSet(Flags::STATIC.clone())? {
                        s = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
                        scope = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                        pre_str = (PrefixUtil::printPrefixStr2(inPrefix.clone())?).clone();
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*pre_str.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCref2 failed on: ")); __mm_s.push_str(&*pre_str.clone()); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" with no constant binding in scope: ")); __mm_s.push_str(&*scope.clone()); ArcStr::from(__mm_s) }).clone())?;
                    }
                    expTy = Types::simplifyType(inType.clone())?;
                    cr = fillCrefSubscripts(inCref.clone(), inType.clone())?;
                    e = Expression::makeCrefExp(cr.clone(), expTy.clone())?;
                    Ok((e.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, InstTypes::SplicedExpData { splicedExp: sexp, identType: idTy }) => {
                    let mut expTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut expIdTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    expTy = Types::simplifyType(inType.clone())?;
                    expIdTy = Types::simplifyType(idTy.clone())?;
                    cr = fillCrefSubscripts(inCref.clone(), inType.clone())?;
                    e = crefVectorize(inVectorize.clone(), Expression::makeCrefExp(cr.clone(), expTy.clone())?, inType.clone(), sexp.clone(), expIdTy.clone())?;
                    r#const = Types::variabilityToConst(var.clone())?;
                    Ok((e.clone(), r#const.clone(), inAttributes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    pre_str = (PrefixUtil::printPrefixStr2(inPrefix.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCref2 failed for: ")); __mm_s.push_str(&*pre_str.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*literal!("\n env:")); __mm_s.push_str(&*FGraph::printGraphStr(inEnv.clone())); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outConst, outAttributes))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefVectorize(mut performVectorization: bool, mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut splicedExp: Option<Arc<DAE::Exp>>, mut crefIdType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (performVectorization.clone(), inExp.clone(), inType.clone(), splicedExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, e, _, _) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. }, _) => {
                    let mut e = (*e).clone();
                    e = crefVectorize(true, e.clone(), t.clone(), None, crefIdType.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: d2, tail: Deref @ metamodelica::List::Nil }, .. }, dims: Deref @ metamodelica::List::Cons { head: d1, tail: Deref @ metamodelica::List::Nil } }, Some(Deref @ DAE::Exp::CREF { componentRef: cr, .. })) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut e: Arc<DAE::Exp>;
                    b1 = Expression::dimensionSize(d1.clone())? < Config::vectorizationLimit()?;
                    b2 = Expression::dimensionSize(d2.clone())? < Config::vectorizationLimit()?;
                    let true = (boolAnd(b1.clone(), b2.clone()) || Config::vectorizationLimit()? == 0) else { bail!("pattern mismatch") };
                    e = elabCrefSlice(cr.clone(), crefIdType.clone())?;
                    e = elabMatrixToMatrixExp(e.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: d1, tail: Deref @ metamodelica::List::Nil } }, Some(Deref @ DAE::Exp::CREF { componentRef: cr, .. })) => {
                    let mut e: Arc<DAE::Exp>;
                    let false = (Types::isArray(t.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::dimensionSize(d1.clone())? < Config::vectorizationLimit()? || Config::vectorizationLimit()? == 0) else { bail!("pattern mismatch") };
                    e = elabCrefSlice(cr.clone(), crefIdType.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { ty: exptp, componentRef: cr }, Deref @ DAE::Type::T_ARRAY { ty: t @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: d2, tail: Deref @ metamodelica::List::Nil }, .. }, dims: Deref @ metamodelica::List::Cons { head: d1, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut e: Arc<DAE::Exp>;
                    let mut ds: i32 = 0;
                    let mut ds2: i32 = 0;
                    ds = Expression::dimensionSize(d1.clone())?;
                    ds2 = Expression::dimensionSize(d2.clone())?;
                    b1 = ds.clone() < Config::vectorizationLimit()?;
                    b2 = ds2.clone() < Config::vectorizationLimit()?;
                    let true = (boolAnd(b1.clone(), b2.clone()) || Config::vectorizationLimit()? == 0) else { bail!("pattern mismatch") };
                    let true = (ComponentReference::crefLastSubs(cr.clone())?.is_empty()) else { bail!("pattern mismatch") };
                    e = createCrefArray2d(cr.clone(), 1, ds.clone(), ds2.clone(), exptp.clone(), t.clone(), crefIdType.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { ty: exptp, componentRef: cr }, Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: d1, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut ds: i32 = 0;
                    let false = (Types::isArray(t.clone())) else { bail!("pattern mismatch") };
                    ds = Expression::dimensionSize(d1.clone())?;
                    let true = (ds.clone() < Config::vectorizationLimit()? || Config::vectorizationLimit()? == 0) else { bail!("pattern mismatch") };
                    e = createCrefArray(cr.clone(), 1, ds.clone(), exptp.clone(), t.clone(), crefIdType.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn extractDimensionOfChild(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::Dimension>>>, bool)> {
    let mut outExp: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut isScalar: bool = false;
    (outExp, isScalar) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { scalar: sc, ty: Deref @ DAE::Type::T_ARRAY { dims: tl, .. }, .. } => {
                    Ok((tl.clone(), sc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl1 @ Deref @ metamodelica::List::Cons { head: exp2 @ Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: _ }, tail: _ }, .. } => {
                    let mut tl: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut x: i32 = 0;
                    (tl, _) = extractDimensionOfChild(exp2.clone())?;
                    x = (expl1.clone().len() as i32);
                    Ok((cons(Arc::new(DAE::Dimension::DIM_INTEGER { integer: x.clone() }), tl.clone()), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl1, .. } => {
                    let mut x: i32 = 0;
                    x = (expl1.clone().len() as i32);
                    Ok((list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: x.clone() })], true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => {
                    Ok((metamodelica::nil(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, isScalar))
}

fn elabCrefSlice(mut inCref: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outCref: Arc<DAE::Exp>;
    outCref = (::match_deref::match_deref! { match &((inCref.clone(), inType.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: ssl, ident: id, .. }, ety) => {
            let mut exp1: Arc<DAE::Exp>;
            exp1 = flattenSubscript(ssl.clone(), (id.clone()).clone(), ety.clone())?;
            exp1.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: child, subscriptLst: ssl, identType: prety, ident: id }, ety) => {
            let mut exp1: Arc<DAE::Exp>;
            let mut childExp: Arc<DAE::Exp>;
            childExp = elabCrefSlice(child.clone(), ety.clone())?;
            exp1 = flattenSubscript(ssl.clone(), (id.clone()).clone(), prety.clone())?;
            exp1 = mergeQualWithRest(exp1.clone(), childExp.clone(), ety.clone())?;
            exp1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

fn mergeQualWithRest(mut qual: Arc<DAE::Exp>, mut rest: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((qual.clone(), rest.clone(), inType.clone())) {
        (exp1 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, exp2, _) => {
            mergeQualWithRest2(exp2.clone(), exp1.clone())?
        },
        (Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 }, exp2, ety) => {
            let mut iLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut scalar: bool = false;
            let mut expl1 = (*expl1).clone();
            let mut exp2 = (*exp2).clone();
            let mut ety = (*ety).clone();
            expl1 = List::map2(expl1.clone(), (std::sync::Arc::new(mergeQualWithRest) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), exp2.clone(), ety.clone());
            exp2 = Arc::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl1.clone() });
            (iLst, scalar) = extractDimensionOfChild(exp2.clone())?;
            ety = Expression::arrayEltType(ety.clone());
            exp2 = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: iLst.clone() }), scalar: scalar.clone(), array: expl1.clone() });
            exp2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn mergeQualWithRest2(mut rest: Arc<DAE::Exp>, mut qual: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((rest.clone(), qual.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cref, ty: ety }, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty2, subscriptLst: ssl }, ty: _ }) => {
            let mut cref_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref_2 = ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty2.clone(), ssl.clone(), cref.clone());
            Expression::makeCrefExp(cref_2.clone(), ety.clone())?
        },
        (exp1 @ Deref @ DAE::Exp::ARRAY { ty: ety, scalar: _, array: expl1 }, exp2 @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, ty: _ }) => {
            let mut scalar: bool = false;
            let mut exp1 = (*exp1).clone();
            let mut expl1 = (*expl1).clone();
            expl1 = List::map1(expl1.clone(), (std::sync::Arc::new(mergeQualWithRest2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), exp2.clone());
            exp1 = Arc::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl1.clone() });
            (_, scalar) = extractDimensionOfChild(exp1.clone())?;
            Arc::new(DAE::Exp::ARRAY { ty: ety.clone(), scalar: scalar.clone(), array: expl1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn flattenSubscript(mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut name: ArcStr, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inSubs.clone(), name.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, id, ety) => {
                    let mut exp1: Arc<DAE::Exp>;
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cref_ = ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ety.clone(), metamodelica::nil());
                    exp1 = Expression::makeCrefExp(cref_.clone(), ety.clone())?;
                    Ok(exp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (subs1, id, ety) => {
                    let mut exp2: Arc<DAE::Exp>;
                    exp2 = flattenSubscript2(subs1.clone(), (id.clone()).clone(), ety.clone())?;
                    Ok(exp2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

// BZ(2010-01-29): Changed to public to be able to vectorize crefs from other places
pub fn flattenSubscript2(mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut name: ArcStr, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inSubs.clone(), name.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), scalar: false, array: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: exp1 @ Deref @ DAE::Exp::ICONST { integer: _ } }, tail: subs1 }, id, ety) => {
                    let mut exp2: Arc<DAE::Exp>;
                    exp2 = flattenSubscript2(subs1.clone(), (id.clone()).clone(), ety.clone())?;
                    exp2 = applySubscript(exp1.clone(), exp2.clone(), (id.clone()).clone(), Expression::unliftArray(ety.clone())?)?;
                    Ok(exp2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 @ Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: 0 }, tail: Deref @ metamodelica::List::Nil } } }, tail: subs1 }, id, ety) => {
                    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exp2: Arc<DAE::Exp>;
                    let mut exp3: Arc<DAE::Exp>;
                    exp2 = flattenSubscript2(subs1.clone(), (id.clone()).clone(), ety.clone())?;
                    expl2 = List::map3(expl1.clone(), (std::sync::Arc::new(applySubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), exp2.clone(), (id.clone()).clone(), ety.clone());
                    exp3 = listHead(expl2.clone())?;
                    Ok(exp3.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 } }, tail: subs1 }, id, ety) => {
                    let mut exp2: Arc<DAE::Exp>;
                    exp2 = flattenSubscript2(subs1.clone(), (id.clone()).clone(), ety.clone())?;
                    Ok(flattenSubscript3(expl1.clone(), (id.clone()).clone(), ety.clone(), exp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: sub1 @ Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::RANGE { .. } }, tail: subs1 }, id, ety) => {
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exp2: Arc<DAE::Exp>;
                    expl1 = Expression::expandRange(var_field!((**sub1).exp, DAE::Subscript::SLICE).clone())?;
                    exp2 = flattenSubscript2(subs1.clone(), (id.clone()).clone(), ety.clone())?;
                    Ok(flattenSubscript3(expl1.clone(), (id.clone()).clone(), ety.clone(), exp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn flattenSubscript3(mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inName: ArcStr, mut inType: Arc<DAE::Type>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut scalar: bool = false;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (inSubscripts.clone()).into_iter().cloned() {
            let __x = applySubscript(e.clone(), inExp.clone(), (inName.clone()).clone(), inType.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outExp = Arc::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl.clone() });
    (dims, scalar) = extractDimensionOfChild(outExp.clone())?;
    ty = Expression::arrayEltType(inType.clone());
    outExp = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() }), scalar: scalar.clone(), array: expl.clone() });
    Ok(outExp)
}

fn removeDoubleEmptyArrays(mut inArr: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outArr: Arc<DAE::Exp>;
    outArr = 'mc: {
        let __mc_input = inArr.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: exp2 @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    Ok(exp2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl1 @ Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { .. }, tail: expl3 }, scalar: sc, ty: ty1 } => {
                    let mut exp1: Arc<DAE::Exp>;
                    let mut expl3 = (*expl3).clone();
                    expl3 = List::map(expl1.clone(), (std::sync::Arc::new(removeDoubleEmptyArrays) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    exp1 = Arc::new(DAE::Exp::ARRAY { ty: ty1.clone(), scalar: sc.clone(), array: expl3.clone() });
                    Ok(exp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp1 => {
                    Ok(exp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp1 => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.removeDoubleEmptyArrays failure for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp1.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArr)
}

fn applySubscript(mut inSub: Arc<DAE::Exp>, mut inSubs: Arc<DAE::Exp>, mut name: ArcStr, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inSub.clone(), inSubs.clone(), name.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, exp1 @ Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: arrDim, .. }, scalar: _, array: Deref @ metamodelica::List::Nil }, _, _) => {
                    let true = (Expression::arrayContainZeroDimension(arrDim.clone())) else { bail!("pattern mismatch") };
                    Ok(exp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: 0 }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: arrDim, .. }, scalar: _, array: _ }, _, ety) => {
                    let mut ety = (*ety).clone();
                    ety = Expression::arrayEltType(ety.clone());
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: cons(Arc::new(DAE::Dimension::DIM_INTEGER { integer: 0 }), arrDim.clone()) }), scalar: true, array: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: 0 }, _, _, ety) => {
                    let mut ety = (*ety).clone();
                    ety = Expression::arrayEltType(ety.clone());
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 0 })] }), scalar: true, array: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp1, Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: Deref @ metamodelica::List::Nil }, id, ety) => {
                    let mut crty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let true = (Expression::isValidSubscript(exp1.clone())) else { bail!("pattern mismatch") };
                    crty = Expression::unliftArray(ety.clone())?;
                    cref_ = ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ety.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: exp1.clone() })]);
                    Ok(Expression::makeCrefExp(cref_.clone(), crty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp1, exp2, _, ety) => {
                    let true = (Expression::isValidSubscript(exp1.clone())) else { bail!("pattern mismatch") };
                    Ok(applySubscript2(exp1.clone(), exp2.clone(), ety.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn applySubscript2(mut inSub: Arc<DAE::Exp>, mut inSubs: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inSub.clone(), inSubs.clone(), inType.clone())) {
        (exp1, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty2, subscriptLst: subs }, ty: _ }, _) => {
            let mut exp2: Arc<DAE::Exp>;
            let mut crty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            crty = Expression::unliftArrayTypeWithSubs(cons(Arc::new(DAE::Subscript::INDEX { exp: exp1.clone() }), subs.clone()), ty2.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty2.clone(), cons(Arc::new(DAE::Subscript::INDEX { exp: exp1.clone() }), subs.clone()));
            exp2 = Expression::makeCrefExp(cref_.clone(), crty.clone())?;
            exp2.clone()
        },
        (exp1, Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 }, ety) => {
            let mut exp2: Arc<DAE::Exp>;
            let mut iLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut scalar: bool = false;
            let mut expl1 = (*expl1).clone();
            let mut ety = (*ety).clone();
            expl1 = List::map2(expl1.clone(), (std::sync::Arc::new(applySubscript3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), exp1.clone(), ety.clone());
            exp2 = Arc::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl1.clone() });
            (iLst, scalar) = extractDimensionOfChild(exp2.clone())?;
            ety = Expression::arrayEltType(ety.clone());
            exp2 = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: iLst.clone() }), scalar: scalar.clone(), array: expl1.clone() });
            exp2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn applySubscript3(mut inSubs: Arc<DAE::Exp>, mut inSub: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inSubs.clone(), inSub.clone(), inType.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty2, subscriptLst: subs }, ty: _ }, exp1, _) => {
            let mut exp2: Arc<DAE::Exp>;
            let mut crty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            crty = Expression::unliftArrayTypeWithSubs(cons(Arc::new(DAE::Subscript::INDEX { exp: exp1.clone() }), subs.clone()), ty2.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty2.clone(), cons(Arc::new(DAE::Subscript::INDEX { exp: exp1.clone() }), subs.clone()));
            exp2 = Expression::makeCrefExp(cref_.clone(), crty.clone())?;
            exp2.clone()
        },
        (Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 }, exp1, ety) => {
            let mut exp2: Arc<DAE::Exp>;
            let mut iLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut scalar: bool = false;
            let mut expl1 = (*expl1).clone();
            let mut ety = (*ety).clone();
            expl1 = List::map2(expl1.clone(), (std::sync::Arc::new(applySubscript3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), exp1.clone(), ety.clone());
            exp2 = Arc::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl1.clone() });
            (iLst, scalar) = extractDimensionOfChild(exp2.clone())?;
            ety = Expression::arrayEltType(ety.clone());
            exp2 = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: iLst.clone() }), scalar: scalar.clone(), array: expl1.clone() });
            exp2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn callVectorize(mut inExp: Arc<DAE::Exp>, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpExpLst = 'mc: {
        let __mc_input = (inExp.clone(), inExpExpLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (callexp @ Deref @ DAE::Exp::CALL { path: r#fn, expLst: args, attr }, Deref @ metamodelica::List::Cons { head: e, tail: es }) => {
                    let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    es_1 = callVectorize(callexp.clone(), es.clone())?;
                    Ok(cons(Arc::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: cons(e.clone(), args.clone()), attr: attr.clone() }), es_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Static.callVectorize failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpExpLst)
}

fn createCrefArray(mut inComponentRef1: Arc<DAE::ComponentRef>, mut inInteger2: i32, mut inInteger3: i32, mut inType4: Arc<DAE::Type>, mut inType5: Arc<DAE::Type>, mut crefIdType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inComponentRef1.clone(), inInteger2.clone(), inInteger3.clone(), inType4.clone(), inType5.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, indx, ds, et, _) => {
                    if !((indx.clone() > ds.clone())) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: true, array: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, indx, ds, et, t) => {
                    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut indx_1: i32 = 0;
                    let mut elt_tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    indx_1 = indx.clone() + 1;
                    cr_1 = ComponentReference::replaceWholeDimSubscript(cr.clone(), indx.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(createCrefArray(cr.clone(), indx_1.clone(), ds.clone(), et.clone(), t.clone(), crefIdType.clone())?) {
                        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl = __pa0.clone();
                    elt_tp = Expression::unliftArray(et.clone())?;
                    e_1 = crefVectorize(true, Expression::makeCrefExp(cr_1.clone(), elt_tp.clone())?, t.clone(), None, crefIdType.clone())?;
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: true, array: cons(e_1.clone(), expl.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, indx, ds, et, t) => {
                    let mut indx_1: i32 = 0;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    indx_1 = indx.clone() + 1;
                    let __pa0 = ::match_deref::match_deref! { match &(createCrefArray(cr.clone(), indx_1.clone(), ds.clone(), et.clone(), t.clone(), crefIdType.clone())?) {
                        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl = __pa0.clone();
                    e_1 = Expression::makeASUB(Expression::makeCrefExp(cr.clone(), et.clone())?, list![Arc::new(DAE::Exp::ICONST { integer: indx.clone() })])?;
                    (e_1, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    e_1 = crefVectorize(true, e_1.clone(), t.clone(), None, crefIdType.clone())?;
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: true, array: cons(e_1.clone(), expl.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("createCrefArray failed on:")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn createCrefArray2d(mut inCref: Arc<DAE::ComponentRef>, mut inIndex: i32, mut inDim1: i32, mut inDim2: i32, mut inType5: Arc<DAE::Type>, mut inType6: Arc<DAE::Type>, mut crefIdType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inCref.clone(), inIndex.clone(), inDim1.clone(), inDim2.clone(), inType5.clone(), inType6.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, indx, ds, _, et, _) => {
                    if !((indx.clone() > ds.clone())) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::MATRIX { ty: et.clone(), integer: 0, matrix: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, indx, ds, ds2, et, t) => {
                    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut indx_1: i32 = 0;
                    let mut elt_tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ms: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    indx_1 = indx.clone() + 1;
                    let __pa0 = ::match_deref::match_deref! { match &(createCrefArray2d(cr.clone(), indx_1.clone(), ds.clone(), ds2.clone(), et.clone(), t.clone(), crefIdType.clone())?) {
                        Deref @ DAE::Exp::MATRIX { matrix: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ms = __pa0.clone();
                    cr_1 = ComponentReference::subscriptCref(cr.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: indx.clone() }) })])?;
                    elt_tp = Expression::unliftArray(et.clone())?;
                    let __pa1 = ::match_deref::match_deref! { match &(crefVectorize(true, Expression::makeCrefExp(cr_1.clone(), elt_tp.clone())?, t.clone(), None, crefIdType.clone())?) {
                        Deref @ DAE::Exp::ARRAY { ty: _, scalar: true, array: __pa1 } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl = __pa1.clone();
                    Ok(Arc::new(DAE::Exp::MATRIX { ty: et.clone(), integer: ds.clone(), matrix: cons(expl.clone(), ms.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.createCrefArray2d failed on: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn absynCrefToComponentReference(mut inComponentRef: Arc<Absyn::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: i } => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = ComponentReferenceBasics::makeCrefIdent((i.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
            cref.clone()
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: c, subscripts: Deref @ metamodelica::List::Nil, name: i } => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = absynCrefToComponentReference(c.clone())?;
            cref = ComponentReferenceBasics::makeCrefQual((i.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), cref.clone());
            cref.clone()
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: c } => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = absynCrefToComponentReference(c.clone())?;
            cref.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

fn elabCrefSubs(mut inCache: FCore::Cache, mut inCrefEnv: FCore::Graph, mut inSubsEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut inTopPrefix: DAE::Prefix, mut inCrefPrefix: DAE::Prefix, mut inBoolean: bool, mut inHasZeroSizeDim: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>, DAE::Const, bool)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    let mut outHasZeroSizeDim: bool = false;
    (outCache, outComponentRef, outConst, outHasZeroSizeDim) = 'mc: {
        let __mc_input = (inCache.clone(), inCrefEnv.clone(), inSubsEnv.clone(), inComponentRef.clone(), inTopPrefix.clone(), inCrefPrefix.clone(), inBoolean.clone(), inHasZeroSizeDim.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: ss, name: id }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut sl: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut id_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ss_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), crefEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), crefPrefix.clone(), ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupVar(cache.clone(), crefEnv.clone(), cr.clone())?) {
                        (__pa0, _, _, _, _, InstTypes::SplicedExpData { identType: __pa1, .. }, _, _, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    id_ty = __pa1.clone();
                    id_ty = Types::simplifyType(id_ty.clone())?;
                    hasZeroSizeDim = Types::isZeroLengthArray(id_ty.clone());
                    sl = TypesDump::getDimensions(id_ty.clone());
                    (cache, ss_1, r#const) = elabSubscriptsDims(cache.clone(), crefSubs.clone(), ss.clone(), sl.clone(), r#impl.clone(), topPrefix.clone(), inComponentRef.clone(), info.clone())?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), id_ty.clone(), ss_1.clone()), r#const.clone(), hasZeroSizeDim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: restCref, subscripts: Deref @ metamodelica::List::Nil, name: id }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sl: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut crefPrefix = (*crefPrefix).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), crefEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), crefPrefix.clone(), ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    (cache, _, t, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), crefEnv.clone(), cr.clone())?;
                    ty = Types::simplifyType(t.clone())?;
                    sl = TypesDump::getDimensions(ty.clone());
                    crefPrefix = PrefixUtil::prefixAdd((id.clone()).clone(), sl.clone(), metamodelica::nil(), crefPrefix.clone(), openmodelica_frontend_types::SCode::Variability::VAR, ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, info.clone())?;
                    (cache, cr, r#const, hasZeroSizeDim) = elabCrefSubs(cache.clone(), crefEnv.clone(), crefSubs.clone(), restCref.clone(), topPrefix.clone(), crefPrefix.clone(), r#impl.clone(), hasZeroSizeDim.clone(), info.clone())?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), metamodelica::nil(), cr.clone()), r#const.clone(), hasZeroSizeDim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: restCref, subscripts: Deref @ metamodelica::List::Nil, name: id }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let mut crefPrefix = (*crefPrefix).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    crefPrefix = PrefixUtil::prefixAdd((id.clone()).clone(), metamodelica::nil(), metamodelica::nil(), crefPrefix.clone(), openmodelica_frontend_types::SCode::Variability::VAR, ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, info.clone())?;
                    (cache, cr, r#const, hasZeroSizeDim) = elabCrefSubs(cache.clone(), crefEnv.clone(), crefSubs.clone(), restCref.clone(), topPrefix.clone(), crefPrefix.clone(), r#impl.clone(), hasZeroSizeDim.clone(), info.clone())?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), DAE::T_COMPLEX_DEFAULT().clone(), metamodelica::nil(), cr.clone()), r#const.clone(), hasZeroSizeDim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: restCref, subscripts: ss @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, name: id }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sl: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut const1: DAE::Const = DAE::Const::C_CONST;
                    let mut const2: DAE::Const = DAE::Const::C_CONST;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut id_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ss_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut vt: SCode::Variability = SCode::Variability::CONST;
                    let mut cache = (*cache).clone();
                    let mut crefPrefix = (*crefPrefix).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), crefEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), crefPrefix.clone(), ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupVar(cache.clone(), crefEnv.clone(), cr.clone())?) {
                        (__pa0, Deref @ DAE::Attributes { variability: __pa1, .. }, __pa2, _, _, InstTypes::SplicedExpData { identType: __pa3, .. }, _, _, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vt = __pa1.clone();
                    t = __pa2.clone();
                    id_ty = __pa3.clone();
                    ty = Types::simplifyType(t.clone())?;
                    id_ty = Types::simplifyType(id_ty.clone())?;
                    sl = TypesDump::getDimensions(id_ty.clone());
                    (cache, ss_1, const1) = elabSubscriptsDims(cache.clone(), crefSubs.clone(), ss.clone(), sl.clone(), r#impl.clone(), topPrefix.clone(), inComponentRef.clone(), info.clone())?;
                    crefPrefix = PrefixUtil::prefixAdd((id.clone()).clone(), sl.clone(), ss_1.clone(), crefPrefix.clone(), vt.clone(), ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, info.clone())?;
                    (cache, cr, const2, hasZeroSizeDim) = elabCrefSubs(cache.clone(), crefEnv.clone(), crefSubs.clone(), restCref.clone(), topPrefix.clone(), crefPrefix.clone(), r#impl.clone(), hasZeroSizeDim.clone(), info.clone())?;
                    r#const = Types::constAnd(const1.clone(), const2.clone());
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), ss_1.clone(), cr.clone()), r#const.clone(), hasZeroSizeDim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: absynCr }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut const1: DAE::Const = DAE::Const::C_CONST;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let mut crefEnv = (*crefEnv).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    crefEnv = FGraph::topScope(crefEnv.clone())?;
                    (cache, cr, const1, hasZeroSizeDim) = elabCrefSubs(cache.clone(), crefEnv.clone(), crefSubs.clone(), absynCr.clone(), topPrefix.clone(), crefPrefix.clone(), r#impl.clone(), hasZeroSizeDim.clone(), info.clone())?;
                    Ok((cache.clone(), cr.clone(), const1.clone(), hasZeroSizeDim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, crefEnv, _, absynCref, topPrefix, crefPrefix, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCrefSubs failed on: ")); __mm_s.push_str(&*literal!("[top:")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(topPrefix.clone())?); __mm_s.push_str(&*literal!("].")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(crefPrefix.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Dump::printComponentRefStr(absynCref.clone())?); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(crefEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outComponentRef, outConst, outHasZeroSizeDim))
}

pub fn elabSubscripts(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynSubscriptLst: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Subscript>>>, DAE::Const)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    (outCache, outExpSubscriptLst, outConst) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynSubscriptLst.clone(), inBoolean.clone(), inPrefix.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, _, _) => {
            (cache.clone(), metamodelica::nil(), openmodelica_frontend_types::DAE::Const::C_CONST)
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: sub, tail: subs }, r#impl, pre) => {
            let mut sub_1: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
            let mut const1: DAE::Const = DAE::Const::C_CONST;
            let mut const2: DAE::Const = DAE::Const::C_CONST;
            let mut r#const: DAE::Const = DAE::Const::C_CONST;
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, sub_1, const1, _) = elabSubscript(cache.clone(), env.clone(), sub.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            (cache, subs_1, const2) = elabSubscripts(cache.clone(), env.clone(), subs.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            r#const = Types::constAnd(const1.clone(), const2.clone());
            (cache.clone(), cons(sub_1.clone(), subs_1.clone()), r#const.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExpSubscriptLst, outConst))
}

fn elabSubscriptsDims(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inSubscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inImpl: bool, mut inPrefix: DAE::Prefix, mut inCref: Arc<Absyn::ComponentRef>, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Subscript>>>, DAE::Const)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut outConst: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    let mut rest_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = inDimensions.clone();
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut dsub: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    let mut prop: Option<DAE::Properties> = None;
    let mut subl_str: ArcStr = arcstr::literal!("");
    let mut diml_str: ArcStr = arcstr::literal!("");
    let mut cref_str: ArcStr = arcstr::literal!("");
    let mut nrdims: i32 = 0;
    let mut nrsubs: i32 = 0;
    for mut asub in &*inSubscripts.clone() {
        let mut asub = asub.clone();
        if rest_dims.clone().is_empty() {
            cref_str = (Dump::printComponentRefStr(inCref.clone())?).clone();
            subl_str = (intString((inSubscripts.clone().len() as i32))).clone();
            diml_str = (intString((inDimensions.clone().len() as i32))).clone();
            Error::addSourceMessageAndFail(Error::WRONG_NUMBER_OF_SUBSCRIPTS.clone(), list![(cref_str.clone()).clone(), (subl_str.clone()).clone(), (diml_str.clone()).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        } else {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim = __pa0.clone();
            rest_dims = __pa1.clone();
        }
        (outCache, dsub, r#const, prop) = elabSubscript(outCache.clone(), inEnv.clone(), asub.clone(), inImpl.clone(), inPrefix.clone(), inInfo.clone())?;
        outConst = Types::constAnd(r#const.clone(), outConst.clone());
        (outCache, dsub) = elabSubscriptsDims2(outCache.clone(), inEnv.clone(), dsub.clone(), dim.clone(), outConst.clone(), prop.clone(), inImpl.clone(), inCref.clone(), inInfo.clone())?;
        outSubs = cons(dsub.clone(), outSubs.clone());
    }
    nrsubs = (outSubs.clone().len() as i32);
    if nrsubs.clone() > 0 {
        nrdims = (inDimensions.clone().len() as i32);
        while nrsubs.clone() < nrdims.clone() {
            outSubs = cons(Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM), outSubs.clone());
            nrsubs = nrsubs.clone() + 1;
        }
    }
    outSubs = outSubs.clone().reverse();
    Ok((outCache, outSubs, outConst))
}

fn elabSubscriptsDims2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inSubscript: Arc<DAE::Subscript>, mut inDimension: Arc<DAE::Dimension>, mut inConst: DAE::Const, mut inProperties: Option<DAE::Properties>, mut inImpl: bool, mut inCref: Arc<Absyn::ComponentRef>, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Subscript>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSubscript: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    (outCache, outSubscript) = 'mc: {
        let __mc_input = (inDimension.clone(), inProperties.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (FGraph::inForOrParforIterLoopScope(inEnv.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::dimensionKnown(inDimension.clone())) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(prop)) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (Types::isParameter(inConst.clone())) else { bail!("pattern mismatch") };
                    ty = Types::getPropType(prop.clone())?;
                    let false = (Types::getFixedVarAttributeParameterOrConstant(ty.clone())) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut sub: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
                    let mut int_dim: i32 = 0;
                    int_dim = Expression::dimensionSize(inDimension.clone())?;
                    let true = (Types::isParameterOrConstant(inConst.clone())) else { bail!("pattern mismatch") };
                    (cache, sub) = Ceval::cevalSubscript(inCache.clone(), inEnv.clone(), inSubscript.clone(), int_dim.clone(), inImpl.clone(), Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
                    Ok((cache.clone(), sub.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_EXP { exp: e }, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut sub: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
                    let mut int_dim: i32 = 0;
                    let true = (Types::isParameterOrConstant(inConst.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Ceval::ceval(inCache.clone(), inEnv.clone(), e.clone(), true, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    int_dim = __pa0.clone();
                    (cache, sub) = Ceval::cevalSubscript(inCache.clone(), inEnv.clone(), inSubscript.clone(), int_dim.clone(), inImpl.clone(), Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
                    Ok((cache.clone(), sub.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    let true = (Types::isParameterOrConstant(inConst.clone())) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Expression::dimensionKnown(inDimension.clone())) else { bail!("pattern mismatch") };
                    let false = (Types::isConstant(inConst.clone()) || Types::isParameter(inConst.clone()) && !(FGraph::inForLoopScope(inEnv.clone())?)) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN, _) => {
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_EXP { exp: _ }, _) => {
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sub_str: ArcStr = arcstr::literal!("");
                    let mut dim_str: ArcStr = arcstr::literal!("");
                    let mut cref_str: ArcStr = arcstr::literal!("");
                    sub_str = (ExpressionBasics::printSubscriptStr(inSubscript.clone())?).clone();
                    dim_str = (ExpressionBasics::dimensionString(inDimension.clone())?).clone();
                    cref_str = (Dump::printComponentRefStr(inCref.clone())?).clone();
                    Error::addSourceMessage(Error::ILLEGAL_SUBSCRIPT.clone(), list![(sub_str.clone()).clone(), (dim_str.clone()).clone(), (cref_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outSubscript))
}

fn elabSubscript(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inSubscript: Arc<Absyn::Subscript>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Subscript>, DAE::Const, Option<DAE::Properties>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSubscript: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    let mut outProperties: Option<DAE::Properties> = None;
    (outCache, outSubscript, outConst, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inSubscript.clone(), inBoolean.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Subscript::NOSUB, _, _) => {
                    Ok((cache.clone(), Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM), openmodelica_frontend_types::DAE::Const::C_CONST, None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: sub }, r#impl, pre) => {
                    let mut sub_1: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut sub_2: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa3, __pa2) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), sub.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, __pa3 @ DAE::Properties::PROP { constFlag: __pa2, .. }) => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    sub_1 = __pa1.clone();
                    r#const = __pa2.clone();
                    prop = __pa3.clone();
                    let (__pa4, __pa5, __pa7, __pa6) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(cache.clone(), env.clone(), sub_1.clone(), prop.clone(), r#impl.clone(), info.clone())?) {
                        (__pa4, __pa5, __pa7 @ DAE::Properties::PROP { type_: __pa6, .. }) => (__pa4.clone(), __pa5.clone(), __pa7.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa4.clone();
                    sub_1 = __pa5.clone();
                    ty = __pa6.clone();
                    prop = __pa7.clone();
                    sub_2 = elabSubscriptType(ty.clone(), sub.clone(), sub_1.clone(), info.clone())?;
                    Ok((cache.clone(), sub_2.clone(), r#const.clone(), Some(prop.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabSubscript failed on ")); __mm_s.push_str(&*Dump::printSubscriptStr(inSubscript.clone())?); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outSubscript, outConst, outProperties))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn elabSubscriptType(mut inType: Arc<DAE::Type>, mut inAbsynExp: Arc<Absyn::Exp>, mut inDaeExp: Arc<DAE::Exp>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Subscript>> {
    let mut outSubscript: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    outSubscript = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            Arc::new(DAE::Subscript::INDEX { exp: inDaeExp.clone() })
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            Arc::new(DAE::Subscript::INDEX { exp: inDaeExp.clone() })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            Arc::new(DAE::Subscript::INDEX { exp: inDaeExp.clone() })
        },
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { .. }, .. } => {
            Arc::new(DAE::Subscript::SLICE { exp: inDaeExp.clone() })
        },
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => {
            Arc::new(DAE::Subscript::SLICE { exp: inDaeExp.clone() })
        },
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_BOOL { .. }, .. } => {
            Arc::new(DAE::Subscript::SLICE { exp: inDaeExp.clone() })
        },
        Deref @ DAE::Type::T_METABOXED { .. } => {
            elabSubscriptType(var_field!((*inType).ty, DAE::Type::T_METABOXED).clone(), inAbsynExp.clone(), inDaeExp.clone(), inInfo.clone())?
        },
        _ => {
            let mut e_str: ArcStr = arcstr::literal!("");
            let mut t_str: ArcStr = arcstr::literal!("");
            e_str = (Dump::printExpStr(inAbsynExp.clone())?).clone();
            t_str = (TypesDump::unparseType(inType.clone())?).clone();
            Error::addSourceMessage(Error::WRONG_DIMENSION_TYPE.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

fn subscriptCrefType(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inExp.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: c, .. }, t) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = subscriptCrefType2(c.clone(), t.clone())?;
                    Ok(t_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn subscriptCrefType2(mut inComponentRef: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inComponentRef.clone(), inType.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, .. }, t) => {
            t.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. }, t) => {
            let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            t_1 = subscriptType(t.clone(), subs.clone())?;
            t_1.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: c, .. }, t) => {
            let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            t_1 = subscriptCrefType2(c.clone(), t.clone())?;
            t_1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn subscriptType(mut inType: Arc<DAE::Type>, mut inExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inType.clone(), inExpSubscriptLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::List::Nil) => {
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { .. }, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: subs }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = subscriptType(t.clone(), subs.clone())?;
                    Ok(t_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { .. }, tail: subs }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = subscriptType(t.clone(), subs.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: subs }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = subscriptType(t.clone(), subs.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, _) => {
                    Print::printBuf((literal!("- subscript_type failed (")).clone())?;
                    Print::printBuf((TypesDump::printTypeStr(t.clone())?).clone())?;
                    Print::printBuf((literal!(" , [...])\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn makeIfExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCondition: Arc<DAE::Exp>, mut inCondProp: DAE::Properties, mut inTrueBranch: Arc<DAE::Exp>, mut inTrueProp: DAE::Properties, mut inFalseBranch: Arc<DAE::Exp>, mut inFalseProp: DAE::Properties, mut inImplicit: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty_match: bool = false;
    let mut cond: bool = false;
    let mut cond_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut true_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut false_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut exp_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut cond_c: DAE::Const = DAE::Const::C_CONST;
    let mut true_c: DAE::Const = DAE::Const::C_CONST;
    let mut false_c: DAE::Const = DAE::Const::C_CONST;
    let mut exp_c: DAE::Const = DAE::Const::C_CONST;
    let mut cond_str: ArcStr = arcstr::literal!("");
    let mut cond_ty_str: ArcStr = arcstr::literal!("");
    let mut e1_str: ArcStr = arcstr::literal!("");
    let mut e2_str: ArcStr = arcstr::literal!("");
    let mut ty1_str: ArcStr = arcstr::literal!("");
    let mut ty2_str: ArcStr = arcstr::literal!("");
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut cond_exp: Arc<DAE::Exp>;
    let mut true_exp: Arc<DAE::Exp>;
    let mut false_exp: Arc<DAE::Exp>;
    let DAE::PROP { constFlag: __pa0, type_: __pa1 } = (inCondProp.clone()) else { bail!("pattern mismatch") };
    cond_c = __pa0.clone();
    cond_ty = __pa1.clone();
    (cond_exp, _, ty_match) = Types::matchTypeNoFail(inCondition.clone(), cond_ty.clone(), DAE::T_BOOL_DEFAULT().clone())?;
    if !(ty_match.clone()) {
        cond_str = (ExpressionBasics::printExpStr(inCondition.clone())?).clone();
        cond_ty_str = (TypesDump::unparseTypeNoAttr(cond_ty.clone())?).clone();
        Error::addSourceMessageAndFail(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(cond_str.clone()).clone(), (cond_ty_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let DAE::PROP { constFlag: __pa2, type_: __pa3 } = (inTrueProp.clone()) else { bail!("pattern mismatch") };
    true_c = __pa2.clone();
    true_ty = __pa3.clone();
    let DAE::PROP { constFlag: __pa4, type_: __pa5 } = (inFalseProp.clone()) else { bail!("pattern mismatch") };
    false_c = __pa4.clone();
    false_ty = __pa5.clone();
    (true_exp, false_exp, exp_ty, ty_match) = Types::checkTypeCompat(inTrueBranch.clone(), true_ty.clone(), inFalseBranch.clone(), false_ty.clone(), false)?;
    if Types::arrayHasUnknownDims(exp_ty.clone()) && !(FGraph::inFunctionScope(inEnv.clone())?) {
        if Types::isParameterOrConstant(cond_c.clone()) {
            cond_c = openmodelica_frontend_types::DAE::Const::C_CONST;
        } else {
            ty_match = false;
        }
    }
    if !(ty_match.clone()) && !(Config::getGraphicsExpMode()?) {
        e1_str = (ExpressionBasics::printExpStr(inTrueBranch.clone())?).clone();
        e2_str = (ExpressionBasics::printExpStr(inFalseBranch.clone())?).clone();
        ty1_str = (TypesDump::unparseTypeNoAttr(true_ty.clone())?).clone();
        ty2_str = (TypesDump::unparseTypeNoAttr(false_ty.clone())?).clone();
        pre_str = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
        Error::addSourceMessageAndFail(Error::TYPE_MISMATCH_IF_EXP.clone(), list![(pre_str.clone()).clone(), (e1_str.clone()).clone(), (ty1_str.clone()).clone(), (e2_str.clone()).clone(), (ty2_str.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if Types::isConstant(cond_c.clone()) {
        if '__try6: {
            let (__pa7, __pa8) = ::match_deref::match_deref! { match &(unwrap_break_err!(Ceval::ceval(inCache.clone(), inEnv.clone(), cond_exp.clone(), inImplicit.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0), '__try6)) {
                (__pa7, Deref @ Values::Value::BOOL { boolean: __pa8 }) => (__pa7.clone(), __pa8.clone()),
                _ => break '__try6 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            outCache = __pa7.clone();
            cond = __pa8.clone();
            if cond.clone() {
                outExp = true_exp.clone();
                outProperties = inTrueProp.clone();
            } else {
                outExp = false_exp.clone();
                outProperties = inFalseProp.clone();
            }
            return Ok((outCache.clone(), outExp.clone(), outProperties.clone()));
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    exp_c = todo!("reduction Types.constAnd: cannot resolve default value");
    outExp = Arc::new(DAE::Exp::IFEXP { expCond: cond_exp.clone(), expThen: true_exp.clone(), expElse: false_exp.clone() });
    outProperties = DAE::Properties::PROP { type_: exp_ty.clone(), constFlag: exp_c.clone() };
    Ok((outCache, outExp, outProperties))
}

fn canonCref2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut inPrefixCref: Arc<DAE::ComponentRef>, mut inBoolean: bool) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outCache, outComponentRef) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inComponentRef.clone(), inPrefixCref.clone(), inBoolean.clone())) {
        (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: ss, identType: ty2, ident: n }, prefixCr, r#impl) => {
            let mut ss_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut sl: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            cr = ComponentReference::crefPrependIdent(prefixCr.clone(), (n.clone()).clone(), metamodelica::nil(), ty2.clone())?;
            (cache, _, t, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
            sl = Types::getDimensionSizes(t.clone())?;
            (cache, ss_1) = Ceval::cevalSubscripts(cache.clone(), env.clone(), ss.clone(), sl.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
            (cache.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), ty2.clone(), ss_1.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outComponentRef))
}

pub fn canonCref(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut inBoolean: bool) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outCache, outComponentRef) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), inBoolean.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::ComponentRef::WILD, _) => {
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: ss, ident: n, .. }, r#impl) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sl: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut ss_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    (cache, _, t, _, _, _, _, _, _) = Lookup::lookupVarIdent(cache.clone(), env.clone(), (n.clone()).clone(), metamodelica::nil())?;
                    sl = Types::getDimensionSizes(t.clone())?;
                    (cache, ss_1) = Ceval::cevalSubscripts(cache.clone(), env.clone(), ss.clone(), sl.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    ty2 = Types::simplifyType(t.clone())?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), ty2.clone(), ss_1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: c, subscriptLst: ss, ident: n, .. }, r#impl) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sl: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut ss_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut componentEnv: FCore::Graph;
                    let mut c_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    (cache, _, t, _, _, _, _, componentEnv, _) = Lookup::lookupVarIdent(cache.clone(), env.clone(), (n.clone()).clone(), metamodelica::nil())?;
                    ty2 = Types::simplifyType(t.clone())?;
                    sl = Types::getDimensionSizes(t.clone())?;
                    (cache, ss_1) = Ceval::cevalSubscripts(cache.clone(), env.clone(), ss.clone(), sl.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    (cache, c_1) = canonCref(cache.clone(), componentEnv.clone(), c.clone(), r#impl.clone())?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefQual((n.clone()).clone(), ty2.clone(), ss_1.clone(), c_1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cr, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Static.canonCref failed, cr: ")).clone())?;
                    Debug::traceln((ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outComponentRef))
}

fn unevaluatedFunctionVariability(mut inEnv: FCore::Graph) -> Result<DAE::Const> {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    if FGraph::inFunctionScope(inEnv.clone())? {
        outConst = openmodelica_frontend_types::DAE::Const::C_VAR;
    } else if Flags::getConfigBool(Flags::CHECK_MODEL.clone())? || Config::splitArrays()? {
        outConst = openmodelica_frontend_types::DAE::Const::C_UNKNOWN;
    } else {
        bail!("fail");
    }
    Ok(outConst)
}

fn slotAnd(mut s: Slot, mut b: bool) -> Result<bool> {
    let mut res: bool = false;
    let Slot { slotFilled: __pa0, .. } = (s.clone()) else { bail!("pattern mismatch") };
    res = __pa0.clone();
    res = b.clone() && res.clone();
    Ok(res)
}

pub fn elabCodeExp(mut exp: Arc<Absyn::Exp>, mut cache: FCore::Cache, mut env: FCore::Graph, mut ct: DAE::CodeType, mut info: SourceInfo) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (exp.clone(), ct.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut dexp: Arc<DAE::Exp>;
                    dexp = elabCodeExp_dispatch(exp.clone(), cache.clone(), env.clone(), ct.clone(), info.clone())?;
                    Ok(dexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CODE { code: Deref @ Absyn::CodeNode::C_MODIFICATION { .. } }, DAE::CodeType::C_EXPRESSION_OR_MODIFICATION) => {
                    Ok(Arc::new(DAE::Exp::CODE { code: var_field!((*exp).code, Absyn::Exp::CODE).clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CODE { code: Deref @ Absyn::CodeNode::C_EXPRESSION { .. } }, DAE::CodeType::C_EXPRESSION) => {
                    Ok(Arc::new(DAE::Exp::CODE { code: var_field!((*exp).code, Absyn::Exp::CODE).clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::CodeType::C_EXPRESSION) => {
                    Ok(Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_EXPRESSION { exp: exp.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::CodeType::C_EXPRESSION_OR_MODIFICATION) => {
                    Ok(Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_EXPRESSION { exp: exp.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, DAE::CodeType::C_TYPENAME) => {
                    let mut path: Arc<Absyn::Path>;
                    path = AbsynUtil::crefToPath(cr.clone())?;
                    Ok(Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_TYPENAME { path: path.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::ARRAY { arrayExp: es }, DAE::CodeType::C_VARIABLENAMES) => {
                    let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut i: i32 = 0;
                    es_1 = List::map4(es.clone(), (std::sync::Arc::new(elabCodeExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Cache, FCore::Graph, DAE::CodeType, SourceInfo) -> Result<Arc<DAE::Exp>> + 'static>), cache.clone(), env.clone(), openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME, info.clone());
                    i = (es.clone().len() as i32);
                    et = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() })] });
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: false, array: es_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::CodeType::C_VARIABLENAMES) => {
                    let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dexp: Arc<DAE::Exp>;
                    et = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] });
                    dexp = elabCodeExp(exp.clone(), cache.clone(), env.clone(), openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME, info.clone())?;
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: false, array: list![dexp.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, DAE::CodeType::C_VARIABLENAME) => {
                    Ok(Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: cr.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CALL { .. }, DAE::CodeType::C_VARIABLENAME) => {
                    if !((isValidDerVariableName(exp.clone(), false))) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_EXPRESSION { exp: exp.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        let DAE::C_VARIABLENAMES { .. } = (ct.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s1 = (Dump::printExpStr(exp.clone())?).clone();
                    s2 = (TypesDump::printCodeTypeStr(ct.clone())).clone();
                    Error::addSourceMessage(Error::ELAB_CODE_EXP_FAILED.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn elabCodeExp_dispatch(mut exp: Arc<Absyn::Exp>, mut cache: FCore::Cache, mut env: FCore::Graph, mut ct: DAE::CodeType, mut info: SourceInfo) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: cr } => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut ct2: DAE::CodeType = DAE::CodeType::C_EXPRESSION;
                    let mut id: ArcStr = arcstr::literal!("");
                    ErrorExt::setCheckpoint((literal!("elabCodeExp_dispatch1")).clone());
                    id = (AbsynUtil::crefFirstIdent(cr.clone())?).clone();
                    let () = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
                    let () = __mc_input.clone() else { bail!("nomatch") };
                    let mut prop: DAE::Properties;
                    let mut dexp: Arc<DAE::Exp>;
                    let true = (id.clone() == literal!("OpenModelica")) else { bail!("pattern mismatch") };
                    (_, dexp, prop) = elabExpInExpression(cache.clone(), env.clone(), exp.clone(), false, false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    let () = __mc_input.clone() else { bail!("nomatch") };
                    let mut dexp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupClassIdent(cache.clone(), env.clone(), (id.clone()).clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (_, dexp, prop) = elabExpInExpression(cache.clone(), env.clone(), exp.clone(), false, false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    let __pa0 = ::match_deref::match_deref! { match &(Types::getPropType(prop.clone())?) {
                        Deref @ DAE::Type::T_CODE { ty: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ct2 = __pa0.clone();
                    let true = (ct.clone() == ct2.clone()) else { bail!("pattern mismatch") };
                    ErrorExt::delCheckpoint((literal!("elabCodeExp_dispatch1")).clone());
                    Ok(dexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { .. } => {
                    ErrorExt::rollBack((literal!("elabCodeExp_dispatch1")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut ct2: DAE::CodeType = DAE::CodeType::C_EXPRESSION;
                    let false = (AbsynUtil::isCref(exp.clone())) else { bail!("pattern mismatch") };
                    ErrorExt::setCheckpoint((literal!("elabCodeExp_dispatch")).clone());
                    (_, dexp, prop) = elabExpInExpression(cache.clone(), env.clone(), exp.clone(), false, false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Types::getPropType(prop.clone())?) {
                        Deref @ DAE::Type::T_CODE { ty: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ct2 = __pa0.clone();
                    let true = (ct.clone() == ct2.clone()) else { bail!("pattern mismatch") };
                    ErrorExt::delCheckpoint((literal!("elabCodeExp_dispatch")).clone());
                    Ok(dexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (AbsynUtil::isCref(exp.clone())) else { bail!("pattern mismatch") };
                    ErrorExt::rollBack((literal!("elabCodeExp_dispatch")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn elabArrayDims(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut inDimensions: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    (outCache, outDimensions) = elabArrayDims2(inCache.clone(), inEnv.clone(), inComponentRef.clone(), inDimensions.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), metamodelica::nil())?;
    Ok((outCache, outDimensions))
}

fn elabArrayDims2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCref: Arc<Absyn::ComponentRef>, mut inDimensions: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inImplicit: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut inElaboratedDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    (outCache, outDimensions) = (::match_deref::match_deref! { match &(inDimensions.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inCache.clone(), inElaboratedDims.clone().reverse())
        },
        Deref @ metamodelica::List::Cons { head: dim, tail: rest_dims } => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut elab_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut elab_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            (cache, elab_dim) = elabArrayDim(inCache.clone(), inEnv.clone(), inCref.clone(), dim.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
            elab_dims = cons(elab_dim.clone(), inElaboratedDims.clone());
            (cache, elab_dims) = elabArrayDims2(cache.clone(), inEnv.clone(), inCref.clone(), rest_dims.clone(), inImplicit.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone(), elab_dims.clone())?;
            (cache.clone(), elab_dims.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outDimensions))
}

fn elabArrayDim(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCref: Arc<Absyn::ComponentRef>, mut inDimension: Arc<Absyn::Subscript>, mut inImpl: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Dimension>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimension: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    (outCache, outDimension) = 'mc: {
        let __mc_input = (inCache.clone(), inDimension.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::NOSUB) => {
                    Ok((inCache.clone(), Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: cr_exp @ Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Cons { head: size_arg, tail: Deref @ metamodelica::List::Nil } }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "size", .. }, .. } }) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut e: Arc<DAE::Exp>;
                    let mut dim_exp: Arc<DAE::Exp>;
                    let true = (AbsynUtil::crefEqual(inCref.clone(), cr.clone())) else { bail!("pattern mismatch") };
                    (cache, e, _) = elabExpInExpression(inCache.clone(), inEnv.clone(), cr_exp.clone(), inImpl.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
                    (cache, dim_exp, _) = elabExpInExpression(cache.clone(), inEnv.clone(), size_arg.clone(), inImpl.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
                    dim = Arc::new(DAE::Dimension::DIM_EXP { exp: Arc::new(DAE::Exp::SIZE { exp: e.clone(), sz: Some(dim_exp.clone()) }) });
                    Ok((inCache.clone(), dim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "Boolean", .. } } }) => {
                    Ok((inCache.clone(), Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_BOOLEAN)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CREF { componentRef: cr } }) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut type_path: Arc<Absyn::Path>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    type_path = AbsynUtil::crefToPath(cr.clone())?;
                    (cache, _, _) = Lookup::lookupClass(cache.clone(), inEnv.clone(), type_path.clone(), None)?;
                    (cache, t, _) = Lookup::lookupType(cache.clone(), inEnv.clone(), type_path.clone(), None)?;
                    dim = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ DAE::Type::T_ENUMERATION { index: None, .. } => Arc::new(DAE::Dimension::DIM_ENUM { enumTypeName: var_field!((*t).path, DAE::Type::T_ENUMERATION).clone(), literals: var_field!((*t).names, DAE::Type::T_ENUMERATION).clone(), size: (var_field!((*t).names, DAE::Type::T_ENUMERATION).clone().len() as i32) }),
        Deref @ DAE::Type::T_BOOL { .. } => Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_BOOLEAN),
        _ => bail!("match: no arm matched"),
    } });
                    Ok((cache.clone(), dim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: sub, .. } }) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    (cache, dim) = elabArrayDim(inCache.clone(), inEnv.clone(), inCref.clone(), Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: sub.clone() }), inImpl.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
                    Ok((cache.clone(), dim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: sub }) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut e: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    (cache, e, prop) = elabExpInExpression(inCache.clone(), inEnv.clone(), sub.clone(), inImpl.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elabArrayDim2(cache.clone(), inEnv.clone(), inCref.clone(), e.clone(), prop.clone(), inImpl.clone(), inDoVect.clone(), inPrefix.clone(), inInfo.clone())?) {
                        (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    dim = __pa1.clone();
                    Ok((cache.clone(), dim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabArrayDim failed on: ")); __mm_s.push_str(&*Dump::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*Dump::printArraydimStr(list![inDimension.clone()])?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDimension))
}

fn elabArrayDim2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCref: Arc<Absyn::ComponentRef>, mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut inImpl: bool, mut inDoVect: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Option<Arc<DAE::Dimension>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimension: Option<Arc<DAE::Dimension>> = None;
    (outCache, outDimension) = 'mc: {
        let __mc_input = (inProperties.clone(), inImpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: cnst }, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut i: i32 = 0;
                    let true = (Types::isParameterOrConstant(cnst.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(inCache.clone(), inEnv.clone(), inExp.clone(), inImpl.clone(), Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    i = __pa1.clone();
                    Ok((cache.clone(), Some(Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: DAE::Const::C_PARAM }, _) => {
                    let false = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), Some(Arc::new(DAE::Dimension::DIM_EXP { exp: inExp.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: DAE::Const::C_VAR }, false) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    e_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                    Error::addSourceMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![(e_str.clone()).clone()], inInfo.clone())?;
                    Ok((inCache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: _ }, true) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut e: Arc<DAE::Exp>;
                    (cache, e, _) = Ceval::cevalIfConstant(inCache.clone(), inEnv.clone(), inExp.clone(), inProperties.clone(), inImpl.clone(), inInfo.clone())?;
                    Ok((cache.clone(), Some(Arc::new(DAE::Dimension::DIM_EXP { exp: e.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut e: Arc<DAE::Exp>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(inCache.clone(), inEnv.clone(), inExp.clone(), inProperties.clone(), inImpl.clone(), inInfo.clone())?) {
                        (__pa0, __pa1 @ Deref @ DAE::Exp::SIZE { exp: _, sz: _ }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e = __pa1.clone();
                    Ok((cache.clone(), Some(Arc::new(DAE::Dimension::DIM_EXP { exp: e.clone() }))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), Some(Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: cnst }, _) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut a_str: ArcStr = arcstr::literal!("");
                    let true = (Types::isParameterOrConstant(cnst.clone())) else { bail!("pattern mismatch") };
                    e_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                    a_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*e_str.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::STRUCTURAL_PARAMETER_OR_CONSTANT_WITH_NO_BINDING.clone(), list![(e_str.clone()).clone(), (a_str.clone()).clone()], inInfo.clone())?;
                    Ok((inCache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: ty, constFlag: _ }, _) => {
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut t_str: ArcStr = arcstr::literal!("");
                    e_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                    t_str = (TypesDump::unparseType(ty.clone())?).clone();
                    Types::typeErrorSanityCheck((t_str.clone()).clone(), (literal!("Integer")).clone(), inInfo.clone())?;
                    Error::addSourceMessage(Error::ARRAY_DIMENSION_INTEGER.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone()], inInfo.clone())?;
                    Ok((inCache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDimension))
}

fn consStrippedCref(mut e: Arc<Absyn::Exp>, mut es: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut oes: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    oes = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = AbsynUtil::crefStripLastSubs(cr.clone())?;
            cons(Arc::new(Absyn::Exp::CREF { componentRef: cr.clone() }), es.clone())
        },
        _ => {
            es.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oes)
}

fn replaceEnd(mut inCref: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut cr_parts: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut cr_no_subs: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynUtil::crefExplode(inCref.clone(), metamodelica::nil())) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCref = __pa0.clone();
    cr_parts = __pa1.clone();
    if !(AbsynUtil::crefIsIdent(outCref.clone())) {
        outCref = inCref.clone();
        return Ok(outCref.clone());
    }
    if AbsynUtil::crefIsFullyQualified(inCref.clone()) {
        outCref = AbsynUtil::crefMakeFullyQualified(outCref.clone());
    }
    outCref = replaceEndInSubs(AbsynUtil::crefStripLastSubs(outCref.clone())?, AbsynUtil::crefLastSubs(outCref.clone())?)?;
    for mut cr in &*cr_parts.clone() {
        let mut cr = cr.clone();
        cr_no_subs = AbsynUtil::crefStripLastSubs(cr.clone())?;
        outCref = AbsynUtil::joinCrefs(outCref.clone(), cr_no_subs.clone())?;
        outCref = replaceEndInSubs(outCref.clone(), AbsynUtil::crefLastSubs(cr.clone())?)?;
    }
    Ok(outCref)
}

fn replaceEndInSubs(mut inCref: Arc<Absyn::ComponentRef>, mut inSubscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = inCref.clone();
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut new_sub: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    let mut i: i32 = 1;
    if inSubscripts.clone().is_empty() {
        return Ok(outCref.clone());
    }
    for mut sub in &*inSubscripts.clone() {
        let mut sub = sub.clone();
        new_sub = replaceEndInSub(sub.clone(), i.clone(), inCref.clone())?;
        subs = cons(new_sub.clone(), subs.clone());
        i = i.clone() + 1;
    }
    outCref = AbsynUtil::crefSetLastSubs(outCref.clone(), subs.clone().reverse())?;
    Ok(outCref)
}

fn replaceEndInSub(mut inSubscript: Arc<Absyn::Subscript>, mut inDimIndex: i32, mut inCref: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Subscript>> {
    let mut outSubscript: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    outSubscript = (::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: replaceEndTraverser(var_field!((*inSubscript).subscript, Absyn::Subscript::SUBSCRIPT).clone(), (inCref.clone(), inDimIndex.clone()))? }),
        _ => inSubscript.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

fn replaceEndTraverser(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<Absyn::ComponentRef>, i32)) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::END => {
            let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut i: i32 = 0;
            (cr, i) = inTuple.clone();
            Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("size")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![Arc::new(Absyn::Exp::CREF { componentRef: cr.clone() }), Arc::new(Absyn::Exp::INTEGER { value: i.clone() })], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() })
        },
        Deref @ Absyn::Exp::CREF { .. } => {
            Arc::new(Absyn::Exp::CREF { componentRef: replaceEnd(var_field!((*inExp).componentRef, Absyn::Exp::CREF).clone())? })
        },
        _ => {
            AbsynUtil::traverseExpShallow(inExp.clone(), inTuple.clone(), (std::sync::Arc::new(replaceEndTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<Absyn::ComponentRef>, i32)) -> Result<Arc<Absyn::Exp>> + 'static>))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn fixTupleMetaModelica(mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut types: Arc<metamodelica::List<Arc<DAE::Type>>>, mut consts: Arc<metamodelica::List<Arc<DAE::TupleConst>>>) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut exp: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut exps2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    if Config::acceptMetaModelicaGrammar()? {
        c = Types::tupleConstListToConst(consts.clone())?;
        tys2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut ty in (types.clone()).into_iter().cloned() {
            let __x = Types::boxIfUnboxedType(ty.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        (exps2, tys2) = Types::matchTypeTuple(exps.clone(), types.clone(), tys2.clone(), false)?;
        exp = Arc::new(DAE::Exp::META_TUPLE { listExp: exps2.clone() });
        prop = DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_METATUPLE { types: tys2.clone() }), constFlag: c.clone() };
    } else {
        exp = Arc::new(DAE::Exp::TUPLE { PR: exps.clone() });
        prop = DAE::Properties::PROP_TUPLE { type_: Arc::new(DAE::Type::T_TUPLE { types: types.clone(), names: None }), tupleConst: Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: consts.clone() }) };
    }
    Ok((exp, prop))
}

fn checkBuiltinCallArgs(mut inPosArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inExpectedArgs: i32, mut inFnName: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    if (inPosArgs.clone().len() as i32) != inExpectedArgs.clone() || !(inNamedArgs.clone().is_empty()) {
        Error::addSourceMessageAndFail(Error::WRONG_NO_OF_ARGS.clone(), list![(inFnName.clone()).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok(())
}

