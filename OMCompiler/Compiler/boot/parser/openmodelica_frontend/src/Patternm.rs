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

use crate::Algorithm;
use crate::Ceval;
use crate::ComponentReference;
use crate::ConnectionGraph;
use crate::DAEUtil;
use crate::Expression;
use crate::FCore;
use crate::FGraph;
use crate::InnerOuter;
use crate::Inst;
use crate::InstSection;
use crate::InstUtil;
use crate::Lookup;
use crate::Static;
use crate::Types;
use crate::UnitAbsyn;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTableStringToPath;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::AvlSetString;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn generatePositionalArgs(mut fieldNameList: Arc<metamodelica::List<ArcStr>>, mut namedArgList: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut accList: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>)> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut outInvalidNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    (outList, outInvalidNames) = (::match_deref::match_deref! { match &((fieldNameList.clone(), namedArgList.clone(), accList.clone())) {
        (Deref @ metamodelica::List::Nil, _, localAccList) => {
            (localAccList.clone().reverse(), namedArgList.clone())
        },
        (Deref @ metamodelica::List::Cons { head: firstFieldName, tail: restFieldNames }, localNamedArgList, localAccList) => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut localNamedArgList = (*localNamedArgList).clone();
            let mut localAccList = (*localAccList).clone();
            (exp, localNamedArgList) = findFieldExpInList((firstFieldName.clone()).clone(), localNamedArgList.clone())?;
            (localAccList, localNamedArgList) = generatePositionalArgs(restFieldNames.clone(), localNamedArgList.clone(), cons(exp.clone(), localAccList.clone()))?;
            (localAccList.clone(), localNamedArgList.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outList, outInvalidNames))
}

fn findFieldExpInList(mut firstFieldName: ArcStr, mut namedArgList: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>)> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outNamedArgList: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    (outExp, outNamedArgList) = 'mc: {
        let __mc_input = (firstFieldName.clone(), namedArgList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok((Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(openmodelica_ast::Absyn::ComponentRef::WILD) }), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (localFieldName, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: aName, argValue: e }, tail: rest }) => {
                    let true = (stringEq((localFieldName.clone()).clone(), (aName.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (localFieldName, Deref @ metamodelica::List::Cons { head: first, tail: rest }) => {
                    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut rest = (*rest).clone();
                    (e, rest) = findFieldExpInList((localFieldName.clone()).clone(), rest.clone())?;
                    Ok((e.clone(), cons(first.clone(), rest.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outNamedArgList))
}

fn checkInvalidPatternNamedArgs(mut args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut fieldNameList: Arc<metamodelica::List<ArcStr>>, mut status: Util::Status, mut info: SourceInfo) -> Result<Util::Status> {
    let mut outStatus: Util::Status = Util::Status::FAILURE;
    outStatus = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Nil => {
            status.clone()
        },
        _ => {
            let mut argsNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            (argsNames, _) = AbsynUtil::getNamedFuncArgNamesAndValues(args.clone());
            str1 = stringDelimitList(argsNames.clone(), (literal!(",")).clone());
            str2 = stringDelimitList(fieldNameList.clone(), (literal!(",")).clone());
            Error::addSourceMessage(Error::META_INVALID_PATTERN_NAMED_FIELD.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
            openmodelica_util::Util::Status::FAILURE
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStatus)
}

pub fn elabPatternCheckDuplicateBindings(mut cache: FCore::Cache, mut env: FCore::Graph, mut lhs: Arc<Absyn::Exp>, mut ty: Arc<DAE::Type>, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Pattern>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut pattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    (outCache, pattern) = elabPattern2(cache.clone(), env.clone(), lhs.clone(), ty.clone(), info.clone(), Error::getNumErrorMessages())?;
    checkPatternsDuplicateAsBindings(cons(pattern.clone(), metamodelica::nil()), info.clone())?;
    Ok((outCache, pattern))
}

fn elabPattern(mut cache: FCore::Cache, mut env: FCore::Graph, mut lhs: Arc<Absyn::Exp>, mut ty: Arc<DAE::Type>, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Pattern>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut pattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    (outCache, pattern) = elabPattern2(cache.clone(), env.clone(), lhs.clone(), ty.clone(), info.clone(), Error::getNumErrorMessages())?;
    Ok((outCache, pattern))
}

fn checkPatternsDuplicateAsBindings(mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>>, mut info: SourceInfo) -> Result<()> {
    let mut usedVariables: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (_, usedVariables) = traversePatternList(patterns.clone(), (std::sync::Arc::new(fnptr!(findBoundVariables, Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)> + 'static>), metamodelica::nil())?;
    usedVariables = List::sortedUniqueOnlyDuplicates(List::sort(usedVariables.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
    if !(usedVariables.clone().is_empty()) {
        Error::addSourceMessage(Error::DUPLICATE_DEFINITION.clone(), list![stringDelimitList(usedVariables.clone(), (literal!(", ")).clone())], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

fn findBoundVariables(mut pat: Arc<DAE::Pattern>, mut boundVars: Arc<metamodelica::List<ArcStr>>) -> (Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>) {
    let mut outPat: Arc<DAE::Pattern> = pat.clone();
    let mut outBoundVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outBoundVars = (::match_deref::match_deref! { match &(pat.clone()) {
        Deref @ DAE::Pattern::PAT_AS { .. } => cons((var_field!((*pat).id, DAE::Pattern::PAT_AS).clone()).clone(), boundVars.clone()),
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. } => cons((var_field!((*pat).id, DAE::Pattern::PAT_AS_FUNC_PTR).clone()).clone(), boundVars.clone()),
        _ => boundVars.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outPat, outBoundVars)
}

fn elabPattern2(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inLhs: Arc<Absyn::Exp>, mut ty: Arc<DAE::Type>, mut info: SourceInfo, mut numError: i32) -> Result<(FCore::Cache, Arc<DAE::Pattern>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut pattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    (outCache, pattern) = 'mc: {
        let __mc_input = (inCache.clone(), inLhs.clone(), ty.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::INTEGER { value: i }, _) => {
                    let mut et: Option<Arc<DAE::Type>> = None;
                    et = validPatternType(ty.clone(), DAE::T_INTEGER_DEFAULT().clone(), inLhs.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::REAL { value: r#str }, _) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut et: Option<Arc<DAE::Type>> = None;
                    et = validPatternType(ty.clone(), DAE::T_REAL_DEFAULT().clone(), inLhs.clone(), info.clone())?;
                    r = stringReal((r#str.clone()).clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: Arc::new(DAE::Exp::RCONST { real: r.clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS, exp: Deref @ Absyn::Exp::INTEGER { value: i } }, _) => {
                    let mut et: Option<Arc<DAE::Type>> = None;
                    let mut i = (*i).clone();
                    et = validPatternType(ty.clone(), DAE::T_INTEGER_DEFAULT().clone(), inLhs.clone(), info.clone())?;
                    i = -(i.clone());
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS, exp: Deref @ Absyn::Exp::REAL { value: r#str } }, _) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut et: Option<Arc<DAE::Type>> = None;
                    et = validPatternType(ty.clone(), DAE::T_REAL_DEFAULT().clone(), inLhs.clone(), info.clone())?;
                    r = stringReal((r#str.clone()).clone())?;
                    r = -(r.clone());
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: Arc::new(DAE::Exp::RCONST { real: r.clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::STRING { value: s }, _) => {
                    let mut et: Option<Arc<DAE::Type>> = None;
                    let mut s = (*s).clone();
                    et = validPatternType(ty.clone(), DAE::T_STRING_DEFAULT().clone(), inLhs.clone(), info.clone())?;
                    s = (System::unescapedString((s.clone()).clone())).clone();
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::BOOL { value: b }, _) => {
                    let mut et: Option<Arc<DAE::Type>> = None;
                    et = validPatternType(ty.clone(), DAE::T_BOOL_DEFAULT().clone(), inLhs.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: Arc::new(DAE::Exp::BCONST { bool: b.clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Nil }, _) => {
                    let mut et: Option<Arc<DAE::Type>> = None;
                    et = validPatternType(ty.clone(), DAE::T_METALIST_DEFAULT().clone(), inLhs.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: Arc::new(DAE::Exp::LIST { valList: metamodelica::nil() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::ARRAY { arrayExp: exps @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, _) => {
                    let mut lhs: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    lhs = List::fold(exps.clone().reverse(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeCons, Arc<Absyn::Exp>, Arc<Absyn::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>), Arc::new(Absyn::Exp::ARRAY { arrayExp: metamodelica::nil() }));
                    (cache, pattern) = elabPattern(cache.clone(), env.clone(), lhs.clone(), ty.clone(), info.clone())?;
                    Ok((cache.clone(), pattern.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "NONE", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Nil, argNames: Deref @ metamodelica::List::Nil }, .. }, _) => {
                    validPatternType(ty.clone(), DAE::T_NONE_DEFAULT().clone(), inLhs.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: None, exp: Arc::new(DAE::Exp::META_OPTION { exp: None }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "SOME", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::Type::T_METAOPTION { ty: ty2 }) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPattern(cache.clone(), env.clone(), exp.clone(), ty2.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_SOME { pat: pattern.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CONS { head, rest: tail }, tyTail @ Deref @ DAE::Type::T_METALIST { ty: tyHead }) => {
                    let mut patternHead: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
                    let mut patternTail: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
                    let mut cache = (*cache).clone();
                    let mut tyHead = (*tyHead).clone();
                    tyHead = Types::boxIfUnboxedType(tyHead.clone())?;
                    (cache, patternHead) = elabPattern(cache.clone(), env.clone(), head.clone(), tyHead.clone(), info.clone())?;
                    (cache, patternTail) = elabPattern(cache.clone(), env.clone(), tail.clone(), tyTail.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONS { head: patternHead.clone(), tail: patternTail.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPattern2(cache.clone(), env.clone(), exp.clone(), ty.clone(), info.clone(), numError.clone())?;
                    Ok((cache.clone(), pattern.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: exps }, Deref @ DAE::Type::T_METATUPLE { types: tys }) => {
                    let mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut tys = (*tys).clone();
                    tys = List::map(tys.clone(), (std::sync::Arc::new(Types::boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    (cache, patterns) = elabPatternTuple(cache.clone(), env.clone(), exps.clone(), tys.clone(), info.clone(), inLhs.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: exps }, Deref @ DAE::Type::T_TUPLE { types: tys, .. }) => {
                    let mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, patterns) = elabPatternTuple(cache.clone(), env.clone(), exps.clone(), tys.clone(), info.clone(), inLhs.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CALL_TUPLE { patterns: patterns.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, lhs @ Deref @ Absyn::Exp::CALL { function_: fcr, functionArgs: fargs, .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: utPath }, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPatternCall(cache.clone(), env.clone(), AbsynUtil::crefToPath(fcr.clone())?, fargs.clone(), utPath.clone(), info.clone(), lhs.clone())?;
                    Ok((cache.clone(), pattern.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, lhs @ Deref @ Absyn::Exp::CALL { function_: fcr, functionArgs: fargs, .. }, Deref @ DAE::Type::T_METAUNIONTYPE { path: utPath, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPatternCall(cache.clone(), env.clone(), AbsynUtil::crefToPath(fcr.clone())?, fargs.clone(), utPath.clone(), info.clone(), lhs.clone())?;
                    Ok((cache.clone(), pattern.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, lhs @ Deref @ Absyn::Exp::CALL { function_: fcr, functionArgs: fargs, .. }, Deref @ DAE::Type::T_METARECORD { utPath, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPatternCall(cache.clone(), env.clone(), AbsynUtil::crefToPath(fcr.clone())?, fargs.clone(), utPath.clone(), info.clone(), lhs.clone())?;
                    Ok((cache.clone(), pattern.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { .. }, ty1) => {
                    if !((Types::isBoxedType(ty1.clone()) || (::match_deref::match_deref! { match &(Types::unboxedType(ty1.clone())?) {
        Deref @ DAE::Type::T_ENUMERATION { .. } => true,
        Deref @ DAE::Type::T_INTEGER { .. } => true,
        Deref @ DAE::Type::T_REAL { .. } => true,
        Deref @ DAE::Type::T_STRING { .. } => true,
        Deref @ DAE::Type::T_BOOL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }))) { bail!("guard") }
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut et: Option<Arc<DAE::Type>> = None;
                    let mut elabExp: Arc<DAE::Exp>;
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabExp(cache.clone(), env.clone(), inLhs.clone(), false, false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { constFlag: __pa2, type_: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    elabExp = __pa1.clone();
                    r#const = __pa2.clone();
                    ty2 = __pa3.clone();
                    et = validPatternType(ty1.clone(), ty2.clone(), inLhs.clone(), info.clone())?;
                    let true = (Types::isConstant(r#const.clone())) else { bail!("pattern mismatch") };
                    (cache, val) = Ceval::ceval(cache.clone(), env.clone(), elabExp.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    elabExp = ValuesUtil::valueExp(val.clone(), None)?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: elabExp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::AS { id, exp }, ty2) => {
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut et: Option<Arc<DAE::Type>> = None;
                    let mut lhs: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?) {
                        (__pa0, Deref @ DAE::Var { attributes: __pa1, ty: __pa2, .. }, _, _, _, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    attr = __pa1.clone();
                    ty1 = __pa2.clone();
                    lhs = Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() }) });
                    Static::checkAssignmentToInput(lhs.clone(), attr.clone(), env.clone(), false, info.clone())?;
                    et = validPatternType(ty2.clone(), ty1.clone(), inLhs.clone(), info.clone())?;
                    (cache, pattern) = elabPattern(cache.clone(), env.clone(), exp.clone(), ty2.clone(), info.clone())?;
                    pattern = if (Types::isFunctionType(ty2.clone())) {Arc::new(DAE::Pattern::PAT_AS_FUNC_PTR { id: (id.clone()).clone(), pat: pattern.clone() })} else {Arc::new(DAE::Pattern::PAT_AS { id: (id.clone()).clone(), ty: et.clone(), attr: attr.clone(), pat: pattern.clone() })};
                    Ok((cache.clone(), pattern.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::List::Nil } }, ty2) => {
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut et: Option<Arc<DAE::Type>> = None;
                    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
                    let mut variability: SCode::Variability = SCode::Variability::CONST;
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?) {
                        (__pa0, Deref @ DAE::Var { attributes: __pa2 @ Deref @ DAE::Attributes { variability: __pa1, .. }, ty: __pa3, .. }, _, _, _, _) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    variability = __pa1.clone();
                    attr = __pa2.clone();
                    ty1 = __pa3.clone();
                    if SCodeUtil::isParameterOrConst(variability.clone()) {
                        Error::addSourceMessage(Error::PATTERN_VAR_NOT_VARIABLE.clone(), list![(id.clone()).clone(), (SCodeDump::unparseVariability(variability.clone())?).clone()], info.clone())?;
                        bail!("fail");
                    }
                    Static::checkAssignmentToInput(inLhs.clone(), attr.clone(), env.clone(), false, info.clone())?;
                    et = validPatternType(ty2.clone(), ty1.clone(), inLhs.clone(), info.clone())?;
                    pattern = if (Types::isFunctionType(ty2.clone())) {Arc::new(DAE::Pattern::PAT_AS_FUNC_PTR { id: (id.clone()).clone(), pat: Arc::new(openmodelica_frontend_types::DAE::Pattern::PAT_WILD) })} else {Arc::new(DAE::Pattern::PAT_AS { id: (id.clone()).clone(), ty: et.clone(), attr: attr.clone(), pat: Arc::new(openmodelica_frontend_types::DAE::Pattern::PAT_WILD) })};
                    Ok((cache.clone(), pattern.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::AS { id, exp: _ }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Error::addSourceMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(id.clone()).clone(), (literal!("")).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "NONE", subscripts: Deref @ metamodelica::List::Nil } }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupIdent(cache.clone(), env.clone(), (literal!("NONE")).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Error::addSourceMessage(Error::META_NONE_CREF.clone(), metamodelica::nil(), info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::List::Nil } }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let false = (literal!("NONE") == id.clone()) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(id.clone()).clone(), (literal!("")).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::WILD }, _) => {
                    Ok((cache.clone(), Arc::new(openmodelica_frontend_types::DAE::Pattern::PAT_WILD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::EXPRESSIONCOMMENT { .. }, _) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: Arc<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPattern2(cache.clone(), env.clone(), var_field!((*inLhs).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(), ty.clone(), info.clone(), numError.clone())?;
                    Ok((cache.clone(), pattern.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, lhs, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (numError.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!(" of type ")); __mm_s.push_str(&*TypesDump::unparseType(ty.clone())?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::META_INVALID_PATTERN.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, pattern))
}

fn elabPatternTuple(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inTys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut info: SourceInfo, mut lhs: Arc<Absyn::Exp>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Pattern>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
    (outCache, patterns) = (::match_deref::match_deref! { match &((inCache.clone(), inExps.clone(), inTys.clone())) {
        (cache, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, Deref @ metamodelica::List::Cons { head: exp, tail: exps }, Deref @ metamodelica::List::Cons { head: ty, tail: tys }) => {
            let mut pattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut cache = (*cache).clone();
            (cache, pattern) = elabPattern(cache.clone(), env.clone(), exp.clone(), ty.clone(), info.clone())?;
            (cache, patterns) = elabPatternTuple(cache.clone(), env.clone(), exps.clone(), tys.clone(), info.clone(), lhs.clone())?;
            (cache.clone(), cons(pattern.clone(), patterns.clone()))
        },
        _ => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (Dump::printExpStr(lhs.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pattern ")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::WRONG_NO_OF_ARGS.clone(), list![(s.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, patterns))
}

fn elabPatternCall(mut inCache: FCore::Cache, mut env: FCore::Graph, mut callPath: Arc<Absyn::Path>, mut fargs: Arc<Absyn::FunctionArgs>, mut utPath: Arc<Absyn::Path>, mut info: SourceInfo, mut lhs: Arc<Absyn::Exp>) -> Result<(FCore::Cache, Arc<DAE::Pattern>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut pattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    (outCache, pattern) = 'mc: {
        let __mc_input = (inCache.clone(), fargs.clone(), utPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: _, tail: _ }, argNames: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, _) => {
                    Error::addSourceMessage(Error::PATTERN_MIXED_POS_NAMED.clone(), list![(AbsynUtil::pathString(callPath.clone(), (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: funcArgs, argNames: namedArgList }, utPath2) => {
                    let mut utPath1: Arc<Absyn::Path>;
                    let mut fqPath: Arc<Absyn::Path>;
                    let mut index: i32 = 0;
                    let mut numPosArgs: i32 = 0;
                    let mut invalidArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut funcArgsNamedFixed: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut funcArgs2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut fieldNameList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut fieldNamesNamed: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut fieldTypeList: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut typeVars: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut fieldVarList: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
                    let mut knownSingleton: bool = false;
                    let mut allWild: bool = false;
                    let mut cache = (*cache).clone();
                    let mut funcArgs = (*funcArgs).clone();
                    let mut namedArgList = (*namedArgList).clone();
                    (cache, _, _) = Lookup::lookupType(cache.clone(), env.clone(), callPath.clone(), None)?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Lookup::lookupType(cache.clone(), env.clone(), callPath.clone(), None)?) {
                        (__pa0, Deref @ DAE::Type::T_METARECORD { path: __pa1, knownSingleton: __pa2, typeVars: __pa3, fields: __pa4, index: __pa5, utPath: __pa6 }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    fqPath = __pa1.clone();
                    knownSingleton = __pa2.clone();
                    typeVars = __pa3.clone();
                    fieldVarList = __pa4.clone();
                    index = __pa5.clone();
                    utPath1 = __pa6.clone();
                    validUniontype(utPath1.clone(), utPath2.clone(), info.clone(), lhs.clone())?;
                    fieldTypeList = List::map(fieldVarList.clone(), (std::sync::Arc::new(Types::getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>));
                    fieldNameList = List::map(fieldVarList.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    if Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? {
                        for mut namedArg in &*namedArgList.clone() {
                            let mut namedArg = namedArg.clone();
                            let () = (::match_deref::match_deref! { match &(namedArg.clone()) {
        Deref @ Absyn::NamedArg { argValue: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::WILD }, .. } => {
                    Error::addSourceMessage(Error::META_EMPTY_CALL_PATTERN.clone(), list![(namedArg.argName.clone()).clone()], info.clone())?;
                    ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                        }
                        if namedArgList.clone().is_empty() && !(funcArgs.clone().is_empty()) {
                            allWild = true;
                            for mut arg in &*funcArgs.clone() {
                                        let mut arg = arg.clone();
                                        allWild = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::WILD } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                                        if !(allWild.clone()) {
                                            break;
                                        }
                            }
                            if allWild.clone() {
                                        Error::addSourceMessage(Error::META_ALL_EMPTY.clone(), list![(AbsynUtil::pathString(callPath.clone(), (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
                            }
                        }
                    }
                    (funcArgs, namedArgList) = checkForAllWildCall(funcArgs.clone(), namedArgList.clone(), (fieldNameList.clone().len() as i32));
                    numPosArgs = (funcArgs.clone().len() as i32);
                    (_, fieldNamesNamed) = List::split(fieldNameList.clone(), numPosArgs.clone())?;
                    checkMissingArgs(fqPath.clone(), numPosArgs.clone(), fieldNamesNamed.clone(), (namedArgList.clone().len() as i32), info.clone());
                    (funcArgsNamedFixed, invalidArgs) = generatePositionalArgs(fieldNamesNamed.clone(), namedArgList.clone(), metamodelica::nil())?;
                    funcArgs2 = listAppend(funcArgs.clone(), funcArgsNamedFixed.clone());
                    let Util::SUCCESS { .. } = (checkInvalidPatternNamedArgs(invalidArgs.clone(), fieldNameList.clone(), openmodelica_util::Util::Status::SUCCESS, info.clone())?) else { bail!("pattern mismatch") };
                    (cache, patterns) = elabPatternTuple(cache.clone(), env.clone(), funcArgs2.clone(), fieldTypeList.clone(), info.clone(), lhs.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CALL { name: fqPath.clone(), index: index.clone(), patterns: patterns.clone(), fields: fieldVarList.clone(), typeVars: typeVars.clone(), knownSingleton: knownSingleton.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: funcArgs, argNames: namedArgList }, utPath2) => {
                    let mut fqPath: Arc<Absyn::Path>;
                    let mut numPosArgs: i32 = 0;
                    let mut invalidArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut funcArgsNamedFixed: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut funcArgs2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut fieldNameList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut fieldNamesNamed: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut fieldTypeList: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut fieldVarList: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
                    let mut namedPatterns: Arc<metamodelica::List<(Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>)>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut funcArgs = (*funcArgs).clone();
                    let mut namedArgList = (*namedArgList).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupType(cache.clone(), env.clone(), callPath.clone(), None)?) {
                        (__pa0, Deref @ DAE::Type::T_FUNCTION { path: __pa1, funcResultType: Deref @ DAE::Type::T_COMPLEX { varLst: __pa2, complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    fqPath = __pa1.clone();
                    fieldVarList = __pa2.clone();
                    let true = (AbsynUtil::pathEqual(fqPath.clone(), utPath2.clone())) else { bail!("pattern mismatch") };
                    fieldTypeList = List::map(fieldVarList.clone(), (std::sync::Arc::new(Types::getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>));
                    fieldNameList = List::map(fieldVarList.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    (funcArgs, namedArgList) = checkForAllWildCall(funcArgs.clone(), namedArgList.clone(), (fieldNameList.clone().len() as i32));
                    numPosArgs = (funcArgs.clone().len() as i32);
                    (_, fieldNamesNamed) = List::split(fieldNameList.clone(), numPosArgs.clone())?;
                    checkMissingArgs(fqPath.clone(), numPosArgs.clone(), fieldNamesNamed.clone(), (namedArgList.clone().len() as i32), info.clone());
                    (funcArgsNamedFixed, invalidArgs) = generatePositionalArgs(fieldNamesNamed.clone(), namedArgList.clone(), metamodelica::nil())?;
                    funcArgs2 = listAppend(funcArgs.clone(), funcArgsNamedFixed.clone());
                    let Util::SUCCESS { .. } = (checkInvalidPatternNamedArgs(invalidArgs.clone(), fieldNameList.clone(), openmodelica_util::Util::Status::SUCCESS, info.clone())?) else { bail!("pattern mismatch") };
                    (cache, patterns) = elabPatternTuple(cache.clone(), env.clone(), funcArgs2.clone(), fieldTypeList.clone(), info.clone(), lhs.clone())?;
                    namedPatterns = List::zip3(patterns.clone(), fieldNameList.clone(), List::map(fieldTypeList.clone(), (std::sync::Arc::new(Types::simplifyType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>)));
                    namedPatterns = List::filterOnTrue(namedPatterns.clone(), (std::sync::Arc::new(fnptr!(filterEmptyPattern, (Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>)) -> Result<bool> + 'static>));
                    Ok((cache.clone(), Arc::new(DAE::Pattern::PAT_CALL_NAMED { name: fqPath.clone(), patterns: namedPatterns.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupType(cache.clone(), env.clone(), callPath.clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s = (AbsynUtil::pathString(callPath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::META_CONSTRUCTOR_NOT_RECORD.clone(), list![(s.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, pattern))
}

fn checkMissingArgs(mut path: Arc<Absyn::Path>, mut numPosArgs: i32, mut missingFieldNames: Arc<metamodelica::List<ArcStr>>, mut numNamedArgs: i32, mut info: SourceInfo) -> () {
    let () = (::match_deref::match_deref! { match &((missingFieldNames.clone(), numNamedArgs.clone())) {
        (Deref @ metamodelica::List::Nil, 0) => (),
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn checkForAllWildCall(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut named: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut numFields: i32) -> (Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut outNamed: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    (outArgs, outNamed) = (::match_deref::match_deref! { match &((args.clone(), named.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::ALLWILD }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil) => (metamodelica::nil(), metamodelica::nil()),
        _ => (args.clone(), named.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outArgs, outNamed)
}

fn validPatternType(mut inTy1: Arc<DAE::Type>, mut inTy2: Arc<DAE::Type>, mut lhs: Arc<Absyn::Exp>, mut info: SourceInfo) -> Result<Option<Arc<DAE::Type>>> {
    let mut ty: Option<Arc<DAE::Type>> = None;
    ty = 'mc: {
        let __mc_input = (inTy1.clone(), inTy2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METABOXED { ty: ty1 }, ty2) => {
                    let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut ty1 = (*ty1).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent((literal!("#DUMMY#")).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    crefExp = Expression::crefExp(cr.clone())?;
                    (_, ty1) = Types::matchType(crefExp.clone(), ty1.clone(), ty2.clone(), true)?;
                    et = Types::simplifyType(ty1.clone())?;
                    Ok(Some(et.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, ty2) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefExp: Arc<DAE::Exp>;
                    cr = ComponentReferenceBasics::makeCrefIdent((literal!("#DUMMY#")).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    crefExp = Expression::crefExp(cr.clone())?;
                    Types::matchType(crefExp.clone(), ty1.clone(), ty2.clone(), true)?;
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, ty2) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s = (Dump::printExpStr(lhs.clone())?).clone();
                    s1 = (TypesDump::unparseType(ty1.clone())?).clone();
                    s2 = (TypesDump::unparseType(ty2.clone())?).clone();
                    Error::addSourceMessage(Error::META_TYPE_MISMATCH_PATTERN.clone(), list![(s.clone()).clone(), (s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ty)
}

fn validUniontype(mut path1: Arc<Absyn::Path>, mut path2: Arc<Absyn::Path>, mut info: SourceInfo, mut lhs: Arc<Absyn::Exp>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = lhs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s = (Dump::printExpStr(lhs.clone())?).clone();
                    s1 = (AbsynUtil::pathString(path1.clone(), (literal!(".")).clone(), true, false)?).clone();
                    s2 = (AbsynUtil::pathString(path2.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::META_CONSTRUCTOR_NOT_PART_OF_UNIONTYPE.clone(), list![(s.clone()).clone(), (s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn patternStr(mut pattern: Arc<DAE::Pattern>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = pattern.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_WILD => {
                    Ok(literal!("_"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS { pat: Deref @ DAE::Pattern::PAT_WILD, id, .. } => {
                    Ok(id.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, pat: Deref @ DAE::Pattern::PAT_WILD } => {
                    Ok(id.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_SOME { pat } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = (patternStr(pat.clone())?).clone();
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SOME(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = stringDelimitList(List::map(pats.clone(), (std::sync::Arc::new(patternStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = stringDelimitList(List::map(pats.clone(), (std::sync::Arc::new(patternStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_CALL { patterns: pats, name, .. } => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = r#str.clone();
                    id = (AbsynUtil::pathString(name.clone(), (literal!(".")).clone(), true, false)?).clone();
                    r#str = stringDelimitList(List::map(pats.clone(), (std::sync::Arc::new(patternStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
                    Ok(stringAppendList(list![(id.clone()).clone(), (literal!("(")).clone(), (r#str.clone()).clone(), (literal!(")")).clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_CALL_NAMED { patterns: namedpats, name } => {
                    let mut fields: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut patsStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = r#str.clone();
                    id = (AbsynUtil::pathString(name.clone(), (literal!(".")).clone(), true, false)?).clone();
                    fields = List::map(namedpats.clone(), std::sync::Arc::new(fnptr!(Util::tuple32, _)));
                    patsStr = List::map1r(List::mapMap(namedpats.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)), (std::sync::Arc::new(patternStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<ArcStr> + 'static>)), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("=")).clone());
                    r#str = stringDelimitList(List::threadMap(fields.clone(), patsStr.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
                    Ok(stringAppendList(list![(id.clone()).clone(), (literal!("(")).clone(), (r#str.clone()).clone(), (literal!(")")).clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_CONS { head, tail } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*patternStr(head.clone())?); __mm_s.push_str(&*literal!("::")); __mm_s.push_str(&*patternStr(tail.clone())?); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_CONSTANT { exp, .. } => {
                    Ok(ExpressionBasics::printExpStr(exp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS { pat, id, .. } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" as ")); __mm_s.push_str(&*patternStr(pat.clone())?); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, pat } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" as ")); __mm_s.push_str(&*patternStr(pat.clone())?); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Patternm.patternStr not implemented correctly")).clone()])?;
                    Ok(literal!("*PATTERN*"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

pub fn elabMatchExpression(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut matchExp: Arc<Absyn::Exp>, mut r#impl: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut numError: i32 = Error::getNumErrorMessages();
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), matchExp.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::MATCHEXP { cases, localDecls: decls, inputExp: inExp, matchTy, .. }, pre) => {
                    let mut inExps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut matchDecls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut elabExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut elabCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut prop: DAE::Properties;
                    let mut elabProps: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut et: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
                    let mut elabMatchTy: DAE::MatchType = DAE::MatchType::MATCHCONTINUE;
                    let mut hashSize: i32 = 0;
                    let mut inputAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut inputAliasesAndCrefs: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut declsTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut matchTy = (*matchTy).clone();
                    inExps = convertExpToPatterns(inExp.clone());
                    (inExps, inputAliases, inputAliasesAndCrefs) = List::map_3(inExps.clone(), (std::sync::Arc::new(fnptr!(getInputAsBinding, Arc<Absyn::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> + 'static>));
                    (cache, elabExps, elabProps) = Static::elabExpList(cache.clone(), env.clone(), inExps.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLocalDecls(cache.clone(), env.clone(), decls.clone(), (arcstr::literal!(FCore::matchScopeName)).clone(), r#impl.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, DAE::DAElist { elementLst: __pa2 }, __pa3))) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    env = __pa1.clone();
                    matchDecls = __pa2.clone();
                    declsTree = __pa3.clone();
                    tys = List::map(elabProps.clone(), (std::sync::Arc::new(Types::getPropType) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Properties) -> Result<Arc<DAE::Type>> + 'static>));
                    env = addAliasesToEnv(env.clone(), tys.clone(), inputAliases.clone(), info.clone())?;
                    (cache, elabCases, resType) = elabMatchCases(cache.clone(), env.clone(), cases.clone(), tys.clone(), inputAliasesAndCrefs.clone(), declsTree.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), info.clone())?;
                    prop = DAE::Properties::PROP { type_: resType.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR };
                    et = Types::simplifyType(resType.clone())?;
                    checkMatchSingleInfallibleCase(matchTy.clone(), elabCases.clone(), info.clone())?;
                    checkInfallibleNoBindingPatterns(elabCases.clone(), matchTy.clone(), info.clone())?;
                    (elabExps, inputAliases, elabCases) = filterUnusedPatterns(elabExps.clone(), inputAliases.clone(), elabCases.clone(), info.clone(), !(isSingleInfallibleMatch(matchTy.clone(), elabCases.clone())))?;
                    elabCases = caseDeadCodeElimination(matchTy.clone(), elabCases.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
                    matchTy = optimizeContinueToMatch(matchTy.clone(), elabCases.clone(), info.clone())?;
                    elabCases = optimizeContinueJumps(matchTy.clone(), elabCases.clone())?;
                    hashSize = Util::nextPrime((matchDecls.clone().len() as i32));
                    ht = getUsedLocalCrefs(Flags::isSet(Flags::PATTERNM_SKIP_FILTER_UNUSED_AS_BINDINGS.clone())?, Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: openmodelica_frontend_types::DAE::MatchType::MATCHCONTINUE, inputs: elabExps.clone(), aliases: inputAliases.clone(), localDecls: matchDecls.clone(), cases: elabCases.clone(), et: et.clone() }), hashSize.clone())?;
                    (matchDecls, ht) = filterUnusedDecls(matchDecls.clone(), ht.clone(), metamodelica::nil(), HashTableStringToPath::emptyHashTableSized(hashSize.clone()))?;
                    (elabExps, inputAliases, elabCases) = filterUnusedPatterns(elabExps.clone(), inputAliases.clone(), elabCases.clone(), info.clone(), false)?;
                    (elabMatchTy, elabCases) = optimizeMatchToSwitch(matchTy.clone(), elabCases.clone(), info.clone())?;
                    elabMatchTy = unboxSwitchType(elabMatchTy.clone(), elabExps.clone())?;
                    checkConstantMatchInputs(elabExps.clone(), info.clone())?;
                    exp = Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: elabMatchTy.clone(), inputs: elabExps.clone(), aliases: inputAliases.clone(), localDecls: matchDecls.clone(), cases: elabCases.clone(), et: et.clone() });
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (numError.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
                    r#str = (Dump::printExpStr(matchExp.clone())?).clone();
                    Error::addSourceMessage(Error::META_MATCH_GENERAL_FAILURE.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn checkConstantMatchInputs(mut inputs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut info: SourceInfo) -> Result<()> {
    for mut i in &*inputs.clone() {
        let mut i = i.clone();
        if Expression::isConstValue(i.clone())? {
            Error::addSourceMessage(Error::META_MATCH_CONSTANT.clone(), list![(ExpressionBasics::printExpStr(i.clone())?).clone()], info.clone())?;
        }
    }
    Ok(())
}

fn optimizeMatchToSwitch(mut matchTy: Absyn::MatchType, mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut info: SourceInfo) -> Result<(DAE::MatchType, Arc<metamodelica::List<Arc<DAE::MatchCase>>>)> {
    let mut outType: DAE::MatchType = DAE::MatchType::MATCHCONTINUE;
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    (outType, outCases) = 'mc: {
        let __mc_input = matchTy.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let Absyn::MatchType::MATCHCONTINUE = __mc_input.clone() else { bail!("nomatch") };
            Ok((openmodelica_frontend_types::DAE::MatchType::MATCHCONTINUE, cases.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut tpl: (i32, Arc<DAE::Type>, i32);
            let mut patternMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>> = metamodelica::nil();
            let mut optPatternMatrix: Arc<metamodelica::List<Option<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>> = metamodelica::nil();
            let mut numNonEmptyColumns: i32 = 0;
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut outType: DAE::MatchType = outType.clone();
            let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = outCases.clone();
            let true = ((cases.clone().len() as i32) > 2) else { bail!("pattern mismatch") };
            for mut c in &*cases.clone() {
                let mut c = c.clone();
                ::match_deref::match_deref! { match &(c.clone()) {
                    Deref @ DAE::MatchCase { patternGuard: None, .. } => (),
                    _ => bail!("pattern mismatch"),
                } };
            }
            patternMatrix = List::transposeList(List::map(cases.clone(), (std::sync::Arc::new(getCasePatterns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::MatchCase>) -> Result<Arc<metamodelica::List<Arc<DAE::Pattern>>>> + 'static>)))?;
            (optPatternMatrix, numNonEmptyColumns) = removeWildPatternColumnsFromMatrix(patternMatrix.clone(), metamodelica::nil(), 0)?;
            tpl = findPatternToConvertToSwitch(optPatternMatrix.clone(), 1, numNonEmptyColumns.clone(), info.clone())?;
            (_, ty, _) = tpl.clone();
            r#str = (TypesDump::unparseType(ty.clone())?).clone();
            Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::MATCH_TO_SWITCH_OPTIMIZATION.clone(), list![(r#str.clone()).clone()], info.clone())?;
            outType = DAE::MatchType::MATCH { switch: Some(tpl.clone()) };
            outCases = optimizeSwitchedMatchCases(outType.clone(), cases.clone());
            Ok((outType.clone(), outCases.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((DAE::MatchType::MATCH { switch: None }, cases.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outType, outCases))
}

fn optimizeSwitchedMatchCases(mut inMatchType: DAE::MatchType, mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> Arc<metamodelica::List<Arc<DAE::MatchCase>>> {
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    outCases = (::match_deref::match_deref! { match &(inMatchType.clone()) {
        DAE::MatchType::MATCH { switch: Some((_, Deref @ DAE::Type::T_METATYPE { .. }, _)) } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut patl: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
            ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
        for mut c in (inCases.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ DAE::MatchCase { patterns: Deref @ metamodelica::List::Cons { head: pat @ Deref @ DAE::Pattern::PAT_CALL { patterns: patl, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut pat = (*pat).clone();
            if allPatternsWild(patl.clone()) {
                assign_variant_field!(pat => DAE::Pattern::PAT_CALL; knownSingleton = true);
                assign_field!(c.patterns = list![pat.clone()]);
            }
            c.clone()
        },
        _ => c.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            inCases.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCases
}

fn removeWildPatternColumnsFromMatrix(mut inPatternMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>, mut inAcc: Arc<metamodelica::List<Option<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>>, mut inNumAcc: i32) -> Result<(Arc<metamodelica::List<Option<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>>, i32)> {
    let mut optPatternMatrix: Arc<metamodelica::List<Option<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>> = metamodelica::nil();
    let mut numNonEmptyColumns: i32 = 0;
    (optPatternMatrix, numNonEmptyColumns) = (::match_deref::match_deref! { match &((inPatternMatrix.clone(), inAcc.clone(), inNumAcc.clone())) {
        (Deref @ metamodelica::List::Nil, acc, numAcc) => {
            (acc.clone().reverse(), numAcc.clone())
        },
        (Deref @ metamodelica::List::Cons { head: pats, tail: patternMatrix }, acc, numAcc) => {
            let mut alwaysMatch: bool = false;
            let mut optPats: Option<Arc<metamodelica::List<Arc<DAE::Pattern>>>> = None;
            let mut acc = (*acc).clone();
            let mut numAcc = (*numAcc).clone();
            alwaysMatch = allPatternsAlwaysMatch(List::stripLast(pats.clone())?);
            optPats = if (alwaysMatch.clone()) {None} else {Some(pats.clone())};
            numAcc = if (alwaysMatch.clone()) {numAcc.clone()} else {numAcc.clone() + 1};
            (acc, numAcc) = removeWildPatternColumnsFromMatrix(patternMatrix.clone(), cons(optPats.clone(), acc.clone()), numAcc.clone())?;
            (acc.clone(), numAcc.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((optPatternMatrix, numNonEmptyColumns))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findPatternToConvertToSwitch(mut inPatternMatrix: Arc<metamodelica::List<Option<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>>, mut index: i32, mut numPatternsInMatrix: i32, mut info: SourceInfo) -> Result<(i32, Arc<DAE::Type>, i32)> {
    let mut tpl: (i32, Arc<DAE::Type>, i32);
    tpl = 'mc: {
        let __mc_input = inPatternMatrix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Some(pats), tail: _ } => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut extraarg: i32 = 0;
                    (ty, extraarg) = findPatternToConvertToSwitch2(pats.clone(), metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT().clone(), true, numPatternsInMatrix.clone())?;
                    Ok((index.clone(), ty.clone(), extraarg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: patternMatrix } => {
                    Ok(findPatternToConvertToSwitch(patternMatrix.clone(), index.clone() + 1, numPatternsInMatrix.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tpl)
}

fn findPatternToConvertToSwitch2(mut ipats: Arc<metamodelica::List<Arc<DAE::Pattern>>>, mut ixs: Arc<metamodelica::List<i32>>, mut ity: Arc<DAE::Type>, mut allSubPatternsMatch: bool, mut numPatternsInMatrix: i32) -> Result<(Arc<DAE::Type>, i32)> {
    let mut outTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut extraarg: i32 = 0;
    (outTy, extraarg) = (::match_deref::match_deref! { match &((ipats.clone(), ity.clone(), allSubPatternsMatch.clone(), numPatternsInMatrix.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::SCONST { string: r#str }, .. }, tail: pats }, _, _, _) => {
            let mut ix: i32 = 0;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ix = stringHashDjb2Mod((r#str.clone()).clone(), 65536);
            let false = (listMember(ix.clone(), ixs.clone())) else { bail!("pattern mismatch") };
            (ty, extraarg) = findPatternToConvertToSwitch2(pats.clone(), cons(ix.clone(), ixs.clone()), DAE::T_STRING_DEFAULT().clone(), allSubPatternsMatch.clone(), numPatternsInMatrix.clone())?;
            (ty.clone(), extraarg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_CALL { patterns: subpats, index: ix, .. }, tail: pats }, _, _, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let false = (listMember(ix.clone(), ixs.clone())) else { bail!("pattern mismatch") };
            (ty, extraarg) = findPatternToConvertToSwitch2(pats.clone(), cons(ix.clone(), ixs.clone()), DAE::T_METATYPE_DEFAULT().clone(), allSubPatternsMatch.clone() && allPatternsAlwaysMatch(subpats.clone()), numPatternsInMatrix.clone())?;
            (ty.clone(), extraarg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::ICONST { integer: ix }, .. }, tail: pats }, _, _, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let false = (listMember(ix.clone(), ixs.clone())) else { bail!("pattern mismatch") };
            (ty, extraarg) = findPatternToConvertToSwitch2(pats.clone(), cons(ix.clone(), ixs.clone()), DAE::T_INTEGER_DEFAULT().clone(), allSubPatternsMatch.clone(), numPatternsInMatrix.clone())?;
            (ty.clone(), extraarg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::ENUM_LITERAL { index: ix, .. }, .. }, tail: pats }, _, _, _) if (!(listMember(ix.clone(), ixs.clone()))) => {
            findPatternToConvertToSwitch2(pats.clone(), cons(ix.clone(), ixs.clone()), DAE::T_ENUMERATION_DEFAULT().clone(), allSubPatternsMatch.clone(), numPatternsInMatrix.clone())?
        },
        (Deref @ metamodelica::List::Nil, Deref @ DAE::Type::T_STRING { .. }, _, _) => {
            let mut ix: i32 = 0;
            let true = ((ixs.clone().len() as i32) > 11) else { bail!("pattern mismatch") };
            ix = findMinMod(ixs.clone(), 1)?;
            (DAE::T_STRING_DEFAULT().clone(), ix.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, Deref @ DAE::Type::T_STRING { .. }, _, 1) => {
            let mut ix: i32 = 0;
            let true = ((ixs.clone().len() as i32) > 11) else { bail!("pattern mismatch") };
            ix = findMinMod(ixs.clone(), 1)?;
            (DAE::T_STRING_DEFAULT().clone(), ix.clone())
        },
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            (ity.clone(), 0)
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, _, true, 1) => {
            (ity.clone(), 0)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outTy, extraarg))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findMinMod(mut inIxs: Arc<metamodelica::List<i32>>, mut inMod: i32) -> Result<i32> {
    let mut outMod: i32 = 0;
    outMod = 'mc: {
        let __mc_input = (inIxs.clone(), inMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ixs, r#mod) => {
                    let mut ixs = (*ixs).clone();
                    ixs = List::map1(ixs.clone(), (std::sync::Arc::new(fnptr!(intMod, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), r#mod.clone());
                    ixs = List::sort(ixs.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    ::match_deref::match_deref! { match &(List::sortedDuplicates(ixs.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(r#mod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (inMod.clone() < 65536) else { bail!("pattern mismatch") };
                    Ok(findMinMod(inIxs.clone(), inMod.clone() * 2)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn filterUnusedPatterns(mut inputs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut info: SourceInfo, mut emitNotifications: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, Arc<metamodelica::List<Arc<DAE::MatchCase>>>)> {
    let mut outInputs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    (outInputs, outAliases, outCases) = 'mc: {
        let __mc_input = inCases.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cases => {
                    let mut patternMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>> = metamodelica::nil();
                    let mut cases = (*cases).clone();
                    let mut outAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = outAliases.clone();
                    let mut outInputs: Arc<metamodelica::List<Arc<DAE::Exp>>> = outInputs.clone();
                    patternMatrix = List::transposeList(List::map(cases.clone(), (std::sync::Arc::new(getCasePatterns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::MatchCase>) -> Result<Arc<metamodelica::List<Arc<DAE::Pattern>>>> + 'static>)))?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(filterUnusedPatterns2(inputs.clone(), inAliases.clone(), patternMatrix.clone(), false, info.clone(), emitNotifications.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?) {
                        (true, __pa0, __pa1, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    outInputs = __pa0.clone();
                    outAliases = __pa1.clone();
                    patternMatrix = __pa2.clone();
                    patternMatrix = List::transposeList(patternMatrix.clone())?;
                    cases = setCasePatternsCheckZero(cases.clone(), patternMatrix.clone());
                    Ok((outInputs.clone(), outAliases.clone(), cases.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inputs.clone(), inAliases.clone(), inCases.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outInputs, outAliases, outCases))
}

fn setCasePatternsCheckZero(mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut patternMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>) -> Arc<metamodelica::List<Arc<DAE::MatchCase>>> {
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    outCases = (::match_deref::match_deref! { match &((inCases.clone(), patternMatrix.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => inCases.clone(),
        (_, Deref @ metamodelica::List::Nil) => List::map1(inCases.clone(), (std::sync::Arc::new(setCasePatterns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::MatchCase>, Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<Arc<DAE::MatchCase>> + 'static>), metamodelica::nil()),
        _ => List::threadMap(inCases.clone(), patternMatrix.clone(), (std::sync::Arc::new(setCasePatterns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::MatchCase>, Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<Arc<DAE::MatchCase>> + 'static>)),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCases
}

fn filterUnusedPatterns2(mut inInputs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut inPatternMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>, mut change: bool, mut info: SourceInfo, mut emitNotifications: bool, mut inputsAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut aliasesAcc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut patternMatrixAcc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>) -> Result<(bool, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>)> {
    let mut outChange: bool = false;
    let mut outInputs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut outPatternMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>> = metamodelica::nil();
    (outChange, outInputs, outAliases, outPatternMatrix) = 'mc: {
        let __mc_input = (inInputs.clone(), inAliases.clone(), inPatternMatrix.clone(), change.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, true) => {
                    Ok((true, inputsAcc.clone().reverse(), aliasesAcc.clone().reverse(), patternMatrixAcc.clone().reverse()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: inputs }, Deref @ metamodelica::List::Cons { head: _, tail: aliases }, Deref @ metamodelica::List::Cons { head: pats, tail: patternMatrix }, _) => {
                    let mut outChange: bool = outChange.clone();
                    let mut outInputs: Arc<metamodelica::List<Arc<DAE::Exp>>> = outInputs.clone();
                    let mut outAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = outAliases.clone();
                    let mut outPatternMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>> = outPatternMatrix.clone();
                    ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(fnptr!(Expression::hasNoSideEffects, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), true)?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let true = (allPatternsWild(pats.clone())) else { bail!("pattern mismatch") };
                    if emitNotifications.clone() && Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? {
                        Error::addSourceMessage(Error::META_MATCH_UNUSED_INPUT.clone(), list![(ExpressionBasics::printExpStr(e.clone())?).clone()], info.clone())?;
                    }
                    (outChange, outInputs, outAliases, outPatternMatrix) = filterUnusedPatterns2(inputs.clone(), aliases.clone(), patternMatrix.clone(), true, info.clone(), emitNotifications.clone(), inputsAcc.clone(), aliasesAcc.clone(), patternMatrixAcc.clone())?;
                    Ok((outChange.clone(), outInputs.clone(), outAliases.clone(), outPatternMatrix.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: inputs }, Deref @ metamodelica::List::Cons { head: alias, tail: aliases }, Deref @ metamodelica::List::Cons { head: pats, tail: patternMatrix }, _) => {
                    let mut outPatternMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>> = outPatternMatrix.clone();
                    let mut outChange: bool = outChange.clone();
                    let mut outAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = outAliases.clone();
                    let mut outInputs: Arc<metamodelica::List<Arc<DAE::Exp>>> = outInputs.clone();
                    if emitNotifications.clone() && Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? && Expression::isCref(e.clone()) && allPatternsAlwaysMatch(pats.clone()) && !(allPatternsWild(pats.clone())) {
                        Error::addSourceMessage(Error::META_PATTERN_AS_ONLY.clone(), list![(ExpressionBasics::printExpStr(e.clone())?).clone(), (ExpressionBasics::printExpStr(e.clone())?).clone()], info.clone())?;
                    }
                    (outChange, outInputs, outAliases, outPatternMatrix) = filterUnusedPatterns2(inputs.clone(), aliases.clone(), patternMatrix.clone(), change.clone(), info.clone(), emitNotifications.clone(), cons(e.clone(), inputsAcc.clone()), cons(alias.clone(), aliasesAcc.clone()), cons(pats.clone(), patternMatrixAcc.clone()))?;
                    Ok((outChange.clone(), outInputs.clone(), outAliases.clone(), outPatternMatrix.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((false, metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outChange, outInputs, outAliases, outPatternMatrix))
}

fn getUsedLocalCrefs(mut skipFilterUnusedAsBindings: bool, mut exp: Arc<DAE::Exp>, mut hashSize: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))> {
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    ht = (::match_deref::match_deref! { match &((skipFilterUnusedAsBindings.clone(), exp.clone())) {
        (true, _) => {
            (_, ht) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(addLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)))> + 'static>), HashTableStringToPath::emptyHashTableSized(hashSize.clone()))?;
            ht.clone()
        },
        (false, Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. }) => {
            (_, ht) = traverseCases(cases.clone(), (std::sync::Arc::new(addLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)))> + 'static>), HashTableStringToPath::emptyHashTableSized(hashSize.clone()))?;
            ht.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ht)
}

fn filterUnusedAsBindings(mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<Arc<metamodelica::List<Arc<DAE::MatchCase>>>> {
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    outCases = (::match_deref::match_deref! { match &(inCases.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard: guardPattern, localDecls, body, result, resultInfo, jump, info }, tail: cases } => {
            let mut patterns = (*patterns).clone();
            let mut cases = (*cases).clone();
            (patterns, _) = traversePatternList(patterns.clone(), (std::sync::Arc::new(removePatternAsBinding) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, ((metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)), SourceInfo)) -> Result<(Arc<DAE::Pattern>, ((metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)), SourceInfo))> + 'static>), (ht.clone(), info.clone()))?;
            cases = filterUnusedAsBindings(cases.clone(), ht.clone())?;
            cons(Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: guardPattern.clone(), localDecls: localDecls.clone(), body: body.clone(), result: result.clone(), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() }), cases.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCases)
}

fn removePatternAsBinding(mut inPat: Arc<DAE::Pattern>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)), SourceInfo)) -> Result<(Arc<DAE::Pattern>, ((metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)), SourceInfo))> {
    let mut pat: Arc<DAE::Pattern> = inPat.clone();
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)), SourceInfo) = inTpl.clone();
    pat = 'mc: {
        let __mc_input = (pat.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Pattern::PAT_AS { pat, id, .. }, (ht, info)) => {
                    let true = (BaseHashTable::hasKey((id.clone()).clone(), ht.clone())) else { bail!("pattern mismatch") };
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::META_UNUSED_AS_BINDING.clone(), list![(id.clone()).clone()], info.clone())?;
                    Ok(pat.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { pat, id }, (ht, _)) => {
                    let true = (BaseHashTable::hasKey((id.clone()).clone(), ht.clone())) else { bail!("pattern mismatch") };
                    Ok(pat.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pat: Arc<DAE::Pattern> = pat.clone();
                    (pat, _) = simplifyPattern(inPat.clone(), 1);
                    Ok(pat.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((pat, outTpl))
}

fn addLocalCref(mut inExp: Arc<DAE::Exp>, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    (outExp, outHt) = (::match_deref::match_deref! { match &((inExp.clone(), inHt.clone())) {
        (exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, ht) => {
            let mut ht = (*ht).clone();
            ht = addLocalCrefHelper(cr.clone(), ht.clone())?;
            (exp.clone(), ht.clone())
        },
        (exp @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, path: Deref @ Absyn::Path::IDENT { name }, .. }, ht) => {
            let mut ht = (*ht).clone();
            ht = BaseHashTable::add((name.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() })), ht.clone())?;
            (exp.clone(), ht.clone())
        },
        (exp @ Deref @ DAE::Exp::PATTERN { pattern: pat }, ht) => {
            let mut ht = (*ht).clone();
            (_, ht) = traversePattern(pat.clone(), (std::sync::Arc::new(addPatternAsBindings) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr))) -> Result<(Arc<DAE::Pattern>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)))> + 'static>), ht.clone())?;
            (exp.clone(), ht.clone())
        },
        (exp @ Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. }, ht) => {
            let mut ht = (*ht).clone();
            ht = addCasesLocalCref(cases.clone(), ht.clone())?;
            (exp.clone(), ht.clone())
        },
        _ => {
            (inExp.clone(), inHt.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHt))
}

fn addLocalCrefHelper(mut cr: Arc<DAE::ComponentRef>, mut iht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))> {
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    ht = (::match_deref::match_deref! { match &((cr.clone(), iht.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, ident: name, .. }, ht) => {
            let mut ht = (*ht).clone();
            ht = addLocalCrefSubs(subs.clone(), ht.clone())?;
            ht = BaseHashTable::add((name.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() })), ht.clone())?;
            ht.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr2, subscriptLst: subs, ident: name, .. }, ht) => {
            let mut ht = (*ht).clone();
            ht = addLocalCrefSubs(subs.clone(), ht.clone())?;
            ht = BaseHashTable::add((name.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() })), ht.clone())?;
            addLocalCrefHelper(cr2.clone(), ht.clone())?
        },
        _ => {
            iht.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ht)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addLocalCrefSubs(mut isubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut iht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    outHt = (::match_deref::match_deref! { match &((isubs.clone(), iht.clone())) {
        (Deref @ metamodelica::List::Nil, ht) => {
            ht.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp }, tail: subs }, ht) => {
            let mut ht = (*ht).clone();
            (_, ht) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(addLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)))> + 'static>), ht.clone())?;
            ht = addLocalCrefSubs(subs.clone(), ht.clone())?;
            ht.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp }, tail: subs }, ht) => {
            let mut ht = (*ht).clone();
            (_, ht) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(addLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)))> + 'static>), ht.clone())?;
            ht = addLocalCrefSubs(subs.clone(), ht.clone())?;
            ht.clone()
        },
        _ => {
            iht.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outHt)
}

fn checkDefUse(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty, componentRef: cr }, extra @ (localsTree, useTree, info)) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut outExp: Arc<DAE::Exp>;
                    name = (ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone();
                    if AvlSetString::hasKey(localsTree.clone(), (name.clone()).clone())? && !(AvlSetString::hasKey(useTree.clone(), (name.clone()).clone())?) {
                        Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::META_UNUSED_ASSIGNMENT.clone(), list![(name.clone()).clone()], info.clone())?;
                        outExp = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD), ty: ty.clone() });
                    } else {
                        outExp = inExp.clone();
                    }
                    Ok((outExp.clone(), extra.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PATTERN { pattern: pat }, extra) => {
                    let mut pat = (*pat).clone();
                    let mut extra = (*extra).clone();
                    (pat, extra) = traversePattern(pat.clone(), (std::sync::Arc::new(checkDefUsePattern) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo)) -> Result<(Arc<DAE::Pattern>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo))> + 'static>), extra.clone())?;
                    Ok((Arc::new(DAE::Exp::PATTERN { pattern: pat.clone() }), extra.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

fn checkDefUsePattern(mut inPat: Arc<DAE::Pattern>, mut inTpl: (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo)) -> Result<(Arc<DAE::Pattern>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo))> {
    let mut outPat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    let mut outTpl: (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo) = inTpl.clone();
    outPat = (::match_deref::match_deref! { match &((inPat.clone(), inTpl.clone())) {
        (Deref @ DAE::Pattern::PAT_AS { pat, id: name, .. }, (localsTree, useTree, info)) => {
            let mut pat = (*pat).clone();
            if AvlSetString::hasKey(localsTree.clone(), (name.clone()).clone())? && !(AvlSetString::hasKey(useTree.clone(), (name.clone()).clone())?) {
                Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::META_UNUSED_AS_BINDING.clone(), list![(name.clone()).clone()], info.clone())?;
            } else {
                pat = inPat.clone();
            }
            pat.clone()
        },
        (Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { pat, id: name }, (localsTree, useTree, info)) => {
            let mut pat = (*pat).clone();
            if AvlSetString::hasKey(localsTree.clone(), (name.clone()).clone())? && !(AvlSetString::hasKey(useTree.clone(), (name.clone()).clone())?) {
                Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::META_UNUSED_AS_BINDING.clone(), list![(name.clone()).clone()], info.clone())?;
            } else {
                pat = inPat.clone();
            }
            pat.clone()
        },
        _ => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            (pat, _) = simplifyPattern(inPat.clone(), 1);
            pat.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPat, outTpl))
}

fn useLocalCref(mut inExp: Arc<DAE::Exp>, mut inTree: Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    (outExp, outTree) = (::match_deref::match_deref! { match &((inExp.clone(), inTree.clone())) {
        (exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tree) => {
            let mut tree = (*tree).clone();
            tree = useLocalCrefHelper(cr.clone(), tree.clone())?;
            (exp.clone(), tree.clone())
        },
        (exp @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, path: Deref @ Absyn::Path::IDENT { name }, .. }, tree) => {
            let mut tree = (*tree).clone();
            tree = AvlSetString::add(tree.clone(), (name.clone()).clone())?;
            (exp.clone(), tree.clone())
        },
        (exp @ Deref @ DAE::Exp::PATTERN { pattern: pat }, tree) => {
            let mut tree = (*tree).clone();
            (_, tree) = traversePattern(pat.clone(), (std::sync::Arc::new(usePatternAsBindings) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Pattern>, Arc<AvlSetString::Tree>)> + 'static>), tree.clone())?;
            (exp.clone(), tree.clone())
        },
        (exp @ Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. }, tree) => {
            let mut tree = (*tree).clone();
            tree = useCasesLocalCref(cases.clone(), tree.clone())?;
            (exp.clone(), tree.clone())
        },
        _ => {
            (inExp.clone(), inTree.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTree))
}

fn useLocalCrefHelper(mut cr: Arc<DAE::ComponentRef>, mut inTree: Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> {
    let mut tree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    tree = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, ident: name, .. } => {
            tree = useLocalCrefSubs(subs.clone(), inTree.clone())?;
            AvlSetString::add(tree.clone(), (name.clone()).clone())?
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr2, subscriptLst: subs, ident: name, .. } => {
            tree = useLocalCrefSubs(subs.clone(), inTree.clone())?;
            tree = AvlSetString::add(tree.clone(), (name.clone()).clone())?;
            useLocalCrefHelper(cr2.clone(), tree.clone())?
        },
        _ => {
            inTree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

fn useLocalCrefSubs(mut isubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inTree: Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> {
    let mut tree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    tree = (::match_deref::match_deref! { match &(isubs.clone()) {
        Deref @ metamodelica::List::Nil => {
            inTree.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp }, tail: subs } => {
            (_, tree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), inTree.clone())?;
            tree = useLocalCrefSubs(subs.clone(), tree.clone())?;
            tree.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp }, tail: subs } => {
            (_, tree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), inTree.clone())?;
            tree = useLocalCrefSubs(subs.clone(), tree.clone())?;
            tree.clone()
        },
        _ => {
            inTree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

fn usePatternAsBindings(mut inPat: Arc<DAE::Pattern>, mut inTree: Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Pattern>, Arc<AvlSetString::Tree>)> {
    let mut outPat: Arc<DAE::Pattern> = inPat.clone();
    let mut outTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    outTree = 'mc: {
        let __mc_input = inPat.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS { .. } => {
                    Ok(AvlSetString::add(inTree.clone(), (var_field!((*inPat).id, DAE::Pattern::PAT_AS).clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. } => {
                    Ok(AvlSetString::add(inTree.clone(), (var_field!((*inPat).id, DAE::Pattern::PAT_AS_FUNC_PTR).clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inTree.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outPat, outTree))
}

fn useCasesLocalCref(mut icases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut inTree: Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> {
    let mut tree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    tree = (::match_deref::match_deref! { match &(icases.clone()) {
        Deref @ metamodelica::List::Nil => {
            inTree.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns: pats, .. }, tail: cases } => {
            (_, tree) = traversePatternList(pats.clone(), (std::sync::Arc::new(usePatternAsBindings) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Pattern>, Arc<AvlSetString::Tree>)> + 'static>), inTree.clone())?;
            tree = useCasesLocalCref(cases.clone(), tree.clone())?;
            tree.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(tree)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addCasesLocalCref(mut icases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut iht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    outHt = (::match_deref::match_deref! { match &((icases.clone(), iht.clone())) {
        (Deref @ metamodelica::List::Nil, ht) => {
            ht.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns: pats, .. }, tail: cases }, ht) => {
            let mut ht = (*ht).clone();
            (_, ht) = traversePatternList(pats.clone(), (std::sync::Arc::new(addPatternAsBindings) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr))) -> Result<(Arc<DAE::Pattern>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)))> + 'static>), ht.clone())?;
            ht = addCasesLocalCref(cases.clone(), ht.clone())?;
            ht.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outHt)
}

fn simplifyPattern<A: Clone + 'static>(mut inPat: Arc<DAE::Pattern>, mut extra: A) -> (Arc<DAE::Pattern>, A) {
    let mut outPat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    let mut outExtra: A = extra.clone();
    outPat = (::match_deref::match_deref! { match &(inPat.clone()) {
        Deref @ DAE::Pattern::PAT_CALL_NAMED { name, patterns: namedPatterns } => {
            let mut namedPatterns = (*namedPatterns).clone();
            namedPatterns = List::filterOnTrue(namedPatterns.clone(), (std::sync::Arc::new(fnptr!(filterEmptyPattern, (Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>)) -> Result<bool> + 'static>));
            if (namedPatterns.clone().is_empty()) {Arc::new(openmodelica_frontend_types::DAE::Pattern::PAT_WILD)} else {Arc::new(DAE::Pattern::PAT_CALL_NAMED { name: name.clone(), patterns: namedPatterns.clone() })}
        },
        Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns } => {
            if (allPatternsWild(patterns.clone())) {Arc::new(openmodelica_frontend_types::DAE::Pattern::PAT_WILD)} else {inPat.clone()}
        },
        Deref @ DAE::Pattern::PAT_META_TUPLE { patterns } => {
            if (allPatternsWild(patterns.clone())) {Arc::new(openmodelica_frontend_types::DAE::Pattern::PAT_WILD)} else {inPat.clone()}
        },
        _ => {
            inPat.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outPat, outExtra)
}

fn addPatternAsBindings(mut inPat: Arc<DAE::Pattern>, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Pattern>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)))> {
    let mut pat: Arc<DAE::Pattern> = inPat.clone();
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr)) = inHt.clone();
    ht = 'mc: {
        let __mc_input = inPat.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS { id, .. } => {
                    Ok(BaseHashTable::add((id.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() })), ht.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, .. } => {
                    Ok(BaseHashTable::add((id.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() })), ht.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((pat, ht))
}

pub fn traversePatternList<TypeA: Clone + 'static>(mut inPatterns: Arc<metamodelica::List<Arc<DAE::Pattern>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, TypeA) -> Result<(Arc<DAE::Pattern>, TypeA)> + 'static>, mut inExtra: TypeA) -> Result<(Arc<metamodelica::List<Arc<DAE::Pattern>>>, TypeA)> {
    pub type Func<TypeA: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, TypeA) -> Result<(Arc<DAE::Pattern>, TypeA)> + 'static>;

    let mut outPatterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
    let mut extra: TypeA = inExtra.clone();
    let mut p: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    for mut pat in &*inPatterns.clone() {
        let mut pat = pat.clone();
        (p, extra) = traversePattern(pat.clone(), func.clone(), extra.clone())?;
        outPatterns = cons(p.clone(), outPatterns.clone());
    }
    outPatterns = Dangerous::listReverseInPlace(outPatterns.clone());
    Ok((outPatterns, extra))
}

pub fn traversePattern<TypeA: Clone + 'static>(mut inPattern: Arc<DAE::Pattern>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, TypeA) -> Result<(Arc<DAE::Pattern>, TypeA)> + 'static>, mut inExtra: TypeA) -> Result<(Arc<DAE::Pattern>, TypeA)> {
    pub type Func<TypeA: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, TypeA) -> Result<(Arc<DAE::Pattern>, TypeA)> + 'static>;

    let mut outPattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    let mut extra: TypeA = inExtra.clone();
    (outPattern, extra) = (::match_deref::match_deref! { match &(inPattern.clone()) {
        Deref @ DAE::Pattern::PAT_AS { id, ty, attr, pat: pat2 } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut pat2 = (*pat2).clone();
            (pat2, extra) = traversePattern(pat2.clone(), func.clone(), extra.clone())?;
            pat = Arc::new(DAE::Pattern::PAT_AS { id: (id.clone()).clone(), ty: ty.clone(), attr: attr.clone(), pat: pat2.clone() });
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, pat: pat2 } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut pat2 = (*pat2).clone();
            (pat2, extra) = traversePattern(pat2.clone(), func.clone(), extra.clone())?;
            pat = Arc::new(DAE::Pattern::PAT_AS_FUNC_PTR { id: (id.clone()).clone(), pat: pat2.clone() });
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        Deref @ DAE::Pattern::PAT_CALL { name, index, patterns: pats, fields: fieldVars, typeVars, knownSingleton } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut pats = (*pats).clone();
            (pats, extra) = traversePatternList(pats.clone(), func.clone(), extra.clone())?;
            pat = Arc::new(DAE::Pattern::PAT_CALL { name: name.clone(), index: index.clone(), patterns: pats.clone(), fields: fieldVars.clone(), typeVars: typeVars.clone(), knownSingleton: knownSingleton.clone() });
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        Deref @ DAE::Pattern::PAT_CALL_NAMED { name, patterns: namedpats } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut pats: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
            let mut fields: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut namedpats = (*namedpats).clone();
            (pats, fields, types) = List::unzip3(namedpats.clone());
            (pats, extra) = traversePatternList(pats.clone(), func.clone(), extra.clone())?;
            namedpats = List::zip3(pats.clone(), fields.clone(), types.clone());
            pat = Arc::new(DAE::Pattern::PAT_CALL_NAMED { name: name.clone(), patterns: namedpats.clone() });
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut pats = (*pats).clone();
            (pats, extra) = traversePatternList(pats.clone(), func.clone(), extra.clone())?;
            pat = Arc::new(DAE::Pattern::PAT_CALL_TUPLE { patterns: pats.clone() });
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut pats = (*pats).clone();
            (pats, extra) = traversePatternList(pats.clone(), func.clone(), extra.clone())?;
            pat = Arc::new(DAE::Pattern::PAT_META_TUPLE { patterns: pats.clone() });
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        Deref @ DAE::Pattern::PAT_CONS { head: pat1, tail: pat2 } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut pat1 = (*pat1).clone();
            let mut pat2 = (*pat2).clone();
            (pat1, extra) = traversePattern(pat1.clone(), func.clone(), extra.clone())?;
            (pat2, extra) = traversePattern(pat2.clone(), func.clone(), extra.clone())?;
            pat = Arc::new(DAE::Pattern::PAT_CONS { head: pat1.clone(), tail: pat2.clone() });
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        pat @ Deref @ DAE::Pattern::PAT_CONSTANT { .. } => {
            let mut pat = (*pat).clone();
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        Deref @ DAE::Pattern::PAT_SOME { pat: pat1 } => {
            let mut pat: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
            let mut pat1 = (*pat1).clone();
            (pat1, extra) = traversePattern(pat1.clone(), func.clone(), extra.clone())?;
            pat = Arc::new(DAE::Pattern::PAT_SOME { pat: pat1.clone() });
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        pat @ Deref @ DAE::Pattern::PAT_WILD => {
            let mut pat = (*pat).clone();
            (pat, extra) = func(pat.clone(), extra.clone())?;
            (pat.clone(), extra.clone())
        },
        pat => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Patternm.traversePattern failed: ")); __mm_s.push_str(&*patternStr(pat.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPattern, extra))
}

fn filterUnusedDecls(mut matchDecls: Arc<metamodelica::List<Arc<DAE::Element>>>, mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)), mut iacc: Arc<metamodelica::List<Arc<DAE::Element>>>, mut iunusedHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<ArcStr> + 'static>)))> {
    let mut outDecls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outUnusedHt: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Arc<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
    (outDecls, outUnusedHt) = 'mc: {
        let __mc_input = (matchDecls.clone(), iacc.clone(), iunusedHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, acc, unusedHt) => {
                    Ok((acc.clone().reverse(), unusedHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { source: Deref @ DAE::ElementSource { info, .. }, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, .. }, tail: rest }, acc, unusedHt) => {
                    let mut acc = (*acc).clone();
                    let mut unusedHt = (*unusedHt).clone();
                    let false = (BaseHashTable::hasKey((name.clone()).clone(), ht.clone())) else { bail!("pattern mismatch") };
                    unusedHt = BaseHashTable::add((name.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() })), unusedHt.clone())?;
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::META_UNUSED_DECL.clone(), list![(name.clone()).clone()], info.clone())?;
                    (acc, unusedHt) = filterUnusedDecls(rest.clone(), ht.clone(), acc.clone(), unusedHt.clone())?;
                    Ok((acc.clone(), unusedHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: el, tail: rest }, acc, unusedHt) => {
                    let mut acc = (*acc).clone();
                    let mut unusedHt = (*unusedHt).clone();
                    (acc, unusedHt) = filterUnusedDecls(rest.clone(), ht.clone(), cons(el.clone(), acc.clone()), unusedHt.clone())?;
                    Ok((acc.clone(), unusedHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDecls, outUnusedHt))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn caseDeadCodeElimination(mut matchType: Absyn::MatchType, mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut prevPatterns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>, mut iacc: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut iter: bool) -> Result<Arc<metamodelica::List<Arc<DAE::MatchCase>>>> {
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    outCases = 'mc: {
        let __mc_input = (matchType.clone(), cases.clone(), iacc.clone(), iter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, acc, false) => {
                    Ok(acc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, acc, true) => {
                    Ok(caseDeadCodeElimination(matchType.clone(), acc.clone().reverse(), metamodelica::nil(), metamodelica::nil(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { info, result: None, body: Deref @ metamodelica::List::Nil, .. }, tail: Deref @ metamodelica::List::Nil }, acc, _) => {
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::META_DEAD_CODE.clone(), list![(literal!("Last pattern is empty")).clone()], info.clone())?;
                    Ok(caseDeadCodeElimination(matchType.clone(), acc.clone().reverse(), metamodelica::nil(), metamodelica::nil(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::MatchType::MATCHCONTINUE, Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { info, result: None, body: Deref @ metamodelica::List::Nil, patterns: pats, .. }, tail: rest }, acc, _) => {
                    let mut acc = (*acc).clone();
                    let true = (Flags::isSet(Flags::PATTERNM_DCE.clone())?) else { bail!("pattern mismatch") };
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::META_DEAD_CODE.clone(), list![(literal!("Empty matchcontinue case")).clone()], info.clone())?;
                    acc = caseDeadCodeElimination(matchType.clone(), rest.clone(), cons(pats.clone(), prevPatterns.clone()), acc.clone(), true)?;
                    Ok(acc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: case_ @ Deref @ DAE::MatchCase { patterns: pats, .. }, tail: rest }, acc, _) => {
                    Ok(caseDeadCodeElimination(matchType.clone(), rest.clone(), cons(pats.clone(), prevPatterns.clone()), cons(case_.clone(), acc.clone()), iter.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCases)
}

/*
protected function findOverlappingPattern
  input list<DAE.Pattern> patterns;
  input list<DAE.MatchCase> prevCases;
  output SourceInfo info;
algorithm
  info := matchcontinue (patterns,prevCases)
    local
      list<DAE.Pattern> ps1,ps2;
    case (ps1,DAE.CASE(patterns=ps2,info=info)::_)
      algorithm
        true = patternListsDoOverlap(ps1,ps2); ???
      then info;
    case (ps1,_::prevCases) then findOverlappingPattern(ps1,prevCases);
  end matchcontinue;
end findOverlappingPattern;
*/
fn optimizeContinueJumps(mut matchType: Absyn::MatchType, mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> Result<Arc<metamodelica::List<Arc<DAE::MatchCase>>>> {
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    outCases = (match matchType.clone() {
        Absyn::MatchType::MATCH => cases.clone(),
        _ => optimizeContinueJumps2(cases.clone())?,
    });
    Ok(outCases)
}

fn optimizeContinueJumps2(mut icases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> Result<Arc<metamodelica::List<Arc<DAE::MatchCase>>>> {
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    outCases = (::match_deref::match_deref! { match &(icases.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: case_, tail: cases } => {
            let mut case_ = (*case_).clone();
            let mut cases = (*cases).clone();
            case_ = optimizeContinueJump(case_.clone(), cases.clone(), 0)?;
            cases = optimizeContinueJumps2(cases.clone())?;
            cons(case_.clone(), cases.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCases)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn optimizeContinueJump(mut case_: Arc<DAE::MatchCase>, mut icases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut jump: i32) -> Result<Arc<DAE::MatchCase>> {
    let mut outCase: Arc<DAE::MatchCase> = Arc::new(<DAE::MatchCase as ::std::default::Default>::default());
    outCase = 'mc: {
        let __mc_input = (case_.clone(), icases.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (case1, Deref @ metamodelica::List::Nil) => {
                    Ok(updateMatchCaseJump(case1.clone(), jump.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (case1 @ Deref @ DAE::MatchCase { patterns: ps1, .. }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns: ps2, .. }, tail: cases }) => {
                    let true = (patternListsDoNotOverlap(ps1.clone(), ps2.clone())?) else { bail!("pattern mismatch") };
                    Ok(optimizeContinueJump(case1.clone(), cases.clone(), jump.clone() + 1)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (case1, _) => {
                    Ok(updateMatchCaseJump(case1.clone(), jump.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCase)
}

fn updateMatchCaseJump(mut case_: Arc<DAE::MatchCase>, mut jump: i32) -> Result<Arc<DAE::MatchCase>> {
    let mut outCase: Arc<DAE::MatchCase> = Arc::new(<DAE::MatchCase as ::std::default::Default>::default());
    outCase = (::match_deref::match_deref! { match &((case_.clone(), jump.clone())) {
        (_, 0) => {
            case_.clone()
        },
        (Deref @ DAE::MatchCase { patterns, patternGuard: guardPattern, localDecls, body, result, resultInfo, jump: _, info }, _) => {
            Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: guardPattern.clone(), localDecls: localDecls.clone(), body: body.clone(), result: result.clone(), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCase)
}

fn optimizeContinueToMatch(mut matchType: Absyn::MatchType, mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut info: SourceInfo) -> Result<Absyn::MatchType> {
    let mut outMatchType: Absyn::MatchType = Absyn::MatchType::MATCH;
    outMatchType = (match matchType.clone() {
        Absyn::MatchType::MATCH => openmodelica_ast::Absyn::MatchType::MATCH,
        _ => optimizeContinueToMatch2(cases.clone(), metamodelica::nil(), info.clone())?,
    });
    if Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? {
        checkMatchContinueSingleCaseToTry(outMatchType.clone(), cases.clone(), info.clone())?;
    }
    Ok(outMatchType)
}

fn checkMatchContinueSingleCaseToTry(mut matchType: Absyn::MatchType, mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((matchType.clone(), cases.clone())) {
        (Absyn::MatchType::MATCHCONTINUE, Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns: firstPats, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns: elsePats, .. }, tail: Deref @ metamodelica::List::Nil } }) if (!(allPatternsWild(firstPats.clone())) && allPatternsWild(elsePats.clone())) => {
            Error::addSourceMessage(Error::MATCHCONTINUE_TO_TRY_OPTIMIZATION.clone(), metamodelica::nil(), info.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn optimizeContinueToMatch2(mut icases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut prevPatterns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>, mut info: SourceInfo) -> Result<Absyn::MatchType> {
    let mut outMatchType: Absyn::MatchType = Absyn::MatchType::MATCH;
    outMatchType = 'mc: {
        let __mc_input = icases.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::MATCHCONTINUE_TO_MATCH_OPTIMIZATION.clone(), metamodelica::nil(), info.clone())?;
                    Ok(openmodelica_ast::Absyn::MatchType::MATCH)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, .. }, tail: cases } => {
                    assertAllPatternListsDoNotOverlap(prevPatterns.clone(), patterns.clone())?;
                    Ok(optimizeContinueToMatch2(cases.clone(), cons(patterns.clone(), prevPatterns.clone()), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(openmodelica_ast::Absyn::MatchType::MATCHCONTINUE)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMatchType)
}

fn assertAllPatternListsDoNotOverlap(mut ipss1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Pattern>>>>>, mut ps2: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ipss1.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: ps1, tail: pss1 } => {
            let true = (patternListsDoNotOverlap(ps1.clone(), ps2.clone())?) else { bail!("pattern mismatch") };
            assertAllPatternListsDoNotOverlap(pss1.clone(), ps2.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn patternListsDoNotOverlap(mut ips1: Arc<metamodelica::List<Arc<DAE::Pattern>>>, mut ips2: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((ips1.clone(), ips2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            false
        },
        (Deref @ metamodelica::List::Cons { head: p1, tail: ps1 }, Deref @ metamodelica::List::Cons { head: p2, tail: ps2 }) => {
            let mut res: bool = false;
            res = patternsDoNotOverlap(p1.clone(), p2.clone())?;
            res = if (!(res.clone())) {patternListsDoNotOverlap(ps1.clone(), ps2.clone())?} else {res.clone()};
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn patternsDoNotOverlap(mut ip1: Arc<DAE::Pattern>, mut ip2: Arc<DAE::Pattern>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((ip1.clone(), ip2.clone())) {
        (Deref @ DAE::Pattern::PAT_WILD, _) => {
            false
        },
        (_, Deref @ DAE::Pattern::PAT_WILD) => {
            false
        },
        (Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. }, _) => {
            false
        },
        (Deref @ DAE::Pattern::PAT_AS { pat: p1, .. }, p2) => {
            patternsDoNotOverlap(p1.clone(), p2.clone())?
        },
        (p1, Deref @ DAE::Pattern::PAT_AS { pat: p2, .. }) => {
            patternsDoNotOverlap(p1.clone(), p2.clone())?
        },
        (Deref @ DAE::Pattern::PAT_CONS { head: head1, tail: tail1 }, Deref @ DAE::Pattern::PAT_CONS { head: head2, tail: tail2 }) => {
            patternsDoNotOverlap(head1.clone(), head2.clone())? || patternsDoNotOverlap(tail1.clone(), tail2.clone())?
        },
        (Deref @ DAE::Pattern::PAT_SOME { pat: p1 }, Deref @ DAE::Pattern::PAT_SOME { pat: p2 }) => {
            patternsDoNotOverlap(p1.clone(), p2.clone())?
        },
        (Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: ps1 }, Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: ps2 }) => {
            patternListsDoNotOverlap(ps1.clone(), ps2.clone())?
        },
        (Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: ps1 }, Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: ps2 }) => {
            patternListsDoNotOverlap(ps1.clone(), ps2.clone())?
        },
        (Deref @ DAE::Pattern::PAT_CALL { name: name1, index: ix1, patterns: Deref @ metamodelica::List::Nil, fields: _, typeVars: _, .. }, Deref @ DAE::Pattern::PAT_CALL { name: name2, index: ix2, patterns: Deref @ metamodelica::List::Nil, fields: _, typeVars: _, .. }) => {
            let mut res: bool = false;
            res = ix1.clone() == ix2.clone();
            res = if (res.clone()) {AbsynUtil::pathEqual(name1.clone(), name2.clone())} else {res.clone()};
            !(res.clone())
        },
        (Deref @ DAE::Pattern::PAT_CALL { name: name1, index: ix1, patterns: ps1, fields: _, typeVars: _, .. }, Deref @ DAE::Pattern::PAT_CALL { name: name2, index: ix2, patterns: ps2, fields: _, typeVars: _, .. }) => {
            let mut res: bool = false;
            res = ix1.clone() == ix2.clone();
            res = if (res.clone()) {AbsynUtil::pathEqual(name1.clone(), name2.clone())} else {res.clone()};
            res = if (res.clone()) {patternListsDoNotOverlap(ps1.clone(), ps2.clone())?} else {!(res.clone())};
            res.clone()
        },
        (Deref @ DAE::Pattern::PAT_CONSTANT { exp: e1, .. }, Deref @ DAE::Pattern::PAT_CONSTANT { exp: e2, .. }) => {
            !(ExpressionBasics::expEqual(e1.clone(), e2.clone())?)
        },
        (Deref @ DAE::Pattern::PAT_CONSTANT { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Pattern::PAT_CONSTANT { .. }) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn elabMatchCases(mut cache: FCore::Cache, mut env: FCore::Graph, mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>, mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inputAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut matchExpLocalTree: Arc<AvlSetString::Tree>, mut r#impl: bool, mut performVectorization: bool, mut pre: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::MatchCase>>>, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut elabCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut resExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut resTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut tysFixed: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    tysFixed = List::map(tys.clone(), (std::sync::Arc::new(Types::getUniontypeIfMetarecordReplaceAllSubtypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
    (outCache, elabCases, resExps, resTypes) = elabMatchCases2(cache.clone(), env.clone(), cases.clone(), tysFixed.clone(), inputAliases.clone(), matchExpLocalTree.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
    (elabCases, resType) = fixCaseReturnTypes(elabCases.clone(), resExps.clone(), resTypes.clone(), info.clone())?;
    Ok((outCache, elabCases, resType))
}

fn elabMatchCases2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>, mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inputAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut matchExpLocalTree: Arc<AvlSetString::Tree>, mut r#impl: bool, mut performVectorization: bool, mut pre: DAE::Prefix, mut inAccCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut inAccExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAccTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::MatchCase>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut elabCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    let mut resExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut resTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outCache, elabCases, resExps, resTypes) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), cases.clone(), inAccExps.clone(), inAccTypes.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, accExps, accTypes) => {
            (cache.clone(), inAccCases.clone().reverse(), accExps.clone().reverse(), accTypes.clone().reverse())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: case_, tail: rest }, accExps, accTypes) => {
            let mut elabCase: Arc<DAE::MatchCase> = Arc::new(<DAE::MatchCase as ::std::default::Default>::default());
            let mut optType: Option<Arc<DAE::Type>> = None;
            let mut optExp: Option<Arc<DAE::Exp>> = None;
            let mut cache = (*cache).clone();
            let mut accExps = (*accExps).clone();
            let mut accTypes = (*accTypes).clone();
            (cache, elabCase, optExp, optType) = elabMatchCase(cache.clone(), env.clone(), case_.clone(), tys.clone(), inputAliases.clone(), matchExpLocalTree.clone(), r#impl.clone(), performVectorization.clone(), pre.clone())?;
            (cache, elabCases, accExps, accTypes) = elabMatchCases2(cache.clone(), env.clone(), rest.clone(), tys.clone(), inputAliases.clone(), matchExpLocalTree.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), cons(elabCase.clone(), inAccCases.clone()), List::consOption(optExp.clone(), accExps.clone()), List::consOption(optType.clone(), accTypes.clone()))?;
            (cache.clone(), elabCases.clone(), accExps.clone(), accTypes.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, elabCases, resExps, resTypes))
}

fn elabMatchCase(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut acase: Arc<Absyn::Case>, mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inputAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut matchExpLocalTree: Arc<AvlSetString::Tree>, mut r#impl: bool, mut performVectorization: bool, mut pre: DAE::Prefix) -> Result<(FCore::Cache, Arc<DAE::MatchCase>, Option<Arc<DAE::Exp>>, Option<Arc<DAE::Type>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut elabCase: Arc<DAE::MatchCase> = Arc::new(<DAE::MatchCase as ::std::default::Default>::default());
    let mut resExp: Option<Arc<DAE::Exp>> = None;
    let mut resType: Option<Arc<DAE::Type>> = None;
    (outCache, elabCase, resExp, resType) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), acase.clone())) {
        (cache, env, Deref @ Absyn::Case::CASE { info, resultInfo, result, classPart: cp, localDecls: decls, patternInfo, patternGuard, pattern, .. }) => {
            let mut patterns: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut elabPatterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
            let mut elabPatterns2: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
            let mut elabResult: Option<Arc<DAE::Exp>> = None;
            let mut dPatternGuard: Option<Arc<DAE::Exp>> = None;
            let mut caseDecls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eqAlgs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut algs: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
            let mut body: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut caseLocalTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
            let mut localsTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
            let mut useTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut resultInfo = (*resultInfo).clone();
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLocalDecls(cache.clone(), env.clone(), decls.clone(), (arcstr::literal!(FCore::caseScopeName)).clone(), r#impl.clone(), info.clone())?) {
                (__pa0, Some((__pa1, DAE::DAElist { elementLst: __pa2 }, __pa3))) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            env = __pa1.clone();
            caseDecls = __pa2.clone();
            caseLocalTree = __pa3.clone();
            patterns = convertExpToPatterns(pattern.clone());
            patterns = if ((tys.clone().len() as i32) == 1) {list![pattern.clone()]} else {patterns.clone()};
            (cache, elabPatterns) = elabPatternTuple(cache.clone(), env.clone(), patterns.clone(), tys.clone(), patternInfo.clone(), pattern.clone())?;
            checkPatternsDuplicateAsBindings(elabPatterns.clone(), patternInfo.clone())?;
            env = FGraph::openNewScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, Some((arcstr::literal!(FCore::patternTypeScope)).clone()), None)?;
            (elabPatterns2, cache) = addPatternAliasesList(elabPatterns.clone(), inputAliases.clone(), cache.clone(), inEnv.clone())?;
            (_, env) = traversePatternList(elabPatterns2.clone(), (std::sync::Arc::new(addEnvKnownAsBindings) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, FCore::Graph) -> Result<(Arc<DAE::Pattern>, FCore::Graph)> + 'static>), env.clone())?;
            eqAlgs = Static::fromEquationsToAlgAssignments(cp.clone())?;
            algs = AbsynToSCode::translateClassdefAlgorithmitems(eqAlgs.clone())?;
            (cache, body) = InstSection::instStatements(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), pre.clone(), ClassInf::State::FUNCTION { path: Arc::new(Absyn::Path::IDENT { name: (literal!("match")).clone() }), isImpure: false }, algs.clone(), ElementSource::addElementSourceFileInfo(DAE::emptyElementSource().clone(), patternInfo.clone()), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, true, InstTypes::neverUnroll.clone())?;
            (cache, body, elabResult, resultInfo, resType) = elabResultExp(cache.clone(), env.clone(), body.clone(), result.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), resultInfo.clone())?;
            (cache, dPatternGuard) = elabPatternGuard(cache.clone(), env.clone(), patternGuard.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), patternInfo.clone())?;
            localsTree = AvlSetString::join(matchExpLocalTree.clone(), caseLocalTree.clone())?;
            useTree = AvlSetString::new();
            (_, useTree) = Expression::traverseExpBottomUp(Arc::new(DAE::Exp::META_OPTION { exp: elabResult.clone() }), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), useTree.clone());
            (_, useTree) = Expression::traverseExpBottomUp(Arc::new(DAE::Exp::META_OPTION { exp: dPatternGuard.clone() }), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            (elabPatterns, _) = traversePatternList(elabPatterns.clone(), (std::sync::Arc::new(checkDefUsePattern) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo)) -> Result<(Arc<DAE::Pattern>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo))> + 'static>), (localsTree.clone(), useTree.clone(), patternInfo.clone()))?;
            useTree = AvlSetString::new();
            (_, useTree) = Expression::traverseExpBottomUp(Arc::new(DAE::Exp::META_OPTION { exp: elabResult.clone() }), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), useTree.clone());
            (_, useTree) = Expression::traverseExpBottomUp(Arc::new(DAE::Exp::META_OPTION { exp: dPatternGuard.clone() }), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            (elabPatterns, _) = traversePatternList(elabPatterns.clone(), (std::sync::Arc::new(checkDefUsePattern) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo)) -> Result<(Arc<DAE::Pattern>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo))> + 'static>), (localsTree.clone(), useTree.clone(), patternInfo.clone()))?;
            elabCase = Arc::new(DAE::MatchCase { patterns: elabPatterns.clone(), patternGuard: dPatternGuard.clone(), localDecls: caseDecls.clone(), body: body.clone(), result: elabResult.clone(), resultInfo: resultInfo.clone(), jump: 0, info: info.clone() });
            (cache.clone(), elabCase.clone(), elabResult.clone(), resType.clone())
        },
        (cache, env, Deref @ Absyn::Case::ELSE { info, resultInfo, result, classPart: cp, localDecls: decls, .. }) => {
            let mut pattern: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut patterns: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut elabResult: Option<Arc<DAE::Exp>> = None;
            let mut len: i32 = 0;
            let mut cache = (*cache).clone();
            len = (tys.clone().len() as i32);
            patterns = List::fill(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(openmodelica_ast::Absyn::ComponentRef::WILD) }), (tys.clone().len() as i32));
            pattern = if (len.clone() == 1) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(openmodelica_ast::Absyn::ComponentRef::WILD) })} else {Arc::new(Absyn::Exp::TUPLE { expressions: patterns.clone() })};
            (cache, elabCase, elabResult, resType) = elabMatchCase(cache.clone(), env.clone(), Arc::new(Absyn::Case::CASE { pattern: pattern.clone(), patternGuard: None, patternInfo: info.clone(), localDecls: decls.clone(), classPart: cp.clone(), result: result.clone(), resultInfo: resultInfo.clone(), comment: None, info: info.clone() }), tys.clone(), inputAliases.clone(), matchExpLocalTree.clone(), r#impl.clone(), performVectorization.clone(), pre.clone())?;
            (cache.clone(), elabCase.clone(), elabResult.clone(), resType.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, elabCase, resExp, resType))
}

fn elabResultExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inBody: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut exp: Arc<Absyn::Exp>, mut r#impl: bool, mut performVectorization: bool, mut pre: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>, Option<Arc<DAE::Exp>>, SourceInfo, Option<Arc<DAE::Type>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outBody: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut resExp: Option<Arc<DAE::Exp>> = None;
    let mut resultInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut resType: Option<Arc<DAE::Type>> = None;
    (outCache, outBody, resExp, resultInfo, resType) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inBody.clone(), AbsynUtil::stripCommentExpressions(exp.clone(), false)?);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, body, Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Nil, argNames: Deref @ metamodelica::List::Nil }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "fail", subscripts: Deref @ metamodelica::List::Nil }, .. }) => {
                    Ok((cache.clone(), body.clone(), None, inInfo.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, body, _) => {
                    let mut elabExp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut cache = (*cache).clone();
                    let mut body = (*body).clone();
                    (cache, elabExp, prop) = Static::elabExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), inInfo.clone())?;
                    ty = Types::getPropType(prop.clone())?;
                    (elabExp, ty) = makeTupleFromMetaTuple(elabExp.clone(), ty.clone())?;
                    (body, elabExp, info) = elabResultExp2(!(Flags::isSet(Flags::PATTERNM_MOVE_LAST_EXP.clone())?), body.clone(), elabExp.clone(), inInfo.clone())?;
                    Ok((cache.clone(), body.clone(), Some(elabExp.clone()), info.clone(), Some(ty.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outBody, resExp, resultInfo, resType))
}

fn elabPatternGuard(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut patternGuard: Option<Arc<Absyn::Exp>>, mut r#impl: bool, mut performVectorization: bool, mut pre: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Option<Arc<DAE::Exp>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outPatternGuard: Option<Arc<DAE::Exp>> = None;
    (outCache, outPatternGuard) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), patternGuard.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, None, _) => {
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Some(exp), info) => {
                    let mut elabExp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, elabExp, prop) = Static::elabExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), info.clone())?;
                    (elabExp, _) = Types::matchType(elabExp.clone(), Types::getPropType(prop.clone())?, DAE::T_BOOL_DEFAULT().clone(), true)?;
                    Ok((cache.clone(), Some(elabExp.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Some(exp), info) => {
                    let mut prop: DAE::Properties;
                    let mut r#str: ArcStr = arcstr::literal!("");
                    (_, _, prop) = Static::elabExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), performVectorization.clone(), pre.clone(), info.clone())?;
                    r#str = (TypesDump::unparseType(Types::getPropType(prop.clone())?)?).clone();
                    Error::addSourceMessage(Error::GUARD_EXPRESSION_TYPE_MISMATCH.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outPatternGuard))
}

fn elabResultExp2(mut skipPhase: bool, mut body: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut elabExp: Arc<DAE::Exp>, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<DAE::Exp>, SourceInfo)> {
    let mut outBody: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outExp: Arc<DAE::Exp>;
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    (outBody, outExp, outInfo) = 'mc: {
        let __mc_input = (skipPhase.clone(), body.clone(), elabExp.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, b, e, i) => {
                    Ok((b.clone(), e.clone(), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, b, elabCr2 @ Deref @ DAE::Exp::CREF { .. }, _) => {
                    let mut elabCr1: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut i: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut b = (*b).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(List::splitLast(b.clone())?) {
                        (Deref @ DAE::Statement::STMT_ASSIGN { source: Deref @ DAE::ElementSource { info: __pa0, .. }, exp: __pa1, exp1: __pa2, .. }, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    e = __pa1.clone();
                    elabCr1 = __pa2.clone();
                    b = __pa3.clone();
                    let true = (ExpressionBasics::expEqual(elabCr1.clone(), elabCr2.clone())?) else { bail!("pattern mismatch") };
                    (b, e, i) = elabResultExp2(false, b.clone(), e.clone(), i.clone())?;
                    Ok((b.clone(), e.clone(), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, b, Deref @ DAE::Exp::TUPLE { PR: elabCrs2 }, _) => {
                    let mut elabCrs1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e: Arc<DAE::Exp>;
                    let mut i: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut b = (*b).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(List::splitLast(b.clone())?) {
                        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source: Deref @ DAE::ElementSource { info: __pa0, .. }, exp: __pa1, expExpLst: __pa2, .. }, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    e = __pa1.clone();
                    elabCrs1 = __pa2.clone();
                    b = __pa3.clone();
                    let true = (List::isEqualOnTrue(elabCrs1.clone(), elabCrs2.clone(), (std::sync::Arc::new(ExpressionBasics::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    (b, e, i) = elabResultExp2(false, b.clone(), e.clone(), i.clone())?;
                    Ok((b.clone(), e.clone(), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((body.clone(), elabExp.clone(), info.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outBody, outExp, outInfo))
}

fn fixCaseReturnTypes(mut icases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut iexps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut itys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::MatchCase>>>, Arc<DAE::Type>)> {
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outCases, ty) = 'mc: {
        let __mc_input = (icases.clone(), iexps.clone(), itys.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cases, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok((cases.clone(), DAE::T_NORETCALL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cases, exps, tys) => {
                    let mut cases = (*cases).clone();
                    let mut exps = (*exps).clone();
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    ty = List::reduce(List::map(tys.clone(), (std::sync::Arc::new(Types::boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>)), (std::sync::Arc::new(Types::superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>))?;
                    ty = Types::superType(ty.clone(), ty.clone())?;
                    ty = Types::unboxedType(ty.clone())?;
                    ty = Types::makeRegularTupleFromMetaTupleOnTrue(Types::allTuple(tys.clone()), ty.clone())?;
                    ty = Types::getUniontypeIfMetarecordReplaceAllSubtypes(ty.clone())?;
                    (exps, _) = Types::matchTypes(exps.clone(), tys.clone(), ty.clone(), true)?;
                    cases = fixCaseReturnTypes2(cases.clone(), exps.clone(), info.clone())?;
                    Ok((cases.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cases, exps, tys) => {
                    let mut cases = (*cases).clone();
                    let mut exps = (*exps).clone();
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    ty = List::reduce(tys.clone(), (std::sync::Arc::new(Types::superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>))?;
                    ty = Types::superType(ty.clone(), ty.clone())?;
                    ty = Types::unboxedType(ty.clone())?;
                    ty = Types::makeRegularTupleFromMetaTupleOnTrue(Types::allTuple(tys.clone()), ty.clone())?;
                    ty = Types::getUniontypeIfMetarecordReplaceAllSubtypes(ty.clone())?;
                    (exps, _) = Types::matchTypes(exps.clone(), tys.clone(), ty.clone(), true)?;
                    cases = fixCaseReturnTypes2(cases.clone(), exps.clone(), info.clone())?;
                    Ok((cases.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    tys = List::unionOnTrue(itys.clone(), metamodelica::nil(), (std::sync::Arc::new(Types::equivtypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<bool> + 'static>));
                    r#str = stringAppendList(List::map1r(List::map(tys.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>)), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("\n  ")).clone()));
                    Error::addSourceMessage(Error::META_MATCHEXP_RESULT_TYPES.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCases, ty))
}

pub fn fixCaseReturnTypes2(mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<Arc<DAE::MatchCase>>>> {
    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    outCases = 'mc: {
        let __mc_input = (inCases.clone(), inExps.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls: decls, body, result: Some(_), resultInfo, jump, info: info2 }, tail: cases }, Deref @ metamodelica::List::Cons { head: exp, tail: exps }, info) => {
                    let mut cases = (*cases).clone();
                    cases = fixCaseReturnTypes2(cases.clone(), exps.clone(), info.clone())?;
                    Ok(cons(Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard.clone(), localDecls: decls.clone(), body: body.clone(), result: Some(exp.clone()), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info2.clone() }), cases.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: case_ @ Deref @ DAE::MatchCase { result: None, .. }, tail: cases }, exps, info) => {
                    let mut cases = (*cases).clone();
                    cases = fixCaseReturnTypes2(cases.clone(), exps.clone(), info.clone())?;
                    Ok(cons(case_.clone(), cases.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Patternm.fixCaseReturnTypes2 failed")).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCases)
}

pub fn traverseConstantPatternsHelper<T: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut inT: T, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>) -> Result<(Arc<DAE::Exp>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outT: T = inT.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        outExp @ Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. } => {
            let mut cases2: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
            let mut case_: Arc<DAE::MatchCase> = Arc::new(<DAE::MatchCase as ::std::default::Default>::default());
            let mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
            let mut outExp = (*outExp).clone();
            cases2 = metamodelica::nil();
            for mut c in &*cases.clone() {
                let mut c = c.clone();
                case_ = c.clone();
                case_ = (::match_deref::match_deref! { match &(case_.clone()) {
        Deref @ DAE::MatchCase { .. } => {
            (patterns, outT) = traversePatternList(case_.patterns.clone(), Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a1| Ok(traverseConstantPatternsHelper2(__pe_a0, __pe_a1, __pe_b2.clone())) }), outT.clone())?;
            if !(case_.patterns.clone() == patterns.clone()) {
                assign_field!(case_.patterns = patterns.clone());
            }
            case_.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
                cases2 = cons(case_.clone(), cases2.clone());
            }
            cases2 = Dangerous::listReverseInPlace(cases2.clone());
            if !(cases.clone() == cases2.clone()) {
                assign_variant_field!(outExp => DAE::Exp::MATCHEXPRESSION; cases = cases2.clone());
            }
            (outExp, outT) = func(outExp.clone(), outT.clone())?;
            outExp.clone()
        },
        _ => {
            (outExp, outT) = func(inExp.clone(), outT.clone())?;
            outExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outT))
}

pub fn traverseConstantPatternsHelper2<T: Clone + 'static>(mut inPattern: Arc<DAE::Pattern>, mut inExtra: T, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>) -> (Arc<DAE::Pattern>, T) {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outPattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    let mut extra: T = inExtra.clone();
    outPattern = (::match_deref::match_deref! { match &(inPattern.clone()) {
        outPattern @ Deref @ DAE::Pattern::PAT_CONSTANT { .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut outPattern = (*outPattern).clone();
            (exp, extra) = func(var_field!((*outPattern).exp, DAE::Pattern::PAT_CONSTANT).clone(), extra.clone()).unwrap();
            if !(referenceEq(&var_field!((*outPattern).exp, DAE::Pattern::PAT_CONSTANT).clone(),&exp.clone())) {
                assign_variant_field!(outPattern => DAE::Pattern::PAT_CONSTANT; exp = exp.clone());
            }
            outPattern.clone()
        },
        _ => {
            inPattern.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outPattern, extra)
}

pub fn traverseCases<A: Clone + 'static>(mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, A)> + 'static>, mut inA: A) -> Result<(Arc<metamodelica::List<Arc<DAE::MatchCase>>>, A)> {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, A)> + 'static>;

    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    let mut oa: A;
    (outCases, oa) = (::match_deref::match_deref! { match &((inCases.clone(), inA.clone())) {
        (Deref @ metamodelica::List::Nil, a) => {
            (metamodelica::nil(), a.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls: decls, body, result, resultInfo, jump, info }, tail: cases }, a) => {
            let mut body1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut result1: Option<Arc<DAE::Exp>> = None;
            let mut patternGuard1: Option<Arc<DAE::Exp>> = None;
            let mut cases1: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
            let mut cases = (*cases).clone();
            let mut a = (*a).clone();
            let (__pa0, (_, __pa1)) = DAEUtil::traverseDAEEquationsStmts(body.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), (func.clone(), a.clone()));
            body1 = __pa0.clone();
            a = __pa1.clone();
            (patternGuard1, a) = Expression::traverseExpOpt(patternGuard.clone(), func.clone(), a.clone())?;
            (result1, a) = Expression::traverseExpOpt(result.clone(), func.clone(), a.clone())?;
            (cases1, a) = traverseCases(cases.clone(), func.clone(), a.clone())?;
            cases = if (referenceEq(&cases.clone(),&cases1.clone()) && referenceEq(&patternGuard.clone(),&patternGuard1.clone()) && referenceEq(&result.clone(),&result1.clone()) && referenceEq(&body.clone(),&body1.clone())) {inCases.clone()} else {cons(Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard1.clone(), localDecls: decls.clone(), body: body1.clone(), result: result1.clone(), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() }), cases1.clone())};
            (cases.clone(), a.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCases, oa))
}

pub fn traverseCasesTopDown<A: Clone + 'static>(mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, bool, A)> + 'static>, mut inA: A) -> Result<(Arc<metamodelica::List<Arc<DAE::MatchCase>>>, A)> {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, bool, A)> + 'static>;

    let mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    let mut a: A = inA.clone();
    let mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
    let mut decls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut body: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut body1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut result: Option<Arc<DAE::Exp>> = None;
    let mut result1: Option<Arc<DAE::Exp>> = None;
    let mut patternGuard: Option<Arc<DAE::Exp>> = None;
    let mut patternGuard1: Option<Arc<DAE::Exp>> = None;
    let mut jump: i32 = 0;
    let mut resultInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut tpl: (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, bool, A)> + 'static>, A);
    for mut c in &*inCases.clone() {
        let mut c = c.clone();
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(c.clone()) {
            Deref @ DAE::MatchCase { patterns: __pa0, patternGuard: __pa1, localDecls: __pa2, body: __pa3, result: __pa4, resultInfo: __pa5, jump: __pa6, info: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
            _ => bail!("pattern mismatch"),
        } };
        patterns = __pa0.clone();
        patternGuard = __pa1.clone();
        decls = __pa2.clone();
        body = __pa3.clone();
        result = __pa4.clone();
        resultInfo = __pa5.clone();
        jump = __pa6.clone();
        info = __pa7.clone();
        tpl = (func.clone(), a.clone());
        let (__pa8, (_, __pa9)) = DAEUtil::traverseDAEEquationsStmts(body.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsTopDownHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), tpl.clone());
        body1 = __pa8.clone();
        a = __pa9.clone();
        (patternGuard1, a) = Expression::traverseExpOptTopDown(patternGuard.clone(), func.clone(), a.clone())?;
        (result1, a) = Expression::traverseExpOptTopDown(result.clone(), func.clone(), a.clone())?;
        cases = cons(Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard1.clone(), localDecls: decls.clone(), body: body1.clone(), result: result1.clone(), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() }), cases.clone());
    }
    cases = cases.clone().reverse();
    Ok((cases, a))
}

fn filterEmptyPattern(mut tpl: (Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>)) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(tpl.clone()) {
        (Deref @ DAE::Pattern::PAT_WILD, _, _) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn addLocalDecls(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut els: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut scopeName: ArcStr, mut r#impl: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Option<(FCore::Graph, DAE::DAElist, Arc<AvlSetString::Tree>)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut res: Option<(FCore::Graph, DAE::DAElist, Arc<AvlSetString::Tree>)> = None;
    (outCache, res) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), els.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Nil) => {
                    let mut declsTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
                    declsTree = AvlSetString::new();
                    Ok((cache.clone(), Some((env.clone(), DAE::emptyDae().clone(), declsTree.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ld) => {
                    let mut ld2: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut ld_mod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut dae1: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
                    let mut env2: FCore::Graph;
                    let mut dummyFunc: ClassInf::State;
                    let mut b: bool = false;
                    let mut declsTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut res: Option<(FCore::Graph, DAE::DAElist, Arc<AvlSetString::Tree>)> = res.clone();
                    env2 = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (scopeName.clone()).clone(), None)?;
                    ld2 = AbsynToSCode::translateEitemlist(ld.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
                    let true = (List::applyAndFold1(ld2.clone(), (std::sync::Arc::new(fnptr!(boolAnd, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(SCodeUtil::isComponentWithDirection, Arc<SCode::Element>, Absyn::Direction)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Absyn::Direction) -> Result<bool> + 'static>), openmodelica_ast::Absyn::Direction::BIDIR, true)) else { bail!("pattern mismatch") };
                    (cache, b) = List::fold1(ld2.clone(), (std::sync::Arc::new(checkLocalShadowing) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, FCore::Graph, (FCore::Cache, bool)) -> Result<(FCore::Cache, bool)> + 'static>), env.clone(), (cache.clone(), false));
                    ld2 = if (b.clone()) {metamodelica::nil()} else {ld2.clone()};
                    ld_mod = InstUtil::addNomod(ld2.clone());
                    dummyFunc = ClassInf::State::FUNCTION { path: Arc::new(Absyn::Path::IDENT { name: (literal!("dummieFunc")).clone() }), isImpure: false };
                    (cache, env2, _) = InstUtil::addComponentsToEnv(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, dummyFunc.clone(), ld_mod.clone(), r#impl.clone())?;
                    (cache, env2, _, _, dae1, _, _, _, _, _) = Inst::instElementList(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, dummyFunc.clone(), ld_mod.clone(), metamodelica::nil(), r#impl.clone(), openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), true)?;
                    names = List::map(ld2.clone(), (std::sync::Arc::new(SCodeUtil::elementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
                    declsTree = AvlSetString::addList(AvlSetString::new(), names.clone())?;
                    res = if (b.clone()) {None} else {Some((env2.clone(), dae1.clone(), declsTree.clone()))};
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ld) => {
                    let mut ld2: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    ld2 = AbsynToSCode::translateEitemlist(ld.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
                    let __pa0 = ::match_deref::match_deref! { match &(List::filterOnTrue(ld2.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isNotComponent, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>))) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ld2 = __pa0.clone();
                    r#str = stringDelimitList(List::map1(ld2.clone(), (std::sync::Arc::new(SCodeDump::unparseElementStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, SCodeDump::SCodeDumpOptions) -> Result<ArcStr> + 'static>), SCodeDump::defaultOptions.clone()), (literal!(", ")).clone());
                    Error::addSourceMessage(Error::META_INVALID_LOCAL_ELEMENT.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ld) => {
                    let mut ld2: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut ld3: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut ld4: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    ld2 = AbsynToSCode::translateEitemlist(ld.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
                    ld3 = List::select1(ld2.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isComponentWithDirection, Arc<SCode::Element>, Absyn::Direction)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Absyn::Direction) -> Result<bool> + 'static>), openmodelica_ast::Absyn::Direction::INPUT);
                    ld4 = List::select1(ld2.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isComponentWithDirection, Arc<SCode::Element>, Absyn::Direction)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Absyn::Direction) -> Result<bool> + 'static>), openmodelica_ast::Absyn::Direction::OUTPUT);
                    let __pa0 = ::match_deref::match_deref! { match &(listAppend(ld3.clone(), ld4.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ld2 = __pa0.clone();
                    r#str = stringDelimitList(List::map1(ld2.clone(), (std::sync::Arc::new(SCodeDump::unparseElementStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, SCodeDump::SCodeDumpOptions) -> Result<ArcStr> + 'static>), SCodeDump::defaultOptions.clone()), (literal!(", ")).clone());
                    Error::addSourceMessage(Error::META_INVALID_LOCAL_ELEMENT.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Patternm.addLocalDecls failed")).clone()], info.clone())?;
                    Ok((inCache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, res))
}

fn checkLocalShadowing(mut elt: Arc<SCode::Element>, mut env: FCore::Graph, mut inTpl: (FCore::Cache, bool)) -> Result<(FCore::Cache, bool)> {
    let mut outTpl: (FCore::Cache, bool) = inTpl.clone();
    let mut name: ArcStr = arcstr::literal!("");
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut b: bool = false;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut var: SCode::Variability = SCode::Variability::CONST;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::COMPONENT { info: __pa0, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    name = __pa1.clone();
    (cache, _) = inTpl.clone();
    match '__try2: {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(Lookup::lookupVarInternalIdent(cache.clone(), env.clone(), (name.clone()).clone(), metamodelica::nil(), openmodelica_frontend_inst::InstTypes::SearchStrategy::SEARCH_LOCAL_ONLY), '__try2)) {
            (__pa3, Deref @ DAE::Attributes { variability: __pa4, .. }, _, _, _, _, _, _, _) => (__pa3.clone(), __pa4.clone()),
            _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        cache = __pa3.clone();
        var = __pa4.clone();
        b = (match var.clone() {
        SCode::Variability::CONST => true,
        _ => false,
    });
        Ok::<_, anyhow::Error>((b.clone(),))
    } {
        Ok((__try2_o0,)) => {
            b = __try2_o0;
        }
        Err(_) => {
            b = true;
        }
    }
    if !(b.clone()) {
        Error::addSourceMessage(Error::MATCH_SHADOWING.clone(), list![(name.clone()).clone()], info.clone())?;
        outTpl = (cache.clone(), true);
    }
    Ok(outTpl)
}

pub fn resultExps(mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    exps = (::match_deref::match_deref! { match &(inCases.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { result: Some(exp), .. }, tail: cases } => {
            exps = resultExps(cases.clone());
            cons(exp.clone(), exps.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: cases } => {
            resultExps(cases.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exps
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn allPatternsWild(mut ipats: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ipats.clone()) {
        Deref @ metamodelica::List::Nil => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_WILD, tail: pats } => {
            allPatternsWild(pats.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn allPatternsAlwaysMatch(mut ipats: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ipats.clone()) {
        Deref @ metamodelica::List::Nil => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_WILD, tail: pats } => {
            allPatternsAlwaysMatch(pats.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_AS { pat, .. }, tail: pats } => {
            allPatternsAlwaysMatch(cons(pat.clone(), pats.clone()))
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { pat, .. }, tail: pats } => {
            allPatternsAlwaysMatch(cons(pat.clone(), pats.clone()))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn isInfallibleNoBinding(mut pat: Arc<DAE::Pattern>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(pat.clone()) {
        Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
            List::all(pats.clone(), (std::sync::Arc::new(fnptr!(isInfallibleNoBindingOrWild, Arc<DAE::Pattern>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<bool> + 'static>))
        },
        Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
            List::all(pats.clone(), (std::sync::Arc::new(fnptr!(isInfallibleNoBindingOrWild, Arc<DAE::Pattern>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<bool> + 'static>))
        },
        Deref @ DAE::Pattern::PAT_CALL { patterns: pats, knownSingleton: true, .. } => {
            List::all(pats.clone(), (std::sync::Arc::new(fnptr!(isInfallibleNoBindingOrWild, Arc<DAE::Pattern>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<bool> + 'static>))
        },
        Deref @ DAE::Pattern::PAT_CALL_NAMED { patterns: namedPats, name: _ } => {
            namedPats.clone().is_empty()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn isInfallibleNoBindingOrWild(mut pat: Arc<DAE::Pattern>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(pat.clone()) {
        Deref @ DAE::Pattern::PAT_WILD => true,
        _ => isInfallibleNoBinding(pat.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isInfalliblePattern(mut pat: Arc<DAE::Pattern>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(pat.clone()) {
        Deref @ DAE::Pattern::PAT_WILD => {
            true
        },
        Deref @ DAE::Pattern::PAT_AS { pat: innerPat, .. } => {
            isInfalliblePattern(innerPat.clone())
        },
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { pat: innerPat, .. } => {
            isInfalliblePattern(innerPat.clone())
        },
        Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
            List::all(pats.clone(), (std::sync::Arc::new(fnptr!(isInfalliblePattern, Arc<DAE::Pattern>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<bool> + 'static>))
        },
        Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
            List::all(pats.clone(), (std::sync::Arc::new(fnptr!(isInfalliblePattern, Arc<DAE::Pattern>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<bool> + 'static>))
        },
        Deref @ DAE::Pattern::PAT_CALL { patterns: pats, knownSingleton: true, .. } => {
            List::all(pats.clone(), (std::sync::Arc::new(fnptr!(isInfalliblePattern, Arc<DAE::Pattern>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<bool> + 'static>))
        },
        Deref @ DAE::Pattern::PAT_CALL_NAMED { patterns: namedPats, .. } => {
            namedPats.clone().is_empty()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn isSingleInfallibleMatch(mut matchType: Absyn::MatchType, mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((matchType.clone(), cases.clone())) {
        (Absyn::MatchType::MATCH, Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns: pats, .. }, tail: Deref @ metamodelica::List::Nil }) => {
            List::all(pats.clone(), (std::sync::Arc::new(fnptr!(isInfalliblePattern, Arc<DAE::Pattern>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>) -> Result<bool> + 'static>))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn checkMatchSingleInfallibleCase(mut matchType: Absyn::MatchType, mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut info: SourceInfo) -> Result<()> {
    if Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? && isSingleInfallibleMatch(matchType.clone(), cases.clone()) {
        Error::addSourceMessage(Error::MATCH_SINGLE_INFALLIBLE_CASE.clone(), metamodelica::nil(), info.clone())?;
    }
    Ok(())
}

fn checkInfallibleNoBindingPatterns(mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut matchType: Absyn::MatchType, mut info: SourceInfo) -> Result<()> {
    if !(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?) {
        return Ok(());
    }
    if isSingleInfallibleMatch(matchType.clone(), cases.clone()) {
        return Ok(());
    }
    for mut c in &*cases.clone() {
        let mut c = c.clone();
        let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ DAE::MatchCase { info: cinfo, patterns: pats, .. } => {
            for mut p in &*pats.clone() {
                let mut p = p.clone();
                checkPatternInfallibleNoBinding(p.clone(), cinfo.clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    Ok(())
}

fn checkPatternInfallibleNoBinding(mut pat: Arc<DAE::Pattern>, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(pat.clone()) {
        Deref @ DAE::Pattern::PAT_WILD => {
            ()
        },
        _ if (isInfallibleNoBinding(pat.clone())) => {
            Error::addSourceMessage(Error::META_PATTERN_INFALLIBLE_NO_BINDING.clone(), list![(patternStr(pat.clone())?).clone()], info.clone())?;
            ()
        },
        Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
            for mut p in &*pats.clone() {
                let mut p = p.clone();
                checkPatternInfallibleNoBinding(p.clone(), info.clone())?;
            }
            ()
        },
        Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
            for mut p in &*pats.clone() {
                let mut p = p.clone();
                checkPatternInfallibleNoBinding(p.clone(), info.clone())?;
            }
            ()
        },
        Deref @ DAE::Pattern::PAT_CALL { patterns: pats, .. } => {
            for mut p in &*pats.clone() {
                let mut p = p.clone();
                checkPatternInfallibleNoBinding(p.clone(), info.clone())?;
            }
            ()
        },
        Deref @ DAE::Pattern::PAT_CALL_NAMED { patterns: namedPats, .. } => {
            for mut tpl in &*namedPats.clone() {
                let mut tpl = tpl.clone();
                checkPatternInfallibleNoBinding(Util::tuple31(tpl.clone()), info.clone())?;
            }
            ()
        },
        Deref @ DAE::Pattern::PAT_CONS { head: innerPat, .. } => {
            checkPatternInfallibleNoBinding(innerPat.clone(), info.clone())?;
            checkPatternInfallibleNoBinding(var_field!((*pat).tail, DAE::Pattern::PAT_CONS).clone(), info.clone())?;
            ()
        },
        Deref @ DAE::Pattern::PAT_SOME { pat: innerPat } => {
            checkPatternInfallibleNoBinding(innerPat.clone(), info.clone())?;
            ()
        },
        Deref @ DAE::Pattern::PAT_AS { pat: innerPat, .. } => {
            checkPatternInfallibleNoBinding(innerPat.clone(), info.clone())?;
            ()
        },
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { pat: innerPat, .. } => {
            checkPatternInfallibleNoBinding(innerPat.clone(), info.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn getCasePatterns(mut case_: Arc<DAE::MatchCase>) -> Result<Arc<metamodelica::List<Arc<DAE::Pattern>>>> {
    let mut pats: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(case_.clone()) {
        Deref @ DAE::MatchCase { patterns: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    pats = __pa0.clone();
    Ok(pats)
}

fn setCasePatterns(mut case1: Arc<DAE::MatchCase>, mut pats: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<Arc<DAE::MatchCase>> {
    let mut case2: Arc<DAE::MatchCase> = Arc::new(<DAE::MatchCase as ::std::default::Default>::default());
    case2 = (::match_deref::match_deref! { match &(case1.clone()) {
        Deref @ DAE::MatchCase { patterns: _, patternGuard, localDecls, body, result, resultInfo, jump, info } => {
            Arc::new(DAE::MatchCase { patterns: pats.clone(), patternGuard: patternGuard.clone(), localDecls: localDecls.clone(), body: body.clone(), result: result.clone(), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(case2)
}

pub fn getValueCtor(mut ix: i32) -> i32 {
    let mut ctor: i32 = 0;
    ctor = ix.clone() + 3;
    ctor
}

pub fn sortPatternsByComplexity(mut inPatterns: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Pattern>, i32)>>> {
    let mut outPatterns: Arc<metamodelica::List<(Arc<DAE::Pattern>, i32)>> = metamodelica::nil();
    outPatterns = List::toListWithPositions(inPatterns.clone());
    outPatterns = List::sort(outPatterns.clone(), (std::sync::Arc::new(sortPatternsByComplexityWork) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Pattern>, i32), (Arc<DAE::Pattern>, i32)) -> Result<bool> + 'static>))?;
    Ok(outPatterns)
}

fn sortPatternsByComplexityWork(mut tpl1: (Arc<DAE::Pattern>, i32), mut tpl2: (Arc<DAE::Pattern>, i32)) -> Result<bool> {
    let mut greater: bool = false;
    let mut pat1: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    let mut pat2: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    (pat1, i1) = tpl1.clone();
    (pat2, i2) = tpl2.clone();
    (_, c1) = traversePattern(pat1.clone(), (std::sync::Arc::new(patternComplexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, i32) -> Result<(Arc<DAE::Pattern>, i32)> + 'static>), 0)?;
    (_, c2) = traversePattern(pat2.clone(), (std::sync::Arc::new(patternComplexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, i32) -> Result<(Arc<DAE::Pattern>, i32)> + 'static>), 0)?;
    greater = if (c1.clone() == c2.clone()) {i1.clone() > i2.clone()} else {if (c2.clone() == 0) {false} else {if (c1.clone() == 0) {true} else {c1.clone() > c2.clone()}}};
    Ok(greater)
}

fn patternComplexity(mut inPat: Arc<DAE::Pattern>, mut inComplexity: i32) -> Result<(Arc<DAE::Pattern>, i32)> {
    let mut outPat: Arc<DAE::Pattern> = inPat.clone();
    let mut i: i32 = inComplexity.clone();
    i = (::match_deref::match_deref! { match &(inPat.clone()) {
        Deref @ DAE::Pattern::PAT_CONSTANT { exp, .. } => {
            (_, i) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(fnptr!(constantComplexity, Arc<DAE::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<(Arc<DAE::Exp>, i32)> + 'static>), i.clone())?;
            i.clone()
        },
        Deref @ DAE::Pattern::PAT_CONS { .. } => {
            i.clone() + 5
        },
        Deref @ DAE::Pattern::PAT_CALL { knownSingleton: false, .. } => {
            i.clone() + 5
        },
        Deref @ DAE::Pattern::PAT_SOME { .. } => {
            i.clone() + 5
        },
        _ => {
            i.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPat, i))
}

fn constantComplexity(mut inExp: Arc<DAE::Exp>, mut ii: i32) -> (Arc<DAE::Exp>, i32) {
    let mut outExp: Arc<DAE::Exp>;
    let mut oi: i32 = 0;
    (outExp, oi) = (::match_deref::match_deref! { match &((inExp.clone(), ii.clone())) {
        (e @ Deref @ DAE::Exp::SCONST { string: r#str }, i) => {
            (e.clone(), i.clone() + 5 + ((r#str.clone()).clone().len() as i32))
        },
        (e @ Deref @ DAE::Exp::ICONST { integer: _ }, i) => {
            (e.clone(), i.clone() + 1)
        },
        (e @ Deref @ DAE::Exp::BCONST { bool: _ }, i) => {
            (e.clone(), i.clone() + 1)
        },
        (e @ Deref @ DAE::Exp::RCONST { real: _ }, i) => {
            (e.clone(), i.clone() + 2)
        },
        (e, i) => {
            (e.clone(), i.clone() + 5)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, oi)
}

fn addEnvKnownAsBindings(mut inPat: Arc<DAE::Pattern>, mut inEnv: FCore::Graph) -> Result<(Arc<DAE::Pattern>, FCore::Graph)> {
    let mut pat: Arc<DAE::Pattern> = inPat.clone();
    let mut env: FCore::Graph = inEnv.clone();
    env = (::match_deref::match_deref! { match &(pat.clone()) {
        Deref @ DAE::Pattern::PAT_AS { .. } => addEnvKnownAsBindings2(pat.clone(), env.clone(), findFirstNonAsPattern(var_field!((*pat).pat, DAE::Pattern::PAT_AS).clone()))?,
        _ => env.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((pat, env))
}

fn addEnvKnownAsBindings2(mut inPat: Arc<DAE::Pattern>, mut inEnv: FCore::Graph, mut firstPattern: Arc<DAE::Pattern>) -> Result<FCore::Graph> {
    let mut env: FCore::Graph = inEnv.clone();
    env = (::match_deref::match_deref! { match &((inPat.clone(), firstPattern.clone())) {
        (Deref @ DAE::Pattern::PAT_AS { attr, id, .. }, Deref @ DAE::Pattern::PAT_CALL { name, knownSingleton, fields, typeVars, index, .. }) => {
            let mut path: Arc<Absyn::Path>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            path = AbsynUtil::stripLast(name.clone())?;
            ty = Arc::new(DAE::Type::T_METARECORD { path: name.clone(), utPath: path.clone(), typeVars: typeVars.clone(), index: index.clone(), fields: fields.clone(), knownSingleton: knownSingleton.clone() });
            env = FGraph::mkComponentNode(env.clone(), Arc::new(DAE::Var { name: (id.clone()).clone(), attributes: attr.clone(), ty: ty.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(SCode::Element::COMPONENT { name: (id.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultVarAttr.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: name.clone(), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_DAE, FGraph::empty())?;
            env.clone()
        },
        _ => {
            env.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(env)
}

fn findFirstNonAsPattern(mut inPattern: Arc<DAE::Pattern>) -> Arc<DAE::Pattern> {
    let mut outPattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
    outPattern = (::match_deref::match_deref! { match &(inPattern.clone()) {
        Deref @ DAE::Pattern::PAT_AS { pat: outPattern, .. } => findFirstNonAsPattern(outPattern.clone()),
        _ => inPattern.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPattern
}

fn getInputAsBinding(mut inExp: Arc<Absyn::Exp>) -> (Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>) {
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut aliases: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut aliasesAndCrefs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (exp, aliases, aliasesAndCrefs) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::List::Nil } } => {
            (inExp.clone(), metamodelica::nil(), list![(id.clone()).clone()])
        },
        Deref @ Absyn::Exp::AS { id, exp } => {
            let mut exp = (*exp).clone();
            (exp, aliases, aliasesAndCrefs) = getInputAsBinding(exp.clone());
            (exp.clone(), cons((id.clone()).clone(), aliases.clone()), cons((id.clone()).clone(), aliasesAndCrefs.clone()))
        },
        _ => {
            (inExp.clone(), metamodelica::nil(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, aliases, aliasesAndCrefs)
}

fn addPatternAliasesList(mut inPatterns: Arc<metamodelica::List<Arc<DAE::Pattern>>>, mut inAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<metamodelica::List<Arc<DAE::Pattern>>>, FCore::Cache)> {
    let mut outPatterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
    let mut outCache: FCore::Cache = inCache.clone();
    let mut aliases: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut rest_aliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = inAliases.clone();
    for mut pat in &*inPatterns.clone() {
        let mut pat = pat.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_aliases.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        aliases = __pa0.clone();
        rest_aliases = __pa1.clone();
        (pat, outCache) = addPatternAliases(pat.clone(), aliases.clone(), outCache.clone(), inEnv.clone())?;
        outPatterns = cons(pat.clone(), outPatterns.clone());
    }
    outPatterns = outPatterns.clone().reverse();
    Ok((outPatterns, outCache))
}

fn addPatternAliases(mut inPattern: Arc<DAE::Pattern>, mut inAliases: Arc<metamodelica::List<ArcStr>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<DAE::Pattern>, FCore::Cache)> {
    let mut pat: Arc<DAE::Pattern> = inPattern.clone();
    let mut outCache: FCore::Cache = inCache.clone();
    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    for mut alias in &*inAliases.clone() {
        let mut alias = alias.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(outCache.clone(), inEnv.clone(), (alias.clone()).clone())?) {
            (__pa0, Deref @ DAE::Var { attributes: __pa1, .. }, _, _, _, _) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        outCache = __pa0.clone();
        attr = __pa1.clone();
        pat = Arc::new(DAE::Pattern::PAT_AS { id: (alias.clone()).clone(), ty: None, attr: attr.clone(), pat: pat.clone() });
    }
    Ok((pat, outCache))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addAliasesToEnv(mut inEnv: FCore::Graph, mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inAliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut info: SourceInfo) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = (::match_deref::match_deref! { match &((inEnv.clone(), inTypes.clone(), inAliases.clone())) {
        (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            inEnv.clone()
        },
        (_, Deref @ metamodelica::List::Cons { head: _, tail: tys }, Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Nil, tail: aliases }) => {
            addAliasesToEnv(inEnv.clone(), tys.clone(), aliases.clone(), info.clone())?
        },
        (env, Deref @ metamodelica::List::Cons { head: ty, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: id, tail: rest }, tail: aliases }) => {
            let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
            let mut env = (*env).clone();
            attr = DAE::dummyAttrInput().clone();
            env = FGraph::mkComponentNode(env.clone(), Arc::new(DAE::Var { name: (id.clone()).clone(), attributes: attr.clone(), ty: ty.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(SCode::Element::COMPONENT { name: (id.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultVarAttr.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$dummy")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: info.clone() }), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_DAE, FGraph::empty())?;
            addAliasesToEnv(env.clone(), inTypes.clone(), cons(rest.clone(), aliases.clone()), info.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

fn statementListFindDeadStoreRemoveEmptyStatements(mut inBody: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut localsTree: Arc<AvlSetString::Tree>, mut inUseTree: Arc<AvlSetString::Tree>) -> (Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<AvlSetString::Tree>) {
    let mut body: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut useTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    (body, useTree) = List::map1Fold(inBody.clone().reverse(), (std::sync::Arc::new(statementFindDeadStore) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Statement>, Arc<AvlSetString::Tree>)> + 'static>), localsTree.clone(), inUseTree.clone());
    body = List::select(body.clone(), (std::sync::Arc::new(isNotDummyStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<bool> + 'static>));
    body = body.clone().reverse();
    (body, useTree)
}

fn statementFindDeadStore(mut inStatement: Arc<DAE::Statement>, mut localsTree: Arc<AvlSetString::Tree>, mut inUseTree: Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Statement>, Arc<AvlSetString::Tree>)> {
    let mut outStatement: Arc<DAE::Statement>;
    let mut useTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    (outStatement, useTree) = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { source: source @ Deref @ DAE::ElementSource { info, .. }, exp, exp1: lhs, type_: ty } => {
            let mut lhs = (*lhs).clone();
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), inUseTree.clone())?;
            (lhs, _) = Expression::traverseExpBottomUp(lhs.clone(), (std::sync::Arc::new(checkDefUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo))> + 'static>), (localsTree.clone(), useTree.clone(), info.clone()))?;
            outStatement = Algorithm::makeAssignmentNoTypeCheck(ty.clone(), lhs.clone(), exp.clone(), source.clone());
            (outStatement.clone(), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source: source @ Deref @ DAE::ElementSource { info, .. }, exp, expExpLst: exps, type_: ty } => {
            let mut exps = (*exps).clone();
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), inUseTree.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(Arc::new(DAE::Exp::TUPLE { PR: exps.clone() }), (std::sync::Arc::new(checkDefUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo))> + 'static>), (localsTree.clone(), useTree.clone(), info.clone()))?) {
                (Deref @ DAE::Exp::TUPLE { PR: __pa0 }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            exps = __pa0.clone();
            outStatement = Algorithm::makeTupleAssignmentNoTypeCheck(ty.clone(), exps.clone(), exp.clone(), source.clone())?;
            (outStatement.clone(), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { source: source @ Deref @ DAE::ElementSource { info, .. }, exp, lhs, type_: ty } => {
            let mut lhs = (*lhs).clone();
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), inUseTree.clone())?;
            (lhs, _) = Expression::traverseExpBottomUp(lhs.clone(), (std::sync::Arc::new(checkDefUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>, SourceInfo))> + 'static>), (localsTree.clone(), useTree.clone(), info.clone()))?;
            outStatement = Algorithm::makeArrayAssignmentNoTypeCheck(ty.clone(), lhs.clone(), exp.clone(), source.clone());
            (outStatement.clone(), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_IF { source, else_, statementLst: body, exp } => {
            let mut elseTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
            let mut else_ = (*else_).clone();
            let mut body = (*body).clone();
            (else_, elseTree) = elseFindDeadStore(else_.clone(), localsTree.clone(), inUseTree.clone())?;
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), inUseTree.clone());
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            useTree = AvlSetString::join(useTree.clone(), elseTree.clone())?;
            (Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: body.clone(), else_: else_.clone(), source: source.clone() }), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_FOR { type_: ty, iterIsArray: b, iter: id, range: exp, statementLst: body, source } => {
            let mut body = (*body).clone();
            ErrorExt::setCheckpoint((literal!("Patternm.statementFindDeadStore")).clone());
            (_, useTree) = List::map1Fold(body.clone(), (std::sync::Arc::new(statementFindDeadStore) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Statement>, Arc<AvlSetString::Tree>)> + 'static>), localsTree.clone(), inUseTree.clone());
            ErrorExt::rollBack((literal!("Patternm.statementFindDeadStore")).clone());
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), useTree.clone());
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            useTree = AvlSetString::join(useTree.clone(), inUseTree.clone())?;
            (Arc::new(DAE::Statement::STMT_FOR { type_: ty.clone(), iterIsArray: b.clone(), iter: (id.clone()).clone(), range: exp.clone(), statementLst: body.clone(), source: source.clone() }), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_WHILE { source, statementLst: body, exp } => {
            let mut body = (*body).clone();
            ErrorExt::setCheckpoint((literal!("Patternm.statementFindDeadStore")).clone());
            (_, useTree) = List::map1Fold(body.clone(), (std::sync::Arc::new(statementFindDeadStore) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<AvlSetString::Tree>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Statement>, Arc<AvlSetString::Tree>)> + 'static>), localsTree.clone(), inUseTree.clone());
            ErrorExt::rollBack((literal!("Patternm.statementFindDeadStore")).clone());
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), useTree.clone());
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            useTree = AvlSetString::join(useTree.clone(), inUseTree.clone())?;
            (Arc::new(DAE::Statement::STMT_WHILE { exp: exp.clone(), statementLst: body.clone(), source: source.clone() }), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_PARFOR { .. } => {
            bail!("fail")
        },
        Deref @ DAE::Statement::STMT_ASSERT { level, msg, cond, .. } => {
            (_, useTree) = Expression::traverseExpBottomUp(cond.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), inUseTree.clone())?;
            (_, useTree) = Expression::traverseExpBottomUp(msg.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            (_, useTree) = Expression::traverseExpBottomUp(level.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            (inStatement.clone(), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_TERMINATE { msg: exp, .. } => {
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), AvlSetString::new())?;
            (inStatement.clone(), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_WHEN { .. } => {
            bail!("fail")
        },
        Deref @ DAE::Statement::STMT_REINIT { .. } => {
            bail!("fail")
        },
        Deref @ DAE::Statement::STMT_NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }, .. }, .. } => {
            (inStatement.clone(), AvlSetString::new())
        },
        Deref @ DAE::Statement::STMT_RETURN { .. } => {
            (inStatement.clone(), AvlSetString::new())
        },
        Deref @ DAE::Statement::STMT_NORETCALL { exp, .. } => {
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), inUseTree.clone())?;
            (inStatement.clone(), useTree.clone())
        },
        Deref @ DAE::Statement::STMT_BREAK { .. } => {
            (inStatement.clone(), inUseTree.clone())
        },
        Deref @ DAE::Statement::STMT_CONTINUE { .. } => {
            (inStatement.clone(), inUseTree.clone())
        },
        Deref @ DAE::Statement::STMT_ARRAY_INIT { .. } => {
            (inStatement.clone(), inUseTree.clone())
        },
        Deref @ DAE::Statement::STMT_FAILURE { source, body } => {
            let mut body = (*body).clone();
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), inUseTree.clone());
            (Arc::new(DAE::Statement::STMT_FAILURE { body: body.clone(), source: source.clone() }), useTree.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outStatement, useTree))
}

fn elseFindDeadStore(mut inElse: Arc<DAE::Else>, mut localsTree: Arc<AvlSetString::Tree>, mut inUseTree: Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Else>, Arc<AvlSetString::Tree>)> {
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut useTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    (outElse, useTree) = (::match_deref::match_deref! { match &(inElse.clone()) {
        Deref @ DAE::Else::NOELSE => {
            (inElse.clone(), inUseTree.clone())
        },
        Deref @ DAE::Else::ELSEIF { exp, statementLst: body, else_ } => {
            let mut elseTree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
            let mut body = (*body).clone();
            let mut else_ = (*else_).clone();
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), inUseTree.clone());
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(useLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetString::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetString::Tree>)> + 'static>), useTree.clone())?;
            (else_, elseTree) = elseFindDeadStore(else_.clone(), localsTree.clone(), inUseTree.clone())?;
            useTree = AvlSetString::join(useTree.clone(), elseTree.clone())?;
            else_ = Arc::new(DAE::Else::ELSEIF { exp: exp.clone(), statementLst: body.clone(), else_: else_.clone() });
            (else_.clone(), useTree.clone())
        },
        Deref @ DAE::Else::ELSE { statementLst: body } => {
            let mut else_: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            let mut body = (*body).clone();
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), inUseTree.clone());
            else_ = Arc::new(DAE::Else::ELSE { statementLst: body.clone() });
            (else_.clone(), useTree.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outElse, useTree))
}

fn isNotDummyStatement(mut statement: Arc<DAE::Statement>) -> Result<bool> {
    let mut b: bool = false;
    b = Algorithm::isNotDummyStatement(statement.clone())?;
    Error::assertionOrAddSourceMessage(b.clone() || !(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), Error::META_DEAD_CODE.clone(), list![(literal!("Statement optimised away")).clone()], ElementSource::getElementSourceFileInfo(Algorithm::getStatementSource(statement.clone())?))?;
    Ok(b)
}

fn makeTupleFromMetaTuple(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut exp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (exp, ty) = (::match_deref::match_deref! { match &((inExp.clone(), inType.clone())) {
        (Deref @ DAE::Exp::META_TUPLE { listExp: exps }, Deref @ DAE::Type::T_METATUPLE { types: tys }) => {
            let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut exps = (*exps).clone();
            tys2 = List::map(tys.clone(), (std::sync::Arc::new(Types::unboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
            (exps, tys2) = Types::matchTypeTuple(exps.clone(), tys.clone(), tys2.clone(), false)?;
            (Arc::new(DAE::Exp::TUPLE { PR: exps.clone() }), Arc::new(DAE::Type::T_TUPLE { types: tys2.clone(), names: None }))
        },
        _ => {
            (inExp.clone(), inType.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, ty))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn convertExpToPatterns(mut inExp: Arc<Absyn::Exp>) -> Arc<metamodelica::List<Arc<Absyn::Exp>>> {
    let mut outInputs: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    outInputs = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp, .. } => {
            convertExpToPatterns(exp.clone())
        },
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil } } => {
            convertExpToPatterns(exp.clone())
        },
        Deref @ Absyn::Exp::TUPLE { .. } => {
            var_field!((*inExp).expressions, Absyn::Exp::TUPLE).clone()
        },
        _ => {
            list![inExp.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInputs
}

fn unboxSwitchType(mut elabMatchTy: DAE::MatchType, mut elabExps: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<DAE::MatchType> {
    let mut elabMatchTy: DAE::MatchType = elabMatchTy;
    let mut idx: i32 = 0;
    let mut hash_mod: i32 = 0;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    elabMatchTy = (::match_deref::match_deref! { match &(elabMatchTy.clone()) {
        DAE::MatchType::MATCH { switch: Some((idx, ty @ Deref @ DAE::Type::T_ENUMERATION { .. }, hash_mod)) } if (Types::isBoxedType(Expression::r#typeof(listHead(elabExps.clone())?)?)) => DAE::MatchType::MATCH { switch: Some((idx.clone(), Arc::new(DAE::Type::T_METABOXED { ty: ty.clone() }), hash_mod.clone())) },
        _ => elabMatchTy.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elabMatchTy)
}

