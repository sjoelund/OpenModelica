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

use crate::Builtin;
use crate::ComponentReference;
use crate::ConnectionGraph;
use crate::DAEUtil;
use crate::Expression;
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use crate::HashTableStringToPath;
use crate::InnerOuter;
use crate::Inst;
use crate::InstExtends;
use crate::InstFunction;
use crate::InstTypes;
use crate::InstUtil;
use crate::Mod;
use crate::PrefixUtil;
use crate::Static;
use crate::Types;
use crate::UnitAbsyn;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/*   - Lookup functions

  These functions look up class and variable names in the environment.
  The names are supplied as a path, and if the path is qualified, a
  variable named as the first part of the path is searched for, and the
  name is looked for in it.

 */
pub fn lookupType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut msg: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<DAE::Type>, FCore::Graph)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut env: FCore::Graph;
    (cache, t, env) = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            (cache, t, env) = lookupTypeIdent(inCache.clone(), inEnv.clone(), (var_field!((*inPath).name, Absyn::Path::IDENT).clone()).clone(), msg.clone())?;
            (cache.clone(), t.clone(), env.clone())
        },
        _ => {
            (cache, t, env) = lookupTypeQual(inCache.clone(), inEnv.clone(), inPath.clone(), msg.clone())?;
            (cache.clone(), t.clone(), env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, t, env))
}

fn lookupTypeQual(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut msg: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPath.clone(), msg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: DAE::T_ANYTYPE_DEFAULT.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: inPath.clone() });
                    Ok((cache.clone(), t.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "uniqueRootIndices" } }, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("roots")).clone(), ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_ANYTYPE_DEFAULT.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("nodes")).clone(), ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_ANYTYPE_DEFAULT.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), Arc::new(DAE::FuncArg { name: (literal!("message")).clone(), ty: DAE::T_STRING_DEFAULT.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: inPath.clone() });
                    Ok((cache.clone(), t.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, path, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    (cache, c, env_1) = lookupClass(cache.clone(), env.clone(), path.clone(), None)?;
                    (cache, t, env_2) = lookupType2(cache.clone(), env_1.clone(), c.clone())?;
                    Ok((cache.clone(), t.clone(), env_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, path, Some(info)) => {
                    let mut classname: ArcStr = arcstr::literal!("");
                    let mut scope: ArcStr = arcstr::literal!("");
                    classname = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    classname = (stringAppend((classname.clone()).clone(), (literal!(" (its type) ")).clone())).clone();
                    scope = (FGraph::printGraphPathStr(env.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_ERROR.clone(), list![(classname.clone()).clone(), (scope.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outType, outEnv))
}

pub fn lookupTypeIdent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ident: ArcStr, mut msg: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), ident.clone(), msg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ "rooted", _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: DAE::T_ANYTYPE_DEFAULT.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("rooted")).clone() }) });
                    Ok((cache.clone(), t.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, t, env_1) = lookupTypeInEnv(cache.clone(), env.clone(), (ident.clone()).clone())?;
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    (cache, c, env_1) = lookupClassIdent(cache.clone(), env.clone(), (ident.clone()).clone(), None)?;
                    (cache, t, env_2) = lookupType2(cache.clone(), env_1.clone(), c.clone())?;
                    Ok((cache.clone(), t.clone(), env_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, Some(info)) => {
                    let mut classname: ArcStr = arcstr::literal!("");
                    let mut scope: ArcStr = arcstr::literal!("");
                    classname = (stringAppend((ident.clone()).clone(), (literal!(" (its type) ")).clone())).clone();
                    scope = (FGraph::printGraphPathStr(env.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_ERROR.clone(), list![(classname.clone()).clone(), (scope.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outType, outEnv))
}

fn lookupType2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inClass: Arc<SCode::Element>) -> Result<(FCore::Cache, Arc<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut env_1 = (*env_1).clone();
                    (cache, env_1, t) = buildRecordType(cache.clone(), env_1.clone(), c.clone())?;
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c @ Deref @ SCode::Element::CLASS { restriction: r @ SCode::Restriction::R_ENUMERATION, encapsulatedPrefix: encflag, name: id, .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_2: FCore::Graph;
                    let mut env_3: FCore::Graph;
                    let mut path: Arc<Absyn::Path>;
                    let mut types: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut ci_state: ClassInf::State;
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    env_2 = FGraph::openScope(env_1.clone(), encflag.clone(), (id.clone()).clone(), Some(crate::FCore::ScopeType::CLASS_SCOPE))?;
                    ci_state = ClassInfUtil::start(r.clone(), FGraph::getGraphName(env_2.clone())?)?;
                    r#mod = Mod::getClassModifier(env_1.clone(), (id.clone()).clone())?;
                    (cache, env_3, _, _, _, _, _, types, _, _, _, _) = Inst::instClassIn(cache.clone(), env_2.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), false, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone(), None)?;
                    (_, names) = SCodeUtil::getClassComponents(c.clone())?;
                    Types::checkEnumDuplicateLiterals(names.clone(), var_field!((**c).info, SCode::Element::CLASS).clone())?;
                    path = FGraph::getGraphName(env_3.clone())?;
                    t = Arc::new(DAE::Type::T_ENUMERATION { index: None, path: path.clone(), names: names.clone(), literalVarLst: types.clone(), attributeLst: metamodelica::nil() });
                    env_3 = FGraph::mkTypeNode(env_3.clone(), (id.clone()).clone(), t.clone())?;
                    Ok((cache.clone(), t.clone(), env_3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Real" }, .. }, .. }, restriction: SCode::Restriction::R_TYPE, .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = DAE::T_REAL_DEFAULT.clone();
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, .. }, .. }, restriction: SCode::Restriction::R_TYPE, .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = DAE::T_INTEGER_DEFAULT.clone();
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Boolean" }, .. }, .. }, restriction: SCode::Restriction::R_TYPE, .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = DAE::T_BOOL_DEFAULT.clone();
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "String" }, .. }, .. }, restriction: SCode::Restriction::R_TYPE, .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = DAE::T_STRING_DEFAULT.clone();
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { .. }, .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_2: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, env_2, t) = buildMetaRecordType(cache.clone(), env_1.clone(), c.clone())?;
                    Ok((cache.clone(), t.clone(), env_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_2: FCore::Graph;
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut env_1 = (*env_1).clone();
                    let true = (SCodeUtil::classIsExternalObject(c.clone())) else { bail!("pattern mismatch") };
                    (cache, env_1, _, _, _, _, _, _, _, _) = Inst::instClass(cache.clone(), env_1.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), false, crate::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(c.clone()) {
                        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    id = __pa0.clone();
                    (env_1, _) = FGraph::stripLastScopeRef(env_1.clone())?;
                    (cache, t, env_2) = lookupTypeInEnv(cache.clone(), env_1.clone(), (id.clone()).clone())?;
                    Ok((cache.clone(), t.clone(), env_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, name: id, .. }) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_2: FCore::Graph;
                    let mut env_3: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, env_2, _) = InstFunction::implicitFunctionTypeInstantiation(cache.clone(), env_1.clone(), InnerOuter::emptyInstHierarchy().clone(), c.clone())?;
                    (cache, t, env_3) = lookupTypeInEnv(cache.clone(), env_2.clone(), (id.clone()).clone())?;
                    Ok((cache.clone(), t.clone(), env_3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outType, outEnv))
}

pub fn lookupMetarecordsRecursive(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inUniontypePaths: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outMetarecordTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outCache, _, outMetarecordTypes) = lookupMetarecordsRecursive2(inCache.clone(), inEnv.clone(), inUniontypePaths.clone(), HashTableStringToPath::emptyHashTable(), metamodelica::nil())?;
    Ok((outCache, outMetarecordTypes))
}

fn lookupMetarecordsRecursive2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inUniontypePaths: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)), mut inAcc: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(FCore::Cache, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    let mut outMetarecordTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outCache, outHt, outMetarecordTypes) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inUniontypePaths.clone(), inHt.clone(), inAcc.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, ht, acc) => {
            (cache.clone(), ht.clone(), acc.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: first, tail: rest }, ht, acc) => {
            let mut cache = (*cache).clone();
            let mut ht = (*ht).clone();
            let mut acc = (*acc).clone();
            (cache, ht, acc) = lookupMetarecordsRecursive3(cache.clone(), env.clone(), first.clone(), (AbsynUtil::pathString(first.clone(), (literal!(".")).clone(), true, false)?).clone(), ht.clone(), acc.clone())?;
            (cache, ht, acc) = lookupMetarecordsRecursive2(cache.clone(), env.clone(), rest.clone(), ht.clone(), acc.clone())?;
            (cache.clone(), ht.clone(), acc.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outHt, outMetarecordTypes))
}

fn lookupMetarecordsRecursive3(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut path: Arc<Absyn::Path>, mut r#str: ArcStr, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)), mut inAcc: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(FCore::Cache, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    let mut outMetarecordTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outCache, outHt, outMetarecordTypes) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), path.clone(), r#str.clone(), inHt.clone(), inAcc.clone())) {
        (cache, _, _, _, ht, acc) if (BaseHashTable::hasKey((r#str.clone()).clone(), ht.clone())) => {
            (cache.clone(), ht.clone(), acc.clone())
        },
        (cache, env, _, _, ht, acc) => {
            let mut uniontypePaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut uniontypeTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            let mut ht = (*ht).clone();
            let mut acc = (*acc).clone();
            ht = BaseHashTable::add((r#str.clone(), path.clone()), ht.clone())?;
            (cache, ty, _) = lookupType(cache.clone(), env.clone(), path.clone(), Some(Absyn::dummyInfo.clone()))?;
            acc = cons(ty.clone(), acc.clone());
            uniontypeTypes = Types::getAllInnerTypesOfType(ty.clone(), Arc::new(fnptr!(Types::uniontypeFilter, Arc<DAE::Type>)));
            uniontypePaths = List::flatten(List::map(uniontypeTypes.clone(), Arc::new(Types::getUniontypePaths)));
            (cache, ht, acc) = lookupMetarecordsRecursive2(cache.clone(), env.clone(), uniontypePaths.clone(), ht.clone(), acc.clone())?;
            (cache.clone(), ht.clone(), acc.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outHt, outMetarecordTypes))
}

pub fn lookupClass(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    (outCache, outClass, outEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::Path::QUALIFIED { name, path: id }) => {
                    let mut cenv: FCore::Graph;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outEnv: FCore::Graph;
                    let mut outClass: Arc<SCode::Element>;
                    ErrorExt::setCheckpoint((literal!("functionViaComponentRef2")).clone());
                    (outCache, _, _, _, _, _, _, cenv, _) = lookupVarIdent(inCache.clone(), inEnv.clone(), (name.clone()).clone(), metamodelica::nil())?;
                    (outCache, outClass, outEnv) = lookupClass(outCache.clone(), cenv.clone(), id.clone(), None)?;
                    ErrorExt::rollBack((literal!("functionViaComponentRef2")).clone());
                    Ok((outCache.clone(), outClass.clone(), outEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::Path::QUALIFIED { name: _, path: _ }) => {
                    ErrorExt::rollBack((literal!("functionViaComponentRef2")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outEnv: FCore::Graph;
                    let mut outClass: Arc<SCode::Element>;
                    (outCache, outClass, outEnv, _) = lookupClass1(inCache.clone(), inEnv.clone(), inPath.clone(), metamodelica::nil(), Mutable::create(false), inInfo.clone())?;
                    Ok((outCache.clone(), outClass.clone(), outEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClass, outEnv))
}

pub fn lookupClassIdent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ident: ArcStr, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    (outCache, outClass, outEnv, _) = lookupClassInEnv(inCache.clone(), inEnv.clone(), (ident.clone()).clone(), metamodelica::nil(), Mutable::create(false), inInfo.clone())?;
    Ok((outCache, outClass, outEnv))
}

fn lookupClass1(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inState: Mutable::Mutable<bool>, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    let mut errors: i32 = Error::getNumErrorMessages();
    let mut info: SourceInfo;
    if let Ok((__pa0, __pa1, __pa2, __pa3)) = lookupClass2(inCache.clone(), inEnv.clone(), inPath.clone(), inPrevFrames.clone(), inState.clone(), inInfo.clone()) {
        outCache = __pa0.clone();
        outClass = __pa1.clone();
        outEnv = __pa2.clone();
        outPrevFrames = __pa3.clone();
    } else {
        if isSome(inInfo.clone()) && errors.clone() == Error::getNumErrorMessages() {
            Error::addSourceMessage(Error::LOOKUP_ERROR.clone(), list![(AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?).clone(), (FGraph::printGraphPathStr(inEnv.clone())?).clone()], Util::getOption(inInfo.clone())?)?;
        }
        bail!("fail");
    }
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn lookupClass2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inState: Mutable::Mutable<bool>, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    (outCache, outClass, outEnv, outPrevFrames) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inPath.clone(), inPrevFrames.clone())) {
        (cache, env, Deref @ Absyn::Path::FULLYQUALIFIED { path }, Deref @ metamodelica::List::Nil) => {
            let mut r: metamodelica::Array<FCore::Node>;
            let mut c: Arc<SCode::Element>;
            let mut env_1: FCore::Graph;
            let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(env.clone())?.reverse()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            prevFrames = __pa1.clone();
            Mutable::update(inState.clone(), true);
            env = FGraph::setScope(env.clone(), list![r.clone()])?;
            (cache, c, env_1, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
            (cache.clone(), c.clone(), env_1.clone(), prevFrames.clone())
        },
        (cache, env, Deref @ Absyn::Path::QUALIFIED { path, name: pack }, prevFrames) => {
            let mut c: Arc<SCode::Element>;
            let mut env_2: FCore::Graph;
            let mut optFrame: Option<metamodelica::Array<FCore::Node>> = None;
            let mut cache = (*cache).clone();
            let mut prevFrames = (*prevFrames).clone();
            (optFrame, prevFrames) = lookupPrevFrames((pack.clone()).clone(), prevFrames.clone())?;
            (cache, c, env_2, prevFrames) = lookupClassQualified(cache.clone(), env.clone(), (pack.clone()).clone(), path.clone(), optFrame.clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
            (cache.clone(), c.clone(), env_2.clone(), prevFrames.clone())
        },
        (cache, env, Deref @ Absyn::Path::IDENT { name: id }, prevFrames) => {
            let mut c: Arc<SCode::Element>;
            let mut env_1: FCore::Graph;
            let mut cache = (*cache).clone();
            let mut prevFrames = (*prevFrames).clone();
            (cache, c, env_1, prevFrames) = lookupClassInEnv(cache.clone(), env.clone(), (id.clone()).clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
            (cache.clone(), c.clone(), env_1.clone(), prevFrames.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn lookupClassQualified(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut id: ArcStr, mut path: Arc<Absyn::Path>, mut inOptFrame: Option<metamodelica::Array<FCore::Node>>, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inState: Mutable::Mutable<bool>, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    (outCache, outClass, outEnv, outPrevFrames) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), id.clone(), path.clone(), inOptFrame.clone(), inPrevFrames.clone())) {
        (cache, env, _, _, Some(frame), prevFrames) => {
            let mut c: Arc<SCode::Element>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut prevFrames = (*prevFrames).clone();
            Mutable::update(inState.clone(), true);
            env = FGraph::pushScopeRef(env.clone(), frame.clone())?;
            (cache, c, env, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
            (cache.clone(), c.clone(), env.clone(), prevFrames.clone())
        },
        (cache, env, _, _, None, _) => {
            let mut c: Arc<SCode::Element>;
            let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
            let mut optFrame: Option<metamodelica::Array<FCore::Node>> = None;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, c, env, prevFrames) = lookupClassInEnv(cache.clone(), env.clone(), (id.clone()).clone(), metamodelica::nil(), inState.clone(), inInfo.clone())?;
            (optFrame, prevFrames) = lookupPrevFrames((id.clone()).clone(), prevFrames.clone())?;
            (cache, c, env, prevFrames) = lookupClassQualified2(cache.clone(), env.clone(), path.clone(), c.clone(), optFrame.clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
            (cache.clone(), c.clone(), env.clone(), prevFrames.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn lookupClassQualified2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut path: Arc<Absyn::Path>, mut inC: Arc<SCode::Element>, mut optFrame: Option<metamodelica::Array<FCore::Node>>, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inState: Mutable::Mutable<bool>, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    (outCache, outClass, outEnv, outPrevFrames) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), path.clone(), inC.clone(), optFrame.clone(), inPrevFrames.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, Some(frame), prevFrames) => {
                    let mut c: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    env = FGraph::pushScopeRef(env.clone(), frame.clone())?;
                    (cache, c, env, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Deref @ SCode::Element::CLASS { name: id, .. }, None, _) => {
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut c: Arc<SCode::Element>;
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    r = FNode::child(FGraph::lastScopeRef(env.clone())?, (id.clone()).clone())?;
                    let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: _ }, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
                    (cache, env) = Inst::getCachedInstance(cache.clone(), env.clone(), (id.clone()).clone(), r.clone())?;
                    (cache, c, env, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), metamodelica::nil(), inState.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Deref @ SCode::Element::CLASS { restriction: restr, encapsulatedPrefix: encflag, name: id, .. }, None, _) => {
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut ci_state: ClassInf::State;
                    let mut c: Arc<SCode::Element>;
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    env = FGraph::openScope(env.clone(), encflag.clone(), (id.clone()).clone(), FGraph::restrictionToScopeType(restr.clone()))?;
                    ci_state = ClassInfUtil::start(restr.clone(), FGraph::getGraphName(env.clone())?)?;
                    r#mod = Mod::getClassModifier(inEnv.clone(), (id.clone()).clone())?;
                    (cache, env, _, _, _) = Inst::partialInstClassIn(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), inC.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    checkPartialScope(env.clone(), inEnv.clone(), cache.clone(), inInfo.clone())?;
                    (cache, c, env, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), metamodelica::nil(), inState.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn checkPartialScope(mut inEnv: FCore::Graph, mut inParentEnv: FCore::Graph, mut inCache: FCore::Cache, mut inInfo: Option<SourceInfo>) -> Result<()> {
    let mut el: Arc<SCode::Element>;
    let mut pre: DAE::Prefix = DAE::Prefix::NOPRE;
    let mut name: ArcStr = arcstr::literal!("");
    let mut pre_str: ArcStr = arcstr::literal!("");
    let mut cc_str: ArcStr = arcstr::literal!("");
    let mut cls_info: SourceInfo;
    let mut pre_info: SourceInfo;
    let mut info: SourceInfo;
    if isSome(inInfo.clone()) && FGraph::isPartialScope(inEnv.clone()) && Config::languageStandardAtLeast(Config::LanguageStandard::_3_2.clone())? {
        let FCore::N { data: FCore::CL { pre: __pa0, e: __pa1, .. }, .. } = (FNode::fromRef(FGraph::lastScopeRef(inEnv.clone())?)?) else { bail!("pattern mismatch") };
        pre = __pa0.clone();
        el = __pa1.clone();
        name = (SCodeUtil::elementName(el.clone())?).clone();
        if FGraph::graphPrefixOf(inParentEnv.clone(), inEnv.clone())? && !(PrefixUtil::isNoPrefix(pre.clone())) {
            pre_str = (PrefixUtil::printPrefixStr(pre.clone())?).clone();
            cls_info = SCodeUtil::elementInfo(el.clone());
            pre_info = PrefixUtil::getPrefixInfo(pre.clone());
            cc_str = (getConstrainingClass(el.clone(), (FGraph::stripLastScopeRef(inEnv.clone())?).0, inCache.clone())?).clone();
            Error::addMultiSourceMessage(Error::USE_OF_PARTIAL_CLASS.clone(), list![(pre_str.clone()).clone(), (name.clone()).clone(), (cc_str.clone()).clone()], list![cls_info.clone(), pre_info.clone()])?;
            bail!("fail");
        } else {
            let Some(__pa2) = (inInfo.clone()) else { bail!("pattern mismatch") };
            info = __pa2.clone();
            if !(Config::getGraphicsExpMode()?) {
                Error::addSourceMessage(Error::LOOKUP_IN_PARTIAL_CLASS.clone(), list![(name.clone()).clone()], info.clone())?;
            }
        }
    }
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getConstrainingClass(mut inClass: Arc<SCode::Element>, mut inEnv: FCore::Graph, mut inCache: FCore::Cache) -> Result<ArcStr> {
    let mut outPath: ArcStr = arcstr::literal!("");
    outPath = ('mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: cc_path, .. }) }, .. }, .. } => {
                    Ok(AbsynUtil::pathString(cc_path.clone(), (literal!(".")).clone(), true, false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: ts, .. }, .. } => {
                    let mut el: Arc<SCode::Element>;
                    let mut env: FCore::Graph;
                    (_, el, env) = lookupClass(inCache.clone(), inEnv.clone(), AbsynUtil::typeSpecPath(ts.clone())?, None)?;
                    Ok(getConstrainingClass(el.clone(), env.clone(), inCache.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*SCodeUtil::elementName(inClass.clone())?); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outPath)
}

fn lookupPrevFrames(mut id: ArcStr, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>) -> Result<(Option<metamodelica::Array<FCore::Node>>, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>)> {
    let mut outFrame: Option<metamodelica::Array<FCore::Node>> = None;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    (outFrame, outPrevFrames) = 'mc: {
        let __mc_input = (id.clone(), inPrevFrames.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: r#ref, tail: prevFrames }) => {
                    let mut sid: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r#ref.clone())?) else { bail!("pattern mismatch") };
                    sid = (FNode::refName(r#ref.clone())?).clone();
                    let true = (id.clone() == sid.clone()) else { bail!("pattern mismatch") };
                    Ok((Some(r#ref.clone()), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outFrame, outPrevFrames))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lookupQualifiedImportedVarInFrame(mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut ident: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = 'mc: {
        let __mc_input = (inImports.clone(), ident.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Absyn::Import::QUAL_IMPORT { path }, tail: _ }, _) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    id = (AbsynUtil::pathLastIdent(path.clone())?).clone();
                    let true = (id.clone() == ident.clone()) else { bail!("pattern mismatch") };
                    Ok(ComponentReference::pathToCref(path.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { path, name: id }, tail: _ }, _) => {
                    let true = (id.clone() == ident.clone()) else { bail!("pattern mismatch") };
                    Ok(ComponentReference::pathToCref(path.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    Ok(lookupQualifiedImportedVarInFrame(rest.clone(), (ident.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

fn moreLookupUnqualifiedImportedVarInFrame(mut inCache: FCore::Cache, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: FCore::Graph, mut inIdent: ArcStr) -> Result<(FCore::Cache, bool)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outBoolean: bool = false;
    (outCache, outBoolean) = 'mc: {
        let __mc_input = (inCache.clone(), inImports.clone(), inEnv.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ }, env, ident) => {
                    let mut f: metamodelica::Array<FCore::Node>;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(env.clone())?.reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    f = __pa0.clone();
                    prevFrames = __pa1.clone();
                    cref = ComponentReference::pathToCref(path.clone())?;
                    cref = ComponentReference::crefPrependIdent(cref.clone(), (ident.clone()).clone(), metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT.clone())?;
                    env = FGraph::setScope(env.clone(), list![f.clone()])?;
                    (cache, _, _, _, _, _, _, _, _) = lookupVarInPackages(cache.clone(), env.clone(), cref.clone(), prevFrames.clone(), Mutable::create(false))?;
                    Ok((cache.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: _, tail: rest }, env, ident) => {
                    let mut res: bool = false;
                    let mut cache = (*cache).clone();
                    (cache, res) = moreLookupUnqualifiedImportedVarInFrame(cache.clone(), rest.clone(), env.clone(), (ident.clone()).clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((cache.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outBoolean))
}

fn lookupUnqualifiedImportedVarInFrame(mut inCache: FCore::Cache, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: FCore::Graph, mut inIdent: ArcStr) -> Result<(FCore::Cache, FCore::Graph, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, bool, InstTypes::SplicedExpData, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClassEnv: FCore::Graph;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut outBoolean: bool = false;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outClassEnv, outAttributes, outType, outBinding, constOfForIteratorRange, outBoolean, splicedExpData, outComponentEnv, name) = 'mc: {
        let __mc_input = (inCache.clone(), inImports.clone(), inEnv.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: rest }, env, ident) => {
                    let mut f: metamodelica::Array<FCore::Node>;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut more: bool = false;
                    let mut unique: bool = false;
                    let mut classEnv: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(env.clone())?.reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    f = __pa0.clone();
                    prevFrames = __pa1.clone();
                    cref = ComponentReference::pathToCref(path.clone())?;
                    cref = ComponentReference::crefPrependIdent(cref.clone(), (ident.clone()).clone(), metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT.clone())?;
                    env2 = FGraph::setScope(env.clone(), list![f.clone()])?;
                    (cache, classEnv, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env2.clone(), cref.clone(), prevFrames.clone(), Mutable::create(false))?;
                    (cache, more) = moreLookupUnqualifiedImportedVarInFrame(cache.clone(), rest.clone(), env.clone(), (ident.clone()).clone())?;
                    unique = boolNot(more.clone());
                    Ok((cache.clone(), classEnv.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), unique.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: _, tail: rest }, env, ident) => {
                    let mut unique: bool = false;
                    let mut classEnv: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut cache = (*cache).clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut name: ArcStr = name.clone();
                    (cache, classEnv, attr, ty, bind, cnstForRange, unique, splicedExpData, componentEnv, name) = lookupUnqualifiedImportedVarInFrame(cache.clone(), rest.clone(), env.clone(), (ident.clone()).clone())?;
                    Ok((cache.clone(), classEnv.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), unique.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClassEnv, outAttributes, outType, outBinding, constOfForIteratorRange, outBoolean, splicedExpData, outComponentEnv, name))
}

fn lookupQualifiedImportedClassInFrame(mut inCache: FCore::Cache, mut inImport: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: FCore::Graph, mut inIdent: ArcStr, mut inState: Mutable::Mutable<bool>, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    (outCache, outClass, outEnv, outPrevFrames) = 'mc: {
        let __mc_input = (inCache.clone(), inImport.clone(), inEnv.clone(), inIdent.clone(), inState.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: Absyn::Import::QUAL_IMPORT { path: Deref @ Absyn::Path::IDENT { name: id } }, tail: _ }, env, ident, _) => {
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut c: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let true = (id.clone() == ident.clone()) else { bail!("pattern mismatch") };
                    Mutable::update(inState.clone(), true);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(env.clone())?.reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r = __pa0.clone();
                    prevFrames = __pa1.clone();
                    env = FGraph::setScope(env.clone(), list![r.clone()])?;
                    (cache, c, env_1, prevFrames) = lookupClassInEnv(cache.clone(), env.clone(), (id.clone()).clone(), prevFrames.clone(), Mutable::create(false), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: Absyn::Import::QUAL_IMPORT { path }, tail: _ }, env, ident, _) => {
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut c: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    id = (AbsynUtil::pathLastIdent(path.clone())?).clone();
                    let true = (id.clone() == ident.clone()) else { bail!("pattern mismatch") };
                    Mutable::update(inState.clone(), true);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(env.clone())?.reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r = __pa0.clone();
                    prevFrames = __pa1.clone();
                    env = FGraph::setScope(env.clone(), list![r.clone()])?;
                    (cache, c, env_1, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), prevFrames.clone(), Mutable::create(false), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { path, name: id }, tail: _ }, env, ident, _) => {
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut c: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let true = (id.clone() == ident.clone()) else { bail!("pattern mismatch") };
                    Mutable::update(inState.clone(), true);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(env.clone())?.reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r = __pa0.clone();
                    prevFrames = __pa1.clone();
                    env = FGraph::setScope(env.clone(), list![r.clone()])?;
                    (cache, c, env_1, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), prevFrames.clone(), Mutable::create(false), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: _, tail: rest }, env, ident, _) => {
                    let mut c: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, c, env_1, prevFrames) = lookupQualifiedImportedClassInFrame(cache.clone(), rest.clone(), env.clone(), (ident.clone()).clone(), inState.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn moreLookupUnqualifiedImportedClassInFrame(mut inCache: FCore::Cache, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: FCore::Graph, mut inIdent: ArcStr) -> Result<(FCore::Cache, bool)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outBoolean: bool = false;
    (outCache, outBoolean) = 'mc: {
        let __mc_input = (inCache.clone(), inImports.clone(), inEnv.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ }, env, ident) => {
                    let mut c: Arc<SCode::Element>;
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut encflag: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut restr: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut env_1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    env = FGraph::topScope(env.clone())?;
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(lookupClass(cache.clone(), env.clone(), path.clone(), None)?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { restriction: __pa1, encapsulatedPrefix: __pa2, name: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    restr = __pa1.clone();
                    encflag = __pa2.clone();
                    id = __pa3.clone();
                    c = __pa4.clone();
                    env_1 = __pa5.clone();
                    env2 = FGraph::openScope(env_1.clone(), encflag.clone(), (id.clone()).clone(), FGraph::restrictionToScopeType(restr.clone()))?;
                    ci_state = ClassInfUtil::start(restr.clone(), FGraph::getGraphName(env2.clone())?)?;
                    r#mod = Mod::getClassModifier(env_1.clone(), (id.clone()).clone())?;
                    (cache, env, _, _, _) = Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    r = FGraph::lastScopeRef(env.clone())?;
                    env = FGraph::setScope(env.clone(), list![r.clone()])?;
                    (cache, _, _) = lookupClass(cache.clone(), env.clone(), Arc::new(Absyn::Path::IDENT { name: (ident.clone()).clone() }), None)?;
                    Ok((cache.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: _, tail: rest }, env, ident) => {
                    let mut res: bool = false;
                    let mut cache = (*cache).clone();
                    (cache, res) = moreLookupUnqualifiedImportedClassInFrame(cache.clone(), rest.clone(), env.clone(), (ident.clone()).clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((cache.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outBoolean))
}

fn lookupUnqualifiedImportedClassInFrame(mut inCache: FCore::Cache, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: FCore::Graph, mut inIdent: ArcStr, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, bool)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    let mut outBoolean: bool = false;
    (outCache, outClass, outEnv, outPrevFrames, outBoolean) = 'mc: {
        let __mc_input = (inCache.clone(), inImports.clone(), inEnv.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: rest }, env, ident) => {
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut c: Arc<SCode::Element>;
                    let mut c_1: Arc<SCode::Element>;
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut encflag: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut more: bool = false;
                    let mut unique: bool = false;
                    let mut restr: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut env_1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut ci_state: ClassInf::State;
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(env.clone())?.reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r = __pa0.clone();
                    prevFrames = __pa1.clone();
                    env3 = FGraph::setScope(env.clone(), list![r.clone()])?;
                    let (__pa2, __pa6, __pa3, __pa4, __pa5, __pa7, __pa8) = ::match_deref::match_deref! { match &(lookupClass2(cache.clone(), env3.clone(), path.clone(), prevFrames.clone(), Mutable::create(false), inInfo.clone())?) {
                        (__pa2, __pa6 @ Deref @ SCode::Element::CLASS { restriction: __pa3, encapsulatedPrefix: __pa4, name: __pa5, .. }, __pa7, __pa8) => (__pa2.clone(), __pa6.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa7.clone(), __pa8.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    restr = __pa3.clone();
                    encflag = __pa4.clone();
                    id = __pa5.clone();
                    c = __pa6.clone();
                    env_1 = __pa7.clone();
                    prevFrames = __pa8.clone();
                    env2 = FGraph::openScope(env_1.clone(), encflag.clone(), (id.clone()).clone(), FGraph::restrictionToScopeType(restr.clone()))?;
                    ci_state = ClassInfUtil::start(restr.clone(), FGraph::getGraphName(env2.clone())?)?;
                    r#mod = Mod::getClassModifier(env_1.clone(), (id.clone()).clone())?;
                    (cache, env2, _, _, _) = Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    (cache, c_1, env2, prevFrames) = lookupClassInEnv(cache.clone(), env2.clone(), (ident.clone()).clone(), prevFrames.clone(), Mutable::create(true), inInfo.clone())?;
                    (cache, more) = moreLookupUnqualifiedImportedClassInFrame(cache.clone(), rest.clone(), env.clone(), (ident.clone()).clone())?;
                    unique = boolNot(more.clone());
                    Ok((cache.clone(), c_1.clone(), env2.clone(), prevFrames.clone(), unique.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::List::Cons { head: _, tail: rest }, env, ident) => {
                    let mut c: Arc<SCode::Element>;
                    let mut unique: bool = false;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, c, env_1, prevFrames, unique) = lookupUnqualifiedImportedClassInFrame(cache.clone(), rest.clone(), env.clone(), (ident.clone()).clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone(), unique.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClass, outEnv, outPrevFrames, outBoolean))
}

pub fn lookupRecordConstructorClass(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    (outCache, outClass, outEnv) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inPath.clone())) {
        (cache, env, path) => {
            let mut c: Arc<SCode::Element>;
            let mut env_1: FCore::Graph;
            let mut cache = (*cache).clone();
            (cache, c, env_1) = lookupClass(cache.clone(), env.clone(), path.clone(), None)?;
            ::match_deref::match_deref! { match &(c.clone()) {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. } => (),
                _ => bail!("pattern mismatch"),
            } };
            (cache, _, c) = buildRecordConstructorClass(cache.clone(), env_1.clone(), c.clone())?;
            (cache.clone(), c.clone(), env_1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outClass, outEnv))
}

pub fn lookupConnectorVar(mut env: FCore::Graph, mut cr: Arc<DAE::ComponentRef>, mut firstId: bool) -> Result<(Arc<DAE::Attributes>, Arc<DAE::Type>, FCore::Status, bool)> {
    let mut attr: Arc<DAE::Attributes>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut status: FCore::Status = FCore::Status::CLS_FULL;
    let mut isExpandable: bool = false;
    let mut comp_env: FCore::Graph;
    let mut parent_attr: Arc<DAE::Attributes>;
    (attr, ty, status) = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupConnectorVar2(env.clone(), (var_field!((*cr).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone())?) {
                (Deref @ DAE::Var { ty: __pa0, attributes: __pa1, .. }, __pa2, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            attr = __pa1.clone();
            status = __pa2.clone();
            ty = checkSubscripts(ty.clone(), var_field!((*cr).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
            (attr.clone(), ty.clone(), status.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(lookupConnectorVar2(env.clone(), (var_field!((*cr).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone())?) {
                (Deref @ DAE::Var { ty: __pa0, attributes: __pa1, .. }, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            parent_attr = __pa1.clone();
            status = __pa2.clone();
            comp_env = __pa3.clone();
            if FCore::isDeletedComp(status.clone()) {
                attr = parent_attr.clone();
            } else {
                match '__try4: {
                    (attr, ty, status, isExpandable) = unwrap_break_err!(lookupConnectorVar(comp_env.clone(), var_field!((*cr).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), false), '__try4);
                    Ok::<_, anyhow::Error>((attr.clone(), isExpandable.clone(), status.clone(), ty.clone()))
                } {
                    Ok((__try4_o0, __try4_o1, __try4_o2, __try4_o3)) => {
                        attr = __try4_o0;
                        isExpandable = __try4_o1;
                        status = __try4_o2;
                        ty = __try4_o3;
                    }
                    Err(_) => {
                        if Types::isExpandableConnector(ty.clone()) {
                            attr = parent_attr.clone();
                            isExpandable = true;
                        } else {
                            bail!("fail");
                        }
                        bail!("try/else: outputs not set in else branch");
                    }
                }
                attr = DAEUtil::setAttrVariability(attr.clone(), SCodeUtil::variabilityOr(DAEUtil::getAttrVariability(attr.clone()), DAEUtil::getAttrVariability(parent_attr.clone())));
                if firstId.clone() {
                    attr = DAEUtil::setAttrInnerOuter(attr.clone(), DAEUtil::getAttrInnerOuter(parent_attr.clone()));
                }
            }
            (attr.clone(), ty.clone(), status.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((attr, ty, status, isExpandable))
}

fn lookupConnectorVar2(mut env: FCore::Graph, mut name: ArcStr) -> Result<(Arc<DAE::Var>, FCore::Status, FCore::Graph)> {
    let mut var: Arc<DAE::Var>;
    let mut status: FCore::Status = FCore::Status::CLS_FULL;
    let mut compEnv: FCore::Graph;
    let mut scope: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
    let FCore::G { scope: __pa0, .. } = (env.clone()) else { bail!("pattern mismatch") };
    scope = __pa0.clone();
    for mut r in &*scope.clone() {
        let mut r = r.clone();
        ht = FNode::children(FNode::fromRef(r.clone())?)?;
        if '__try1: {
            (var, _, _, status, compEnv) = unwrap_break_err!(lookupVar2(ht.clone(), (name.clone()).clone(), env.clone()), '__try1);
            return Ok((var, status, compEnv));
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            let true = (FNode::isImplicitRefName(r.clone())?) else { bail!("pattern mismatch") };
        }
    }
    bail!("fail");
    Ok((var, status, compEnv))
}

pub fn lookupVar(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<(FCore::Cache, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut outSplicedExpData: InstTypes::SplicedExpData;
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outAttributes, outType, outBinding, constOfForIteratorRange, outSplicedExpData, outClassEnv, outComponentEnv, name) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut componentEnv: FCore::Graph;
                    let mut classEnv: FCore::Graph;
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, classEnv, componentEnv, name) = lookupVarInternal(cache.clone(), env.clone(), cref.clone(), crate::InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN)?;
                    Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), classEnv.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut componentEnv: FCore::Graph;
                    let mut classEnv: FCore::Graph;
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    (cache, classEnv, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env.clone(), cref.clone(), metamodelica::nil(), Mutable::create(false))?;
                    checkPackageVariableConstant(env.clone(), classEnv.clone(), componentEnv.clone(), attr.clone(), ty.clone(), cref.clone())?;
                    Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), classEnv.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _) => {
                    if !((Config::getGraphicsExpMode()?)) { bail!("guard") }
                    Ok((cache.clone(), DAE::dummyAttrConst.clone(), DAE::T_UNKNOWN_DEFAULT.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), None, InstTypes::SplicedExpData { splicedExp: None, identType: DAE::T_UNKNOWN_DEFAULT.clone() }, env.clone(), env.clone(), literal!("#varNotFound#")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outAttributes, outType, outBinding, constOfForIteratorRange, outSplicedExpData, outClassEnv, outComponentEnv, name))
}

pub fn lookupVarIdent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ident: ArcStr, mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<(FCore::Cache, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut outSplicedExpData: InstTypes::SplicedExpData;
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outAttributes, outType, outBinding, constOfForIteratorRange, outSplicedExpData, outClassEnv, outComponentEnv, name) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env) = __mc_input.clone() else { bail!("nomatch") };
            let mut attr: Arc<DAE::Attributes>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let mut componentEnv: FCore::Graph;
            let mut classEnv: FCore::Graph;
            let mut splicedExpData: InstTypes::SplicedExpData;
            let mut cnstForRange: Option<DAE::Const> = None;
            let mut name: ArcStr = name.clone();
            (cache, attr, ty, binding, cnstForRange, splicedExpData, classEnv, componentEnv, name) = lookupVarInternalIdent(cache.clone(), env.clone(), (ident.clone()).clone(), ss.clone(), crate::InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN)?;
            Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), classEnv.clone(), componentEnv.clone(), name.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env) = __mc_input.clone() else { bail!("nomatch") };
            let mut attr: Arc<DAE::Attributes>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let mut componentEnv: FCore::Graph;
            let mut classEnv: FCore::Graph;
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut splicedExpData: InstTypes::SplicedExpData;
            let mut cnstForRange: Option<DAE::Const> = None;
            let mut name: ArcStr = name.clone();
            cref = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), ss.clone());
            (cache, classEnv, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env.clone(), cref.clone(), metamodelica::nil(), Mutable::create(false))?;
            checkPackageVariableConstant(env.clone(), classEnv.clone(), componentEnv.clone(), attr.clone(), ty.clone(), cref.clone())?;
            Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), classEnv.clone(), componentEnv.clone(), name.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outAttributes, outType, outBinding, constOfForIteratorRange, outSplicedExpData, outClassEnv, outComponentEnv, name))
}

fn checkPackageVariableConstant(mut parentEnv: FCore::Graph, mut classEnv: FCore::Graph, mut componentEnv: FCore::Graph, mut attr: Arc<DAE::Attributes>, mut tp: Arc<DAE::Type>, mut cref: Arc<DAE::ComponentRef>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (parentEnv.clone(), classEnv.clone(), componentEnv.clone(), attr.clone(), tp.clone(), cref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Attributes { variability: SCode::Variability::CONST, .. }, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s1 = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
                    s2 = (FGraph::printGraphPathStr(classEnv.clone())?).clone();
                    Error::addMessage(Error::PACKAGE_VARIABLE_NOT_CONSTANT.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()])?;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Lookup.checkPackageVariableConstant failed: ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn lookupVarInternal(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut searchStrategy: InstTypes::SearchStrategy) -> Result<(FCore::Cache, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outClassEnv, outComponentEnv, name) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), searchStrategy.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, r#ref, _) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    ht = FNode::children(FNode::fromRef(r.clone())?)?;
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarF(cache.clone(), ht.clone(), r#ref.clone(), inEnv.clone())?;
                    Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), inEnv.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, r#ref, _) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut name: ArcStr = name.clone();
                    let true = (FNode::isImplicitRefName(r.clone())?) else { bail!("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, env, componentEnv, name) = lookupVarInternal(cache.clone(), env.clone(), r#ref.clone(), searchStrategy.clone())?;
                    Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), env.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }, r#ref, InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let true = (Builtin::variableIsBuiltin(r#ref.clone())?) else { bail!("pattern mismatch") };
                    env = FGraph::topScope(inEnv.clone())?;
                    ht = FNode::children(FNode::fromRef(FGraph::lastScopeRef(env.clone())?)?)?;
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarF(cache.clone(), ht.clone(), r#ref.clone(), env.clone())?;
                    Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), env.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outClassEnv, outComponentEnv, name))
}

pub fn lookupVarInternalIdent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ident: ArcStr, mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut searchStrategy: InstTypes::SearchStrategy) -> Result<(FCore::Cache, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outClassEnv, outComponentEnv, name) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), searchStrategy.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, _) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    ht = FNode::children(FNode::fromRef(r.clone())?)?;
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarFIdent(cache.clone(), ht.clone(), (ident.clone()).clone(), ss.clone(), inEnv.clone())?;
                    Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), inEnv.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, _) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let true = (FNode::isImplicitRefName(r.clone())?) else { bail!("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, env, componentEnv, name) = lookupVarInternalIdent(cache.clone(), env.clone(), (ident.clone()).clone(), ss.clone(), searchStrategy.clone())?;
                    Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), env.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }, InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut name: ArcStr = name.clone();
                    let true = (Builtin::variableNameIsBuiltin((ident.clone()).clone())?) else { bail!("pattern mismatch") };
                    env = FGraph::topScope(inEnv.clone())?;
                    ht = FNode::children(FNode::fromRef(FGraph::lastScopeRef(env.clone())?)?)?;
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarFIdent(cache.clone(), ht.clone(), (ident.clone()).clone(), ss.clone(), env.clone())?;
                    Ok((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), env.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outClassEnv, outComponentEnv, name))
}

fn frameIsImplAddedScope(mut f: FCore::Node) -> Result<bool> {
    let mut b: bool = false;
    b = (match f.clone() {
        FCore::Node { name: mut oname, .. } => {
            FCore::isImplicitScope((oname.clone()).clone())?
        },
        _ => {
            false
        },
    });
    Ok(b)
}

pub fn lookupVarInPackages(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inState: Mutable::Mutable<bool>) -> Result<(FCore::Cache, FCore::Graph, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClassEnv: FCore::Graph;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outClassEnv, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outComponentEnv, name) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), inPrevFrames.clone(), inState.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref, subscriptLst: Deref @ metamodelica::List::Nil, ident: id, .. }, prevFrames, _) => {
                    let mut c: Arc<SCode::Element>;
                    let mut n: ArcStr = arcstr::literal!("");
                    let mut encflag: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut r: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut env5: FCore::Graph;
                    let mut p_env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut f: metamodelica::Array<FCore::Node>;
                    let mut rr: metamodelica::Array<FCore::Node>;
                    let mut of: Option<metamodelica::Array<FCore::Node>> = None;
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    (of, prevFrames) = lookupPrevFrames((id.clone()).clone(), prevFrames.clone())?;
                    let _ = (match of.clone() {
        Some(mut f) => {
                    Mutable::update(inState.clone(), true);
                    env5 = FGraph::pushScopeRef(env.clone(), f.clone())?;
                    ()
        },
        None => {
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5, __pa6) = ::match_deref::match_deref! { match &(lookupClassInEnv(cache.clone(), env.clone(), (id.clone()).clone(), prevFrames.clone(), Mutable::create(true), None)?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { restriction: __pa1, encapsulatedPrefix: __pa2, name: __pa3, .. }, __pa5, __pa6) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    r = __pa1.clone();
                    encflag = __pa2.clone();
                    n = __pa3.clone();
                    c = __pa4.clone();
                    env2 = __pa5.clone();
                    prevFrames = __pa6.clone();
                    Mutable::update(inState.clone(), true);
                    rr = FNode::child(FGraph::lastScopeRef(env2.clone())?, (id.clone()).clone())?;
                    if FNode::isRefInstance(rr.clone())? {
                        (cache, env5) = Inst::getCachedInstance(cache.clone(), env2.clone(), (id.clone()).clone(), rr.clone())?;
                    } else {
                        env3 = FGraph::openScope(env2.clone(), encflag.clone(), (n.clone()).clone(), FGraph::restrictionToScopeType(r.clone()))?;
                        ci_state = ClassInfUtil::start(r.clone(), FGraph::getGraphName(env3.clone())?)?;
                        r#mod = Mod::getClassModifier(env2.clone(), (n.clone()).clone())?;
                        (cache, env5, _, _, _, _, _, _, _, _, _, _) = Inst::instClassIn(cache.clone(), env3.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), false, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone(), None)?;
                    }
                    ()
        },
    });
                    (cache, p_env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env5.clone(), cref.clone(), prevFrames.clone(), inState.clone())?;
                    splicedExpData = prefixSplicedExp(ComponentReferenceBasics::crefFirstCref(inComponentRef.clone())?, splicedExpData.clone())?;
                    Ok((cache.clone(), p_env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _, _) => {
                    let mut componentEnv: FCore::Graph;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    (cache, env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackagesIdent(cache.clone(), env.clone(), (var_field!((**cr).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), var_field!((**cr).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), inPrevFrames.clone(), inState.clone())?;
                    Ok((cache.clone(), env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, _, _) => {
                    let mut componentEnv: FCore::Graph;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut cache = (*cache).clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut name: ArcStr = name.clone();
                    ht = FNode::children(FNode::fromRef(FGraph::lastScopeRef(env.clone())?)?)?;
                    (cache, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarF(cache.clone(), ht.clone(), cr.clone(), env.clone())?;
                    Ok((cache.clone(), env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: f, tail: fs }, .. }, cr @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, prevFrames, _) => {
                    let mut env: FCore::Graph;
                    let mut p_env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut cache = (*cache).clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut name: ArcStr = name.clone();
                    let false = (Mutable::access(inState.clone())) else { bail!("pattern mismatch") };
                    env = FGraph::setScope(inEnv.clone(), fs.clone())?;
                    (cache, p_env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env.clone(), cr.clone(), cons(f.clone(), prevFrames.clone()), inState.clone())?;
                    Ok((cache.clone(), p_env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClassEnv, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outComponentEnv, name))
}

pub fn lookupVarInPackagesIdent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut id: ArcStr, mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inState: Mutable::Mutable<bool>) -> Result<(FCore::Cache, FCore::Graph, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClassEnv: FCore::Graph;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outClassEnv, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outComponentEnv, name) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPrevFrames.clone(), inState.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _) => {
                    let mut componentEnv: FCore::Graph;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    (cache, attr, ty, bind, cnstForRange, splicedExpData, _, componentEnv, name) = lookupVarInternalIdent(cache.clone(), env.clone(), (id.clone()).clone(), ss.clone(), crate::InstTypes::SearchStrategy::SEARCH_LOCAL_ONLY)?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _) => {
                    let mut componentEnv: FCore::Graph;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut cache = (*cache).clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut name: ArcStr = name.clone();
                    ht = FNode::children(FNode::fromRef(FGraph::lastScopeRef(env.clone())?)?)?;
                    (cache, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarFIdent(cache.clone(), ht.clone(), (id.clone()).clone(), ss.clone(), env.clone())?;
                    Ok((cache.clone(), env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, prevFrames, _) => {
                    let mut p_env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut node: FCore::Node;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut f: metamodelica::Array<FCore::Node>;
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut unique: bool = false;
                    let mut qimports: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
                    let mut uqimports: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
                    node = FNode::fromRef(FGraph::lastScopeRef(env.clone())?)?;
                    (qimports, uqimports) = FNode::imports(node.clone())?;
                    let _ = 'mc: {
        let __mc_input = (qimports.clone(), uqimports.clone());
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
                            let mut bind: Arc<DAE::Binding>;
                            let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;
                            let mut cnstForRange: Option<DAE::Const>;
                            let mut cr: Arc<DAE::ComponentRef>;
                            let mut name: ArcStr = name.clone();
                            let mut cache: FCore::Cache;
                            let mut componentEnv: FCore::Graph;
                            let mut env: FCore::Graph;
                            let mut attr: Arc<DAE::Attributes>;
                            let mut p_env: FCore::Graph;
                            let mut splicedExpData: InstTypes::SplicedExpData;
                            let mut f: metamodelica::Array<FCore::Node>;
                            let mut ty: Arc<DAE::Type>;
                            cr = lookupQualifiedImportedVarInFrame(qimports.clone(), (id.clone()).clone())?;
                            Mutable::update(inState.clone(), true);
                            cr = if (FNode::name(FNode::fromRef(FGraph::lastScopeRef(env.clone())?)?)? == ComponentReferenceBasics::crefFirstIdent(cr.clone())?) {ComponentReference::crefStripFirstIdent(cr.clone())?} else {cr.clone()};
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(env.clone())?.reverse()) {
                                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                                        _ => bail!("pattern mismatch"),
                            } };
                            f = __pa0.clone();
                            prevFrames = __pa1.clone();
                            env = FGraph::setScope(env.clone(), list![f.clone()])?;
                            (cache, p_env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env.clone(), cr.clone(), prevFrames.clone(), inState.clone())?;
                            Ok(())
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        (_, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                            let mut cnstForRange: Option<DAE::Const>;
                            let mut p_env: FCore::Graph;
                            let mut bind: Arc<DAE::Binding>;
                            let mut unique: bool;
                            let mut splicedExpData: InstTypes::SplicedExpData;
                            let mut componentEnv: FCore::Graph;
                            let mut ty: Arc<DAE::Type>;
                            let mut name: ArcStr = name.clone();
                            let mut attr: Arc<DAE::Attributes>;
                            let mut cache: FCore::Cache;
                            (cache, p_env, attr, ty, bind, cnstForRange, unique, splicedExpData, componentEnv, name) = lookupUnqualifiedImportedVarInFrame(cache.clone(), uqimports.clone(), env.clone(), (id.clone()).clone())?;
                            reportSeveralNamesError(unique.clone(), (id.clone()).clone())?;
                            Mutable::update(inState.clone(), true);
                            Ok(())
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    Ok((cache.clone(), p_env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: f, tail: fs }, .. }, prevFrames, _) => {
                    let mut env: FCore::Graph;
                    let mut p_env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut cache = (*cache).clone();
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut name: ArcStr = name.clone();
                    let false = (Mutable::access(inState.clone())) else { bail!("pattern mismatch") };
                    env = FGraph::setScope(inEnv.clone(), fs.clone())?;
                    (cache, p_env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackagesIdent(cache.clone(), env.clone(), (id.clone()).clone(), ss.clone(), cons(f.clone(), prevFrames.clone()), inState.clone())?;
                    Ok((cache.clone(), p_env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClassEnv, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outComponentEnv, name))
}

pub fn lookupVarLocal(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<(FCore::Cache, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outClassEnv, outComponentEnv, name) = lookupVarInternal(inCache.clone(), inEnv.clone(), inComponentRef.clone(), crate::InstTypes::SearchStrategy::SEARCH_LOCAL_ONLY)?;
    Ok((outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outClassEnv, outComponentEnv, name))
}

pub fn lookupIdentLocal(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIdent: ArcStr) -> Result<(FCore::Cache, Arc<DAE::Var>, Arc<SCode::Element>, Arc<DAE::Mod>, FCore::Status, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outVar: Arc<DAE::Var>;
    let mut outElement: Arc<SCode::Element>;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut instStatus: FCore::Status = FCore::Status::CLS_FULL;
    let mut outComponentEnv: FCore::Graph;
    (outCache, outVar, outElement, outMod, instStatus, outComponentEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, id) => {
                    let mut fv: Arc<DAE::Var>;
                    let mut c: Arc<SCode::Element>;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut i: FCore::Status = FCore::Status::CLS_FULL;
                    let mut componentEnv: FCore::Graph;
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    ht = FNode::children(FNode::fromRef(r.clone())?)?;
                    (fv, c, m, i, componentEnv) = lookupVar2(ht.clone(), (id.clone()).clone(), inEnv.clone())?;
                    Ok((cache.clone(), fv.clone(), c.clone(), m.clone(), i.clone(), componentEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, id) => {
                    let mut fv: Arc<DAE::Var>;
                    let mut c: Arc<SCode::Element>;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut i: FCore::Status = FCore::Status::CLS_FULL;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let true = (FNode::isImplicitRefName(r.clone())?) else { bail!("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, fv, c, m, i, componentEnv) = lookupIdentLocal(cache.clone(), env.clone(), (id.clone()).clone())?;
                    Ok((cache.clone(), fv.clone(), c.clone(), m.clone(), i.clone(), componentEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outVar, outElement, outMod, instStatus, outComponentEnv))
}

pub fn lookupClassLocal(mut inEnv: FCore::Graph, mut inIdent: ArcStr) -> Result<(Arc<SCode::Element>, FCore::Graph)> {
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    (outClass, outEnv) = (::match_deref::match_deref! { match &((inEnv.clone(), inIdent.clone())) {
        (env @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, id) => {
            let mut cl: Arc<SCode::Element>;
            let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
            let mut r = (*r).clone();
            ht = FNode::children(FNode::fromRef(r.clone())?)?;
            r = FCore::RefTree::get(ht.clone(), (id.clone()).clone())?;
            let FCore::N { data: FCore::CL { e: __pa0, .. }, .. } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
            cl = __pa0.clone();
            (cl.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outClass, outEnv))
}

pub fn lookupIdent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIdent: ArcStr) -> Result<(FCore::Cache, Arc<DAE::Var>, Arc<SCode::Element>, Arc<DAE::Mod>, FCore::Status, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outVar: Arc<DAE::Var>;
    let mut outElement: Arc<SCode::Element>;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut instStatus: FCore::Status = FCore::Status::CLS_FULL;
    let mut outEnv: FCore::Graph;
    (outCache, outVar, outElement, outMod, instStatus, outEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, id) => {
                    let mut fv: Arc<DAE::Var>;
                    let mut c: Arc<SCode::Element>;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut i: FCore::Status = FCore::Status::CLS_FULL;
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    ht = FNode::children(FNode::fromRef(r.clone())?)?;
                    (fv, c, m, i, _) = lookupVar2(ht.clone(), (id.clone()).clone(), inEnv.clone())?;
                    Ok((cache.clone(), fv.clone(), c.clone(), m.clone(), i.clone(), inEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, id) => {
                    let mut fv: Arc<DAE::Var>;
                    let mut c: Arc<SCode::Element>;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut i: FCore::Status = FCore::Status::CLS_FULL;
                    let mut e: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (e, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, fv, c, m, i, e) = lookupIdent(cache.clone(), e.clone(), (id.clone()).clone())?;
                    Ok((cache.clone(), fv.clone(), c.clone(), m.clone(), i.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outVar, outElement, outMod, instStatus, outEnv))
}

// Function lookup
pub fn lookupFunctionsInEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inId: Arc<Absyn::Path>, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outTypesTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outCache, outTypesTypeLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inId.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::QUALIFIED { name, path: id }, info) => {
                    let mut cenv: FCore::Graph;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    ErrorExt::setCheckpoint((literal!("functionViaComponentRef")).clone());
                    (cache, _, _, _, _, _, _, cenv, _) = lookupVarIdent(cache.clone(), env.clone(), (name.clone()).clone(), metamodelica::nil())?;
                    (cache, res) = lookupFunctionsInEnv(cache.clone(), cenv.clone(), id.clone(), info.clone())?;
                    ErrorExt::rollBack((literal!("functionViaComponentRef")).clone());
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::Path::QUALIFIED { name: _, path: _ }, _) => {
                    ErrorExt::rollBack((literal!("functionViaComponentRef")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, id, _) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    env = FGraph::selectScope(env.clone(), id.clone())?;
                    name = (AbsynUtil::pathLastIdent(id.clone())?).clone();
                    (cache, res) = lookupFunctionsInEnv(cache.clone(), env.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inInfo.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::IDENT { name: r#str }, info) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut httypes: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let _ = Static::elabBuiltinHandler((r#str.clone()).clone())?;
                    env = FGraph::topScope(env.clone())?;
                    ht = FNode::children(FNode::fromRef(FGraph::lastScopeRef(env.clone())?)?)?;
                    httypes = getHtTypes(FGraph::lastScopeRef(env.clone())?)?;
                    (cache, res) = lookupFunctionsInFrame(cache.clone(), ht.clone(), httypes.clone(), env.clone(), (r#str.clone()).clone(), info.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::IDENT { name: r#str @ Deref @ "cardinality" }, _) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut env = (*env).clone();
                    env = FGraph::topScope(env.clone())?;
                    res = createGenericBuiltinFunctions(env.clone(), (r#str.clone()).clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, id, info) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(id.clone()) {
                            Deref @ Absyn::Path::FULLYQUALIFIED { path: _ } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), env.clone(), id.clone(), false, info.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::FULLYQUALIFIED { path: id }, info) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    env = FGraph::topScope(env.clone())?;
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), env.clone(), id.clone(), true, info.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, id, _) => {
                    let mut env_1: FCore::Graph;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut info: SourceInfo;
                    let mut cache = (*cache).clone();
                    let mut id = (*id).clone();
                    id = (::match_deref::match_deref! { match &(id.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "Clock" } => Arc::new(Absyn::Path::QUALIFIED { name: (literal!("OpenModelica")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Internal")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("ClockConstructor")).clone() }) }) }),
        _ => id.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(lookupClass(cache.clone(), env.clone(), id.clone(), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { info: __pa1, classDef: Deref @ SCode::ClassDef::OVERLOAD { pathLst: __pa2 }, .. }, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    info = __pa1.clone();
                    names = __pa2.clone();
                    env_1 = __pa3.clone();
                    (cache, res) = lookupFunctionsListInEnv(cache.clone(), env_1.clone(), names.clone(), info.clone(), metamodelica::nil())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, id, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lookupFunctionsInEnv failed on: ")); __mm_s.push_str(&*AbsynUtil::pathString(id.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outTypesTypeLst))
}

pub fn lookupFunctionsListInEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIds: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut info: SourceInfo, mut inAcc: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outTypesTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outCache, outTypesTypeLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIds.clone(), info.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::List::Nil, _, acc) => {
                    Ok((cache.clone(), acc.clone().reverse()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: id, tail: ids }, _, acc) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut acc = (*acc).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupFunctionsInEnv(cache.clone(), env.clone(), id.clone(), info.clone())?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    res = __pa1.clone();
                    (cache, acc) = lookupFunctionsListInEnv(cache.clone(), env.clone(), ids.clone(), info.clone(), listAppend(res.clone(), acc.clone()))?;
                    Ok((cache.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, Deref @ metamodelica::List::Cons { head: id, tail: _ }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(id.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" not found in scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outTypesTypeLst))
}

fn lookupFunctionsInEnv2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut followedQual: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outTypesTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outCache, outTypesTypeLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPath.clone(), followedQual.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, Deref @ Absyn::Path::IDENT { name: r#str }, _, _) => {
                    let mut httypes: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    ht = FNode::children(FNode::fromRef(r.clone())?)?;
                    httypes = getHtTypes(r.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupFunctionsInFrame(cache.clone(), ht.clone(), httypes.clone(), inEnv.clone(), (r#str.clone()).clone(), info.clone())?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    res = __pa1.clone();
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, id @ Deref @ Absyn::Path::IDENT { .. }, _, _) => {
                    let mut httypes: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut c: Arc<SCode::Element>;
                    let mut restr: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut cache = (*cache).clone();
                    let mut r = (*r).clone();
                    let (__pa0, __pa3, __pa1, __pa2, __pa4) = ::match_deref::match_deref! { match &(lookupClass(cache.clone(), inEnv.clone(), id.clone(), None)?) {
                        (__pa0, __pa3 @ Deref @ SCode::Element::CLASS { restriction: __pa1, name: __pa2, .. }, __pa4) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    restr = __pa1.clone();
                    r#str = __pa2.clone();
                    c = __pa3.clone();
                    env_1 = __pa4.clone();
                    let true = (SCodeUtil::isFunctionRestriction(restr.clone())) else { bail!("pattern mismatch") };
                    let (__pa5, __pa7, __pa6) = ::match_deref::match_deref! { match &(InstFunction::implicitFunctionTypeInstantiation(cache.clone(), env_1.clone(), InnerOuter::emptyInstHierarchy().clone(), c.clone())?) {
                        (__pa5, __pa7 @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: __pa6, tail: _ }, .. }, _) => (__pa5.clone(), __pa7.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa5.clone();
                    r = __pa6.clone();
                    env_2 = __pa7.clone();
                    ht = FNode::children(FNode::fromRef(r.clone())?)?;
                    httypes = getHtTypes(r.clone())?;
                    let (__pa8, __pa9) = ::match_deref::match_deref! { match &(lookupFunctionsInFrame(cache.clone(), ht.clone(), httypes.clone(), env_2.clone(), (r#str.clone()).clone(), info.clone())?) {
                        (__pa8, __pa9 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (__pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa8.clone();
                    res = __pa9.clone();
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, Deref @ Absyn::Path::QUALIFIED { path, name: pack }, _, _) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut env_1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut c: Arc<SCode::Element>;
                    let mut encflag: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut restr: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut ci_state: ClassInf::State;
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut r = (*r).clone();
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(lookupClass(cache.clone(), inEnv.clone(), Arc::new(Absyn::Path::IDENT { name: (pack.clone()).clone() }), None)?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { restriction: __pa1, encapsulatedPrefix: __pa2, name: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    restr = __pa1.clone();
                    encflag = __pa2.clone();
                    r#str = __pa3.clone();
                    c = __pa4.clone();
                    env_1 = __pa5.clone();
                    r = FNode::child(FGraph::lastScopeRef(env_1.clone())?, (r#str.clone()).clone())?;
                    if FNode::isRefInstance(r.clone())? {
                        (cache, env2) = Inst::getCachedInstance(cache.clone(), env_1.clone(), (r#str.clone()).clone(), r.clone())?;
                    } else {
                        env2 = FGraph::openScope(env_1.clone(), encflag.clone(), (r#str.clone()).clone(), FGraph::restrictionToScopeType(restr.clone()))?;
                        ci_state = ClassInfUtil::start(restr.clone(), FGraph::getGraphName(env2.clone())?)?;
                        r#mod = Mod::getClassModifier(env_1.clone(), (r#str.clone()).clone())?;
                        (cache, env2, _, _, _) = Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    }
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), env2.clone(), path.clone(), true, info.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, id, false, _) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut env: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let false = (FNode::isEncapsulated(FNode::fromRef(r.clone())?)?) else { bail!("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), env.clone(), id.clone(), false, info.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, id @ Deref @ Absyn::Path::IDENT { .. }, false, _) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut env: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let true = (FNode::isEncapsulated(FNode::fromRef(r.clone())?)?) else { bail!("pattern mismatch") };
                    env = FGraph::topScope(inEnv.clone())?;
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), env.clone(), id.clone(), true, info.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outTypesTypeLst))
}

fn createGenericBuiltinFunctions(mut inEnv: FCore::Graph, mut inString: ArcStr) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> {
    let mut outTypesTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    outTypesTypeLst = (::match_deref::match_deref! { match &((inEnv.clone(), inString.clone())) {
        (_, Deref @ "cardinality") => {
            list![Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$$")).clone() }), isExpandable: false }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("cardinality")).clone() }) }), Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$$")).clone() }), isExpandable: true }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("cardinality")).clone() }) })]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTypesTypeLst)
}

// - Internal functions
//   Type lookup
fn lookupTypeInEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut id: ArcStr) -> Result<(FCore::Cache, Arc<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }) => {
                    let mut c: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_1: FCore::Graph;
                    let mut httypes: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut cache = (*cache).clone();
                    ht = FNode::children(FNode::fromRef(r.clone())?)?;
                    httypes = getHtTypes(r.clone())?;
                    (cache, c, env_1) = lookupTypeInFrame(cache.clone(), ht.clone(), httypes.clone(), env.clone(), (id.clone()).clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }) => {
                    let mut c: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    (env, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, c, env_1) = lookupTypeInEnv(cache.clone(), env.clone(), (id.clone()).clone())?;
                    env_1 = FGraph::pushScopeRef(env_1.clone(), r.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outType, outEnv))
}

fn getHtTypes(mut inParentRef: metamodelica::Array<FCore::Node>) -> Result<Arc<FCore::RefTree::Tree>> {
    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
    ht = 'mc: {
        let __mc_input = inParentRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r: metamodelica::Array<FCore::Node>;
            let mut ht: Arc<FCore::RefTree::Tree> = ht.clone();
            r = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone())?;
            ht = FNode::children(FNode::fromRef(r.clone())?)?;
            Ok(ht.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(FCore::RefTree::new())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ht)
}

fn lookupTypeInFrame(mut inCache: FCore::Cache, mut inBinTree1: Arc<FCore::RefTree::Tree>, mut inBinTree2: Arc<FCore::RefTree::Tree>, mut inEnv3: FCore::Graph, mut inIdent4: ArcStr) -> Result<(FCore::Cache, Arc<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = (::match_deref::match_deref! { match &((inCache.clone(), inBinTree1.clone(), inBinTree2.clone(), inEnv3.clone(), inIdent4.clone())) {
        (cache, _, httypes, env, id) => {
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut item: FCore::Node;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            item = FNode::fromRef(FCore::RefTree::get(httypes.clone(), (id.clone()).clone())?)?;
            (cache, t, env) = lookupTypeInFrame2(cache.clone(), item.clone(), env.clone(), (id.clone()).clone())?;
            (cache.clone(), t.clone(), env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outType, outEnv))
}

fn lookupTypeInFrame2(mut inCache: FCore::Cache, mut item: FCore::Node, mut inEnv3: FCore::Graph, mut inIdent4: ArcStr) -> Result<(FCore::Cache, Arc<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = (::match_deref::match_deref! { match &((inCache.clone(), item.clone(), inEnv3.clone(), inIdent4.clone())) {
        (cache, FCore::Node { data: FCore::Data::FT { tys: Deref @ metamodelica::List::Cons { head: t, tail: _ } }, .. }, env, _) => {
            (cache.clone(), t.clone(), env.clone())
        },
        (_, FCore::Node { data: FCore::Data::CO { e: comp, .. }, .. }, _, id) => {
            let mut info: SourceInfo;
            info = SCodeUtil::elementInfo(comp.clone());
            Error::addSourceMessage(Error::LOOKUP_TYPE_FOUND_COMP.clone(), list![(id.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (cache, FCore::Node { data: FCore::Data::CL { e: cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, .. }, .. }, env, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut env_3: FCore::Graph;
            let mut cache = (*cache).clone();
            (cache, env_3, ty) = buildRecordType(cache.clone(), env.clone(), cdef.clone())?;
            (cache.clone(), ty.clone(), env_3.clone())
        },
        (cache, FCore::Node { data: FCore::Data::CL { e: cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { .. }, .. }, .. }, .. }, env, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut env_3: FCore::Graph;
            let mut cache = (*cache).clone();
            (cache, env_3, ty) = buildMetaRecordType(cache.clone(), env.clone(), cdef.clone())?;
            (cache.clone(), ty.clone(), env_3.clone())
        },
        (cache, FCore::Node { data: FCore::Data::CL { e: cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, .. }, .. }, .. }, env, id) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cenv: FCore::Graph;
            let mut env_1: FCore::Graph;
            let mut env_3: FCore::Graph;
            let mut cache = (*cache).clone();
            cenv = env.clone();
            (cache, env_1, _) = InstFunction::implicitFunctionInstantiation(cache.clone(), cenv.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef.clone(), metamodelica::nil())?;
            (cache, ty, env_3) = lookupTypeInEnv(cache.clone(), env_1.clone(), (id.clone()).clone())?;
            (cache.clone(), ty.clone(), env_3.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outType, outEnv))
}

fn lookupFunctionsInFrame(mut inCache: FCore::Cache, mut inClasses: Arc<FCore::RefTree::Tree>, mut inFuncTypes: Arc<FCore::RefTree::Tree>, mut inEnv: FCore::Graph, mut inFuncName: ArcStr, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outFuncTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut r: metamodelica::Array<FCore::Node>;
    let mut data: FCore::Data = FCore::Data::TOP;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    match '__try0: {
        r = unwrap_break_err!(FCore::RefTree::get(inFuncTypes.clone(), (inFuncName.clone()).clone()), '__try0);
        let FCore::N { data: FCore::FT { tys: __pa1 }, .. } = (unwrap_break_err!(FNode::fromRef(r.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        outFuncTypes = __pa1.clone();
        outCache = inCache.clone();
        Ok::<_, anyhow::Error>((outCache.clone(), outFuncTypes.clone(), r.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            outCache = __try0_o0;
            outFuncTypes = __try0_o1;
            r = __try0_o2;
        }
        Err(_) => {
            r = FCore::RefTree::get(inClasses.clone(), (inFuncName.clone()).clone())?;
            let FCore::N { data: __pa2, .. } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
            data = __pa2.clone();
            (outCache, outFuncTypes) = 'mc: {
        let __mc_input = data.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(FNode::refInstVar(r.clone())?) {
                        Deref @ DAE::Var { ty: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty = __pa0.clone();
                    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { .. } => {
                    assign_variant_field!(ty => DAE::Type::T_FUNCTION; path = Arc::new(Absyn::Path::IDENT { name: (inFuncName.clone()).clone() }));
                    ty.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
                    Ok((inCache.clone(), list![ty.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Data::CO { e: _, .. } => {
                    Error::addSourceMessage(Error::LOOKUP_TYPE_FOUND_COMP.clone(), list![(inFuncName.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Data::CL { e: cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, .. } => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    (cache, _, ty) = buildRecordType(inCache.clone(), inEnv.clone(), cl.clone())?;
                    Ok((cache.clone(), list![ty.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Data::CL { e: cl, .. } => {
                    if !((SCodeUtil::isFunction(cl.clone()))) { bail!("guard") }
                    let mut tps: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    (cache, env, _) = InstFunction::implicitFunctionTypeInstantiation(inCache.clone(), inEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), cl.clone())?;
                    (cache, tps) = lookupFunctionsInEnv2(cache.clone(), env.clone(), Arc::new(Absyn::Path::IDENT { name: (inFuncName.clone()).clone() }), true, inInfo.clone())?;
                    Ok((cache.clone(), tps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Data::CL { e: cl, .. } => {
                    if !((SCodeUtil::classIsExternalObject(cl.clone()))) { bail!("guard") }
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    (cache, env, _, _, _, _, _, _, _, _) = Inst::instClass(inCache.clone(), inEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil(), false, crate::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?;
                    (cache, ty, _) = lookupTypeInEnv(cache.clone(), env.clone(), (inFuncName.clone()).clone())?;
                    Ok((cache.clone(), list![ty.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        }
    }
    Ok((outCache, outFuncTypes))
}

pub fn selectUpdatedEnv(mut inNewEnv: FCore::Graph, mut inOldEnv: FCore::Graph) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = 'mc: {
        let __mc_input = (inNewEnv.clone(), inOldEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FGraph::isTopScope(inNewEnv.clone())?) else { bail!("pattern mismatch") };
            Ok(inOldEnv.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (stringEq((FGraph::getGraphNameStr(inNewEnv.clone())?).clone(), (FGraph::getGraphNameStr(inOldEnv.clone())?).clone())) else { bail!("pattern mismatch") };
            Ok(inNewEnv.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inOldEnv.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

fn buildRecordType(mut cache: FCore::Cache, mut env: FCore::Graph, mut icdef: Arc<SCode::Element>) -> Result<(FCore::Cache, FCore::Graph, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut ftype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut name: ArcStr = arcstr::literal!("");
    let mut env_1: FCore::Graph;
    let mut cdef: Arc<SCode::Element>;
    (outCache, _, cdef) = buildRecordConstructorClass(cache.clone(), env.clone(), icdef.clone())?;
    name = (SCodeUtil::className(cdef.clone())?).clone();
    (outCache, outEnv, _) = InstFunction::implicitFunctionTypeInstantiation(outCache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), cdef.clone())?;
    (outCache, ftype, _) = lookupTypeInEnv(outCache.clone(), outEnv.clone(), (name.clone()).clone())?;
    Ok((outCache, outEnv, ftype))
}

fn buildRecordConstructorClass(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inClass: Arc<SCode::Element>) -> Result<(FCore::Cache, FCore::Graph, Arc<SCode::Element>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outClass: Arc<SCode::Element>;
    (outCache, outEnv, outClass) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cl @ Deref @ SCode::Element::CLASS { info, name: id, .. }) => {
                    let mut funcelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut reselt: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut cl = (*cl).clone();
                    (cache, env, funcelts, _) = buildRecordConstructorClass2(cache.clone(), env.clone(), cl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))?;
                    reselt = buildRecordConstructorResultElt(funcelts.clone(), (id.clone()).clone(), env.clone(), info.clone());
                    cl = Arc::new(SCode::Element::CLASS { name: (id.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_RECORD_CONSTRUCTOR }, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: cons(reselt.clone(), funcelts.clone()), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: info.clone() });
                    Ok((cache.clone(), env.clone(), cl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("buildRecordConstructorClass failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outClass))
}

fn buildRecordConstructorClass2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut cl: Arc<SCode::Element>, mut mods: Arc<DAE::Mod>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut funcelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (outCache, outEnv, funcelts, elts) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), cl.clone(), mods.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::Element::CLASS { info, name, .. }, _) => {
                    let mut cdefelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut classExtendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut eltsMods: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut fpath: Arc<Absyn::Path>;
                    let mut env1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>> = elts.clone();
                    let mut funcelts: Arc<metamodelica::List<Arc<SCode::Element>>> = funcelts.clone();
                    (cache, env, _, elts, _, _, _, _, _, _) = InstExtends::instDerivedClasses(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), true, info.clone())?;
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (name.clone()).clone(), Some(crate::FCore::ScopeType::CLASS_SCOPE))?;
                    fpath = FGraph::getGraphName(env.clone())?;
                    (cdefelts, classExtendsElts, extendsElts, compElts) = InstUtil::splitElts(elts.clone())?;
                    (cache, env, _, _, eltsMods, _, _, _, _, _) = InstExtends::instExtendsAndClassExtendsList(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, extendsElts.clone(), classExtendsElts.clone(), elts.clone(), ClassInf::State::RECORD { path: fpath.clone() }, (name.clone()).clone(), true, false)?;
                    eltsMods = listAppend(eltsMods.clone(), InstUtil::addNomod(compElts.clone()));
                    (cache, env1, _) = InstUtil::addClassdefsToEnv(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdefelts.clone(), false, None, false)?;
                    (cache, env1, _) = InstUtil::addComponentsToEnv(cache.clone(), env1.clone(), InnerOuter::emptyInstHierarchy().clone(), mods.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ClassInf::State::RECORD { path: fpath.clone() }, eltsMods.clone(), true)?;
                    (cache, env1, funcelts) = buildRecordConstructorElts(cache.clone(), env1.clone(), eltsMods.clone(), mods.clone())?;
                    Ok((cache.clone(), env1.clone(), funcelts.clone(), elts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildRecordConstructorClass2 failed, cl:")); __mm_s.push_str(&*SCodeDump::unparseElementStr(cl.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, funcelts, elts))
}

fn selectModifier(mut inModID: Arc<DAE::Mod>, mut inModNoID: Arc<DAE::Mod>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inModID.clone(), inModNoID.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::NOMOD, _) => {
                    Ok(inModNoID.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inModID.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn buildRecordConstructorElts(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inSCodeElementLst: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut mods: Arc<DAE::Mod>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outSCodeElementLst: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (outCache, outEnv, outSCodeElementLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inSCodeElementLst.clone(), mods.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Nil, _) => {
                    Ok((cache.clone(), env.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { name: id, prefixes: Deref @ SCode::Prefixes { visibility: _, redeclarePrefix: redecl, finalPrefix: f @ SCode::Final::FINAL, innerOuter: io, replaceablePrefix: repl }, attributes: SCode::Attributes { arrayDims: d, connectorType: ct, parallelism: prl, variability: var, direction: _, isField: isf }, typeSpec: tp, modifications: r#mod, comment, condition: cond, info }, cmod), tail: rest }, _) => {
                    let mut res: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut umod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut compMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut fullMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut selectedMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut cmod = (*cmod).clone();
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::COMPONENT { name: (id.clone()).clone() }, info.clone())?;
                    mod_1 = Mod::merge(mods.clone(), mod_1.clone(), (literal!("")).clone(), true)?;
                    compMod = Mod::lookupCompModification(mod_1.clone(), (id.clone()).clone())?;
                    fullMod = mod_1.clone();
                    selectedMod = selectModifier(compMod.clone(), fullMod.clone())?;
                    (cache, cmod) = Mod::updateMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cmod.clone(), true, info.clone())?;
                    selectedMod = Mod::merge(cmod.clone(), selectedMod.clone(), (literal!("")).clone(), true)?;
                    umod = Mod::unelabMod(selectedMod.clone())?;
                    (cache, env, res) = buildRecordConstructorElts(cache.clone(), env.clone(), rest.clone(), mods.clone())?;
                    dir = openmodelica_ast::Absyn::Direction::BIDIR;
                    vis = openmodelica_frontend_types::SCode::Visibility::PROTECTED;
                    Ok((cache.clone(), env.clone(), cons(Arc::new(SCode::Element::COMPONENT { name: (id.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: f.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: SCode::Attributes { arrayDims: d.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var.clone(), direction: dir.clone(), isField: isf.clone() }, typeSpec: tp.clone(), modifications: umod.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() }), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { name: id, prefixes: Deref @ SCode::Prefixes { visibility: vis, redeclarePrefix: redecl, finalPrefix: _, innerOuter: io, replaceablePrefix: repl }, attributes: SCode::Attributes { arrayDims: d, connectorType: ct, parallelism: prl, variability: SCode::Variability::CONST, direction: _, isField: isf }, typeSpec: tp, modifications: r#mod @ Deref @ SCode::Mod::NOMOD, comment, condition: cond, info }, cmod), tail: rest }, _) => {
                    let mut res: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut f: SCode::Final = SCode::Final::FINAL;
                    let mut var: SCode::Variability = SCode::Variability::CONST;
                    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut umod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut compMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut fullMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut selectedMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut vis = (*vis).clone();
                    let mut cmod = (*cmod).clone();
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::COMPONENT { name: (id.clone()).clone() }, info.clone())?;
                    mod_1 = Mod::merge(mods.clone(), mod_1.clone(), (literal!("")).clone(), true)?;
                    compMod = Mod::lookupCompModification(mod_1.clone(), (id.clone()).clone())?;
                    fullMod = mod_1.clone();
                    selectedMod = selectModifier(compMod.clone(), fullMod.clone())?;
                    (cache, cmod) = Mod::updateMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cmod.clone(), true, info.clone())?;
                    selectedMod = Mod::merge(cmod.clone(), selectedMod.clone(), (literal!("")).clone(), true)?;
                    umod = Mod::unelabMod(selectedMod.clone())?;
                    (cache, env, res) = buildRecordConstructorElts(cache.clone(), env.clone(), rest.clone(), mods.clone())?;
                    var = openmodelica_frontend_types::SCode::Variability::VAR;
                    dir = openmodelica_ast::Absyn::Direction::INPUT;
                    vis = openmodelica_frontend_types::SCode::Visibility::PUBLIC;
                    f = openmodelica_frontend_types::SCode::Final::NOT_FINAL;
                    Ok((cache.clone(), env.clone(), cons(Arc::new(SCode::Element::COMPONENT { name: (id.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: f.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: SCode::Attributes { arrayDims: d.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var.clone(), direction: dir.clone(), isField: isf.clone() }, typeSpec: tp.clone(), modifications: umod.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() }), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { name: id, prefixes: Deref @ SCode::Prefixes { visibility: _, redeclarePrefix: redecl, finalPrefix: f, innerOuter: io, replaceablePrefix: repl }, attributes: SCode::Attributes { arrayDims: d, connectorType: ct, parallelism: prl, variability: var @ SCode::Variability::CONST, direction: _, isField: isf }, typeSpec: tp, modifications: r#mod, comment, condition: cond, info }, cmod), tail: rest }, _) => {
                    let mut res: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut umod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut compMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut fullMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut selectedMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut cmod = (*cmod).clone();
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::COMPONENT { name: (id.clone()).clone() }, info.clone())?;
                    mod_1 = Mod::merge(mods.clone(), mod_1.clone(), (literal!("")).clone(), true)?;
                    compMod = Mod::lookupCompModification(mod_1.clone(), (id.clone()).clone())?;
                    fullMod = mod_1.clone();
                    selectedMod = selectModifier(compMod.clone(), fullMod.clone())?;
                    (cache, cmod) = Mod::updateMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cmod.clone(), true, info.clone())?;
                    selectedMod = Mod::merge(cmod.clone(), selectedMod.clone(), (literal!("")).clone(), true)?;
                    umod = Mod::unelabMod(selectedMod.clone())?;
                    (cache, env, res) = buildRecordConstructorElts(cache.clone(), env.clone(), rest.clone(), mods.clone())?;
                    dir = openmodelica_ast::Absyn::Direction::BIDIR;
                    vis = openmodelica_frontend_types::SCode::Visibility::PROTECTED;
                    Ok((cache.clone(), env.clone(), cons(Arc::new(SCode::Element::COMPONENT { name: (id.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: f.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: SCode::Attributes { arrayDims: d.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var.clone(), direction: dir.clone(), isField: isf.clone() }, typeSpec: tp.clone(), modifications: umod.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() }), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { name: id, prefixes: Deref @ SCode::Prefixes { visibility: _, redeclarePrefix: redecl, finalPrefix: _, innerOuter: io, replaceablePrefix: repl }, attributes: SCode::Attributes { arrayDims: d, connectorType: ct, parallelism: prl, variability: _, direction: _, isField: isf }, typeSpec: tp, modifications: r#mod, comment, condition: cond, info }, cmod), tail: rest }, _) => {
                    let mut res: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut f: SCode::Final = SCode::Final::FINAL;
                    let mut var: SCode::Variability = SCode::Variability::CONST;
                    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut umod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut compMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut fullMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut selectedMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut cmod = (*cmod).clone();
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::COMPONENT { name: (id.clone()).clone() }, info.clone())?;
                    mod_1 = Mod::merge(mods.clone(), mod_1.clone(), (literal!("")).clone(), true)?;
                    compMod = Mod::lookupCompModification(mod_1.clone(), (id.clone()).clone())?;
                    fullMod = mod_1.clone();
                    selectedMod = selectModifier(compMod.clone(), fullMod.clone())?;
                    (cache, cmod) = Mod::updateMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cmod.clone(), true, info.clone())?;
                    selectedMod = Mod::merge(cmod.clone(), selectedMod.clone(), (literal!("")).clone(), true)?;
                    umod = Mod::unelabMod(selectedMod.clone())?;
                    (cache, env, res) = buildRecordConstructorElts(cache.clone(), env.clone(), rest.clone(), mods.clone())?;
                    var = openmodelica_frontend_types::SCode::Variability::VAR;
                    vis = openmodelica_frontend_types::SCode::Visibility::PUBLIC;
                    f = openmodelica_frontend_types::SCode::Final::NOT_FINAL;
                    dir = openmodelica_ast::Absyn::Direction::INPUT;
                    Ok((cache.clone(), env.clone(), cons(Arc::new(SCode::Element::COMPONENT { name: (id.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: f.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: SCode::Attributes { arrayDims: d.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var.clone(), direction: dir.clone(), isField: isf.clone() }, typeSpec: tp.clone(), modifications: umod.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() }), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: (comp, cmod), tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Lookup.buildRecordConstructorElts failed ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(comp.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(" with mod: ")); __mm_s.push_str(&*Mod::printModStr(cmod.clone())?); __mm_s.push_str(&*literal!(" and: ")); __mm_s.push_str(&*Mod::printModStr(mods.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outSCodeElementLst))
}

fn buildRecordConstructorResultElt(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>, mut id: ArcStr, mut env: FCore::Graph, mut info: SourceInfo) -> Arc<SCode::Element> {
    let mut outElement: Arc<SCode::Element>;
    outElement = Arc::new(SCode::Element::COMPONENT { name: (literal!("result")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::OUTPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: info.clone() });
    outElement
}

fn lookupClassInEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut id: ArcStr, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inState: Mutable::Mutable<bool>, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    (outCache, outClass, outEnv, outPrevFrames) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), id.clone(), inPrevFrames.clone(), inState.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, _, prevFrames, _, _) => {
                    let mut c: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut frame: FCore::Node;
                    let mut cache = (*cache).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    frame = FNode::fromRef(r.clone())?;
                    (cache, c, env_1, prevFrames) = lookupClassInFrame(cache.clone(), frame.clone(), env.clone(), (id.clone()).clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, _, prevFrames, _, _) => {
                    let mut c: Arc<SCode::Element>;
                    let mut frame: FCore::Node;
                    let mut sid: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    let false = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
                    frame = FNode::fromRef(r.clone())?;
                    sid = (FNode::refName(r.clone())?).clone();
                    let true = (FNode::isEncapsulated(frame.clone())?) else { bail!("pattern mismatch") };
                    let true = (stringEq((id.clone()).clone(), (sid.clone()).clone())) else { bail!("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, c, env, prevFrames) = lookupClassInEnv(cache.clone(), env.clone(), (id.clone()).clone(), cons(r.clone(), prevFrames.clone()), inState.clone(), inInfo.clone())?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), c.clone(), env.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, _, _, _, Some(info)) => {
                    let mut i_env: FCore::Graph;
                    let mut frame: FCore::Node;
                    let mut scope: ArcStr = arcstr::literal!("");
                    let false = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
                    frame = FNode::fromRef(r.clone())?;
                    let true = (FNode::isEncapsulated(frame.clone())?) else { bail!("pattern mismatch") };
                    i_env = FGraph::topScope(env.clone())?;
                    if '__try0: {
                        (_, _, _, _) = unwrap_break_err!(lookupClassInEnv(cache.clone(), i_env.clone(), (id.clone()).clone(), metamodelica::nil(), inState.clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    scope = (FGraph::printGraphPathStr(env.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_ERROR.clone(), list![(id.clone()).clone(), (scope.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, _, prevFrames, _, _) => {
                    let mut c: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut i_env: FCore::Graph;
                    let mut frame: FCore::Node;
                    let mut cache = (*cache).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    frame = FNode::fromRef(r.clone())?;
                    let true = (FNode::isEncapsulated(frame.clone())?) else { bail!("pattern mismatch") };
                    i_env = FGraph::topScope(env.clone())?;
                    (cache, c, env_1, prevFrames) = lookupClassInEnv(cache.clone(), i_env.clone(), (id.clone()).clone(), metamodelica::nil(), inState.clone(), inInfo.clone())?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, _, prevFrames, _, _) => {
                    let mut c: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut frame: FCore::Node;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    let false = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
                    frame = FNode::fromRef(r.clone())?;
                    let false = (FNode::isEncapsulated(frame.clone())?) else { bail!("pattern mismatch") };
                    let false = (Mutable::access(inState.clone())) else { bail!("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, c, env_1, prevFrames) = lookupClassInEnv(cache.clone(), env.clone(), (id.clone()).clone(), cons(r.clone(), prevFrames.clone()), inState.clone(), inInfo.clone())?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn lookupClassInFrame(mut inCache: FCore::Cache, mut inFrame: FCore::Node, mut inEnv: FCore::Graph, mut inIdent: ArcStr, mut inPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>, mut inState: Mutable::Mutable<bool>, mut inInfo: Option<SourceInfo>) -> Result<(FCore::Cache, Arc<SCode::Element>, FCore::Graph, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    (outCache, outClass, outEnv, outPrevFrames) = 'mc: {
        let __mc_input = (inCache.clone(), inFrame.clone(), inEnv.clone(), inIdent.clone(), inPrevFrames.clone(), inState.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Node { children: ht, .. }, totenv, name, prevFrames, _) => {
                    let mut c: Arc<SCode::Element>;
                    let mut r: metamodelica::Array<FCore::Node>;
                    r = FCore::RefTree::get(ht.clone(), (name.clone()).clone())?;
                    let FCore::N { data: FCore::CL { e: __pa0, .. }, .. } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    c = __pa0.clone();
                    Ok((cache.clone(), c.clone(), totenv.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, totenv, name, _, _) => {
                    let mut c: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
                    let mut qimports: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
                    let mut uqimports: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
                    let mut unique: bool = false;
                    (qimports, uqimports) = FNode::imports(inFrame.clone())?;
                    let _ = 'mc: {
        let __mc_input = (qimports.clone(), uqimports.clone());
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
                            let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;
                            let mut env_1: FCore::Graph;
                            let mut c: Arc<SCode::Element>;
                            let mut cache: FCore::Cache;
                            (cache, c, env_1, prevFrames) = lookupQualifiedImportedClassInFrame(cache.clone(), qimports.clone(), totenv.clone(), (name.clone()).clone(), inState.clone(), inInfo.clone())?;
                            Ok(())
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        (_, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                            let mut c: Arc<SCode::Element>;
                            let mut unique: bool;
                            let mut env_1: FCore::Graph;
                            let mut prevFrames: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;
                            let mut cache: FCore::Cache;
                            (cache, c, env_1, prevFrames, unique) = lookupUnqualifiedImportedClassInFrame(cache.clone(), uqimports.clone(), totenv.clone(), (name.clone()).clone(), inInfo.clone())?;
                            Mutable::update(inState.clone(), true);
                            reportSeveralNamesError(unique.clone(), (name.clone()).clone())?;
                            Ok(())
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn reportSeveralNamesError(mut unique: bool, mut name: ArcStr) -> Result<()> {
    let _ = (match (unique.clone(), name.clone()) {
        (true, _) => (),
        (false, _) => {
            Error::addMessage(Error::IMPORT_SEVERAL_NAMES.clone(), list![(name.clone()).clone()])?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn lookupVar2(mut inBinTree: Arc<FCore::RefTree::Tree>, mut inIdent: ArcStr, mut inGraph: FCore::Graph) -> Result<(Arc<DAE::Var>, Arc<SCode::Element>, Arc<DAE::Mod>, FCore::Status, FCore::Graph)> {
    let mut outVar: Arc<DAE::Var>;
    let mut outElement: Arc<SCode::Element>;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut instStatus: FCore::Status = FCore::Status::CLS_FULL;
    let mut outEnv: FCore::Graph;
    let mut r: metamodelica::Array<FCore::Node>;
    let mut s: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
    let mut n: FCore::Node;
    let mut name: ArcStr = arcstr::literal!("");
    r = FCore::RefTree::get(inBinTree.clone(), (inIdent.clone()).clone())?;
    outVar = FNode::refInstVar(r.clone())?;
    s = FNode::refRefTargetScope(r.clone())?;
    n = FNode::fromRef(r.clone())?;
    if !(FNode::isComponent(n.clone())) && Flags::isSet(Flags::LOOKUP.clone())? {
        let false = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
        let FCore::N { data: FCore::CL { e: __t1, .. }, .. } = (n.clone()) else { bail!("pattern mismatch") };
        let __pa0 = ::match_deref::match_deref! { match &((*__t1).clone()) {
            Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa0.clone();
        name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIdent.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inGraph.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Lookup.lookupVar2 failed because we found a class instead of a variable: ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    let FCore::N { data: FCore::CO { e: __pa2, r#mod: __pa3, kind: _, status: __pa4 }, .. } = (n.clone()) else { bail!("pattern mismatch") };
    outElement = __pa2.clone();
    outMod = __pa3.clone();
    instStatus = __pa4.clone();
    outEnv = FGraph::setScope(inGraph.clone(), s.clone())?;
    Ok((outVar, outElement, outMod, instStatus, outEnv))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn checkSubscripts(mut inType: Arc<DAE::Type>, mut inExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Type>> {
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
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e @ Deref @ DAE::Exp::RANGE { .. } }, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dim_int: i32 = 0;
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    dim_int = Expression::rangeSize(e.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { array: se, .. } }, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dim_int: i32 = 0;
                    let _ = Expression::dimensionSize(dim.clone())?;
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    dim_int = (se.clone().len() as i32);
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(e.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    dim = __pa0.clone();
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: ind } }, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sz: i32 = 0;
                    sz = Expression::dimensionSize(dim.clone())?;
                    let true = (ind.clone() > 0) else { bail!("pattern mismatch") };
                    let true = (ind.clone() <= sz.clone()) else { bail!("pattern mismatch") };
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (Expression::dimensionKnown(dim.clone())) else { bail!("pattern mismatch") };
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, .. }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: ys }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. }, ys) => {
                    Ok(checkSubscripts(t.clone(), ys.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t @ Deref @ DAE::Type::T_UNKNOWN, _) => {
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { .. }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(var_field!((*inType).ty, DAE::Type::T_METAARRAY).clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { .. }, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, s) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Lookup.checkSubscripts failed (tp: ")).clone())?;
                    Debug::trace((TypesDump::printTypeStr(t.clone())?).clone())?;
                    Debug::trace((literal!(" subs:")).clone())?;
                    Debug::trace(stringDelimitList(List::map(s.clone(), Arc::new(ExpressionBasics::printSubscriptStr)), (literal!(",")).clone()))?;
                    Debug::trace((literal!(")\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn lookupVarF(mut inCache: FCore::Cache, mut inBinTree: Arc<FCore::RefTree::Tree>, mut inComponentRef: Arc<DAE::ComponentRef>, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut constOfForIteratorRange: Option<DAE::Const> = None;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outComponentEnv, name) = (::match_deref::match_deref! { match &((inCache.clone(), inBinTree.clone(), inComponentRef.clone(), inEnv.clone())) {
        (_, _, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: ss, ident: id, .. }, _) => {
            (outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outComponentEnv, name) = lookupVarFIdent(inCache.clone(), inBinTree.clone(), (id.clone()).clone(), ss.clone(), inEnv.clone())?;
            (outCache.clone(), outAttributes.clone(), outType.clone(), outBinding.clone(), constOfForIteratorRange.clone(), splicedExpData.clone(), outComponentEnv.clone(), name.clone())
        },
        (cache, ht, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: ids, subscriptLst: ss, ident: id, .. }, _) => {
            let mut ct: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
            let mut prl: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
            let mut vt: SCode::Variability = SCode::Variability::CONST;
            let mut vt2: SCode::Variability = SCode::Variability::CONST;
            let mut di: Absyn::Direction = Absyn::Direction::BIDIR;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut idTp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty2_2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut tyParent: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut tyChild: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let mut parentBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let mut componentEnv: FCore::Graph;
            let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
            let mut texp: Option<Arc<DAE::Exp>> = None;
            let mut xCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut tCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut ltCref: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut splicedExp: Arc<DAE::Exp>;
            let mut eType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cnstForRange: Option<DAE::Const> = None;
            let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
            let mut attr: Arc<DAE::Attributes>;
            let mut oSplicedExp: Option<Arc<DAE::Exp>> = None;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(lookupVar2(ht.clone(), (id.clone()).clone(), inEnv.clone())?) {
                (Deref @ DAE::Var { name: _, attributes: Deref @ DAE::Attributes { variability: __pa0, .. }, ty: __pa1, binding: __pa2, bind_from_outside: _, .. }, _, _, _, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            vt2 = __pa0.clone();
            tyParent = __pa1.clone();
            parentBinding = __pa2.clone();
            componentEnv = __pa3.clone();
            (attr, ty, binding, cnstForRange, componentEnv, name) = (::match_deref::match_deref! { match &(tyParent.clone()) {
        Deref @ DAE::Type::T_METAARRAY { .. } => {
            let true = ((TypesDump::getDimensions(tyParent.clone()).len() as i32) == (ss.clone().len() as i32)) else { bail!("pattern mismatch") };
            (cache, attr, ty, binding, cnstForRange, name) = lookupVarFMetaModelica(cache.clone(), componentEnv.clone(), ids.clone(), Types::metaArrayElementType(tyParent.clone())?)?;
            splicedExpData = InstTypes::SplicedExpData { splicedExp: None, identType: ty.clone() };
            (attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), componentEnv.clone(), name.clone())
        },
        _ if (Types::isBoxedType(tyParent.clone()) && !(Types::isUnknownType(tyParent.clone()))) => {
            ::match_deref::match_deref! { match &(ss.clone()) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            (cache, attr, ty, binding, cnstForRange, name) = lookupVarFMetaModelica(cache.clone(), componentEnv.clone(), ids.clone(), tyParent.clone())?;
            splicedExpData = InstTypes::SplicedExpData { splicedExp: None, identType: ty.clone() };
            (attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), componentEnv.clone(), name.clone())
        },
        _ => {
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(lookupVar(cache.clone(), componentEnv.clone(), ids.clone())?) {
                (__pa0, Deref @ DAE::Attributes { connectorType: __pa1, parallelism: __pa2, variability: __pa3, direction: __pa4, innerOuter: __pa5, visibility: __pa6 }, __pa7, __pa8, __pa9, InstTypes::SplicedExpData { splicedExp: __pa10, identType: __pa11 }, _, __pa12, __pa13) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            ct = __pa1.clone();
            prl = __pa2.clone();
            vt = __pa3.clone();
            di = __pa4.clone();
            io = __pa5.clone();
            vis = __pa6.clone();
            tyChild = __pa7.clone();
            binding = __pa8.clone();
            cnstForRange = __pa9.clone();
            texp = __pa10.clone();
            idTp = __pa11.clone();
            componentEnv = __pa12.clone();
            name = __pa13.clone();
            ltCref = elabComponentRecursive(texp.clone());
            ty = tyChild.clone();
            oSplicedExp = (::match_deref::match_deref! { match &(ltCref.clone()) {
        Deref @ metamodelica::List::Cons { head: tCref, tail: _ } => {
            ty1 = checkSubscripts(tyParent.clone(), ss.clone())?;
            ty = sliceDimensionType(ty1.clone(), tyChild.clone())?;
            ty2_2 = Types::simplifyType(tyParent.clone())?;
            ss = addArrayDimensions(ty2_2.clone(), ss.clone())?;
            xCref = ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty2_2.clone(), ss.clone(), tCref.clone());
            eType = Types::simplifyType(ty.clone())?;
            splicedExp = Expression::makeCrefExp(xCref.clone(), eType.clone())?;
            Some(splicedExp.clone())
        },
        Deref @ metamodelica::List::Nil => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            vt = SCodeUtil::variabilityOr(vt.clone(), vt2.clone());
            binding = lookupBinding(inComponentRef.clone(), tyParent.clone(), ty.clone(), parentBinding.clone(), binding.clone())?;
            splicedExpData = InstTypes::SplicedExpData { splicedExp: oSplicedExp.clone(), identType: idTp.clone() };
            (Arc::new(DAE::Attributes { connectorType: ct.clone(), parallelism: prl.clone(), variability: vt.clone(), direction: di.clone(), innerOuter: io.clone(), visibility: vis.clone() }), ty.clone(), binding.clone(), cnstForRange.clone(), componentEnv.clone(), name.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outAttributes, outType, outBinding, constOfForIteratorRange, splicedExpData, outComponentEnv, name))
}

fn lookupVarFIdent(mut cache: FCore::Cache, mut ht: Arc<FCore::RefTree::Tree>, mut ident: ArcStr, mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, InstTypes::SplicedExpData, FCore::Graph, ArcStr)> {
    let mut cache: FCore::Cache = cache;
    let mut attr: Arc<DAE::Attributes>;
    let mut ty_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut bind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut cnstForRange: Option<DAE::Const> = None;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut componentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    let mut tty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ss_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(lookupVar2(ht.clone(), (ident.clone()).clone(), inEnv.clone())?) {
        (Deref @ DAE::Var { name: __pa0, attributes: __pa1, ty: __pa2, binding: __pa3, bind_from_outside: _, constOfForIteratorRange: __pa4 }, _, _, _, __pa5) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    attr = __pa1.clone();
    ty = __pa2.clone();
    bind = __pa3.clone();
    cnstForRange = __pa4.clone();
    componentEnv = __pa5.clone();
    ty_1 = checkSubscripts(ty.clone(), ss.clone())?;
    tty = Types::simplifyType(ty.clone())?;
    ss_1 = addArrayDimensions(tty.clone(), ss.clone())?;
    splicedExpData = InstTypes::SplicedExpData { splicedExp: Some(Expression::makeCrefExp(ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), tty.clone(), ss_1.clone()), tty.clone())?), identType: ty.clone() };
    Ok((cache, attr, ty_1, bind, cnstForRange, splicedExpData, componentEnv, name))
}

fn lookupVarFMetaModelica(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut cr: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>) -> Result<(FCore::Cache, Arc<DAE::Attributes>, Arc<DAE::Type>, Arc<DAE::Binding>, Option<DAE::Const>, ArcStr)> {
    let mut cache: FCore::Cache = inCache.clone();
    let mut attr: Arc<DAE::Attributes>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut cnstForRange: Option<DAE::Const> = None;
    let mut name: ArcStr = arcstr::literal!("");
    (attr, ty, binding, cnstForRange, name) = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            let mut fields: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            fields = Types::getMetaRecordFields(inType.clone())?;
            let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((fields.clone()).get(Types::findVarIndex((var_field!((*cr).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), fields.clone()) + 1)?) {
                Deref @ DAE::Var { name: __pa0, attributes: __pa1, ty: __pa2, binding: __pa3, bind_from_outside: _, constOfForIteratorRange: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            attr = __pa1.clone();
            ty = __pa2.clone();
            binding = __pa3.clone();
            cnstForRange = __pa4.clone();
            for mut s in &*var_field!((*cr).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone() {
                let mut s = s.clone();
                ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAARRAY { .. } => var_field!((*ty).ty, DAE::Type::T_METAARRAY).clone(),
        _ => bail!("match: no arm matched"),
    } });
            }
            ty = Types::getMetaRecordIfSingleton(ty.clone());
            (attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), name.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            let mut fields: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            fields = Types::getMetaRecordFields(inType.clone())?;
            let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((fields.clone()).get(Types::findVarIndex((var_field!((*cr).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), fields.clone()) + 1)?) {
                Deref @ DAE::Var { name: __pa0, attributes: __pa1, ty: __pa2, binding: __pa3, bind_from_outside: _, constOfForIteratorRange: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            attr = __pa1.clone();
            ty = __pa2.clone();
            binding = __pa3.clone();
            cnstForRange = __pa4.clone();
            for mut s in &*var_field!((*cr).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone() {
                let mut s = s.clone();
                ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAARRAY { .. } => var_field!((*ty).ty, DAE::Type::T_METAARRAY).clone(),
        _ => bail!("match: no arm matched"),
    } });
            }
            ty = Types::getMetaRecordIfSingleton(ty.clone());
            (cache, attr, ty, binding, cnstForRange, name) = lookupVarFMetaModelica(cache.clone(), inEnv.clone(), var_field!((*cr).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), ty.clone())?;
            (attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), name.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cache, attr, ty, binding, cnstForRange, name))
}

fn lookupBinding(mut inCref: Arc<DAE::ComponentRef>, mut inParentType: Arc<DAE::Type>, mut inChildType: Arc<DAE::Type>, mut inParentBinding: Arc<DAE::Binding>, mut inChildBinding: Arc<DAE::Binding>) -> Result<Arc<DAE::Binding>> {
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    outBinding = 'mc: {
        let __mc_input = (inCref.clone(), inParentType.clone(), inChildType.clone(), inParentBinding.clone(), inChildBinding.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: ss, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: cId, identType: _, subscriptLst: Deref @ metamodelica::List::Nil } }, _, _, Deref @ DAE::Binding::EQBOUND { exp: e, evaluatedExp: _, constant_: c, source: s }, _) => {
                    let mut tyElement: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut b: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut comp: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut e = (*e).clone();
                    let true = (Types::isArray(inParentType.clone())) else { bail!("pattern mismatch") };
                    tyElement = Types::arrayElementType(inParentType.clone());
                    let true = (Types::isRecord(tyElement.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Expression::applyExpSubscripts(e.clone(), ss.clone())?) {
                        Deref @ DAE::Exp::RECORD { path: _, exps: __pa0, comp: __pa1, ty: _ } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    exps = __pa0.clone();
                    comp = __pa1.clone();
                    e = (exps.clone()).get(List::position((cId.clone()).clone(), comp.clone())?)?;
                    b = Arc::new(DAE::Binding::EQBOUND { exp: e.clone(), evaluatedExp: None, constant_: c.clone(), source: s.clone() });
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: ss, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: cId, identType: _, subscriptLst: Deref @ metamodelica::List::Nil } }, _, _, Deref @ DAE::Binding::VALBOUND { valBound: v, source: s }, _) => {
                    let mut tyElement: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut b: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut e: Arc<DAE::Exp>;
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut comp: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Types::isArray(inParentType.clone())) else { bail!("pattern mismatch") };
                    tyElement = Types::arrayElementType(inParentType.clone());
                    let true = (Types::isRecord(tyElement.clone())) else { bail!("pattern mismatch") };
                    e = ValuesUtil::valueExp(v.clone(), None)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Expression::applyExpSubscripts(e.clone(), ss.clone())?) {
                        Deref @ DAE::Exp::RECORD { path: _, exps: __pa0, comp: __pa1, ty: _ } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    exps = __pa0.clone();
                    comp = __pa1.clone();
                    e = (exps.clone()).get(List::position((cId.clone()).clone(), comp.clone())?)?;
                    b = Arc::new(DAE::Binding::EQBOUND { exp: e.clone(), evaluatedExp: None, constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: s.clone() });
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inChildBinding.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBinding)
}

fn elabComponentRecursive(mut oCref: Option<Arc<DAE::Exp>>) -> Arc<metamodelica::List<Arc<DAE::ComponentRef>>> {
    let mut lref: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    lref = (::match_deref::match_deref! { match &(oCref.clone()) {
        Some(Deref @ DAE::Exp::CREF { componentRef: ecpr @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, ty: _ }) => {
            cons(ecpr.clone(), metamodelica::nil())
        },
        Some(Deref @ DAE::Exp::CREF { componentRef: ecpr @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: _, componentRef: _ }, ty: _ }) => {
            cons(ecpr.clone(), metamodelica::nil())
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lref
}

fn addArrayDimensions(mut tySub: Arc<DAE::Type>, mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outType: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    outType = 'mc: {
        let __mc_input = (tySub.clone(), ss.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let true = (Types::isArray(tySub.clone())) else { bail!("pattern mismatch") };
                    dims = TypesDump::getDimensions(tySub.clone());
                    subs = List::map(dims.clone(), Arc::new(makeDimensionSubscript));
                    subs = expandWholeDimSubScript(ss.clone(), subs.clone())?;
                    Ok(subs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ss.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn makeDimensionSubscript(mut inDim: Arc<DAE::Dimension>) -> Result<Arc<DAE::Subscript>> {
    let mut outSub: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    outSub = (::match_deref::match_deref! { match &(inDim.clone()) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 } => {
            Arc::new(DAE::Subscript::SLICE { exp: Arc::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT.clone(), scalar: true, array: list![Arc::new(DAE::Exp::ICONST { integer: 0 })] }) })
        },
        Deref @ DAE::Dimension::DIM_INTEGER { .. } => {
            Arc::new(DAE::Subscript::SLICE { exp: Arc::new(DAE::Exp::RANGE { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: var_field!((*inDim).integer, DAE::Dimension::DIM_INTEGER).clone() })] }), start: Arc::new(DAE::Exp::ICONST { integer: 1 }), step: None, stop: Arc::new(DAE::Exp::ICONST { integer: var_field!((*inDim).integer, DAE::Dimension::DIM_INTEGER).clone() }) }) })
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expl = list![Arc::new(DAE::Exp::BCONST { bool: false }), Arc::new(DAE::Exp::BCONST { bool: true })];
            Arc::new(DAE::Subscript::SLICE { exp: Arc::new(DAE::Exp::ARRAY { ty: DAE::T_BOOL_DEFAULT.clone(), scalar: true, array: expl.clone() }) })
        },
        Deref @ DAE::Dimension::DIM_ENUM { literals: l, enumTypeName: enum_name, .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expl = makeEnumLiteralIndices(enum_name.clone(), l.clone(), 1)?;
            Arc::new(DAE::Subscript::SLICE { exp: Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ENUMERATION { index: None, path: enum_name.clone(), names: l.clone(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), scalar: true, array: expl.clone() }) })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSub)
}

fn makeEnumLiteralIndices(mut enumTypeName: Arc<Absyn::Path>, mut enumLiterals: Arc<metamodelica::List<ArcStr>>, mut enumIndex: i32) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut enumIndices: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    enumIndices = (::match_deref::match_deref! { match &((enumTypeName.clone(), enumLiterals.clone(), enumIndex.clone())) {
        (_, Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (_, Deref @ metamodelica::List::Cons { head: l, tail: ls }, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut enum_type_name: Arc<Absyn::Path>;
            enum_type_name = AbsynUtil::joinPaths(enumTypeName.clone(), Arc::new(Absyn::Path::IDENT { name: (l.clone()).clone() }))?;
            e = Arc::new(DAE::Exp::ENUM_LITERAL { name: enum_type_name.clone(), index: enumIndex.clone() });
            expl = makeEnumLiteralIndices(enumTypeName.clone(), ls.clone(), enumIndex.clone() + 1)?;
            cons(e.clone(), expl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(enumIndices)
}

fn expandWholeDimSubScript(mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inSlice: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    outSubs = 'mc: {
        let __mc_input = (inSubs.clone(), inSlice.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: sub1 @ Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { .. } }, tail: subs1 }, subs2) => {
                    let mut subs2 = (*subs2).clone();
                    subs2 = expandWholeDimSubScript(subs1.clone(), subs2.clone())?;
                    Ok(cons(sub1.clone(), subs2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
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
                (Deref @ metamodelica::List::Nil, subs2) => {
                    Ok(subs2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: subs1 }, Deref @ metamodelica::List::Cons { head: sub2, tail: subs2 }) => {
                    let mut subs2 = (*subs2).clone();
                    subs2 = expandWholeDimSubScript(subs1.clone(), subs2.clone())?;
                    Ok(cons(sub2.clone(), subs2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: sub1, tail: subs1 }, Deref @ metamodelica::List::Cons { head: _, tail: subs2 }) => {
                    let mut subs2 = (*subs2).clone();
                    subs2 = expandWholeDimSubScript(subs1.clone(), subs2.clone())?;
                    Ok(cons(sub1.clone(), subs2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSubs)
}

fn sliceDimensionType(mut inTypeD: Arc<DAE::Type>, mut inTypeL: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inTypeD.clone(), inTypeL.clone())) {
        (t, tOrg) => {
            let mut dimensions: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut t = (*t).clone();
            dimensions = Types::getDimensionSizes(t.clone())?;
            dim2 = List::map(dimensions.clone(), Arc::new(fnptr!(Expression::intDimension, i32)));
            dim2 = dim2.clone().reverse();
            t = List::foldr(dim2.clone(), Arc::new(fnptr!(Types::liftArray, Arc<DAE::Type>, Arc<DAE::Dimension>)), tOrg.clone());
            t.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub fn buildMetaRecordType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut cdef: Arc<SCode::Element>) -> Result<(FCore::Cache, FCore::Graph, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut ftype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut id: ArcStr = arcstr::literal!("");
    let mut env_1: FCore::Graph;
    let mut env: FCore::Graph;
    let mut utPath: Arc<Absyn::Path>;
    let mut path: Arc<Absyn::Path>;
    let mut index: i32 = 0;
    let mut varlst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut singleton: bool = false;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut typeVarsType: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut typeVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa0, .. }, restriction: SCode::Restriction::R_METARECORD { typeVars: __pa1, singleton: __pa2, index: __pa3, name: __pa4, .. }, name: __pa5, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    els = __pa0.clone();
    typeVars = __pa1.clone();
    singleton = __pa2.clone();
    index = __pa3.clone();
    utPath = __pa4.clone();
    id = __pa5.clone();
    env = FGraph::openScope(inEnv.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (id.clone()).clone(), Some(crate::FCore::ScopeType::CLASS_SCOPE))?;
    (cache, utPath) = Inst::makeFullyQualified(inCache.clone(), env.clone(), utPath.clone())?;
    path = AbsynUtil::joinPaths(utPath.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
    (outCache, outEnv, _, _, _, _, _, varlst, _, _) = Inst::instElementList(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, ClassInf::State::META_RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, List::map1(els.clone(), Arc::new(fnptr!(Util::makeTuple, _, _)), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)), metamodelica::nil(), false, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone(), true)?;
    varlst = Types::boxVarLst(varlst.clone())?;
    typeVarsType = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut tv in (typeVars.clone()).into_iter().cloned() {
            let __x = Arc::new(DAE::Type::T_METAPOLYMORPHIC { name: (tv.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    ftype = Arc::new(DAE::Type::T_METARECORD { path: path.clone(), utPath: utPath.clone(), typeVars: typeVarsType.clone(), index: index.clone(), fields: varlst.clone(), knownSingleton: singleton.clone() });
    Ok((outCache, outEnv, ftype))
}

pub fn isIterator(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCref: Arc<DAE::ComponentRef>) -> Result<(Option<bool>, FCore::Cache)> {
    let mut outIsIterator: Option<bool> = None;
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    (outIsIterator, outCache) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inCref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }, .. }, _) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut ht: Arc<FCore::RefTree::Tree> = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut ic: Option<DAE::Const> = None;
                    let mut b: bool = false;
                    ht = FNode::children(FNode::fromRef(r#ref.clone())?)?;
                    id = (ComponentReferenceBasics::crefFirstIdent(inCref.clone())?).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(lookupVar2(ht.clone(), (id.clone()).clone(), inEnv.clone())?) {
                        (Deref @ DAE::Var { constOfForIteratorRange: __pa0, .. }, _, _, _, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ic = __pa0.clone();
                    b = isSome(ic.clone());
                    Ok((Some(b.clone()), cache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r#ref, tail: _ }, .. }, _) => {
                    let mut env: FCore::Graph;
                    let mut res: Option<bool> = None;
                    let mut cache = (*cache).clone();
                    let true = (frameIsImplAddedScope(FNode::fromRef(r#ref.clone())?)?) else { bail!("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (res, cache) = isIterator(cache.clone(), env.clone(), inCref.clone())?;
                    Ok((res.clone(), cache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, inCache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outIsIterator, outCache))
}

pub fn isFunctionCallViaComponent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>) -> Result<bool> {
    let mut yes: bool = false;
    yes = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::Path::QUALIFIED { name, path: _ }) => {
                    ErrorExt::setCheckpoint((literal!("functionViaComponentRef10")).clone());
                    (_, _, _, _, _, _, _, _, _) = lookupVarIdent(inCache.clone(), inEnv.clone(), (name.clone()).clone(), metamodelica::nil())?;
                    ErrorExt::rollBack((literal!("functionViaComponentRef10")).clone());
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::Path::QUALIFIED { name: _, path: _ }) => {
                    ErrorExt::rollBack((literal!("functionViaComponentRef10")).clone());
                    Ok(bail!("fail"))
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
    Ok(yes)
}

fn prefixSplicedExp(mut inCref: Arc<DAE::ComponentRef>, mut inSplicedExp: InstTypes::SplicedExpData) -> Result<InstTypes::SplicedExpData> {
    let mut outSplicedExp: InstTypes::SplicedExpData;
    outSplicedExp = (::match_deref::match_deref! { match &(inSplicedExp.clone()) {
        InstTypes::SplicedExpData { splicedExp: Some(Deref @ DAE::Exp::CREF { componentRef: cref, ty: ety }), identType: ty } => {
            let mut cref = (*cref).clone();
            cref = ComponentReference::joinCrefs(inCref.clone(), cref.clone())?;
            InstTypes::SplicedExpData { splicedExp: Some(Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ety.clone() })), identType: ty.clone() }
        },
        _ => {
            inSplicedExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSplicedExp)
}

pub fn isArrayType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>) -> (FCore::Cache, bool) {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outIsArray: bool = false;
    let mut el: Arc<SCode::Element>;
    let mut p: Arc<Absyn::Path>;
    let mut env: FCore::Graph;
    match '__try0: {
        (outCache, el, env) = unwrap_break_err!(lookupClass(inCache.clone(), inEnv.clone(), inPath.clone(), None), '__try0);
        outIsArray = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(_), .. }, .. }, .. } => true,
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { attributes: SCode::Attributes { arrayDims: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, .. } => true,
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: p, .. }, .. }, .. } => {
            (outCache, outIsArray) = isArrayType(outCache.clone(), env.clone(), p.clone());
            outIsArray.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok::<_, anyhow::Error>((el.clone(), env.clone(), outCache.clone(), outIsArray.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            el = __try0_o0;
            env = __try0_o1;
            outCache = __try0_o2;
            outIsArray = __try0_o3;
        }
        Err(_) => {
            outIsArray = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (outCache, outIsArray)
}

